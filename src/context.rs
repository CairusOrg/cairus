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
 *  Evan Smelser <evanjsmelser@gmail.com>
 */



use surfaces::ImageSurface;
use types::Rgba;
use operators::{Operator, fetch_operator};


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
    surface: &'a mut ImageSurface,
    operator: Operator,
}

impl<'a> Context<'a>{

    fn create(surface: &'a mut ImageSurface, operator: Operator)-> Context {

        Context{
            rgba: Rgba::new(0., 0., 0., 0.),
            surface: surface,
            operator: operator,
        }
    }

    fn set_source_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32){

        self.rgba.red = red;
        self.rgba.green = green;
        self.rgba.blue = blue;
        self.rgba.alpha = alpha;

    }

    ///Set Operator function.
    ///Changes the operator held by the context object to the passed in operator.
    ///The operator passed in is just a copy of the enum which gives the context knowledge of the
    ///current operator in use.
    fn set_operator(&mut self, operator: Operator){
        self.operator = operator;
    }

    fn get_operator(&self)-> &Operator{
        &self.operator
    }

    /// Paints this context's Rgba on the destination surface with the over operator.
    ///
    /// This is a completely naive, and frankly useless implementation.  It is a place holder for
    /// the real paint function to later be implemented.  It operates on every 'pixel' of the
    /// destination surface.
    pub fn paint(&mut self) {
        let op = Operator::Over;
        let operator = fetch_operator(&op);
        for mut pixel in self.surface.iter_mut() {
            operator(&self.rgba, pixel);
        }
    }

}


/// # References
/// [Cairo Operators]: https://www.cairographics.org/operators/

#[cfg(test)]
mod tests{
    use surfaces::ImageSurface;
    use types::Rgba;
    use operators::{Operator, fetch_operator};
    use super::Context;
    use super::paint;
    use super::set_operator;
    use super::get_operator;
    use super::create;
    use super::set_source_rgba;

    #[test]
    fn test_get_default_operator(){
        let context = Context::create(
            ImageSurface::create(255, 255),
            Operator::
            );
    }

    #[test]
    fn test_set_get_operator(){

    }



}

