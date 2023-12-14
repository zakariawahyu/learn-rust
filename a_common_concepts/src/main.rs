fn main() {
    /*
     1. Variable
    - Penamaan variabel menggunakan snake case
    - Secara default semua variabel adalah immutable, artinya tidak bisa diubah valuenya
    - Cara merubah menjadi mutable => tambahkan mut agar variabel dapat diubah valunya
    - Rust mendukung dua metode deklarasi variabel, yaitu inference dan manifest

    2. Constant
    - Berbeda dengan variabel, contant tidak di ijinkan untuk merubah menjadi mutable
    - Contant selalu immutable dan dapat di deklarasi dalam semua scope dan penulisan uppercase dan snake case
    - Selain itu kontanta bisa ditulis dengan kerword 'static', secara teknis bedanya ada di manajemen memory
    - Jika ditulis dengan const, tidak memiliki alamat memory yang pasti dan setiap di gunakan pasti terjadi proses copy value
    - Jika ditulis dengan static, mempunyai alamat memory yang jelas

    3. Shadowing
    - Shadowing sendiri adalah pendefinisan ulang variabel yang sebelumnya sudah didefinisikan
    - Biasanya teknik ini dipakai untuk isolasi variabel dalam sebuah blok kode
    - Bisa terjadi dalam satu scope yang sama, bisa juga terjadi pada scope yang berbeda
    - Shadowing juga berbeda dengan variabel mutable, perbedaan pada saat alokasi memori
    - Mutable jika terdapat perubahan nilai maka data baru disimpan ke alamat memory yang sama menggatikan data sebelumnya
    - Shadowing ketika deklarasi variabel baru menggunakan 'let' maka akan dianggap sebagai variabel baru dan akan
    mengalokasikan alamat memory baru untuk menampung data variabel baru tersebut
    - Shadowing juga dapat digunakan pada variable dengan tipe data yang berbeda, jika menggunakan 'mut' tidak diperbolehkan menggubah tipe data

    4. Data Types
    - Rust mempunyai 2 jenis tipe data, scalar dan compund. Scalar menampung single value dan Compound dapat menampung multiple value
    - Scalar : Integer, Floating Point, Boolean, Char
    - Compound : Array, Tuple
     */

    // VARIABLE AND MUTABILITY
    let mut message_number = 1;
    let message1 = "Hello World";
    println!("Message number {}: {}", message_number, message1);

    message_number = 3;
    let message3 = 4;
    // {0} pemanggilan argument yang ke 1, {1} berati argument yang ke 2
    println!("Message number {1}: {0}", message_number, message3);

    let _num1 = 1; // deklarasi inference, tidak ditulis tipe datanya
    let _num2: i8 = 2; // deklarasi manifest, ditulis secara jelas tipe datanya

    // penulisan variabel juga boleh dipisan seperti ini
    let _number3: i16;
    _number3 = 3;

    // deklarasi banyak variabel, inference
    let (_num1, _num2, mut _num3) = (1, 2, 3);

    // deklarasi banyak variabel, manifest
    let (_num4, mut _num5, _num6): (i8, i16, i32) = (4, 5, 6);

    // deklatasi variabel dengan tipe data di tentukan dari value
    let _num7 = 7i64;

    // CONSTANTS
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Konstanta const : {}", THREE_HOURS_IN_SECONDS);
    static ONE_HOURS_IN_SECONDS: u32 = 60 * 60 * 2;
    println!("Konstanta static : {}", ONE_HOURS_IN_SECONDS);

    // SHADOWING
    let x = 5;
    let mut y = 5;
    let x = x + 1;
    y = y +1;

    {
        let x = x * 2;
        y = y *2;
        println!("The value of x in the inner scope is: {x}");
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // Shadowing bisa dilakukan beda tipe data
    // let x = "5";
    // mutable tidak bisa beda tipe data, akan error
    //y = "5";

    /* SCALAR DATA TYPES
     1. Integer (Signed and Unsigned)
     - Signed integer adalah tipe data integer yang dapat menampung nilai positif dan juga negatif
     - Unsigned integer adalah tipe data integer yang hanya dapat menampung nilai positif
     - Integer mempunyai size dari 8, 16, 32, 64 dan 128 bit */
    let numerik1 = 24;
    let numerik2: i8 = -2;
    println!("Signed : {} | {} ", numerik1, numerik2);

    let numerik4: u16 = 4;
    let numerik5: u64 = 56;
    println!("Unsigned : {} | {}", numerik4, numerik5);

    /* 2. Floating Point
    - Floating point adalah tipe data yang mendukung nilai di belakang koma. Mempunyai 2 size yaitu f32 dan f64 */
    let fp1: f32 = 3.14;
    let fp2: f64 = 3.1415926535;
    // {:.n}, dimana n menetukan jumlah angka yang tambil di belakang koma
    println!("Floating Point : {:.1} | {:.5}", fp1, fp2);

    /* 3. Bool
    - Menerima 2 pilihan saja yaitu true or false */
    let b1 = true;
    let b2 = false;
    println!("Bool : {} | {}", b1, b2);

    /* 4. Char
    - Tipe data char menampung sebuah data(unicode), contohnya seperti 'n', '-', '2', '😻'*/
    let c1 = 'z';
    let c2: char = 'ℤ'; // with explicit type annotation
    let c3 = '😻';
    println!("Char : {} | {} | {}", c1, c2, c3);

    /* COMPOUND DATA TYPES
    1. Tuple
    - Merupakan tipe data yang isinya koleksi dari banyak data atau value, digunakan untuk menampung data heterogeneous atau campuran
    - Memiliki panjang yang tetap. setelah di deklarasikan, tidak dapat bertambah atau menyusut
    - Untuk menampikan nilai per elemen, dapat menggunakan notasi .N dimana 'n' merupakan indeks elemenya */
    let tuple_a = ("jason", 27, ["racing", "working out"], true); // penulisan inference
    println!("tuple_a: {:?}", tuple_a);
    println!("index 0: {:?}", tuple_a.0);
    println!("index 1: {:?}", tuple_a.1);
    println!("index 2: {:?} {:?}", tuple_a.2[0], tuple_a.2[1]);
    println!("index 3: {:?}", tuple_a.3);

    // Untuk membuat tuple menjadi data yang mutable, dengan menambahkan mut pada saat deklatasi
    let mut tuple_b: (&str, i32, [&str; 2], bool) = ("default", 0, ["age", "name"], false); // penulisan manifest
    tuple_b.0 = "damian";
    tuple_b.2 = ["gaming", "adventuring"];
    tuple_b.3 = true;
    println!("tuple_b: {:?}", tuple_b);

    // packing tuple: cara membuat tuple yang dimana nilai elemen dari variabel lain
    let name = "jhon";
    let age = 23;
    let tuple_c = (name, age);
    println!("tuple_c: {:?}", tuple_c);

    // unpacking tuple: kebalikan dari packing tuple, data tuple di distribusikan ke banyak variabel dalam 1 baris deklarasi
    let tuple_d = ("doe", ["software engineer"], true);
    let (names, workes, is_male) = tuple_d;
    println!("tuple_d: {} | {:?} | {}", names, workes, is_male);

    /* 2. Array
    - Array adalah kumpulan data dengan tipe sejenis dan disimpan dalam 1 variabel dan fixed lenght
    - Array memiliki kapasitas yang nilainya ditentukan saat deklarasi/alokasi
    - Data dalam array tidak boleh melebihi dari kapasitas yang sudah ditentukan
    - Tidak bisa menambahkan elemen baru ke array, karena array memiliki size fixed. solusinya menggunakan vector */
    let mut numbers = [1, 2,3];
    println!("array : {:?}", numbers);
    let data0 = numbers[0];

    // pengaksesan elemen array
    println!("elemen array 0 : {}", data0);
    let data1 = numbers[1];
    println!("element array 1 : {}", data1);

    // mengubah isi elemen, harus dipastikan variabelnya mutable
    numbers[1] = 7;
    numbers[2] = 9;
    println!("array : {numbers:?}");

    // deklarasi penulisan array iference ditulis tipe data dahulu baru panjang array
    let data_numerik1: [i64; 5] = [234, 235, 344, 345, 355];
    println!("{data_numerik1:?}");

    // penulisan predefined value [T; N], maka T adalah nilai default setiap elemen array dan N adalah size array
    let data_numerik2 = [1; 3];
    println!("panjanga array {}", data_numerik2.len());
}
