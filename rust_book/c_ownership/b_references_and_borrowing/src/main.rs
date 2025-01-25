use rand::Rng;
/*
    Pointer:
    - Pointer artinya adalah alamat memory. Variabel pointer artinya adalah variabel yang berisikan alamat memory(bukan value sebenernya)
    - Variabel pointer di tandai dengan adanya karakter '&' tipe datanya, &i32, &bool

    Reference:
    - Reference artinya adalah pointer dari sebuah variabel atau data
    - operasi pengambilan pointer dari variabel disebut referencing, dengan cara menggunakan karakter '&'
    - By deafult reference sifatnya immutable
    - Dalam waktu yang sama, hanya boleh satu mutable reference atau banyak imutable reference (keduanya tidak bisa bersamaan, harus salah datu)

    Deference:
    - Deference atau underlying adalah cara mengambil nilai sebenernya dari sebuah pointer
    - Cukup menambahkan karakter '*' pada variabel pointer

    Borrowing:
    - Borrowing adalah meminjam data milik owner, dipinjam agar bisa diakses tanpa harus memindahkan ownernya
    - Borrowing akan menghemat memory karena tidak melakukan copy data, melainkan meminjam reference datanya
    - Semua state reference di rust baik mutable atau immutable adalah operasi borrowing
 */

fn pointer() {
    let mut number: i32 = 24;
    println!("value: {:?}", number);

    let pointer_number: &mut i32 = &mut number;
    // akan memunculkan alamt memory
    // alamat memory yang muncul sangat mungkin berbeda karena alokasi adalah random
    println!("pointer: {:p}", pointer_number);

    *pointer_number = 12;

    let underlaying_number = *pointer_number;
    println!("deference: {:?}", underlaying_number);
    println!("value: {:?}", number)
}

fn borrowing() {
    let msg_1 = String::from("Hello rust");
    let msg_2 = &msg_1; // <-- borrow operation
    println!("Message_2 : {:?}", msg_2);

    let mut msg_3 = String::from("Hello");
    let msg_4 = &mut msg_3; // <-- borrow mutable operation

    *msg_4 = String::from("World");
    println!("Message_4 : {:?}", msg_4);

    // dalam waktu yang sama, borrow hanya di perbolehkan 1 mutable reference atau banyak imutable reference
    let a = String::from("Hello Immutable");
    // banyak immutable fererence diperbolehkan
    let s1 = &a;
    let s2 = &a;
    println!("s1 : {} , s2 : {}", s1, s2);

    let mut b = String::from("Hello Mutable");
    // tidak di perbolehkan banyak mutable reference seperti ini
    let s3 = &mut b;
    // let s4 = &mut b;
    println!("s3 : {}", s3);

    let mut c = String::from("Hello Mutable");
    // ataupun tidak diperbolehkan mutable dan immutable dalam waktu bersamaan seperti ini
    let s5 = &mut c;
    // let s6 = &c;
    println!("s5 : {}", s5);

    // let reference_to_dangle  =  dangle();
    // println!("reference_to_dangle : {}", reference_to_dangle);
}

// kasus seperti ini akan di handle nanti dalam rust lifetime
// fn dangle() -> &String {
    // variabel s akan valid dalam scop dangle ini, karena s berasal dari dalam function ini
    // let s = String::from("reference from dengle");
    // &s
    // jika dikeluarkan maka tidak akan valid lagi
// }

fn main() {
    println!("==== Pointer and Reference ====");
    pointer();

    let mut number = 25;
    println!("first number: {number}");

    for _ in 0..5 {
        change_number(&mut number);
        println!("change number: {number}")
    }

    println!("last number : {number}");

    println!("==== Borrowing ====");
    borrowing();

    // borrowing dalam satu waktu asalkan berbeda scope
    let mut fac_one = String::from("Message from one");
    println!("{:?}", fac_one);

    change_value(&mut fac_one);
    println!("{:?}", fac_one);

    {
        let fac_two = &mut fac_one;
        *fac_two = String::from("Message from two");
        println!("{:?}", fac_two)
    }

    if fac_one.contains("Message"){
        let fac_three = &mut fac_one;
        *fac_three = String::from("Message from three");
        println!("{:?}", fac_three)
    }
}

fn change_number(i: &mut i32) {
    *i = generate_rand_number()
}

fn generate_rand_number() -> i32 {
    let n = rand::thread_rng().gen_range(1..100);

    return n
}

fn change_value(s: &mut String){
    *s = String::from("Message Change")
}