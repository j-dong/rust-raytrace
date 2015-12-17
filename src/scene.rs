use types::*;
use shapes::*;
use color::*;
use camera::*;

use types::na::Norm;

use std::boxed::Box;
use std::option::Option;
use std::iter::Iterator;
use std::cmp::Ordering;

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

pub struct SceneIntersectionResult<'a> {
    object: &'a Object,
    result: IntersectionResult,
}

#[derive(PartialEq, PartialOrd)]
struct FloatNotNan(f32);

impl FloatNotNan {
    fn new(v: f32) -> Option<FloatNotNan> {
        if v.is_nan() {
            None
        } else {
            Some(FloatNotNan(v))
        }
    }
}

impl Eq for FloatNotNan {}

impl Ord for FloatNotNan {
    fn cmp(&self, other: &FloatNotNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Scene {
    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersectionResult> {
        self.objects.iter().filter_map(|o| o.bounds.intersect(ray).map( |r| SceneIntersectionResult { object: o, result: r })).min_by_key(|o| FloatNotNan::new(o.result.t))
    }
}
