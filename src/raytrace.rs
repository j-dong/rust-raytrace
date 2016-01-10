//! Module where raytracing actually happens
//!
//! In this module, the interaction of light with various
//! objects is simulated using various formulae.

use types::*;
use shapes::{Shape, Ray};
use camera::Camera;
use color;
use color::Color;
use scene::*;

use types::na::dot;

const MIN_SIGNIFICANCE: f32 = 1.0f32 / 256.0 / 2.0;

/// Get the color from a ray intersection; generally involves
/// getting the interaction from the object's material. Significance is a float that is decreased
/// when a ray is generated recursively.
pub fn intersection_color(scene: &Scene, result: &SceneIntersectionResult, ray: &Ray, significance: f32) -> Color {
    let mut res = color::BLACK;
    let pt = ray.cast(result.result.t);
    for light in &scene.lights {
        if result.object.material.diffuse.significance() * significance > MIN_SIGNIFICANCE {
            let (ldir, sray) = light.model.light_shadow_for(&pt);
            res = res + result.object.material.diffuse * light.color * dot(&ldir, &result.result.normal);
        }
    }
    res
}

/// Get the color when a ray does not intersect any geometry.
/// Later on this function may compute sky color using scattering
/// or perhaps a skybox.
pub fn background_color(ray: &Ray) -> Color {
    color::BLACK
}

/// Trace a ray to an object or nothing and return the result of
/// color computation. Significance is a float that is decreased
/// when a ray is generated recursively.
pub fn raytrace(scene: &Scene, pos: &Pnt2, significance: f32) -> Color {
    // find the object that the ray hits and compute the color
    let ray = scene.camera.project(pos);
    match scene.intersect(&ray) {
        Some(result) => intersection_color(scene, &result, &ray, significance),
        None => background_color(&ray),
    }
}
