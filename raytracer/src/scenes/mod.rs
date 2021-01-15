pub mod cover_scene;
pub mod scene1;

pub struct RenderSettings {
    pub img_size: (u32, u32),
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub threads: u32,
}
