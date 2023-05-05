use nalgebra::{Vector3, Unit};
use rand::Rng;

use crate::{material::Material, ray::Ray, hitrecord::HitRecord, util::{reflect, schlick, refract}};



pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }
    
    
    
}
impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let reflected = reflect(ray_in.direction, hit_record.normal);
        let view_direction = -ray_in.direction.normalize();
        let light_direction = Vector3::new(1.0, 1.0, 1.0).normalize();
        let light_color = Vector3::new(1.0, 1.0, 1.0);
        let half_vector = (view_direction + light_direction).normalize();
        let specular_intensity = hit_record.normal.dot(&half_vector).max(0.0).powf(90.0);
        let (outward_normal, ni_over_nt, cosine) = if ray_in.direction.dot(&hit_record.normal) > 0.0 {
            (
                -hit_record.normal,
                self.ref_idx,
                self.ref_idx * ray_in.direction.dot(&hit_record.normal) / ray_in.direction.magnitude(),
            )
        } else {
            (
                hit_record.normal,
                1.0 / self.ref_idx,
                -ray_in.direction.dot(&hit_record.normal) / ray_in.direction.magnitude(),
            )
        };

        let scattered = if let Some(refracted) = refract(ray_in.direction, outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_idx);
            if rand::thread_rng().gen::<f32>() < reflect_prob {
                Ray::new(hit_record.p, reflected)
            } else {
                Ray::new(hit_record.p, refracted)
            }
        } else {
            Ray::new(hit_record.p, reflected)
        };
        let color = Vector3::new(1.0, 1.0, 1.0) + specular_intensity * light_color;
        Some((color, scattered))
    }
    
    
}