/*
    Enum
    - Enumerated type adalah sebuah tipe data yang digunakan untuk menampung nilai konstan
    - Enum ada kemiripan dengan variabel konstanta, bedanya pada nilai atau underlaying value-nya
    - Jika konstanta didefinisikan nama beserta valuenya, tapi di enum yang didefinisikan tipe data enum dan enum value
    - Enum value ini bentuknya seperti variabel tanpa nilai
    - Notasi path digunakan dalam penulisan enum value dengan format NamaEnum::EnumValue
    - Penamaan Enum beserta Valuenya menggunakan Upper Camel Case

    Enum Value - Tuple
    - Enum value bisa di desain seperti tuple struct

    Enum Value - Struct
    - Enum value juga bisa di desain memiliki property seperti struct

    Enum option
    - Option adalah salah satu tipe data yang digunakan untuk menampung data yang isinya bisa berpotensi kosong(None)
    - Tipe option dibagi menjadi 2 yaitu Option::Some yang mempunyai nilai dan Option::none yang tidak ada nilainya
 */

// NamaEnum adalah tipe data custom yang didefinisikan bertipe enum
// Sedangkan NilaiEnum1, Nilai2, NilaiEnumKe3 adalah yang disebut enum value
// Dengan kata lain ketika enum value tersebut bertipe data yang sama yaitu NamaEnum
enum NamaEnum {
    NilaiEnum1,
    Nilai2,
    NilaiEnumKe3,
    // . .
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Contoh enum vaue berisi tuple dan struct
// dengan menggunakan enum lebih ringkas
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum di atas sama halnya dengan struct di bawah ini
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

fn main() {
    // Pembuatan konstanta, tipe data berserta valuenya harus di tentukan di awal
    // Pada enum yang perlu didefinisikan adalah tipe data Enumnya kemudian diikuti dengan enum valuenya tanpa pengisian nilai

    // Definisi konstanta
    const SUPERHERO_SUPERMAN : &str = "superman";
    const SUPERHERO_BATMAN : &str = "batman";

    // Definisi enum
    enum SuperHero {
        Superman,
        Batman,
    }

    let _superman = SUPERHERO_SUPERMAN;
    let _bataman = SuperHero::Batman;

    let _four = IpAddrKind::V4;
    let _six :IpAddrKind = IpAddrKind::V6;

    let _home :IpAddr = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _write_msg = Message::Write(String::from("hello"));
    let _change_color_msg = Message::ChangeColor(0, 255, 255);
    let _quit_msg = Message::Quit;
    let _move_msg = Message::Move { x: 10, y: 20 };

    let divider1 = divider(10, 5);
    println!("{:?}", divider1);

    let divider2 = divider(10,0);
    println!("{:?}", divider2);
}

// Fungsi divider nilai baliknya bertipe Option<i32>
// Dari tipe data option yang digunakan nantinya bisa diprediksi pasti akan ada 2 potensi value
fn divider(a :i32, b :i32) -> Option<i32> {
    if b == 0 {
        return None;
    }

    let result = a / b;
    Some(result)
}
