extern crate nalgebra as na;
use na::Inv;
use na::ToHomogeneous;
use na::FromHomogeneous;

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

    fn transform(&self, mat: Mat4) -> Ray {
        Ray {
            origin: FromHomogeneous::<Pnt4>::from(Mul::<Pnt4>::mul(mat, self.origin.to_homogeneous())),
            direction: FromHomogeneous::<Mat4>::from(&mat) * self.direction,
        }
    }
}

trait Shape {
    fn intersect(&self, ray: &Ray);
}

struct Sphere {
    center: Pnt3,
    radius: f32,
}
