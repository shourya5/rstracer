use nalgebra::Point3;

pub struct Light {
    source:Point3<f64>,
    radius:f64
}


impl Light {
    // Constructor to create a new Light instance
    pub fn new(source: Point3<f64>, radius: f64) -> Self {
        Light { source, radius }
    }

    // Getter method for the source position
    pub fn source(&self) -> Point3<f64> {
        self.source
    }

    // Getter method for the radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    // Method to update the source position
    pub fn set_source(&mut self, new_source: Point3<f64>) {
        self.source = new_source;
    }

    // Method to update the radius
    pub fn set_radius(&mut self, new_radius: f64) {
        self.radius = new_radius;
    }
}