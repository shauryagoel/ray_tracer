const MATRIX_SIZE: usize = 4;

#[derive(Default)]
pub struct Matrix {
    pub data: [[f32; MATRIX_SIZE]; MATRIX_SIZE],
}

impl Matrix {
    // Initialize a new matrix with 0's
    pub fn new() -> Matrix {
        Matrix {
            data: [[0.0; MATRIX_SIZE]; MATRIX_SIZE],
        }
    }
}

// Use this to get a row of matrix when indexing Matrix
impl std::ops::Index<usize> for Matrix {
    type Output = [f32; MATRIX_SIZE];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    fn matrix_init4x4() {
        let mut m = Matrix::new();
        m[0][0] = 1.0;
        m[0][1] = 2.0;
        m[0][2] = 3.0;
        m[0][3] = 4.0;

        m[1][0] = 5.5;
        m[1][1] = 6.5;
        m[1][2] = 7.5;
        m[1][3] = 8.5;

        m[2][0] = 9.0;
        m[2][1] = 10.0;
        m[2][2] = 11.0;
        m[2][3] = 12.0;

        m[3][0] = 13.5;
        m[3][1] = 14.5;
        m[3][2] = 15.5;
        m[3][3] = 16.5;

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }
}
