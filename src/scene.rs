use types::*;
use shapes::*;
use color::*;
use camera::*;

use types::na::Norm;

use std::boxed::Box;

struct Material {
    diffuse: Color,
}

struct Object {
    bounds: Box<Shape>,
    material: Material,
}

trait Light {
    fn light_dir_for(&self, pt: &Pnt3) -> Vec3;
    fn shadow_ray_for(&self, pt: &Pnt3) -> Ray {
        Ray { origin: pt.clone(), direction: -self.light_dir_for(pt) }
    }
}

struct PointLight {
    location: Pnt3,
}

impl Light for PointLight {
    fn light_dir_for(&self, pt: &Pnt3) -> Vec3 {
        Vec3::new(pt.x - self.location.x, pt.y - self.location.y, pt.z - self.location.z).normalize()
    }
}

struct DirectionalLight {
    direction: Vec3,
}

impl Light for DirectionalLight {
    fn light_dir_for(&self, pt: &Pnt3) -> Vec3 {
        self.direction
    }
}

struct Scene {
    objects: Vec<Box<Object>>,
    lights: Vec<Box<Light>>,
    camera: Box<Camera>,
}
