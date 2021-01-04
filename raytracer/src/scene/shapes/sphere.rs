use crate::{
    scene::materials::material::*,
    utils::{ray::Ray, vector::*},
};

use super::hit::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, dist_range: (f64, f64)) -> Option<HitData> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        // find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrt_d) / a;
        if root < dist_range.0 || dist_range.1 < root {
            let root2 = (-half_b + sqrt_d) / a;
            if root2 < dist_range.0 || dist_range.1 < root2 {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let front_face = dot(r.direction, outward_normal) < 0.0;

        let data = HitData {
            dist: root,
            p,
            front_face,
            normal: match front_face {
                true => outward_normal,
                false => -outward_normal,
            },
            material: &self.material,
        };

        Some(data)
    }
}
