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
 *  Bobby Eshleman <bobbyeshleman@gmail.com>
 *
 */

use std::f32;

struct Polygon {
    diameter: f32,
    vertices: i32,
}

impl Polygon {
    // Returns a new Polygon
    fn new(diameter: f32, vertices: i32) -> Polygon {
        Polygon {
            diameter: diameter,
            vertices: vertices,
        }
    }

    // Returns a new polygon based on the width and flatness of the pen it is approximating
    fn from_width_flatness(width: f32, flatness: f32) -> Polygon {
        Polygon {
            diameter: width,
            vertices: Polygon::polygon_vertices(width, flatness),
        }
    }


    /// Returns the number of vertices in a polygon for approximating a pen tip of a certain width and
    /// desired flatness.
    ///
    /// The lower the flatness, the more vertices the polygon will have.  This is because the more
    /// vertices a polygon has, the closer it is to a circle.
    /// The higher the flatness, the more 'square-like' the polygon will be.
    /// The fewest vertices possible is 4.
    ///
    /// See the Keith Packard paper for this equation [1].
    fn polygon_vertices(width: f32, flatness: f32) -> i32 {
        let result = (
            f32::consts::PI / (1.0 - ((2.0 * flatness) / width)).acos()
        ).ceil() as i32;

        match result % 2 == 0 {
            true if result > 4 => result,
            false if result > 4 => result + 1,
            _ => 4
        }
    }


    // Returns sum of each interior angle of a polygon of `vertices`
    //
    // Example:  A triangle (3 vertices) angle sum is 180 degrees
    // ```
    // assert_eq!(angel_sum(3), 180); // true
    // ```
    fn interior_angle_sum(&self) -> f32 {
        ((self.vertices - 2) * 180) as f32
    }

    // Returns the exterior angle of the polygon
    //
    // Since the exterior angle of all vertices always adds up to 360, each individual angle is
    // equal to `360 / vertices`
    fn exterior_angle(&self) -> f32 {
        (360 / self.vertices) as f32
    }

    // Returns the side length of this polygon
    fn side_length(&self) -> f32 {
        let radius = self.diameter / 2.;
        2.0 * radius *  (180.0 / self.vertices as f32).to_radians().sin()
    }
}


/// ### References
/// [1](https://keithp.com/~keithp/talks/cairo2003.pdf)

#[cfg(test)]
mod tests {
    use super::Polygon;
    #[test]
    fn polygon_edge_count1() {
        let width = 3.0;
        let flatness = 0.01;
        assert_eq!(Polygon::polygon_vertices(width, flatness), 28);
    }

    #[test]
    fn polygon_edge_count2() {
        let width = 2.0;
        let flatness = 1.0;
        assert_eq!(Polygon::polygon_vertices(width, flatness), 4);
    }

    #[test]
    fn polygon_edge_count3() {
        let width = 5.0;
        let flatness = 0.0001;
        assert_eq!(Polygon::polygon_vertices(width, flatness), 352);
    }

    #[test]
    fn polygon_exterior_angle() {
        let diameter = 2.0;
        let vertices = Polygon::polygon_vertices(diameter, 0.1);
        let polygon = Polygon::new(diameter, vertices);
        assert_eq!(polygon.exterior_angle(), 45.);
    }

    #[test]
    fn polygon_constructor_init_correct_angle() {
        let diameter = 2.0;
        let vertices = Polygon::polygon_vertices(diameter, 0.1);
        let polygon = Polygon::new(diameter, vertices);
        assert_eq!(polygon.interior_angle_sum(), 1080.);
    }

    #[test]
    fn polygon_from_width_flatness() {
        let polygon = Polygon::from_width_flatness(2., 0.1);
        assert_eq!(polygon.exterior_angle(), 45.);
    }

    #[test]
    fn polygon_side_length() {
        let polygon = Polygon::new(2., 4);
        assert_eq!(polygon.side_length(), 1.4142135)
    }
}
