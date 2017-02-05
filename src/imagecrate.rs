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
 *
 */

use std::fs::File;
use std::path::Path;
use surfaces::ImageSurface;
use types::Rgba;
extern crate image;


/// The supported image outputs in Cairus
pub enum ImageType {
    PNG,
}

pub fn fetch_image_converter(itc: &ImageType) -> fn(&ImageSurface, &Path) {
    match *itc {
        ImageType::PNG      => output_png,
    }
}

/// Writes the image to a PNG file
fn output_png(is: &ImageSurface, path: &Path) {

}

#[cfg(test)]
mod tests {
    use super::ImageType;
    use super::fetch_image_converter;
    use super::output_png;
    use surfaces::ImageSurface;
    use std::path::Path;

    #[test]
    // checks that..
    fn test_one() {
        // Setup
        // How do i create a new image surface when create is private?
        let is =  ImageSurface::create(100,100);
        let path = Path::new("file.jpg");
        // Call
        output_png(&is, &path);
        // Test

    }
}