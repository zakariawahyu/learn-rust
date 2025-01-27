/*
    Pattern Match
    - Tipe data enum secara defaultnya tidak bisa dipakai untuk kondisi menggunakan if
    - Match adalah salah satu keyword untuk operasi seleksi pada Rust, penerapanya bisa digunakan untuk enum
    - Semua kondisi yang memungkinkan harus wajib ditulis, harus lengkap. Ibarat if yang harus ada block else-nya. Jika tidak, maka akan error
    - bisa menggunakan _ => atau jika tidak ingin menjalankan apa" pada akhir bisa menggunakan _ => ()
    - Penulisan kondisi default ini harus berada di setiap akhir match
 */

enum Food {
    AyamGoreng,
    PecelLele,
    MieSetan{level :i32, bungkus : bool},
    MakananLainnya(String)
}

fn get_food(food:Food) {
    match food {
        Food::AyamGoreng => {
            println!("Thats good");
        },
        Food::MieSetan{level, bungkus} => {
            if level > 3 {
                println!("mie setan lvl {} is too much!", level);
            }

            if !bungkus {
                println!("how are you going to eat right now?");
            }
        },
        Food::MakananLainnya(m) => {
            println!("do you like {m}? nice taste!");
        },
        _ => {
            println!("never heard about that food?");
        }
    }
}

fn main() {
    let pecel_lele = Food::PecelLele;
    get_food(pecel_lele);

    get_food(Food::AyamGoreng);
    get_food(Food::MakananLainnya(String::from("Sate")));
    get_food(Food::MieSetan{level:5, bungkus:false});

    // ekuivalen menggunakan if else
    let time = "morning";
    match time {
        "morning"   => println!("Selamat pagi"),
        "afternoon" => println!("Selamat siang"),
        _           => println!("I don't know what to do"),
    }

    // bisa juga di tampung dalam variable
    let _time_but_in_javanese = match time {
        "morning"   => "isuk",
        _           => "mbuh kapan",
    };

    let x = plus_one(Some(1));
    println!("Plus one {:?}", x);
    let none = plus_one(None);
    println!("Plus one {:?}", none);
}

// match juga bisa digunakan untuk tipe Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Operator | digunakan sebagai logika OR
// Operator .. atau ..= digunakan sebagai logika IN
fn pattern_matching(value : i32) {
    match value {
        1 | 2   => println!("one or two"),
        3..=4   => println!("three to four"),
        6       => println!("six"),
        _       => println!("other number"),
    }
}

// Match guard adalah teknik menambahkan sub seleksi kondisi pada klausul match. Contoh:
fn match_guard(value: Option<i32>) {
    match value {
        Some(x) if x % 2 == 0 => println!("number {} is even", x),
        Some(x)               => println!("number {} is odd", x),
        None => println!("no number"),
    }
}

// Operator @ digunakan untuk menampung nilai klausul match yang default-nya tidak bisa ditampung
// Khusus untuk penggunaan @ binding pada operator |, pada penulisan klausul seleksi kondisinya harus diapit tanda ().
fn binding(value : i32) {
    match value {
        value @ (1 | 2) => println!("one or two ({})", value),
        value @ 3..=5   => println!("three through five ({})", value),
        6   => println!("six"),
        _   => println!("other number"),
    }
}

// Operasi destructuring (menampung item suatu tipe) bisa dilakukan menggunakan pattern matching.
#[test]
fn destructuring() {
    struct Point {
        x: i32,
        y: i32
    }

    let origin = Point { x: 0, y: 7  };
    match origin {
        Point { x, y:0 } => println!("x axis at {x}"),
        Point{x :0 ,y} => println!("y axis at {y}"),
        Point{x, y} => println!("axis at {x} = {y}"),
    }
}

#[test]
fn enum_destructuring() {
    enum Color {
        Black,
        White,
        RGB(u32, u32, u32)
    }

    let rgb_color = Color::RGB(122, 17, 40);

    if let Color::RGB(r, g, b) = rgb_color  {
        println!("r = {}, g = {}, b = {}", r, g, b);
    }
    
    match rgb_color {
        Color::RGB(r, g, b) => println!("r = {}, g = {}, b = {}", r, g, b),
        _ => ()
    }
}

#[test]
fn tuple_destructuring() {
    let grades = ("A", "B", "C");

    if let (grade_a, grade_b, grade_c) = grades {
        println!("Grade ({}, {}, {})", grade_a, grade_b, grade_c);
    }

    // Variabel _ bisa dimanfaatkan pada statement destructuring untuk menampung item yang tidak digunakan.
    match grades {
        (grade_a, _, grade_c) => {
            println!("Grade ({}, {})", grade_a, grade_c);
        }
    }
}

// Operator .. bisa digunakan untuk meng-exclude item dalam range tertentu.
// Sebagai contoh, tuple numbers di-destructure dan hanya diambil nilai elemen ke-1 dan terakhirnya.
#[test]
fn test_exclude() {
    let numbers = (2, 4, 8, 16, 32);

    let (first, .., last) = numbers;
    println!("first number: {first}");
    println!("last number: {last}");

    let (first, ..) = numbers;
    println!("first number: {first}");

    let (.., last) = numbers;
    println!("last number: {last}");
}