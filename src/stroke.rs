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
 *	Evan Smelser <evanjsmelser@gmail.com>
 *
 */

//! This module defines integration functions related to the stroke pipeline in Cairus.

//use decasteljau::DeCasteljauPoints;
use common_geometry::Point;
use common_geometry::SplineKnots;
use common_geometry::LineSegment;
use std::ops::{Add, Sub};
use std::usize;

/// ## Bezier 
///
/// Defines a Bezier Curve by an array of Points which, when connected, define the Bezier.
 #[derive(Debug, Copy, Clone)]
struct Bezier<`a>{
    tolerance: usize,
    spline_knots: &`a SplineKnots,
    points_vec: &`a Vec<Point>,
}

//Implemented the points of the Bezier as a vector so that they can have variable size based on the
//tolerance indicated in the tolerance value of the Bezier. The tolerance lets us know how many
//points the Bezier is comprised of and also lets us know how far down each of the three lines the
//curve is composed of we need to go between each point while calculating the actual Bezier Curve.  
impl Bezier{

    //creates a new Bezier blueprint with and assigns it's tolerance
    pub fn new(t: usize, )-> Bezier{
        Bezier{
            tolerance: t,
            points_vec: vec![],
        }
    }
    
    //Actually creates the Bezier curve. Fills the points_vec with points created using the
    //DeCasteljau algorithm for finding a point on the curve. 
    //e1 and e2 are the end points, c1 and c2 are the control points of the curve.
    //This algorithm will walk down each of the three lines (between the four points) and create a
    //vector of
    fn create_bezier(&mut self, e1: &Point, c1: &Point, c2: &Point, e2: &Point)-> Bezier{
        let knots = SplineKnots::create(&e1, &c1, &c2, &e2);
        let ab = LineSegment::from_points(e1, c1);
        let bc = LineSegment::from_points(c1, c2);
        let cd = LineSegment::from_points(c2, e2);
    
    
    }

}





