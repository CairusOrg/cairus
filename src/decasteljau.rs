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

//use std::f32;
use common_geometry::Point;

///SplineKnots for bezier curves
pub struct SplineKnots{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

///Implements SplineKnots methods
impl SplineKnots{
    ///Creates a new SplineKnots with user defined points
    fn create(a: &Point, b: &Point, c: &Point, d: &Point)->SplineKnots{
        SplineKnots{
            a:Point::new(a.x, a.y),
            b:Point::new(b.x, b.y),
            c:Point::new(c.x, c.y),
            d:Point::new(d.x, d.y),
        }
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

#[cfg(test)]
mod tests{
    use::common_geometry::Point;
    use::decasteljau::SplineKnots;
    use::decasteljau::DeCasteljauPoints;
    use::decasteljau::lerp_half;


    #[test]
    fn test_splineknots(){
        //Functional test for the creation of Splineknots using provided points

        //Setup
        let p1 = Point::new(1., 1.);
        let p2 = Point::new(-1., 2.);
        let p3 = Point::new(-1.5, -2.4);
        let p4 = Point::new(2.6, -3.3);

        //Call
        let s1 = SplineKnots::create(&p1, &p2, &p3, &p4);
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
        let mut s1 = SplineKnots::create(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create(&p5, &p6, &p7, &p8);
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
        let mut s1 = SplineKnots::create(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create(&p5, &p6, &p7, &p8);
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
        let mut s1 = SplineKnots::create(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create(&p5, &p6, &p7, &p8);
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
        let mut s1 = SplineKnots::create(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create(&p5, &p6, &p7, &p8);
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
        let mut s1 = SplineKnots::create(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create(&p5, &p6, &p7, &p8);
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
        let mut s1 = SplineKnots::create(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create(&p5, &p6, &p7, &p8);
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



