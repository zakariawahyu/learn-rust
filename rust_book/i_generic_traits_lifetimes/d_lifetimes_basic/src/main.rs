/*
    Lifetime
    - Dalam Rust, setiap variabel atau objek memiliki lifetime, yaitu berapa lama variabel atau objek
    tersebut dapat digunakan oleh program
    - Rust memastikan bahwa memori yang digunakan pada variabel atau objek hanya valid selama periode waktu yang diperlukan,
    tidak lama dari itu
    - Lifetime biasanya ditentukan oleh blok variabel atau objek dideklarasikan, serta ownnership dengan variabel atau objek lain
    - Rust juga menggunakan bebrapa aturan untuk menentukan lifetime secara otomatis, sehingga programmer tidak perlu secara manual
    menentukan lifetime pada variabel atau objek
 */

#[test]
fn lifetime_test() {
    // variabel x akan memiliki lifetime dalam block main, setelah main selesai dieksekusi maka x dihapus dalam memori
    let x;

    {
        // variabel y dideklarasikan pada scope/block baru
        // y akan memiliki lifetime hingga akhir scope/block baru ini
        let y = 5;
        // var x memiliki alamat memori (reference) dari var y
        x = &y;
        println!("x : {}", x); // masih bisa di akses karena var y masih memiliki lifetime pada scope ini
    }
    // setelah ini lifetime var y akan dihapus otomatis oleh rust karena berada diluar scopenya

    // ketika mencoba mengakses var x yang telah memiliki reference var y akan terjadi error
    // dikarenakan alamat memori var y sudah tidak valid lagi karena hilang dari memori
    // seperti ini disebut dengan dagling reference (mengarah data yang sudah hilang)
    println!("x: {}", x);
}

fn main() {

}
