use crate::{
    aabb::AABB,
    hitrecord::{HitRecord, Hitable},
    Ray,
};

use nalgebra::{Point3, Vector3};

use crate::material::Material;

pub struct Cube {
    min: Point3<f32>,
    max: Point3<f32>,
    material: Material,
}

impl Cube {
    pub fn new(min: Point3<f32>, max: Point3<f32>, material: Material) -> Self {
        Cube { min, max, material }
    }
}

impl Hitable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
                        normal[dim] =
                            (edge - self.min[dim]) * 2.0 / (self.max[dim] - self.min[dim]) - 1.0;
                        let u = (p[other_dimensions[0]] - self.min[other_dimensions[0]])
                            / (self.max[other_dimensions[0]] - self.min[other_dimensions[0]]);
                        let v = (p[other_dimensions[1]] - self.min[other_dimensions[1]])
                            / (self.max[other_dimensions[1]] - self.min[other_dimensions[1]]);
                        hit_record = Some(HitRecord {
                            t,
                            p,
                            normal,
                            u,
                            v,
                            material: (&self.material),
                        });
                    }
                }
            }
        }

        hit_record
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
    fn centre(&self) -> Point3<f32> {
        Point3::from((self.min.coords + self.max.coords) * 0.5)
    }
}
