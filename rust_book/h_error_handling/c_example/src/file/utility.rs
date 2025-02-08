use std::io;
use std::io::Write;

// fungsi untuk membaca inputan user
pub fn read_entry() -> Result<String, String> {
    let mut message = String::new();
    let mut header = io::stdin().read_line(&mut message);

    // error handling using guard method
    let content = match header {
        Ok(_) => message.trim().to_string(),
        Err(err) => {
            return Err(err.to_string());
        },
    };

    Ok(content)
}

// fungsi untuk mem-flush output stdout
pub fn stdout_flush() -> Result<(), String> {
    // error handling using basic implpementataion
    match io::stdout().flush() {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}