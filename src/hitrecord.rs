use std::marker::{Send, Sync};
// use std::simd::f32x16;
use nalgebra::{Point3, Vector3};
use std::sync::Arc;

use crate::aabb::AABB;
use crate::{material::Material, ray::Ray};

#[derive(Clone)]
pub struct HitRecord<'m> {
    // p + t * normal
    //when hit with a light ray or photon,the hit method return a ray str
    pub t: f32,
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    //static?,reduce dynamic
    pub material: &'m Material,
}
pub trait Hitable: Send + Sync {
    fn hit<'o>(&'o self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'o>>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
// pub struct UnsafeSyncHitable {
//     hitable: Box<dyn Hitable>,
// }
// use std::ops::Deref;

// impl Deref for UnsafeSyncHitable {
//     type Target = dyn Hitable;

//     fn deref(&self) -> &Self::Target {
//         &*self.hitable
//     }
// }
// unsafe impl Sync for UnsafeSyncHitable {}
