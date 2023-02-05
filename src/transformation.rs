use crate::Matrix;

impl Matrix {
    pub fn get_translation_matrix(x: f32, y: f32, z: f32) -> Matrix {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[0][3] = x;
        transformation_matrix[1][3] = y;
        transformation_matrix[2][3] = z;
        transformation_matrix
    }
}

#[cfg(test)]
mod transformation_tests {
    use super::*;
    use crate::{point, vector};

    #[test]
    fn point_translation1() {
        let transform = Matrix::get_translation_matrix(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        let _p = point(2.0, 1.0, 7.0);

        assert!(transform * p == _p);
    }

    #[test]
    fn point_translation2() {
        let transform = Matrix::get_translation_matrix(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        let _p = point(-8.0, 7.0, 3.0);

        assert!(inv * p == _p);
    }

    #[test]
    fn vector_translation() {
        let transform = Matrix::get_translation_matrix(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);

        assert!(transform * v == v);
    }
}
