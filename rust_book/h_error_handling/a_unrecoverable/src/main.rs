/*
   Rust mengenal 2 jenis error, yaitu unrecoverable error (panic error) dan recoverable error

   Uncoverable error (RUNTIME error & COMPILE-TIME error)
   - Uncoverable adalah error yang tidak bisa ditangani, entah itu karena disengaja atau memang karena bug fatal
   - Jika mendapatkan error jenis ini, maka program akan crash atau berhenti secara paksa

    Macro panic!()
    - panic bisa dimunculkan dengan sengaja menggunakan macro panic!()
    - bisa dibilang macro panic!() adalah media untuk membuat recoverable error menjadi unrecoverable error

    Kapan harus menggunakan panic?
    - Error ketika ada inputan yang tidak valid, maka cukup di respon dengan pesan error saja, jangan panic
    - Error ketika operasi pembagian terhadap angka 0, maka tidak perlu menggunakan panic
    - Error karena file konfigurasi yang tidak sesuai, idealnya menggunakan panic
 */
use std::io;
use std::io::Write;

fn main() {
    let data = vec![1,2,3]; // contoh runtime error
    let data_array = [1,2,3]; // contoh compile error jika akses diluar kapasitas array

    // salah satu operasi yang menyebabkan panic adalah menggunakan index diluar kapasitas vector
    println!("data index 1: {}", data[1]);
    // println!("data index 6: {}", data[6]); // index out of bounds: the len is 3 but the index is 6
    println!("data index 2: {}", data[2]);

    // jadi bisa berbeda antara vector dan array. vector menghasilkan runtime error
    // runtime error menandakan program berhasil di compile dan juga berhasil di eksekusi
    // tipe data vector tidak tidak mampu mengetahui kapasitas datanya saat kompilasi, inilah kenapa error bisa lolos kompilasi

    // compile error menandakan program gagal di kompilasi dan tidak sampai di eksekusi
    // tipe data array kapasitasnya sudah bisa diketahui oleh compiler saat kompilasi, menyebabkan proses kompilasi gagal

    print!("enter your name : ");
    let _ = io::stdout().flush();

    let name = read_entry();
    if name.is_empty() {
        panic!("Enter your name !");
    }

    println!("name: {}", name);
}

fn read_entry() -> String {
    let mut message = String::new();
    let reader = io::stdin().read_line(&mut message);

    if reader.is_err(){
        return message;
    }

    message.trim().to_string()
}