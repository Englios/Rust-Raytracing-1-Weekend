use std::sync::Arc;

use crate::interval::Interval;
use crate::material::Material;
use crate::vec3::Point3;
use crate::hittable::{Hittable,HitRecord};
use crate::ray::Ray;


#[derive(Clone)]
pub struct Sphere {
    center:Point3,
    radius:f64,
    mat: Arc<dyn Material>
}

impl Sphere {
    pub fn new(center:Point3,radius:f64,material: Arc<dyn Material>) -> Self{
        Self {
            center,
            radius : f64::max(0.0,radius),
            mat:material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self,r:&Ray,t: &Interval,rec:&mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h: f64 = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0 { return false; };

        let sqrtd = discriminant.sqrt();

        //Find nearest root in acceptable range
        let mut root = (h - sqrtd) / a;
        if !t.surrounds(root){
            root = (h + sqrtd) / a;
            
            if!t.surrounds(root) {
                return false;
            }
        }

        rec.set_t(root);
        rec.set_p(r.at(rec.t()));
        let outward_normal = ((rec.p() - self.center) / self.radius).unit_vector();
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        true
    }

    
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{commons::INFINITY,vec3::Vec3};
    use crate::material::Metal;

    #[test]
    fn test_sphere_new() {
        let center = Point3::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        let material = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
        let sphere = Sphere::new(center, radius, material.clone());

        assert_eq!(sphere.center, center);
        assert_eq!(sphere.radius, radius);
    }

    #[test]
    fn test_sphere_negative_radius() {
        let center = Point3::new(0.0, 0.0, 0.0);
        let radius = -1.0;
        let material = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
        let sphere = Sphere::new(center, radius, material.clone());

        assert_eq!(sphere.radius, 0.0);
    }

    #[test]
    fn test_ray_hits_sphere(){
        let material = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, material.clone());
        let ray = Ray::new(
            Point3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        let mut rec = HitRecord::default();

        let hit = sphere.hit(&ray, &Interval::new(0.0, INFINITY), &mut rec);

        assert!(hit);
        assert_eq!(rec.t(), 4.0);
        assert_eq!(rec.p(), Point3::new(0.0, 0.0, -1.0));
        assert_eq!(rec.normal(), Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_ray_misses_sphere(){
        let material = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, material.clone());
        let ray = Ray::new(
            Point3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 1.0, 0.0)
        );
        let mut rec = HitRecord::default();

        let hit = sphere.hit(&ray,  &Interval::new(0.0, INFINITY), &mut rec);

        assert!(!hit);
    }

    #[test]
    fn test_ray_hits_sphere_from_inside() {
        let material = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, material.clone());
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0)
        );
        let mut rec = HitRecord::default();

        let hit = sphere.hit(&ray, &Interval::new(0.0, INFINITY), &mut rec);

        assert!(hit);
        assert_eq!(rec.t(), 1.0);
        assert_eq!(rec.p(), Point3::new(0.0, 0.0, -1.0));
        assert_eq!(rec.normal(), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_ray_hits_sphere_at_t_min() {
        let material = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, material.clone());
        let ray = Ray::new(
            Point3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        let mut rec = HitRecord::default();

        let hit = sphere.hit(&ray, &Interval::new(0.0, INFINITY), &mut rec);

        assert!(hit);
        assert_eq!(rec.t(), 4.0);
        assert_eq!(rec.p(), Point3::new(0.0, 0.0, -1.0));
        assert_eq!(rec.normal(), Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_ray_hits_sphere_at_t_max() {
        let material = Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -3.0), 1.0, material.clone());
        let ray = Ray::new(
            Point3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        let mut rec = HitRecord::default();

        let hit = sphere.hit(&ray, &Interval::new(0.0, 3.0), &mut rec);

        assert!(hit);
        assert_eq!(rec.t(), 1.0);
        assert_eq!(rec.p(), Point3::new(0.0, 0.0, -4.0));
        assert_eq!(rec.normal(), Vec3::new(0.0, 0.0, -1.0));
    }

}