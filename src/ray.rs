use crate::Tuple;

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,    // Is a point
    pub direction: Tuple, // Is a vector
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    // Computes new position of ray after time `t`
    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod ray_tests {
    use super::*;
    use crate::{point, vector};

    #[test]
    fn ray_creation() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn comuting_position_with_time() {
        let origin = point(2.0, 3.0, 4.0);
        let direction = vector(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.position(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), point(4.5, 3.0, 4.0));
    }
}
