use crate::{Ray, hitrecord::{Hitable, HitRecord}, aabb::AABB};
use std::{sync::Arc};

use nalgebra::{Point3, Vector3};

use crate::material::Material;

pub struct Cone {
    apex: Point3<f64>,
    height: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Cone {
    pub fn new(apex: Point3<f64>, height: f64, radius: f64, material: Arc<dyn Material>) -> Self {
        Cone {
            apex,
            height,
            radius,
            material,
        }
    }

    fn is_point_inside_cone(&self, point: &Point3<f64>) -> bool {
        let dist_sq = (point.x - self.apex.x).powi(2) + (point.z - self.apex.z).powi(2);
        let y_diff = point.y - self.apex.y;
        if y_diff >= 0.0 && y_diff <= self.height {
            let radius_at_y = (y_diff / self.height) * self.radius;
            dist_sq <= radius_at_y.powi(2)
        } else {
            false
        }
    }
}

impl Hitable for Cone {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let k = self.radius / self.height;
        let k_sq = k * k;
        let apex_to_origin = ray.origin - self.apex;

        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2) - k_sq * ray.direction.y.powi(2);
        let b = 2.0 * (apex_to_origin.x * ray.direction.x + apex_to_origin.z * ray.direction.z - k_sq * apex_to_origin.y * ray.direction.y);
        let c = apex_to_origin.x.powi(2) + apex_to_origin.z.powi(2) - k_sq * apex_to_origin.y.powi(2);

        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            for t in [t1, t2].iter() {
                if *t < t_max && *t > t_min {
                    let p = ray.point_at_parameter(*t);
                    if self.is_point_inside_cone(&p) {
                        let mut normal = (p - self.apex) / self.radius;
                        normal.y = -(self.height / self.radius);
                        return Some(HitRecord {
                            t: *t,
                            p,
                            normal,
                            material: Arc::clone(&self.material),
                        });
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let min = Point3::new(
            self.apex.x - self.radius,
            self.apex.y,
            self.apex.z - self.radius,
        );
        let max = Point3::new(
            self.apex.x + self.radius,
            self.apex.y + self.height,
            self.apex.z + self.radius,
        );
        Some(AABB::new(min, max))
    }
}
