use crate::{
    render::shapes::hit::HitData,
    utils::{ray::*, vector::*},
};

use rand::Rng;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_data: &HitData) -> (Ray, Color);
}

pub struct Lambertian {
    pub albedo: Color,
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

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (2.0 * dot(v, n) * n)
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_data: &HitData) -> (Ray, Color) {
        let reflected = reflect(ray_in.direction, hit_data.normal);
        let scattered = Ray::new(hit_data.p, reflected + self.fuzz * random_in_unit_sphere());

        let attenuation = if dot(scattered.direction, hit_data.normal) <= 0.0 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            self.albedo
        };

        return (scattered, attenuation);
    }
}

pub struct Dielectric {
    pub albedo: Color,
    pub ior: f64,
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    return r_out_perp + r_out_parallel;
}

fn shlick_reflectance(cosine: f64, ior: f64) -> f64 {
    let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2);
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_data: &HitData) -> (Ray, Color) {
        let refraction_ratio = if hit_data.front_face == true {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = ray_in.direction;
        let cos_theta = dot(-unit_direction, hit_data.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();
        let mut attenuation = Color::new(1.0, 1.0, 1.0);
        let direction =
            if cannot_refract || shlick_reflectance(cos_theta, self.ior) > rng.gen::<f64>() {
                reflect(unit_direction, hit_data.normal)
            } else {
                attenuation = self.albedo;
                refract(unit_direction, hit_data.normal, refraction_ratio)
            };

        return (Ray::new(hit_data.p, direction), attenuation);
    }
}
