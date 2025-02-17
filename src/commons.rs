use std::f64::consts::PI as STD_PI;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = STD_PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::random_range(0.0..1.0)
}

pub fn random_double_range(min:f64,max:f64) -> f64{
    rand::random_range(min..max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degrees_to_radians() {
        assert_eq!(degrees_to_radians(0.0), 0.0);
        assert_eq!(degrees_to_radians(180.0), PI);
        assert_eq!(degrees_to_radians(90.0), PI / 2.0);
    }

    #[test]
    fn test_random_double() {
        let r = random_double();
        assert!(r >= 0.0 && r < 1.0);
    }

    #[test]
    fn test_random_double_range() {
        let min = 10.0;
        let max = 20.0;
        let r = random_double_range(min, max);
        assert!(r >= min && r < max);
    }
}