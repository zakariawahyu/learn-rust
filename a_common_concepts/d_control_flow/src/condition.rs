pub fn condition(){
    // kondisi if pada rust = ekspresi logika yang hasilnya boolean
    let number_a = 6;
    if number_a < 5 {
        println!("Number less than 5")
    }

    let number_b = number_a >= 5;
    if number_b {
        println!("Number greather than 5")
    }

    // if, else if, else
    let number_c = 1;
    if number_c == 2 {
        println!("number_b adalah 2");
    } else if number_c < 2 {
        println!("number_b adalah dibawah 2");
    } else {
        println!("number_b adalah di atas 2");
    }

    // nested if
    let number_d = 18;
    if number_d > 6 {
        println!("selamat, anda lulus");

        if number_d == 10 {
            println!("dengan nilai sempurna");
        } else if number_d > 7 {
            println!("dengan nilai baik");
        } else {
            println!("dengan nilai cukup");
        }
    } else {
        println!("anda tidak lulus");

        if number_d < 4 {
            println!("belajar lagi ya");
        } else {
            println!("jangan malas belajar!");
        }
    }

    // menulis kondisi pada variabel
    // jika menulis kondisi dengan cara ini, harus single type
    let number_e = 5;
    let result_number = if number_e == 5 { true } else { false };
    println!("result number_d is {result_number}");

    // bisa juga menulis kondisi return dari if, sama persi seperti yang diatas
    let number_f = 3;
    let result_g: bool;

    if number_f == 2 {
        result_g = true
    } else {
        result_g = false
    }
    println!("result_g adalah {result_g}");

    // ini akan error karena bukan single typem terdapat 2 type yaitu str dan bool
    // let result_number = if number_d == 5 { "Sucees" } else { false };
}