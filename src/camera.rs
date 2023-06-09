use crate::{ray::Ray, util::random_in_unit_disk};

use nalgebra::{Point3, Vector3};

#[derive(Copy, Clone,Debug)]
pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    lens_radius: f64,
}
impl Camera {
    pub fn new(
        origin: Point3<f64>,
        look_at: Point3<f64>,
        vup: Vector3<f64>,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset;
        Ray::new(self.origin + offset, direction)
    }
}

