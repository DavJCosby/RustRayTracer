pub mod hit;

use crate::{
    render::materials::material::*,
    utils::{ray::Ray, vector::*},
};
use hit::*;

pub enum Shape {
    Sphere {
        center: Point3,
        radius: f64,
        material: Material,
    },
}

impl Hittable for Shape {
    fn hit(&self, r: &Ray, dist_range: (f64, f64)) -> Option<HitData> {
        match self {
            Shape::Sphere {
                center,
                radius,
                material,
            } => sphere_hit(*center, *radius, material, r, dist_range),
        }
    }
}

fn sphere_hit<'a>(
    center: Point3,
    radius: f64,
    material: &'a Material,
    r: &Ray,
    dist_range: (f64, f64),
) -> Option<HitData<'a>> {
    let oc = r.origin - center;

    if oc.length() <= radius {
        // inside sphere, can't collide with surface
        return None;
    }

    let half_b = -dot(oc, r.direction);
    let c = oc.length_squared() - radius * radius;

    // if c > 0.0 && half_b > 0.0 {
    //     return None;
    // }

    let discriminant = (half_b * half_b) - c;
    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    // find the nearest root that lies in the acceptable range.
    let root = half_b - sqrt_d;
    if root < dist_range.0 || dist_range.1 < root {
        let root2 = half_b + sqrt_d;
        if root2 < dist_range.0 || dist_range.1 < root2 {
            return None;
        }
    }

    let dist = root;
    let p = r.at(dist);
    let outward_normal = (p - center) / radius;
    let front_face = dot(r.direction, outward_normal) < 0.0;

    let data = HitData {
        dist,
        p,
        front_face,
        normal: match front_face {
            true => outward_normal,
            false => -outward_normal,
        },
        material: material,
    };

    Some(data)
}
