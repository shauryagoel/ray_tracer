use crate::utils::Compare;

pub struct Tuple {
    pub x: f32, // x coordinate of the Tuple
    pub y: f32, // y coordinate of the Tuple
    pub z: f32, // z coordinate of the Tuple
    pub w: f32, // indicator whether the Tuple is a vector(w=0.0) or a point(w=1.0). It is float as we need it for computation rather than an actual indicator
}

// Factory methods for creating vector and point
pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    // Pythagoras theorem
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.x.eq(other.x) && self.y.eq(other.y) && self.z.eq(other.z) && self.w.eq(other.w)
    }
}

impl std::ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl std::ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl std::ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, a: f32) -> Tuple {
        Tuple::new(self.x * a, self.y * a, self.z * a, self.w * a)
    }
}

impl std::ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, a: f32) -> Tuple {
        Tuple::new(self.x / a, self.y / a, self.z / a, self.w / a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_validity() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn vector_validity() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_factory() {
        let p = point(4.0, -4.0, 3.0);
        let _p = Tuple::new(4.0, -4.0, 3.0, 1.0);
        assert!(p == _p);
    }

    #[test]
    fn vector_factory() {
        let p = vector(4.0, -4.0, 3.0);
        let _p = Tuple::new(4.0, -4.0, 3.0, 0.0);
        assert!(p == _p);
    }

    #[test]
    fn tuple_addition() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let _a = Tuple::new(1.0, 1.0, 6.0, 1.0);
        assert!(a1 + a2 == _a)
    }

    #[test]
    fn point_subtraction() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let _p = vector(-2.0, -4.0, -6.0);
        assert!(p1 - p2 == _p);
    }

    #[test]
    fn point_vector_backward() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        let _a = point(-2.0, -4.0, -6.0);
        assert!(p - v == _a);
    }

    #[test]
    fn vector_subtraction() {
        let p1 = vector(3.0, 2.0, 1.0);
        let p2 = vector(5.0, 6.0, 7.0);
        let _p = vector(-2.0, -4.0, -6.0);
        assert!(p1 - p2 == _p);
    }

    #[test]
    fn reverse_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        let _v = vector(-1.0, 2.0, -3.0);
        assert!(zero - v == _v);
    }

    #[test]
    fn negation() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let _a = Tuple::new(-1.0, 2.0, -3.0, 4.0);
        assert!(-a == _a);
    }

    #[test]
    fn scaling_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let _a = Tuple::new(3.5, -7.0, 10.5, -14.0);
        assert!(a * 3.5 == _a);
    }

    #[test]
    fn fraction_scaling_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let _a = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert!(a * 0.5 == _a);
    }

    #[test]
    fn dividing_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let _a = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert!(a / 2.0 == _a);
    }

    #[test]
    fn vector_magnitude1() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn vector_magnitude2() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn vector_magnitude3() {
        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn vector_magnitude4() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), f32::sqrt(14.0));
    }

    #[test]
    fn vector_magnitude5() {
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), f32::sqrt(14.0));
    }
}
