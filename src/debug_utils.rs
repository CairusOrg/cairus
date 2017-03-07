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
 *	Bobby Eshleman <bobbyeshleman@gmail.com>
 *
 */

// The debug version
#[cfg(feature = "debug-tesselator")]
macro_rules! debug_render_lines {
    ($lines:expr, $color:expr, $width:expr, $height:expr, $pathname:expr) => {
        use types::Rgba;
        use surfaces::ImageSurface;
        use std::path::Path;


        let color =
            match $color.as_ref() {
                "red" => Rgba{red: 1., green: 0., blue: 0., alpha: 1.},
                "blue" => Rgba{red: 0., green: 0., blue: 1., alpha: 1.},
                "green" => Rgba{red: 0., green: 1., blue: 0., alpha: 1.},
                "black" | _ => Rgba{red: 1., green: 1., blue: 1., alpha: 1.}
            };

        let mut surface = ImageSurface::create($width, $height);

        for line in $lines {
            for (x, y) in line.into_pixel_coordinates() {
                let mut pixel = surface.get_mut(x as usize, y as usize);
                pixel.red = color.red;
                pixel.blue = color.blue;
                pixel.green = color.green;
                pixel.alpha = color.alpha;
            }
        }

        let path = Path::new($pathname);
        surface.to_file(path);
    }
}

// Non-debug version
#[cfg(not(feature = "debug-tesselator"))]
macro_rules! debug_render_lines {
    ($lines:expr, $color:expr, $width:expr, $height:expr, $pathname:expr) => {}
}


// Unused imports are allowed because as the 'debug-tesselator' flag is turned on and off,
// certain imports become used and unused. 
#[allow(unused_imports)]
#[macro_use]
#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs;
    extern crate image;
    use common_geometry::LineSegment;

    // Tests that an image is output when the debug-tesselator feature flag is set
    #[cfg(feature = "debug-tesselator")]
    #[test]
    fn test_debug_render_lines_flag_on() {

        // Setup
        let lines = vec![
            LineSegment::new(0., 0., 20., 20.),
            LineSegment::new(20., 0., 0., 20.),
        ];
        let path = Path::new("debug_test.png");

        // Test
        debug_render_lines!(lines, "red", 25, 25, "debug_test.png");
        let img = image::open(path).unwrap().to_rgba();
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
    #[cfg(not(feature = "debug-tesselator"))]
    #[test]
    fn test_debug_render_lines_flag_off() {
        // Test
        debug_render_lines!(lines, "red", 25, 25, "debug_test.png");
        let path = Path::new("debug_test.png");
        let exists = path.exists();

        // Cleanup
        if exists {
            fs::remove_file(path).unwrap();
        }

        assert_eq!(exists, false);
    }

}
