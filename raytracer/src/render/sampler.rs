const CLIP_RANGE: (f64, f64) = (0.001, f64::INFINITY);

extern crate image;

use crate::{
    scene::{camera::Camera, shapes::hit::Hittable, Scene},
    utils::{ray::Ray, vector::*},
};

use rand::Rng;

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

fn sample_ray(r: &Ray, s: &Scene, depth: u32) -> Color {
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
            return s.environment.sky_color(r);
            /*
            let sky_uv = get_sky_uv(r.direction.unit());
            //println!("{:?} and {:?}", sky_uv, r);
            let idx = uv_to_pixel_index(sky_uv, s.environment_size) as usize;
            let c = match s.environment.get(idx) {
                Some(x) => x,
                None => {panic!("couldn't get from env vec: {}", idx)}
            };
            Color::new(c.0[0] as f64, c.0[1] as f64, c.0[2] as f64)
            /*let t = 0.5 * (r.direction.unit().y + 1.0);
            return (1.0 - t) * Color::new(1.3, 0.0, 0.0) + t * Color::new(0.13, 0.25, 1.75);
            */
            */
        }
    }
}

pub struct Sampler<'a, 'b> {
    viewport_size: (u32, u32),
    scene: &'a Scene,
    camera: &'b Camera,
    pub max_depth: u32,
}

impl Sampler<'_, '_> {
    pub fn new<'a, 'b>(x: u32, y: u32, scene: &'a Scene, camera: &'b Camera) -> Sampler<'a, 'b> {
        Sampler {
            viewport_size: (x, y),
            scene,
            camera,
            max_depth: 50,
        }
    }

    pub fn take_samples(&mut self, x: u32, y: u32, samples: u32) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let mut rng = rand::thread_rng();

        for _ in 0..samples {
            let rx = rng.gen::<f64>();
            let ry = rng.gen::<f64>();

            let u = (x as f64 + rx) / (self.viewport_size.0 - 1) as f64;
            let v = (y as f64 + ry) / (self.viewport_size.1 - 1) as f64;
            let r = self.camera.get_ray(u, v);

            color += sample_ray(&r, self.scene, self.max_depth);
        }
        let avg = color / (samples as f64);

        return avg;
    }
}
