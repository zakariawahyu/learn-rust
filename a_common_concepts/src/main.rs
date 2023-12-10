fn main() {
    /*
     1. Variable
    - Penamaan variabel menggunakan snake case
    - Secara default semua variabel adalah immutable, artinya tidak bisa diubah valuenya
    - Cara merubah menjadi mutable => tambahkan mut agar variabel dapat diubah valunya
    - Rust mendukung dua metode deklarasi variabel, yaitu inference dan manifest
     */

    let mut message_number = 1;
    let message1 = "Hello";
    println!("Message number {}: {}", message_number, message1);

    message_number = 2;
    let message2 = "World";
    println!("Message number {}: {}", message_number, message2);

    message_number = 3;
    let message3 = 4;
    // {0} pemanggilan argument yang ke 1, {1} berati argument yang ke 2
    println!("Message number {1}: {0}", message_number, message3);

    let _num1 = 1; // deklarasi inference, tidak ditulis tipe datanya
    let _num2: i8 = 2; // deklarasi manifest, ditulis secara jelas tipe datanya

    // penulisan variabel juga boleh dipisan seperti ini
    let _number3: i16;
    _number3 = 3;

    // deklarasi banyak variabel, inference
    let (_num1, _num2, mut _num3) = (1, 2, 3);

    // deklarasi banyak variabel, manifest
    let (_num4, mut _num5, _num6): (i8, i16, i32) = (4, 5, 6);

    // deklatasi variabel dengan tipe data di tentukan dari value
    let _num7 = 7i64;
}
