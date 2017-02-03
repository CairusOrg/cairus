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

use surfaces::ImageSurface;
use common_geometry::{Point, LineSegment};
use std::{f32, i32};
use std::collections::HashMap;

// Defines the a collection for holding a Trapezoid's bases.
//
// A Trapezoid's base line segments are always parallel.
// If a trapezoid is a rectangle, it has two base pairs, otherwise just one
//
// Warning! -  TrapezoidBasePair doesn't check for parallelity, it assumes it is being passed
//             parallel line segments.
struct TrapezoidBasePair(LineSegment, LineSegment);

// Returns true if TrapezoidBasePairs have the same LineSegments, disregarding order.
impl PartialEq for TrapezoidBasePair {
    fn eq(&self, other: &TrapezoidBasePair) -> bool {
        (self.0 == other.0 && self.1 == other.1) ||
        (self.0 == other.1 && self.1 == other.0)
    }
}

impl TrapezoidBasePair {
    fn slope(&self) -> f32 {
        self.0.slope()
    }
}

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

    // Returns a Vec<LineSegment> of the four lines that make up this Trapezoid.
    fn lines(&self) -> Vec<LineSegment> {
        // TODO: Organize lines to be returned in counter-clockwise order
        let bases = self.bases();
        if bases.len() == 2 {
            vec![bases[0].0, bases[0].1, bases[1].0, bases[1].1]
        } else {
            let base = &bases[0];
            let mut lines = vec![base.0, base.1];
            let slope = bases[0].slope(); // TrapezoidBasePair, not a LineSegment
            if slope == f32::INFINITY {
                let highest_from_base0 = base.0.highest_point();
                let lowest_from_base0 = base.0.lowest_point();
                let highest_from_base1 = base.1.highest_point();
                let lowest_from_base1 = base.1.lowest_point();

                let top_leg = LineSegment::from_points(highest_from_base0, highest_from_base1);
                let bottom_leg = LineSegment::from_points(lowest_from_base0, lowest_from_base1);
                lines.push(top_leg);
                lines.push(bottom_leg);
            } else {
                let leftmost_from_base0 = base.0.leftmost_point();
                let rightmost_from_base0 = base.0.rightmost_point();
                let leftmost_from_base1 = base.1.leftmost_point();
                let rightmost_from_base1 = base.1.rightmost_point();

                let left_leg = LineSegment::from_points(leftmost_from_base0, leftmost_from_base1);
                let right_leg = LineSegment::from_points(rightmost_from_base0, rightmost_from_base1);
                lines.push(left_leg);
                lines.push(right_leg);
            }

            lines
        }
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
        for line in self.lines().iter() {
            if ray_from_point_crosses_line(point, line) {
                crossing_count += 1;
            }
        }

        crossing_count % 2 != 0
    }

    fn into_pixels(&self) -> Vec<Pixel> {
        let outline = self.lines();

        let mut outline_pixels = Vec::new();
        for line in outline {
            for pixel in line.into_pixel_coordinates() {
                outline_pixels.push(pixel);
            }
        }

        // Order by y-value, for scanline from bottom
        outline_pixels.sort_by(|&a, &b| a.1.cmp(&b.1));
        let mut minmap = HashMap::new();
        let mut maxmap = HashMap::new();
        for pixel in outline_pixels.iter() {
            minmap.insert(pixel.1, pixel.0);
            maxmap.insert(pixel.1, pixel.0);
        }

        for pixel in outline_pixels.iter() {
            if pixel.0 < *minmap.get_mut(&pixel.1).unwrap() {
                minmap.insert(pixel.1, pixel.0);
            }

            if pixel.0 > *maxmap.get_mut(&pixel.1).unwrap() {
                maxmap.insert(pixel.1, pixel.0);
            }
        }

        let mut pixels = Vec::new();
        let min_y = outline_pixels[0].1;
        let max_y = outline_pixels[outline_pixels.len() - 1].1 + 1;
        for y in min_y..max_y {
            for x in minmap[&y]..(maxmap[&y] + 1) {
                let pixel = Pixel{x: x, y: y};
                pixels.push(pixel);

            }
        }

        pixels
    }

    fn extent(&self) -> Extent {
        let mut smallest_x = self.a.x;
        let mut biggest_x = self.a.x;
        let mut smallest_y = self.a.y;
        let mut biggest_y = self.a.y;
        let points = vec![self.a, self.b, self.c, self.d];
        for point in points {
            if point.x < smallest_x {
                smallest_x = point.x;
            }

            if point.x > biggest_x {
                biggest_x = point.x;
            }

            if point.y < smallest_y {
                smallest_y = point.y;
            }

            if point.y > biggest_y {
                biggest_y = point.y;
            }
        }

        let a = Point{x: smallest_x, y: smallest_y};
        let b = Point{x: biggest_x, y: smallest_y};
        let c = Point{x: biggest_x, y: biggest_y};
        let d = Point{x: smallest_x, y: biggest_y};
        Extent::from_points(a, b, c, d)
    }
}

#[derive(Debug)]
struct Pixel {
    x: i32,
    y: i32,
}

impl Pixel {
    fn sample_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let x_increment = 1. / 18.;
        let y_increment = 1. / 16.;
        let mut x = self.x as f32 + x_increment;
        let mut y = self.y as f32 + y_increment;
        while x < (self.x as f32 + 1. - x_increment) as f32 {
            for _ in 0..15 {
                let point = Point{x: x, y: y};
                points.push(point);
                y += y_increment;
            }

            x += x_increment;
            y = self.y as f32 + y_increment;
        }

        points
    }
}


/// # Extent
///
/// An extent is the smallest possible rectangle that could surround a given Trapezoid.
/// Points go in counter-clockwise order.  `a` is least x and least y, b is most x and least y, etc...
struct Extent {
    a: Point,
    b: Point,
    c: Point,
    d: Point,
}

impl Extent {
    fn width(&self) -> f32 {
        (self.a.x - self.b.x).abs()
    }

    fn height(&self) -> f32 {
        (self.a.y - self.c.y).abs()
    }

    fn lines(&self) -> Vec<LineSegment> {
        vec![
            LineSegment::from_points(self.a, self.b),
            LineSegment::from_points(self.b, self.c),
            LineSegment::from_points(self.c, self.d),
            LineSegment::from_points(self.d, self.a),
        ]
    }

    fn from_points(a: Point, b: Point, c: Point, d: Point) -> Extent {
        Extent {
            a: a,
            b: b,
            c: c,
            d: d,
        }
    }

    fn raster_range(&self) -> PixelGridIterator {
        let mut smallest_x = i32::MAX;
        let mut biggest_x = i32::MIN;
        let mut smallest_y = i32::MAX;
        let mut biggest_y = i32::MIN;

        for line in self.lines() {
            for (x, y) in line.into_pixel_coordinates() {
                if x < smallest_x {
                    smallest_x = x;
                }

                if x > biggest_x {
                    biggest_x = x;
                }

                if y < smallest_y {
                    smallest_y = y;
                }

                if y > biggest_y {
                    biggest_y = y;
                }
            }
        }

        let start = (smallest_x, smallest_y);
        let end = (biggest_x, biggest_y);
        PixelGridIterator{current: start, end: end, width: biggest_x - smallest_x}
    }
}

struct PixelGridIterator {
    current: (i32, i32),
    end: (i32, i32),
    width: i32,
}

impl Iterator for PixelGridIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        if self.current == self.end {
            return None
        }

        let result = self.current;
        if self.current.0 < self.width {
            self.current = (self.current.0 + 1, self.current.1);
        } else {
            self.current = (0, self.current.1 + 1);
        }

        Some(result)
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
            let b = line.point1.y - slope * line.point1.x;
            let x = ((point.y - b) / slope) - point.x;
            x.is_sign_positive()
        }
    } else {
            false
    }
}

fn mask_from_trapezoids(trapezoids: &Vec<Trapezoid>, width: usize, height: usize) -> ImageSurface {
    let mut mask = ImageSurface::create(width, height);

    for trapezoid in trapezoids {
        for pixel in trapezoid.into_pixels() {
            let mut rgba = mask.get_mut(pixel.x as usize, pixel.y as usize);

            let mut successes = 0;
            for sample_point in pixel.sample_points() {
                if trapezoid.contains_point(&sample_point) {
                    successes +=1;
                }
            }

            rgba.alpha = successes as f32 / 255.;
         }
     }

     mask
}


#[cfg(test)]
mod tests {
    use super::{Trapezoid, TrapezoidBasePair, ray_from_point_crosses_line, mask_from_trapezoids};
    use common_geometry::{Point, LineSegment};


    ///TODO: Test what happens with bad point values

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
    fn trapezoid_vertical_bases_get_lines() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 2., y: 1.};
        let c = Point{x: 2., y: 2.};
        let d = Point{x: 0., y: 3.};
        let ab = LineSegment::from_points(a, b);
        let bc = LineSegment::from_points(b, c);
        let cd = LineSegment::from_points(a, b);
        let da = LineSegment::from_points(b, c);

        let trap = Trapezoid::from_points(a, b, c, d);
        let lines = trap.lines();
        assert!(lines.contains(&ab));
        assert!(lines.contains(&bc));
        assert!(lines.contains(&cd));
        assert!(lines.contains(&da));
        assert_eq!(lines.len(), 4);
    }


    #[test]
    fn trapezoid_rectangle_get_lines() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 2., y: 0.};
        let c = Point{x: 2., y: 2.};
        let d = Point{x: 0., y: 2.};
        let ab = LineSegment::from_points(a, b);
        let bc = LineSegment::from_points(b, c);
        let cd = LineSegment::from_points(a, b);
        let da = LineSegment::from_points(b, c);

        let trap = Trapezoid::from_points(a, b, c, d);
        let lines = trap.lines();
        assert!(lines.contains(&ab));
        assert!(lines.contains(&bc));
        assert!(lines.contains(&cd));
        assert!(lines.contains(&da));
        assert_eq!(lines.len(), 4);
    }


    #[test]
    fn trapezoid_horizontal_base_lines() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 4., y: 0.};
        let c = Point{x: 2., y: 2.};
        let d = Point{x: 1., y: 2.};
        let ab = LineSegment::from_points(a, b);
        let bc = LineSegment::from_points(b, c);
        let cd = LineSegment::from_points(a, b);
        let da = LineSegment::from_points(b, c);

        let trap = Trapezoid::from_points(a, b, c, d);
        let lines = trap.lines();
        assert!(lines.contains(&ab));
        assert!(lines.contains(&bc));
        assert!(lines.contains(&cd));
        assert!(lines.contains(&da));
        assert_eq!(lines.len(), 4);
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

    #[test]
    fn trapezoid_base_pair_slope() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 1., y: 1.};
        let c = Point{x: 1., y: 0.};
        let d = Point{x: 2., y: 1.};

        let base1 = LineSegment::from_points(a, b);
        let base2 = LineSegment::from_points(c, d);
        let base_pair = TrapezoidBasePair(base1, base2);
        assert_eq!(base_pair.slope(), 1.);
    }


    #[test]
    fn trapezoid_extent_width() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 1., y: 0.};
        let c = Point{x: 1., y: 1.};
        let d = Point{x: 0., y: 1.};
        let trap = Trapezoid::from_points(a, b, c, d);
        let extent = trap.extent();
        assert_eq!(extent.width(), 1.);
    }


    #[test]
    fn trapezoid_extent_height() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 1., y: 0.};
        let c = Point{x: 1., y: 1.};
        let d = Point{x: 0., y: 1.};
        let trap = Trapezoid::from_points(a, b, c, d);
        let extent = trap.extent();
        assert_eq!(extent.height(), 1.);
    }

    #[test]
    fn trapezoid_extent_lines() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 1., y: 0.};
        let c = Point{x: 1., y: 1.};
        let d = Point{x: 0., y: 1.};
        let trap = Trapezoid::from_points(a, b, c, d);
        let extent = trap.extent();
        let extent_lines = extent.lines();
        let trap_lines = trap.lines();

        for line in extent_lines {
            assert!(trap_lines.contains(&line));
        }
    }

    // Tests that the returned ImageSurface is correct.
    // This test assumes the Trapezoid::lines() is functioning correctly.
    // We check that the pixels in between the outline of the Trapezoid (non-inclusive)
    // have alpha values that are not zero.
    #[test]
    fn mask_from_single_trapezoid() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 10., y: 0.};
        let c = Point{x: 5., y: 9.};
        let d = Point{x: 7., y: 9.};
        let trap = Trapezoid::from_points(a, b, c, d);
        let trapezoids = vec![trap];
        let mask = mask_from_trapezoids(&trapezoids, 10, 10);
        let rgba = mask.get(2, 1);
        assert!(rgba.unwrap().alpha > 0.);

        let rgba = mask.get(1, 9);
        assert!(rgba.unwrap().alpha == 0.);
    }
}
