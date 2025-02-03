/*
    Default Visibility
    - Di rust hampir semua item secara default adalah private
    - Jika suatu item adalah private, maka item itu hanya bisa di akses dari current module scope dan dari submodules milik current scope
    - Jika suatu item adalah publik, maka dia bisa di akses dari module lain di luar current module scope, dengan catatan
    parrent module scope tersebut harus publik

    Re Exported Item
    - Re-export item adalah sebuah cara untuk mem-bypass pengaksesan item yang secara hierarki memang tidak bisa diakses dari luar module,
    bisa jadi karena visibility item atau parent modulenya adalah private
    - Cara re-export item adalah menggunakan keyword pub use kemudian diikuti dengan path yang ingin di export,
    dan juga nama export item

    Public visibility scope
    - Keyword pub digunakan untuk mengubah visibility item menjadi public
    - Bisa juga di kombinasikan salah satu keyword self, crate dan super
    - Dengannya kita bisa menentukan visibility publik item dengan scope yang lebih spesifik
 */
mod messaging;

fn main() {
    messaging::say_hello();

    // pemanggilan re-exported yang sudah di definisikan pub use
    messaging::some_black_magic();

    // tidak bisa dikses karena hanya akan valid di path module outer saja
    // messaging::outer::inner::say_in_path();

    // bisa di akses karena dalam 1 crate yang sama
    messaging::outer::inner::say_in_crate();
}
