//! Projection types
//!
//! Cameras represent ways to project a point on the image into the
//! scene as a ray.

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
    fn project(&self, position: &Pnt2) -> Ray;
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
    fn project(&self, position: &Pnt2) -> Ray {
        Ray { origin: self.position, direction: (self.matrix * Vec3::new(position.x, position.y, 1.0)).normalize() }
    }
}
