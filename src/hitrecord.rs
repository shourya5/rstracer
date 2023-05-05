use std::rc::Rc;
use std::marker::{Send, Sync};
// use std::simd::f32x16;
use std::sync::Arc;
use nalgebra::{Vector3, Point3};

use crate::aabb::AABB;
use crate::{material::Material, ray::Ray};

#[derive(Clone)]
pub struct HitRecord {
    // p + t * normal
    //when hit with a light ray or photon,the hit method return a ray str
    pub t: f32,
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub material: Arc<dyn Material>,
}
pub trait Hitable : Send + Sync{

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
pub struct UnsafeSyncHitable {
    hitable: Box<dyn Hitable>,
}
use std::ops::Deref;

impl Deref for UnsafeSyncHitable {
    type Target = dyn Hitable;

    fn deref(&self) -> &Self::Target {
        &*self.hitable
    }
}
unsafe impl Sync for UnsafeSyncHitable {}