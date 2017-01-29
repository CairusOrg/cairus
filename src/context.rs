/*
 * Cairus - a reimplementation of the cairo graphics library in Rust
 *
 * Copyright © 20XX CairusOrg
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



/*struct cairo_array{
    size: u64,
    num_elements: u64,
    element_size: u64, //no need
    elements: &char //DONT NEED IT FOR NOW
}

impl cairo_array{

    fn new(size: u64, num_elements: u64, element_size: u64, elements: &char)->cairo_array{
        cairo_array{size: size, num_elements: num_elements, element_size: element_size, elements: elements}
    }
}*/

pub struct Context<'a>{

    //hold a surface and an rgba or just rgba
    //holds a reference to another surface

    //pub surface: &'a ImageSurface,
    pub rgba: Rgba,
    //pub ref_count: u64, // no need
    
    //pub user_data_array: cairo_array
    surface: &'a ImageSurface,

}

impl<'a> Context<'a>{

    fn create(surface: &'a ImageSurface )-> Context {

        Context{
            rgba: Rgba::new(0., 0., 0., 0.),
            surface: surface,
        }
    }

    fn set_source_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32){

        self.rgba.red = red;
        self.rgba.green = green;
        self.rgba.blue = blue;
        self.rgba.alpha = alpha;

    }

}

mod tests{

    use types::Rgba;
    use surfaces::ImageSurface;
    use context::Context;


    #[test]
    fn test_create_context(){

        let surface = ImageSurface::create(100, 100);
        let empty_context = Context::create(&surface);


    }

    #[test]
    fn test_set_rgba(){

        let surface = ImageSurface::create(100, 100);
        let mut empty_context = Context::create(&surface);
        let set_context_rgba = Context::set_source_rgba(&mut empty_context, 1., 1., 1., 1.);
    }





}



