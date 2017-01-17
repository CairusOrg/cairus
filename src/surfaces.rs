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

use types::Rgba;

struct ImageSurface {
    base: Vec<Rgba>,
    width: usize,
    height: usize,
}

impl ImageSurface {
    fn new(width: usize, height: usize) -> ImageSurface {
        ImageSurface{
            base: vec![Rgba::new(0., 0., 0., 0.); width * height],
            width: width,
            height: height
        }
    }
}


struct ImageSurfaceIterator {
    
}

impl Iterator for ImageSurfaceIterator {
    type Item = Rgba;

    fn next(&mut self) -> Option<Self::Item> {

    }
}

impl IntoIterator for ImageSurface {
    type Item = Rgba;
    type IntoIter = ::std::vec::IntoIter<Rgba>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use types::Rgba;
    use surfaces::ImageSurface;

    #[test]
    fn test_image_surface_default() {
        // Test that ImageSurface's iterator is functioning correctly
        let default_rgba = Rgba::new(0., 0., 0., 0.);
        let surface = ImageSurface::new(100, 100);
        for pixel in surface {
            assert_eq!(pixel, default_rgba);
        }
    }

    #[test]
    fn test_image_surface_map_collect() {
        let surface = ImageSurface::new(100, 100);
        let result = surface.iter().map(|&pixel| pixel.red = 0.3).collect::<ImageSurface>();
    }
}
