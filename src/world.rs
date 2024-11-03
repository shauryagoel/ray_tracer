use crate::Light;
use crate::Matrix;
use crate::Ray;
use crate::Sphere;
use crate::{point, Color};
use crate::{Computation, Intersections};

pub struct World {
    pub light: Light,
    pub objects: Vec<Sphere>,
}

impl World {
    // Returns the intersections of the ray with all objects in the world
    // sorted by the `t` value
    fn intersect_world(&self, ray: Ray) -> Intersections {
        let mut xs: Intersections = Default::default();
        for obj in &self.objects {
            let obj_xs = obj.intersect(ray);
            xs.extend(obj_xs);
        }
        xs.sort();
        xs
    }

    // Compute the color at the intersection point via computation object
    fn shade_hit(&self, comps: &Computation) -> Color {
        comps
            .object
            .material
            .lighting(self.light, comps.point, comps.eyev, comps.normalv, false)
    }

    // Get the color at the intersection point of the ray
    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect_world(*ray);
        match xs.hit() {
            Some(i) => {
                let comp = i.prepare_computations(ray);
                self.shade_hit(&comp)
            }
            None => Color::black(),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = Light::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1: Sphere = Default::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2: Sphere = Default::default();
        s2.set_transform(Matrix::get_scaling_matrix(0.5, 0.5, 0.5));

        Self {
            light,
            objects: vec![s1, s2],
        }
    }
}

#[cfg(test)]
mod world_test {
    use super::*;
    use crate::vector;
    use crate::Intersection;

    #[test]
    fn check_default_world() {
        let w: World = Default::default();
        let light = Light::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut s1: Sphere = Default::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2: Sphere = Default::default();
        s2.set_transform(Matrix::get_scaling_matrix(0.5, 0.5, 0.5));

        assert_eq!(w.light, light);
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_world_with_ray() {
        let w: World = Default::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = w.intersect_world(r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_intersection_from_outside() {
        let w: World = Default::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[0];
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_intersection_from_inside() {
        let w = World {
            light: Light::new(point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0)),
            ..Default::default()
        };
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(&r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_at_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_at_when_ray_hits() {
        let w = World::default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_at_when_intersection_behind_ray() {
        let mut w = World::default();
        let inner = &mut w.objects[1];
        inner.material.ambient = 1.0;
        let inner_color = inner.material.color; // Needed due to Rust's borrow checker
        let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);
        assert_eq!(c, inner_color);
    }
}
