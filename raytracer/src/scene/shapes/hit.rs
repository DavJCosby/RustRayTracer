use crate::utils::{ray::Ray, vector::*};

pub struct HitData {
    pub p: Point3,
    pub normal: Vec3,
    pub dist: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, dist_range: (f64, f64)) -> Option<HitData>;
}
