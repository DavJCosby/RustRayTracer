use crate::{
    render::shapes::hit::HitData,
    utils::{ray::*, vector::*},
};

use rand::prelude::*;

pub trait Scatterer {
    fn scatter(&self, ray_in: &Ray, hit_data: &HitData) -> (Ray, Color);
}

/// * Lambertian(albedo: Color)
/// * Metal(albedo: Color, fuzz: f64)
/// * Lambertian(albedo: Color, ior: f64)
//#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { albedo: Color, ior: f64 },
}

impl Scatterer for Material {
    fn scatter(&self, ray_in: &Ray, hit_data: &HitData) -> (Ray, Color) {
        match self {
            Material::Lambertian { albedo } => lambertian(*albedo, ray_in, hit_data),
            Material::Metal { albedo, fuzz } => metal(*albedo, *fuzz, ray_in, hit_data),
            Material::Dielectric { albedo, ior } => dielectric(*albedo, *ior, ray_in, hit_data),
        }
    }
}

fn lambertian(albedo: Color, ray_in: &Ray, hit_data: &HitData) -> (Ray, Color) {
    let mut scatter_direction = hit_data.normal + random_in_unit_sphere();

    if scatter_direction.near_zero() {
        scatter_direction = hit_data.normal;
    }

    let scattered = Ray::new(hit_data.p, scatter_direction);
    return (scattered, albedo);
}

fn metal(albedo: Color, fuzz: f64, ray_in: &Ray, hit_data: &HitData) -> (Ray, Color) {
    let reflected = reflect(ray_in.direction, hit_data.normal);

    let direction = if fuzz == 0.0 {
        reflected
    } else {
        reflected + fuzz * random_in_unit_sphere()
    };

    let scattered = Ray::new(hit_data.p, direction);

    let attenuation = if dot(scattered.direction, hit_data.normal) <= 0.0 {
        Color::new(0.0, 0.0, 0.0)
    } else {
        albedo
    };

    return (scattered, attenuation);
}

fn dielectric(albedo: Color, ior: f64, ray_in: &Ray, hit_data: &HitData) -> (Ray, Color) {
    let refraction_ratio = if hit_data.front_face == true {
        1.0 / ior
    } else {
        ior
    };

    let unit_direction = ray_in.direction;
    let cos_theta = dot(-unit_direction, hit_data.normal).min(1.0);
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let cannot_refract = refraction_ratio * sin_theta > 1.0;

    let mut attenuation = Color::new(1.0, 1.0, 1.0);
    let direction = if cannot_refract || shlick_reflectance(cos_theta, ior) > random() {
        reflect(unit_direction, hit_data.normal)
    } else {
        attenuation = albedo;
        refract(unit_direction, hit_data.normal, refraction_ratio)
    };

    return (Ray::new(hit_data.p, direction), attenuation);
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

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (2.0 * dot(v, n) * n)
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
