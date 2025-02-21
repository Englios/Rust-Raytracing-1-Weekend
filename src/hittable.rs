use std::sync::Arc;

use crate::interval::Interval;
use crate::material::{Material, Metal};
use crate::vec3::{Point3,Vec3};
use crate::ray::Ray;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal : Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face:bool
}

pub trait Hittable:Sync {
    fn hit(&self,r: &Ray,t: &Interval,rec: &mut HitRecord) ->bool;
}

impl HitRecord {
    pub fn new(p:Point3,normal:Vec3,mat:Arc<dyn Material>,t:f64)-> Self {
        Self{p,normal,mat,t,front_face:false}
    }

    //Getter
    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }


    //Setter
    pub fn set_t(&mut self,t:f64) {
        self.t = t;
    }

    pub fn set_p(&mut self, p: Point3) {
        self.p = p;
    }

    pub fn set_normal(&mut self, normal:Vec3) {
        self.normal = normal;
    }


    // Determine if the normal is pointing in the same direction as the ray
    pub fn set_face_normal(&mut self,r:&Ray,outward_normal:Vec3){ 
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat: Arc::new(Metal::new(
                Vec3::new(0.0, 0.0, 0.0),
                1.0)
            ),
            t: 0.0,
            front_face: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;

    #[test]
    fn test_hit_record_creation() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let rec = HitRecord::new(p, normal, 
                                        Arc::new(
                                            Metal::new(
                                                Vec3::new(0.0, 0.0, 0.0)
                                                ,1.0)
                                        ), 
                                        1.0);

        assert_eq!(rec.p(), p);
        assert_eq!(rec.normal(), normal);
        assert_eq!(rec.t(), 1.0);
        assert!(!rec.front_face());
    }

    #[test]
    fn test_hit_record_default() {
        let rec = HitRecord::default();
        
        assert_eq!(rec.p(), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(rec.normal(), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(rec.t(), 0.0);
        assert!(!rec.front_face());
    }

    #[test]
    fn test_face_normal_front_hit() {
        let mut rec = HitRecord::default();
        let outward_normal = Vec3::new(0.0, 0.0, 1.0).unit_vector();
        // Ray pointing in -z direction (opposite to normal)
        let r = Ray::new(
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0).unit_vector()
        );

        rec.set_face_normal(&r, outward_normal);
        assert!(rec.front_face());
        assert_eq!(rec.normal(), outward_normal);
    }

    #[test]
    fn test_face_normal_back_hit() {
        let mut rec = HitRecord::default();
        let outward_normal = Vec3::new(0.0, 0.0, 1.0).unit_vector();
        // Ray pointing in -z direction (opposite to normal)
        let r = Ray::new(
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, 1.0).unit_vector()
        );

        rec.set_face_normal(&r, outward_normal);
        assert!(!rec.front_face());
        assert_eq!(rec.normal(), -outward_normal);
    }
}