//! Quick BMP file module
//!
//! This module contains functions to write BMP files.

use std::io;
use std::io::prelude::*;
use std::fs::File;

/// Write BMP file header, returning the length of a row in bytes
pub fn write_header(f: &mut File, width: u32, height: u32) -> io::Result<u32> {
    let bytewidth = (3 * width + 3) & 0xFFFFFFFC;
    let pasize = bytewidth * height; // size of pixel array
    let fsize = 14 + 108 + pasize;
    try!(f.write(&[
        0x42u8, 0x4Du8, // "BM"
        ((fsize)       & 0xFF) as u8, // file size in little endian
        ((fsize >> 8)  & 0xFF) as u8,
        ((fsize >> 16) & 0xFF) as u8,
        ((fsize >> 24) & 0xFF) as u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // implementation defined
        0x7Au8, 0x00u8, 0x00u8, 0x00u8, // offset of pixel array
        0x6Cu8, 0x00u8, 0x00u8, 0x00u8, // size of DIB header
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
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // no red bitmask
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // no green bitmask
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // no blue bitmask
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // no alpha bitmask
        0x42u8, 0x47u8, 0x52u8, 0x73u8, // sRGB
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8,
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // no CIEXYZTRIPLE endpoints
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // no red gamma
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // no green gamma
        0x00u8, 0x00u8, 0x00u8, 0x00u8, // no blue gamma
    ]));
    Ok(bytewidth)
}
