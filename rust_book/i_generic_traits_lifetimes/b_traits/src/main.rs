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
*/
use crate::calculation_spec::Area;

mod calculation_spec;
mod two_dimensional;

fn main() {
    //m ethod calculate_area() milik object bertipe Circle dan Square diakses untuk kemudian di-print
    let calculate_circle_one = two_dimensional::Circle{radius: 5};
    println!("circle area {}", calculate_circle_one.calculate_area());

    let calculate_square_one = two_dimensional::Square{length: 4};
    println!("square area {}", calculate_square_one.calculate_area());
}
