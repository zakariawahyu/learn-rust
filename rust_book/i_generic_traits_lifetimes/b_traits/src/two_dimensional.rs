use crate::calculation_spec::{Area, Circumference, Shape};
pub struct Circle {
    pub radius: i32,
}

impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        // rumus PI * r kuadrat
        // ada operasi casting ke tipe f64 karena self.radius bertipe i32
        // method pow untuk operasi pangkat
        3.14 * self.radius.pow(2) as f64
    }
}

impl Shape for Circle {
    type Area = f64;
    fn area(&self) -> Self::Area {
        3.14 * self.radius.pow(2) as f64
    }
}

impl Circumference for Circle {
    fn circumference(&self) -> f64 {
        // rumus 2 * PI * r
        2.0 * 3.14 * self.radius as f64
    }
}

pub struct Square {
    pub length: i32,
}

impl Area for Square {
    fn calculate_area(&self) -> f64 {
        // rumus s x s
        // ada operasi casting ke tipe f64 karena self.length bertipe i32
        // method pow untuk operasi pangkat
        self.length.pow(2) as f64
    }
}

impl Circumference for Square {
    fn circumference(&self) -> f64 {
        // rumus 4 * s
        4.0 * self.length as f64
    }
}

impl Shape for Square {
    type Area = i64;
    fn area(&self) -> Self::Area {
        4 * self.length as i64
    }
}