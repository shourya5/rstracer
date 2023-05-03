use std::rc::Rc;
use std::marker::{Send, Sync};
use std::sync::Arc;
use nalgebra::{Vector3, Point3};

use crate::aabb::AABB;
use crate::{material::Material, ray::Ray};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: Arc<dyn Material>,
}
pub trait Hitable : Send + Sync{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
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