/*
    Generic
    - Digunakan untuk menambah fleksibilitas dalam pemanfaatan tipe data pada suatu blok
    - Bisa menentukan tipe data yang digunakan pada parameter maupun return value sebuah blok fungsi, method dan lainnya
    - Generic dinotasikan dengan <T>
    - Contoh pada vector dimana pendefinisian tipe data harus dituliskan juga tipe data item (via generic parameter)
    - Contoh Vec<i32>, Vec<&str>

    Generic Struct
    - Selain di terapkan pada function, generic bisa juga diterapkan pada struct
    - Cara penulisan menambahkan notasi parameter generic di antara nama struct dan block struct
    - contoh struct nama_struct<T, U> {}

    Generic Method
    - Notasi penulisanya kurang lebih sama seperti pada penulisan method
    - Hanya pada syntax impl perlu di ikuti blok paramter genericnya

    Generic Enum
    - Cara penulisan setalah nama enum, kemudian gunakan parameter generic sesuai kebutuhan
 */

// pada function ini terdapat 1 parameter argument dan 1 parameter generic
// parameter 1 bertipe i32 dan parameter 2 bertipe T adalah parameter generic
// pemanggilan functinya bisa juga ditentukan tipe data genericnya dengan do_someting::<tipe_data>
fn do_someting<T>(arg1: i32, arg2: T){
    println!("Do someting");
}

// parameter generic tidak ada batasanya
// function ini memiliki 2 buah parameter generic, yaitu R dan T.
fn do_something_v2<R,T>(arg1: R, arg2: T){
    println!("Do someting v2");
}

struct Point<T, U>{
    x: T,
    y: T,
    z: U
}

// jika struct mempunyai 3 parameter generic, maka pada syntax impl juga harus sama yaitu impl<T, U>
impl <T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }

    fn get_z(&self) -> &U {
        &self.z
    }
}

// selain itu bisa mendefinisikan method hanya untuk tipe parameter generic
// Misalnya, method hanya bisa diakses ketika T adalah i32 dan U adalah f64
impl Point<i32, f64> {
    fn get_x_2(&self) -> &i32 {
        &self.x
    }

    fn get_y_2(&self) -> &i32 {
        &self.y
    }

    fn get_z_2(&self) -> &f64 {
        &self.z
    }
}

// contoh enum kendaraan tipe generic T digunakan pada value enum Yamaha(T)
enum Kendaraan<T> {
    Honda,
    Yamaha(T),
    Suzuki
}

fn main() {
    // dalam pemanggilan fungsinya, tipe generic data argumen harus sama
    do_someting::<bool>(42, false);
    // do_someting::<bool>(42, "Helo"); jika seperti ini program akan error

    // Tipe data generic boleh tidak ditulis karena nilai T bisa diketahui dari argument pemanggilan fungsi.
    do_someting(42, "Hello");
    do_someting(42, 45.5);

    // Sedangkan jika nilai T tidak digunakan sebagai tipe data parameter, maka wajib untuk di-isi nilai T saat pemanggilan fungsi.
    do_someting::<i32>(42, 42);

    // struct Point<T, U> bisa digunakan dalam banyak kombinasi tipe data, misalnya:
    let num_one:Point<i32, bool> = Point{x: 502, y: 120, z: true};
    println!("{} {} {}", num_one.get_x(), num_one.get_y(), num_one.get_z());
    let num_two: Point<f64, i32> = Point { x: 1.2, y: 4.3, z: 534 };
    println!("{} {} {}", num_two.get_x(), num_two.get_y(), num_two.get_z());
    let num_three: Point<f32, &str> = Point { x: 1.2, y: 1.3, z: "Hello" };
    println!("{} {} {}", num_three.get_x(), num_three.get_y(), num_three.get_z());

    // jika selain tipe i32 dan f64 akan error
    let num_four:Point<i32, f64> = Point { x: 12, y: 13, z: 1.4 };
    println!("{} {} {}", num_four.get_x_2(), num_four.get_y_2(), num_four.get_z_2());

    let kendaraan1 = Kendaraan::<i32>::Honda;
    let kendaraan2 = Kendaraan::<&str>::Yamaha("Vixion");
}
