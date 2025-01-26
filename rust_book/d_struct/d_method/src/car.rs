#[derive(Debug)]
pub struct Car{
    brand: String,
    model: String,
    manufactur_year: i32
}

impl Car{
    pub fn new(brand: String, model: String) -> Car{
        Car{brand, model, manufactur_year:0}
    }

    // jika parameter pertama self, maka ini disebut method
    pub fn info(&self) -> String{
        if self.manufactur_year == 0{
            format!("{} model {}", self.brand, self.model)
        } else {
            format!("{} model {}, manufactur at {}", self.brand, self.model, self.manufactur_year)
        }
    }

    // pemanggilan method didalam method
    pub fn congratulations(&self, name: String){
        println!("Congratulations {}", name);
        println!("Your new car is {}", self.info());
    }

    // nilai properti struct bisa di ubah dari dalam method, harus menggunakan mutable reference
    pub fn set_manufactur_year(&mut self, manufactur_year: i32) {
        self.manufactur_year = manufactur_year
    }
}