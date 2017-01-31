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
 *	Bobby Eshleman <bobbyeshleman@gmail.com>
 *
 */

//! This module defines geometric structs and methods common to algorithms used throughout Cairus.

use std::ops::Add;
use std::f32;

/// ## Point
///
/// Defines a point by two floating points x and y.
 #[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// ## LineSegment
///
/// Defines a line by two points.
#[derive(Debug, Copy, Clone)]
pub struct LineSegment {
    point1: Point,
    point2: Point,
}

impl LineSegment {
    // Returns a line.  Constructed by (x,y)-coordinates of two points.
    pub fn new(first_x: f32, first_y: f32, second_x: f32, second_y: f32) -> LineSegment {
        LineSegment {
            point1: Point{x: first_x, y: first_y},
            point2: Point{x: second_x, y: second_y}
        }
    }

    // Returns a line.  Constructed from two points.
    pub fn from_points(point1: Point, point2: Point) -> LineSegment {
        LineSegment {
            point1: point1,
            point2: point2,
        }
    }

    /// Returns the slope of this LineSegment.
    ///
    /// If the slope is completely vertical, this function will return f32::INFINITY, otherwise
    /// it will return any valid f32 (assuming valid points form this LineSegment).
    ///
    /// One of the ways Cairo C implements slope comparision is using the following formula:
    ///     `(adx * bdy) ? (bdx * ady)`, where `?` is the comparison operator.
    ///
    /// Using this equation, any line with a slope of `delta x == 0` (divide by zero for the
    /// common rise/run slope equation) will zero out one side of the equation.  This means that
    /// any vertical line has a greater slope than any other non-vertical line.
    ///
    /// Fortunately, this logic is exactly equivalent to Rust's f32 implementation, and so the following
    /// slope implementation simply leverages f32's native comparison operations.  The only change
    /// is to make negative infinity a positive infinity, so that all vertical lines have equal
    /// slope, regardless of the direction from point1 to point2.
    pub fn slope(&self) -> f32 {
        let delta_x = self.point2.x - self.point1.x;
        let delta_y = self.point2.y - self.point1.y;
        let result = delta_y / delta_x;

        // Slope of negative infinity should be equal to positive infinity.
        if result.is_infinite() && result.is_sign_negative() {
            f32::INFINITY
        } else {
            result
        }
    }

    // Returns a Point, the midpoint between the two endpoints of self.
    pub fn midpoint(&self) -> Point {
        Point {
            x: (self.point1.x + self.point2.x) / 2.,
            y: (self.point1.y + self.point2.y) / 2.,
        }
    }
}

/// ## Vector
///
/// Defines a vector by (x, y) direction.
#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector {
            x: x,
            y: y,
        }
    }

    // Returns the dot product of self and rhs.
    pub fn dot_product(&self, rhs: &Vector) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn get_magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    // Returns the angle between self and rhs.
    pub fn angle_between(&self, rhs: &Vector) -> f32 {
        (
            self.dot_product(rhs) / (self.get_magnitude() * rhs.get_magnitude())
        ).acos()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod tests {
    use super::{LineSegment, Point, Vector};

    #[test]
    fn line_new() {
        let line = LineSegment::new(0., 0., 1., 1.);
        assert_eq!(line.point1, Point{x: 0., y: 0.});
        assert_eq!(line.point2, Point{x: 1., y: 1.});
    }

    #[test]
    fn line_from_points() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        let line = LineSegment::from_points(p1, p2);
        assert_eq!(line.point1, Point{x: 0., y: 0.});
        assert_eq!(line.point2, Point{x: 1., y: 1.});
    }

    #[test]
    fn line_slope() {
        let line = LineSegment::new(0., 0., 1., 1.);
        assert_eq!(line.slope(), 1.);
    }

    #[test]
    fn line_midpoint() {
        let line = LineSegment::new(0., 0., 2., 2.);
        assert_eq!(line.midpoint(), Point{x: 1., y: 1.});
    }

    #[test]
    fn line_opposite_direction_midpoint() {
        let line = LineSegment::new(2., 2., 0., 0.);
        assert_eq!(line.midpoint(), Point{x: 1., y: 1.});
    }

    #[test]
    fn line_negative_slope_midpoint() {
        let line = LineSegment::new(0., 0., 2., -2.);
        assert_eq!(line.midpoint(), Point{x: 1., y: -1.});
    }

    #[test]
    fn vertical_line_midpoint() {
        let line = LineSegment::new(0., 0., 0., 2.);
        assert_eq!(line.midpoint(), Point{x: 0., y: 1.});
    }

    #[test]
    fn vertical_negative_slope_midpoint() {
        let line = LineSegment::new(0., 0., 0., -2.);
        assert_eq!(line.midpoint(), Point{x: 0., y: -1.});
    }

    #[test]
    fn vertical_slope_gt_positive() {
        let vertical = LineSegment::new(0., 0., 0., 1.);
        let positive = LineSegment::new(0., 0., 1., 1.);
        assert!(vertical.slope() > positive.slope());
    }

    #[test]
    fn vertical_slope_gt_negative() {
        let vertical = LineSegment::new(0., 0., 0., 1.);
        let negative = LineSegment::new(0., 0., 1., -1.);
        assert!(vertical.slope() > negative.slope());
    }

    #[test]
    fn vertical_slope_eq_vertical() {
        let vertical1 = LineSegment::new(0., 0., 0., 1.);
        let vertical2 = LineSegment::new(2., 2., 2., -1.);
        assert_eq!(vertical1.slope(), vertical2.slope());
    }

    #[test]
    fn vector_new() {
        let vec = Vector::new(1., 1.);
        assert_eq!(vec.x, 1.);
        assert_eq!(vec.y, 1.);
    }

    #[test]
    fn vector_add() {
        let a = Vector::new(0., 0.);
        let b = Vector::new(1., 1.);
        let c = a + b;
        assert_eq!(c, b);
    }

    #[test]
    fn vector_dot_product() {
        let a = Vector::new(1., 0.);
        let b = Vector::new(1., 1.);
        let c = a.dot_product(&b);
        assert_eq!(c, 1.);
    }

    #[test]
    fn vector_magnitude() {
        let b = Vector::new(3., 4.);
        assert_eq!(b.get_magnitude(), 5.);
    }

    #[test]
    fn vector_angle_between() {
        let a = Vector::new(1., 0.);
        let b = Vector::new(1., 1.);
        assert_eq!(a.angle_between(&b).to_degrees(), 45.)
    }
}
