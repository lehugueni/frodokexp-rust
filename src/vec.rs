use crate::params::{Q, Element};

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Element>,
}


impl Matrix {
   pub fn new(r: usize, c: usize, d: Vec<Element>) -> Self {
       if r*c != d.len() {
           panic!("Provided data should be of size row*col ({}) but is of size {}", r*c, d.len());
       }
       Self { rows: r, cols: c, data: d }
   } 

   pub fn from_bytes(bytes: &[u8], r: usize, c: usize) -> Self {
      let size = r*c; 
      let mut mat = Vec::with_capacity(size);
      for i in (0..bytes.len()).step_by(2) {
          let msb = (bytes[i] as i16) << 8;
          let lsb = bytes[i+1] as i16;
          mat.push((msb | lsb) as Element);
      }
      Self::new(r, c, mat)
   }


   pub fn get_data(self) -> Vec<Element> {
      self.data
   }

   pub fn to_bytes(self) -> Vec<u8> {
       let mut v = Vec::with_capacity(self.data.len()*2);
       for i in 0..self.data.len(){
           let msb = (self.data[i] & 0xFF00) >> 8;
           let lsb = self.data[i] & 0xFF;
           v.push(msb as u8);
           v.push(lsb as u8);
       }
       v
   }


   pub fn get(&self, i: usize, j: usize) -> Element {
      self.data[i*self.cols+j] 
   }

   pub fn double(self) -> Self {
       let double_mat = self.data.into_iter().map(|e| {2*e}).collect();
       Matrix::new(self.rows, self.cols, double_mat)
   }


   pub fn add_mat(self, rhs: Matrix) -> Matrix {
       if self.rows != rhs.rows || self.cols != rhs.cols {
           panic!("LHS and RHS should have the same size");
       }
       let mut res = Vec::with_capacity(self.data.len());
       for i in 0..self.data.len() {
           res.push((self.data[i] + rhs.data[i]).rem_euclid(Q));
       }
       Matrix::new(self.rows, self.cols, res)
   }


   pub fn mul_add_transpose(&self, rhs: &Matrix, add: &Matrix) -> Matrix {
       if self.cols != rhs.cols {
           panic!("LHS and RHS should have the same width ({}, {})", self.cols, rhs.cols);
       }
       if add.rows != rhs.rows || add.cols != self.rows {
           panic!("ADD should be of size RHS.rows ({}) x LHS.rows ({}) but is of size ({} x {})", rhs.rows, self.rows, add.rows, add.cols);
       }
       if rhs.rows % 8 != 0 {
           panic!("RHS height should be a multiple of 8");
       }
       let mut out = vec![0; self.rows*rhs.rows].to_vec();
       for lhs_row in 0..self.rows {
           for rhs_row in 0..rhs.rows / 8 {
               let mut acc = [0; 8];
               for off in 0..rhs.cols {
                   acc[0] += (self.data[lhs_row*self.cols + off]*rhs.data[8*rhs_row*rhs.cols + off]) % Q;
                   acc[1] += (self.data[lhs_row*self.cols + off]*rhs.data[(8*rhs_row+1)*rhs.cols + off]) % Q;
                   acc[2] += (self.data[lhs_row*self.cols + off]*rhs.data[(8*rhs_row+2)*rhs.cols + off]) % Q;
                   acc[3] += (self.data[lhs_row*self.cols + off]*rhs.data[(8*rhs_row+3)*rhs.cols + off]) % Q;
                   acc[4] += (self.data[lhs_row*self.cols + off]*rhs.data[(8*rhs_row+4)*rhs.cols + off]) % Q;
                   acc[5] += (self.data[lhs_row*self.cols + off]*rhs.data[(8*rhs_row+5)*rhs.cols + off]) % Q;
                   acc[6] += (self.data[lhs_row*self.cols + off]*rhs.data[(8*rhs_row+6)*rhs.cols + off]) % Q;
                   acc[7] += (self.data[lhs_row*self.cols + off]*rhs.data[(8*rhs_row+7)*rhs.cols + off]) % Q;
               }

               out[(8*rhs_row)*self.rows + lhs_row] = (acc[0] + add.data[(8*rhs_row)*self.rows + lhs_row]).rem_euclid(Q);
               out[(8*rhs_row+1)*self.rows + lhs_row] = (acc[1] + add.data[(8*rhs_row+1)*self.rows + lhs_row]).rem_euclid(Q);
               out[(8*rhs_row+2)*self.rows + lhs_row] = (acc[2] + add.data[(8*rhs_row+2)*self.rows + lhs_row]).rem_euclid(Q);
               out[(8*rhs_row+3)*self.rows + lhs_row] = (acc[3] + add.data[(8*rhs_row+3)*self.rows + lhs_row]).rem_euclid(Q);
               out[(8*rhs_row+4)*self.rows + lhs_row] = (acc[4] + add.data[(8*rhs_row+4)*self.rows + lhs_row]).rem_euclid(Q);
               out[(8*rhs_row+5)*self.rows + lhs_row] = (acc[5] + add.data[(8*rhs_row+5)*self.rows + lhs_row]).rem_euclid(Q);
               out[(8*rhs_row+6)*self.rows + lhs_row] = (acc[6] + add.data[(8*rhs_row+6)*self.rows + lhs_row]).rem_euclid(Q);
               out[(8*rhs_row+7)*self.rows + lhs_row] = (acc[7] + add.data[(8*rhs_row+7)*self.rows + lhs_row]).rem_euclid(Q)
           } 
       }
       Matrix::new(rhs.rows, self.rows, out)
   }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_matrix() {
        let v: Vec<Element> = (0..15).collect();
        Matrix::new(5, 3, v);
        assert!(1 == 1);
    }
    #[test]
    fn mutiply_transpose_test() {
        let m = Matrix::new(3, 2, vec![1,1,2,2,3,3]);
        let rhs = Matrix::new(8, 2, vec![1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,]);
        let add = Matrix::new(8, 3, vec![0;24]);
        let mat_res = m.mul_add_transpose(&rhs, &add);
        let correct = vec![2, 4, 6, 4, 8, 12, 6, 12, 18, 8, 16, 24, 10, 20, 30, 12, 24, 36, 14, 28, 42, 16, 32, 48];
        assert_eq!(mat_res.data, correct);
    }

    #[test]
    fn byte_conversion_test() {
        let m = Matrix::new(3, 4, vec![-1021, 1123, 1224, 3243, 4324, 31750, 10001, -213, 2, 0, 32, 25-1]);
        let m_copy = m.clone();
        let b = m.to_bytes();
        let m_p = Matrix::from_bytes(&b, 3, 4);
        assert_eq!(m_copy.data, m_p.data);
    }
}
