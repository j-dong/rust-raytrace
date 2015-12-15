use types::*;
use shapes::*;

use types::na::{ToHomogeneous, Cross, cross, Norm};

pub trait Camera {
    fn project(&self, position: &Pnt2) -> Ray;
}

pub struct SimplePerspectiveCamera {
    pub position: Pnt3,
    pub im_dist: f32,
    pub matrix: Mat3,
}

impl SimplePerspectiveCamera {
    pub fn new(position: &Pnt3, look: &Vec3, up: &Vec3, im_dist: f32) -> SimplePerspectiveCamera {
        let u = cross(look, up);
        let v = cross(&u, look);
        let w = look.clone() * im_dist;
        SimplePerspectiveCamera {
            position: position.clone(),
            im_dist: im_dist,
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
