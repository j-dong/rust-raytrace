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
    let ref mat = result.object.material;
    for light in &scene.lights {
        if mat.diffuse.significance() * significance > MIN_SIGNIFICANCE {
            let (ldir, sray) = light.model.light_shadow_for(&pt);
            // check if in shadow
            if let Some(intersection) = scene.intersect(ray) {
                if match light.model.sq_shadow_range(&pt) {
                    Some(r2) => intersection.result.t * intersection.result.t < r2,
                    None => false
                } {
                    continue;
                }
            }
            res = res + mat.diffuse * light.color * dot(&ldir, &result.result.normal);
        }
    }
    if mat.reflect.significance() * significance > MIN_SIGNIFICANCE {
        let d = ray.direction;
        let n = result.result.normal;
        let rd = d - n * (2.0 * dot(&d, &n));
        let reflect = Ray { origin: pt + rd * 0.001, direction: rd };
        res = res + mat.reflect * ray_color(scene, &reflect, significance * mat.reflect.significance());
    }
    // TODO: refraction
    // http://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
    // TODO: refactor this method into different impls for
    // Material trait
    // such as Phong, Translucent, etc.
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
pub fn ray_color(scene: &Scene, ray: &Ray, significance: f32) -> Color {
    // find the object that the ray hits and compute the color
    match scene.intersect(ray) {
        Some(result) => intersection_color(scene, &result, ray, significance),
        None => background_color(ray),
    }
}

/// Project the position onto the scene and trace the ray.
pub fn raytrace(scene: &Scene, pos: &Pnt2, significance: f32) -> Color {
    ray_color(scene, &scene.camera.project(pos), significance)
}
