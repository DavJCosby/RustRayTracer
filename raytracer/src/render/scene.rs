use super::{camera::Camera, materials::environment::Environment, shapes::hit::*};

use crate::{scenes::RenderSettings, utils::ray::Ray};

pub struct Scene {
    pub components: Vec<Box<dyn Hittable>>,
    pub environment: Box<dyn Environment>,
    pub camera: Camera,
    pub render_settings: RenderSettings,
}

impl Hittable for Scene {
    fn hit(&self, r: &Ray, dist_range: (f64, f64)) -> Option<HitData> {
        let mut closest_so_far = dist_range.1;
        let mut hit_data_option: Option<HitData> = None;

        for obj in self.components.iter() {
            if let Some(hit_data) = obj.hit(r, (dist_range.0, closest_so_far)) {
                closest_so_far = hit_data.dist;
                hit_data_option = Some(hit_data);
            }
        }
        return hit_data_option;
    }
}
