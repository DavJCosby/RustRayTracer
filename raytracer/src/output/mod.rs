pub mod ppm;

pub trait ImageGenerator {
    fn new(file_path: &'static std::path::Path, w: i32, h: i32) -> Self;
    fn set_pixel(&mut self, coord_v2: (i32, i32), color_v3: (f64, f64, f64));
    fn write(&self);
}