use crate::Sphere;

// Store data for ray intersection with a object in the scene
// #[derive(Default)]
pub struct Intersection {
    pub t: f32,         // At what time hit occured
    pub object: Sphere, // TODO: use dynamic data type
}

// Store vector of all intersections
#[derive(Default)]
pub struct Intersections {
    data: Vec<Intersection>,
}

impl Intersection {
    pub fn new(t: f32, object: Sphere) -> Self {
        Self { t, object }
    }
}

// impl PartialEq for Intersection {
//     fn eq(&self, other: &Self) -> bool {
//         self.t == other.t && self.object == other.object
//     }
// }

impl Intersections {
    pub fn new() -> Self {
        Default::default()
    }

    // Self has only a vector so abstract out push
    pub fn push(&mut self, intersection: Intersection) {
        self.data.push(intersection)
    }

    // Self has only a vector so abstract out length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

}

// Self has only a vector so abstract out indexing
impl std::ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[idx]
    }
}

impl std::ops::IndexMut<usize> for Intersections {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.data[idx]
    }
}

#[cfg(test)]
mod sphere_tests {
    use super::*;
    use crate::Ray;
    use crate::{point, vector};

    #[test]
    fn intersection_creation() {
        let s = Default::default();
        let i = Intersection::new(3.5, Default::default());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn intersections_test() {
        let s: Sphere = Default::default(); // INFO: this will work if we remove `Sphere`. As compiler infers it from the next line
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let mut xs: Intersections = Default::default();
        xs.push(i1);
        xs.push(i2);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersection_returning_objects() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s: Sphere = Default::default();
        let xs = s.intersects(r);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }
}
