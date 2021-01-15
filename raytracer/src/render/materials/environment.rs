use crate::utils::{ray::Ray, vector::*};

const PI: f64 = std::f64::consts::PI;
const TAU: f64 = 2.0 * PI;

pub enum Environment<'a> {
    ColorEnvironment {
        color: Color,
    },
    DefaultSkyEnvironment,
    HDRIEnvironment {
        texture: &'a [Color],
        size: (u32, u32),
        brightness: f64,
    },
}
impl Environment<'_> {
    pub fn sky_color(&self, r: &Ray) -> Color {
        match self {
            Environment::ColorEnvironment { color } => *color,
            Environment::DefaultSkyEnvironment => default_sky_environment(r),
            Environment::HDRIEnvironment {
                texture,
                size,
                brightness,
            } => hdri_environment(*texture, *size, *brightness, r),
        }
    }
}

fn default_sky_environment(r: &Ray) -> Color {
    let d = r.direction;
    let t = 0.5 * (d.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn hdri_environment<'a>(texture: &[Color], size: (u32, u32), brightness: f64, r: &Ray) -> Color {
    let sky_uv = get_sky_uv(r.direction);
    let idx = uv_to_pixel_index(sky_uv, size) as usize;
    let c = texture.get(idx).unwrap();

    *c * brightness
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
