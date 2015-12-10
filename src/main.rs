extern crate nalgebra as na;
extern crate raytrace;

use std::fs::File;

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
    // file handle
    let mut f = try!(File::create("out.bmp"));
    let bytewidth = (3 * width + 3) & ~0x3;
    let pasize = bytewidth * height; // size of pixel array
    let fsize = 14 + 40 + pasize;
    f.write(&[
        0x42u8, 0x4Du8, // "BM"
        ((fsize)       & 0xFF) as u8, // file size in little endian
        ((fsize >> 8)  & 0xFF) as u8,
        ((fsize >> 16) & 0xFF) as u8,
        ((fsize >> 24) & 0xFF) as u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // implementation defined
        0x36u8, 0x00u8, 0x00u8, 0x00u8, // offset of pixel array
        0x28u8, 0x00u8, 0x00u8, 0x00u8, // size of DIB header
        ((width)        & 0xFF) as u8, // width
        ((width >> 8)   & 0xFF) as u8,
        ((width >> 16)  & 0xFF) as u8,
        ((width >> 24)  & 0xFF) as u8,
        ((height)       & 0xFF) as u8, // height
        ((height >> 8)  & 0xFF) as u8,
        ((height >> 16) & 0xFF) as u8,
        ((height >> 24) & 0xFF) as u8,
        0x01u8, 0x00u8, // 1 plane
        0x18u8, 0x00u8, // 24 bpp
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // no compression
        ((pasize)       & 0xFF) as u8, // pixel array size
        ((pasize >> 8)  & 0xFF) as u8,
        ((pasize >> 16) & 0xFF) as u8,
        ((pasize >> 24) & 0xFF) as u8,
        0x13u8, 0x0Bu8, 0x00u8, 0x00u8, // resolution (72 DPI)
        0x13u8, 0x0Bu8, 0x00u8, 0x00u8, // resolution (72 DPI)
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // no color palette
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // important colors
    ]);
    let black = [0u8, 0u8, 0u8];
    let white = [255u8, 255u8, 255u8];
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
            let color = match my_sphere.intersect(ray) {
                Some(_) => &black,
                None => &white,
            }
            f.write(color);
        }
        for _ in 0..(bytewidth - 3 * width) {
            f.write(&[0]);
        }
    }
}
