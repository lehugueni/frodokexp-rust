use std::os::raw::{c_uchar, c_ulonglong};
use std::ptr;
mod vec;
mod params;
mod noise;
mod poly;
mod skem;
pub use skem::{gen_pp, gen_a, gen_b, encaps, decaps};
pub use vec::Matrix;
pub const SEED_SIZE_BYTES: usize = 32;
pub const SK_SIZE_BYTES: usize = params::N_BAR*params::N*2;
pub const PK_SIZE_BYTES: usize = params::N*params::N_BAR*2;
pub const F_SIZE_BYTES: usize = params::N_BAR*params::N_BAR*2;
pub const SS_SIZE_BYTES: usize = 32;


#[no_mangle]
pub extern "C" fn frodokexp_gen_pp(seed:  *mut c_uchar){
    let res = gen_pp();
    let res_ptr = res.as_ptr() as *const c_uchar;
    unsafe{
        ptr::copy_nonoverlapping(res_ptr, seed, SEED_SIZE_BYTES);
    }
}

#[no_mangle]
pub extern "C" fn frodokexp_gen_a(seed: *const c_uchar, sk_out: *mut c_uchar, f_out: *mut c_uchar, pk_out: *mut c_uchar){
    unsafe{ 
        let seed_arr = std::slice::from_raw_parts(seed as *const u8, SEED_SIZE_BYTES); 
        let (b_a, (sk_a, f_a)) = gen_a(seed_arr.try_into().unwrap());
        ptr::copy_nonoverlapping(sk_a.to_bytes().as_ptr(), sk_out, SK_SIZE_BYTES);
        ptr::copy_nonoverlapping(f_a.to_bytes().as_ptr(), f_out, F_SIZE_BYTES);
        ptr::copy_nonoverlapping(b_a.to_bytes().as_ptr(), pk_out, PK_SIZE_BYTES);
    }
}

#[no_mangle]
pub extern "C" fn frodokexp_gen_b(seed: *const c_uchar, sk_out: *mut c_uchar, f_out: *mut c_uchar, pk_out: *mut c_uchar){
    unsafe{ 
        let seed_arr = std::slice::from_raw_parts(seed as *const u8, SEED_SIZE_BYTES); 
        let (b_b, (sk_b, f_b)) = gen_b(seed_arr.try_into().unwrap());
        ptr::copy_nonoverlapping(sk_b.to_bytes().as_ptr(), sk_out, SK_SIZE_BYTES);
        ptr::copy_nonoverlapping(f_b.to_bytes().as_ptr(), f_out, F_SIZE_BYTES);
        ptr::copy_nonoverlapping(b_b.to_bytes().as_ptr(), pk_out, PK_SIZE_BYTES);
    }
}

#[no_mangle]
pub extern "C" fn frodokexp_encaps(b_a: *const c_uchar, sk_b: *const c_uchar, key_out: *mut c_uchar, ct_out: *mut c_ulonglong) {
    unsafe {
        let b_a_arr = std::slice::from_raw_parts(b_a as *const u8, PK_SIZE_BYTES);
        let sk_b_arr = std::slice::from_raw_parts(sk_b as *const u8, SK_SIZE_BYTES);
        let b_a_mat = Matrix::from_bytes(b_a_arr, params::N_BAR, params::N);
        let sk_b_mat = Matrix::from_bytes(sk_b_arr, params::N_BAR, params::N);
        let (k, ct) = encaps(b_a_mat, sk_b_mat);
        let k_bytes = key_to_bytes(k);
        ptr::copy_nonoverlapping(&k_bytes as *const c_uchar, key_out, SS_SIZE_BYTES);
        *ct_out = ct;
    }
}

#[no_mangle]
pub extern "C" fn frodokexp_decaps(b_b: *const c_uchar, sk_a: *const c_uchar, f_a: *const c_uchar, ct: *const c_ulonglong, key_out: *mut c_uchar) {
    unsafe {
        let b_b_arr = std::slice::from_raw_parts(b_b as *const u8, PK_SIZE_BYTES);
        let sk_a_arr = std::slice::from_raw_parts(sk_a as *const u8, SK_SIZE_BYTES);
        let f_a_arr = std::slice::from_raw_parts(f_a as *const u8, F_SIZE_BYTES);
        let b_b_mat = Matrix::from_bytes(b_b_arr, params::N_BAR, params::N);
        let sk_a_mat = Matrix::from_bytes(sk_a_arr, params::N_BAR, params::N);
        let f_a_mat = Matrix::from_bytes(f_a_arr, params::N_BAR, params::N_BAR);
        let k = decaps(b_b_mat, sk_a_mat, f_a_mat, *ct);
        let k_bytes = key_to_bytes(k);
        ptr::copy_nonoverlapping(&k_bytes as *const c_uchar, key_out, SS_SIZE_BYTES);
    }
}

fn key_to_bytes(key: [i32; 64]) -> [u8; 32] {
    let mut k_bytes = [0u8; 32];
    for i in 0..32 {
        let msb = (key[i] << 4) as u8;
        let lsb = key[i + 32] as u8;
        k_bytes[i] = msb | lsb;
    }
    k_bytes
}
