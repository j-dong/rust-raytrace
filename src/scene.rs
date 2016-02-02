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
use texture::*;

use types::na::{Norm, FloatPnt};

use std::boxed::Box;
use std::option::Option;
use std::iter::Iterator;
use std::cmp::Ordering;

/// An object's material. A material is used to compute the color
/// of an object when a ray hits it.
pub trait Material {
    /// Get the color from a ray intersection; generally involves
    /// getting the interaction from the object's material. Significance is a float that is decreased
    /// when a ray is generated recursively.
    fn color(&self, scene: &Scene, result: &IntersectionResult, ray: &Ray, significance: f64, depth: u32, rng: &mut RngT) -> Color;
}

/// Material using the Blinn-Phong reflection model.
pub struct PhongMaterial {
    /// Diffuse color of Lambertian reflectance.
    pub diffuse: Color,
    /// Color of specular reflectance. Currently glossy reflection is not implemented, and thus
    /// only highlights will be glossy.
    pub specular: Color,
    /// Shininess (specular exponent) in the Phong reflection model.
    pub exponent: f64,
    /// Ambient light (light from scattered light in the environment). Currently ambient
    /// occlusion is not implemented.
    pub ambient: Color,
}

/// Material using Blinn-Phong, but with a Fresnel term.
/// There is little effect on metals, but for dielectrics,
/// there is a dramatic effect.
pub struct FresnelMaterial {
    /// Diffuse color of Lambertian reflectance.
    pub diffuse: Color,
    /// Color of specular reflectance. Currently glossy reflection is not implemented, and thus
    /// only highlights will be glossy.
    pub specular: Color,
    /// Shininess (specular exponent) in the Phong reflection model.
    pub exponent: f64,
    /// Ambient light (light from scattered light in the environment). Currently ambient
    /// occlusion is not implemented.
    pub ambient: Color,
    /// Index of refraction. The IOR of air is 1.00. There are tables on the Internet.
    pub ior: f64,
}

/// Material using Blinn-Phong that transmits light. Suitable for materials like glass
/// which are clear. The amount of reflectance is determined using the Fresnel equation.
pub struct TransparentMaterial {
    /// Color of specular reflectance. Currently glossy reflection is not implemented, and thus
    /// only highlights will be glossy.
    pub specular: Color,
    /// Shininess (specular exponent) in the Phong reflection model.
    pub exponent: f64,
    /// Index of refraction. The IOR of air is 1.00. There are tables on the Internet.
    pub ior: f64,
}

/// Material using the Blinn-Phong reflection model with indirect lighting.
pub struct IndirectPhongMaterial {
    /// Diffuse color of Lambertian reflectance.
    pub diffuse: Color,
    /// Color of specular reflectance. Currently glossy reflection is not implemented, and thus
    /// only highlights will be glossy.
    pub specular: Color,
    /// Shininess (specular exponent) in the Phong reflection model.
    pub exponent: f64,
    /// Ambient light (light from scattered light in the environment). Currently ambient
    /// occlusion is not implemented.
    pub ambient: Color,
    /// Number of samples to use
    pub samples: u32,
}

/// An object in a scene. The `Object` struct contains everything
/// necessary to render the object.
pub struct Object {
    /// The bounds of the object, which is used for ray intersection.
    pub bounds: Box<Shape>,
    /// The material of the object.
    pub material: Box<Material>,
}

/// A light that can project rays onto an object.
pub trait LightModel {
    /// Get the light direction for lighting a specific point.
    /// This is the vector from the point to the light, not the
    /// light's direction. Also gets the square of the range.
    fn light_dir_and_sq_range_for(&self, pt: &Pnt3, rng: &mut RngT) -> (Vec3, Option<f64>);
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
    fn light_dir_and_sq_range_for(&self, pt: &Pnt3, _: &mut RngT) -> (Vec3, Option<f64>) {
        (Vec3::new(self.location.x - pt.x, self.location.y - pt.y, self.location.z - pt.z).normalize(), Some(self.location.sqdist(pt)))
    }
}

/// A simple directional light.
pub struct DirectionalLight {
    /// The direction of the light.
    pub direction: Vec3,
}

impl LightModel for DirectionalLight {
    fn light_dir_and_sq_range_for(&self, _: &Pnt3, _: &mut RngT) -> (Vec3, Option<f64>) {
        (-self.direction, None)
    }
}

/// The background of a scene. They are used when a ray does not
/// intersect any object.
pub trait Background {
    /// The color of the background with a specified ray.
    fn color(&self, ray: &Ray, rng: &mut RngT) -> Color;
}

/// A background where the result is always a solid color.
pub struct SolidColorBackground {
    /// The color of the background.
    pub color: Color,
}

/// A background where the result is looking up the ray direction
/// in a skybox. A skybox is an environment mapping technique
/// that stores the environment in a cube of images. This produces
/// rich backgrounds with little computation.
#[cfg(feature = "skybox")]
pub struct SkyboxBackground {
    /// The face in the positive X direction.
    pub px: Texture,
    /// The face in the negative X direction.
    pub nx: Texture,
    /// The face in the positive Y direction.
    pub py: Texture,
    /// The face in the negative Y direction.
    pub ny: Texture,
    /// The face in the positive Z direction.
    pub pz: Texture,
    /// The face in the negative Z direction.
    pub nz: Texture,
}

/// Render options
pub struct Options {
    /// width of the rendered image
    pub width: u32,
    /// height of the rendered image
    pub height: u32,
    /// number of anti-aliasing samples
    pub antialias: u32,
}

/// A scene with objects, lights, a camera, and a background.
pub struct Scene {
    /// The objects in the scene.
    pub objects: Vec<Object>,
    /// The lights in the scene.
    pub lights: Vec<Light>,
    /// The camera of the scene.
    pub camera: Box<Camera>,
    /// The background of the scene.
    pub background: Box<Background>,
    /// Rendering options
    pub options: Options,
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
struct FloatNotNan(f64);

impl FloatNotNan {
    fn new(v: f64) -> Option<FloatNotNan> {
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
