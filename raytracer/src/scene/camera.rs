use crate::utils::{ray::Ray, vector::*};

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new(w: f64, h: f64) -> Camera {
        let aspect_ratio = w / h;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}