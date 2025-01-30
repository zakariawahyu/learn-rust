use std::io::stdin;

// Function untuk embaca inputan dari user kemudian return dalam bentuk string
// Keyword pub digunakan untuk menjadikan suatu item menjadi public, agar bisa di akses dari luar module
pub fn read_entry() -> String {
    let mut message = String::new();
    let stdin_reader = stdin();
    let reader_res = stdin_reader.read_line(&mut message);

    if reader_res.is_err() {
        println!("Error reading from STDIN: {}", reader_res.err().unwrap());
    }

    message.trim().to_string()
}