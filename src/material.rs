use core::arch;

use crate::{
    color::Color, 
    hittable::HitRecord, 
    ray::Ray, 
    vec3::{Vec3,Point3}
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
    
}

impl Material for Dielectric {
    fn scatter(&self,
                    ray:&Ray, 
                    rec :&HitRecord,
                    attenuation: &mut Color,
                    scattered:&mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face() {1.0/self.refraction_index} else {self.refraction_index};

        let unit_direction  = ray.direction().unit_vector();
        let refracted = Vec3::refract(&unit_direction, &rec.normal(), ri);

        *scattered = Ray::new(rec.p(), refracted);

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