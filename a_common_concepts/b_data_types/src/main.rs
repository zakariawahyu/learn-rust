fn main() {
    /*
    1. Data Types
    - Rust mempunyai 2 jenis tipe data, scalar dan compund. Scalar menampung single value dan Compound dapat menampung multiple value
    - Scalar : Integer, Floating Point, Boolean, Char
    - Compound : Array, Tuple
    */

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
