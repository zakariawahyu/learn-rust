#[derive(Debug)]
// membuat value struct pub agar bisa di akses color::Color()
pub struct Color(pub i32, pub i32, pub i32);

impl Color {
    // atau membuat associate functuin untuk membuat constructornya
    pub fn new(r:i32, g: i32, b: i32) -> Self {
        Self (r,g,b)
    }

    pub fn red() -> Self {
        Self(255, 0, 0)
    }

    pub fn green() -> Self {
        Self(0, 255, 0)
    }

    pub fn blue() -> Self {
        Self(0, 0, 255)
    }
}