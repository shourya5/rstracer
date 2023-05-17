use nalgebra::{Point3, Vector3};

#[derive(Clone)]

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f32) -> Point3<f32> {
        return self.origin + (self.direction * t);
    }
    //TODO add a face at normal function that inverts the normals(outside the )
}
