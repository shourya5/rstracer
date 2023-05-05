use nalgebra::{Vector3, Unit};

use crate::{material::Material, ray::Ray, hitrecord::HitRecord, util::random_unit_vector};

pub struct Lambertian {
    albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Lambertian { albedo }
    }
    fn blinn_phong(&self, ray_in: &Ray, hit_record: &HitRecord, light_dir: &Unit<Vector3<f32>>) -> Vector3<f32> {
        let view_dir = -ray_in.direction.normalize();
        let halfway_dir = (light_dir.as_ref() + view_dir).normalize();

        let ambient = self.albedo * 0.06;
        let diffuse = self.albedo * hit_record.normal.dot(light_dir.as_ref()).max(0.01);
        let specular = Vector3::new(1.0, 1.0, 1.0) * hit_record.normal.dot(&halfway_dir).max(0.0).powf(30.0);

        return ambient + diffuse + specular;
    }
}

impl Material for Lambertian {
    // fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
    //     let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
    //     let scattered = Ray::new(hit_record.p, target - hit_record.p);
    //     Some((self.albedo, scattered))
    // }

    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let scatter_direction = &hit_record.normal + random_unit_vector(hit_record.normal);
        let scattered = Ray::new(hit_record.p, scatter_direction);
        //Some((self.albedo, scattered))
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            let light_dir = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0)); // example light direction
            let color = self.blinn_phong(ray_in, hit_record, &light_dir);
            Some((color, scattered))
        } else {
            None
        }
    }
}