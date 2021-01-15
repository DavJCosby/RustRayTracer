use super::{
    camera::Camera,
    materials::environment::Environment,
    shapes::{hit::*, Shape},
};

use crate::{scenes::RenderSettings, utils::ray::Ray};

pub struct Scene<'a, const NUM_COMPONENTS: usize> {
    pub components: [Shape; NUM_COMPONENTS],
    pub environment: Environment<'a>,
    pub camera: Camera,
    pub render_settings: RenderSettings,
}

unsafe impl<const NUM_COMPONENTS: usize> Send for Scene<'_, NUM_COMPONENTS> {}

impl<const NUM_COMPONENTS: usize> Hittable for Scene<'_, NUM_COMPONENTS> {
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
