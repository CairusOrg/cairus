use std::f32;

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

struct Polygon {
    empty: ()
}

impl Polygon {
    // Returns sum of each angle of a polygon of `vertices`
    //
    // Example:  A triangle (3 vertices) angle sum is 180 degrees
    // ```
    // assert_eq!(angel_sum(3), 180); // true
    // ```
    fn angle_sum(vertices: i32) -> f32 {
        180. + (180. * ((vertices as f32) - 3.))
    }
}


/// ### References
/// [1](https://keithp.com/~keithp/talks/cairo2003.pdf)

#[cfg(test)]
mod tests {
    use super::polygon_vertices;
    use super::Polygon;

    #[test]
    fn test_polygon_edge_count1() {
        let width = 3.0;
        let flatness = 0.01;
        assert_eq!(polygon_vertices(width, flatness), 28);
    }

    #[test]
    fn test_polygon_edge_count2() {
        let width = 2.0;
        let flatness = 1.0;
        assert_eq!(polygon_vertices(width, flatness), 4);
    }

    #[test]
    fn test_polygon_edge_count3() {
        let width = 5.0;
        let flatness = 0.0001;
        assert_eq!(polygon_vertices(width, flatness), 352);
    }

    #[test]
    fn test_polygon_angle(){
        //let diameter = 2.0;
        //let vertices = polygon_vertices(diameter, 0.1);
        //let polygon = Polygon::new(diameter, vertices);
        assert_eq!(Polygon::angle_sum(8), 1080.);
        assert_eq!(Polygon::angle_sum(5), 540.);
        assert_eq!(Polygon::angle_sum(3), 180.);

    }

    #[test]
    fn test_polygon_angle() {
        let diameter = 2.0;
        let vertices = polygon_vertices(diameter, 0.1);
        let polygon = Polygon::new(diameter, vertices);
        assert_eq!(polygon.angle, 1080.);
    }

}
