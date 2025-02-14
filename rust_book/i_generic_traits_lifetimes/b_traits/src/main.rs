/*
    Traits
    - Trait jika diartikan dalam bahasa indonesia adalah sifat
    - Di Rust, trait adalah definisi header method yang bisa di share ke banyak tipe data
    - Trait isinya hanya definisi header method (bisa diartikan method tanpa isi)
    - Ketika tipe data mengimplement suatu trait, maka tipe tersebut wajib menuliskan implamentasi
    sesuai dengan header method yang ada ditrait

    Rust menyediakan cukup banyak trait yang diimplement ke banyak tipe data, beberapa di antaranya:
    - Trait std::fmt::Debug, digunakan agar data bisa di print menggunakan formatted print {:?}
    - Trait std::iter::Enumerate, digunakan agar data bisa di iterasi menggunakan keyword for
    - Trait std::ops:Add, diimplementasikan agar data bisa digunakan pada operasi aritmatik penambahan +

    Jenis trait berdasakan tempat deklarasinya
    - External Trait (foreign trait)
    Yaitu trait yang tempat deklarasinya berada di luar crate kode yang ditulis. Misal trait std::fmt::Debug dan
    std::ops::Add, keduanya merupakan external trait yang berada di crate std atau crate Rust Standard Library

    - Local Trait
    Adalah trait yang di buat di crate yang berada dalam package/project yang sedang dikerjakan

    Trait sebagai tipe parameter
    - Manfaatnya adalah saat pemanggilan fungsi, parameter tersebut bisa di isi dengan argument data
    bertipe apapun dengan catatan, tipe dari data tersebut mengimplementasikan trait yang sama dengan yang
    digunakan pada parameter
    - Bisa juga trait parameter lebih dari 1, notasi penulisannya &(impl Trait1 + Trait2 + Trait3 + ...)

    Trait bound syntax
    - Penerapan trait sebagai parameter juga bisa dituliskan dalam notasi generic
    - notasi penulisannya <T: Trait1 + Trait2>nama_fungsi(parameter: T)
    - Jika tipenya menggunakan &T maka ekuivales dengan &(impl Trait1 + Trait2)
    - bisa juga menggunakan lebih dari 1 Generic type
    - maka notasinya seperti berikut <T: Trait1, U: Trait2>nama_fungsi(param1: T, param2: U)

    Trait where clause
    - Alternatif penulisan trait bound syntax, menggunakan keyword where
    - maka notasinya seperti berikut <T>nama_fungsi(param: T) where T: Trait1 + Trait2 {
    - atau <T, U>nama_fungsi(param: T, param2: U) where T: Trait1 + Trait2, U: Trait1 {

    Trait sebagai return type
    - Bisa juga digunakan return type dengan penulisan impl Trait1

    Associated Types Trait
    - Associated Types Trait adalah tipe data yang didefinisikan dalam suatu trait
    - Associated types tidak memiliki tipe data konkret saat didefinisikan
    - Namun ketika trait di implementasi maka tipe tersebut harus ditentukan tipe data konkretnya
*/
use crate::calculation_spec::{Area, Circumference, Shape};

mod calculation_spec;
mod two_dimensional;

// &impl Area ini tipe pointer, tipe non pointernya adalah impl Area
// menggunakan pointer untuk antisipasi move semantics
fn calculate_and_print_result(name : &str, item: &impl Area) {
    println!("{} area is : {}", name, item.calculate_area())
}
fn calculate_and_print(name : &str, item: &(impl Area + Circumference)) {
    println!("{} area is : {}", name, item.calculate_area());
    println!("{} circumference is {}", name, item.circumference());
}

fn trait_bound_syntax<T: Area + Circumference>(name: &str, item: &T) {
    println!("{} area is : {}", name, item.calculate_area());
    println!("{} circumference is {}", name, item.circumference());
}

fn two_bound_syntax<T: Area, U: Circumference>(name: &str, area: &T, circumference: &U) {
    println!("{} area is : {}", name, area.calculate_area());
    println!("{} circumference is {}", name, circumference.circumference());
}

fn where_bound_syntax<T>(name: &str, item: &T) where T:Area + Circumference {
    println!("{} area is : {}", name, item.calculate_area());
    println!("{} circumference is {}", name, item.circumference());
}

// return type
fn new_circle(radius: i32) -> impl Area {
    two_dimensional::Circle { radius }
}

fn new_square(length: i32) -> impl Area + Circumference{
    two_dimensional::Square { length }
}

fn main() {
    //method calculate_area() milik object bertipe Circle dan Square diakses untuk kemudian di-print
    let calculate_circle = two_dimensional::Circle{radius: 5};
    println!("circle area {}", calculate_circle.calculate_area());

    let calculate_square = two_dimensional::Square{length: 4};
    println!("square area {}", calculate_square.calculate_area());

    calculate_and_print_result("circle one", &calculate_circle);
    calculate_and_print_result("square one", &calculate_square);

    calculate_and_print("circle two", &calculate_circle);
    calculate_and_print("square two", &calculate_square);

    trait_bound_syntax("circle three", &calculate_circle);
    trait_bound_syntax("square three", &calculate_square);

    trait_bound_syntax("circle three", &calculate_circle);
    trait_bound_syntax("square three", &calculate_square);

    two_bound_syntax("circle four", &calculate_circle, &calculate_circle);
    two_bound_syntax("square four", &calculate_square, &calculate_square);

    where_bound_syntax("circle five", &calculate_circle);
    where_bound_syntax("square five", &calculate_square);

    // return trait
    let circle = new_circle(5);
    println!("data circle area is {}", circle.calculate_area());
    let square = new_square(10);
    where_bound_syntax("data square", &square);

    // associated type traits
    let obj1 = two_dimensional::Circle{radius:5};
    println!("area of circle: {}", obj1.area());
    let obj2 = two_dimensional::Square{length:4};
    println!("area of square: {}", obj2.area());
}
