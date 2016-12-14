//! This module is for image compositing operations.
//!
//! Cairus currently supports the `over` compositing operation.


/// Rgba is the primary representation of color in Cairus.  Rgba is for API-level definition of
/// color, and is NOT the 32-bit (8-bit per channel) pixel representation found in bitmaps.
#[derive(Debug)]
pub struct Rgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}


impl Rgba {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Rgba {
        Rgba{red: red, green: green, blue: blue, alpha: alpha}
    }

    pub fn to_int(&self) -> (i32, i32, i32, i32) {
        ((self.red * 255.) as i32,  (self.green * 255.) as i32,
         (self.blue * 255.) as i32, (self.alpha * 255.) as i32)
    }
}


impl PartialEq for Rgba {
    fn eq(&self, other: &Rgba) -> bool {
        self.red == other.red && self.green == other.green &&
        self.blue == other.blue && self.alpha == other.alpha
    }
}


/// This enum will hold all types of supported operations.
#[allow(dead_code)]
pub enum Operator {
    Over,
}


/// Composite two Rgba types using the over operation.
///
/// This is cairus's default operator.  If the source is semi-transparent, the over operation will
/// blend the src and the destination.  If the source is opaque, it will cover the destination.
pub fn over(src: &Rgba, dst: &Rgba) -> Rgba {
    // Returns a new Rgba struct, the result of compositing `src` and `dst`.
    let new_alpha = src.alpha + (dst.alpha * (1. - src.alpha));
    let composite = | x: f32, y: f32 | -> f32 {
        ((x * src.alpha) + y * dst.alpha * (1. - src.alpha)) / new_alpha
    };

    Rgba {
          red: composite(src.red, dst.red),
        green: composite(src.green, dst.green),
         blue: composite(src.blue, dst.blue),
        alpha: new_alpha
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_over_operator_semi_transparent_src() {
        let src = Rgba::new(1., 0., 0., 0.5);
        let dst = Rgba::new(0., 1., 0., 0.5);
        let result = over(&src, &dst);

        // This result was computed manually to be correct, and then modified to match Rust's
        // default floating point decimal place rounding.
        assert_eq!(result, Rgba::new(0.6666667, 0.33333334, 0.0, 0.75));
    }

    #[test]
    fn test_over_operator_opaque_src() {
        let src = Rgba::new(1., 0., 0., 1.0);
        let dst = Rgba::new(0., 1., 1., 0.5);
        let result = over(&src, &dst);
        assert_eq!(result, Rgba::new(1., 0., 0., 1.0));
    }

    #[test]
    fn test_over_operator_opaque_dst() {
        let src = Rgba::new(0., 0., 1., 0.5);
        let dst = Rgba::new(0., 1., 0., 1.);
        let result = over(&src, &dst);
        assert_eq!(result, Rgba::new(0., 0.5, 0.5, 1.0));
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
}
