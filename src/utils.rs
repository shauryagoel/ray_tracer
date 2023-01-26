// Utility functions or structs or traits

const EPSILON: f32 = 1e-5;

pub fn float_compare(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_compare_equal() {
        assert!(float_compare(3.3, 3.3));
    }

    #[test]
    fn float_compare_not_equal() {
        assert!(!float_compare(3.3, 3.2));
    }
}
