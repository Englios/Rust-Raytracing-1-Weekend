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
    albedo : Color
}

impl Metal {
    pub fn new(albedo:Color) -> Self {
        Self {albedo}
    }
}

impl Material for Metal {
    fn scatter(&self,
                    ray:&Ray, 
                    rec :&HitRecord,
                    attenuation: &mut Color,
                    scattered:&mut Ray) -> bool {
        let reflected = Vec3::reflect(&ray.direction().unit_vector(), &rec.normal());

        *scattered = Ray::new(rec.p(), reflected);
        *attenuation = self.albedo;
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