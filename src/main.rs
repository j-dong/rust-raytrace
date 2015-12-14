extern crate nalgebra as na;
extern crate raytrace;

use std::io::prelude::*;
use std::fs::File;

use self::na::{Norm,Dot};

use raytrace::shapes::{Ray, Sphere, Shape};
use raytrace::types::*;

fn main() {
    // camera parameters
    let eye  = Pnt3::new(0.0, 0.0, 0.0);
    let look = Vec3::new(0.0, 0.0, -1.0);
    let up   = Vec3::new(0.0, 1.0, 0.0);
    let focus = 0.5f32;
    // image parameters
    let width:u32  = 100;
    let height:u32 = 100;
    // code
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
    let my_sphere = Sphere { center: Pnt3::new(0.0, 0.0, -2.0), radius: 1.0 };
    // file handle
    let mut f = File::create("out.bmp").ok().expect("error creating out.bmp");
    let bytewidth = (3 * width + 3) & 0xFFFFFFFC;
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
    let bg = [0u8, 0u8, 0u8];
    let mut fg = [255u8, 255u8, 255u8];
    let zeros = [0u8, 0u8, 0u8];
    let padding = match width & 0x3 {
        0 => &zeros[0..0], // perfect multiple
        1 => &zeros[0..3], // remainder 1
        2 => &zeros[0..2],
        3 => &zeros[0..1],
        _ => panic!("Result of width & 0x3 not 0, 1, 2, or 3"),
    };
    // render image
    let lightdir = -look;
    for y in 0..height {
        for x in 0..width {
            // transform to (-1, 1)
            let pos = Vec3::new(
                ((x as f32) - halfwidth)  / halfwidth,
                ((y as f32) - halfheight) / halfheight,
                focus
            );
            let ray = Ray { origin: eye, direction: (mat * pos).normalize() };
            match my_sphere.intersect(&ray) {
                Some(result) => {
                    let z = ray.cast(result.t).z;
                    let color = result.normal.dot(&lightdir).abs();
                    let colbyte = (color * 256.0) as u8;
                    fg[0] = colbyte;
                    fg[1] = colbyte;
                    fg[2] = colbyte;
                    f.write(&fg);
                },
                None => {f.write(&bg);},
            }
        }
        f.write(padding);
    }
}
