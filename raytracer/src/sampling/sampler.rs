const CLIP_RANGE: (f64, f64) = (0.001, f64::INFINITY);
const MAX_DEPTH: i32 = 50;

use crate::{
    math::{ray::Ray, vector::*},
    sampling::camera::Camera,
    scene::Scene,
    shapes::hit::Hittable,
};

use rand::{prelude::ThreadRng, Rng};

fn random_in_unit_sphere() -> Point3 {
   let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        );
        if p.length() >= 1.0 {
            continue;
        }
        return p;
    }
}

fn sample_ray(r: &Ray, s: &Scene, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    match s.hit(r, CLIP_RANGE) {
        Some(hit_data) => {
            let target = hit_data.p + hit_data.normal.unit() + random_in_unit_sphere().unit();
            let new_ray = Ray::new(hit_data.p, (target - hit_data.p).unit());
            return 0.5 * sample_ray(&new_ray, s, depth - 1);
        }
        None => {
            let t = 0.5 * (r.direction.unit().y + 1.0);
            return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        }
    }
}

pub struct Sampler<'a, 'b> {
    viewport_size: (i32, i32),
    scene: &'a Scene,
    camera: &'b Camera,
}

impl Sampler<'_, '_> {
    pub fn new<'a, 'b>(x: i32, y: i32, scene: &'a Scene, camera: &'b Camera) -> Sampler<'a, 'b> {
        Sampler {
            viewport_size: (x, y),
            scene,
            camera,
        }
    }

    pub fn take_samples(&mut self, x: i32, y: i32, samples: i32) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();

        for _ in 0..samples {
            let rx = rng.gen::<f64>();
            let ry = rng.gen::<f64>();

            let u = (x as f64 + rx) / (self.viewport_size.0 - 1) as f64;
            let v = (y as f64 + ry) / (self.viewport_size.1 - 1) as f64;
            let r = self.camera.get_ray(u, v);

            color += sample_ray(&r, self.scene, MAX_DEPTH);
        }
        let avg = color / (samples as f64); 
        return Color::new(
            avg.x.sqrt(),
            avg.y.sqrt(),
            avg.z.sqrt(),
        ) 
    }
}
