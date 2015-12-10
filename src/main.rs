extern crate nalgebra as na;
extern crate raytrace;

use self::na::Norm;

use raytrace::shapes::{Ray, Sphere};
use raytrace::types::*;

fn main() {
    // camera parameters
    let eye  = Pnt3::new(0.0, 0.0, 0.0);
    let look = Vec3::new(0.0, 0.0, -1.0);
    let up   = Vec3::new(0.0, 1.0, 0.0);
    let focus = 0.5f32;
    // image parameters
    let width:i32  = 100;
    let height:i32 = 100;
    // code
    // TODO: put into matrix
    let u = na::cross(&look, &up);
    let v = na::cross(&u, &look);
    let mat = Mat3::new(
        u.x, v.x, look.x,
        u.y, v.y, look.y,
        u.z, v.z, look.z,
    );
    let halfwidth  = (width  as f32) / 2.0;
    let halfheight = (height as f32) / 2.0;
    // test scene
    let my_sphere = Sphere { center: Pnt3::new(0.0, 0.0, -5.0), radius: 1.0 };
    // render image
    for y in 0..height {
        for x in 0..width {
            // transform to (-1, 1)
            let pos = Vec3::new(
                ((x as f32) - halfwidth)  / halfwidth,
                ((y as f32) - halfheight) / halfheight,
                focus
            );
            let ray = Ray { origin: eye, direction: (mat * pos).normalize() };
        }
    }
}
