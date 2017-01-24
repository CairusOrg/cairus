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
 */

//! Defines Cairus types
//!
//! Currently the only types here are for representing color.

/// Represents color with red, green, blue, and alpha channels.
#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Rgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    // The opacity channel
    pub alpha: f32,
}

impl Rgba {
    /// Returns an Rgba struct.
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Rgba {
        // Each color is multiplied by the alpha channel because this ensures that operations on
        // this Rgba are correct.  This is called pre-multiplied alpha.
        //
        // See the Nvidia article in the references section below on why pre-multiplying always
        // gives the correct result, and post multiplying sometimes doesn't.
        //
        // Note: All compositing operations in Cairus assume that the Rgba is pre-multiplied.
        Rgba{
            red: red * alpha,
            green: green * alpha,
            blue: blue * alpha,
            alpha: alpha}
    }

    /// Returns a vector of bytes representing the Rgba values.
    ///
    /// Each channel gets converted from a float to a byte (which can represent numbers up to 255).
    /// They are divided by the alpha value to 'factor out' colors being pre-multiplied (see method
    /// Rgba::new() on pre-multiplied alpha).
    pub fn into_bytes(&self) -> Vec<u8> {
        vec![
             (self.red * 255. / self.alpha) as u8,  (self.green * 255. / self.alpha) as u8,
             (self.blue * 255. / self.alpha) as u8, (self.alpha * 255.) as u8
            ]
    }

    /// Modifies all RGBA values to be between 1.0 and 0.0.
    /// Any value greater than 1.0 resets to 1.0, any value lower than 0.0 resets to 0.0.  This is
    /// not a feature of color theory, but of Cairo (it also corrects bad Rgba values without
    /// throwing errors).
    pub fn correct(&mut self) {
        // Because Rgba is pre-multiplied by the alpha value, if alpha is zero or less then all
        // channels are zero.
        if self.alpha < 0. {
            self.red = 0.;
            self.green = 0.;
            self.blue = 0.;
            self.alpha = 0.;
        } else {
            // Bound every channel between 0 and 1
            self.red = self.red.min(1.).max(0.);
            self.green = self.green.min(1.).max(0.);
            self.blue = self.blue.min(1.).max(0.);
            self.alpha = self.alpha.min(1.).max(0.);
        }
    }
}

impl PartialEq for Rgba {
    fn eq(&self, other: &Rgba) -> bool {
        self.red == other.red && self.green == other.green &&
        self.blue == other.blue && self.alpha == other.alpha
    }
}

#[cfg(test)]
mod tests {
    use super::Rgba;

    #[test]
    fn test_rgba_into_bytes_all_ones() {
        let color = Rgba::new(1., 1., 1., 1.);
        let expected = vec![255, 255, 255, 255];
        assert_eq!(color.into_bytes(), expected);
    }

    #[test]
    fn test_rgba_into_bytes_all_zeroes() {
        let color = Rgba::new(0., 0., 0., 0.);
        let expected = vec![0, 0, 0, 0];
        assert_eq!(color.into_bytes(), expected);
    }

    #[test]
    fn test_rgba_into_bytes_all_half() {
        let color = Rgba::new(0.5, 0.5, 0.5, 0.5);
        let expected = vec![127, 127, 127, 127];
        assert_eq!(color.into_bytes(), expected);
    }

    #[test]
    fn test_rgba_corrects_large_values() {
        let mut color = Rgba::new(3., 3., 3., 3.);
        color.correct();
        assert_eq!(color, Rgba::new(1., 1., 1., 1.));
    }

    #[test]
    fn test_rgba_corrects_small_values() {
        let mut color = Rgba::new(-3., -3., -3., -3.);
        color.correct();
        assert_eq!(color, Rgba::new(0., 0., 0., 0.));
    }

}
