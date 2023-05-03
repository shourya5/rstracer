use crate::{Ray, hitrecord::{Hitable, HitRecord}, aabb::AABB};
use std::{sync::Arc};

use nalgebra::{Point3, Vector3};

use crate::material::Material;

pub struct Cube {
    min: Point3<f64>,
    max: Point3<f64>,
    material: Arc<dyn Material>,
}

impl Cube {
    pub fn new(min: Point3<f64>, max: Point3<f64>, material: Arc<dyn Material>) -> Self {
        Cube {
            min,
            max,
            material,
        }
    }
}

impl Hitable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut current_t = t_max;

        for dim in 0..3 {
            for edge in &[self.min[dim], self.max[dim]] {
                let t = (edge - ray.origin[dim]) / ray.direction[dim];
                if t > t_min && t < current_t {
                    let p = ray.point_at_parameter(t);
                    let other_dimensions = (0..3).filter(|x| *x != dim).collect::<Vec<_>>();

                    if p[other_dimensions[0]] >= self.min[other_dimensions[0]]
                        && p[other_dimensions[0]] <= self.max[other_dimensions[0]]
                        && p[other_dimensions[1]] >= self.min[other_dimensions[1]]
                        && p[other_dimensions[1]] <= self.max[other_dimensions[1]]
                    {
                        current_t = t;
                        let mut normal = Vector3::zeros();
                        normal[dim] = (edge - self.min[dim]) * 2.0 / (self.max[dim] - self.min[dim]) - 1.0;
                        hit_record = Some(HitRecord {
                            t,
                            p,
                            normal,
                            material: Arc::clone(&self.material),
                        });
                    }
                }
            }
        }

        hit_record
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
