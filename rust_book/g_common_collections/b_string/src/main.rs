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

    // String kosong
    let str6 = String::new();

    // String from_utf8, berguna untuk konversi data bytes ke string
    let str7 = String::from_utf8(vec![78, 55, 51]).unwrap();
    println!("{}", str7); // N73
}

#[test]
fn string_mutability() {
    // keyword mut untuk membuat mutable bisa digunakan untuk mengganti/replace data string ke data baru
    let mut str1 = String::new();

    // mereplace dari string kosong
    str1 = String::from("Zakaria Wahyu");

    // method .replace() digunakan untuk mengganti suatu substring dengan string lain
    // menghasilkan object string baru dan tidak merubah data string aslinya
    // jadi tidak perlu menggunakan keyword mut, karena datanya di copy data baru
    let str2 = String::from("My gear is Iphone");
    let str3 = str2.replace("Iphone", "Samsung Zeenbook");
    println!("str 2 : {str2}");
    println!("str 3 : {str3}");

    // method .insert_str() digunakan untuk menyisipkan substring pada posisi tertentu
    let mut str4 = String::from("Iphone");
    println!("str 4 : {str4}");
    str4.insert_str(6, " 12");
    println!("str 4 : {str4}");
    str4.insert_str(0, "My phone is ");
    println!("str 4 : {str4}");

    // method .insert() sama halnya dengan .insert_str()
    // perbedaannya pada parameter kedua di isi dengan char
    let mut str5 = String::from("3310");

    str5.insert(0, 'N'); // N3310
    str5.insert(1, 'o'); // No3310
    str5.insert(2, 'k'); // Nok3310
    str5.insert(3, 'i'); // Noki3310
    str5.insert(4, 'a'); // Nokia3310

    // method .push_str(), digunakan untuk menambah string di akhir
    let mut str6 = String::from("Planet");
    str6.push_str("Pluto"); //PlanetPluto

    // method .push() sama halnya dengan .push_str()
    // perbedaan untuk menambahkan data char
    let mut str7 = String::from("Mari");
    str7.push('a');

    // method .clear() digunakan untuk mengosongkan string
    let mut str8 = String::from("Mary");
    str8.clear()
}

#[test]
fn string_operation() {
    // method .contains() digunakan untuk megecek apakah suatu substring yang dicari ada atau tidak
    // mengembalikan tipe data bool
    let str1 = String::from("Hello");
    let is_exists = str1.contains("lo"); // true
    let is_exists2 = str1.contains("Hai"); // false
    println!("{is_exists} {is_exists2} {str1}");

    // Concat strings / slice join
    // operasi concat string bisa dilakukan memanfaatkan methos insert_str() dan push_strr()
    // bisa juga menggunakan methos .slice()
    let str2 = String::from("Apple");
    let str3 = String::from("Iphone 15");

    let str  = [str1, str2, str3].join(" ");
    println!("{str}");
}