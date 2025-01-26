mod movie;
mod color;
/*
    Associated function
    - Associated item adalah item yang memiliki asosiasi/hubungan dengan struct. Item disini bisa banyak hal, berupa fungsi atau lainnya
    - Fungsi yang berhubungan dengan struct/trait disebut dengan associated function
    - Harus di deklarasi didalam block impl dan cara pengaminnal NamaStruct::nama_fungsi
    - Name fungsi pada associated function harus snack case

    Tipe data Self
    - Tipe data Self merupakan representasi untuk tipe data struct di mana blok impl di deklarasikan
    - Hanya bisa digunakan didalam block impl
 */

#[derive(Debug)]
struct LegoSet {
    code: i32,
    name: String,
    category: String,
    age_minimum: i32
}

impl LegoSet {
    fn new(code: i32, name: String, category: String, age_minimum: i32) -> LegoSet {
        LegoSet {code, name, category, age_minimum}
    }

    fn what_is_lego() {
        println!("Lego is line of plastic construction toys");
    }

    fn new_self(code: i32, name: String, category: String, age_minimum: i32) -> Self {
        Self{code, name, category, age_minimum}
    }
}

fn main() {
    let lego_set : LegoSet = LegoSet::new(1, String::from("Lego"), String::from("B"), 30);
    println!("{:#?}", lego_set);

    LegoSet::what_is_lego();

    let self_lego_set : LegoSet = LegoSet::new(2, String::from("Lego Self"), String::from("B"), 30);
    println!("{:#?}", self_lego_set);

    let movie : movie::Movie = movie::Movie::new(String::from("Star War"), String::from("Movie"), 4.3);
    println!("{:#?}", movie);

    let red = color::Color::red();
    let green: color::Color = color::Color::green();
    let blue: color::Color = color::Color::blue();
    println!("{:#?} {:#?} {:#?}", red, green, blue);

    let random_color = color::Color(12, 54, 43);
    println!("{:#?}", random_color);
}
