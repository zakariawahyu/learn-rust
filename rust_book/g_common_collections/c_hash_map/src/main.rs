
/*
    Hash Map
    - Untuk menyimpan data dengan key yang terasosiasi
    - HashMap<K, V> yaitu K mewakili key dan V itu value
    - Untuk menggunakan hash map bisa menggunakan std library dengan use std::collections::HashMap;

 */

use std::collections::HashMap;

fn main() {
    // untuk menambahkan data menggunakan method .insert()
    // Key dan value harus menggunakan tipe data yang sama
    let mut scores = HashMap::new();
    scores.insert(String::from("Napoli"), 5);
    scores.insert(String::from("Hans"), 6);
    println!("Data score : {:?}", scores);

    // untuk mengakses data bisa menggunakan method .get()
    // parameter key untuk mendapatkan datanya
    let get_score = scores.get(&String::from("Hans"));
    println!("Get Score : {:?}", get_score);

    // memperbarui nilai jika key tidak memiliki nilai
    scores.insert(String::from("Team_A"), 10);
    scores.entry(String::from("Team_A")).or_insert(11);
    scores.entry(String::from("Team_B")).or_insert(12);
    println!("Data score before : {:?}", scores);

    // ownership pada hashmap ketika menambahkan data dari variabel
    let the_question = String::from("What is the team you are?");
    let the_answer = String::from("England Red Team");

    let mut games = HashMap::new();
    games.insert(the_question, the_answer);
    // setelah insert variabel akan berpindah ownership ke games

    for (key, value) in scores{
        println!("{}: {}", key, value);
    }

    let text = "hari ini saya ngantuk, saya pengen tidur saja";
        let mut result = HashMap::new();

    // meloping setiap whitespace
    for word in text.split_whitespace(){
        // melakukan pengecekan jika exist maka count + 1
        let count = result.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", result);
}
