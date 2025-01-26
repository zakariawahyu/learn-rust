mod car;
/*
    Method
    - Method adalah associated function yang hanya bisa di akses lewat instance atau objek
    - Berbeda dengan associated function yang pengaksesan fungsinya via tipe data struct
    - Cara deklarasi method mirip dengan associated function, perbedaanya adalah parameter pertama harus di isi dengan self
    - Parameter tersebut menjadi identifier apakah fungsi tersebut merupakan associated function atau method
    - Penamaan method harus snack case
 */
fn main() {
    let mut new_car : car::Car = car::Car::new(
        String::from("Mercedes-Benz"),
        String::from("Vision Gran Turismo")
    );
    println!("New Car : {:#?} ", new_car);

    let info = new_car.info();
    println!("info : {:?} ", info);

    new_car.congratulations(String::from("Zakaria"));

    new_car.set_manufactur_year(2025);
    let detail_info = new_car.info();
    println!("detail_info : {:?}", detail_info);
}
