/*
    Tipe data string
    - Dalam rust terdapat 2 jenis tipe data string:
    ~ Tipe string literal (kadang disebut string slice atau &str). Tipe data ini ada pada nilai
    yang dideklarasikan dengan diapit tanda petik dua (string literal)
    ~ Tipe string yang merupakan tipe data custom, yang merupakan sebuah struct

    String literal - &str
    - Tipe data yang menampung data kolektif UTF-8 bytes (seperti String) tetapi immutable
    - Datanya tidak disimpan di heap maupun juga di stack, melainkan pada static storage
    - Termasuk kategori tipe data unowned atau ferefence tanpa owner (boleh diartikan ownernya adalah program)

    String custom type
    - Tipe data yang menampung data kolektif UTF-8 bytes yang dinamis dan dibuat via struct
    - Isinya adalah data kolektif bertipe bytes datanya disimpan di heap dan metadata disimpan di stack
    - Tipe data ini dikategorikan sebagai tipe data owned

    Kesimpulan
    - Data bertipe String reference bisa di akses dalam bentuk &String atau &str (menggunakan method as_str atau as_mut_str)
    - Data pada String tersebut bisa di modifikasi, ditambahin dan dikurangi

    - Data bertipe &str adalah fixed dan immutable, konversi data dari &str menghasilkan data baru dan owner baru
    - Sangat terbatas apa yang bisa dilakukan menggunakan &str

    Kapan harus memakai &str dan String
    - Tipe &str lebih cepat performanya dibanding String karena disimpan di static storage dan dijamin valid oleh rust
    - Kekurangan adalah unowned, operasi mutability tidak bisa digunakan
    - Dalam case normal, sangat dianjurkan menggunakan &str, kecuali memang dibutuhkan adalah owned string
 */

fn main() {
    // konversi String ke &str
    // di dapat melalui operasi borrow dari String
    let str1 = String::from("Hello");
    let str2 = str1.as_str(); // sudah bertipe &str
    // Bisa juga menggunakan method as_mut_str untuk mutable borrow.
    // Namun dalam penggunaannya, owner data diwajibkan mutable
    let mut str_mut = String::from("Hello");
    let str3 = str_mut.as_mut_str();

    // konversi &str ke String
    // cara kerjanya adalah &str di copy sebagai data baru bertipe String
    let str4 = "World";
    let str5 = str2.to_string();
}
