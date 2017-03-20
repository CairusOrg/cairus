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
 */

//use std::f32;
use common_geometry::{Point, Slope, Edge};
use status::Status;

///SplineKnots for bezier curves
#[derive(Clone)]
pub struct SplineKnots{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

///Implements SplineKnots methods
impl SplineKnots{
    ///Creates a new SplineKnots with user defined points
    fn create_from_points(a: &Point, b: &Point, c: &Point, d: &Point)->SplineKnots{
        SplineKnots{
            a:Point::new(a.x, a.y),
            b:Point::new(b.x, b.y),
            c:Point::new(c.x, c.y),
            d:Point::new(d.x, d.y),
        }
    }

    fn new() -> SplineKnots {
        SplineKnots{
            a:Point::new(0.0,0.0),
            b:Point::new(0.0,0.0),
            c:Point::new(0.0,0.0),
            d:Point::new(0.0,0.0),
        }
    }
}

///Spline
pub struct Spline{
    //not sure which if any of these we will need.
    //add_point_func
    //closure

    //working on figuring out mutability of these attributes...
    //Also wondering if I will need to use lifetime specifics here for ownership.
    pub knots: SplineKnots,
    pub initial_slope: Slope,
    pub final_slope: Slope,
    pub has_point: bool,
    pub last_point: Point,
}

///Implements Spline methods
impl Spline{
    ///Creates a new Spline with user defined values
    fn spline_init(spline: & mut Spline,
                   a: Point, b: Point,
                   c: Point, d: Point) -> bool {
        // If both tangents are zero, this is just a straight line
        if a.x == b.x && a.y == b.y && c.x == d.x && c.y == d.y {
            return false;
        }

        //The cairo code calls the add point function here and assigns the closure here, I'm not
        //sure if we need these concepts or not.
        //spline.add_point_func
        //spline.closure

        spline.knots = SplineKnots::create_from_points(&a, &b, &c, &d);

        if a.x != b.x || a.y != b.y {
            spline.initial_slope = Slope::slope_init(spline.knots.a, spline.knots.b);
        }
        else if a.x != c.x || a.y != c.y {
            spline.initial_slope = Slope::slope_init(spline.knots.a, spline.knots.c);
        }
        else if a.x != d.x || a.y != d.y {
            spline.initial_slope = Slope::slope_init(spline.knots.a, spline.knots.d);
        }
        else{
            //This is just a straight line
            return false;
        }


        if c.x != d.x || c.y != d.y {
            spline.final_slope = Slope::slope_init(spline.knots.c, spline.knots.d);
        }
        else if b.x != d.x || b.y != d.y {
            spline.final_slope = Slope::slope_init(spline.knots.b, spline.knots.d);
        }
        else{
            //This is just a straight line
            return false;
        }
        true
    }
}
///This function takes two end points which are interpolated providing the intermediate point
fn lerp_half(a: &Point, b: &Point)->Point{
    Point{
        x: a.x + (b.x - a.x)/2.,
        y: a.y + (b.y - a.y)/2.,
    }
}

///Initial four points of the Bezier curve
struct DeCasteljauPoints{
    ab: Point,
    bc: Point,
    cd: Point,
    abbc: Point,
    bccd: Point,
    fin: Point,
}

// This will be refactored in a later commit
///Implemetation of Decasteljau methods
impl DeCasteljauPoints {
    ///Sets all the Points of the bezier curve to 0.0 using origin method of Point
    fn create()-> DeCasteljauPoints{
        DeCasteljauPoints{
            ab: Point::origin(),
            bc: Point::origin(),
            cd: Point::origin(),
            abbc: Point::origin(),
            bccd: Point::origin(),
            fin: Point::origin(),
        }
    }

    ///Implementation of the bezier curve
    fn create_spline(& mut self, s1: & mut SplineKnots, s2: & mut SplineKnots){
        self.ab = lerp_half(&s1.a, &s1.b);
        self.bc = lerp_half(&s1.b, &s1.c);
        self.cd = lerp_half(&s1.c, &s1.d);
        self.abbc = lerp_half(&self.ab, &self.bc);
        self.bccd = lerp_half(&self.bc, &self.cd);
        self.fin = lerp_half(&self.abbc, &self.bccd);
        s2.a = Point::new(self.fin.x, self.fin.y);
        s2.b = Point::new(self.bccd.x, self.bccd.y);
        s2.c = Point::new(self.cd.x, self.cd.y);
        s2.d = Point::new(s1.d.x, s1.d.y);
        s1.b = Point::new(self.ab.x, self.ab.y);
        s1.c = Point::new(self.abbc.x, self.abbc.y);
        s1.d = Point::new(self.fin.x, self.fin.y);
    }

}

fn de_casteljau(s1: & mut SplineKnots, s2: & mut SplineKnots){
    let ab = lerp_half(&s1.a, &s1.b);
    let bc = lerp_half(&s1.b, &s1.c);
    let cd = lerp_half(&s1.c, &s1.d);
    let abbc = lerp_half(&ab, &bc);
    let bccd = lerp_half(&bc, &cd);
    let fin = lerp_half(&abbc, &bccd);
    s2.a = fin;
    s2.b = bccd;
    s2.c = cd;
    s2.d = s1.d;
    s1.b = ab;
    s1.c = abbc;
    s1.d = fin;
}

fn lerp(a: &Point, b: &Point, t: f32 ) -> Point
{
    Point::new(a.x + (b.x-a.x)*t, a.y + (b.y-a.y)*t)
}

// evaluate a point on a bezier-curve. t goes from 0 to 1.0
fn bezier(a: &Point, b: &Point, c: &Point, d: &Point, t: f32 ) -> Point
{
    //let ab,bc,cd,abbc,bccd;
    let ab =lerp(&a,&b,t);           // point between a and b (green)
    let bc = lerp(&b,&c,t);           // point between b and c (green)
    let cd = lerp(&c,&d,t);           // point between c and d (green)
    let abbc = lerp(&ab,&bc,t);       // point between ab and bc (blue)
    let bccd = lerp(&bc,&cd,t);       // point between bc and cd (blue)
    lerp(&abbc,&bccd,t)   // point on the bezier-curve (black)
}

pub fn decasteljau (a: &Point, b: &Point, c: &Point, d: &Point) -> Vec<Point> {
    let mut points = Vec::new();
    for i in 1..900 {
        let p = bezier(a,b,c,d,i as f32/1000.0);
        points.push(p);
    }
    points
}

///Calculates the upper bound on the error (squared) that could result from approximating a
///spline as a line segment connecting the two endpoints.
fn error_squared(knots: & SplineKnots) -> f64{

    //We are going to compute the distance (squared) between each of the b and c control points and
    //the segment a-b. The maximum of these two distances will be our approximation error.

    //we will use these values to determine the difference in slope between the bezier control
    //points and point a for comparison with the slope of point d below to see how close we are to
    //a straight line
    let mut bdx = (knots.b.x - knots.a.x) as f64;
    let mut bdy = (knots.b.y - knots.a.y) as f64;

    let mut cdx = (knots.c.x - knots.a.x) as f64;
    let mut cdy = (knots.c.y - knots.a.y) as f64;

    if knots.a.x != knots.d.x || knots.a.y != knots.d.y {

        let dx: f64 = (knots.d.x - knots.a.x) as f64;
        let dy: f64 = (knots.d.y - knots.a.y) as f64;
        //we will compare v and u to see how close our Bezier is to a straight line from a to d.
        let v = dx*dx + dy*dy;

        //how close is the slope of a-b to a-d
        let u = bdx*dx + bdy*dy;
        if u <= 0. {}
        else if u >= v {
            bdx -= dx;
            bdy -= dy;
        }
        else {
            bdx -= u/v * dx;
            bdy -= u/v * dy;
        }

        //how close is the slope of a-c to a-d
        let z = cdx*dx + cdy*dy;
        if z <= 0. {}
        else if z >= v {
            cdx -= dx;
            cdy -= dy;
        }
        else {
            cdx -= z/v * dx;
            cdy -= z/v * dy;
        }
    }
    //calculate and return the upper bound of the error from approximating a spline as a line
    //segment connecting the two endpoints.
    let berr = bdx*bdx + bdy*bdy;
    let cerr = cdx*cdx + cdy*cdy;
    if berr > cerr {
        berr
    }
    else {
        cerr
    }
}

//will return false to let us know if it failed rather than a status like in Cairo.
fn spline_decompose_into (mut s1: &mut SplineKnots,
                          tolerance_squared: f64,
                          result: &mut Spline) -> Status {
    let mut s2 = SplineKnots::new();;

    if error_squared(s1) < tolerance_squared {
        // This actually needs to be a call to add point
        return Status::Success;
    }


    de_casteljau(&mut s1,&mut s2);

    let status = spline_decompose_into(&mut s1, tolerance_squared, result);

    if status != Status::Success {
        return status;
    }

    spline_decompose_into(&mut s2, tolerance_squared, result)
}

fn spline_decompose (mut spline: &mut Spline, tolerance: f64) -> Status {
    // TODO: VERIFY THAT CLONING IS ACCEPTABLE HERE!!!!!!
    let mut s1 = spline.knots.clone();

    spline.last_point = s1.a;

    let status = spline_decompose_into(&mut s1, tolerance*tolerance, &mut spline);

    if status != Status::Success {
        return status;
    }

    // This actually needs to be a call to add point func
    status

}

#[cfg(test)]
mod tests{
    use::common_geometry::Point;
    use::splines::SplineKnots;
    use::splines::DeCasteljauPoints;
    use::splines::lerp_half;


    #[test]
    fn test_splineknots(){
        //Functional test for the creation of Splineknots using provided points

        //Setup
        let p1 = Point::new(1., 1.);
        let p2 = Point::new(-1., 2.);
        let p3 = Point::new(-1.5, -2.4);
        let p4 = Point::new(2.6, -3.3);

        //Call
        let s1 = SplineKnots::create_from_points(&p1, &p2, &p3, &p4);
        //Test
        assert_eq!(s1.a.x, 1.);
        assert_eq!(s1.a.y, 1.);
        assert_eq!(s1.b.x, -1.);
        assert_eq!(s1.b.y, 2.);
        assert_eq!(s1.c.x, -1.5);
        assert_eq!(s1.c.y, -2.4);
        assert_eq!(s1.d.x, 2.6);
        assert_eq!(s1.d.y, -3.3);
    }

    #[test]
    fn test_lerp_half_quadrant1(){
        //Functional test to ensure the creation of the calculated intermediate point with two
        //endpoints located in Q1

        //Setup
        let p1 = Point::new(1.9, 2.4);
        let p2 = Point::new(2.7, 3.3);
        //Call
        let l1 = lerp_half(&p1, &p2);
        //Test
        assert_eq!(l1.x, 2.3);
        assert_eq!(l1.y, 2.85);
    }

    #[test]
    fn test_lerp_half_quadrant2(){
        //Functional test to ensure the creation of the calculated intermediate point with two
        //endpoints located in Q2

        //Setup
        let p1 = Point::new(-1.9, 2.4);
        let p2 = Point::new(-2.7, 3.3);
        //Call
        let l1 = lerp_half(&p1, &p2);
        //Test
        assert_eq!(l1.x, -2.3);
        assert_eq!(l1.y, 2.85);
    }

    #[test]
    fn test_lerp_half_quadrant3(){
        //Functional test to ensure the creation of the calculated intermediate point with two
        //endpoints located in Q3

        //Setup
        let p1 = Point::new(-1.9, -2.4);
        let p2 = Point::new(-2.7, -3.3);
        //Call
        let l1 = lerp_half(&p1, &p2);
        //Test
        assert_eq!(l1.x, -2.3);
        assert_eq!(l1.y, -2.85);
    }

    #[test]
    fn test_lerp_half_quadrant4(){
        //Functional test to ensure the creation of the calculated intermediate point with two
        //endpoints located in Q4

        //Setup
        let p1 = Point::new(-1.9, -2.4);
        let p2 = Point::new(-2.7, -3.3);
        //Call
        let l1 = lerp_half(&p1, &p2);
        //Test
        assert_eq!(l1.x, -2.3);
        assert_eq!(l1.y, -2.85);
    }

    #[test]
    fn test_lerp_half_quad1_quad2(){
        //Functional test to ensure the creation of the calculated intermediate point with two
        //endpoints located in Q1 & Q2

        //Setup
        let q1 = Point::new(1.9, 2.4);
        let q2 = Point::new(-2.7, 3.3);
        //Call
        let l1 = lerp_half(&q1, &q2);
        //Test
        assert_eq!(l1.x, -0.39999998);
        assert_eq!(l1.y, 2.85);
    }

    #[test]
    fn test_lerp_half_quad3_quad4(){
        //Functional test to ensure the creation of the calculated intermediate point with two
        //endpoints located in Q3 & Q4

        //Setup
        let p1 = Point::new(-1.9, -2.4);
        let p2 = Point::new(2.7, -3.3);
        //Call
        let l1 = lerp_half(&p1, &p2);
        //Test
        assert_eq!(l1.x, 0.39999998);
        assert_eq!(l1.y, -2.85);
    }

    #[test]
    fn test_lerp_half_quad1_quad3(){
        //Functional test to ensure the creation of the calculated intermediate point with two
        //endpoints located in Q1 & Q3

        //Setup
        let p1 = Point::new(1.9, 2.4);
        let p2 = Point::new(-2.7, -3.3);
        //Call
        let l1 = lerp_half(&p1, &p2);
        //Test
        assert_eq!(l1.x, -0.39999998);
        assert_eq!(l1.y, -0.4499998);
    }

    #[test]
    fn test_lerp_half_quad2_quad4(){
        //Functional test to ensure the creation of the calculated intermediate point with two
        //endpoints located in Q2 & Q4

        //Setup
        let p1 = Point::new(-1.9, 2.4);
        let p2 = Point::new(2.7, -3.3);
        //Call
        let l1 = lerp_half(&p1, &p2);
        //Test
        assert_eq!(l1.x, 0.39999998);
        assert_eq!(l1.y, -0.4499998);
    }

    #[test]
    fn test_lerp_half_quad1_quad4(){
        //Functional test to ensure the creation of the calculated intermediate point with two
        //endpoints located in Q1 & Q4

        //Setup
        let p1 = Point::new(1.9, 2.4);
        let p2 = Point::new(2.7, -3.3);
        //Call
        let l1 = lerp_half(&p1, &p2);
        //Test
        assert_eq!(l1.x, 2.3);
        assert_eq!(l1.y, -0.4499998);
    }

    #[test]
    fn test_lerp_half_quad2_quad3(){
        //Functional test to ensure the creation of the calculated intermediate point with two
        //endpoints located in Q2 & Q3

        //Setup
        let p1 = Point::new(-1.9, 2.4);
        let p2 = Point::new(-2.7, -3.3);
        //Call
        let l1 = lerp_half(&p1, &p2);
        //Test
        assert_eq!(l1.x, -2.3);
        assert_eq!(l1.y, -0.4499998);
    }

    #[test]
    fn test_initial_spline_points(){
        //Tests the constructor for deCasteljau - tests ensures origin remains valid
        //Setup

        //Call
        let d1 = DeCasteljauPoints::create();
        //Test
        assert_eq!(d1.ab.x, 0.0);
        assert_eq!(d1.ab.y, 0.0);
        assert_eq!(d1.bc.x, 0.0);
        assert_eq!(d1.bc.y, 0.0);
        assert_eq!(d1.cd.x, 0.0);
        assert_eq!(d1.cd.y, 0.0);
        assert_eq!(d1.abbc.x, 0.0);
        assert_eq!(d1.abbc.y, 0.0);
    }

    #[test]
    fn test_create_spline_quadrant1(){
        //Functional test to ensure that the splineknots are effectively updated using the
        //DeCasteljau algorithm with the call to create_spline() using points from Q1

        //Setup
        //Points for splineknot one
        let p1 = Point::new(0.,0.);
        let p2 = Point::new(1., 2.);
        let p3 = Point::new(1.5, 2.4);
        let p4 = Point::new(2.6, 3.3);
        //Points for splineknot two
        let p5 = Point::new(0., 1.);
        let p6 = Point::new(2., 2.);
        let p7 = Point::new(1.9, 2.4);
        let p8 = Point::new(2.7, 3.3);
        //Splineknots
        let mut s1 = SplineKnots::create_from_points(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create_from_points(&p5, &p6, &p7, &p8);
        //the curve
        let mut d1 = DeCasteljauPoints::create();

        //Call
        d1.create_spline(& mut s1,  & mut s2);

        //Test
        assert_eq!(s2.a.x, d1.fin.x);
        assert_eq!(s2.a.y, d1.fin.y);
        assert_eq!(s2.b.x, d1.bccd.x);
        assert_eq!(s2.b.y, d1.bccd.y);
        assert_eq!(s2.c.x, d1.cd.x);
        assert_eq!(s2.c.y, d1.cd.y);
        assert_eq!(s2.d.x, p4.x);
        assert_eq!(s2.d.y, p4.y);
        assert_eq!(s1.b.x, d1.ab.x);
        assert_eq!(s1.b.y, d1.ab.y);
        assert_eq!(s1.c.x, d1.abbc.x);
        assert_eq!(s1.c.y, d1.abbc.y);
        assert_eq!(s1.d.x, d1.fin.x);
        assert_eq!(s1.d.y, d1.fin.y);
    }

    #[test]
    fn test_create_spline_quadrant2(){
        //Functional test to ensure that the splineknots are effectively updated using the
        //DeCasteljau algorithm with the call to create_spline() using points from Q2

        //Setup
        //Points for splineknot one
        let p1 = Point::new(0.,0.);
        let p2 = Point::new(-1., 2.);
        let p3 = Point::new(-1.5, 2.4);
        let p4 = Point::new(-2.6, 3.3);
        //Points for splineknot 2
        let p5 = Point::new(0., 0.);
        let p6 = Point::new(-2., 2.);
        let p7 = Point::new(-1.9, 2.4);
        let p8 = Point::new(-2.7, 3.3);
        //declare splineknots
        let mut s1 = SplineKnots::create_from_points(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create_from_points(&p5, &p6, &p7, &p8);
        //curve
        let mut d1 = DeCasteljauPoints::create();

        //Call
        d1.create_spline(& mut s1,  & mut s2);

        //Test
        assert_eq!(s2.a.x, d1.fin.x);
        assert_eq!(s2.a.y, d1.fin.y);
        assert_eq!(s2.b.x, d1.bccd.x);
        assert_eq!(s2.b.y, d1.bccd.y);
        assert_eq!(s2.c.x, d1.cd.x);
        assert_eq!(s2.c.y, d1.cd.y);
        assert_eq!(s2.d.x, p4.x);
        assert_eq!(s2.d.y, p4.y);
        assert_eq!(s1.b.x, d1.ab.x);
        assert_eq!(s1.b.y, d1.ab.y);
        assert_eq!(s1.c.x, d1.abbc.x);
        assert_eq!(s1.c.y, d1.abbc.y);
        assert_eq!(s1.d.x, d1.fin.x);
        assert_eq!(s1.d.y, d1.fin.y);
    }

    #[test]
    fn test_create_spline_quadrant3(){
        //Functional test to ensure that the splineknots are effectively updated using the
        //DeCasteljau algorithm with the call to create_spline() using points from Q3

        //Setup
        //Points for splineknot one
        let p1 = Point::new(0., 0.);
        let p2 = Point::new(-1., -2.);
        let p3 = Point::new(-1.5, -2.4);
        let p4 = Point::new(-2.6, -3.3);
        //Points for splineknot 2
        let p5 = Point::new(0., -1.);
        let p6 = Point::new(-2., -2.);
        let p7 = Point::new(-1.9, -2.4);
        let p8 = Point::new(-2.7, -3.3);
        //declare splineknots
        let mut s1 = SplineKnots::create_from_points(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create_from_points(&p5, &p6, &p7, &p8);
        //curve
        let mut d1 = DeCasteljauPoints::create();

        //Call
        d1.create_spline(& mut s1,  & mut s2);

        //Test
        assert_eq!(s2.a.x, d1.fin.x);
        assert_eq!(s2.a.y, d1.fin.y);
        assert_eq!(s2.b.x, d1.bccd.x);
        assert_eq!(s2.b.y, d1.bccd.y);
        assert_eq!(s2.c.x, d1.cd.x);
        assert_eq!(s2.c.y, d1.cd.y);
        assert_eq!(s2.d.x, p4.x);
        assert_eq!(s2.d.y, p4.y);
        assert_eq!(s1.b.x, d1.ab.x);
        assert_eq!(s1.b.y, d1.ab.y);
        assert_eq!(s1.c.x, d1.abbc.x);
        assert_eq!(s1.c.y, d1.abbc.y);
        assert_eq!(s1.d.x, d1.fin.x);
        assert_eq!(s1.d.y, d1.fin.y);
    }

    #[test]
    fn test_create_spline_quadrant4(){
        //Functional test to ensure that the splineknots are effectively updated using the
        //DeCasteljau algorithm with the call to create_spline() using points from Q4

        //Setup
        //Points for splineknot one
        let p1 = Point::new(0., 0.);
        let p2 = Point::new(1., -2.);
        let p3 = Point::new(1.5, -2.4);
        let p4 = Point::new(2.6, -3.3);
        //Points for splineknot 2
        let p5 = Point::new(0., -1.);
        let p6 = Point::new(2., -2.);
        let p7 = Point::new(1.9, -2.4);
        let p8 = Point::new(2.7, -3.3);
        //declare splineknots
        let mut s1 = SplineKnots::create_from_points(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create_from_points(&p5, &p6, &p7, &p8);
        //curve
        let mut d1 = DeCasteljauPoints::create();

        //Call
        d1.create_spline(& mut s1,  & mut s2);

        //Test
        assert_eq!(s2.a.x, d1.fin.x);
        assert_eq!(s2.a.y, d1.fin.y);
        assert_eq!(s2.b.x, d1.bccd.x);
        assert_eq!(s2.b.y, d1.bccd.y);
        assert_eq!(s2.c.x, d1.cd.x);
        assert_eq!(s2.c.y, d1.cd.y);
        assert_eq!(s2.d.x, p4.x);
        assert_eq!(s2.d.y, p4.y);
        assert_eq!(s1.b.x, d1.ab.x);
        assert_eq!(s1.b.y, d1.ab.y);
        assert_eq!(s1.c.x, d1.abbc.x);
        assert_eq!(s1.c.y, d1.abbc.y);
        assert_eq!(s1.d.x, d1.fin.x);
        assert_eq!(s1.d.y, d1.fin.y);
    }

    #[test]
    fn test_create_spline_mixedquad(){
        //Functional test to ensure that the splineknots are effectively updated using the
        //DeCasteljau algorithm with the call to create_spline() using points from Q1/Q2/Q3/Q4

        //Setup
        //Points for s1
        let p1 = Point::new(2., -2.9);
        let p2 = Point::new(1., 2.);
        let p3 = Point::new(-1.5, -2.4);
        let p4 = Point::new(-2.6, 3.3);
        //Points for s2
        let p5 = Point::new(0., -1.);
        let p6 = Point::new(-2., 2.);
        let p7 = Point::new(-1.9, -2.4);
        let p8 = Point::new(2.7, 3.3);
        //declare splineknots
        let mut s1 = SplineKnots::create_from_points(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create_from_points(&p5, &p6, &p7, &p8);
        //decasteljau points
        let mut d1 = DeCasteljauPoints::create();

        //Call
        d1.create_spline(& mut s1,  & mut s2);

        //Test
        assert_eq!(s2.a.x, d1.fin.x);
        assert_eq!(s2.a.y, d1.fin.y);
        assert_eq!(s2.b.x, d1.bccd.x);
        assert_eq!(s2.b.y, d1.bccd.y);
        assert_eq!(s2.c.x, d1.cd.x);
        assert_eq!(s2.c.y, d1.cd.y);
        assert_eq!(s2.d.x, p4.x);
        assert_eq!(s2.d.y, p4.y);
        assert_eq!(s1.b.x, d1.ab.x);
        assert_eq!(s1.b.y, d1.ab.y);
        assert_eq!(s1.c.x, d1.abbc.x);
        assert_eq!(s1.c.y, d1.abbc.y);
        assert_eq!(s1.d.x, d1.fin.x);
        assert_eq!(s1.d.y, d1.fin.y);
    }

    #[test]
    fn test_create_spline_all_origin(){
        //Functional test to ensure that the splineknots are effectively updated using the
        //DeCasteljau algorithm with the call to create_spline() using points from Q1/Q2/Q3/Q4

        //Setup
        //Points for s1
        let p1 = Point::origin();
        let p2 = Point::origin();
        let p3 = Point::origin();
        let p4 = Point::origin();
        //Points for s2
        let p5 = Point::origin();
        let p6 = Point::origin();
        let p7 = Point::origin();
        let p8 = Point::origin();
        //declare splineknots
        let mut s1 = SplineKnots::create_from_points(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create_from_points(&p5, &p6, &p7, &p8);
        //decasteljau points
        let mut d1 = DeCasteljauPoints::create();

        //Call
        d1.create_spline(& mut s1,  & mut s2);

        //Test
        assert_eq!(s2.a.x, d1.fin.x);
        assert_eq!(s2.a.y, d1.fin.y);
        assert_eq!(s2.b.x, d1.bccd.x);
        assert_eq!(s2.b.y, d1.bccd.y);
        assert_eq!(s2.c.x, d1.cd.x);
        assert_eq!(s2.c.y, d1.cd.y);
        assert_eq!(s2.d.x, p4.x);
        assert_eq!(s2.d.y, p4.y);
        assert_eq!(s1.b.x, d1.ab.x);
        assert_eq!(s1.b.y, d1.ab.y);
        assert_eq!(s1.c.x, d1.abbc.x);
        assert_eq!(s1.c.y, d1.abbc.y);
        assert_eq!(s1.d.x, d1.fin.x);
        assert_eq!(s1.d.y, d1.fin.y);
    }
}



