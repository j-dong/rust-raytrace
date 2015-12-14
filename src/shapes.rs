extern crate nalgebra as na;

use self::na::{Inv, Norm, Dot};

use types::*;

pub struct Ray {
    pub origin: Pnt3,
    pub direction: Vec3,
}

impl Ray {
    pub fn cast(&self, t: f32) -> Pnt3 {
        self.origin + self.direction * t
    }
}

pub struct IntersectionResult {
    pub t: f32,
    pub normal: Vec3,
}

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionResult>;
}

pub struct Sphere {
    pub center: Pnt3,
    pub radius: f32,
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionResult> {
        // pt = origin + direction * t
        // |pt - center|^2 = radius * radius
        // (ox + dx*t - cx)^2 + ... = radius * radius
        // (dx^2 + ...) t^2 + 2 ((ox - cx) dx + ...) t + (ox - cx)^2 + ... - radius * radius = 0
        // so as a quadratic
        // a = dx^2 + dy^2 + dz^2 = direction.sqnorm()
        // b = 2 * ((ox - cx) dx + (oy - cy) dy + (oz - cz) dz) = 2 * direction.dot(origin - center)
        // c = (ox - cx)^2 + (oy - cy)^2 + (oz - cz)^2 - radius * radius = (origin - center).sqnorm() - radius * radius
        let ominusc = ray.origin - self.center;
        let a = ray.direction.sqnorm();
        let b = 2.0 * ray.direction.dot(&ominusc);
        let c = ominusc.sqnorm() - self.radius * self.radius;
        // intersects iff b^2 - 4*a*c > 0
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let dsqrt = discriminant.sqrt();
            let t = (-b - dsqrt) / (2.0 * a);
            if t > 0.0 {
                Some(IntersectionResult {
                    t: t,
                    normal: (ray.cast(t) - self.center).normalize(),
                })
            } else {
                let t2 = (-b + dsqrt) / (2.0 * a);
                if t2 > 0.0 {
                    Some(IntersectionResult {
                        t: t2,
                        normal: (ray.cast(t2) - self.center).normalize(),
                    })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
