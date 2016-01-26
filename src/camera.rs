//! Projection types
//!
//! Cameras represent ways to project a point on the image into the
//! scene as a ray.

use types::rand::distributions::IndependentSample;
use types::rand::Rng;
use types::rand;
use std::cell::RefCell;
use std::f64;

use types::*;
use shapes::*;

use types::na::{cross, Norm};

/// Trait for cameras.
///
/// Cameras are able to project an image position to a ray. The
/// `project` function is called for every pixel in the image, where
/// the x and y coordinates of the `position` argument are normalized
/// image coordinates.
pub trait Camera {
    /// Projects a point onto the scene, and returns the ray.
    ///
    /// `position` is normalized image coordinates, where (-1, -1)
    /// to (1, 1) is the largest centered square in the image.
    fn project<R: Rng>(&self, position: &Pnt2, rng: &mut R) -> Ray;
    /// Get the number of samples per pixel.
    fn samples(&self) -> u32 {1}
}

/// A very simple perspective camera that projects the given position
/// from its location onto an image plane.
pub struct SimplePerspectiveCamera {
    /// The position of the camera.
    pub position: Pnt3,
    /// A matrix that transforms a homogeneous vector (x, y, 1)
    /// into the direction in world-space.
    pub matrix: Mat3,
}

impl SimplePerspectiveCamera {
    /// Create a new `SimplePerspectiveCamera` from the given
    /// position, look vector, up vector, and image distance. The
    /// resulting camera will always point in the direction of `look`
    /// (i.e. (0, 0) will project with a direction equal to that of
    /// `look`), and `up` points in an upward direction.
    ///
    /// `look` and `up` should not point in the same direction, or
    /// the resulting rays will have NaN directions.
    ///
    /// For best results, `look` and `up` should be normalized, but
    /// this is not necessary since the resulting rays are normalized.
    pub fn new(position: &Pnt3, look: &Vec3, up: &Vec3, im_dist: f64) -> SimplePerspectiveCamera {
        let u = cross(look, up).normalize();
        let v = cross(&u, look).normalize();
        let w = look.normalize() * im_dist;
        SimplePerspectiveCamera {
            position: position.clone(),
            matrix: Mat3::new(
                u.x, v.x, w.x,
                u.y, v.y, w.y,
                u.z, v.z, w.z,
            )
        }
    }
    /// Create a new `SimplePerspectiveCamera` from the given focus
    /// point, viewing direction, and up vector, POV angle, and half
    /// the focus height.
    pub fn look_at(focus: &Pnt3, look: &Vec3, up: &Vec3, pov: f64, h: f64) -> SimplePerspectiveCamera {
        let cot = (pov / 2.0).tan().recip();
        let im_dist = cot;
        let d = h * cot;
        let position = focus.clone() - look.normalize() * d;
        SimplePerspectiveCamera::new(&position, look, up, im_dist)
    }
}

impl Camera for SimplePerspectiveCamera {
    fn project<R: Rng>(&self, position: &Pnt2, _: &mut R) -> Ray {
        Ray { origin: self.position, direction: (self.matrix * Vec3::new(position.x, position.y, 1.0)).normalize() }
    }
}

/// A perspective camera that has a depth of field effect.
pub struct DepthOfFieldCamera {
    /// The base camera.
    pub camera: SimplePerspectiveCamera,
    /// The distance to the focal plane (from the camera).
    pub focus: f64,
    /// The aperture radius. The aperture is a circle.
    pub aperture: f64,
    /// The number of samples.
    pub samples: u32,
    im_dist: f64,
    ang_range: rand::distributions::Range<f64>,
}

impl DepthOfFieldCamera {
    /// Create a new `DepthOfFieldCamera`.
    pub fn new(camera: SimplePerspectiveCamera, focus: f64, aperture: f64, samples: u32) -> DepthOfFieldCamera {
        let im_dist = (camera.matrix * Vec3::new(0.0, 0.0, 1.0)).norm();
        DepthOfFieldCamera {
            camera: camera,
            focus: focus,
            aperture: aperture,
            samples: samples,
            im_dist: im_dist,
            ang_range: rand::distributions::Range::new(0.0, 2.0 * f64::consts::PI),
        }
    }
}

impl Camera for DepthOfFieldCamera {
    fn project<R: Rng>(&self, position: &Pnt2, rng: &mut R) -> Ray {
        let dir = self.camera.matrix * Vec3::new(position.x, position.y, 1.0); // not normalized
        let ip = self.camera.position + dir; // point on image plane
        let fp = self.camera.position + dir * (self.focus / self.im_dist); // focal point
        // generate a random angle
        let theta = self.ang_range.ind_sample(&mut *rng);
        // taking the square root of the radius yields a uniform distribution
        let rand::Closed01(r2) = rng.gen::<rand::Closed01<f64>>();
        let r = r2.sqrt() * self.aperture;
        let orig = ip + self.camera.matrix * Vec3::new(theta.cos() * r, theta.sin() * r, 0.0);
        Ray { origin: orig, direction: (fp - orig).normalize() }
    }
    fn samples(&self) -> u32 {self.samples}
}
