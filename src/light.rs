use nalgebra::Point3;

#[derive(Clone, Copy)]
pub struct Light {
    pub source: Point3<f32>,
    pub radius: f32,
}

impl Light {
    // Constructor to create a new Light instance
    pub fn new(source: Point3<f32>, radius: f32) -> Self {
        Light { source, radius }
    }

    // Method to update the source position
    pub fn set_source(&mut self, new_source: Point3<f32>) {
        self.source = new_source;
    }

    // Method to update the radius
    pub fn set_radius(&mut self, new_radius: f32) {
        self.radius = new_radius;
    }
}
