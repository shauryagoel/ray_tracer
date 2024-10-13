use crate::Matrix;

impl Matrix {
    pub fn get_translation_matrix(x: f32, y: f32, z: f32) -> Self {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[0][3] = x;
        transformation_matrix[1][3] = y;
        transformation_matrix[2][3] = z;
        transformation_matrix
    }

    pub fn get_scaling_matrix(x: f32, y: f32, z: f32) -> Self {
        let mut transformation_matrix = Matrix::new();
        transformation_matrix[0][0] = x;
        transformation_matrix[1][1] = y;
        transformation_matrix[2][2] = z;
        transformation_matrix[3][3] = 1.0;
        transformation_matrix
    }

    // Get rotation matrix around +x direction, r is in radians
    pub fn get_rotation_x_matrix(r: f32) -> Self {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[1][1] = r.cos();
        transformation_matrix[1][2] = -r.sin();
        transformation_matrix[2][1] = r.sin();
        transformation_matrix[2][2] = r.cos();
        transformation_matrix
    }

    // Get rotation matrix around +y direction, r is in radians
    pub fn get_rotation_y_matrix(r: f32) -> Self {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[0][0] = r.cos();
        transformation_matrix[0][2] = r.sin();
        transformation_matrix[2][0] = -r.sin();
        transformation_matrix[2][2] = r.cos();
        transformation_matrix
    }

    // Get rotation matrix around +z direction, r is in radians
    pub fn get_rotation_z_matrix(r: f32) -> Self {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[0][0] = r.cos();
        transformation_matrix[0][1] = -r.sin();
        transformation_matrix[1][0] = r.sin();
        transformation_matrix[1][1] = r.cos();
        transformation_matrix
    }

    pub fn get_shearing_matrix(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        let mut transformation_matrix = Matrix::I();
        transformation_matrix[0][1] = xy;
        transformation_matrix[0][2] = xz;
        transformation_matrix[1][0] = yx;
        transformation_matrix[1][2] = yz;
        transformation_matrix[2][0] = zx;
        transformation_matrix[2][1] = zy;
        transformation_matrix
    }
}

#[cfg(test)]
mod transformation_tests {
    use super::*;
    use crate::{point, vector};
    use std::f32::consts::PI;

    #[test]
    fn point_translation1() {
        let transform = Matrix::get_translation_matrix(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        let _p = point(2.0, 1.0, 7.0);

        assert_eq!(transform * p, _p);
    }

    #[test]
    fn point_translation2() {
        let transform = Matrix::get_translation_matrix(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        let _p = point(-8.0, 7.0, 3.0);

        assert_eq!(inv * p, _p);
    }

    #[test]
    fn vector_translation() {
        let transform = Matrix::get_translation_matrix(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn point_scaling() {
        let transform = Matrix::get_scaling_matrix(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        let _p = point(-8.0, 18.0, 32.0);

        assert_eq!(transform * p, _p);
    }

    #[test]
    fn vector_scaling() {
        let transform = Matrix::get_scaling_matrix(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        let _v = vector(-8.0, 18.0, 32.0);

        assert_eq!(transform * v, _v);
    }

    #[test]
    fn inverse_scaling() {
        let transform = Matrix::get_scaling_matrix(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = vector(-4.0, 6.0, 8.0);
        let _v = vector(-2.0, 2.0, 2.0);

        assert_eq!(inv * v, _v);
    }

    #[test]
    fn reflection_scaling() {
        let transform = Matrix::get_scaling_matrix(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(-2.0, 3.0, 4.0);

        assert_eq!(transform * p, _p);
    }

    #[test]
    fn rotation_x() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::get_rotation_x_matrix(PI / 4.0);
        let full_quarter = Matrix::get_rotation_x_matrix(PI / 2.0);
        let _p1 = point(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0);
        let _p2 = point(0.0, 0.0, 1.0);

        assert_eq!(half_quarter * p, _p1);
        assert_eq!(full_quarter * p, _p2);
    }

    #[test]
    fn inverse_rotation_x() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::get_rotation_x_matrix(PI / 4.0);
        let inv = half_quarter.inverse();
        let _p = point(0.0, 2.0_f32.sqrt() / 2.0, -(2.0_f32.sqrt()) / 2.0);
        assert_eq!(inv * p, _p);
    }

    #[test]
    fn rotation_y() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::get_rotation_y_matrix(PI / 4.0);
        let full_quarter = Matrix::get_rotation_y_matrix(PI / 2.0);
        let _p1 = point(2.0_f32.sqrt() / 2.0, 0.0, 2.0_f32.sqrt() / 2.0);
        let _p2 = point(1.0, 0.0, 0.0);

        assert_eq!(half_quarter * p, _p1);
        assert_eq!(full_quarter * p, _p2);
    }

    #[test]
    fn rotation_z() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::get_rotation_z_matrix(PI / 4.0);
        let full_quarter = Matrix::get_rotation_z_matrix(PI / 2.0);
        let _p1 = point(-(2.0_f32.sqrt()) / 2.0, 2.0_f32.sqrt() / 2.0, 0.0);
        let _p2 = point(-1.0, 0.0, 0.0);

        assert_eq!(half_quarter * p, _p1);
        assert_eq!(full_quarter * p, _p2);
    }

    #[test]
    fn shear_x_according_y() {
        let transform = Matrix::get_shearing_matrix(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(5.0, 3.0, 4.0);
        assert_eq!(transform * p, _p);
    }

    #[test]
    fn shear_x_according_z() {
        let transform = Matrix::get_shearing_matrix(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(6.0, 3.0, 4.0);
        assert_eq!(transform * p, _p);
    }

    #[test]
    fn shear_y_according_x() {
        let transform = Matrix::get_shearing_matrix(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(2.0, 5.0, 4.0);
        assert_eq!(transform * p, _p);
    }

    #[test]
    fn shear_y_according_z() {
        let transform = Matrix::get_shearing_matrix(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(2.0, 7.0, 4.0);
        assert_eq!(transform * p, _p);
    }

    #[test]
    fn shear_z_according_x() {
        let transform = Matrix::get_shearing_matrix(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(2.0, 3.0, 6.0);
        assert_eq!(transform * p, _p);
    }

    #[test]
    fn shear_z_according_y() {
        let transform = Matrix::get_shearing_matrix(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        let _p = point(2.0, 3.0, 7.0);
        assert_eq!(transform * p, _p);
    }

    #[allow(non_snake_case)]
    #[test]
    fn chaining_transformations1() {
        let p = point(1.0, 0.0, 1.0);
        let A = Matrix::get_rotation_x_matrix(PI / 2.0);
        let B = Matrix::get_scaling_matrix(5.0, 5.0, 5.0);
        let C = Matrix::get_translation_matrix(10.0, 5.0, 7.0);

        let p2 = A * p;
        assert_eq!(p2, point(1.0, -1.0, 0.0));

        let p3 = B * p2;
        assert_eq!(p3, point(5.0, -5.0, 0.0));

        let p4 = C * p3;
        assert_eq!(p4, point(15.0, 0.0, 7.0));
    }

    #[allow(non_snake_case)]
    #[test]
    fn chaining_transformations2() {
        let p = point(1.0, 0.0, 1.0);
        let A = Matrix::get_rotation_x_matrix(PI / 2.0);
        let B = Matrix::get_scaling_matrix(5.0, 5.0, 5.0);
        let C = Matrix::get_translation_matrix(10.0, 5.0, 7.0);
        let T = C * B * A;

        assert_eq!(T * p, point(15.0, 0.0, 7.0));
    }
}
