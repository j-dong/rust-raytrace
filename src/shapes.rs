//! Shapes that can collide with rays
//!
//! This module contains various shapes that have a collision
//! test with the `Ray` struct, which is also in here. These
//! shapes are used to model how an object looks and reflects light.
use types::*;
use types::na::{Norm, Dot};

/// A ray. A ray has an origin and a direction.
pub struct Ray {
    /// The origin of the ray.
    pub origin: Pnt3,
    /// The direction of the ray. Should be normalized most of the
    /// time.
    pub direction: Vec3,
}

impl Ray {
    /// Casts the ray. `t` is the length of the casted segment.
    /// Returns the point `t` away from the ray's origin in the
    /// ray's direction (assuming the direction is normalized).
    pub fn cast(&self, t: f64) -> Pnt3 {
        self.origin + self.direction * t
    }
}

/// The result of an intersection between a shape and a ray.
pub struct IntersectionResult {
    /// The `t` value at which the ray hits the shape.
    pub t: f64,
    /// The normal of the shape at the point of intersection.
    pub normal: Vec3,
}

/// A shape that can be intersected with a ray.
pub trait Shape {
    /// Intersect the ray with a shape, and return the closest
    /// result that is in the direction of the ray.
    fn intersect(&self, ray: &Ray) -> Option<IntersectionResult>;
}

/// A sphere. A sphere has a center and a radius.
pub struct Sphere {
    /// The center of the sphere.
    pub center: Pnt3,
    /// The radius of the sphere.
    pub radius: f64,
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

/// A plane. Defined by a point on it and the normal vector.
/// Not the kind that flies.
pub struct Plane {
    /// A point on the plane.
    pub point: Pnt3,
    /// The normal vector.
    pub normal: Vec3,
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionResult> {
        let t = self.normal.dot(&(self.point - ray.origin)) / self.normal.dot(&ray.direction);
        if t <= 0.0 {
            None
        } else {
            Some(IntersectionResult {
                t: t,
                normal: self.normal,
            })
        }
    }
}
