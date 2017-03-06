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
 *  Kyle Kneitinger <kyle@kneit.in>
 *  Evan Smelser <evanjsmelser@gmail.com>
 */

use common_geometry::Point;
use status::Status;
use splines::Spline;
use context::Context;

pub enum Data {
    MoveTo (  Point ),
    LineTo (  Point ),
    CurveTo ( Point, Point, Point),
    ClosePath,
}

pub struct Path {
   status: Status,
   data_vec: Vec<Data>,
   data_num: usize,
}
/*
///In cairo these take in a closure, point, and slope
//Starting to think this implementation won't work because the curve_to should take in three points
//and the close_path shouldn't need any...
pub fn add_point_func(d: &Data) -> fn(&Point, &Slope)  {
    match *d {
        Data::MoveTo    => move_to,
        Data::LineTo    => line_to,
        Data::CurveTo   => curve_to,
        Data::ClosePath => close_path,
    }
    unimplemented!();
}

fn move_to(p: &Point, s: &Slope){
    unimplemented!();
}
fn line_to(p: &Point, s: &Slope){
    unimplemented!();
}
fn curve_to(p: &Point, s: &Slope){
    unimplemented!();
}
fn close_path(p: &Point, s: &Slope){
    unimplemented!();
}
*/
/// Path Related Operations
/// This section will define the implementation of all Path related functionality.
impl Path {
    ///new_path
    ///
    ///Clears the current path. 
    ///After this call there will be no path and no current point.
    pub fn new_path(context: & mut Context) {
        let mut status = Status::Success;

        if context.status != Status::Success {
            return;
        }
        
        //TODO: Implement context.new_path()
        //status = context.new_path();
        if status != Status::Success {
            context.set_error(status);
        }
        unimplemented!();
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
    pub fn new_sub_path(context: & mut Context) {
        let mut status = Status::Success;

        if context.status != Status::Success {
            return;
        }
        
        //TODO: Implement context.new_sub_path()
        //status = context.new_sub_path();
        if status != Status::Success {
            context.set_error(status);
        }
        unimplemented!();
    }

    ///move_to
    ///
    ///Begin a new sub-path. After this call the current point will be (x, y).
    pub fn move_to(context: & mut Context, x: f32, y: f32){
        let mut status = Status::Success;

        if context.status != Status::Success {
            return;
        }
        
        //TODO: Implement context.move_to(x, y)
        //status = context.move_to(x, y);
        if status != Status::Success {
            context.set_error(status);
        }
        unimplemented!();

        unimplemented!();
    }

    ///line_to
    ///
    ///Adds a line to the path from the current point to position (x, y) in user-space coordinates.
    ///After this call the current point will be (x, y)
    pub fn line_to(context: & mut Context, x: f32, y: f32){
        let mut status = Status::Success;

        if context.status != Status::Success {
            return;
        }
        
        //TODO: Implement context.line_to(x, y)
        //status = context.line_to(x, y);
        if status != Status::Success {
            context.set_error(status);
        }
        unimplemented!();

    }

    ///curve_to
    ///
    ///Adds a cubic Bezier spline to the path from the current point to position (x3, y3) in
    ///user-space coordinates, using (x1, y1) and (x2, y2) as the control points. After this call
    ///the current point will be (x3, y3).
    pub fn curve_to(context: & mut Context, x1: f32, y1: f32,
                    x2: f32, y2: f32,
                    x3: f32, y3: f32){
        let mut status = Status::Success;

        if context.status != Status::Success {
            return;
        }
        
        //TODO: Implement context.curve_to(x1, y1, x2, y2, x3, y3)
        //status = context.curve_to(x1, y1, x2, y2, x3, y3);
        if status != Status::Success {
            context.set_error(status);
        }
        unimplemented!();
        unimplemented!();
    }
}

#[cfg(test)]
mod tests{


    #[test]
    fn test_something(){
        assert_eq!(1,1);
    }
}



