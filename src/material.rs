use crate::commons::{ray::Ray, color::Color};
use crate::hittable::HitRecord;
use crate::commons::vec3::Vec3;

pub trait Material: Send + Sync { 
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self { 
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, 
                r_in: &Ray, 
                rec: &HitRecord, 
                attenuation: &mut Color, 
                scattered: &mut Ray
    ) -> bool {
        let mut scatter_direction = rec.normal 
                                    + Vec3::random_unit_vector();
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

