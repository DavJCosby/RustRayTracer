use crate::math::{ray::Ray, vector::*};

use super::hit::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
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
        };

        Some(data)
    }
}

/*
pub fn hit_sphere(s: &Sphere, r: &Ray) -> f64 {
    let oc = r.origin - s.center;
    let a = r.direction.length_squared();
    let half_b = dot(oc, r.direction);
    let c = oc.length_squared() - s.radius * s.radius;
    let discriminant = (half_b * half_b) - (a * c);

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
*/
