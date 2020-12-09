const CLIP_RANGE: (f64, f64) = (0.0, f64::INFINITY);

use crate::{
    math::{ray::Ray, vector::*},
    objects::hit::Hittable,
    sampling::camera::Camera,
    scene::Scene,
};

use rand::{prelude::ThreadRng, Rng};

fn sample_ray(r: &Ray, s: &Scene) -> Color {
    match s.hit(r, CLIP_RANGE) {
        Some(hit_data) => {
            return 0.5 * (hit_data.normal + Vec3::new(1.0, 1.0, 1.0));
        }
        None => {
            let unit_dir = r.direction.unit();
            let t = 0.5 * (unit_dir.y + 1.0);
            return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        }
    }
}

pub struct Sampler<'a, 'b> {
    viewport_size: (i32, i32),
    scene: &'a Scene,
    camera: &'b Camera,
    rng: ThreadRng,
}

impl Sampler<'_, '_> {
    pub fn new<'a, 'b>(
        viewport_size: (i32, i32),
        scene: &'a Scene,
        camera: &'b Camera,
    ) -> Sampler<'a, 'b> {
        Sampler {
            viewport_size,
            scene,
            camera,
            rng: rand::thread_rng(),
        }
    }

    pub fn take_samples(&mut self, xy: (i32, i32), samples: i32) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..samples {
            let rx = self.rng.gen::<f64>();
            let ry = self.rng.gen::<f64>();

            let u = (xy.0 as f64 + rx) / (self.viewport_size.0 - 1) as f64;
            let v = (xy.1 as f64 + ry) / (self.viewport_size.1 - 1) as f64;
            let r = self.camera.get_ray(u, v);

            color += sample_ray(&r, self.scene);
        }

        return color / (samples as f64);
    }
}
