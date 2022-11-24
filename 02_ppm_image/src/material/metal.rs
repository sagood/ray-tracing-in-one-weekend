use crate::model::{hit::HitRecord, ray::Ray, vec3::Vec3};

use super::material::Material;

pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: &Vec3) -> Self {
        Self {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.dir().unit_vector().reflect(&rec.normal);
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo.clone();
        scattered.dir().dot(&rec.normal) > 0.0
    }
}
