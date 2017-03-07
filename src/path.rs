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
use std::f32;

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
   current_point: Point,
}
/*
///In cairo these take in a closure, point, and slope
//Starting to think this implementation won't work because the curve_to should take in three points
//and the close_path shouldn't need any...
pub fn add_point_func(d: &Data) -> fn(&Point, &Slope)  {
    match *d {
        Data::MoveTo    => move_to,
        Data::LineTo    => line_to,
        _               => panic!("add_point_func Data ERROR");
        //Data::CurveTo   => curve_to,
        //Data::ClosePath => close_path,
    }
}
pub fn spline_add_point_func(d: &Data) -> fn(&Point, &Point, &Point, &Slope) {
    match *d {
        Data::CurveTo   => curve_to,
        _               => panic!("spline_add_point_func Data ERROR");
    }
}
fn move_to(p: &Point, s: &Slope){
    unimplemented!();
}
fn line_to(p: &Point, s: &Slope){
    unimplemented!();
}
fn curve_to(p1: &Point, p2: &Point, p3: &Point, s: &Slope){
    unimplemented!();
}
fn close_path(p: &Point, s: &Slope){
    unimplemented!();
}
*/
/// Path Related Operations
/// This section will define the implementation of all Path related functionality.
impl Path {
    ///Path::create()
    ///
    ///This will create a new (empty) path with no current point signified by the point having NAN
    ///as both it's x and y value.
    pub fn create() -> Path {
        Path{
            status: Status::Success,
            //may want to use Vec::with_capacity here, not sure of syntax restrictions
            data_vec: Vec::new(),
            data_num: 0,
            current_point: Point::create(f32::NAN, f32::NAN),
        }
    }
    ///new_path
    ///
    ///Clears the current path. 
    ///After this call there will be no path and no current point.
    ///The current point will be signified empty by it's x and y coordinate value both being
    ///f32::NAN values.
    pub fn new_path(&mut self) -> Status {
        self.status = Status::Success;
        //may want to use Vec::with_capacity here, not sure of syntax restrictions
        self.data_vec = Vec::new();
        self.data_num = 0;
        self.current_point = Point::create(f32::NAN, f32::NAN);
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
    pub fn new_sub_path(&mut self) -> Status {
        //This will not be a part of our MVP, so has yet to be implemented as it relates more to
        //Arc implementation then our general line_to and curve_to
        unimplemented!();
    }

    ///move_to
    ///
    ///Begin a new sub-path. After this call the current point will be (x, y).
    pub fn move_to(&mut self, x: f32, y: f32) -> Status {
        let point = Point::create(x, y);
        self.data_vec.push(Data::MoveTo(point));
        self.data_num += 1;
        self.current_point = point;
        self.status
    }

    ///line_to
    ///
    ///Adds a line to the path from the current point to position (x, y) in user-space coordinates.
    ///After this call the current point will be (x, y)
    pub fn line_to(&mut self, x: f32, y: f32) -> Status {
        let point = Point::create(x, y);
        self.data_vec.push(Data::LineTo(point));
        self.data_num += 1;
        self.current_point = point;
        self.status
    }

    ///curve_to
    ///
    ///Adds a cubic Bezier spline to the path from the current point to position (x3, y3) in
    ///user-space coordinates, using (x1, y1) and (x2, y2) as the control points. After this call
    ///the current point will be (x3, y3).
    pub fn curve_to(&mut self, x1: f32, y1: f32,
                    x2: f32, y2: f32,
                    x3: f32, y3: f32) -> Status{
        let b = Point::create(x1, y1);
        let c = Point::create(x2, y2);
        let d = Point::create(x3, y3);
        //call path related functions here?? Not sure how this all works with Splines etc...
        self.data_vec.push(Data::CurveTo(b, c, d));
        self.data_num += 1;
        self.current_point = d;
        self.status
    }
}

#[cfg(test)]
mod tests{

    use path::Path;
    use context::Context;


    #[test]
    fn test_create_new_path(){
        assert_eq!(1,1);
    }
    
    #[test]
    fn test_move_to_different_location(){
        assert_eq!(1,1);
    }

    #[test]
    fn test_move_to_same_location(){
        //should fail
        assert_eq!(1,1);
    }
}



