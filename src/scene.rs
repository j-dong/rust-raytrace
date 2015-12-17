use types::*;
use shapes::*;
use color::*;
use camera::*;

use types::na::Norm;

use std::boxed::Box;
use std::option::Option;
use std::iter::Iterator;
use std::cmp::Ordering;

/// An object's material. A material is used to compute the color
/// of an object when a ray hits it.
struct Material {
    /// Diffuse color of Lambertian reflectance.
    diffuse: Color,
}

/// An object in a scene. The `Object` struct contains everything
/// necessary to render the object.
struct Object {
    /// The bounds of the object, which is used for ray intersection.
    bounds: Box<Shape>,
    /// The material of the object.
    material: Material,
}

/// A light that can project rays onto an object.
trait Light {
    /// Get the light direction for lighting a specific point.
    fn light_dir_for(&self, pt: &Pnt3) -> Vec3;
    /// The the shadow ray used to project back onto the light
    /// to see if it intersects any objects on the way there.
    fn shadow_ray_for(&self, pt: &Pnt3) -> Ray {
        Ray { origin: pt.clone(), direction: -self.light_dir_for(pt) }
    }
}

/// A simple point light.
struct PointLight {
    location: Pnt3,
}

impl Light for PointLight {
    fn light_dir_for(&self, pt: &Pnt3) -> Vec3 {
        Vec3::new(pt.x - self.location.x, pt.y - self.location.y, pt.z - self.location.z).normalize()
    }
}

/// A simple directional light.
struct DirectionalLight {
    direction: Vec3,
}

impl Light for DirectionalLight {
    fn light_dir_for(&self, pt: &Pnt3) -> Vec3 {
        self.direction
    }
}

/// A scene with objects, lights, and a camera.
struct Scene {
    /// The objects in the scene.
    pub objects: Vec<Box<Object>>,
    /// The lights in the scene.
    pub lights: Vec<Box<Light>>,
    /// The camera of the scene.
    pub camera: Box<Camera>,
}

/// Intersection result of a scene, containing the object it hit.
pub struct SceneIntersectionResult<'a> {
    /// The object the ray hit.
    pub object: &'a Object,
    /// The `IntersectionResult` returned by the object's `intersect`
    /// method.
    pub result: IntersectionResult,
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
    /// Intersect a ray with the scene, returning a result which
    /// contains the `intersect` result and the object it hit.
    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersectionResult> {
        self.objects.iter().filter_map(|o| o.bounds.intersect(ray).map( |r| SceneIntersectionResult { object: o, result: r })).min_by_key(|o| FloatNotNan::new(o.result.t))
    }
}
