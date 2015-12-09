extern crate nalgebra as na;
use na::Inv;
use na::ToHomogeneous;
use na::FromHomogeneous;
use na::Norm;

use std::ops::Mul;

type Pnt3 = na::Pnt3<f32>;
type Pnt4 = na::Pnt4<f32>;
type Vec3 = na::Vec3<f32>;
type Vec4 = na::Vec4<f32>;
type Mat3 = na::Mat3<f32>;
type Mat4 = na::Mat4<f32>;

struct Ray {
    origin: Pnt3,
    direction: Vec3,
}

impl Ray {
    fn cast(&self, t: f32) -> Pnt3 {
        self.origin + self.direction * t
    }
}

struct IntersectionResult {
    t: f32,
    normal: Vec3,
}

trait Shape {
    fn intersect(&self, ray: &Ray) -> IntersectionResult;
}

struct Sphere {
    center: Pnt3,
    radius: f32,
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
        let b = 2 * ray.direction.dot(ominusc);
        let c = ominusc.sqnorm() - self.radius * self.radius;
        // intersects iff b^2 - 4*a*c > 0
        if (b * b - 4 * a * c > 0) {
            Some(IntersectionResult { (-b - num::sqrt()) })
        } else {
            None
        }
    }
}
