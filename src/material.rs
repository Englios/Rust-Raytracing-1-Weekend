use core::arch;

use crate::{
    color::Color, commons::random_double, hittable::HitRecord, ray::Ray, vec3::{Point3, Vec3}
};

pub trait Material: Send + Sync {

    fn scatter(&self,
                ray:&Ray, 
                rec :&HitRecord,
                attenuation: &mut Color,
                scattered:&mut Ray) -> bool 
    {
        false
    }
}


pub struct Lambertian {
    albedo : Color
}

impl Lambertian {

    pub fn new(albedo:Color) -> Self {
        Self { albedo }
    }
    
}

impl Material for Lambertian {

    fn scatter(&self,
                    ray:&Ray, 
                    rec :&HitRecord,
                    attenuation: &mut Color,
                    scattered:&mut Ray) -> bool {
        let mut scatter_direction = rec.normal() + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal()
        }
        
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}


pub struct Metal {
    albedo : Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo:Color,fuzz:f64) -> Self {
        Self {
            albedo,
            fuzz : if fuzz < 1.0 {fuzz} else {1.0}
        }
    }
}

impl Material for Metal {
    fn scatter(&self,
                    ray:&Ray, 
                    rec :&HitRecord,
                    attenuation: &mut Color,
                    scattered:&mut Ray) -> bool {
        let mut reflected = Vec3::reflect(&ray.direction().unit_vector(), &rec.normal());
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new(rec.p(), reflected);
        *attenuation = self.albedo;
        
        scattered.direction().dot(rec.normal) > 0.0
    }
}


//Dielectric
pub struct Dielectric{
    refraction_index : f64
}

impl Dielectric {
    pub fn new(refraction_index:f64) -> Self {
        Self {
            refraction_index
        }
    }

    fn reflectance(cosine:f64 ,refraction_index:f64) -> f64 {
        // Use Schlicks Approximation

        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;

        r0 + ( 1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
    
}

impl Material for Dielectric {
    fn scatter(&self,
                    ray:&Ray, 
                    rec :&HitRecord,
                    attenuation: &mut Color,
                    scattered:&mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction  = ray.direction().unit_vector();        
        let cos_theta = f64::min((-unit_direction).dot(rec.normal()), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract 
        || (Dielectric::reflectance(cos_theta, ri) > random_double()) 
        {
            Vec3::reflect(&unit_direction, &rec.normal())
        } else {
            Vec3::refract(&unit_direction, &rec.normal(), ri)
        };
        
        *scattered = Ray::new(rec.p(), direction);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lambertian_scatter() {
        let lambertian = Lambertian::new(Color::new(0.5, 0.5, 0.5));
        let ray_in = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        let mut hit_record = HitRecord::default();
        hit_record.set_normal(Vec3::new(0.0, 0.0, -1.0));
        hit_record.set_p(Point3::new(0.0, 0.0, 1.0));

        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        assert!(lambertian.scatter(&ray_in, &hit_record, &mut attenuation, &mut scattered));
        assert_eq!(attenuation, Color::new(0.5, 0.5, 0.5));
        assert_eq!(scattered.origin(), Point3::new(0.0, 0.0, 1.0));
        
        // Verify that scattered direction is in the same hemisphere as the normal
        assert!(scattered.direction().dot(hit_record.normal()) > 0.0);
    }
}