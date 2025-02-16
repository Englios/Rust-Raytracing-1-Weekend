use std::f64::consts::PI as STD_PI;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = STD_PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
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
}