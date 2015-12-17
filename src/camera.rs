use types::*;
use shapes::*;

use types::na::{ToHomogeneous, cross, Norm};

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
    pub fn new(position: &Pnt3, look: &Vec3, up: &Vec3, im_dist: f32) -> SimplePerspectiveCamera {
        let u = cross(look, up);
        let v = cross(&u, look);
        let w = look.clone() * im_dist;
        SimplePerspectiveCamera {
            position: position.clone(),
            matrix: Mat3::new(
                u.x, v.x, w.x,
                u.y, v.y, w.y,
                u.z, v.z, w.z,
            )
        }
    }
}

impl Camera for SimplePerspectiveCamera {
    fn project(&self, position: &Pnt2) -> Ray {
        Ray { origin: self.position, direction: (self.matrix * position.as_vec().to_homogeneous()).normalize() }
    }
}
