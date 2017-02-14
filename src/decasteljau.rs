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

use std::f32;
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
            a:Point::create(a.x, a.y),
            b:Point::create(b.x, b.y),
            c:Point::create(c.x, c.y),
            d:Point::create(d.x, d.y),
        }
    }
}

///This function takes two Points and provides the median value
fn lerp_half(a: & Point, b: & Point)->Point{
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
    ///This method is implemented for testing purpose
    fn constructor(ab: Point, bc: Point, cd: Point, abbc: Point, bccd: Point, fin: Point)->DeCasteljauPoints{
        DeCasteljauPoints{
            ab: ab,
            bc: bc,
            cd: cd,
            abbc: abbc,
            bccd: bccd,
            fin: fin,
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

        s2.a = Point::create(self.fin.x, self.fin.y);
        s2.b = Point::create(self.bccd.x, self.bccd.y);
        s2.c = Point::create(self.cd.x, self.cd.y);
        s2.d = Point::create(s1.d.x, s1.d.y);

        s1.b = Point::create(self.ab.x, self.ab.y);
        s1.c = Point::create(self.abbc.x, self.abbc.y);
        s1.d = Point::create(self.fin.x, self.fin.y);
    }
}

mod tests{

    use::common_geometry::Point;
    use::decasteljau::SplineKnots;
    use::decasteljau::DeCasteljauPoints;
    use::decasteljau::lerp_half;

    ///tests in Quadrant I
    #[test]
    fn test_splineknots_positive(){

        let q1 = Point::create(0.,0.);
        let q2 = Point::create(1., 2.);
        let q3 = Point::create(1.5, 2.4);
        let q4 = Point::create(2.6, 3.3);

        let q5 = Point::create(0.,1.);
        let q6 = Point::create(2., 2.);
        let q7 = Point::create(1.9, 2.4);
        let q8 = Point::create(2.7, 3.3);

        let r1 = SplineKnots::create(&q1, &q2, &q3, &q4);
        let r2 = SplineKnots::create(&q5, &q6, &q7, &q8);

        assert_eq!(r1.a.x, 0.0);
        assert_eq!(r1.c.y, 2.4);
        assert_eq!(r2.d.x, 2.7);
        assert_eq!(r2.d.y, 3.3);
    }

    #[test]
    fn test_lerp_half(){
        let p7 = Point::create(1.9, 2.4);
        let p8 = Point::create(2.7, 3.3);

        let mut a1 = Point::origin();
        a1 = lerp_half(&p7, &p8);
        assert_eq!(a1.x, 2.3);
        assert_eq!(a1.y, 2.85);
    }

    #[test]
    fn test_create_spline_positive(){
        //Points for splineknot one
        let p1 = Point::create(0.,0.);
        let p2 = Point::create(1., 2.);
        let p3 = Point::create(1.5, 2.4);
        let p4 = Point::create(2.6, 3.3);

        //Points for splineknot two
        let p5 = Point::create(0.,1.);
        let p6 = Point::create(2., 2.);
        let p7 = Point::create(1.9, 2.4);
        let p8 = Point::create(2.7, 3.3);

        let mut s1 = SplineKnots::create(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create(&p5, &p6, &p7, &p8);

        let mut d1 = DeCasteljauPoints::create();

        assert_eq!(d1.ab.x, 0.0);

        d1.create_spline(& mut s1,  & mut s2);

        //testing if values of s1 and s2 changed accordingly
        assert_eq!(s2.a.x, d1.fin.x);
        assert_eq!(s2.a.y, d1.fin.y);
        assert_eq!(s2.b.x, d1.bccd.x);
        assert_eq!(s2.b.y, d1.bccd.y);
        assert_eq!(s2.c.x, d1.cd.x);
        assert_eq!(s2.c.y, d1.cd.y);
        assert_eq!(s2.d.x, p4.x);
        assert_eq!(s2.d.y, p4.y);
    }

    //test in Quadrant II
    #[test]
    fn test_create_spline_qii(){
        //Points for splineknot one
        let p1 = Point::create(0.,0.);
        let p2 = Point::create(-1., 2.);
        let p3 = Point::create(-1.5, 2.4);
        let p4 = Point::create(-2.6, 3.3);

        //Points for splineknot 2
        let p5 = Point::create(0.,1.);
        let p6 = Point::create(-2., 2.);
        let p7 = Point::create(-1.9, 2.4);
        let p8 = Point::create(-2.7, 3.3);

        //declare splineknots
        let mut s1 = SplineKnots::create(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create(&p5, &p6, &p7, &p8);

        let mut d1 = DeCasteljauPoints::create();
        assert_eq!(d1.ab.x, 0.0); //set to origin

        d1.create_spline(& mut s1,  & mut s2);

        //testing if values of s1 and s2 changed accordingly
        assert_eq!(s2.a.x, d1.fin.x);
        assert_eq!(s2.a.y, d1.fin.y);
        assert_eq!(s2.b.x, d1.bccd.x);
        assert_eq!(s2.b.y, d1.bccd.y);
        assert_eq!(s2.c.x, d1.cd.x);
        assert_eq!(s2.c.y, d1.cd.y);
        assert_eq!(s2.d.x, p4.x);
        assert_eq!(s2.d.y, p4.y);
    }

    #[test]
    fn test_create_spline_mixedquad(){
        //Points for s1
        let p1 = Point::create(2., -2.9);
        let p2 = Point::create(1., 2.);
        let p3 = Point::create(-1.5, -2.4);
        let p4 = Point::create(-2.6, 3.3);

        //Points for s2
        let p5 = Point::create(0., -1.);
        let p6 = Point::create(-2., 2.);
        let p7 = Point::create(-1.9, -2.4);
        let p8 = Point::create(2.7, 3.3);

        //declare splineknots
        let mut s1 = SplineKnots::create(&p1, &p2, &p3, &p4);
        let mut s2 = SplineKnots::create(&p5, &p6, &p7, &p8);

        let mut d1 = DeCasteljauPoints::create();

        assert_eq!(d1.ab.x, 0.0);

        d1.create_spline(& mut s1,  & mut s2);

        //testing if values of s1 and s2 changed accordingly
        assert_eq!(s2.a.x, d1.fin.x);
        assert_eq!(s2.a.y, d1.fin.y);
        assert_eq!(s2.b.x, d1.bccd.x);
        assert_eq!(s2.b.y, d1.bccd.y);
        assert_eq!(s2.c.x, d1.cd.x);
        assert_eq!(s2.c.y, d1.cd.y);
        assert_eq!(s2.d.x, p4.x);
        assert_eq!(s2.d.y, p4.y);
    }
}



