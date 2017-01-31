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

use std::ops::{Add, Sub};
use std::cmp::Ordering;

/// ## Point
///
/// Defines a point by two floating points x and y.
 #[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    fn x_less_than(&self, other: &Point) -> bool {
        self.x < other.x
    }

    fn y_less_than(&self, other: &Point) -> bool {
        self.y < other.y
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x < other.x {
            Ordering::Less
        } else if self.x == other.x {
            if self.y < other.y {
                Ordering::Less
            } else if self.y == other.y {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Point {}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// ## Line
///
/// Defines a line by two points.
#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub point1: Point,
    pub point2: Point,
}

impl Line {
    // Returns a line.  Constructed by (x,y)-coordinates of two points.
    pub fn new(first_x: f32, first_y: f32, second_x: f32, second_y: f32) -> Line {
        Line {
            point1: Point{x: first_x, y: first_y},
            point2: Point{x: second_x, y: second_y}
        }
    }

    // Returns a line.  Constructed from two points.
    pub fn from_points(point1: Point, point2: Point) -> Line {
        Line {
            point1: point1,
            point2: point2,
        }
    }

    pub fn get_slope(&self) -> Option<f32> {
        let delta_x = self.point2.x - self.point1.x;
        let delta_y = self.point2.y - self.point1.y;
        match delta_x {
            0. => None,
            _ => Some(delta_y / delta_x)
        }
    }

    pub fn same_slope(&self, rhs: &Line) -> bool {
        match self.get_slope() {
                Some(slope1) => {
                    match rhs.get_slope() {
                        Some(slope2) => slope2 == slope1,
                        None => false,
                    }
                },
                None => {
                    match rhs.get_slope() {
                        Some(_) => false,
                        None => true
                    }
                },
            }
        }

    pub fn is_vertical(&self) -> bool {
        match self.get_slope() {
            Some(_) => false,
            None => true,
        }
    }


    // Returns a Point, the midpoint between the two endpoints of self.
    pub fn get_midpoint(&self) -> Point {
        let mid_x = self.point1.x + (self.point2.x - self.point1.x) / 2.;
        Point {
            x: mid_x,
            y: self.point1.y + (mid_x * self.get_slope().unwrap() ),
        }
    }

    // Returns a Vector of coordinates indicating which pixels this line should color when
    // rasterized.  The algorithm is a straight-forward DDA.
    pub fn into_pixel_coordinates(&self) -> Vec<(i32, i32)> {
        let slope = self.get_slope().unwrap();
        match slope <= 1. {
            true => self.step_by_x_coordinates(),
            false => self.step_by_y_coordinates(),
        }
    }

    fn step_by_x_coordinates(&self) -> Vec<(i32, i32)> {
        let max_x = self.point1.x.max(self.point2.x) as i32;
        let slope = self.get_slope().unwrap();
        let mut running_total_y = 0.;
        let mut result = Vec::with_capacity(max_x as usize);
        for x in 0..max_x {
            running_total_y += slope;
            let coordinate = (x, running_total_y.round() as i32);
            result.push(coordinate);
        }

        result
    }

    fn step_by_y_coordinates(&self) -> Vec<(i32, i32)> {
        let max_y = self.point1.y.max(self.point2.y) as i32;
        let mut result = Vec::with_capacity(max_y as usize);
        match self.get_slope() {
            Some(x) => {
                let slope = 1. / x;
                let mut running_total_x = 0.;
                for y in 0..max_y {
                    running_total_x += slope;
                    let coordinate = (running_total_x.round() as i32, y);
                    result.push(coordinate);
                }
            },

            None => {
                let mut result = Vec::with_capacity(max_y as usize);
                for y in 0..max_y {
                    let coordinate = (self.point1.x.round() as i32, y);
                    result.push(coordinate);
                }
            }
        }

        result
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
    use super::{Line, Point, Vector};

    #[test]
    fn point_lt() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        assert!(p1.x_less_than(&p2));
    }

    #[test]
    fn point_ordering_lt() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        assert!(p1 < p2);
    }

    #[test]
    #[should_panic]
    fn point_lt2() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        assert!(p2.x_less_than(&p1));
    }

    #[test]
    fn point_add() {
        let p = Point{x: 1., y: 1.};
        assert_eq!(p + p, Point{x: 2., y: 2.});
    }

    #[test]
    fn point_sub() {
        let p = Point{x: 1., y: 1.};
        assert_eq!(p - p, Point{x: 0., y: 0.});
    }

    #[test]
    fn line_new() {
        let line = Line::new(0., 0., 1., 1.);
        assert_eq!(line.point1, Point{x: 0., y: 0.});
        assert_eq!(line.point2, Point{x: 1., y: 1.});
    }

    #[test]
    fn line_from_points() {
        let p1 = Point{x: 0., y: 0.};
        let p2 = Point{x: 1., y: 1.};
        let line = Line::from_points(p1, p2);
        assert_eq!(line.point1, Point{x: 0., y: 0.});
        assert_eq!(line.point2, Point{x: 1., y: 1.});
    }

    #[test]
    fn line_get_slope() {
        let line = Line::new(0., 0., 1., 1.);
        assert_eq!(line.get_slope().unwrap(), 1.);
    }

    #[test]
    fn line_midpoint() {
        let line = Line::new(0., 0., 2., 2.);
        assert_eq!(line.get_midpoint(), Point{x: 1., y: 1.});
    }

    #[test]
    fn line_into_pixel_coordinates_slope_lt_one() {
        // The following coordinates were calculated by hand to be known pixels in the defined
        // line.
        let line = Line::new(0., 0., 20., 5.);
        let expected = vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 2),
        ];

        let pixel_coordinates = line.into_pixel_coordinates();
        for coordinate in expected {
            assert!(pixel_coordinates.contains(&coordinate));
        }
    }

    #[test]
    fn line_into_pixel_coordinates_slope_gt_one() {
        // The following coordinates were calculated by hand to be known pixels in the defined
        // line.
        let line = Line::new(0., 0., 5., 20.);
        let expected = vec![
            (0, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 5),
        ];

        let pixel_coordinates = line.into_pixel_coordinates();
        for coordinate in expected {
            assert!(pixel_coordinates.contains(&coordinate));
        }
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
