use nalgebra::{Point3, Vector3};
use std::hash::{Hash, Hasher};


pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction * t
    }
}
impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.direction == other.direction
    }
}



impl Eq for Ray {}


