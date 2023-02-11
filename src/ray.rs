use crate::Tuple;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    // Computes new position of ray after time t
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
        assert!(r.origin == origin);
        assert!(r.direction == direction);
    }
}
