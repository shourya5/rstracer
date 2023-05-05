use std::cmp::Ordering;
use std::sync::Arc;

use rand::Rng;

use crate::{
    aabb::{AABB, self, box_y_compare, box_x_compare, box_z_compare},
    hitrecord::{HitRecord, Hitable},
    ray::Ray,
};
#[derive(Clone)]
pub struct BVHNode {
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
    aabb: AABB,
}

impl BVHNode {
    pub fn new(mut objects: Vec<Arc<dyn Hitable>>, time0: f32, time1: f32) -> Arc<Self> {
        let axis = (3.0 * rand::random::<f32>()).floor() as usize;

        objects.sort_unstable_by(|a, b| {
            let a_box = a.bounding_box(time0, time1).unwrap();
            let b_box = b.bounding_box(time0, time1).unwrap();
            a_box.min[axis].partial_cmp(&b_box.min[axis]).unwrap_or(Ordering::Equal)
        });

        let left: Arc<dyn Hitable>;
        let right: Arc<dyn Hitable>;
        let aabb: AABB;

        let obj_count = objects.len();

        if obj_count == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
            aabb = objects[0].bounding_box(time0, time1).unwrap();
        } else if obj_count == 2 {
            left = objects[0].clone();
            right = objects[1].clone();
            let aabb_left = objects[0].bounding_box(time0, time1).unwrap();
            let aabb_right = objects[1].bounding_box(time0, time1).unwrap();
            aabb = AABB::surrounding_box(&aabb_left, &aabb_right);
        } else {
            let split_point = obj_count / 2;
            left = BVHNode::new(objects[0..split_point].to_vec(), time0, time1);
            right = BVHNode::new(objects[split_point..].to_vec(), time0, time1);

            let aabb_left = left.bounding_box(time0, time1).unwrap();
            let aabb_right = right.bounding_box(time0, time1).unwrap();
            aabb = AABB::surrounding_box(&aabb_left, &aabb_right);
        }

        Arc::new(BVHNode { left, right, aabb })
    }
    // pub fn from_objects(objects: &mut [Arc<dyn Hitable>], time0: f32, time1: f32) -> Arc<Self> {
    //     let axis = rand::thread_rng().gen_range(0..3);
    //     let object_span = objects.len();

    //     let left: Arc<dyn Hitable>;
    //     let right: Arc<dyn Hitable>;
    //     let aabb: AABB;

    //     if object_span == 1 {
    //         left = objects[0].clone();
    //         right = objects[0].clone();
    //         aabb = objects[0].bounding_box(time0, time1).unwrap();
    //     } else if object_span == 2 {
    //         if box_compare(&objects[0], &objects[1], axis,time0,time1) {
    //             left = objects[0].clone();
    //             right = objects[1].clone();
    //         } else {
    //             left = objects[1].clone();
    //             right = objects[0].clone();
    //         }
    //         let aabb_left = left.bounding_box(time0, time1).unwrap();
    //         let aabb_right = right.bounding_box(time0, time1).unwrap();
    //         aabb = AABB::surrounding_box(&aabb_left, &aabb_right);
    //     } else {
    //         objects.sort_unstable_by(|a, b| {
    //             let a_box = a.bounding_box(time0, time1).unwrap();
    //             let b_box = b.bounding_box(time0, time1).unwrap();
    //             a_box.min[axis].partial_cmp(&b_box.min[axis]).unwrap_or(Ordering::Equal)
    //         });

    //         let mid = object_span / 2;
    //         left = BVHNode::from_objects(&mut objects[..mid], time0, time1);
    //         right = BVHNode::from_objects(&mut objects[mid..], time0, time1);

    //         let aabb_left = left.bounding_box(time0, time1).unwrap();
    //         let aabb_right = right.bounding_box(time0, time1).unwrap();
    //         aabb = AABB::surrounding_box(&aabb_left, &aabb_right);
    //     }

    //     Arc::new(BVHNode { left, right, aabb })
    // }
    pub fn from_objects(mut objects: Vec<Arc<dyn Hitable>>, time0: f32, time1: f32) -> Arc<Self> {
        let axis = rand::thread_rng().gen_range(0..3);

        objects.sort_unstable_by(|a, b| {
            let a_box = a.bounding_box(time0, time1).unwrap();
            let b_box = b.bounding_box(time0, time1).unwrap();
            a_box.min[axis].partial_cmp(&b_box.min[axis]).unwrap_or(Ordering::Equal)
        });

        let left: Arc<dyn Hitable>;
        let right: Arc<dyn Hitable>;
        let aabb: AABB;

        let object_count = objects.len();

        if object_count == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
            aabb = objects[0].bounding_box(time0, time1).unwrap();
        } else {
            let mid = object_count / 2;
            left = BVHNode::from_objects(objects[..mid].to_vec(), time0, time1);
            right = BVHNode::from_objects(objects[mid..].to_vec(), time0, time1);

            let aabb_left = left.bounding_box(time0, time1).unwrap();
            let aabb_right = right.bounding_box(time0, time1).unwrap();
            aabb = AABB::surrounding_box(&aabb_left, &aabb_right);
        }

        Arc::new(BVHNode { left, right, aabb })
    }

}


impl Hitable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }
    
        let (first, second) = if self.aabb.centroid().coords.dot(&ray.direction) < 0.0 {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };
    
        let hit_first = first.hit(ray, t_min, t_max);
    
        match hit_first {
            Some(first_rec) => {
                let hit_second = second.hit(ray, t_min, first_rec.t);
                match hit_second {
                    Some(second_rec) => Some(if first_rec.t < second_rec.t { first_rec } else { second_rec }),
                    None => Some(first_rec),
                }
            }
            None => second.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.aabb)
    }
    
   
}
fn box_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>, axis: usize, time0: f32, time1: f32) -> bool {
    let a_box = a.bounding_box(time0, time1).unwrap();
    let b_box = b.bounding_box(time0, time1).unwrap();
    a_box.min[axis] < b_box.min[axis]
}