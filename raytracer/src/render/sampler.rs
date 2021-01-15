const CLIP_RANGE: (f64, f64) = (0.001, f64::INFINITY);

extern crate image;

use crate::utils::{ray::Ray, vector::*};

use super::{materials::material::Scatterer, scene::Scene, shapes::hit::Hittable};

use rand::prelude::*;

fn sample_ray<const N: usize>(r: &Ray, s: &Scene::<N>, depth: u32) -> Color {
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

pub struct Sampler<'a, const N: usize> {
    scene: Scene<'a, N>,
}

impl<const N: usize> Sampler<'_, N> {
    pub fn new<'a>(scene: Scene<'a, N>) -> Sampler<'a, N> {
        Sampler { scene }
    }

    pub fn sample(&self, pos: (u32, u32), samples: u32) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..samples {
            let rx: f64 = random();
            let ry: f64 = random();

            let u = (pos.0 as f64 + rx) / (self.scene.render_settings.img_size.0 - 1) as f64;
            let v = (pos.1 as f64 + ry) / (self.scene.render_settings.img_size.1 - 1) as f64;
            let r = self.scene.camera.get_ray(u, v);

            color += sample_ray(&r, &self.scene, self.scene.render_settings.max_depth);
        }
        let avg = color / (samples as f64);

        return avg;
    }
}
