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
 *	Sara Ferdousi <ferdousi@pdx.edu>
 *
 */

use surfaces::ImageSurface;
use types::Rgba;

//Struct defined for context
pub struct Context<'a>{
    pub rgba: Rgba,
    //target surface
    target: &'a ImageSurface,
}

//Implementation of methods for context
impl<'a> Context<'a>{

    //default constructor. Sets Rgba values to zeroes and set the target to passed ImageSurface.
    //When new context is created a target surface needs to be passed in.
    pub fn create(target: &'a ImageSurface )-> Context {
        Context{
            rgba: Rgba::new(0., 0., 0., 0.),
            target: target,
        }
    }

    //Sets Rgba values of source
    //This function changes the Rgba values of the source
    pub fn set_source_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32){
        self.rgba.red = red;
        self.rgba.green = green;
        self.rgba.blue = blue;
        self.rgba.alpha = alpha;
    }
}

//Unit tests
mod tests{
    use types::Rgba;
    use surfaces::ImageSurface;
    use context::Context;

    //Test to check if the constructors work
    #[test]
    fn test_create_context(){
        let surface = ImageSurface::create(100, 100);
        let empty_context = Context::create(&surface);

    }

    //Testing set_rgba function
    #[test]
    fn test_set_rgba(){
        let surface = ImageSurface::create(100, 100);
        let mut empty_context = Context::create(&surface);
        let set_context_rgba = Context::set_source_rgba(&mut empty_context, 1., 1., 1., 1.);
        assert_eq!(empty_context.rgba.blue, 1.);
        assert_eq!(empty_context.rgba.green, 1.);
        assert_eq!(empty_context.rgba.red, 1.);
        assert_eq!(empty_context.rgba.alpha, 1.);
    }

}



