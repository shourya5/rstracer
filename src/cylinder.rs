use crate::{
    aabb::AABB,
    hitrecord::{HitRecord, Hitable},
    Ray,
};
use std::sync::Arc;

use nalgebra::Point3;

use crate::material::Material;

pub struct Cylinder {
    base_center: Point3<f32>,
    height: f32,
    radius: f32,
    material: Material,
}

impl Cylinder {
    pub fn new(base_center: Point3<f32>, height: f32, radius: f32, material: Material) -> Self {
        Cylinder {
            base_center,
            height,
            radius,
            material,
        }
    }

    fn is_point_inside_cylinder(&self, point: &Point3<f32>) -> bool {
        let dist_sq =
            (point.x - self.base_center.x).powi(2) + (point.z - self.base_center.z).powi(2);
        dist_sq < self.radius.powi(2)
            && point.y >= self.base_center.y
            && point.y <= self.base_center.y + self.height
    }
}

impl Hitable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let base_to_origin = ray.origin - self.base_center;
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
        let b = 2.0 * (base_to_origin.x * ray.direction.x + base_to_origin.z * ray.direction.z);
        let c = base_to_origin.x.powi(2) + base_to_origin.z.powi(2) - self.radius.powi(2);

        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            for t in [t1, t2].iter() {
                if *t < t_max && *t > t_min {
                    let p = ray.point_at_parameter(*t);
                    if self.is_point_inside_cylinder(&p) {
                        let mut normal = (p - self.base_center) / self.radius;
                        normal.y = 0.0;
                        return Some(HitRecord {
                            t: *t,
                            p,
                            normal,
                            material: &self.material,
                        });
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = Point3::new(
            self.base_center.x - self.radius,
            self.base_center.y,
            self.base_center.z - self.radius,
        );
        let max = Point3::new(
            self.base_center.x + self.radius,
            self.base_center.y + self.height,
            self.base_center.z + self.radius,
        );
        Some(AABB::new(min, max))
    }
}
