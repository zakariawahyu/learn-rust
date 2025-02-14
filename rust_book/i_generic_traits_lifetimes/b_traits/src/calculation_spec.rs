pub trait Area {
    fn calculate_area(&self) -> f64;
}

pub trait Circumference {
    fn circumference(&self) -> f64;
}

// Associated type pada traits
// Pada trait Shape memiliki Associated type adalah Area yang didefinisikan dalam blok trait
// Tipe didefinisikan tanpa assigment operator, artinya tidak ada tipe data konkretnya
pub trait Shape {
    type Area;
    fn area(&self) -> Self::Area;
}