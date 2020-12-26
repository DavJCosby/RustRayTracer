use crate::utils::vector::Color;

pub mod ppm;
pub mod tonemapping;

pub trait ImageGenerator {
    fn new(file_path: &'static std::path::Path, w: u32, h: u32) -> Self;
    fn set_pixel(&mut self, coord_v2: (u32, u32), color_v3: Color);
    fn write(&self);
}
