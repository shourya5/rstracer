use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use nalgebra::{Point3, Vector3};
use rand::Rng;


use crate::{
    aabb::AABB,
    bvhnode::BVHNode,
    hitrecord::{HitRecord, Hitable},
    kdnode::KdNode,
    light::Light,
    ray::Ray,
};

pub fn refract(v: Vector3<f32>, n: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    //TODO: try to remove normalize call,use Unit() instead.This can be used
    //throughout the project
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

pub fn ray_color_dup(
    ray: &Ray,
    world: &KdNode,
    depth: u32,
    tmax: f32,
    lit: &Light,
) -> Vector3<f32> {
    if depth == 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit_record) = world.hit(ray, 0.001, tmax) {
        let center = world.centre();
        let scatter_result = hit_record.material.scatter(ray, &hit_record, &center, lit);
        if let Some((attenuation, scattered_ray)) = scatter_result {
            //hadamard product
            return attenuation.component_mul(&ray_color_dup(
                &scattered_ray,
                world,
                depth - 1,
                tmax,
                lit,
            ));
        } else {
            return Vector3::new(0.0, 0.0, 0.0);
        }
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t
}
pub fn is_shadowed(world: &KdNode, point: Point3<f32>, light: Point3<f32>) -> bool {
    let v = light - point;
    let distance = v.magnitude();
    let direction = v.normalize();
    let r = Ray::new(point, direction);

    if let Some(hit) = world.hit(&r, 0.001, distance) {
        hit.t < distance
    } else {
        false
    }
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
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        return None;
    }
    fn centre(&self) -> Point3<f32> {
        unimplemented!()
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
