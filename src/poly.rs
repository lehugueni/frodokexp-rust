use crate::params::{Element, Q, CUTOFFS, B, N_BAR};
use crate::vec::Matrix;
use rand::rngs::OsRng;
use rand::distributions::{Distribution, Bernoulli};

pub fn help_rec(v: Matrix) -> ([i32; 64], u64) {
   let coin = Bernoulli::new(0.5).unwrap(); 
   let v_data = v.get_data();
   let mut key = [0; 64];
   if v_data.len() as i32 != (N_BAR*N_BAR).try_into().unwrap() {
       panic!("V matrix should have size n_bar*n_bar ({})", N_BAR*N_BAR);
   }
  let mut ct: u64 = 0;
   for i in 0..64 {
       let a = coin.sample(&mut OsRng);
       let b = coin.sample(&mut OsRng);
       let v_bar = 2 * v_data[i] - (a as Element - b as Element);
       let bit = ((v_bar << B) / Q) & 0x1;
       ct |= (bit as u64) << i;
       key[i] = rec_element(v_data[i] << 1, bit);
   }
   (key, ct)
}

fn rounding_div(n: Element, d: Element) -> Element {
    (n + (d >> 1)) / d
}
fn abs_sub(a: Element, b: Element) -> (Element, Element) {
    let r = a - b;
    let s = r >> 31;
    (((r + s) ^ s), s)
}
fn closest_v(v: Element, b: Element) -> Element {
    let cr_v = ((v << B) / Q) & 1 == b;
    let mut curr_dist = Q;
    let mut curr_closest_v = 0;
    for c in CUTOFFS {
        let (distance, sign) = abs_sub(c, v);
        let offset = -sign;
        let rep = (distance+offset < curr_dist) as Element;
        let rep_bar = 1-rep;
        let equal = (c == v) as i32;
        curr_dist = (rep * (distance+offset)) | (rep_bar * curr_dist);
        curr_closest_v = (rep * (c+sign-equal)) | (rep_bar * curr_closest_v);
    }
    curr_closest_v = (cr_v as Element * v) | ((1-cr_v as Element) * curr_closest_v);
    curr_closest_v.rem_euclid(Q << 1)
}
pub fn rec_element(w: Element, b: Element) -> Element {
    let closest_v = closest_v(w, b);
    rounding_div(closest_v << (B-1), Q) & 0b1111
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs_sub_test() {
        let (r, _s) = abs_sub(5, 43);
        assert_eq!(r, 38);
    }
}

