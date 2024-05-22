/*
- Setiap nilai di Rust memiliki tipe data. Secara garis besar Rust membagi tipe data menjadi dua subsets; scalar dan compound
- Scalar type merepresentasikan single value (nilai tunggal), yaitu integer, float, boolean dan char
- Compound type merepresentasikan beberapa value (bisa lebih dari satu) dalam satu type, yaitu tuple dan array

1. Scalar Type
- Integer type, yaitu tipe data number dalam bilangan bulat
- Float type, yaitu tipe data number dalam desimal
- Boolean type, yaitu tipe data yang hanya bernilai true (benar) atau false (salah)
- Char type, yaitu tipe data karakter

2. Compound Type 
- Tuple type, yaitu kumpulan beberapa data yang bisa berbeda tipe data
- Array type, yaitu kumpulan beberapa data yang harus tipe data yang sama

Default Number
- Saat membuat variable secara implicit (tidak menyebutkan tipe data), maka Rust akan menggunakan default type
- Jika bilangan bulat, maka akan menggunakan i32
- Jika bilangan dengan pecahan desimal, maka akan menggunakan f64

*/

#[test]
fn number() {
    let a:i32 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b)
}

/*
Konversi Tipe Data
- Rust bisa melakukan konversi tipe data dari tipe data Number yang ukurannya kecil ke ukurannya lebih besar begitu juga sebaliknya
- Namun perlu diperhatikan, jika kita lakukan konversi tipe data Number dari ukuran besar ke kecil, 
maka bisa terjadi yang namanya Integer Overflow, yaitu kondisi dimana nilai number tidak bisa ditampung oleh tipe data tujuan konversi. 
- Misal kita punya number 1000000 dalam bentuk i32, lalu kita konversi ke bentuk i8, maka akan terjadi Integer Overflow, karena i8 tidak bisa menampung nilai tersebut
- Untuk melakukan konversi, kita bisa gunakan kata kunci as
 */

#[test]
fn number_conversion() {
    let a:i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    // terjadi integer overflow, tidak bisa menampung kapasitas dari variabel d
    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e)
}

/*
Numeric Operations
- Rust mendukung semua operasi numerik
- Operator yang digunakan hampir sama dengan kebanyakan bahasa pemrograman lainnya
 */

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);

    let d = a / b;
    println!("{}", d);

    let e = c + a;
    println!("{}", e)
}

#[test]
fn augmented_assigments() {
    let mut a = 10;
    println!("{}", a);
   
    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);

    a *= 10;
    println!("{}", a);

    a /= 10;
    println!("{}", a);

    a %= 10;
    println!("{}", a);
}

/*
Boolean
- Boolean adalah tipe data yang sederhana, hanya bernilai true (benar) atau false (salah)
- Namun walaupun tipe data ini adalah tipe data sederhana, namun tipe data ini banyak digunakan di mana-mana, terutama percabangan dan perulangan
*/

#[test]
fn boolean() {
    let a: bool = true;
    let b = false;

    println!("{} , {}", a, b)
}

/*
Comparison Operators
- Comparison Operator (operator perbandingan) adalah operator yang menghasilkan nilai boolean (benar / salah)
 */

 #[test]
fn comparison() {
    let a: bool = 10 > 8;
    let b = 100 < 10;
    let c = 10 <= 9;
    let d = 8 >= 8;
    let e = 7 == 7;

    println!("{} , {}, {}, {}, {}", a, b, c, d, e)
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;

    let lulus = absen >= 75;
    let lulus_akhir = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_akhir;
    println!("Lulus : {}", lulus);
    println!("Lulus Akhir : {}", lulus_akhir);
    println!("{}", lulus_final)
}