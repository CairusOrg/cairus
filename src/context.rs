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
 *  Sara Ferdousi <ferdousi@pdx.edu>
 *  Evan Smelser <evanjsmelser@gmail.com>
 *  Bobby Eshleman <bobbyeshleman@gmail.com>
 *  Kyle Kneitinger <kyle@kneit.in>
 *
 */

use surfaces::ImageSurface;
use types::Rgba;
use operators::Operator;
use operators::fetch_operator;

//Struct defined for context
pub struct Context<'a>{
    pub rgba: Rgba,
    target: &'a mut ImageSurface,
    operator: Operator,
}

//Implementation of methods for context
impl<'a> Context<'a>{
    //Creates a new cairo context with rgba values set to zeroes with passed ImageSurface as target surface
    //When new context is created a target surface needs to be passed in.
    pub fn create(target: &'a mut ImageSurface )-> Context {
        Context{
            rgba: Rgba::new(0., 0., 0., 0.),
            target: target,
            operator: Operator::Over
        }
    }

    //Sets Rgba values of source to used defined values
    //This function changes the Rgba values of the source
    pub fn set_source_rgba(&mut self, red: f32, green: f32, blue: f32, alpha: f32){
        self.rgba.red = red * alpha;
        self.rgba.green = green * alpha;
        self.rgba.blue = blue * alpha;
        self.rgba.alpha = alpha;
        self.rgba.correct();
    }

    ///Set Operator function
    ///
    ///Changes the operator held by the context object to the passed in operator.
    ///The operator passed in is just a copy of the enum which gives the context knowledge of the
    ///current operator in use. 
    ///Sets the operator held within the context object to the passed in operator of choice. 
    ///
    ///# Arguments    
    ///* `&mut self` - Reference to the `Context` to hold the desired `Operator`.
    ///* `operator` - An enum `Operator` that matches the desired operation.    
    ///
    ///# Usage    
    ///set_operator(&context, op_enum);
    fn set_operator(&mut self, operator: Operator){
        self.operator = operator;
    }

    /// Get Operator function.
    ///
    /// Returns the operator held within the passed in context object.
    ///
    /// # Arguments
    /// * `&self` - Reference to the `Context` object that maintains the `Operator` functionality.
    ///
    /// # Usage
    /// let op_enum = get_operator();
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
        for mut pixel in self.target.iter_mut() {
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

    #[test]
    fn test_get_default_operator(){
        //setup
        let mut surface = ImageSurface::create(255, 255);
        let context = Context::create( &mut surface );
        //call and assert
        assert_eq!( &Operator::Over, context.get_operator() );
    }

    #[test]
    fn test_set_get_operator(){
       //My intent here is to instantiate a context and then set the operator to another
       //operator and check to see that it was actually changed. However, I only have the
       //Over operator implemented in this branch so there really isn't anything to change
       //it to here.

        //setup
        //call
        //assert
    }

    // This tests that naive paint covers the target.  It does two calls, in order to check that
    // multiple mutable borrows (via paint) work fine too.
    #[test]
    fn test_paint() {
        // Setup
        let mut target = ImageSurface::create(100, 100);

        // Call
        {
            let mut context = Context::create(&mut target);
            context.set_source_rgba(1., 0., 0., 1.);
            context.paint();
            context.set_source_rgba(0., 1., 0., 1.);
            context.paint();
        }

        // Test
        let expected = Rgba::new(0., 1., 0., 1.);
        for pixel in target.iter() {
            assert_eq!(*pixel, expected);
        }
    }

    fn test_set_rgba_happy(){
        let mut surface = ImageSurface::create(100, 100);
        let mut context = Context::create(&mut surface);
        context.set_source_rgba(0.1, 0.2, 0.3, 1.);
        assert_eq!(context.rgba.red, 0.1);
        assert_eq!(context.rgba.green, 0.2);
        assert_eq!(context.rgba.blue, 0.3);
        assert_eq!(context.rgba.alpha, 1.);

        // Test Rbga premultiply
        context.set_source_rgba(0.2, 0.4, 0.6, 0.5);
        assert_eq!(context.rgba.red, 0.1);
        assert_eq!(context.rgba.green, 0.2);
        assert_eq!(context.rgba.blue, 0.3);
        assert_eq!(context.rgba.alpha, 0.5);
    }

    #[test]
    fn test_set_rgba_out_of_bounds_values(){
        let mut surface = ImageSurface::create(100, 100);
        let mut context = Context::create(&mut surface);

        // Test negative alpha value pre-multiplting to zero
        context.set_source_rgba(1., 1., 1., -10.);
        assert_eq!(context.rgba.red, 0.);
        assert_eq!(context.rgba.green, 0.);
        assert_eq!(context.rgba.blue, 0.);
        assert_eq!(context.rgba.alpha, 0.);

        // Test bound to range [0,1]
        context.set_source_rgba(-22.,22.,-22.,9.);
        assert_eq!(context.rgba.red, 0.);
        assert_eq!(context.rgba.green, 1.);
        assert_eq!(context.rgba.blue, 0.);
        assert_eq!(context.rgba.alpha, 1.);
    }
}
