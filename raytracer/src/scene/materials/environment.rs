use crate::utils::{ray::Ray, vector::*};
use image::Rgb;

const PI: f64 = std::f64::consts::PI;
const TAU: f64 = 2.0 * PI;

pub trait Environment {
    fn sky_color(&self, r: &Ray) -> Color;
}

pub struct ColorEnvironment {
    pub color: Color,
}

impl Environment for ColorEnvironment {
    fn sky_color(&self, r: &Ray) -> Color {
        self.color
    }
}


pub struct TexturedEnvironment {
    pub texture: Vec<Rgb<f32>>,
    pub size: (u32, u32),
    pub brightness: f64,
}

fn get_sky_uv(direction: Vec3) -> (f64, f64) {
    let x = ((TAU + direction.z.atan2(-(direction.x))) % TAU) / TAU;
    let y = (direction.y + 1.0) / 2.0;
    return (x, y);
}

fn uv_to_pixel_index(uv: (f64, f64), size: (u32, u32)) -> u32 {
    let x = (uv.0 * size.0 as f64) as u32;
    let y = (uv.1 * size.1 as f64) as u32;
    return x + (size.1 - 1 - y) * size.0;
}

impl Environment for TexturedEnvironment {
    fn sky_color(&self, r: &Ray) -> Color {
        let sky_uv = get_sky_uv(r.direction.unit());
        let idx = uv_to_pixel_index(sky_uv, self.size) as usize;
        let c = self.texture.get(idx).unwrap().0;

        Color::new(c[0] as f64, c[1] as f64, c[2] as f64) * self.brightness
    }
}
