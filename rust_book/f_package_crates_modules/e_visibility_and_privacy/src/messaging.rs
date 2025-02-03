// karena mod service_layer private, kali ini menggunakan re-exported agar bisa mengakses some_black_magic dari luar module
pub use self::service_layer::some_black_magic;
const SOME_MESSAGE: &str = "Hello, world!";

mod service_layer {
    pub fn some_black_magic() {
        println!("{}", crate::messaging::SOME_MESSAGE);
    }
}

pub fn say_hello() {
    service_layer::some_black_magic();
}

/*
    Keyword pub(in path)
    - Keyword ini menjadikan visibility item hanya di dalam path yang ditulis di pub(in path),
    dengan ketentuan path tersebut merupaka parent dari module item dimana keyword digunakan

    Keyword pub(crate)
    - Keyword ini digunakan untuk membuat visibility item menjadi publik dengan scope akses current crate
    - Dengan hal ini item bisa di akses dari manapun asalkan masih dalam crate yang sama

    Keyword pub(super)
    - Keyword ini digunakan untuk membuat visibility item menjadi publik dengan scope akses 1 ke atas dari current module

    Keyword pub(self)
    - Keyword ini digunakan untuk membuat visibility item menjadi publik dengan scope akses current module
 */

pub mod outer {
    pub mod inner {
        use  crate::messaging;
        // fungsi say_in_path berikut hanya bisa di akses dari dalam module outer
        // pengaksesan dari luar module akan error
        pub(crate) fn say_in_path() {
            println!("Pub In Path : {}", messaging::SOME_MESSAGE);
        }

        // fungsi ini visibility scope-nya di level crate
        pub(crate) fn say_in_crate() {
            println!("Pub In Crate : {}", messaging::SOME_MESSAGE);
        }

        pub mod sub_inner {
            use crate::messaging;

            // fungsi ini hanya bisa di akses 1 atas dari module sub iner
            // yaitu hanya valid di mod inner
            pub(super) fn say_in_super() {
                println!("Sub Inner : {}", messaging::SOME_MESSAGE);
            }

            // hanya valid pada current module yaitu sub_inner
            pub(self) fn say_in_self() {
                println!("Sub Inner : {}", messaging::SOME_MESSAGE);
            }

            // seperti ini hanya bisa di akses dalam current module
            pub fn do_in_self() {
                say_in_self()
            }
        }

        // seperti ini masih valid untuk keyword super
        pub fn do_in_super() {
            sub_inner::say_in_super()
        }
    }

    pub fn do_in_path() {
        inner::say_in_path()
    }
}

pub mod outer_two {
    pub fn do_in_crate(){
        // masih bisa akses karena dalam 1 crate yang sama
        crate::messaging::outer::inner::say_in_crate()
    }
}