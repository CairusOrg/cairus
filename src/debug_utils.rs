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
 *
 */

//! This module provides some debugging tools.

use std::env;
use std::path::PathBuf;
use std::ffi::OsStr;

// ## Renders a Vec of LineSegments to a '.png' file.
// This will only compile when the '--feature debug-tesselator' flag is passed to Cargo.
//
// ## How to Compile:
//       cargo test --features debug-tesselator
//  or:
//       cargo run --features debug-tesselator
//
// The following example will render the two LineSegments in red to a png file called
// located at "{project_root}/target/debug/images/{filename}_{linenumber}.png".
//
//  If debug_render! gets called in common_geometry.rs at line 611, then the file location
//  would be "/target/debug/images/common_geometry_611.png".
//
// ## Usage:
//
// ```
//      let lines = vec![
//          LineSegment::new(0., 0., 20., 20.),
//          LineSegment::new(20., 0., 0., 20.),
//      ];
//
//      debug_render!(lines, "red");
//
// ```
//
//
// The debug version
#[cfg(feature = "debug-tesselator")]
macro_rules! debug_render {
    ($lines:expr, $color:expr) => {
        {
            use $crate::types::Rgba;
            use surfaces::ImageSurface;
            use debug_utils::get_target_dir;
            use types::{Pixel, IntoPixels};
            use std::env;

            let color =
                match $color.as_ref() {
                    "red" => Rgba{red: 1., green: 0., blue: 0., alpha: 1.},
                    "blue" => Rgba{red: 0., green: 0., blue: 1., alpha: 1.},
                    "green" => Rgba{red: 0., green: 1., blue: 0., alpha: 1.},
                    "black" | _ => Rgba{red: 0., green: 0., blue: 0., alpha: 1.}
                };


            // Get the surface size by finding the positions of the most extreme pixels
            let mut max_x = 0 ;
            let mut max_y = 0;
            for line in $lines.iter() {
                for pixel in line.into_pixels() {
                    if pixel.x > max_x {
                        max_x = pixel.x;
                    }

                    if pixel.y > max_y {
                        max_y = pixel.y;
                    }
                }

            }
            // Buffer edges by 20 pixels
            max_x = max_x + 20;
            max_y = max_y + 20;

            let mut surface = ImageSurface::create(max_x as usize, max_y as usize);

            // Actually color in the pixels
            for line in $lines.iter() {
                for pixel in line.into_pixels() {
                    match surface.get_mut(pixel.x as usize, pixel.y as usize) {
                        Some(pixel) => {
                            pixel.red = color.red;
                            pixel.blue = color.blue;
                            pixel.green = color.green;
                            pixel.alpha = color.alpha;
                        },
                        None => {},
                    }
                }
            }


            // Push folders onto path
            let mut path = get_target_dir();
            path.push("debug");
            path.push("images");

            // If this macro is in a loop, the filename and line number will be the same
            // Here we store the number of times we call this function an a environment variable
            // with key={filename}{lineno}
            let copy = path.clone();
            let string = match copy.to_str() {
                Some(val) => val,
                _ => "",
            };

            let lineno = line!().to_string();
            let split_path: Vec<&str> = file!().split("/").collect();
            let filename = split_path[1].replace(".rs", "_") + &lineno.to_string();
            let count = match env::var(string) {
                Ok(val) => {
                    val.parse::<i32>().unwrap() + 1
                },
                Err(_) => {
                    1
                }
            };

            let key = string.clone();
            env::set_var(key, count.to_string());
            let count = format!("_{}", count);
            let filename = filename + &count.to_string();
            let extension = ".png";
            path.push(filename + &extension.to_string());
            surface.to_file(path.as_path());
            path
        }
    }
}

#[cfg(feature = "debug-tesselator")]
macro_rules! debug_render_traps {
    ($traps:expr, $color:expr) => {
        {
            use $crate::types::Rgba;
            use surfaces::ImageSurface;
            use debug_utils::get_target_dir;
            use types::IntoPixels;
            use trapezoid_rasterizer::mask_from_trapezoids;
            use operators::{operator_in, operator_over};
            use std::env;

            // Get the surface size by finding the positions of the most extreme pixels
            let mut max_x = 0 ;
            let mut max_y = 0;
            for trap in $traps.iter() {
                for pixel in trap.into_pixels() {
                    if pixel.x > max_x {
                        max_x = pixel.x;
                    }

                    if pixel.y > max_y {
                        max_y = pixel.y;
                    }
                }

            }
            // Buffer edges by 20 pixels
            max_x = max_x + 20;
            max_y = max_y + 20;

            let mut destination = ImageSurface::create(max_x as usize, max_y as usize);
            let mut mask = mask_from_trapezoids(&$traps, max_x as usize, max_y as usize);
            let mut source = ImageSurface::create(max_x as usize, max_y as usize);



            let color =
                match $color.as_ref() {
                    "red" => Rgba{red: 1., green: 0., blue: 0., alpha: 1.},
                    "blue" => Rgba{red: 0., green: 0., blue: 1., alpha: 1.},
                    "green" => Rgba{red: 0., green: 1., blue: 0., alpha: 1.},
                    "black" | _ => Rgba{red: 0., green: 0., blue: 0., alpha: 1.}
                };

            for pixel in source.iter_mut() {
                pixel.red = color.red;
                pixel.green = color.green;
                pixel.blue = color.blue;
                pixel.alpha = color.alpha;
            }


            for (idx, mut mask_pixel) in mask.iter_mut().enumerate() {
                match source.get_with_index(idx) {
                    Some(src_pixel) => {
                        operator_in(&src_pixel, &mut mask_pixel);
                    },
                    None => {}
                }
            }

            for (idx, mask_pixel) in mask.iter().enumerate() {
                match destination.get_mut_with_index(idx) {
                    Some(mut dest_pixel) => {
                        operator_over(&mask_pixel, &mut dest_pixel);
                    },
                    None => {}
                }
            }

            // Push folders onto path
            let mut path = get_target_dir();
            path.push("debug");
            path.push("images");

            // If this macro is in a loop, the filename and line number will be the same
            // Here we store the number of times we call this function an a environment variable
            // with key={filename}{lineno}
            let copy = path.clone();
            let string = match copy.to_str() {
                Some(val) => val,
                _ => "",
            };

            let lineno = line!().to_string();
            let split_path: Vec<&str> = file!().split("/").collect();
            let filename = split_path[1].replace(".rs", "_") + &lineno.to_string();
            let count = match env::var(string) {
                Ok(val) => {
                    val.parse::<i32>().unwrap() + 1
                },
                Err(_) => {
                    1
                }
            };

            let key = string.clone();
            env::set_var(key, count.to_string());
            let count = format!("_{}", count);
            let filename = filename + &count.to_string();
            let extension = ".png";
            path.push(filename + &extension.to_string());
            destination.to_file(path.as_path());
            path
        }
    }
}


// Get absolute path to the "target" directory ("build" dir)
pub fn get_target_dir() -> PathBuf {
    let bin = env::current_exe().expect("exe path");
    let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
    while target_dir.file_name() != Some(OsStr::new("target")) {
        target_dir.pop();
    }
    target_dir
}

// Non-debug version
// This is here so that when the '--feature debug-tesselator' flag is not set
// the compiler will still compile but this macro won't generate any code.
#[cfg(not(feature = "debug-tesselator"))]
macro_rules! debug_render {
    ($lines:expr, $color:expr) => {
        {
            use std::path::Path;
            let path = Path::new("not a real path");
            path
        }
    };
}

// Unused imports are allowed because as the 'debug-tesselator' flag is turned on and off,
// certain imports become used and unused.
#[allow(unused_imports)]
#[macro_use]
#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs;
    extern crate image;
    use common_geometry::LineSegment;
    use trapezoid_rasterizer::Trapezoid;
    use super::get_target_dir;

    // Tests that an image is output when the debug-tesselator feature flag is set
    #[cfg(feature = "debug-tesselator")]
    #[test]
    fn test_debug_render_lines_flag_on() {

        // Setup
        let lines = vec![
            LineSegment::new(0., 0., 20., 20.),
            LineSegment::new(20., 0., 0., 20.),
        ];

        // Test
        let path = debug_render!(lines, "red");
        let img = image::open(&path).unwrap().to_rgba();
        let mut passed = false;
        for pixel in img.pixels() {
            let r = pixel.data[0];
            if r > 0 {
                passed = true;
            }
        }

        // Cleanup
        fs::remove_file(path).unwrap();
        assert!(passed);
    }

    // Tests that an image is output when the debug-tesselator feature flag is set
    #[cfg(feature = "debug-tesselator")]
    #[test]
    fn test_debug_render_fancy_lines_flag_on() {

        // Setup
        let mut lines = Vec::new();
        for x in 0..500 {
            if x % 25 == 0 {
                let upper_y = ((x + 20) as f32).min(500.);
                let lower_y = ((x - 20) as f32).max(1.);
                if lower_y < 0. {
                    panic!("Can not be lower than zero");
                }
                let line = LineSegment::new(x as f32, lower_y, x as f32, upper_y);
                lines.push(line);
            }
        }

        let line = LineSegment::new(0., 0., 500., 500.);
        lines.push(line);

        // Test
        let path = debug_render!(lines, "black");
        let img = image::open(&path).unwrap().to_rgba();
        let mut passed = false;
        for pixel in img.pixels() {
            let alpha = pixel.data[3];
            if alpha > 0 {
                passed = true;
            }
        }

        // Cleanup
        fs::remove_file(path).unwrap();
        assert!(passed);
    }


    // Tests that an image is output when the debug-tesselator feature flag is set
    #[cfg(not(feature = "debug-tesselator"))]
    #[test]
    fn test_debug_render_lines_flag_off() {
        // Test
        debug_render!(lines, "red");
        let mut path = PathBuf::new();
        path.push(get_target_dir());
        path.push("images");
        path.push("debug_utils_246.png"); // Must be line number of debug_render! call
        // Cleanup
        assert_eq!(path.exists(), false);
    }

    // Tests that an image is output when the debug-tesselator feature flag is set
    #[cfg(feature = "debug-tesselator")]
    #[test]
    fn test_debug_render_traps_flag_on() {
        let base1 = LineSegment::new(0., 0., 400., 0.);
        let base2 = LineSegment::new(100., 500., 300., 500.);

        // Setup
        let trapezoids = vec![Trapezoid::from_bases(base1, base2)];

        // Test
        let path = debug_render!(trapezoids, "red");
        let img = image::open(&path).unwrap().to_rgba();
        let mut passed = false;
        for pixel in img.pixels() {
            let r = pixel.data[0];
            if r > 0 {
                passed = true;
            }
        }

        // Cleanup
        fs::remove_file(path).unwrap();
        assert!(passed);
    }

    // Tests that an image is output when the debug-tesselator feature flag is set
    #[cfg(feature = "debug-tesselator")]
    #[test]
    fn test_debug_render_in_loop() {
        // Setup
        let line1 = LineSegment::new(0., 0., 400., 0.);
        let line2 = LineSegment::new(100., 500., 300., 500.);

        for line in vec![line1, line2] {
            // Test
            let path = debug_render!(vec![line], "red");
            let img = image::open(&path).unwrap().to_rgba();
            let mut passed = false;
            for pixel in img.pixels() {
                let r = pixel.data[0];
                if r > 0 {
                }
                    passed = true;
                }

            // Cleanup
            fs::remove_file(path).unwrap();
            assert!(passed);
        }
    }

    // Tests that an image is output when the debug-tesselator feature flag is set
    #[cfg(feature = "debug-tesselator")]
    #[test]
    fn test_debug_render_traps_macro() {
        let base1 = LineSegment::new(0., 0., 400., 0.);
        let base2 = LineSegment::new(100., 500., 300., 500.);

        let base3 = LineSegment::new(300., 0., 900., 0.);
        let base4 = LineSegment::new(700., 1000., 800., 1000.);

        // Setup
        let trapezoids = vec![Trapezoid::from_bases(base1, base2), Trapezoid::from_bases(base3, base4)];

        // Test
        let path = debug_render_traps!(trapezoids, "red");
/*
        let img = image::open(&path).unwrap().to_rgba();
        let mut passed = false;
        for pixel in img.pixels() {
            let r = pixel.data[0];
            if r > 0 {
                passed = true;
            }
        }
*/
        // Cleanup
//        fs::remove_file(path).unwrap();
//        assert!(passed);
    }
}
