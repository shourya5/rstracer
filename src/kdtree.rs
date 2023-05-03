// use std::sync::Arc;
// use nalgebra::{Point3, Vector3};
// use crate::{hitrecord::{Hitable, HitRecord}, ray::Ray, aabb::AABB};

// pub enum KdNode {
//     Empty,
//     Leaf(Vec<Arc<dyn Hitable>>),
//     Node {
//         axis: usize,
//         pos: f64,
//         left: Box<KdNode>,
//         right: Box<KdNode>,
//     },
// }
// // 
// pub struct KdTree {
//     pub(crate) root: KdNode,
// }

// impl KdTree {
//     pub fn new(objects: &mut [Arc<dyn Hitable>], time0: f64, time1: f64) -> Self {
//         let root = Self::build(objects, objects.len(), time0, time1);
//         KdTree { root }
//     }

//     pub fn build(objects: &mut [Arc<dyn Hitable>], n: usize, time0: f64, time1: f64) -> KdNode {
//         if n == 0 {
//             KdNode::Empty
//         } else if n <= 4 {
//             KdNode::Leaf(objects.to_vec())
//         } else {
//             let (bounds, centroid_bounds) = Self::calculate_bounds(objects, time0, time1);
//             let axis = centroid_bounds.maximum_extent();
//             let mut mid = n / 2;

//             if centroid_bounds.max[axis] == centroid_bounds.min[axis] {
//                 KdNode::Leaf(objects.to_vec())
//             } else {
//                 objects.sort_unstable_by(|a, b| {
//                     let abox = a.bounding_box(time0, time1).unwrap();
//                     let bbox = b.bounding_box(time0, time1).unwrap();
//                     abox.min[axis].partial_cmp(&bbox.min[axis]).unwrap()
//                 });

//                 let (left, right) = objects.split_at_mut(mid);
//                 let left_node = Self::build(left, mid, time0, time1);
//                 let right_node = Self::build(right, n - mid, time0, time1);

//                 KdNode::Node {
//                     axis,
//                     pos: centroid_bounds.min[axis],
//                     left: Box::new(left_node),
//                     right: Box::new(right_node),
//                 }
//             }
//         }
//     }

//     pub fn calculate_bounds(objects: &[Arc<dyn Hitable>], time0: f64, time1: f64) -> (AABB, AABB) {
//         let bounds = objects.iter().fold(None, |bounds, object| {
//             if let Some(object_bounds) = object.bounding_box(time0, time1) {
//                 if let Some(current_bounds) = bounds {
//                     Some(AABB::surrounding_box(&current_bounds, &object_bounds))
//                 } else {
//                     Some(object_bounds)
//                 }
//             } else {
//                 bounds
//             }
//         });

//         let centroid_bounds = objects.iter().fold(None, |bounds, object| {
//             if let Some(object_bounds) = object.bounding_box(time0, time1) {
//                 let centroid = object_bounds.centroid();
//                 if let Some(current_bounds) = bounds {
//                     Some(AABB::surrounding_box(&current_bounds, &AABB::new(centroid, centroid)))
//                 } else {
//                     Some(AABB::new(centroid, centroid))
//                 }
//             } else {
//                 bounds
//             }
//         });

//         (
//             bounds.expect("No bounding box in KdTree::calculate_bounds."),
//             centroid_bounds.expect("No bounding box in KdTree::calculate_bounds."),
//         )
//     }
//     pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         Self::hit_node(&self.root, ray, t_min, t_max)
//     }
    
//     pub fn hit_node(node: &KdNode, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         match node {
//             KdNode::Empty => None,
//             KdNode::Leaf(objects) => objects.iter().fold(None, |closest_hit, object| {
//                 if let Some(hit) = object.hit(ray, t_min, t_max) {
//                     Some(hit)
//                 } else {
//                     closest_hit
//                 }
//             }),
//             KdNode::Node { axis, pos, left, right } => {
//                 let t_plane = (*pos - ray.origin[*axis]) / ray.direction[*axis];
//                 let (first, second) = if ray.direction[*axis] < 0.0 {
//                     (right, left)
//                 } else {
//                     (left, right)
//                 };
    
//                 if t_plane > t_max {
//                     Self::hit_node(first, ray, t_min, t_max)
//                 } else if t_plane < t_min {
//                     Self::hit_node(second, ray, t_min, t_max)
//                 } else {
//                     let hit1 = Self::hit_node(first, ray, t_min, t_plane);
//                     let hit2 = Self::hit_node(second, ray, t_plane, t_max);
//                     hit1.or(hit2)
//                 }
//             }
//         }
//     }
    

// }
// impl Hitable for KdTree {
//     fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//     self.hit(ray, t_min, t_max)
//     }
//     fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
//         // let (bounds, _) = Self::calculate_bounds(&self.objects, t0, t1);
//         // Some(bounds)
//         unimplemented!("Bounding box call")
//     }
// }