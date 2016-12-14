//! The main entrance point to the Cairus library.

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
