use crate::{
    scene::shapes::hit::HitData,
    utils::{ray::*, vector::*},
};

use rand::Rng;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_data: &HitData) -> (Ray, Color);
}

fn random_in_unit_sphere() -> Point3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        );
        if p.length() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub struct Lambertian {
    pub albedo: Color,
}
impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_data: &HitData) -> (Ray, Color) {
        let mut scatter_direction = hit_data.normal + random_in_unit_sphere();

        if scatter_direction.near_zero() {
            scatter_direction = hit_data.normal;
        }

        let scattered = Ray::new(hit_data.p, scatter_direction);
        let attenuation = self.albedo;
        return (scattered, attenuation);
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (2.0 * dot(v, n) * n)
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_data: &HitData) -> (Ray, Color) {
        let reflected = reflect(ray_in.direction, hit_data.normal);
        let scattered = Ray::new(hit_data.p, reflected + self.fuzz*random_in_unit_sphere());
        let mut attenuation = self.albedo;
        if dot(scattered.direction, hit_data.normal) <= 0.0 {
            attenuation = Color::new(0.0, 0.0, 0.0);
        }
        return (scattered, attenuation);
    }
}
