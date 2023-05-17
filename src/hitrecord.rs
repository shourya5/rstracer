use std::marker::{Send, Sync};
// use std::simd::f32x16;
use nalgebra::{Point3, Vector3};

use crate::aabb::AABB;
use crate::{material::Material, ray::Ray};

#[derive(Clone)]
pub struct HitRecord<'m> {
    // p + t * normal
    //when hit with a light ray or photon,the hit method return a ray str
    pub t: f32,
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub u: f32,
    pub v: f32,
    //static?,reduce dynamic
    pub material: &'m Material,
}
pub trait Hitable: Send + Sync {
    fn hit<'o>(&'o self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'o>>;

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;

    fn centre(&self) -> Point3<f32>;
}
