/*
    Keyword self
    - Keyword self merepresentasikan current module scope, kita bisa mengakses item yang deklarasinya ada pada module scope yang sama
    - Bisa juga digunakan untuk receiver method
    - Keyword ini untuk mengakses item dalam module

    Crate Root
    - Apapun yang didefinisikan di file entry point crate (yaitu src/main.rs untuk binary dan src/lib.rs untuk library crate)
    - Crate root adalah module scope yang berada di root (paling atas)
    - Apapun yang berada di crate root berati dalam module scope yang sama

    Keyword crate
    - Keyword ini digunakan untuk mengakses apapun yang ada di crate root

    Keyword super
    - Digunakan untuk mengakses parent module scope atau 1 scope di atas current module scope
 */

fn my_func() {
    println!("call `my_func()`");
}

mod module_a {

    // path item ➜ `module_a::my_func`.
    pub fn my_func() {
        println!("call `module_a::my_func()`");
        // keyword super bisa akses 1 ke atas, diatas module_a yaitu pada crate root
        super::my_func();
        super::module_b::submodule_b_one::my_func();
        // begitupun dengan keyword crate bisa akses pada root
        crate::my_func();
    }
}

mod module_b {
    // path item ➜ `module_b::my_func`.
    // fungsi ini tidak publik, jadi hanya bisa diakses dalam scope module `module_b` saja.
    fn my_func() {
        println!("call `module_b::my_func()`");
    }

    // path item ➜ `module_b::run_all_funcs`.
    pub fn run_all_funcs() {
        // semua fungsi yang didefinisikan akan di call dalam blok kode ini.
        println!("call `module_b::run_all_funcs()`");

        // keyword `self` merepresentasikan current module scope.
        // karena berada pada module_b, maka bisa akses yang ada di module_b
        my_func();
        self::my_func();
        self::submodule_b_one::my_func();
        self::submodule_b_two::my_func();

        // keyword `super` di sini mengarah 1 atas dari module_b yaitu crate root atau scope paling luar
        super::my_func(); // my_func()
        super::module_a::my_func();

        // module `submodule_b_two` bisa diakses menggunakan self ataupun tidak
        // karena module tersebut merupakan item yang deklarasinya 1 scope dengan fungsi ini,
        // 1 level dengan `run_all_funcs`.
        submodule_b_two::my_func();
        self::submodule_b_two::my_func();
    }

    // path item ➜ `module_b::submodule_b_one`.
    pub mod submodule_b_one {
        // path item ➜ `module_b::submodule_b_one::my_func`.
        pub fn my_func() {
            println!("call `module_b::submodule_b_one::my_func()`");
        }
    }

    // path item ➜ `module_b::submodule_b_two`.
    pub mod submodule_b_two {
        // path item ➜ `module_b::submodule_b_two::my_func`.
        pub fn my_func() {
            println!("call `module_b::submodule_b_two::my_func()`");

            // current module scope adalah module `submodule_b_two`.
            // keyword `super` di sini mengarah ke parent scope, yaitu `module_b`
            super::my_func(); // module_b::my_func()
            super::submodule_b_one::my_func();

            // keyword crate bisa mengakses semua dari root
            crate::my_func(); // my_func()
            crate::module_a::my_func();
            crate::module_b::submodule_b_one::my_func();
        }
    }
}

fn main() {
    module_b::submodule_b_two::my_func();
}
