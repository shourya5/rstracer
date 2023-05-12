use nalgebra::Point3;

use crate::{hitrecord::Hitable, ray::Ray};

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl AABB {
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Self {
        AABB { min, max }
    }
    pub fn centroid(&self) -> Point3<f32> {
        let centroid_vector = (self.min.coords + self.max.coords) / 2.0;
        Point3::from(centroid_vector)
    }
    pub fn union(&self, other: &AABB) -> AABB {
        surrounding_box(self, other)
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let tmin = if t0 > t_min { t0 } else { t_min };
            let tmax = if t1 < t_max { t1 } else { t_max };

            if tmax <= tmin {
                return false;
            }
        }
        true
    }
    pub fn surrounding_box(a: &AABB, b: &AABB) -> AABB {
        let small = Point3::new(
            a.min.x.min(b.min.x),
            a.min.y.min(b.min.y),
            a.min.z.min(b.min.z),
        );

        let big = Point3::new(
            a.max.x.max(b.max.x),
            a.max.y.max(b.max.y),
            a.max.z.max(b.max.z),
        );

        AABB::new(small, big)
    }
    pub fn maximum_extent(&self) -> usize {
        let extent = self.max - self.min;
        if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        }
    }
    pub fn surface_area(&self) -> f32 {
        let diff = self.max - self.min;
        2.0 * (diff.x * diff.y + diff.x * diff.z + diff.y * diff.z)
    }
}
pub fn box_x_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> std::cmp::Ordering {
    if let (Some(a_box), Some(b_box)) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        a_box
            .min
            .x
            .partial_cmp(&b_box.min.x)
            .unwrap_or(std::cmp::Ordering::Equal)
    } else {
        panic!("No bounding box in BVHNode constructor.");
    }
}

pub fn box_y_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> std::cmp::Ordering {
    if let (Some(a_box), Some(b_box)) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        a_box
            .min
            .y
            .partial_cmp(&b_box.min.y)
            .unwrap_or(std::cmp::Ordering::Equal)
    } else {
        panic!("No bounding box in BVHNode constructor.");
    }
}

pub fn box_z_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> std::cmp::Ordering {
    if let (Some(a_box), Some(b_box)) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        a_box
            .min
            .z
            .partial_cmp(&b_box.min.z)
            .unwrap_or(std::cmp::Ordering::Equal)
    } else {
        panic!("No bounding box in BVHNode constructor.");
    }
}
pub fn surrounding_box(a: &AABB, b: &AABB) -> AABB {
    let small = Point3::new(
        a.min.x.min(b.min.x),
        a.min.y.min(b.min.y),
        a.min.z.min(b.min.z),
    );

    let big = Point3::new(
        a.max.x.max(b.max.x),
        a.max.y.max(b.max.y),
        a.max.z.max(b.max.z),
    );

    AABB::new(small, big)
}
