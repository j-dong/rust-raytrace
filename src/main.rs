extern crate libraytrace;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

use libraytrace::types::*;
use libraytrace::color;
use libraytrace::raytrace;
use libraytrace::bmp;
use libraytrace::serialize;

fn main() {
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
    let width = scene.options.width;
    let height = scene.options.height;
    // write BMP
    let mut file_handle = File::create("out.bmp")
                          .ok().expect("error creating out.bmp");
    let bytewidth = bmp::write_header(&mut file_handle, width, height)
                    .ok().expect("error writing BMP header");
    // render image
    let halfwidth  = (width  as f64) / 2.0;
    let halfheight = (height as f64) / 2.0;
    let scale = (1.0 / halfwidth).max(1.0 / halfheight);
    let mut row: Vec<u8> = vec![0; bytewidth as usize];
    let mut rng = rand::weak_rng();
    let aasamples = scene.options.antialias;
    for y in 0..height {
        for x in 0..width {
            let mut res = color::BLACK;
            for _ in 0..aasamples {
                // transform to (-1, 1)
                let pos = Pnt2::new(
                    ((x as f64 + rng.gen::<f64>()) - halfwidth)  * scale,
                    ((y as f64 + rng.gen::<f64>()) - halfheight) * scale,
                );
                res = res + raytrace::raytrace(&scene, &pos, 1.0, &mut rng);
            }
            (res / aasamples as f64).write_bgr(&mut row, x as usize);
        }
        file_handle.write_all(&row[..]).ok().expect("error writing row");
    }
}
