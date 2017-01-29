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

use std::ops::Add;

 #[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f32,
    y: f32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    first_endpoint: Point,
    second_endpoint: Point,
}

impl Line {
    pub fn new(first_x: f32, first_y: f32, second_x: f32, second_y: f32) -> Line {
        Line {
            first_endpoint: Point{x: first_x, y: first_y},
            second_endpoint: Point{x: second_x, y: second_y}
        }
    }

    pub fn from_points(first_endpoint: Point, second_endpoint: Point) -> Line {
        Line {
            first_endpoint: first_endpoint,
            second_endpoint: second_endpoint,
        }
    }

    pub fn get_slope(&self) -> f32 {
        let delta_x = self.second_endpoint.x - self.first_endpoint.x;
        let delta_y = self.second_endpoint.y - self.first_endpoint.y;
        delta_y / delta_x
    }

    pub fn get_midpoint(&self) -> Point {
        let mid_x = self.first_endpoint.x + (self.second_endpoint.x - self.first_endpoint.x) / 2.;
        Point {
            x: mid_x,
            y: self.first_endpoint.y + (mid_x * self.get_slope() ),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    fn new(x: f32, y: f32) -> Vector {
        Vector {
            x: x,
            y: y,
        }
    }

    fn dot_product(&self, rhs: &Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y
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
    use super::{Line, Point, Vector};

    #[test]
    fn line_new() {
        let line = Line::new(0., 0., 1., 1.);
        assert_eq!(line.first_endpoint, Point{x: 0., y: 0.});
        assert_eq!(line.second_endpoint, Point{x: 1., y: 1.});
    }

    #[test]
    fn line_from_points() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        let line = Line::from_points(p1, p2);
        assert_eq!(line.first_endpoint, Point{x: 0., y: 0.});
        assert_eq!(line.second_endpoint, Point{x: 1., y: 1.});
    }

    #[test]
    fn line_get_slope() {
        let line = Line::new(0., 0., 1., 1.);
        assert_eq!(line.get_slope(), 1.);
    }

    #[test]
    fn line_midpoint() {
        let line = Line::new(0., 0., 2., 2.);
        assert_eq!(line.get_midpoint(), Point{x: 1., y: 1.});
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
        let a = Vector::new(0., 0.);
        let b = Vector::new(1., 1.);
        let c = a.dot_product(&b);
        assert_eq!(c, 0.);
    }
}
