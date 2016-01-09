//! The scene and the objects in them
//!
//! This module contains the `Scene` struct and the objects it
//! contains. These classes are used to model the objects that
//! interact with the light from light sources, the light sources
//! themselves, and how the light interacts with the objects.

use types::*;
use shapes::*;
use color::*;
use camera::*;

use types::na::Norm;

use std::boxed::Box;
use std::option::Option;
use std::iter::Iterator;
use std::cmp::Ordering;

#[cfg(not(feature = "nightly"))]
use std::iter::FilterMap;

/// An object's material. A material is used to compute the color
/// of an object when a ray hits it.
pub struct Material {
    /// Diffuse color of Lambertian reflectance.
    pub diffuse: Color,
    /// Color of mirror reflectance.
    pub reflect: Color,
}

/// An object in a scene. The `Object` struct contains everything
/// necessary to render the object.
pub struct Object {
    /// The bounds of the object, which is used for ray intersection.
    pub bounds: Box<Shape>,
    /// The material of the object.
    pub material: Material,
}

/// A light that can project rays onto an object.
pub trait LightModel {
    /// Get the light direction for lighting a specific point.
    fn light_dir_for(&self, pt: &Pnt3) -> Vec3;
    /// The the shadow ray used to project back onto the light
    /// to see if it intersects any objects on the way there.
    fn shadow_ray_for(&self, pt: &Pnt3) -> Ray {
        Ray { origin: pt.clone(), direction: -self.light_dir_for(pt) }
    }
    /// Combination of light direction and shadow ray.
    fn light_shadow_for(&self, pt: &Pnt3) -> (Vec3, Ray) {
        let dir = self.light_dir_for(pt);
        (dir, Ray { origin: pt.clone(), direction: -dir })
    }
}

/// A light that can project rays of a color onto an object.
pub struct Light {
    /// A model for the geometry of the light, used to project rays.
    pub model: Box<LightModel>,
    /// The color of the light.
    pub color: Color,
}

/// A simple point light.
pub struct PointLight {
    /// The location of the light.
    pub location: Pnt3,
}

impl LightModel for PointLight {
    fn light_dir_for(&self, pt: &Pnt3) -> Vec3 {
        Vec3::new(pt.x - self.location.x, pt.y - self.location.y, pt.z - self.location.z).normalize()
    }
}

/// A simple directional light.
pub struct DirectionalLight {
    /// The direction of the light.
    pub direction: Vec3,
}

impl LightModel for DirectionalLight {
    fn light_dir_for(&self, _: &Pnt3) -> Vec3 {
        self.direction
    }
}

/// A scene with objects, lights, and a camera.
pub struct Scene {
    /// The objects in the scene.
    pub objects: Vec<Object>,
    /// The lights in the scene.
    pub lights: Vec<Light>,
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

// following copied from Rust source
#[cfg(not(feature = "nightly"))]
#[inline]
fn select_fold1<I,B, FProj, FCmp>(mut it: I,
                                  mut f_proj: FProj,
                                  mut f_cmp: FCmp) -> Option<(B, I::Item)>
    where I: Iterator,
          FProj: FnMut(&I::Item) -> B,
          FCmp: FnMut(&B, &I::Item, &B, &I::Item) -> bool {
    it.next().map(|mut sel| {
        let mut sel_p = f_proj(&sel);

        for x in it {
            let x_p = f_proj(&x);
            if f_cmp(&sel_p,  &sel, &x_p, &x) {
                sel = x;
                sel_p = x_p;
            }
        }
        (sel_p, sel)
    })
}
#[cfg(not(feature = "nightly"))]
trait HackMin {
    type Item;
    fn min_by_key<B, F>(self, f: F) -> Option<Self::Item> where F: FnMut(&Self::Item) -> B, B: Ord;
}
#[cfg(not(feature = "nightly"))]
impl<B, I, F> HackMin for FilterMap<I, F> where F: FnMut(I::Item) -> Option<B>, I: Iterator {
    type Item = B;
    fn min_by_key<BB: Ord, FF>(self, f: FF) -> Option<Self::Item> where Self: Sized, FF: FnMut(&Self::Item) -> BB {
        select_fold1(self, f, |x_p, _, y_p, _| x_p > y_p).map(|(_, x)| x)
    }
}

impl Scene {
    /// Intersect a ray with the scene, returning a result which
    /// contains the `intersect` result and the object it hit.
    pub fn intersect(&self, ray: &Ray) -> Option<SceneIntersectionResult> {
        self.objects.iter().filter_map(|o| o.bounds.intersect(ray).map( |r| SceneIntersectionResult { object: o, result: r })).min_by_key(|o| FloatNotNan::new(o.result.t))
    }
}
