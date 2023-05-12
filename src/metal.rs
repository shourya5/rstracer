// use nalgebra::{Vector3, Unit};

// use crate::{material::Material, hitrecord::HitRecord, util::{reflect, random_in_unit_sphere}, ray::Ray};

// pub struct Metal {
//     albedo: Vector3<f32>,
//     fuzz: f32,
// }

// impl Metal {
//     pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Self {
//         Metal {
//             albedo,
//             fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
//         }
//     }
//     fn blinn_phong(&self, ray_in: &Ray, hit_record: &HitRecord, light_dir: &Unit<Vector3<f32>>) -> Vector3<f32> {
//         let view_dir = -ray_in.direction.normalize();
//         let halfway_dir = (light_dir.as_ref() + view_dir).normalize();

//         let ambient = self.albedo * 0.1;
//         let diffuse = self.albedo * hit_record.normal.dot(light_dir.as_ref()).max(0.0);
//         let specular = Vector3::new(1.0, 1.0, 1.0) * hit_record.normal.dot(&halfway_dir).max(0.0).powf(150.0);

//         return ambient + diffuse + specular;
//     }

// }

// // Helper function to reflect a vector

// // Implement the Material trait for Metal
// impl Material for Metal {
//     fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
//         let reflected = reflect(ray_in.direction.normalize(), hit_record.normal);
//         let scattered = Ray::new(hit_record.p, reflected + self.fuzz * random_in_unit_sphere());
//         if scattered.direction.dot(&hit_record.normal) > 0.0 {
//             let light_dir = Unit::new_normalize(Vector3::new(1.0, 1.0, 1.0)); // example light direction
//             let color = self.blinn_phong(ray_in, hit_record, &light_dir);
//             return Some((color, scattered))
//         }
//             else {return None}

//     }

// }
