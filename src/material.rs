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

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz: f64::min(fuzz, 1.0) }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        
        let mut reflected = Vec3::reflect(r_in.direction().unit_vector(), rec.normal);
        reflected = reflected.unit_vector() + self.fuzz * Vec3::random_unit_vector();
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        
        reflected.dot(scattered.direction()) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face() { 
            1.0 / self.refraction_index 
        } else { 
            self.refraction_index 
        };

        let unit_direction = r_in.direction().unit_vector();
        let refracted = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);

        *scattered = Ray::new(rec.p, refracted);
        true
    }
}   


