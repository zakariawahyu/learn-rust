fn main() {
    /* Function Parameter
    - Definisi function dalam programming secara terminologi adalah sebuah modul atau sub-program kecil yang
    digunakan untuk mengeksekusi sebuah perintah dan bisa di re-use dalam penggunaanya
    - Penulisan nama function sama halnya variabel, meggunakan snake case
    - Function bisa didefinisikan dengan disertai parameter, dengan itu bisa menyisipkan data saat pemanggilan fungsi */
    great_custom_message("Jhon", 24, "Hello");

    /* Function Return Value
    - Caranya dengan menambahkan tanda '->' diikuti tipe data dari nilai returnnya
    - Cara penulisan nilai bisa menggunakan keyword 'return' ataupun tidak sama sekali
    -
    */
    let wide_square = calculate_square(5);
    println!("Calculate Square : {wide_square}");

    let wide_rectangle = calculate_rectangle(8 ,5);
    println!("Calculate Rectangle : {wide_rectangle}");

    println!("{}", get_score_message());

    // untuk function yang tidak didefinisikan nilai returnya, maka secara default nilainya adalah tuple kosong
    // contoh great_custom_message tidak memiliki nilai return, coba tampung nilai returnya dalam variabel
    let default = great_custom_message("Dhoe", 31, "Hai");
    println!("Result default : {:?}", default)
}

fn great_custom_message(name: &str, age:i32, message: &str) {
    println!("hi {name}, your age is {age}, and you say {message}")
}

fn calculate_square(width :i32) -> i32 {
    let wide:i32 = width*width;
    return wide
}

fn calculate_rectangle(lengt: i8, width:i8) -> i8 {
    let wide = lengt*width;
    // Statement terakhir sebuah blok kode fungsi yang ditulis tanpa semicolon ; disebut sebagai tail atau body tail
    wide
}

// return value bertipe string literal &str
// jika digunakan sebagai tipe data return value fungsi harus ditambahi keyword static dengan penulisan &'static str
fn get_score_message() -> &'static str {
    return "you got a perfect score!"
}