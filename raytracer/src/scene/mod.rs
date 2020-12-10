use hit::*;

use crate::{
    math::{ray::Ray, vector::*},
    shapes::{sphere::Sphere, *},
};

pub struct Scene {
    pub components: Vec<Box<dyn Hittable>>,
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
        Scene { components: cmpts }
    }
}

pub fn scene1() -> Scene {
    let mut scene = Scene::new();
    let sphere = Sphere::new(Point3::new(0.0, -0.05, -1.0), 0.5);
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);

    scene.components.push(Box::new(sphere));
    scene.components.push(Box::new(ground));

    return scene;
}