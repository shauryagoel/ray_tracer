#[derive(Default)]
pub struct Matrix2 {
    pub data: [[f32; 2]; 2],
}

#[derive(Default)]
pub struct Matrix3 {
    pub data: [[f32; 3]; 3],
}

impl Matrix2 {
    // Initialize a new matrix with 0's
    pub fn new() -> Self {
        Self::default()
    }
}

impl Matrix3 {
    // Initialize a new matrix with 0's
    pub fn new() -> Self {
        Self::default()
    }
}

// Use this to get a row of matrix when indexing Matrix
impl std::ops::Index<usize> for Matrix2 {
    type Output = [f32; 2];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl std::ops::IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

impl std::ops::Index<usize> for Matrix3 {
    type Output = [f32; 3];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl std::ops::IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

#[cfg(test)]
mod small_matrix_tests {
    use super::*;

    #[test]
    fn matrix_init2x2() {
        let mut m = Matrix2::new();
        m[0][0] = -3.0;
        m[0][1] = 5.0;
        m[1][0] = 1.0;
        m[1][1] = -2.0;

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn matrix_init3x3() {
        let mut m = Matrix3::new();
        m[0][0] = -3.0;
        m[0][1] = 5.0;
        m[1][0] = 1.0;
        m[1][1] = -2.0;
        m[1][2] = -7.0;
        m[2][1] = 1.0;
        m[2][2] = 1.0;

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }
}
