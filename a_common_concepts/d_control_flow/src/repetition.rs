use std::thread::sleep;
use std::time::Duration;
pub fn while_repetition() {
    // While, perulangan akan di eksekusi terus jika konsidinya true dan akan berhenti jika nilainya false
    let mut i =0;
    let max = 5;
    while i <= max {
        println!("Nilai : {i}");
        i += 1;

        sleep(Duration::from_millis(100));
    }

    // Nested while
    let mut j = 0;
    while j < max {
        let mut k = 0;
        let max_inner = j;

        while k <= max_inner {
            print!(" *");
            k += 1;
        }

        println!();
        j += 1;
    }

    // perulangan array
    let names = ["jason", "grayon", "drake", "damian"];
    let mut x = 0;
    while x < names.len() {
        println!("array indeks ke {} : {}", x, names[x]);
        x += 1;
    }
}

pub fn loop_repetition() {
    // loop sedikit berbeda dengan while, loop tidak membutuhkan argment
    // blok kode loop akan terus di eksekusi selama program tidak di stop
    // cara memberhentikan perulangan menggunakan keywoard break
    // continue digunakan untuk melanjutkan paksa sebuah perulangan
    let mut i = 0;
    loop {
        i += 1;

        if i % 2 == 1 {
            continue;
        }

        println!("Nilai {i}");

        if i > 9 {
            break
        }
    }

    // loop label, manfaat bisa mengeksekusi break atau continue ke perulangan di luar blok kode perulangan itu berada
    let mut x = 0;
    'main_loop: loop {
        x += 1;
        let mut y = 0;

        loop {
            if x > 5 {
                break 'main_loop // akan menjalankan break di main loop
            }

            y += 1;
            if y > x{
                break
            }
            print!("{x}")
        }
        println!()
    }

    // returning from loop, pemanfaatan loop dan break untuk menampung sebuah return valur dari blok kode perulangan loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter * 2;
        }
    };

    println!("result: {result}");

    // perulangan array
    let names = ["jason", "grayon", "drake", "damian"];
    let mut a  = 0;
    loop {
        if a >= names.len() {
            break;
        }
        println!("array indeks ke {} : {}", a, names[a]);
        a += 1;
    }
}

pub fn for_repetition(){
    // notasai penulisan range, a..b artinya dari a hingga satu angka dibawah b
    // notasai penulisan range, a..=b artinya dari a hingga b
    for i in 0..4 {
        println!("Perulangan i ke {i}")
    }

    for j in 0..=4 {
        println!("Perulangan j ke {j}")
    }

    //label perulangan, sama seperti loop cukup menambahkan 'NamaLabel
    'forx: for x in 0..=5 {
        if x == 2 {
            println!("perulangan dihentikan paksa pada iterasi {x}");
            break 'forx
        }
        println!("{x}");
    }

    // perulangan pada array
    let names = ["jason", "grayon", "drake", "damian"];
    for name in names {
        println!("{name}");
    }

    // bisa juga perulangan array menggunakan indeks
    for i in 0..names.len() {
        println!("array indeks ke {} : {}", i, names[i]);
    }
}