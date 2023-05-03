use nalgebra::Vector3;

use crate::{ray::Ray, HitRecord};
use std::marker::{Send, Sync};

pub trait Material : Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vector3<f64>, Ray)>;
}