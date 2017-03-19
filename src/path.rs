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

//extern crate cairus;

use common_geometry::Point;
use status::Status;
use splines::Spline;
use context::Context;
use std::f32;
use std::vec::Vec;

/// A data structure for holding a path.
///
/// The `data_num` member gives the number of elements in the `data_vec` vector. This number is larger than
/// the number of independent path portions (defined in path::Data), since the data includes both
/// headers and coordinates for each portion. The `current_point` is the "endpoint" of the path. The
/// `status` maintains the truthiness of the path.
//#[derive(Debug, Copy, Clone, PartialEq)]
//Can't implement Copy or Clone for Path because of the Vec<Data>
//TODO: More research on implementing copy and clone for better testing.
pub struct Path {
   status: Status,
   pub data_vec: Vec<Data>,
   data_num: usize,
   current_point: Point,
}

///An enumeration of the possible Data elements which describe a `Path`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Data {
    MoveTo (  Point ),
    LineTo (  Point ),
    CurveTo ( Point, Point, Point),
    ClosePath,
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

/// Implementation of `Path` related operations.
impl Path {

    /// Constructs a new `Path`.
    ///
    /// This will create a new (empty) path with no current point signified by the point having NAN
    /// as both it's x and y value.
    ///
    /// # Return
    /// * `Path` - A new empty path object
    ///
    /// # Example
    /// ```
    /// use cairus::path::Path;
    /// use cairus::status::Status;
    ///
    /// let mut path = Path::create();
    /// ```
    pub fn create() -> Path {
        Path{
            status: Status::Success,
            //may want to use Vec::with_capacity here, not sure of syntax restrictions
            data_vec: Vec::new(),
            data_num: 0,
            current_point: Point::new(f32::NAN, f32::NAN),
        }
    }

    /// Clears the current path.
    ///
    /// After this call there will be no path and no current point.
    /// The current point will be signified empty by it's x and y coordinate value both being
    /// f32::NAN values.
    ///
    /// # Return
    /// * `Status` - A status which is indicative of the current truthiness of the Path related
    /// operations.
    ///
    /// # Examples
    /// ```
    /// use cairus::path::Path;
    /// use cairus::status::Status;
    ///
    /// let mut path = Path::create();
    /// let mut status = path.move_to(1., 1.5);
    /// ```
    pub fn new_path(&mut self) -> Status {
        self.status = Status::Success;
        //may want to use Vec::with_capacity here, not sure of syntax restrictions
        self.data_vec = Vec::new();
        self.data_num = 0;
        self.current_point = Point::new(f32::NAN, f32::NAN);
        self.status
    }

    /// Returns the current point of the path.
    ///
    /// # Return
    /// * `Point` - The current _last_ point in the path.
    ///
    /// # Examples
    /// ```
    /// use cairus::path::Path;
    /// use cairus::status::Status;
    /// use cairus::common_geometry::Point;
    ///
    /// let mut path = Path::create();
    /// let status = path.move_to(0., 0.);
    /// let point = path.get_current_point();
    /// ```
    pub fn get_current_point(&mut self) -> Point {
        self.current_point
    }

    /// Begin a new sub-path. Note that the existing path is not
    /// affected. After this call there will be no current point.
    ///
    /// In many cases, this call is not needed since new sub-paths are
    /// frequently started with cairo_move_to().
    ///
    /// A call to cairo_new_sub_path() is particularly useful when
    /// beginning a new sub-path with one of the cairo_arc() calls. This
    /// makes things easier as it is no longer necessary to manually
    /// compute the arc's initial coordinates for a call to
    /// cairo_move_to().
    ///
    /// # Return
    /// * `Status` - A status which is indicative of the current truthiness of the Path related
    /// operations.
    ///
    /// # Examples
    /// ```
    /// use cairus::path::Path;
    /// use cairus::status::Status;
    ///
    /// let mut path = Path::create();
    /// //let status: Status = path.new_sub_path();
    /// ```
    pub fn new_sub_path(&mut self) -> Status {
        //This will not be a part of our MVP, so has yet to be implemented as it relates more to
        //Arc implementation then our general line_to and curve_to
        unimplemented!();
    }

    /// Begin a new sub-path. After this call the current point will be (x, y).
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the point to move the current point to.
    /// * `y` - The y coordinate of the point to move the current point to.
    ///
    /// # Return
    /// * `Status` - A status which is indicative of the current truthiness of the Path related
    /// operations.
    ///
    /// # Examples
    /// ```
    /// use cairus::path::Path;
    /// use cairus::status::Status;
    ///
    /// let mut path = Path::create();
    /// let status = path.move_to(1., 1.5);
    /// ```
    pub fn move_to(&mut self, x: f32, y: f32) -> Status {
        let point = Point::new(x, y);
        if x < 0. || y < 0. {
            return Status::InvalidPathData;
        }
        if self.current_point == point {
            return Status::InvalidPathData;
        }

        self.data_vec.push(Data::MoveTo(point));
        self.data_num += 1;
        self.current_point = point;
        self.status
    }

    pub fn close(&mut self) -> Status {
        self.data_vec.push(Data::ClosePath);
//        self.current_point = match data_vec.get(0) {
//            Data::MoveTo    => a,
//            _               => Point::origin(),
//        };
        self.status
    }

    /// Adds a line to the path from the current point to position (x, y) in user-space coordinates.
    /// After this call the current point will be (x, y)
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the point to join the path with.
    /// * `y` - The y coordinate of the point to join the path with.
    ///
    /// # Return
    /// * `Status` - A status which is indicative of the current truthiness of the Path related
    /// operations.
    ///
    /// # Examples
    /// ```
    /// use cairus::path::Path;
    /// use cairus::status::Status;
    ///
    /// let mut path = Path::create();
    /// let status = path.line_to(2.5, 3.);
    /// ```
    pub fn line_to(&mut self, x: f32, y: f32) -> Status {
        //Disallow line_to() if no current_point
        if self.current_point.x.is_nan() || self.current_point.y.is_nan(){
            return Status::InvalidPathData;
        }
        let point = Point::new(x, y);
        if  x < 0. || y < 0. {
            return Status::InvalidPathData;
        }
        if self.current_point == point {
            return Status::InvalidPathData;
        }
        self.data_vec.push(Data::LineTo(point));
        self.data_num += 1;
        self.current_point = point;
        self.status
    }

    /// Adds a cubic Bezier spline to the path from the current point to position (x3, y3) in
    /// user-space coordinates, using (x1, y1) and (x2, y2) as the control points. After this call
    /// the current point will be (x3, y3).
    ///
    /// # Arguments
    /// * `x1` - The x coordinate of the first control point of the Bezier.
    /// * `y1` - The y coordinate of the first control point of the Bezier.
    /// * `x2` - The x coordinate of the second control point of the Bezier.
    /// * `y2` - The y coordinate of the second control point of the Bezier.
    /// * `x3` - The x coordinate of the end point of the Bezier.
    /// * `y3` - The y coordinate of the end point of the Bezier.
    ///
    /// # Return
    /// * `Status` - A status which is indicative of the current truthiness of the Path related
    /// operations.
    ///
    /// # Examples
    /// ```
    /// use cairus::path::Path;
    /// use cairus::status::Status;
    ///
    /// let mut path = Path::create();
    /// let status = path.curve_to(1., 2., 3., 4., 5., 6.);
    /// ```
    pub fn curve_to(&mut self, x1: f32, y1: f32,
                    x2: f32, y2: f32,
                    x3: f32, y3: f32) -> Status{
        //Disallow curve_to() from empty current_point
        if self.current_point.x.is_nan() || self.current_point.y.is_nan(){
            return Status::InvalidPathData;
        }
        if x1<0. || y1<0. || x2<0. || y2<0. || x3<0. || y3<0. {
            return Status::InvalidPathData;
        }

        let b = Point::new(x1, y1);
        let c = Point::new(x2, y2);
        let d = Point::new(x3, y3);
        if self.current_point == d {
            return Status::InvalidPathData;
        }
        //call path related functions here?? Not sure how this all works with Splines etc...
        self.data_vec.push(Data::CurveTo(b, c, d));
        self.data_num += 1;
        self.current_point = d;
        self.status
    }
}

#[cfg(test)]
mod tests{

    use common_geometry::Point;
    use status::Status;
    use splines::Spline;
    use context::Context;
    use std::f32;
    use path::Path;
    use path::Data;

    //test Path::create()
    #[test]
    fn test_create_path(){
        let path = Path::create();

        assert_eq!(path.status, Status::Success);
        assert_eq!(path.data_vec.len(), 0);
        assert_eq!(path.data_num, 0);
        assert!(path.current_point.x.is_nan());
        assert!(path.current_point.y.is_nan());
    }

    //test Path::new_path()
    #[test]
    fn test_new_path(){
        let mut path = Path::create();
        let p1 = Point::new(1., 1.5);
        let mut status = path.move_to(p1.x, p1.y);
        status = path.new_path();

        assert_eq!(path.status, Status::Success);
        assert_eq!(path.data_vec.len(), 0);
        assert_eq!(path.data_num, 0);
        assert!(path.current_point.x.is_nan());
        assert!(path.current_point.y.is_nan());
    }

    //test Path::move_to()
    #[test]
    fn test_move_to_different_location(){
        let mut path = Path::create();
        let p1 = Point::new(1., 1.5);
        let p2 = Point::new(2., 2.5);
        let mut status = path.move_to(p1.x, p1.y);
        status = path.move_to(p2.x, p2.y);

        assert_eq!(status, Status::Success);
        assert_eq!(path.current_point, p2);
        assert_eq!(path.data_vec.len(), 2);
        assert_eq!(path.data_vec[0], Data::MoveTo(p1));
        assert_eq!(path.data_vec[1], Data::MoveTo(p2));
        assert_eq!(path.data_num, 2);
    }

    #[test]
    fn test_move_to_same_location(){
        let mut path = Path::create();
        let p1 = Point::new(1., 1.5);
        let p2 = Point::new(1., 1.5);
        let mut status = path.move_to(p1.x, p1.y);
        status = path.move_to(p2.x, p2.y);

        assert_eq!(status, Status::InvalidPathData);
    }

    #[test]
    fn test_move_to_negative_location(){
        let mut path = Path::create();
        let p1 = Point::new(-1., 1.5);
        let mut status = path.move_to(p1.x, p1.y);

        assert_eq!(status, Status::InvalidPathData);
    }

    //test Path::line_to()
    #[test]
    fn test_line_to_different_location(){
        let mut path = Path::create();
        let p1 = Point::new(1., 1.5);
        let p2 = Point::new(2., 2.5);
        let origin = Point::origin();

        let mut status = path.move_to(origin.x, origin.y);
        status = path.line_to(p1.x, p1.y);
        status = path.line_to(p2.x, p2.y);

        assert_eq!(status, Status::Success);
        assert_eq!(path.current_point, p2);
        assert_eq!(path.data_vec.len(), 3);
        assert_eq!(path.data_vec[1], Data::LineTo(p1));
        assert_eq!(path.data_vec[2], Data::LineTo(p2));
        assert_eq!(path.data_num, 3);
    }

    #[test]
    fn test_line_to_from_empty_default_point(){
        let mut path = Path::create();
        let p1 = Point::new(1., 1.5);

        let mut status = path.line_to(p1.x, p1.y);

        assert_eq!(status, Status::InvalidPathData);
    }

    #[test]
    fn test_line_to_same_location(){
        let mut path = Path::create();
        let p1 = Point::new(1., 1.5);
        let p2 = Point::new(1., 1.5);
        let mut status = path.line_to(p1.x, p1.y);
        status = path.line_to(p2.x, p2.y);

        assert_eq!(status, Status::InvalidPathData);
    }

    #[test]
    fn test_line_to_negative_location(){
        let mut path = Path::create();
        let p1 = Point::new(-1., 1.5);
        let mut status = path.line_to(p1.x, p1.y);

        assert_eq!(status, Status::InvalidPathData);
    }

   //test Path::curve_to()
    #[test]
    fn test_curve_to_different_location(){
        let mut path = Path::create();
        let p1 = Point::new(1., 1.5);
        let p2 = Point::new(2., 2.5);
        let p3 = Point::new(3., 3.5);
        let p4 = Point::new(4., 4.5);
        let p5 = Point::new(5., 5.5);
        let p6 = Point::new(6., 6.5);
        let origin = Point::origin();

        let mut status = path.move_to(origin.x, origin.y);
        status = path.curve_to(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
        let p7 = path.current_point;
        status = path.curve_to(p4.x, p4.y, p5.x, p5.y, p6.x, p6.y);

        assert_eq!(status, Status::Success);
        assert_eq!(path.current_point, p6);
        assert_eq!(p3, p7);
        assert_eq!(path.data_vec.len(), 3);
        assert_eq!(path.data_vec[1], Data::CurveTo(p1, p2, p3));
        assert_eq!(path.data_vec[2], Data::CurveTo(p4, p5, p6));
        assert_eq!(path.data_num, 3);
    }

    #[test]
    fn test_curve_to_from_default_empty_point(){
        let mut path = Path::create();
        let p1 = Point::new(1., 1.5);
        let p2 = Point::new(2., 2.5);
        let p3 = Point::new(3., 3.5);
        let p4 = Point::new(4., 4.5);
        let p5 = Point::new(5., 5.5);
        let p6 = Point::new(6., 6.5);
        let mut status = path.curve_to(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
        let p7 = path.current_point;
        status = path.curve_to(p4.x, p4.y, p5.x, p5.y, p6.x, p6.y);

        assert_eq!(status, Status::InvalidPathData);
    }

    #[test]
    fn test_curve_to_same_location(){
        let mut path = Path::create();
        let p1 = Point::new(1., 1.5);
        let p2 = Point::new(2., 2.5);
        let p3 = Point::new(3., 3.5);
        let p4 = Point::new(1., 1.5);
        let p5 = Point::new(2., 2.5);
        let p6 = Point::new(3., 3.5);
        let mut status = path.curve_to(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
        status = path.curve_to(p4.x, p4.y, p5.x, p5.y, p6.x, p6.y);

        assert_eq!(status, Status::InvalidPathData);
    }

    #[test]
    fn test_curve_to_negative_location(){
        let mut path = Path::create();
        let p1 = Point::new(-1., 1.5);
        let p2 = Point::new(1., 1.5);
        let p3 = Point::new(2., 1.5);
        let mut status = path.curve_to(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);

        assert_eq!(status, Status::InvalidPathData);
    }

}



