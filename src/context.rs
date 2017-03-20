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
use status::Status;
use path::Path;
use filler::Filler;

/// Struct defined for context
pub struct Context<'a>{
    pub rgba: Rgba,
    pub status: Status,
    pub target: &'a mut ImageSurface,
    operator: Operator,
    pub path: Path,
    //filler: Option<Filler>,
}
/// Implementation of methods for context
impl<'a> Context<'a> {
    //Creates a new cairo context with rgba values set to zeroes with passed ImageSurface as target surface
    //When new context is created a target surface needs to be passed in.
    pub fn create(target: &'a mut ImageSurface )-> Context {
        Context{
            rgba: Rgba::new(0., 0., 0., 0.),
            target: target,
            operator: Operator::Over,
            status: Status::Success,
            path: Path::create(),
            //filler: None,
        }
    }

    /// Sets Rgba values of source to used defined values
    /// This function changes the Rgba values of the source
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
    pub fn set_error(&mut self, status: Status) {
        self.status = status;
    }

    ///Implementation of user facing path related functions

    ///Clears the current path.
    ///After this call there will be no path and the current point will be set t.
    pub fn new_path(&mut self) -> Status {
        //let mut status = Status::Success;
        if self.status != Status::Success {
            return Status::InvalidPathData;
        }

        let status = self.path.new_path();
        if status != Status::Success {
            self.set_error(status);
        }
        self.status
    }

    ///new_sub_path
    ///
    ///Begin a new sub-path. Note that the existing path is not
    ///affected. After this call there will be no current point.
    ///
    ///In many cases, this call is not needed since new sub-paths are
    ///frequently started with cairo_move_to().
    ///
    ///A call to cairo_new_sub_path() is particularly useful when
    ///beginning a new sub-path with one of the cairo_arc() calls. This
    ///makes things easier as it is no longer necessary to manually
    ///compute the arc's initial coordinates for a call to
    ///cairo_move_to().
    pub fn new_sub_path(&mut self) -> Status{
        //let mut status = Status::Success;
        if self.status != Status::Success {
            return Status::InvalidPathData;
        }

        let status = self.path.new_sub_path();
        if status != Status::Success {
            self.set_error(status);
        }
        self.status
    }

    ///move_to
    ///
    ///Begin a new sub-path. After this call the current point will be (x, y).
    pub fn move_to(&mut self, x: f32, y: f32) -> Status{
        //let mut status = Status::Success;
        if self.status != Status::Success {
            return Status::InvalidPathData;
        }

        let status = self.path.move_to(x, y);
        if status != Status::Success {
            self.set_error(status);
        }
        self.status
    }

    ///line_to
    ///
    ///Adds a line to the path from the current point to position (x, y) in user-space coordinates.
    ///After this call the current point will be (x, y)
    pub fn line_to(&mut self, x: f32, y: f32)  -> Status{
        //let mut status = Status::Success;
        if self.status != Status::Success {
            return Status::InvalidPathData;
        }

        let status = self.path.line_to(x, y);
        if status != Status::Success {
            self.set_error(status);
        }
        self.status
    }

    ///curve_to
    ///
    ///Adds a cubic Bezier spline to the path from the current point to position (x3, y3) in
    ///user-space coordinates, using (x1, y1) and (x2, y2) as the control points. After this call
    ///the current point will be (x3, y3).
    pub fn curve_to(&mut self, x1: f32, y1: f32,
                    x2: f32, y2: f32,
                    x3: f32, y3: f32)  -> Status{
        //let mut status = Status::Success;
        if self.status != Status::Success {
            return Status::InvalidPathData;
        }

        let status = self.path.curve_to(x1, y1, x2, y2, x3, y3);
        if status != Status::Success {
            self.set_error(status);
        }
        self.status
    }

    pub fn close_path(&mut self) {
        self.path.close();

    }

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn fill(&mut self) {
        let mut filler = Filler::new();
        filler.fill(&self.path, &mut self.target, self.rgba);
    }
}

/// # References
/// [Cairo Operators]: https://www.cairographics.org/operators/

#[cfg(test)]
mod tests{

    use surfaces::ImageSurface;
    use types::Rgba;
    use operators::Operator;
    use super::Context;
    use std::path::Path;

    #[test]
    fn test_get_default_operator(){
        // Setup
        let mut surface = ImageSurface::create(255, 255);
        let context = Context::create( &mut surface );

        // Call
        let op = context.get_operator();

        // Assert
        assert_eq!( &Operator::Over, op );
    }

    #[test]
    fn test_set_operator(){
        // Setup
        let mut surface = ImageSurface::create(255, 255);
        let mut context = Context::create( &mut surface );

        // Call
        context.set_operator(Operator::In);
        let op = context.get_operator();

        // Assert
        assert_eq!( &Operator::In, op );
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

    #[test]
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


    #[test]
    fn demo() {
        let mut surface = ImageSurface::create(400, 400);
        let path = Path::new("Demo.png");
        {
            let mut context = Context::create(&mut surface);
            context.set_source_rgba(0.,1.,0.,1.);
            context.move_to(100., 100.);
            context.line_to(300., 100.);
            context.line_to(300., 300.);
            context.line_to(100., 300.);
 //           context.line_to(100., 100.);
            context.close_path();
            context.fill();
        }
        surface.to_file(path);
    }

    #[test]
    fn demo_two() {
        let mut surface = ImageSurface::create(400, 400);
        let path = Path::new("Demo2.png");
        {
            let mut context = Context::create(&mut surface);
            context.set_source_rgba(0.,1.,0.,1.);
            context.move_to(100., 100.);
            context.line_to(300., 100.);
            context.curve_to(300., 200., 200., 300., 100., 300.);
            context.close_path();
            context.fill();
        }
        surface.to_file(path);
    }

    #[test]
    fn demo_two_a() {
        let mut surface = ImageSurface::create(400, 400);
        let path = Path::new("Demo2a.png");
        {
            let mut context = Context::create(&mut surface);
            context.set_source_rgba(0.,0.,0.,1.);
            context.move_to(100., 200.);
            context.line_to(300., 200.);
            context.curve_to(250., 300., 150., 300., 99., 200.);
            context.close_path();
            context.fill();
            context.clear_path();
            context.move_to(125., 100.);
            context.line_to(175., 100.);
            context.line_to(175., 150.);
            context.line_to(125., 150.);
            context.close_path();
            context.fill();
            context.clear_path();
            context.move_to(225., 100.);
            context.line_to(275., 100.);
            context.line_to(275., 150.);
            context.line_to(225., 150.);
            context.close_path();
            context.fill();
        }
        surface.to_file(path);
    }

    #[test]
    fn demo_two_b() {
        let mut surface = ImageSurface::create(400, 400);
        let path = Path::new("Demo2b.png");
        {
            let mut context = Context::create(&mut surface);
            context.set_source_rgba(0.,0.,0.,1.);
            context.move_to(100., 200.);
            context.line_to(300., 200.);
            context.curve_to(250., 300., 150., 300., 99., 200.);
            context.close_path();
            context.fill();
            context.clear_path();
            context.move_to(125., 100.);
            context.line_to(175., 100.);
            context.line_to(175., 150.);
            context.line_to(125., 150.);
            context.close_path();
            context.fill();
            context.clear_path();
            context.move_to(225., 100.);
            context.line_to(275., 100.);
            context.line_to(275., 150.);
            context.line_to(225., 150.);
            context.close_path();
            context.fill();
            context.clear_path();
            context.set_source_rgba(138.,43.,226.,0.75);
            context.move_to(50., 50.);
            context.line_to(350., 50.);
            context.line_to(350., 350.);
            context.line_to(50., 350.);
            context.close_path();
            context.fill();
        }
        surface.to_file(path);
    }

    #[test]
    fn demo_three() {
        let mut surface = ImageSurface::create(500, 500);
        let path = Path::new("Demo3.png");
        {
            let mut context = Context::create(&mut surface);
            context.set_source_rgba(0.,1.,0.,1.);
            context.move_to(100., 100.);
            context.line_to(300., 100.);
            context.line_to(300., 300.);
            context.line_to(100., 300.);
            context.close_path();
            context.fill();
            context.clear_path();
            context.set_source_rgba(1.,0.,0.,0.75);
            context.move_to(200., 200.);
            context.line_to(400., 200.);
            context.line_to(400., 400.);
            context.line_to(200., 400.);
            context.close_path();
            context.fill();

        }
        surface.to_file(path);
    }
}
