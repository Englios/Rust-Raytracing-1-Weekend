use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};


pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Self { center: Point3::default(), radius: 1.0 }
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (half_b - sqrtd) / a;
        if root <= t_min || root >= t_max {
            root = (half_b + sqrtd) / a;
            if root <= t_min || root >= t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        return true;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sphere() -> Sphere {
        let center = Point3::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        Sphere::new(center, radius)
    }

    #[test]
    fn test_sphere_hit() {  
        let sphere = create_sphere();
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::default();
        let hit = sphere.hit(&r, 0.0, f64::INFINITY, &mut rec);
        assert!(hit);
    }

    #[test]
    fn test_sphere_hit_miss() {
        let sphere = create_sphere();
        let r = Ray::new(
                Point3::new(1.0, 0.0, 0.0), 
                Vec3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::default();
        let hit = sphere.hit(&r, 0.0, f64::INFINITY, &mut rec);
        assert!(!hit);
    }
}
