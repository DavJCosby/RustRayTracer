use crate::{
    utils::{ray::Ray, vector::*},
    render::materials::material::Material
};

pub struct HitData<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub dist: f32,
    pub front_face: bool,
    pub material: &'a Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, dist_range: (f32, f32)) -> Option<HitData>;
}
