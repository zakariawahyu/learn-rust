pub struct Circle {
    pub radius: i32,
}

impl crate::calculation_spec::Area for Circle {
    fn calculate_area(&self) -> f64 {
        // rumus PI * r kuadrat
        // ada operasi casting ke tipe f64 karena self.radius bertipe i32
        // method pow untuk operasi pangkat
        3.14 * self.radius.pow(2) as f64
    }
}

pub struct Square {
    pub length: i32,
}

impl crate::calculation_spec::Area for Square {
    fn calculate_area(&self) -> f64 {
        // rumus s x s
        // ada operasi casting ke tipe f64 karena self.length bertipe i32
        // method pow untuk operasi pangkat
        self.length.pow(2) as f64
    }
}