use crate::Compare;
use crate::Tuple;

const MATRIX_SIZE: usize = 4;

#[derive(Default, Copy, Clone)]
pub struct Matrix {
    pub data: [[f32; MATRIX_SIZE]; MATRIX_SIZE],
}

impl Matrix {
    // Initialize a new matrix with 0's
    pub fn new() -> Self {
        Self::default()
    }

    // Initialize identity matrix
    #[allow(non_snake_case)] // For usage of I
    pub fn I() -> Self {
        let mut identity_matrix = Self::new();
        for i in 0..MATRIX_SIZE {
            identity_matrix[i][i] = 1.0;
        }
        identity_matrix
    }

    // Create a copy of input and transposes inplace
    pub fn transpose(&self) -> Self {
        let mut result = *self;
        for i in 1..MATRIX_SIZE {
            for j in 0..i {
                (result[i][j], result[j][i]) = (result[j][i], result[i][j]);
            }
        }
        result
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

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..MATRIX_SIZE {
            for col in 0..MATRIX_SIZE {
                if self.data[row][col].neq(other[row][col]) {
                    return false;
                }
            }
        }
        true
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Matrix::new();
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                for k in 0..MATRIX_SIZE {
                    result[i][j] += self[i][k] * other[k][j];
                }
            }
        }
        result
    }
}

impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        Tuple::new(
            self[0][0] * other.x
                + self[0][1] * other.y
                + self[0][2] * other.z
                + self[0][3] * other.w,
            self[1][0] * other.x
                + self[1][1] * other.y
                + self[1][2] * other.z
                + self[1][3] * other.w,
            self[2][0] * other.x
                + self[2][1] * other.y
                + self[2][2] * other.z
                + self[2][3] * other.w,
            self[3][0] * other.x
                + self[3][1] * other.y
                + self[3][2] * other.z
                + self[3][3] * other.w,
        )
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

    #[test]
    fn matrix_compare1() {
        let mut m1 = Matrix::new();
        m1[0][0] = 1.0;
        m1[0][1] = 2.0;
        m1[0][2] = 3.0;
        m1[0][3] = 4.0;

        m1[1][0] = 5.0;
        m1[1][1] = 6.0;
        m1[1][2] = 7.0;
        m1[1][3] = 8.0;

        m1[2][0] = 9.0;
        m1[2][1] = 8.0;
        m1[2][2] = 7.0;
        m1[2][3] = 6.0;

        m1[3][0] = 5.0;
        m1[3][1] = 4.0;
        m1[3][2] = 3.0;
        m1[3][3] = 2.0;

        let mut m2 = Matrix::new();
        m2[0][0] = 1.0;
        m2[0][1] = 2.0;
        m2[0][2] = 3.0;
        m2[0][3] = 4.0;

        m2[1][0] = 5.0;
        m2[1][1] = 6.0;
        m2[1][2] = 7.0;
        m2[1][3] = 8.0;

        m2[2][0] = 9.0;
        m2[2][1] = 8.0;
        m2[2][2] = 7.0;
        m2[2][3] = 6.0;

        m2[3][0] = 5.0;
        m2[3][1] = 4.0;
        m2[3][2] = 3.0;
        m2[3][3] = 2.0;

        assert!(m1 == m2);
    }

    #[test]
    fn matrix_compare2() {
        let mut m1 = Matrix::new();
        m1[0][0] = 1.0;
        m1[0][1] = 2.0;
        m1[0][2] = 3.0;
        m1[0][3] = 4.0;

        m1[1][0] = 5.0;
        m1[1][1] = 6.0;
        m1[1][2] = 7.0;
        m1[1][3] = 8.0;

        m1[2][0] = 9.0;
        m1[2][1] = 8.0;
        m1[2][2] = 7.0;
        m1[2][3] = 6.0;

        m1[3][0] = 5.0;
        m1[3][1] = 4.0;
        m1[3][2] = 3.0;
        m1[3][3] = 2.0;

        let mut m2 = Matrix::new();
        m2[0][0] = 2.0;
        m2[0][1] = 3.0;
        m2[0][2] = 4.0;
        m2[0][3] = 5.0;

        m2[1][0] = 6.0;
        m2[1][1] = 7.0;
        m2[1][2] = 8.0;
        m2[1][3] = 9.0;

        m2[2][0] = 8.0;
        m2[2][1] = 7.0;
        m2[2][2] = 6.0;
        m2[2][3] = 5.0;

        m2[3][0] = 4.0;
        m2[3][1] = 3.0;
        m2[3][2] = 2.0;
        m2[3][3] = 1.0;

        assert!(m1 != m2);
    }

    #[test]
    fn matrix_multiplication() {
        let mut m1 = Matrix::new();
        m1[0][0] = 1.0;
        m1[0][1] = 2.0;
        m1[0][2] = 3.0;
        m1[0][3] = 4.0;

        m1[1][0] = 5.0;
        m1[1][1] = 6.0;
        m1[1][2] = 7.0;
        m1[1][3] = 8.0;

        m1[2][0] = 9.0;
        m1[2][1] = 8.0;
        m1[2][2] = 7.0;
        m1[2][3] = 6.0;

        m1[3][0] = 5.0;
        m1[3][1] = 4.0;
        m1[3][2] = 3.0;
        m1[3][3] = 2.0;

        let mut m2 = Matrix::new();
        m2[0][0] = -2.0;
        m2[0][1] = 1.0;
        m2[0][2] = 2.0;
        m2[0][3] = 3.0;

        m2[1][0] = 3.0;
        m2[1][1] = 2.0;
        m2[1][2] = 1.0;
        m2[1][3] = -1.0;

        m2[2][0] = 4.0;
        m2[2][1] = 3.0;
        m2[2][2] = 6.0;
        m2[2][3] = 5.0;

        m2[3][0] = 1.0;
        m2[3][1] = 2.0;
        m2[3][2] = 7.0;
        m2[3][3] = 8.0;

        let mut m3 = Matrix::new();
        m3[0][0] = 20.0;
        m3[0][1] = 22.0;
        m3[0][2] = 50.0;
        m3[0][3] = 48.0;

        m3[1][0] = 44.0;
        m3[1][1] = 54.0;
        m3[1][2] = 114.0;
        m3[1][3] = 108.0;

        m3[2][0] = 40.0;
        m3[2][1] = 58.0;
        m3[2][2] = 110.0;
        m3[2][3] = 102.0;

        m3[3][0] = 16.0;
        m3[3][1] = 26.0;
        m3[3][2] = 46.0;
        m3[3][3] = 42.0;

        assert!(m1 * m2 == m3);
    }

    #[test]
    fn matrix_multiplication_with_tuple() {
        let mut a = Matrix::new();
        a[0][0] = 1.0;
        a[0][1] = 2.0;
        a[0][2] = 3.0;
        a[0][3] = 4.0;

        a[1][0] = 2.0;
        a[1][1] = 4.0;
        a[1][2] = 4.0;
        a[1][3] = 2.0;

        a[2][0] = 8.0;
        a[2][1] = 6.0;
        a[2][2] = 4.0;
        a[2][3] = 1.0;

        a[3][0] = 0.0;
        a[3][1] = 0.0;
        a[3][2] = 0.0;
        a[3][3] = 1.0;

        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let result = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert!(a * b == result);
    }

    #[test]
    fn matrix_multiplication_with_identity() {
        let mut a = Matrix::new();
        a[0][0] = 0.0;
        a[0][1] = 1.0;
        a[0][2] = 2.0;
        a[0][3] = 4.0;

        a[1][0] = 1.0;
        a[1][1] = 2.0;
        a[1][2] = 4.0;
        a[1][3] = 8.0;

        a[2][0] = 2.0;
        a[2][1] = 4.0;
        a[2][2] = 8.0;
        a[2][3] = 16.0;

        a[3][0] = 4.0;
        a[3][1] = 8.0;
        a[3][2] = 16.0;
        a[3][3] = 32.0;

        let i = Matrix::I();
        assert!(a * i == a);
    }

    #[test]
    fn tuple_multiplication_with_identity() {
        let i = Matrix::I();
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);

        assert!(i * a == a);
    }

    #[test]
    fn transpose_matrix() {
        let mut a = Matrix::new();
        a[0][0] = 0.0;
        a[0][1] = 9.0;
        a[0][2] = 3.0;
        a[0][3] = 0.0;

        a[1][0] = 9.0;
        a[1][1] = 8.0;
        a[1][2] = 0.0;
        a[1][3] = 8.0;

        a[2][0] = 1.0;
        a[2][1] = 8.0;
        a[2][2] = 5.0;
        a[2][3] = 3.0;

        a[3][0] = 0.0;
        a[3][1] = 0.0;
        a[3][2] = 5.0;
        a[3][3] = 8.0;

        let mut b = Matrix::new();
        b[0][0] = 0.0;
        b[0][1] = 9.0;
        b[0][2] = 1.0;
        b[0][3] = 0.0;

        b[1][0] = 9.0;
        b[1][1] = 8.0;
        b[1][2] = 8.0;
        b[1][3] = 0.0;

        b[2][0] = 3.0;
        b[2][1] = 0.0;
        b[2][2] = 5.0;
        b[2][3] = 5.0;

        b[3][0] = 0.0;
        b[3][1] = 8.0;
        b[3][2] = 3.0;
        b[3][3] = 8.0;
        assert!(a.transpose() == b);
    }

    #[test]
    fn transpose_identity_matrix() {
        assert!(Matrix::I() == Matrix::I().transpose());
    }
}
