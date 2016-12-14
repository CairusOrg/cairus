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
}


impl PartialEq for Rgba {
    fn eq(&self, other: &Rgba) -> bool {
        self.red == other.red && self.green == other.green &&
        self.blue == other.blue && self.alpha == other.alpha
    }
}


/// Composite two Rgba types using the over operation.
///
/// This is cairus's default operator.
/// If the source is semi-transparent, the over operation will blend the src and
/// the destination.  If the source is opaque, it will cover the destination.
pub fn over(src: &Rgba, dst: &Rgba) -> Rgba {
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
    use Rgba;
    use over;
    #[test]
    fn over_operator_function() {
        let src = Rgba::new(1.,0., 0., 0.5);
        let dst = Rgba::new(0., 1., 0., 0.5);
        let result: Rgba = over(&src, &dst);
        assert_eq!(result, Rgba::new(0.6666667, 0.33333334, 0.0, 0.75));
    }
}
