use crate::{Ray, hitrecord::{Hitable, HitRecord}, aabb::AABB};
use std::{sync::Arc};

use nalgebra::{Point3, Vector3};

use crate::material::Material;

pub struct Sphere {
    center: Point3<f32>,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}


impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let oc = ray.origin - self.center;
    //squared norm or magnitude
    let a = ray.direction.magnitude_squared();
    let b = oc.dot(&ray.direction);
    let c = oc.magnitude_squared() - (self.radius * self.radius);
    let discriminant = b * b - a * c;
    if discriminant > 0.0 {
        let t = (-b - discriminant.sqrt()) / a;
        if t < t_max && t > t_min {
            let p = ray.point_at_parameter(t);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord {
                t,
                p,
                normal,
                material: Arc::clone(&self.material),
            });
        }
    }
    None
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = self.center - Vector3::new(self.radius, self.radius, self.radius);
        let max = self.center + Vector3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(min, max))
    }
}