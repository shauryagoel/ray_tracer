// Utility functions or structs or traits

const EPSILON: f32 = 1e-5;

trait PartialEq {
    fn eq(&self, other: &f32) -> bool;
    fn ne(&self, other: &f32) -> bool;
}

impl PartialEq for f32 {
    fn eq(&self, other: &f32) -> bool {
        (self - other).abs() < EPSILON
    }

    fn ne(&self, other: &f32) -> bool {
        (self - other).abs() >= EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_compare_equal() {
        assert!(3.3 == 3.3);
    }

    #[test]
    fn float_compare_not_equal() {
        assert!(3.3 != 3.2);
    }
}
