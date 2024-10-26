use crate::Intersections;
use crate::Light;
use crate::Matrix;
use crate::Ray;
use crate::Sphere;
use crate::{point, Color};

struct World {
    light: Light,
    objects: Vec<Sphere>,
}

impl World {
    // Returns the intersections of the ray with all objects in the world
    // sorted by the `t` value
    pub fn intersect_world(&self, ray: Ray) -> Intersections {
        let mut xs: Intersections = Default::default();
        for obj in &self.objects {
            let obj_xs = obj.intersect(ray);
            xs.extend(obj_xs);
        }
        xs.sort();
        xs
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
}
