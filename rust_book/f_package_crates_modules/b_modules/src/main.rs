/*
    Rust module system
    - Di Rust, module memiliki hierarki atau biasa disebut module tree yang root akarnya adalah file entrypoint
    crate yaitu main.rs untuk binary create dan lib.rs untuk library crate
    - Rust memiliki 2 jenis module, yaitu normal module dan inline module
    - Keyword mod digunakan untuk mendefinisikan/mendaftarkan sebuah module
    - Nama module menjadi path dimana isi module atau module item itu berada
    - Penulisan nama module harus menggunakan snake_case

    Normal Module
    - Module yang didefinisikan dengan nama my_number.rs, maka itemnya harus berada dalam my_number.rs atau my_number/mod.rs
    - Nama module ditulis di main.rs atau lib.rs (untuk library crate)

    Submodule
    - Sebuah module bisa saja memiliki module dibawahnya (biasa disebut submodule)
    - Di rust, aturan dalam pembuatan submodule masih sama seperti module, perbedaanya adalah
    tempat dimana submodule didefinisikan
    - Jika root module didefinisikan pada main.rs atau lib.rs, maka submodule didefinisikan pada file dimana parent module berada

    Note Submodule:
    - Penerapan notasi penulisan nama_module.rs biasanya dalam case ketika module tersebut tidak memiliki submodule.
    - Untuk module yang memiliki submodule, parent module harus menerapkan notasi penulisan nama_module/mod.rs,
    hal ini karena pendefinisian submodule berada pada file mod.rs dalam parent module tersebut.
 */

// definisi module my_io dari my_io.rs
// file harus sejajar dengan main.rs
mod my_io;

// definisi module my_number dari my_number/mod.rs
mod my_number;

fn main() {
    println!("Enter any number");
    let message = my_io::read_entry();
    println!("The number is {}", message);

    let number = my_number::conversion_utility::string_to_number(message);
    let odd = my_number::is_odd_number(number);
    let even = my_number::check_number::is_even_number(number);

    println!("The odd number : {}", odd);
    println!("The even number : {}", even);
}
