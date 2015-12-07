extern crate nalgebra;
use nalgebra::{Pnt3, Vec3};

struct Ray {
    origin: Pnt3<f32>,
    direction: Vec3<f32>,
}

fn main() {
    // camera parameters
    let eye  = Pnt3::new(0.0f32, 0.0, 0.0);
    let look = Vec3::new(0.0f32, 0.0, -1.0);
    let up   = Vec3::new(0.0f32, 1.0, 0.0);
    let focus = 0.5f32;
    // image parameters
    let width:i32  = 100;
    let height:i32 = 100;
    // code
    // TODO: put into matrix
    let u = nalgebra::cross(&look, &up);
    let v = up;
    let w = look;
    let halfwidth  = (width  as f32) / 2.0;
    let halfheight = (height as f32) / 2.0;
    // render image
    for y in 0..height {
        for x in 0..width {
            let xx = ((x as f32) - halfwidth)  / halfwidth;
            let yy = ((y as f32) - halfheight) / halfheight;
            let ray = Ray { origin: eye, direction: u * xx + v * yy + w * focus };
        }
    }
}
