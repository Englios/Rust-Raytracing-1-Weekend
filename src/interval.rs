use crate::commons::INFINITY;

pub struct Interval {
    min:f64,
    max:f64
}

impl Default for Interval{
    fn default() -> Self {
        Self {
            min : -INFINITY,
            max : INFINITY,
        }
    }
}

impl Interval {

    pub fn new(min:f64,max:f64) -> Self {
        Self {min,max}
    }

    pub fn size(&self) -> f64{
        self.max - self.min
    }

    pub fn contains(&self,x:f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self,x:f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x:f64) -> f64{
        if x < self.min { return self.min; }
        if x > self.max { return  self.max; }
        x
    }

    //Getters
    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn min(&self) -> f64 {
        self.min
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_default() {
        let interval = Interval::default();
        assert_eq!(interval.min, -INFINITY);
        assert_eq!(interval.max, INFINITY);
    }

    #[test]
    fn test_interval_contains() {
        let interval = Interval::new(0.0, 1.0);
        assert!(interval.contains(0.5));
        assert!(interval.contains(0.0));
        assert!(interval.contains(1.0));
        assert!(!interval.contains(-0.1));
        assert!(!interval.contains(1.1));
    }

    #[test]
    fn test_interval_surrounds() {
        let interval = Interval::new(0.0, 1.0);
        assert!(interval.surrounds(0.5));
        assert!(!interval.surrounds(0.0));
        assert!(!interval.surrounds(1.0));
    }

    #[test]
    fn test_interval_clamp() {
        let interval = Interval::new(0.0, 1.0);
        assert_eq!(interval.clamp(-0.5), 0.0);
        assert_eq!(interval.clamp(0.5), 0.5);
        assert_eq!(interval.clamp(1.5), 1.0);
    }
}