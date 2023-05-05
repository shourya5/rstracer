use nalgebra::Point3;

pub struct Light {
    source:Point3<f32>,
    radius:f32
}


impl Light {
    // Constructor to create a new Light instance
    pub fn new(source: Point3<f32>, radius: f32) -> Self {
        Light { source, radius }
    }

    // Getter method for the source position
    pub fn source(&self) -> Point3<f32> {
        self.source
    }

    // Getter method for the radius
    pub fn radius(&self) -> f32 {
        self.radius
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