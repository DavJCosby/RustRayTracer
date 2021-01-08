use rand::{prelude::ThreadRng, Rng};

use crate::utils::{ray::Ray, vector::*};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    lens_radius: f64,
    rng: ThreadRng,
    u: Vec3,
    v: Vec3,
}

fn random_in_unit_disk(mut rng: ThreadRng) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(
        origin: Point3,
        lookat: Point3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focal_point: Point3,
    ) -> Camera {
        let focus_dist = (origin - focal_point).length();

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let vup = Vec3::new(0.0, 1.0, 0.0);

        let w = (origin - lookat).unit();
        let u = cross(vup, w).unit();
        let v = cross(w, u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        let rng = rand::thread_rng();
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            rng,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(self.rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
