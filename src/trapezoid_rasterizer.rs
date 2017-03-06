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

//! This module defines trapezoid rasterization structs and functions.
//!
//! # Algorithms
//!   The following section will descibe the important algorithms used in Cairus's rasterization
//! sequence.
//!
//! ## Rasterization Overiew
//!
//!   When we take a trapezoid and map it onto pixels, we need to decide which pixels the trapezoid
//! actually covers.  Additionally, trapezoids will often only cover a part of a pixel but not the
//! full pixel itself.  In these cases, we need to figure out how much of the pixel is covered by
//! the trapezoid.  If we were to simply fill in every pixel that the trapezoid touches, the result
//! would be full of 'jaggies', it would look very pixelated.  We need to find these
//! pixels that the trapezoid only partially covers, and instead make them more transparent.
//! This will make the trapezoid's edges look much smoother (this is anti-aliasing).
//!
//!   In order to decide the degree to which a trapezoid covers any given pixel, we need to
//! divide that pixel into smaller parts.  For every smaller part that the trapezoid covers,
//! we can increase the amount that the trapezoid is considered to cover that pixel.  These smaller
//! parts are 'subpixel' or 'sampling points'.  The more subpixel points that are covered by the
//! trapezoid, the more opaque that pixel will be.  This is called point-sampling anti-aliasing.
//!
//!   The way we divide a pixel is into a 17x15 uniform grid.  For example, a single pixel goes
//! from image on the left, to that on the right.
//!
//!
//! Pixel                                          Subpixel grid
//!
//! +--------------------------+                   X--X -X--X---X---X---X--X--X
//! |                          |                   |                          |
//! |                          |                   X  X  X  X   X   X   X  X  X
//! |                          |    into point     |                          |
//! |                          |    sample         X  X  X  X   X   X   X  X  X
//! |                          |    grid           |                          |
//! |                          |   +------------>  X  X  X  X   X   X   X  X  X
//! |                          |                   |                          |
//! |                          |                   X  X  X  X   X   X   X  X  X
//! |                          |                   |                          |
//! |                          |                   X  X  X  X   X   X   X  X  X
//! |                          |                   |                          |
//! +--------------------------+                   X--X -X--X---X---X---X--X--X

//! Cairus iterates through each X in the Subpixel grid above, and checks if that X point is
//! inside the trapezoid.  If it is, the opacity of the original pixel will increase.
//!
//!  See the `fn Pixel::sample_points()` function for the implementation.
//!
//!  Checking If A Point Is In A Trapezoid
//!
//!  The algorithm used is a ray intersection algorithm and takes advantage of the even-odd
//!  rule. The idea is that if you take a point and make a ray that runs in the positive x-axis
//!  direction, it will intersect any given polygon an odd number of times or an even number
//!  of times.  If it intersects an *odd* number of times, the point is inside the polygon.  If it
//!  intersects an *even* number of times, it is outside the polygon.  The diagram below shows
//!  two points, one inside and one outside of a trapezoid.
//!
//!  ^
//!  |
//!  |                                              Internal point crosses
//!  |                                              convex trapezoid only once (odd).
//!  |                         XXXXXXXXXXXX
//!  |                        X            X
//!  |    External point     X        +------------------------>
//!  |    crosses twice.    X                X
//!  |     (even)          X                  X
//!  |          +------------------------------------------------------------->
//!  |                   X                      X
//!  |                  XXXXXXXXXXXXXXXXXXXXXXXXXX
//!  |
//!  |
//!  +-------------------------------------------------------------------------------->
//!
//!  As Cairus iterates through a pixel's subpixel points, it uses this ray intersection
//!  technique to deterimine whether the subpixel is inside or outside of the trapezoid.  For every
//!  subpixel point that is inside the opacity of that pixel increases by 1/255.  Because it is
//!  a 17x15 subpixel grid, and 17 * 15 = 255, for a trapezoid to make a pixel fully opaque, it
//!  must cover every single subpixel point inside that pixel.  If it doesn't cover any subpixel,
//!  the pixel is left transparent.
//!
//!  See `fn ray_from_point_crosses_line` for ray intersection algorithm, and
//!  `fn Trapezoid::contains_point` for how it is used to determine if a point is in a trapezoid.

use surfaces::ImageSurface;
use common_geometry::{Point, LineSegment};
use std::{f32, i32};
use std::collections::HashMap;

/// ## Trapezoid
///
/// Defines a trapezoid as four points.
///
/// ### Panics
/// `fn Trapezoid::from_bases` will panic if the LineSegments are not parallel.
///
/// TODO: Implement `fn points()` or `fn a()`, `fn b()` , etc...
/// TODO: Test/verify degenerate Trapezoid (a triangle) is still valid
pub struct Trapezoid {
    lines: Vec<LineSegment>
}

impl Trapezoid {

    // Returns a new Trapezoid defined by points.
    fn from_points(a: Point, b: Point, c: Point, d: Point) -> Trapezoid {
        let bases = bases_from_points(a, b, c, d);
        Trapezoid::from_bases(bases[0].0, bases[0].1)
    }

    // Returns a new Trapezoid from two bases
    fn from_bases(base1: LineSegment, base2: LineSegment) -> Trapezoid {
        if base1.length() != 0. &&
           base2.length() != 0. &&
           base1.slope() != base2.slope() {
            panic!("Trapezoid::from_bases() can only be called on LineSegments with equal slope!");
        }

        Trapezoid {
            lines: lines_from_bases(base1, base2)
        }
    }

    fn lines(&self) -> &Vec<LineSegment> {
        &self.lines
    }

    /// Returns true if this Trapezoid contains `point`, otherwise returns false
    fn contains_point(&self, point: &Point) -> bool {
        let mut crossing_count = 0;
        for line in self.lines().iter() {
            if ray_from_point_crosses_line(point, line) {
                crossing_count += 1;
            }
        }

        crossing_count % 2 != 0
    }

    /// Converts this trapezoid into a Vec of Pixels
    ///
    /// The returned pixels don't contain color or alpha information, they are just the coordinates
    /// for the pixels that this trapezoid covers.
    fn into_pixels(&self) -> Vec<Pixel> {
        let mut outline_pixels = Vec::new();
        for line in self.lines() {
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
}

// Defines a collection for holding a Trapezoid's bases.
//
// A Trapezoid's base line segments are always parallel.
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
    // Returns the slope of the bases
    fn slope(&self) -> f32 {
        self.0.slope()
    }
}

/// Returns base line segments constructed from points.
///
/// A Trapezoid's base line segments are the parallel lines that form the Trapezoid.
fn bases_from_points(a: Point, b: Point, c: Point, d: Point) -> Vec<TrapezoidBasePair> {
    let mut points = vec![a, b, c, d];
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

// Returns a Vec<LineSegment> of the four lines that make up a Trapezoid with bases base1 and
// base2.
fn lines_from_bases(base1: LineSegment, base2: LineSegment) -> Vec<LineSegment> {
    if base1.slope() == f32::INFINITY {
        let top_leg = LineSegment::from_points(base1.highest_point(), base2.highest_point());
        let bottom_leg = LineSegment::from_points(base1.lowest_point(), base2.lowest_point());
        vec![bottom_leg, base1, top_leg, base2]
    } else {
        let left_leg = LineSegment::from_points(base1.min_x_point(), base2.min_x_point());
        let right_leg = LineSegment::from_points(base1.max_x_point(), base2.max_x_point());
        vec![base1, left_leg, base2, right_leg]
    }
}

#[derive(Debug)]
struct Pixel {
    x: i32,
    y: i32,
}

impl Pixel {
    /// Returns a Vec of Points whose coordinates are the points to be sampled for anti-aliasing.
    fn sample_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let x_increment = 1. / 16.;
        let y_increment = 1. / 14.;
        for subgrid_x in 0..17 {
            let x = self.x as f32 + (subgrid_x as f32 * x_increment);
            for subgrid_y in 0..15 {
                let y =  self.y as f32 + (subgrid_y as f32 * y_increment);
                let point = Point{x: x, y: y};
                points.push(point);
            }
        }

        points
    }
}

/// Returns true if a ray running along the positive x-axis intersects the line `line`.
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

/// Returns an ImageSurface mask from a Vec of Trapezoids.
///
/// The Rgba values will only have alpha values, as it is expected that this mask will only be
/// used with the `operator_in` operator.
pub fn mask_from_trapezoids(trapezoids: &Vec<Trapezoid>, width: usize, height: usize) -> ImageSurface {
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

            rgba.alpha += successes as f32 / 255.;
            rgba.alpha.max(1.);
         }
     }

     mask
}

#[cfg(test)]
mod tests {
    use super::{
        Trapezoid,
        TrapezoidBasePair,
        ray_from_point_crosses_line,
        mask_from_trapezoids,
        bases_from_points,
    };
    use common_geometry::{Point, LineSegment};

    // Test that you can construct a trapezoid from points
    #[test]
    fn trapezoid_from_points() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 0., y: 1.};
        let c = Point{x: 1., y: 0.};
        let d = Point{x: 1., y: 1.};
        let trap = Trapezoid::from_points(a, b, c, d);

        let (mut hasa, mut hasb, mut hasc, mut hasd) = (false, false, false, false);
        for line in trap.lines() {
            if line.point1 == a || line.point2 == a {
                hasa = true;
            }

            if line.point1 == b || line.point2 == b {
                hasb = true;
            }

            if line.point1 == c || line.point2 == c {
                hasc = true;
            }

            if line.point1 == d || line.point2 == d {
                hasd = true;
            }
        }

        assert!(hasa);
        assert!(hasb);
        assert!(hasc);
        assert!(hasd);
    }

    // Passes if bases_from_points returns the correct bases pairs
    #[test]
    fn test_bases_from_points() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 4., y: 0.};
        let c = Point{x: 2., y: 2.};
        let d = Point{x: 3., y: 2.};

        let base1 = LineSegment{point1: a, point2: b};
        let base2 = LineSegment{point1: c, point2: d};
        let pair = TrapezoidBasePair(base1, base2);
        let bases = bases_from_points(a, b, c, d);
        assert!(bases[0] == pair);
        assert!(bases.len() >= 1);
    }

    // Test that the ray_from_point_crosses_line function performs the 'crossings_test'
    #[test]
    fn crossings_test() {
        let p = Point{x: 1., y: 1.};
        let line = LineSegment::new(0., 0., 2., 2.);
        assert!(ray_from_point_crosses_line(&p, &line));
    }

    // Test the negative of the 'crossings test'.
    #[test]
    #[should_panic]
    fn crossings_test2() {
        let p = Point{x: 1., y: 1.};
        let line = LineSegment::new(2., 2., 3., 3.);
        assert!(ray_from_point_crosses_line(&p, &line));
    }

    // Test that the lines() method works for trapezoids with vertical bases.
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

    // Tests that the trapezoid lines() function returns the lines it should, and no more.
    #[test]
    fn trapezoid_rectangle_lines() {
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

    // Tests that the horizontal trapezoid's lines() function returns the lines it should, and
    // no more.
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

    // Test that the trapezoid contains_point() method returns true for a valid point.
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

    // Test that the bases() method returns the trapezoid's bases, and not its legs.
    #[test]
    fn trapezoid_bases() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 4., y: 0.};
        let c = Point{x: 2., y: 2.};
        let d = Point{x: 3., y: 2.};
        let bases = bases_from_points(a, b, c, d);

        let base1 = LineSegment::from_points(a, b);
        let base2 = LineSegment::from_points(c, d);
        let base_pair = TrapezoidBasePair(base1, base2);
        assert!(bases.contains(&base_pair));
    }

    // Check that the TrapezoidBasePair::slope() function returns the correct slope.
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

    // Tests that a sample of pixels internal to the trapezoid are at least somewhat opaque
    // (i.e., alpha > 0), and that a sampling of pixels external to the trapezoid are transparent
    // (i.e., alpha == 0).
    #[test]
    fn mask_from_single_trapezoid() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 10., y: 0.};
        let c = Point{x: 5., y: 9.};
        let d = Point{x: 7., y: 9.};
        let trap = Trapezoid::from_points(a, b, c, d);
        let trapezoids = vec![trap];
        let mask = mask_from_trapezoids(&trapezoids, 10, 10);

        // filled_pixels is the coordinates for pixels that should be filled (or somewhat opaque)
        let filled_pixels = vec![(2, 1), (8, 1), (5, 8), (7, 0)];
        for (x, y) in filled_pixels {
            let rgba = mask.get(x, y).unwrap();
            assert!(rgba.alpha > 0.);
        }

        // transparent_pixels is the coordinates for pixels that should be transparent
        let transparent_pixels = vec![(1, 9), (10, 2), (0, 2), (3, 9), (9, 9)];
        for (x, y) in transparent_pixels {
            let rgba = mask.get(x, y).unwrap();
            assert_eq!(rgba.alpha, 0.);
        }
    }

    /// Check that when two trapezoids share a line, that line gets rasterized when
    /// `fn mask_from_trapezoids` is called.
    #[test]
    fn adjacent_trapezoids_shared_line_is_opaque() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 5., y: 0.};

        let c = Point{x: 4., y: 3.};
        let d = Point{x: 2., y: 3.};
        let trap1 = Trapezoid::from_points(a, b, c, d);

        let trap2_point_e = Point{x: 0., y: 7.};
        let trap2_point_f = Point{x: 5., y: 7.};
        let trap2 = Trapezoid::from_points(d, c, trap2_point_f, trap2_point_e);

        let trapezoids = vec![trap1, trap2];
        let mask = mask_from_trapezoids(&trapezoids, 9, 9);

        let rgba = mask.get(2, 3).unwrap();
        assert_eq!(rgba.alpha, 1.);
        let rgba = mask.get(3, 3).unwrap();
        assert!(rgba.alpha > 0.9);
    }

    // Passes if a degenerate trapezoid (a triangle) functions correctly
    #[test]
    fn degenerate_trapezoid_works() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 4., y: 0.};
        let base1 = LineSegment{point1: a, point2: b};

        let c = Point{x: 3., y: 3.};
        let d = Point{x: 3., y: 3.};
        let base2 = LineSegment{point1: c, point2: d};

        let trapezoid = Trapezoid::from_bases(base1, base2);

        let internal_point = Point{x: 3., y: 2.};
        let external_point = Point{x: 2., y: 2.5};
        assert!(trapezoid.contains_point(&internal_point));
        assert!(!trapezoid.contains_point(&external_point));
    }

    #[test]
    #[should_panic]
    fn trap_from_bases_panics_on_non_parallel() {
        let a = Point{x: 0., y: 0.};
        let b = Point{x: 1., y: 1.};
        let base1 = LineSegment{point1: a, point2: b};

        let c = Point{x: 0., y: 0.};
        let d = Point{x: 1., y: 2.};
        let base2 = LineSegment{point1: c, point2: d};

        let _ = Trapezoid::from_bases(base1, base2);
    }
}
