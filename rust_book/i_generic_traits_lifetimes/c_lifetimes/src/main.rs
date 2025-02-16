/*
    Static
    - Static adalah item yang mirip dengan konstanta, tapi memiliki perbedaan yaitu alamat memory yang dialokasikan
    untuk menampung data static item adalah jelas/fix.
    - Semua reference terhadap static item mengarah ke alamat memory yang sama
    - Dengan karaterisik seperti itu, static biasanya diterapkan pada data yang sifatnya shared atau bisa di akses secara global
    - Menggunakan keyword static pada pendefinisian konstanta
    - Menggunakan keyword 'static pada string literal (&str)
    - Static bisa digunakan pada semua tipe data primitif, tetapi tidak bisa digunakan untuk custom stype seperti String
    - Jika ingin membuat kontanta bertiipe string, solusinya menggunakan tipe data &'static str

    Lifetimes
    - Lifetime digunakan oleh rust compiler untuk memonitor umur dari reference agar tetap dianggap valid, menempel di variabel lebih tepatnya refrence variable
    - Normalnya kita tidak perlu berurusan dengan lifetime, karena rust yang mengelola lifetime dari sebuah reference
    - Ketike berurusan dengan data primitif maupun non-primitif, tak perlu khawatir dengan lifetime
    - Lifetime hanya perlu diperhatikan sewaktu berurusan dengan data pointer/reference apalagi data tersebut keluar masuk block
    - Rust menerapkan default lifetime dalam pengecekan reference, seperti variabel akan valid dalam block dan invalid jika diluar block
    dan datanya di dealokasi ketika sudah tidak ada referencenya
 */
static PI: f64 = 3.1415926535;

fn main() {
    println!("Hello, world!");
}
