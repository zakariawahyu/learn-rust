/*
    Best practice penanganan error di Rust adalah dengan mengkombinasikan keyword match dengan tipe data Result<T, E>
    Rust tidak mengenal konsep exception

    Result <T, E>
    - Tipe data yang digunakan untuk menampung nilai yang isinya bisa berupa operasi sukes Ok atau Err
    - Dibagi menjadi 2 enum value
        ~ Result::Ok<T> (atau Ok<T>) digunakan untuk menandakan sukses Ok
        ~ Result::Err<T> (atau Err<T>) digunakan untuk menandakan Error

     Pattern matching
     - Ketika ada data bertipe Result artinya data tersebut berpotensi nilai Err<E> atau Ok<T>
     - Bisa menggunakan keyword match
 */

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    InfinityNumber,
    OtherError,
}

fn main() {
    let result = divider(10.0, 5.0);
    println!("result: {:?}", result);

    let result_1 = divider(10.0, 0.0);
    println!("result_1: {:?}", result_1);

    let ressult_3 = divider(5.0, 0.0);
    match ressult_3 {
        Ok(2.0) => println!("The result is 2"),
        Ok(divider) => println!("Result: {:?}", divider),
        Err(MathError::DivisionByZero) => println!("Error: Unnable to divide by zero"),
        Err(MathError::InfinityNumber) => println!("Error: Result is infinity number"),
        Err(_) => println!("Unknow error"),
    }

    // Method tipe data Result
    // 1. method .is_ok dan .unwarp()
    // Isi dari enum value Ok<T> bisa diakses tanpa menggunakann match, dengan cara unwarp
    // Sebelum mengakses method .unwarp() harus melakukan pengecekan dulu apakah Ok atau Err
    // Jika data berisi Err, maka pengaksesan .unwarp() akan menghasilkan error
    let result_4 = divider(20.0, 10.0);
    if result_4.is_ok() {
        println!("Result is {}", result_4.as_ref().unwrap());
    }

    // 2. method .as_ref()
    // untuk mengakses reference T dan E pada Result<T, E>
    // dibutuhkan untuk menghindari terjadinya move semantics pada owner tipe Result
    let result_5: Result<f64, MathError> = divider(10.0, 0.0);
    let result_borrow: Result<&f64, &MathError> = result_5.as_ref();
    println!("Result 5: {:?}", result_borrow);

    // 3. methosd .is_err() & .err()
    // sama seperti halnya .unwarp(), harus melakukan pengecekan berisi .is_err() atau tidak
    // karenan jika tidak, maka pengaksesan method .err() akan error
    let result_6 = divider(2.0, 0.0);
    if result_6.is_err() {
        println!("Result is {:?}", result_6.as_ref().err().unwrap());
    }
    // menggunakan method .as_ref() sebelum .err() agar ownership tidak berpindah move semantics
    // maka setelah ini variabel masih bisa di akses
    println!("Result 6: {:?}", result_6);

    // 4. method .ok()
    // Method as_ref harus diakses terlebih dahulu sebelum memanggil method ok agar tidak terjadi move semantics.
    let result_7 = divider(2.0, 1.0);
    if result_7.is_ok() {
        println!("Result is {:?}", result_7.as_ref().ok().unwrap());
    }

    // 5. method .unwarp_or_default()
    // mengembalikan nilai T jika data berisi Ok<T>, namun jika berisi Err maka yang dikembalikan adalah default value dari tipe data T
    let result_8 = divider(2.0, 0.0);
    let number_8 = result_8.unwrap_or_default();
    println!("Result 8: {:?}", number_8);

    // 6. method .unwrap_or()
    // mengembalikan nilai T jika data berisi Ok<T>, namun jika berisi Err maka yang dikembalikan adalah argument pemanggilan method tersebut.
    let result_9 = divider(2.0, 0.0);
    let number_9 = result_9.unwrap_or(0.0);
    println!("Result 9: {:?}", number_9);

    // 7. method .unwrap_or_else()
    // Method ini mengembalikan nilai T ketika data berisi Ok<T>, namun jika data isinya adalah Err<E>,
    // maka yang dikembalikan adalah hasil eksekusi closure yang disisipkan saat memanggil method unwrap_or_else
    let result_10 = divider(2.0, 0.0);
    let number_10 = result_10.unwrap_or_else(|_| 0.0);
    println!("Result 10: {:?}", number_10);

    let result_11 = divide_and_print(10.0, 1.0);
    println!("result_11: {:?}", result_11);
}

fn divider (a :f64, b:f64) -> Result<f64, MathError> {
    if b == 0.0 {
        return Err(MathError::DivisionByZero)
    }

    let result = a / b;
    return Ok(result);
}

/*
    Tipe Result<(), E>
    - dengan T diisi tipe data ()
    - tipe ini cukup sering digunakan pada fungsi yang memiliki potensi error tapi kita hanya butuh informasi errornya saja tanpa nilai balik lainnya.
 */

// Jika sukses, nilainya langsung di-print; jika error, nilai errornya dikembalikan
fn divide_and_print(a: f64, b: f64) -> Result<(), MathError> {
    let res = divider(a, b);
    match res {
        Err(m) => {
            println!("ERROR! {:?}", m);
            Err(m)
        },
        Ok(n) => {
            println!("result: {}", n);
            Ok(())
        },
    }
}