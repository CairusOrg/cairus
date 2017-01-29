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

use common_geometry::{Point, Line};

/// ## Trapezoid
///
/// Defines a trapezoid as four points.
struct Trapezoid {
    a: Point,
    b: Point,
    c: Point,
    d: Point,
}

impl Trapezoid {
    // Returns a new Trapezoid defined by coordinates.
    fn new(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32, dx: f32, dy: f32) -> Trapezoid {
        Trapezoid {
            a: Point {x: ax, y: ay},
            b: Point {x: bx, y: by},
            c: Point {x: cx, y: cy},
            d: Point {x: dx, y: dy},
        }
    }

    // Returns a new Trapezoid defined by points.
    fn from_points(a: Point, b: Point, c: Point, d: Point) -> Trapezoid {
        Trapezoid {
            a: a,
            b: b,
            c: c,
            d: d,
        }
    }

    // Returns a Vec<Line> of the four lines that make up this Trapezoid.
    fn get_lines(&self) -> Vec<Line> {
        vec![
            Line::from_points(self.a, self.b),
            Line::from_points(self.b, self.c),
            Line::from_points(self.c, self.d),
            Line::from_points(self.d, self.a),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::{Trapezoid};
    use common_geometry::Point;

    #[test]
    fn trapezoid_new() {
        let trap = Trapezoid::new(0., 0.,
                                  0., 1.,
                                  1., 0.,
                                  1., 1.);

        let a = Point{x: 0., y: 0.};
        let b = Point{x: 0., y: 1.};
        let c = Point{x: 1., y: 0.};
        let d = Point{x: 1., y: 1.};

        assert_eq!(trap.a, a);
        assert_eq!(trap.b, b);
        assert_eq!(trap.c, c);
        assert_eq!(trap.d, d);
    }

    #[test]
    fn trapezoid_from_points() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 0., y: 1.};
        let c = Point{x: 1., y: 0.};
        let d = Point{x: 1., y: 1.};
        let trap = Trapezoid::from_points(a, b, c, d);
        assert_eq!(trap.a, a);
        assert_eq!(trap.b, b);
        assert_eq!(trap.c, c);
        assert_eq!(trap.d, d);
    }

    #[test]
    fn trapezoid_get_lines() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 0., y: 1.};
        let c = Point{x: 1., y: 0.};
        let d = Point{x: 1., y: 1.};
        let point_vec = vec![a, b, c, d];
        let trap = Trapezoid::from_points(a, b, c, d);

        let mut points = Vec::new();
        for line in trap.get_lines() {
            points.push(line.point1);
            points.push(line.point2);
        }

        for point in point_vec {
            assert!(points.contains(&point));
        }
    }
}
