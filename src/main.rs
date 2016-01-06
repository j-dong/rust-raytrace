extern crate nalgebra as na;
extern crate libraytrace;

use std::io::prelude::*;
use std::fs::File;

use libraytrace::types::*;
use libraytrace::scene::*;
use libraytrace::shapes::*;
use libraytrace::camera::*;
use libraytrace::color::Color;
use libraytrace::raytrace;
use libraytrace::bmp;

fn main() {
    // camera parameters
    let eye  = Pnt3::new(0.0, 0.0, 0.0);
    let look = Vec3::new(0.0, 0.0, -1.0);
    let up   = Vec3::new(0.0, 1.0, 0.0);
    let im_dist = 0.5f32;
    // image parameters
    let width:u32  = 100;
    let height:u32 = 100;
    // test scene
    let scene = Scene {
        objects: vec!(
            Object {
                bounds: Box::new(Sphere {
                    center: Pnt3::new(0.0, 0.0, -2.0),
                    radius: 1.0,
                }),
                material: Material {
                    diffuse: Color::from_rgb(1.0, 1.0, 1.0),
                },
            },
        ),
        lights: vec!(
            Light {
                model: Box::new(DirectionalLight {
                    direction: -look,
                }),
                color: Color::from_rgb(1.0, 1.0, 1.0),
            },
        ),
        camera: Box::new(
            SimplePerspectiveCamera::new(&eye, &look, &up, im_dist)
        ),
    };
    // write BMP
    let mut file_handle = File::create("out.bmp")
                          .ok().expect("error creating out.bmp");
    let bytewidth = bmp::write_header(&mut file_handle, width, height)
                    .ok().expect("error writing BMP header");
    // render image
    let halfwidth  = (width  as f32) / 2.0;
    let halfheight = (height as f32) / 2.0;
    let mut row: Vec<u8> = vec![0; bytewidth as usize];
    for y in 0..height {
        // actual image y coordinate
        let y = height - 1 - y;
        for x in 0..width {
            // transform to (-1, 1)
            let pos = Pnt2::new(
                ((x as f32) - halfwidth)  / halfwidth,
                ((y as f32) - halfheight) / halfheight,
            );
            let color = raytrace::raytrace(&scene, &pos);
            color.write_bgr(&mut row, x as usize);
        }
        file_handle.write_all(&row[..]).ok().expect("error writing row");
    }
}
