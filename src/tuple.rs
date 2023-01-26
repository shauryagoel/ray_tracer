use crate::utils;

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
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
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
}
