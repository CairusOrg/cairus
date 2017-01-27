use std::f32;

/// Returns the number of vertices in a polygon for apporiximating a pen tip of a certain width and
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

/// ### References
/// [1](https://keithp.com/~keithp/talks/cairo2003.pdf)

#[cfg(test)]
mod tests {
    use super::polygon_vertices;

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
}
