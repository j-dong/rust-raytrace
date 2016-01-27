//! Module where raytracing actually happens
//!
//! In this module, the interaction of light with various
//! objects is simulated using various formulae.

use types::*;
use shapes::{Shape, Ray, IntersectionResult};
use camera::Camera;
use color;
use color::Color;
use scene::*;

use types::na::{dot, Norm};

const MIN_SIGNIFICANCE: f64 = 1.0f64 / 256.0 / 2.0;

#[inline]
fn clamp_zero(x: f64) -> f64 {
    if x < 0.0 { 0.0 } else { x }
}

#[inline]
fn clamp_one(x: f64) -> f64 {
    if x > 1.0 { 1.0 } else { x }
}

impl Material for PhongMaterial {
    fn color(&self, scene: &Scene, result: &IntersectionResult, ray: &Ray, significance: f64, rng: &mut RngT) -> Color {
        let mut res = self.ambient;
        let pt = ray.cast(result.t);
        let diffuse = self.diffuse.significance() * significance > MIN_SIGNIFICANCE;
        let specular = self.specular.significance() * significance > MIN_SIGNIFICANCE;
        // normal should face the viewer; if not, flip it
        let normal = if dot(&result.normal, &ray.direction) > 0.0 { -result.normal } else { result.normal };
        for light in &scene.lights {
            if diffuse || specular {
                let ldir = light.model.light_dir_for(&pt, rng);
                // check if in shadow
                if let Some(intersection) = scene.intersect(&Ray { origin: pt + ldir * 0.00001, direction: ldir }) {
                    if match light.model.sq_shadow_range(&pt) {
                        Some(r2) => intersection.result.t * intersection.result.t < r2,
                        None => true,
                    } {
                        continue;
                    }
                }
                if diffuse {
                    res = res + self.diffuse * light.color * clamp_zero(dot(&ldir, &normal));
                }
                if specular {
                    res = res + self.specular * light.color * clamp_zero(dot(&normal, &((ldir - ray.direction).normalize()))).powf(self.exponent);
                }
            }
        }
        if specular {
            let d = ray.direction;
            let rd = d - normal * (2.0 * dot(&d, &normal));
            let reflect = Ray { origin: pt + rd * 0.00001, direction: rd };
            res = res + self.specular * ray_color(scene, &reflect, significance * self.specular.significance(), rng);
        }
        res
    }
}

impl Material for FresnelMaterial {
    fn color(&self, scene: &Scene, result: &IntersectionResult, ray: &Ray, significance: f64, rng: &mut RngT) -> Color {
        let mut res = self.ambient;
        let pt = ray.cast(result.t);
        let nd = dot(&result.normal, &ray.direction);
        // normal should face the viewer; if not, flip it
        let normal = if nd > 0.0 { -result.normal } else { result.normal };
        // I think the Schlick approximation should work well
        let r0 = (self.ior - 1.0) / (self.ior + 1.0);
        let r0 = r0 * r0;
        let omcos = 1.0 - nd.abs();
        let omcos2 = omcos * omcos;
        let fresnel = clamp_one(r0 + (1.0 - r0) * omcos2 * omcos2 * omcos);
        let diffuse = self.diffuse.significance() * significance > MIN_SIGNIFICANCE;
        let specular = self.specular.significance() * fresnel * significance > MIN_SIGNIFICANCE;
        for light in &scene.lights {
            if diffuse || specular {
                let ldir = light.model.light_dir_for(&pt, rng);
                // check if in shadow
                if let Some(intersection) = scene.intersect(&Ray { origin: pt + ldir * 0.00001, direction: ldir }) {
                    if match light.model.sq_shadow_range(&pt) {
                        Some(r2) => intersection.result.t * intersection.result.t < r2,
                        None => true,
                    } {
                        continue;
                    }
                }
                if diffuse {
                    res = res + self.diffuse * light.color * clamp_zero(dot(&ldir, &normal));
                }
                if specular {
                    res = res + self.specular * light.color * fresnel * clamp_zero(dot(&normal, &((ldir - ray.direction).normalize()))).powf(self.exponent);
                }
            }
        }
        if specular {
            let d = ray.direction;
            let rd = d - normal * (2.0 * dot(&d, &normal));
            let reflect = Ray { origin: pt + rd * 0.00001, direction: rd };
            res = res + self.specular * ray_color(scene, &reflect, fresnel * significance * self.specular.significance(), rng) * fresnel;
        }
        res
    }
}

impl Material for TransparentMaterial {
    fn color(&self, scene: &Scene, result: &IntersectionResult, ray: &Ray, significance: f64, rng: &mut RngT) -> Color {
        let mut res = color::BLACK;
        let pt = ray.cast(result.t);
        let nd = dot(&result.normal, &ray.direction);
        // normal should face the viewer; if not, flip it
        let normal = if nd > 0.0 { -result.normal } else { result.normal };
        // calculate refraction vector
        let ndv = dot(&normal, &ray.direction);
        let n = if nd > 0.0 { self.ior } else { 1.0 / self.ior };
        let sin2 = n * n * (1.0 - nd * nd);
        let refract = if sin2 < 1.0 {
            let cos = (1.0 - sin2).sqrt();
            Some(ray.direction * n - normal * (n * nd.abs() + cos))
        } else {
            None
        };
        // I think the Schlick approximation should work well
        let r0 = (self.ior - 1.0) / (self.ior + 1.0);
        let r0 = r0 * r0;
        let omcos = if nd > 0.0 { if let Some(r) = refract { 1.0 - dot(&normal, &r) } else { 0.0 } } else { 1.0 - nd.abs() };
        let omcos2 = omcos * omcos;
        let fresnel = if refract.is_some() { clamp_one(r0 + (1.0 - r0) * omcos2 * omcos2 * omcos) } else { 1.0 };
        let specular = self.specular.significance() * fresnel * significance > MIN_SIGNIFICANCE;
        for light in &scene.lights {
            if specular {
                let ldir = light.model.light_dir_for(&pt, rng);
                // check if in shadow
                if let Some(intersection) = scene.intersect(&Ray { origin: pt + ldir * 0.00001, direction: ldir }) {
                    if match light.model.sq_shadow_range(&pt) {
                        Some(r2) => intersection.result.t * intersection.result.t < r2,
                        None => true,
                    } {
                        continue;
                    }
                }
                res = res + self.specular * light.color * fresnel * clamp_zero(dot(&normal, &((ldir - ray.direction).normalize()))).powf(self.exponent);
            }
        }
        if specular {
            let rd = ray.direction - normal * (2.0 * ndv);
            let reflect = Ray { origin: pt + rd * 0.00001, direction: rd };
            res = res + self.specular * ray_color(scene, &reflect, fresnel * significance * self.specular.significance(), rng) * fresnel;
        }
        if fresnel < 1.0 {
            match refract {
                None => (),
                Some(refract) => {
                    let omf = clamp_one(1.0 - fresnel);
                    let refract = refract.normalize();
                    res = res + ray_color(scene, &Ray { origin: pt + refract * 0.00001, direction: refract }, omf * significance, rng) * omf;
                }
            }
        }
        res
    }
}

impl Background for SolidColorBackground {
    fn color(&self, _: &Ray, _: &mut RngT) -> Color {
        self.color
    }
}

macro_rules! skybox_axis {
    ($self_:ident, $rayd:expr, $namep:ident, $namen:ident, $dir:ident, $otherx:ident, $othery:ident, $px:expr, $py:expr) => {
        if $rayd.$dir.abs() > $rayd.$otherx.abs() && $rayd.$dir.abs() > $rayd.$othery.abs() {
            if $rayd.$dir > 0.0 {
                return $self_.$namep.sample($px * 0.5 + 0.5, $py * 0.5 + 0.5);
            } else {
                return $self_.$namen.sample($px * 0.5 + 0.5, $py * 0.5 + 0.5);
            }
        }
    };
}

impl Background for SkyboxBackground {
    fn color(&self, ray: &Ray, _: &mut RngT) -> Color {
        let d = ray.direction;
        skybox_axis!(self, d, px, nx, x, z, y, -d.z / d.x, -d.y / d.x.abs());
        skybox_axis!(self, d, py, ny, y, x, z, d.x / d.y.abs(), d.z / d.y);
        skybox_axis!(self, d, pz, nz, z, x, y, d.x / d.z, -d.y / d.z.abs());
        color::BLACK
    }
}

/// Trace a ray to an object or nothing and return the result of
/// color computation. Significance is a float that is decreased
/// when a ray is generated recursively.
pub fn ray_color(scene: &Scene, ray: &Ray, significance: f64, rng: &mut RngT) -> Color {
    // find the object that the ray hits and compute the color
    match scene.intersect(ray) {
        Some(result) => result.object.material.color(scene, &result.result, ray, significance, rng),
        None => scene.background.color(ray, rng),
    }
}

/// Project the position onto the scene and trace the ray.
pub fn raytrace(scene: &Scene, pos: &Pnt2, significance: f64, rng: &mut RngT) -> Color {
    let mut res = color::BLACK;
    for _ in 0..scene.camera.samples() {
        res = res + ray_color(scene, &scene.camera.project(pos, rng), significance, rng);
    }
    res / scene.camera.samples() as f64
}
