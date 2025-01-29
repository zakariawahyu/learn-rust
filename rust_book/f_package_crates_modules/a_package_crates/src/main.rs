/*
    Rust Path
    - Path adalah notasi penulisan alamat path dan item pada rust programming
    - std::time::Duration adalah path untuk item yang isinya adalah Struct bernama Duration
    - Item bisa berupa banyak jenis, bisa saja struct, macro, konstanta atau lainnya
    - Sebuah path bisa memiliki banyak bagian (disebut segment), sebagai contoh diatas memiliki 3 segment yaitu std, time dan Duration
    - Karakter :: digunakan dalam penulisan path untuk pembatas antar segment
    - Dalam sebuah path, yang disebut Item adalah segment terakirnyam, contoh di atas berati Itemnya Duration

    Absolute dan relative path
    - Absolute path adalah path yang penulisannya lengkap dari root path. contoh std::time::Duration
    - Relative path adalah path yang penulisannya relatif terhadap current path, contohnya self::my_func, super::my_mod::my_constant

    Penggunaan keyword use untuk import path
    - Ada alternatif cara lain untuk memperpendek penulisan dan pengaksesan path, yaitu dengan menggunakan keyword use.
    - Use std::io; atau use std::io::stdin; atau use std::io::{stdin};
    - Keyword ini bisa digunakan di mana saja, artinya tidak harus di luar fungsi main. Bisa saja di dalam fungsi, atau di dalam blok kode seleksi kondisi atau lainnya

    contoh parent path sama yaitu io
    use std::io::stdin;
    use std::io::stderr;

    jika items yang parents pathnya sama bisa ditulis seperti ini => use std::io::{stdin, stdout};

    Import semua items dalam suatu path => use std::io::*;
    Eukuivalen dengan use std::io::{stdin, stderr, stdout, <path lainnya>}
 */

/*
    Rust crate
    - Crate adalah unit kompilasi di rust
    - Eksekusi cargo run, cargo build atau rustc akan men-trigger proses kompilasi dan unit (yang disebut crate) akan di compile
    - Rust mengkategorikan crate menjadi 2 jenis, yaitu binary crate dan library crate

    Binary crate
    - Binary crate adalah program yang dikompilasi ke bentuk executable, untuk kemudian dijalankan
    - Binary crate berada dalam sebuah package yang dibuat menggunakan command cargo new <nama package> atau cargo new  --bin <nama package>

    Library crate
    - Library crate tidak di compile ke bentuk executable dan tidak memiliki fungsi main
    - Library crate digunakan untuk mendefinisikan set functionality yang reusable atau bisa digunakan di banyak project/package
    - Untuk membuat library crate bisa menggunakan command cargo new --lib <nama package>
 */


/*
    Rust Package
    - Package adalah sebuah set yang berisi banyak functionality
    - Satu buat package bisa berisi satu atau banyak crate
    - Package dimanage oleh cargo, yang merupakan package manager Rust
    - Penulisan package dan juga crate belum ada dokumentasi resminya, kebanyakan menggunakan snake_case atau kebab-case

    File Cargo.toml
    - File ini menampung beberapa informasi penting milik package, seperti nama package, versi package, versi rust serta depedencies dll
    - Secara default block package berisi 3 buah field yaitu name, version dan edition
    - Secara default block dependencies berisi kosong

    Menambahkan external dependencied
    - Kunjungi official crate registry rust yaitu : crates.io
    - Menambahkan melalui fille cargo.toml dan masukkan ke dalam blok dependencies. <nama crates> = "<versi>" contoh rand = "0.9.0"
    - Bisa juga ditambahkan melalui command line cargo add <nama crates>@<version> contoh cargo add rand@0.9.0
    -
 */

fn main() {
    println!("Hello, world!");
    println!("Welcome to Rust book! Manage packages and crates");
}
