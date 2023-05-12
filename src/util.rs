use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use nalgebra::{Point3, Vector3};
use rand::Rng;
use rayon::prelude::ParallelIterator;

use crate::{
    aabb::AABB,
    bvhnode::BVHNode,
    hitrecord::{HitRecord, Hitable},
    kdnode::KdNode,
    light::{self, Light},
    ray::Ray,
};

pub fn refract(v: Vector3<f32>, n: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}
#[inline]
pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}
// Helper function to calculate Schlick's approximation
#[inline]
pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

// Implement the Material trait for Dielectric

pub fn ray_color_dup(ray: &Ray, world: &KdNode, depth: u32, tmax: f32) -> Vector3<f32> {
    if depth == 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    //let l = Light::new(Point3::new(0.0, 0.0, 14.0),0.01);
    // if let Some(hit_record) = world.hit(ray, 0.001, f32::INFINITY) {
    //     if let Some(scatter_record) = hit_record.material.scatter(ray, &hit_record) {
    //         return scatter_record.attenuation * ray_color(&scatter_record.scattered, world, depth - 1);
    //     }
    //     // if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
    //     //     return attenuation.component_mul(&color(&scattered, world, depth - 1));
    //     // }
    //     return Vector3::new(0.0, 0.0, 0.0);
    // }
    if let Some(hit_record) = world.hit(ray, 0.001, tmax) {
        let scatter_result = hit_record.material.scatter(ray, &hit_record);
        if let Some((attenuation, scattered_ray)) = scatter_result {
            //let shadow = is_in_shadow(world, &hit_record.p, &l);
            // return attenuation.component_mul( &ray_color(&scattered_ray, world, depth - 1));

            //hadamard product
            return attenuation.component_mul(&ray_color_dup(
                &scattered_ray,
                world,
                depth - 1,
                tmax,
            ));
        } else {
            return Vector3::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
}

// pub fn ray_color_iter(ray: &Ray, world: &KdNode, max_depth: u32, tmax: f32) -> Vector3<f32> {
//     let mut result_color = Arc::new( Mutex::new(Vector3::new(1.0, 1.0, 1.0)));
//     let mut current_ray = Some(ray.clone());
//     let mut depth = max_depth;

//     rayon::iter::repeat(())
//         .take(max_depth as usize)
//         .for_each(|_| {
//             if let Some(ref r) = current_ray {
//                 if let Some(hit_record) = world.hit(r, 0.001, tmax) {
//                     if let Some((attenuation, scattered_ray)) = hit_record.material.scatter(r, &hit_record) {
//                         result_color = Arc::new(Mutex::new((*&result_color).lock().unwrap().component_mul(&attenuation)));
//                         current_ray = Some(scattered_ray);
//                         depth -= 1;
//                         return;
//                     } else {
//                         result_color = Arc::new(Mutex::new(Vector3::new(0.0, 0.0, 0.0)));
//                         return;
//                     }
//                 } else {
//                     let unit_direction = r.direction.normalize();
//                     let t = 0.5 * (unit_direction.y + 1.0);
//                     let background_color = Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t;
//                     result_color = Arc::new(Mutex::new((*result_color).lock().unwrap().component_mul(&background_color)));
//                     return;
//                 }
//             }
//         });

//     return *(*&result_color).lock().unwrap()
// }

// pub fn ray_color(ray: &Ray, world: &Arc<BVHNode>, depth: u32) -> Vector3<f32> {
//     if depth == 0 {
//         return Vector3::new(0.0, 0.0, 0.0);
//     }
//     let l = Light::new(Point3::new(0.0, 0.0, 14.0), 0.01);

//     if let Some(hit_record) = world.hit(ray, 0.001, f32::INFINITY) {
//         let scatter_result = hit_record.material.scatter(ray, &hit_record);
//         if let Some((attenuation, scattered_ray)) = scatter_result {
//             // let shadow = is_in_shadow(world, &hit_record.p, &l);
//             return attenuation.component_mul(&ray_color(&scattered_ray, world, depth - 1));
//         }
//         return Vector3::new(0.0, 0.0, 0.0);
//     }

//     let unit_direction = ray.direction.normalize();
//     let t = 0.5 * (unit_direction.y + 1.0);
//     Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
// }
pub fn ray_color(
    ray: &Ray,
    world: &Arc<BVHNode>,
    depth: u32,
    background_cache: &Mutex<HashMap<(i32, i32), Vector3<f32>>>,
) -> Vector3<f32> {
    if depth == 0 || random_f32() < 0.001 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    //let l = Light::new(Point3::new(0.0, 0.0, 14.0), 0.01);

    if let Some(hit_record) = world.hit(ray, 0.001, f32::INFINITY) {
        let scatter_result = hit_record.material.scatter(ray, &hit_record);
        if let Some((attenuation, scattered_ray)) = scatter_result {
            let color = attenuation.component_mul(&ray_color(
                &scattered_ray,
                world,
                depth - 1,
                background_cache,
            ));

            return color;
        }

        return Vector3::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    //let color = Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t;
    //memo.insert(*ray, color);
    let color = {
        let mut cache = background_cache.lock().unwrap();
        let key = (
            (unit_direction.x * 1000.0) as i32,
            (unit_direction.y * 1000.0) as i32,
        );
        let quantized_t = (t * 100.0).round() as i32;

        if let Some(color) = cache.get(&(quantized_t, quantized_t)) {
            *color
        } else {
            let new_color =
                Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t;
            cache.insert((quantized_t, quantized_t), new_color);
            new_color
        }
    };
    color
}
impl Hitable for Vec<Arc<dyn Hitable>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = t_max;

        for object in self {
            if let Some(hit) = object.hit(ray, t_min, closest_t) {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        return None;
    }
}
#[inline]
pub fn random_f32() -> f32 {
    rand::thread_rng().gen_range(0.0..1.0)
}
#[inline]
pub fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vector3::new(1.0, 1.0, 1.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}
#[inline]
pub fn random_unit_vector(normal: Vector3<f32>) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    loop {
        let random_vector = Vector3::new(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
        );
        if random_vector.magnitude_squared() >= 1.0 {
            continue;
        }
        if random_vector.dot(&normal) > 0.0 {
            return random_vector.normalize();
        }
    }
}
#[inline]
pub fn random_in_unit_disk() -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}
// fn is_in_shadow(world: &Vec<Box<dyn Hitable>>, point: &Point3<f32>, light: &Light) -> bool {
//     let light_direction = (light.source() - *point).normalize();
//     let shadow_ray = Ray::new(*point, light_direction);

//     let t_max = (light.source() - *point).magnitude();

//     if let Some(shadow_hit_record) = world.hit(&shadow_ray, 0.001, t_max) {
//         true
//     } else {
//         false
//     }
// }
