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

use common_geometry::{Point, LineSegment};

/// ## Trapezoid
///
/// Defines a trapezoid as four points.
struct Trapezoid {
    a: Point,
    b: Point,
    c: Point,
    d: Point,
}

// A TrapezoidBase is always two parallel line segments
// If a trapezoid is a rectangle, it has two base pairs, otherwise just one
struct TrapezoidBasePair(LineSegment, LineSegment);

impl PartialEq for TrapezoidBasePair {
    fn eq(&self, other: &TrapezoidBasePair) -> bool {
        (self.0 == other.0 && self.1 == other.1) ||
        (self.0 == other.1 && self.1 == other.0)
    }
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

    // Returns a Vec<LineSegment> of the four lines that make up this Trapezoid.
    fn get_lines(&self) -> Vec<LineSegment> {
        // TODO: This algorithm is probably not general!!! research further...
        let mut points = vec![self.a, self.b, self.c, self.d];
        points.sort_by(|&a, &b| { a.x.partial_cmp(&b.x).unwrap() });
        vec![
            LineSegment::from_points(points[0], points[1]),
            LineSegment::from_points(points[1], points[3]),
            LineSegment::from_points(points[3], points[2]),
            LineSegment::from_points(points[2], points[0]),
        ]
    }

    /// Returns self's base line segments.
    ///
    /// A Trapezoid's base line segments are the parallel lines that form the Trapezoid.
    /// If the returned Vec is of length 1, it is a normal trapezoid.
    /// If the returned Vec is of length 2, it is a rectangle.
    fn bases(&self) -> Vec<TrapezoidBasePair> {
        let mut points = vec![self.a, self.b, self.c, self.d];
        points.sort_by(|&a, &b| { a.x.partial_cmp(&b.x).unwrap() });

        let mut possible_lines = Vec::new();
        for outer in 0..points.len() {
            for inner in (outer+1)..points.len() {
                let line = LineSegment::from_points(points[outer], points[inner]);
                 possible_lines.push(line);
            }
        }

        let mut base_pairs = Vec::new();
        for outer in 0..possible_lines.len() {
            for inner in (outer+1)..possible_lines.len() {
                let line1 = possible_lines[inner];
                let line2 = possible_lines[outer];
                if line1.slope() == line2.slope() {
                    let base_pair = TrapezoidBasePair(line1, line2);
                    base_pairs.push(base_pair);
                }
            }
        }

        base_pairs
    }

    fn contains_point(&self, point: &Point) -> bool {
        let mut crossing_count = 0;
        for line in self.get_lines().iter() {
            if ray_from_point_crosses_line(point, line) {
                crossing_count += 1;
            }
        }

        crossing_count % 2 != 0
    }
}

/// Returns true if a ray running along the x-axis intersects the line `line`.
fn ray_from_point_crosses_line(point: &Point, line: &LineSegment) -> bool {
    let p1 = line.point1 - *point;
    let p2 = line.point2 - *point;
    let origin = Point{x: 0., y: 0.};
    let point_is_on_vertex = p1 == origin || p2 == origin;
    if point_is_on_vertex  {
        true
    } else if p1.y.signum() != p2.y.signum() {
        if  p1.x > 0. && p2.x > 0. {
            true
        } else {
            // Find sign of x-crossing of point's ray and line
            let slope = line.slope();
            let line_point = line.point1;
            let b = line_point.y - slope * line_point.x;
            let x = (point.y - b) / slope;
            x.is_sign_positive()
        }
    } else {
            false
    }
}

#[cfg(test)]
mod tests {
    use super::{Trapezoid, TrapezoidBasePair, ray_from_point_crosses_line};
    use common_geometry::{Point, LineSegment};

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
    fn crossings_test() {
        let p = Point{x: 1., y: 1.};
        let line = LineSegment::new(0., 0., 2., 2.);
        assert!(ray_from_point_crosses_line(&p, &line));
    }

    #[test]
    #[should_panic]
    fn crossings_test2() {
        let p = Point{x: 1., y: 1.};
        let line = LineSegment::new(2., 2., 3., 3.);
        assert!(ray_from_point_crosses_line(&p, &line));
    }

    #[test]
    fn point_in_trapezoid() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 0., y: 2.};
        let c = Point{x: 2., y: 0.};
        let d = Point{x: 2., y: 2.};
        let trap = Trapezoid::from_points(a, b, c, d);

        let test_point = Point{x: 1., y: 1.};
        assert!(trap.contains_point(&test_point));
    }


    #[test]
    fn trapezoid_bases() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 4., y: 0.};
        let c = Point{x: 2., y: 2.};
        let d = Point{x: 3., y: 2.};
        let trap = Trapezoid::from_points(a, b, c, d);
        let bases = trap.bases();

        let base1 = LineSegment::from_points(a, b);
        let base2 = LineSegment::from_points(c, d);
        let base_pair = TrapezoidBasePair(base1, base2);
        assert!(bases.contains(&base_pair));
    }
}
