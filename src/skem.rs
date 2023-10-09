use crate::noise::*;
use crate::vec::Matrix;
use crate::params::{N, N_BAR, Element};
use crate::poly::{help_rec, rec_element};

pub fn gen_pp() -> [u8; 32] {
    gen_seed()
}

pub fn gen_a(seed: [u8; 32]) -> (Matrix, (Matrix, Matrix)) {
   let pub_mat = gen_matrix_uniform_seed(N, N, seed);
   let sk_a_t = gen_matrix_chi(N_BAR, N);
   let d_a_t = gen_matrix_chi(N_BAR, N);
   let b_a_t = pub_mat.mul_add_transpose(&sk_a_t, &d_a_t); 
   let f_a_t = gen_matrix_chi(N_BAR, N_BAR);
   (b_a_t, (sk_a_t, f_a_t))
}


pub fn gen_b(seed: [u8; 32]) -> (Matrix, (Matrix, Matrix)) {
    let pub_mat_t = gen_matrix_transpose_uniform_seed(N, N, seed);
    let sk_b = gen_matrix_chi(N_BAR, N);
    let sk_b_copy = sk_b.clone();
    let d_b_t = gen_matrix_chi(N_BAR, N);
    let b_b = pub_mat_t.mul_add_transpose(&sk_b_copy, &d_b_t);
    let f_b_t = gen_matrix_chi(N_BAR, N_BAR);
    (b_b, (sk_b, f_b_t))
}

pub fn encaps(b_a_t: Matrix, sk_b: Matrix) -> ([i32; 64], u64) {
    //let pub_mat_t = gen_matrix_transpose_uniform_seed(N, N, [0;32]);
    let e_b_t = gen_matrix_uniform_beta(N_BAR, N_BAR);
    let v_t = sk_b.mul_add_transpose(&b_a_t, &e_b_t);
    let (key_t, ct_t) = help_rec(v_t);
    (key_t, ct_t)
}

pub fn decaps(b_b: Matrix, sk_a_t: Matrix, f_a_t: Matrix, ct: u64) -> [i32; 64] {
    let v_p_t = b_b.mul_add_transpose(&sk_a_t, &f_a_t);
    let v_p_t_double = v_p_t.double().get_data();
    let mut key_p = [0; 64];
    for i in 0..64 {
        let bit = (ct >> i) & 0x1;
        key_p[i] = rec_element(v_p_t_double[i], bit as Element);
    }
    key_p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctness_test() {
        let nb_tests = 1;
        for _ in 0..nb_tests {
            let seed = gen_pp();
            let (b_a, (sk_a, f_a)) = gen_a(seed);
            let (b_b, (sk_b, _f_b)) = gen_b(seed);
            let (k, ct) = encaps(b_a, sk_b);
            let k_p = decaps(b_b, sk_a, f_a, ct);
            //println!("{:?}", k);
            //println!("{:?}", k_p);
            assert_eq!(k.to_vec(), k_p.to_vec());
        }
    }
}





