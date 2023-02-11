use crate::Compare;
use crate::Ray;
use crate::{point, Tuple};

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: Tuple,
    radius: f32,
}

impl Sphere {
    // Create a sphere centered at origin and of radius 1
    pub fn new() -> Sphere {
        Sphere {
            center: point(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    // Find at which t, ray intersects sphere
    pub fn intersects(&self, ray: Ray) -> Vec<f32> {
        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![t1, t2]
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Sphere) -> bool {
        self.center == other.center && self.radius.eq(other.radius)
    }
}

#[cfg(test)]
mod sphere_tests {
    use super::*;
    use crate::vector;

    #[test]
    fn sphere_ray_intersection1() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn sphere_ray_intersection2() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }
    #[test]
    fn sphere_ray_intersection3() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn sphere_ray_intersection_ray_origin_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_ray_intersection_before_ray_origin() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersects(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}
