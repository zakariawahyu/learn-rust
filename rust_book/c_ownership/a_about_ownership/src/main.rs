/*
- Ownership adalah kumpulan aturan yang ada pada rust yang dijadikan acuan oleh compiler dalam pengelolaan memory
- Aturan Ownership :
    - Semua nilai/data/value do rust memiliki owner
    - Pada waktu yang sama, hanya boleh ada 1 owner. Satu data, ownernya hanya satu
    - Ketika eksekusi sebuah block scope selesai, maka owner dari data-data yang ada dalam scope tersebut akan di drop atau di dealokasi

- Variabel scope adalah block scope dimana suatu variabel di deklarasikan, dan dalam block scope tersebut variabel menjadi valid(bisa digunakan),
  di luar scopenya variabel menjadi tidak valid, tidak bisa digunakan

- Semua data tipe primitif (integer, bool, floating, char) secara default mengandung copy semantics
- Copy semantics pada rust disimpan dalam stack dan pengaksesannya cepat
- Ketika terjadi proses copy, datanya akan di copy sebagai data baru, dengan owner baru dan terjadi alokasi alamat memory baru

- Untuk tipe data non primitif, akan terjadi move semantic
- Akan terjadi pemindahan ownership
*/
fn do_someting(){
    let _data_one = "one";
    // disini variabel data_one hanya valid dalam scope do_something
}

fn main() {
    //VARIABLE SCOPE
    let _data_two = "two";
    // disini variabel data_two valid dalam scope main
    do_someting();
    {
        let _data_three = "three";
        // disini variabel data_three hanya valid di dalam scope ini saja, tidak di scope main
    }
    if true {
        let _data_four = "four";
        // disini variabel data_four hanya valid di dalam scope kondisi, tidak di scope main
    }

    //COPY SEMANTICS
    let x = 24;
    let y = x;// terjadi proses copy traits
    println!("x: {x}, y: {y}");

    //MOVE SEMANTICS
    let a = String::from("Hello world");
    // let b = a; // akan terjadi pemindahan owner dari a ke b
    // jika variable a dipanggil maka akan error, karena a tidak mempunyai owner lagi setelah berpindah ke b
    let b = a.clone(); // untuk tipe data non primitiv bisa menggunakan clone
    println!("a: {:?}, b: {:?}", a, b);

    //OWNER AND FUNCTION
    let s = String::from("Rust");
    // disini variabel s masih valid karena belum di pindahkan ke function lain
    takes_ownership(s); // s move ke dalam function
    // disini s tidak valid lagi, maka akan error jika dipanggil

    let i = 5;
    // disini variabel i valid
    makes_copy(i); // i move ke dalam function
    // disini i masih valid karena yang terjadi adalah Copy Trait (variabel primitif)


    //RETURN VALUES AND SCOPE
    let s1 = gives_ownership(); // memberikan ownership ke variabel dengan return value
    println!("{s1}");

    let s2 = String::from("Ownershiop");
    let s3 = takes_and_gives_back(s2); // s2 move to function
    // disini s2 tidak valid lagi karena owner sudah berpindah ke function
    // s3 valid karena diberikan ownership dari return value
    println!("{s3}");

    let (s4, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s4, len);
}

fn takes_ownership(s: String) {
    println!("{s}");
    // disini variabel s valid karena telah dipindahkan ke function ini
}

fn makes_copy(i: i32) {
    println!("{i}");
    // disini variabel i valid karena telah dipindahkan ke function ini
}

fn gives_ownership() -> String {
    String::from("Hello Rust")
}

fn takes_and_gives_back(s: String) -> String{
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}