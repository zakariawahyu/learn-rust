/*
1. Variable
- Variable adalah tempat untuk menyimpan data
- Cara membuat variable di Rust bisa menggunakan kata kunci let 
- Setelah variabel diisi data, maka tidak tidak bisa variable tersebut diubah lagi datanya

2. Mutable
- Variable yang sudah diisi datanya tidak bisa diubah lagi, atau disebut Immutable
- Namun Rust juga memperbolehkan jika kita ingin membuat Variable yang bisa diubah lagi, atau disebut Mutable
- Caranya kita bisa gunakan kata kunci let mut ketika membuat variable

3. Static Typing
- Rust adalah bahasa yang Static Typing,artinya setiap kita membuat variable 
dengan jenis data tertentu, maka dia tidak akan bisa berubah menjadi tipe data lainnya
- Sebelumnya kita membuat variable dengan tipe text/string, kita tidak bisa mengubah variable tersebut dengan data angka/number

4. Shadowing
- Di Rust, kita bisa membuat variable dengan nama yang sama
- Namun, saat kita membuat variable dengan nama yang sama, maka variable sebelumnya akan tertutup atau disebut shadowing
- Praktek ini mungkin kurang baik jika dilakukan terlalu sering, karena bisa membingungkan yang membaca kode kita
 */

#[test]
fn test_variable() {
    let name = "Zakaria Wahyu";
    println!("Nama : {}", name)
}

#[test]
fn test_mutable() {
    let mut name = "Zakaria Wahyu";
    println!("Nama : {}", name);

    name = "Nur Utomo";
    println!("Nama : {}", name);
}

#[test]
fn test_static_typing() {
    let mut name = "Zakaria Wahyu";
    println!("Nama : {}", name);

    // tidak boleh di ubah ke int karena sebelumnya str
    // name = 8;

    name = "Jaka";
    println!("Nama : {}", name);
}

#[test]
fn test_shadowing() {
    let name = "Zakaria";
    println!("Nama : {}", name);

    // shadowing membuat variabel baru dari variabel sebelumnya
    let name = "Wahyu";
    println!("Nama : {}", name);
}