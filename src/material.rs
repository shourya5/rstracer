use nalgebra::{Unit, Vector3};
use rand::Rng;

use crate::{
    ray::Ray,
    util::{random_in_unit_sphere, random_unit_vector, reflect, refract, schlick},
    HitRecord,
};
use std::marker::{Send, Sync};

#[derive(Clone)]

pub enum Material {
    Lambertian { albedo: Vector3<f32> },
    Metal { albedo: Vector3<f32>, fuzz: f32 },
    Dielectric { ref_idx: f32 },
}
impl Material {
    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let scatter_direction = &hit_record.normal + random_unit_vector(hit_record.normal);
                let scattered = Ray::new(hit_record.p, scatter_direction);
                //Some((self.albedo, scattered))
                if scattered.direction.dot(&hit_record.normal) > 0.0 {
                    let light_dir = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0)); // example light direction
                    let color = blinn_phong_lamb(*albedo, ray_in, hit_record, &light_dir);
                    Some((color, scattered))
                } else {
                    None
                }
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(ray_in.direction.normalize(), hit_record.normal);
                let scattered = Ray::new(hit_record.p, reflected + *fuzz * random_in_unit_sphere());
                if scattered.direction.dot(&hit_record.normal) > 0.0 {
                    let light_dir = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0)); // example light direction
                    let color = blinn_phong_metal(*albedo, ray_in, hit_record, &light_dir);
                    return Some((color, scattered));
                } else {
                    return None;
                }
            }
            Material::Dielectric { ref_idx } => {
                let reflected = reflect(ray_in.direction, hit_record.normal);
                let view_direction = -ray_in.direction.normalize();
                let light_direction = Vector3::new(1.0, 1.0, 1.0).normalize();
                let light_color = Vector3::new(1.0, 1.0, 1.0);
                let half_vector = (view_direction + light_direction).normalize();
                let specular_intensity = hit_record.normal.dot(&half_vector).max(0.0).powf(90.0);
                let inv_ref_idx = 1.0 / ref_idx;
                let (outward_normal, ni_over_nt, cosine) =
                    if ray_in.direction.dot(&hit_record.normal) > 0.0 {
                        (
                            -hit_record.normal,
                            ref_idx,
                            ref_idx * ray_in.direction.dot(&hit_record.normal)
                                / ray_in.direction.magnitude(),
                        )
                    } else {
                        (
                            hit_record.normal,
                            &inv_ref_idx,
                            (-ray_in.direction.dot(&hit_record.normal)
                                / ray_in.direction.magnitude()),
                        )
                    };

                let scattered = if let Some(refracted) =
                    refract(ray_in.direction, outward_normal, *ni_over_nt)
                {
                    let reflect_prob = schlick(cosine, *ref_idx);
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
    }
}
fn blinn_phong_metal(
    albedo: Vector3<f32>,
    ray_in: &Ray,
    hit_record: &HitRecord,
    light_dir: &Unit<Vector3<f32>>,
) -> Vector3<f32> {
    let view_dir = -ray_in.direction.normalize();
    let halfway_dir = (light_dir.as_ref() + view_dir).normalize();

    let ambient = albedo * 0.1;
    let diffuse = albedo * hit_record.normal.dot(light_dir.as_ref()).max(0.0);
    let specular =
        Vector3::new(1.0, 1.0, 1.0) * hit_record.normal.dot(&halfway_dir).max(0.0).powf(150.0);

    return ambient + diffuse + specular;
}
fn blinn_phong_lamb(
    albedo: Vector3<f32>,
    ray_in: &Ray,
    hit_record: &HitRecord,
    light_dir: &Unit<Vector3<f32>>,
) -> Vector3<f32> {
    let view_dir = -ray_in.direction.normalize();
    let halfway_dir = (light_dir.as_ref() + view_dir).normalize();

    let ambient = albedo * 0.06;
    let diffuse = albedo * hit_record.normal.dot(light_dir.as_ref()).max(0.01);
    let specular =
        Vector3::new(1.0, 1.0, 1.0) * hit_record.normal.dot(&halfway_dir).max(0.0).powf(30.0);

    return ambient + diffuse + specular;
}
