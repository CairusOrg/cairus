//! This module is for image compositing operations.
//!
//! Cairus currently only supports the `over` compositing operation.

/// This enum will hold all types of supported operations.
#[allow(dead_code)]
pub enum Operator {
    Over,
}

/// Rgba is the primary representation of color in Cairus.
#[derive(Debug)]
pub struct Rgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Rgba {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Rgba {
        // Returns a new Rgba.  Initialized Rgba's call correctify() by default.
        let mut result = Rgba {red: red, green: green, blue: blue, alpha: alpha};
        result.correctify();
        result
    }

    pub fn to_int(&self) -> (i32, i32, i32, i32) {
        ((self.red * 255.) as i32,  (self.green * 255.) as i32,
         (self.blue * 255.) as i32, (self.alpha * 255.) as i32)
    }

    pub fn correctify(&mut self) {
        // Any value greater than 1.0 resets to 1.0, any value lower than 0.0 resets to 0.0
        self.red = self.red.min(1.).max(0.);
        self.green = self.green.min(1.).max(0.);
        self.blue = self.blue.min(1.).max(0.);
        self.alpha = self.alpha.min(1.).max(0.);
    }
}

impl PartialEq for Rgba {
    fn eq(&self, other: &Rgba) -> bool {
        self.red == other.red && self.green == other.green &&
        self.blue == other.blue && self.alpha == other.alpha
    }
}

/// Returns a new Rgba struct, the result of compositing `src` and `dst`.
///
/// # Arguments
/// * `src` - A reference to the source's Rgba
/// * `dst` - A referece to the destination's Rgba
///
/// This is cairus's default operator.  If the source is semi-transparent, the over operation will
/// blend the src and the destination.  If the source is opaque, it will cover the destination with
/// no blending.
pub fn over(src: &Rgba, dst: &Rgba) -> Rgba {
    let alpha = over_alpha(src.alpha, dst.alpha);
    Rgba::new(
        over_color(src.red, dst.red, src.alpha, dst.alpha, alpha),
        over_color(src.green, dst.green, src.alpha, dst.alpha, alpha),
        over_color(src.blue, dst.blue, src.alpha, dst.alpha, alpha),
        alpha
    )
}

fn over_color(x: f32, y: f32, src_alpha: f32, dst_alpha: f32, new_alpha: f32) -> f32 {
    ((x * src_alpha) + y * dst_alpha * (1. - src_alpha)) / new_alpha
}

fn over_alpha(src: f32, dst: f32) -> f32 {
    src + (dst * (1. - src))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_over_operator_semi_transparent_src() {
        let src = Rgba::new(1., 0., 0., 0.5);
        let dst = Rgba::new(0., 1., 0., 0.5);
        // This result was computed manually to be correct, and then modified to match Rust's
        // default floating point decimal place rounding.
        assert_eq!(over(&src, &dst), Rgba::new(0.6666667, 0.33333334, 0.0, 0.75));
    }

    #[test]
    fn test_over_operator_opaque_src() {
        let src = Rgba::new(1., 0., 0., 1.0);
        let dst = Rgba::new(0., 1., 1., 0.5);
        over(&src, &dst);
        assert_eq!(over(&src, &dst), Rgba::new(1., 0., 0., 1.0));
    }

    #[test]
    fn test_over_operator_opaque_dst() {
        let src = Rgba::new(0., 0., 1., 0.5);
        let dst = Rgba::new(0., 1., 0., 1.);
        assert_eq!(over(&src, &dst), Rgba::new(0., 0.5, 0.5, 1.0));
    }

    #[test]
    fn test_over_operator_too_large() {
        let src = Rgba{red: 3.0, green: 3.0, blue: 3.0, alpha: 3.0};
        let dst = Rgba::new(0., 1., 0., 1.);
        assert_eq!(over(&src, &dst), Rgba::new(1., 1., 1., 1.));
    }

    #[test]
    fn test_rgba_to_int_all_ones() {
        let color = Rgba::new(1., 1., 1., 1.);
        assert_eq!(color.to_int(), (255, 255, 255, 255));
    }

    #[test]
    fn test_rgba_to_int_all_zeroes() {
        let color = Rgba::new(0., 0., 0., 0.);
        assert_eq!(color.to_int(), (0, 0, 0, 0));
    }

    #[test]
    fn test_rgba_to_int_all_half() {
        let color = Rgba::new(0.5, 0.5, 0.5, 0.5);
        assert_eq!(color.to_int(), (127, 127, 127, 127));
    }

    #[test]
    fn test_rgba_corrects_large_values() {
        let color = Rgba::new(3., 3., 3., 3.);
        assert_eq!(color, Rgba::new(1., 1., 1., 1.));
    }

    #[test]
    fn test_rgba_corrects_small_values() {
        let color = Rgba::new(-3., -3., -3., -3.);
        assert_eq!(color, Rgba::new(0., 0., 0., 0.));
    }

    #[test]
    fn test_rgba_vector() {
        // This test demonstrates the use case of having a 2D vector of RGBAs, similar to how a
        // surface might be.

        // Create a 2D vector
        let width = 10;
        let height = 20;
        let src = Rgba::new(0., 0., 1., 0.5);
        let mut dst = Vec::with_capacity(height);

        // Construct 10x20 matrix of RGBAs
        for h in 0..height {
            dst.push(Vec::with_capacity(width));
            for _ in 0..width {
                dst[h].push(Rgba::new(0., 1., 0., 1.));
            }
        }

        // Create a new matrix with the source applied to the destination
        let mut new_vector = Vec::new();

        let expected = Rgba::new(0., 0.5, 0.5, 1.0);
        for row in dst.iter() {
            new_vector.push(Vec::with_capacity(width));
            for (idx, col) in row.iter().enumerate() {
                let new = over(&src, &col);
                new_vector[idx].push(new);
            }
        }


        for row in new_vector.iter() {
            for col in row.iter() {
                assert_eq!(col.red, expected.red);
                assert_eq!(col.blue, expected.blue);
                assert_eq!(col.green, expected.green);
                assert_eq!(col.alpha, expected.alpha);
            }

        }
    }
}
