use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable : Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl Default for HitRecord {
    fn default() -> Self {
        Self { p: Point3::default(), normal: Vec3::default(), t: 0.0 }
    }
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        Self { p, normal, t }
    }

    // Getters
    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }   

    pub fn t(&self) -> f64 {
        self.t
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_record_creation() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let normal = Vec3::new(4.0, 5.0, 6.0);
        let t = 7.0;
        let hit_record = HitRecord::new(p, normal, t);
        
    }

    #[test]
    fn test_hit_record_default() {
        let hit_record = HitRecord::default();
        assert_eq!(hit_record.p, Point3::default());
        assert_eq!(hit_record.normal, Vec3::default());
        assert_eq!(hit_record.t, 0.0);
    }

    #[test]
    fn test_hit_record_getters() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let normal = Vec3::new(4.0, 5.0, 6.0);
        let t = 7.0;
        let hit_record = HitRecord::new(p, normal, t);

        assert_eq!(hit_record.p(), p);
        assert_eq!(hit_record.normal(), normal);
        assert_eq!(hit_record.t(), t);
    }
}

