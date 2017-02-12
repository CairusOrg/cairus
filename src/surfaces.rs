/*
 * Cairus - a reimplementation of the cairo graphics library in Rust
 *
 * Copyright © 2017 CairusOrg
 *
 * This library is free software; you can redistribute it and/or
 * modify it either under the terms of the GNU Lesser General Public
 * License version 2.1 as published by the Free Software Foundation
 * (the "LGPL") or, at your option, under the terms of the Mozilla
 * Public License Version 2.0 (the "MPL"). If you do not alter this
 * notice, a recipient may use your version of this file under either
 * the MPL or the LGPL.
 *
 * You should have received a copy of the LGPL along with this library
 * in the file LICENSE-LGPL-2_1; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Suite 500, Boston, MA 02110-1335, USA
 * You should have received a copy of the MPL along with this library
 * in the file LICENSE-MPL-2_0
 *
 * The contents of this file are subject to the Mozilla Public License
 * Version 2.0 (the "License"); you may not use this file except in
 * compliance with the License. You may obtain a copy of the License at
 * http://www.mozilla.org/MPL/
 *
 * This software is distributed on an "AS IS" basis, WITHOUT WARRANTY
 * OF ANY KIND, either express or implied. See the LGPL or the MPL for
 * the specific language governing rights and limitations.
 *
 * The Original Code is the cairus graphics library.
 *
 * Contributor(s):
 *  Bobby Eshleman <bobbyeshleman@gmail.com>
 *  Evan Smelser <evanjsmelser@gmail.com>
 *  Kyle Kneitinger <kneit@pdx.edu>
 *
 */

//! # Overview
//! Cairo surfaces are basically raster (bitmap) containers.  They 'receive' operations performed
//! on them by contexts.  They are the 'canvas' of Cairus.

use std::fs::File;
use std::path::Path;
use std::slice::{IterMut, Iter};
use std::vec::IntoIter;
use types::Rgba;
extern crate image;



///Format enum descriptors for the surface object
///These are specifically the format types copied from the C implementation,
///analogous to cairo_format_t
#[allow(non_camel_case_types)]
pub enum Format {
    Invalid,
    ARGB32,
    RGB24,
    A8,
    A1,
    RGB16_565,
    RGB30,
}

/// Analogous to cairo_surface_type_t, indicates target drawing type
pub enum Type {
    Image,
    Pdf,
    Ps,
    Xlib,
    Xcb,
    Glitz,
    Quartz,
    Win32,
    Beos,
    Directfb,
    Svg,
    Os2,
    Win32Printing,
    QuartzImage,
    Script,
    Qt,
    Recording,
    Vg,
    Gl,
    Drm,
    Tee,
    Xml,
    Skia,
    Subsurface,
    Cogl,
}

/// A surface needs to hold pixels (Rgba's) and its width and height.  The width and height
/// will be used in rendering to images and calculating clipping, and the pixels will be the things
/// that actually are operated on by stroke or paint operations.  See the
/// `test_image_surface_with_operator` test case below for an example of what that might look like.
pub struct ImageSurface {
    // base is just a collection of pixels
    base: Vec<Rgba>,
    width: usize,
    height: usize,
}

/// ImageSurface provides iter(), into_iter(), and iter_mut() so that when a Cairus context calls
/// paint, it can simply iterate through the pixels in the image surface and use a image
/// compositing operator to operate on them.  See `operators.rs` for those operations.
impl ImageSurface {
    // Analogous to cairo_create(), you pass in a width and height and get in a surface in exchange.
    pub fn create(width: usize, height: usize) -> ImageSurface {
        ImageSurface {
            base: vec![Rgba::new(0., 0., 0., 0.); width * height],
            width: width,
            height: height,
        }
    }

    fn iter(&self) -> Iter<Rgba> {
        self.base.iter()
    }

    fn iter_mut(&mut self) -> IterMut<Rgba> {
        self.base.iter_mut()
    }

    fn into_bytes(& self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for pixel in self.base.iter() {
            for byte in pixel.into_bytes() {
              bytes.push(byte);
            }
        }
        bytes
    }

    pub fn to_file(&self, path: &Path){
        let extension = path.extension().unwrap();

        if extension == "jpg" {
            self.to_png_jpg(path);
        }
        else {
         //   panic!("error: {:?} output not supported", extension);
        }
    }

    fn to_png_jpg(&self, path: &Path) {
        let buffer = self.into_bytes();
        let our_image = image::save_buffer(path, buffer.as_slice(), self.width as u32, self.height as u32, image::RGBA(8)).unwrap();

    }
}
///Test the equality of formats (useful to ensure the output file is rendered as a PNG
//impl PartialEq for ImageSurface{
//    fn eq(&self, other: &ImageFormat) -> bool {
//        self.ImageFormat == other.ImageFormat
//    }
//} Needs to reference the ENUM PNG within Image crate to compile

impl IntoIterator for ImageSurface {
    type Item = Rgba;
    type IntoIter = IntoIter<Rgba>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use types::Rgba;
    use surfaces::ImageSurface;
    use operators::{Operator, fetch_operator};
    use surfaces::image::GenericImage;

    use std::fs::File;
    use std::path::Path;
    extern crate image;


    #[test]
    fn test_image_surface_create() {
        // Test that ImageSurface's IntoIterator is functioning correctly by comparing every pixel
        // in the surface to the default (which is transparent).
        let transparent_pixel = Rgba::new(0., 0., 0., 0.);
        let surface = ImageSurface::create(100, 100);
        for pixel in surface {
            assert_eq!(pixel, transparent_pixel);
        }
    }

    #[test]
    fn test_image_surface_into_iter() {
        // Test that the explicit into_iter() call functions correctly.
        let transparent_pixel = Rgba::new(0., 0., 0., 0.);
        let surface = ImageSurface::create(100, 100);
        for pixel in surface.into_iter() {
            assert_eq!(pixel, transparent_pixel);
        }
    }

    // TODO: test into_iter().map()

    #[test]
    fn test_image_surface_iter() {
        // Passes if ImageSurface::iter() functions properly
        let surface = ImageSurface::create(100, 100);

        // Leave pixel.red to default (0.0), change all other channels to 1.0
        let result = surface.iter()
            .map(|&pixel| {
                Rgba {
                    red: pixel.red,
                    green: 1.,
                    blue: 1.,
                    alpha: 1.,
                }
            })
            .collect::<Vec<Rgba>>();

        let expected = Rgba {
            red: 0.,
            green: 1.,
            blue: 1.,
            alpha: 1.,
        };

        for pixel in result.into_iter() {
            // Red is 0. because it is the default, the others got set to 1.
            assert_eq!(pixel, expected);
        }
    }

    #[test]
    fn test_image_surface_iter_mut() {
        // Passes if ImageSurface::iter_mut() functions properly
        let mut surface = ImageSurface::create(100, 100);
        let expected = Rgba::new(1., 0., 0., 1.);

        for mut pixel in surface.iter_mut() {
            pixel.alpha = expected.alpha;
            pixel.red = expected.red;
        }

        for pixel in surface {
            assert_eq!(pixel, expected);
        }
    }

    #[test]
    fn test_image_surface_with_operator() {
        // Demonstrates usage with an operator
        //
        // Our goal here is to take a surface and paint it red.  We use the the surface's iter_mut
        // function because operators modify the image's pixels in-place.

        // Create our source Rgba, destination, and choose an operator
        let source_rgba = Rgba::new(1., 0., 0., 1.);
        let mut destination = ImageSurface::create(100, 100);
        let op = Operator::Over;

        // Using fetch_operator and the Operator enum.
        let operator = fetch_operator(&op);
        for mut pixel in destination.iter_mut() {
            operator(&source_rgba, pixel);
        }

        // Check that the resulting pixels in destination are red RGBA(1, 0, 0, 1)
        let expected = Rgba::new(1., 0., 0., 1.);
        for pixel in destination {
            assert_eq!(pixel, expected);
        }
    }

    #[test]
    fn test_image_surface_to_png() {
        // Setup
        let surface = ImageSurface::create(200, 200);
        let path = Path::new("test1.png");

        // Call
        surface.to_png_jpg(path);

        // Test
    }

    #[test]
    fn test_to_png_output_correct_dimensions() {
        //setup
        let surface = ImageSurface::create(100, 100);
        let path = Path::new("test2.png");
        let expected_width = surface.width as u32;
        let expected_height = surface.height as u32;
        //call
        surface.to_png_jpg(path);
        let img = image::open(path).unwrap();
        let (result_width, result_height) = img.dimensions();
        //test
        assert_eq!(result_width, expected_width);
        assert_eq!(result_height, expected_height);
    }

//    #[test]
//    fn test_int_to_png_intergrity_per_pixel() {
//        //setup
//        let surface = ImageSurface::create(100, 100);
//        //let transparent_pixel = image::RGBA(i as u8);
//        let path = Path::new("test3.png");
//        //call
//        surface.to_png_jpg(path);
//        let img = image::open(path).unwrap();
//        //test
//        for pixel in img.pixels() {
//            let (_,_, apixel) = pixel;
//            //let () = apixel;
//            let channels = pixel.channels4();
//            //assert_eq!(pixel, transparent_pixel);
//            //println!("{:?}", channels);
//        }
//    }

    #[test]
    fn test_to_file_happy_path() {
        // Setup
        let surface = ImageSurface::create(100,100);
        let path = Path::new("test.jpg");

        // Call
        surface.to_file(path);
        // Test
    }
}
