pub mod camera;
pub mod materials;
pub mod shapes;

use crate::utils::{ray::Ray, vector::Color};
use materials::environment::{ColorEnvironment, Environment};
use shapes::hit::*;

pub struct Scene {
    pub components: Vec<Box<dyn Hittable>>,
    pub environment: Box<dyn Environment>,
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

impl Scene {
    pub fn new() -> Scene {
        let cmpts: Vec<Box<dyn Hittable>> = Vec::new();
        Scene {
            components: cmpts,
            environment: Box::new(ColorEnvironment {
                color: Color::new(0.7, 0.7, 0.7),
            }),
        }
    }
}
