use crate::commons::INFINITY;

#[derive(Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self { min: INFINITY, max: -INFINITY }
    }
}

impl Interval{

    pub const EMPTY: Self = Self { min: INFINITY, max: -INFINITY };
    pub const UNIVERSE: Self = Self { min: -INFINITY, max: INFINITY };

    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min,
            max
        }
    }

    //Getters
    pub fn min(self) -> f64{
        self.min
    }

    pub fn max(self) -> f64{
        self.max
    }

    pub fn size(self) -> f64{
        self.max() - self.min()
    }

    pub fn contains(self,x:f64) -> bool {
        self.min() <= x && x <= self.max()
    }
    
    pub fn surrounds(self,x:f64) -> bool {
        self.min() < x && x < self.max()
    }

    pub fn clamp(self,x:f64) -> f64 {
        if x < self.min() {
            self.min()
        } else if x > self.max() {
            self.max()
        } else {
            x
        }
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_default() {
        let interval = Interval::default();
        assert_eq!(interval.min(), INFINITY);
        assert_eq!(interval.max(), -INFINITY);
    }

    #[test]
    fn test_interval_new() {
        let interval = Interval::new(1.0, 2.0);
        assert_eq!(interval.min(), 1.0);
        assert_eq!(interval.max(), 2.0);
    }

    #[test]
    fn test_interval_size() {
        let interval = Interval::new(1.0, 3.0);
        assert_eq!(interval.size(), 2.0);
    }

    #[test]
    fn test_interval_contains() {
        let interval = Interval::new(1.0, 3.0);
        assert!(interval.contains(1.0));
        assert!(interval.contains(2.0));
        assert!(interval.contains(3.0));
        assert!(!interval.contains(0.9));
        assert!(!interval.contains(3.1));
    }

    #[test]
    fn test_interval_surrounds() {
        let interval = Interval::new(1.0, 3.0);
        assert!(interval.surrounds(2.0));
        assert!(!interval.surrounds(1.0));
        assert!(!interval.surrounds(3.0));
        assert!(!interval.surrounds(0.9));
        assert!(!interval.surrounds(3.1));
    }

    #[test]
    fn test_interval_constants() {
        assert_eq!(Interval::EMPTY.min(), INFINITY);
        assert_eq!(Interval::EMPTY.max(), -INFINITY);
        assert_eq!(Interval::UNIVERSE.min(), -INFINITY);
        assert_eq!(Interval::UNIVERSE.max(), INFINITY);
    }
}


