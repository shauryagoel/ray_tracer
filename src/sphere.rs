use crate::Ray;
use crate::{point, Matrix, Tuple};
use crate::{Intersection, Intersections};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    // TODO: add `id` to it as described in the book
    center: Tuple,
    radius: f32,
    transform: Matrix, // Transformation matrix
}

impl Sphere {
    pub fn new(center: Tuple, radius: f32, transform: Matrix) -> Self {
        Self {
            center,
            radius,
            transform,
        }
    }

    // Returns the time at which the `ray` intersects the sphere
    pub fn intersect(&self, ray: Ray) -> Intersections {
        // Transform the ray to the object space coordinates of the sphere
        // This means applying inverse transformation of the sphere to the ray
        let ray = ray.transform(self.transform.inverse());

        let sphere_to_ray = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        let mut intersections: Intersections = Default::default();

        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            intersections.push(Intersection::new(t1, *self));
            intersections.push(Intersection::new(t2, *self));
        }
        intersections
    }

    pub fn set_transform(&mut self, t: Matrix) {
        self.transform = t;
    }
}

impl Default for Sphere {
    // Create a sphere centered at origin, of radius 1 and with identity transformation matrix
    fn default() -> Self {
        Self::new(point(0.0, 0.0, 0.0), 1.0, Matrix::I())
    }
}

// impl PartialEq for Sphere {
//     fn eq(&self, other: &Self) -> bool {
//         self.center == other.center && self.radius.eq(other.radius)
//     }
// }

#[cfg(test)]
mod sphere_tests {
    use super::*;
    use crate::vector;

    #[test]
    fn sphere_ray_intersection1() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn sphere_ray_intersection2() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn sphere_ray_intersection3() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn sphere_ray_intersection_ray_origin_inside_sphere() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_ray_intersection_before_ray_origin() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn sphere_ray_intersection_object_property() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, Sphere::default());
        assert_eq!(xs[1].object, Sphere::default());
    }

    #[test]
    fn check_sphere_default_transformation() {
        let s: Sphere = Default::default();
        assert_eq!(s.transform, Matrix::I());
    }

    #[test]
    fn change_sphere_default_transformation() {
        let mut s: Sphere = Default::default();
        let t = Matrix::get_translation_matrix(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(s.transform, t);
    }

    #[test]
    fn ray_intersect_scaled_sphere() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s: Sphere = Default::default();
        s.set_transform(Matrix::get_scaling_matrix(2.0, 2.0, 2.0));
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn ray_intersect_translated_sphere() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s: Sphere = Default::default();
        s.set_transform(Matrix::get_translation_matrix(5.0, 0.0, 0.0));
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }
}
