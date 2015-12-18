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

/// Get the color from a ray intersection; generally involves
/// getting the interaction from the object's material
pub fn intersection_color(scene: &Scene, result: &SceneIntersectionResult, ray: &Ray) -> Color {
    let mut res = color::BLACK;
    let pt = ray.cast(result.result.t);
    for light in &scene.lights {
        if result.object.material.diffuse.significance() > 0.0 {
            let (ldir, sray) = light.light_shadow_for(&pt);
            res = res + result.object.material.diffuse * dot(&ldir, &result.result.normal);
        }
    }
    res
}

pub fn background_color(ray: &Ray) -> Color {
    color::BLACK
}

/// Trace a ray to an object or nothing and return the result of
/// color computation.
pub fn raytrace(scene: &Scene, pos: &Pnt2) -> Color {
    // find the object that the ray hits and compute the color
    let ray = scene.camera.project(pos);
    match scene.intersect(&ray) {
        Some(result) => intersection_color(scene, &result, &ray),
        None => background_color(&ray),
    }
}
