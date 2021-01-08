const CLIP_RANGE: (f64, f64) = (0.001, f64::INFINITY);

extern crate image;

use crate::utils::{ray::Ray, vector::*};

use super::{shapes::hit::Hittable, scene::Scene};

use rand::Rng;

fn sample_ray(r: &Ray, s: &Scene, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    match s.hit(r, CLIP_RANGE) {
        Some(hit_data) => {
            let (scattered, attenuation) = hit_data.material.scatter(r, &hit_data);
            return attenuation * sample_ray(&scattered, s, depth - 1);
        }
        None => {
            return s.environment.sky_color(r);
        }
    }
}

pub struct Sampler<'a> {
    scene: &'a Scene,
}

impl Sampler<'_> {
    pub fn new<'a>(scene: &'a Scene) -> Sampler<'a> {
        Sampler { scene }
    }

    pub fn sample(&mut self, x: u32, y: u32) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();
        let samples = self.scene.render_settings.samples_per_pixel;

        for _ in 0..samples {
            let rx = rng.gen::<f64>();
            let ry = rng.gen::<f64>();

            let u = (x as f64 + rx) / (self.scene.render_settings.img_size.0 - 1) as f64;
            let v = (y as f64 + ry) / (self.scene.render_settings.img_size.1 - 1) as f64;
            let r = self.scene.camera.get_ray(u, v);

            color += sample_ray(&r, self.scene, self.scene.render_settings.max_depth);
        }
        let avg = color / (samples as f64);

        return avg;
    }
}
