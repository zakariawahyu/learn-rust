/*
    Struct
    - Struct adalah tipe data custom yang dengannya kita bisa mengumpulkan beberapa definisi tipe data lalu menjadikannya
      sebagai satu buah tipe data dalam struktur tertentu
    - Struct merupakan tipe data cutom, yang berati tipe data tersebut bisa digunakan untuk pembuatan variabel
    - Aturan penamaan struct menggunakan Upper Camel Case

    #[derive(Debug)]
    - Secara default, error akan muncul ketika berusaha menampilkan nilai variabel struct (bukan nilai propertynya)
    - Error ini disebabkan karena data yang bisa di tampilkan melalui macro println! harus memiliki trait Debug

    Tuple Struct
    - Tuple struct adalah struct yang didefinisikan dengan gaya tuple
    - Property pada tuple bisa di akses menggunakan notasi pengaksesan tuple

    Struct property visibility
    - Struct jika didefinisikan di file yang sama dengan statement pemanggilan struct tersebut tidak menghasilkan error
    - Tetapi jika didefinisikan di file yang terpisah, maka propertynya harus juga publik
    - cara membuat field/poperty publik dengan menambahkan keywoard 'pub'
*/
mod models;

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Car {
    brand: String,
    model: String,
}

fn main() {
    let user_one = User {
        name: String::from("Zakaria"),
        email: String::from("zakaria@gmail.com"),
        sign_in_count: 1,
        active: false
    };

    println!("name: {}", user_one.name);
    println!("email: {:?}", user_one.email);
    println!("sign-in count: {}", user_one.sign_in_count);
    println!("is active : {}", user_one.active);

    // mutable struct
    let mut user_two = User {
        name: String::from("Wahyu"),
        email: String::from("wahyu@gmail.com"),
        ..user_one // syntax untuk mengambil data yang berasal dari data struct lain
        // dibawah sini sign_in_count dan active terisi otomatis dari user_one
    };

    user_two.name = String::from("Wahyu Nur");
    user_two.email = String::from("wahyu01@gmail.com");

    println!("=== Mutable Struct ===");
    println!("name: {}", user_one.name);
    println!("email: {:?}", user_one.email);
    println!("sign-in count: {}", user_one.sign_in_count);
    println!("is active : {}", user_one.active);

    // deklatarasi melalui function
    let car_one = new_car(
        String::from("Mitsubishi"),
        String::from("Pajero Sport")
    );

    // deklarasi secara horizontal
    let car_two = Car{brand: car_one.brand, model: car_one.model};

    // Destructuring assignment
    // Teknik penulisan ini bisa dipakai dalam case dimana nilai property struct perlu ditampung ke variabel baru.
    let Car{brand: b, model: m} = car_two;
    println!("brand: {}", b);
    println!("model: {}", m);

    // debug
    println!("data_struct_one: {:#?}", user_one);

    // tuple struct
    struct Color(i32, i32, i32, bool);
    let red = Color(255, 0, 0, true);
    println!("{:?} {:?} {:?} {:?}", red.0, red.1, red.2, red.3);

    // property visibility
    let ps5 = models::games::GamingConsole{
        name: String::from("PS 5")
    };
    println!("{:#?}", ps5);

    let shirt = models::games::Shirt(String::from("XL"), true);
    println!("{:#?}", shirt);
}

fn new_car(brand: String, model: String) -> Car {
    Car {
        brand,
        model,
    }
}