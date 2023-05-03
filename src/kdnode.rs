use crate::{Hitable, HitRecord, Ray, aabb::{AABB, surrounding_box}};
use nalgebra::{Point3, Vector3};
use std::sync::Arc;

pub struct KdNode {
    left: Option<Box<KdNode>>,
    right: Option<Box<KdNode>>,
    hitable: Option<Arc<dyn Hitable>>,
}

impl KdNode {
    pub fn new(objects: &mut [Arc<dyn Hitable>], depth: u32) -> Self {
        let axis = depth % 3;
        objects.sort_by(|a, b| {
            let aabb_a = a.bounding_box(0.0, 0.0).unwrap();
            let aabb_b = b.bounding_box(0.0, 0.0).unwrap();
            aabb_a.min[axis.try_into().unwrap()].partial_cmp(&aabb_b.min[axis.try_into().unwrap()]).unwrap()
        });

        let middle = objects.len() / 2;
        if objects.len() == 1 {
            KdNode {
                left: None,
                right: None,
                hitable: Some(objects[0].clone()),
            }
        } else if objects.len() == 2 {
            KdNode {
                left: Some(Box::new(KdNode::new(&mut objects[..middle], depth + 1))),
                right: Some(Box::new(KdNode::new(&mut objects[middle..], depth + 1))),
                hitable: None,
            }
        } else {
            KdNode {
                left: Some(Box::new(KdNode::new(&mut objects[..middle], depth + 1))),
                right: Some(Box::new(KdNode::new(&mut objects[middle..], depth + 1))),
                hitable: None,
            }
        }
    }

    pub fn from_objects(objects: Vec<Arc<dyn Hitable>>) -> Self {
        let mut objects = objects;
        KdNode::new(&mut objects, 0)
    }
}

impl Hitable for KdNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(hitable) = &self.hitable {
            return hitable.hit(ray, t_min, t_max);
        }

        let left_hit = self.left.as_ref().and_then(|l| l.hit(ray, t_min, t_max));
        let right_hit = self.right.as_ref().and_then(|r| r.hit(ray, t_min, t_max));

        match (left_hit, right_hit) {
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    Some(left)
                } else {
                    Some(right)
                }
            }
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            _ => None,
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        unimplemented!();
    }
}
fn split_sah(objects: &[Arc<dyn Hitable>], axis: usize, pos: f64) -> f64 {
    let mut left_bbox = None;
    let mut right_bbox = None;
    let mut left_count = 0;
    let mut right_count = 0;

    for object in objects {
        if let Some(bbox) = object.bounding_box(0.0, 0.0) {
            if bbox.centroid()[axis] <= pos {
                left_bbox = Some(match left_bbox {
                    Some(existing) => surrounding_box(&existing, &bbox),
                    None => bbox,
                });
                left_count += 1;
            } else {
                right_bbox = Some(match right_bbox {
                    Some(existing) => surrounding_box(&existing, &bbox),
                    None => bbox,
                });
                right_count += 1;
            }
        }
    }

    let left_area = left_bbox.map_or(0.0, |b| b.surface_area());
    let right_area = right_bbox.map_or(0.0, |b| b.surface_area());

    let total_area = left_area + right_area;

    (left_count as f64 * left_area + right_count as f64 * right_area) / total_area
}