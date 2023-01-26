pub struct Tuple {
    pub x: f32, // x coordinate of the Tuple
    pub y: f32, // y coordinate of the Tuple
    pub z: f32, // z coordinate of the Tuple
    pub w: f32, // indicator whether the Tuple is a vector(w=0.0) or a point(w=1.0). It is float as we need it for computation rather than an actual indicator
}

// Factory methods for creating vector and point
pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

impl Tuple {
    pub fn add(&self, other: &Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_validity() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn vector_validity() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
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
        let _p = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0,
        };
        assert!(p.is_point());
    }

    #[test]
    fn vector_factory() {
        let p = vector(4.0, -4.0, 3.0);
        let _p = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 0.0,
        };
        assert!(p.is_vector());
    }
}
