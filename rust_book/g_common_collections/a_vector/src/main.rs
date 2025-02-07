use std::collections::VecDeque;

/*
    Vector
    - Vector adalah tipe data seperti array, tetapi dinamis
    - Dinamis artinya bisa bertambah dana berkurang kapanpun sesuai kebutuhan
    - Perbedaan vector dibanding array adalah pada jumlah elemen pada vector bisa
    bertambah lebih dari kapasitas yang ditentukan

 */

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // membuat vector dengan macro vec
    let mut data_one = vec!["one", "two", "three"];
    println!("data one : {data_one:?}");

    // method .len() untuk mencari tahu size atau jumlah elemen
    println!("len data one : {:?}", data_one.len());

    // method .capacity untuk mencari tahu kapasitas atau jumlah maksimum elemen
    println!("capacity data one : {:?}", data_one.capacity());

    // method .pop() untuk menghapus elemen terakhir
    println!("pop data one : {:?}", data_one.pop());
    println!("then length: {}, capacity: {}", data_one.len(),  data_one.capacity());

    // method .remove() untuk menghapus pada elemen index tertentu
    println!("remove data one index 0: {:?}", data_one.remove(0));
    println!("then data one : {:?}, length: {}, capacity: {}", data_one, data_one.len(), data_one.capacity());

    // method .push untuk menambahkan elemen baru pada vector
    data_one.push("zakaria");
    data_one.push("wahyu");
    data_one.push("nur");
    println!("new push data: {:?}", data_one);
    // kenapa capacity menjadi 6, padahal di awal 3
    // perubahan kapasitas terjadi ketika sebuah vector isinya bertambah lebih banyak dari jumlah alokasi maksimalnya
    // proses alokasi menhasilkan vector yang baru dengan kapasitas lebih besar
    println!("length: {}, capacity: {}", data_one.len(),  data_one.capacity());

    // notasi [i] digunakan untuk modifikasi nilai elemen
    data_one[3] = "four";
    println!("new push data: {:?}", data_one);
    // bisa juga digunakan untuk mengakses elemen
    println!("data index 0 {:?}", data_one[0]);
    // atau akses menggunakan method get serta error handlingnya
    let get_data_one = data_one.get(1);
    match get_data_one {
        Some(get_data_one) => println!("Index 1 in data one is : {:?}", get_data_one),
        None => println!("No data in index 1"),
    }

    // method .is_empty() digunakan untuk mengecek apakah vector kosong atau tidak
    let check_empty = data_one.is_empty();
    println!("is empty ? : {}", check_empty);

    // method .clear() untuk mengosongkan nilai vector
    data_one.clear();
    println!("clear data one : {:?}", data_one);
    println!("length: {}, capacity: {}", data_one.len(),  data_one.capacity());

    // method .append() digunakan untuk menggabungkan dua buah vector
    // ethod .append() parameternya adalah langsung mutable reference dari literal vector
    let mut result_one = vec![17, 2, 7];
    let mut result_two = vec![8, 6];
    result_one.append(&mut result_two);

    println!("data result one : {:?}", result_one);
    println!("length: {}, capacity: {}", result_one.len(),  result_one.capacity());

    // methos .sort() digunakan untuk mengurutkan elemen vector
    result_one.sort();
    println!("sort data result one : {:?}", result_one);

    // Macam deklarasi vector
    // deklaradi di tentukan tipe data secara explisit
    // dengan notasi penulisan Vec<T> bisa ditentukan tipe data elemen yang diinginkan.
    let vector_1 :Vec<u32> = vec![1, 2, 3];

    // deklarasi vector kosong, tapi harus mewajibkan tipe data ditulis secara explisit
    // dikarenakan tipe data tidak bisa diidentifikasi dari isinya (karena isinya kosong)
    let vector_2 :Vec<&str> = vec![];
    let vector_3 :Vec<&str> = Vec::new();

    // ownership tipe data vector
    for e in vector_1 {
        // terjadi pemindahan ownership ke variable e
        println!("{}", e);
    }
    // terjadi move semantics (pemindahan owner) karena vector bukan tipe data primitif, solusinya menggunakan borrowing
    // println!("{:?}", vector_1[1]);

    // borrowing data menggunakan &
    let vector_4 = vec![1, 2];
    for data in &vector_4  {
        println!("{}", data);
    }
    // disini masih bisa akses
    println!("vector borrowing : {:?}", vector_4[0]);

    // vector slice. Sama seperti array, vector juga bisa dibuat seperti slice
    let vec_population = vec![3, 2, 1];
    let vec_population_slice = &vec_population[1..2];
    println!("vector slice : {:?}", vec_population_slice);

    // bisa juga membuat vector dengan enum sepererti ini
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("SpradshetCell {:?}", row)
}

/*
    Tipe data VecDeque<T>
    - Sama seperti Vec<T> plus mendukung operasi menambah dan mengurangi elemen dari dua sisi secara efisien
    - Tipe data Vec<T> ada method pop yang fungsinya menghapus data elemen terakhir
    dan method push untuk menambah elemen baru dari kanan
    - Pada tipe data VecDeque<T> ada beberapa method tambahan yaitu
       ~ pop_front() untuk hapus data elemen pertama atau paling kiri (indeks 0)
       ~ push_front() untuk menambah data dari kiri (indeks 0)
       ~ pop_back() untuk hapus data dari elemen pertama atau kanan (indeks terakhir)
       ~ push_back() untuk menambah data dari kanan
 */
#[test]
fn vector_deque() {
    let mut vec_deque = VecDeque::from(vec!["one", "two", "three"]);
    println!("vec deque : {:?}", vec_deque);

    vec_deque.pop_front();
    println!("pop front : {:?}", vec_deque);
    vec_deque.push_front("zero");
    println!("push front : {:?}", vec_deque);

    vec_deque.pop_back();
    println!("pop back : {:?}", vec_deque);
    vec_deque.push_back("four");
    println!("push back : {:?}", vec_deque);
}