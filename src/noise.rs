use crate::params::{BETA, Q};
use crate::vec::Matrix;
use rand::SeedableRng;
use rand_core::RngCore;
use rand::Rng;
use rand::rngs::{OsRng,StdRng};
use rand::distributions::{Distribution, Uniform};

pub fn gen_matrix_chi(r: usize, c: usize) -> Matrix {
    let size = r*c;
    let mut data = Vec::with_capacity(size);
    //let coin = Bernoulli::new(0.5).unwrap();
    for _ in 0..size {
        data.push(1 - 2*(rand::thread_rng().gen::<i32>() & 0x1));
    }
    Matrix::new(r, c, data)
}

pub fn gen_matrix_uniform_beta(r: usize, c: usize) -> Matrix {
    let size = r*c;
    let mut data = Vec::with_capacity(size);
    let dist = Uniform::from(0..2*BETA+1);
    for _ in 0..size {
        data.push(dist.sample(&mut OsRng) - BETA);
    }
    Matrix::new(r, c, data)
}


pub fn gen_matrix_uniform_seed(r: usize, c: usize, seed: [u8; 32]) -> Matrix {
    let size = r*c;
    let mut data = Vec::with_capacity(size);
    let dist = Uniform::from(0..Q+1);
    let mut prng = StdRng::from_seed(seed);
    for _ in 0..size {
        data.push(dist.sample(&mut prng));
    }
    Matrix::new(r, c, data)
}

pub fn gen_matrix_transpose_uniform_seed(r: usize, c:usize, seed: [u8; 32]) -> Matrix {
    let size = r*c;
    let mut data = vec![0; size];
    let dist = Uniform::from(0..Q+1);
    let mut prng = StdRng::from_seed(seed);
    for i in 0..c {
        for j in 0..r {
            data[j*c+i] = dist.sample(&mut prng);
        }
    }
    Matrix::new(r, c, data)
}


pub fn gen_seed() -> [u8; 32] {
    let mut seed = [0; 32];
    OsRng.fill_bytes(&mut seed);
    seed
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn same_seed_gen_same_mat_transpose_test() {
        let r = 3;
        let c = 11;
        let seed = gen_seed();
        let mat = gen_matrix_uniform_seed(r, c, seed);
        let mat_t = gen_matrix_transpose_uniform_seed(c, r, seed);
        println!("{:?}", mat);
        println!("{:?}", mat_t);
    }
    #[test]
    fn same_seed_gen_same_matrix_test() {
        let r = 1452;
        let c = 8;
        let seed = gen_seed();
        let mat1 = gen_matrix_uniform_seed(r, c, seed);
        let mat2 = gen_matrix_uniform_seed(r, c, seed);
        for row in 0..r {
            for col in 0..c {
                assert!(mat1.get(row, col) == mat2.get(row, col));
            }
        }
    }
    #[test]
    fn gen_matrix_close_to_uniform_test() {
        let threshold: f64 = 500.0;
        let expectation: f64 = Q as f64 / 2.0;
        let r = 1452;
        let c = 8;
        let size = (r*c) as f64;
        let seed = gen_seed();
        let mat = gen_matrix_uniform_seed(r, c, seed);
        let mut acc = 0.0;
        for row in 0..r {
            for col in 0..c {
                let v = mat.get(row, col);
                acc += v as f64 / size;
            }
        }
        assert!(acc <= expectation + threshold && acc > expectation - threshold);
    }

    #[test]
    fn gen_matrix_close_to_chi_test() {
        let r = 1452;
        let c = 8;
        let mat = gen_matrix_chi(r, c);
        let mut cnt = 0.0;
        for row in 0..r {
            for col in 0..c {
                if mat.get(row, col) == 1 {
                    cnt += 1.0;
                }
            }
        }
        let ratio = cnt/((r*c) as f64);
        assert!(ratio <= 0.6 && ratio > 0.4);
    }
    #[test]
    fn gen_matrix_contains_correct_values_test() {
        let r = 1452;
        let c = 8;
        let mat = gen_matrix_chi(r, c);
        for row in 0..r {
            for col in 0..c {
                let v = mat.get(row, col);
                assert!(v == 1 || v == -1);
            }
        }
    }
}
