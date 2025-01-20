fn main() {
    /*
    1. Variable
    - Penamaan variabel menggunakan snake case
    - Secara default semua variabel adalah immutable, artinya tidak bisa diubah valuenya
    - Cara merubah menjadi mutable => tambahkan mut agar variabel dapat diubah valunya
    - Rust mendukung dua metode deklarasi variabel, yaitu inference dan manifest

    2. Constant
    - Berbeda dengan variabel, contant tidak di ijinkan untuk merubah menjadi mutable
    - Contant selalu immutable dan dapat di deklarasi dalam semua scope dan penulisan uppercase dan snake case
    - Selain itu kontanta bisa ditulis dengan kerword 'static', secara teknis bedanya ada di manajemen memory
    - Jika ditulis dengan const, tidak memiliki alamat memory yang pasti dan setiap di gunakan pasti terjadi proses copy value
    - Jika ditulis dengan static, mempunyai alamat memory yang jelas

    3. Shadowing
    - Shadowing sendiri adalah pendefinisan ulang variabel yang sebelumnya sudah didefinisikan
    - Biasanya teknik ini dipakai untuk isolasi variabel dalam sebuah blok kode
    - Bisa terjadi dalam satu scope yang sama, bisa juga terjadi pada scope yang berbeda
    - Shadowing juga berbeda dengan variabel mutable, perbedaan pada saat alokasi memori
    - Mutable jika terdapat perubahan nilai maka data baru disimpan ke alamat memory yang sama menggatikan data sebelumnya
    - Shadowing ketika deklarasi variabel baru menggunakan 'let' maka akan dianggap sebagai variabel baru dan akan
      mengalokasikan alamat memory baru untuk menampung data variabel baru tersebut
    - Shadowing juga dapat digunakan pada variable dengan tipe data yang berbeda, jika menggunakan 'mut' tidak diperbolehkan menggubah tipe data
    */

    // VARIABLE AND MUTABILITY
    let mut message_number = 1;
    let message1 = "Hello World";
    println!("Message number {}: {}", message_number, message1);

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

    // CONSTANTS
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Konstanta const : {}", THREE_HOURS_IN_SECONDS);
    static ONE_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;
    println!("Konstanta static : {}", ONE_HOURS_IN_SECONDS);

    // SHADOWING
    let x = 5;
    let mut y = 5;
    let x = x + 1;
    y = y +1;

    {
        let x = x * 2;
        y = y *2;
        println!("The value of x in the inner scope is: {x}");
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // Shadowing bisa dilakukan beda tipe data
    // let x = "5";
    // mutable tidak bisa beda tipe data, akan error
    //y = "5";
}
