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

 #[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Line {
    first_endpoint: Point,
    second_endpoint: Point,
}

impl Line {
    fn new(first_x: f32, first_y: f32, second_x: f32, second_y: f32) -> Line {
        Line {
            first_endpoint: Point{x: first_x, y: first_y},
            second_endpoint: Point{x: second_x, y: second_y}
        }
    }

    fn from_points(first_endpoint: Point, second_endpoint: Point) -> Line {
        Line {
            first_endpoint: first_endpoint,
            second_endpoint: second_endpoint,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Line, Point};

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

}
