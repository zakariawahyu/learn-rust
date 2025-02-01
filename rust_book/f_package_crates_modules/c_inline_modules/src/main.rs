/*
    Inline Module
    - Inline module adalah cara lain dalam pendefinisian module beserta itemnya
    - Caranya tetap menggunakan keyword mod, hanya saja item ditulis di dalam blok kode mod
    - Inline module juga bisa digunakan dalam nama_module.rs atau nama_module/mod.rs
 */

mod test_module {
    pub fn fungsi_satu() {}

    pub mod sub_module_test {
        pub fn fungsi_sub_modulle_satu() {}
    }
}

mod utilities {
    pub mod random {
        pub fn string(length: u32) -> String {
            use rand::Rng;

            const CHARSET: &[u8] = "abcdefghijklmnopqrstuvwxyz".as_bytes();
            let mut arr = Vec::new();

            for _ in 0..=length {
                let mut rng = rand::thread_rng().gen_range(0..(CHARSET.len()));
                let char = CHARSET[rng];
                arr.push(char);
            }

            std::str::from_utf8(&arr[..]).unwrap().to_string()
        }
    }

    pub mod password {
        pub fn hash_password(text: &str) -> String {
            bcrypt::hash(text, bcrypt::DEFAULT_COST).unwrap()
        }

        pub fn is_valid(password: &str, hash: &str) -> bool {
            bcrypt::verify(password, hash).unwrap()
        }
    }
}

// module_a.rs
mod module_a;

// module_b/mod.rs
mod module_b;

// pkg/utils.rs
// pkg/helper.rs
mod pkg {
    pub mod utils;
    pub mod helper;
}

fn main() {
    let password = format!("hello world {}", utilities::random::string(10));
    println!("raw password : {}", password);

    let hashed = utilities::password::hash_password(&password);
    println!("hashed password : {}", hashed);

    let is_valid = utilities::password::is_valid(&password, &hashed);
    println!("is_valid password? {}", is_valid);

    module_a::check_module_a::check_this_a();
    module_a::test_module_a::check_this_a();

    module_b::check_module_b::check_this_b();
}
