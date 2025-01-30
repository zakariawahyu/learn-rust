/*
- Sebelumnya tadi efinisi item module my_io menggunakan notasi penulisan file nama_module.rs (yaitu my_io.rs)
- Pada module my_number ini kita akan gunakan notasi penulisan nama_module/mod.rs (menjadi my_number/mod.rs) untuk menampung definisi item module my_number.
 */

// karena submodule merupakan sebuah item milik module, maka harus ditambahkan juga keyword pub, agar submodule bisa diakses dari luar scope-nya
pub mod conversion_utility;

// atau bisa membuat submodule seperti ini
// berasal dari file check_number.rs
// akan tetapi penggunaan seperti ini jika check_number tidak mempunyai submodule lagi
pub mod check_number;

pub fn is_odd_number(number: i32) -> bool {
    return number % 2 == 1
}
