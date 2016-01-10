extern crate nalgebra as na;
extern crate libraytrace;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

use libraytrace::types::*;
use libraytrace::raytrace;
use libraytrace::bmp;
use libraytrace::serialize;

fn main() {
    // image parameters
    let width:u32  = 100;
    let height:u32 = 100;
    // read a file
    let scene = {
        let file = match File::open("test_scene.txt") {
            Ok(f) => f,
            Err(e) => { println!("error: {}", e); return },
        };
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        match reader.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(e) => { println!("error: {}", e); return },
        }
        match serialize::deserialize(&contents) {
            Ok(s) => s,
            Err(e) => { println!("error: {}", e); return },
        }
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
            let color = raytrace::raytrace(&scene, &pos, 1.0);
            color.write_bgr(&mut row, x as usize);
        }
        file_handle.write_all(&row[..]).ok().expect("error writing row");
    }
}
