//! This module is for image compositing operations.
//!
//! Cairus currently supports the `over` compositing operation.
use cairus::Rgba;


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
    use cairus::Rgba;

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

}
