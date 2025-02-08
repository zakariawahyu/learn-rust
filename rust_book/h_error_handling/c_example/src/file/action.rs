use std::fs::{create_dir, read_dir, write, read_to_string, remove_file};
use std::path::Path;
use crate::file::action_constant;
use crate::file::utility;


// melakukan pengecekan pembuatan folder FOLDER_BASEPATH (jika belum ada), dan menampilkan list file di dalam folder tersebut
// Jika tidak ada file, pesan No files dimunculkan ke layar.
pub fn print_files() -> Result<(), String> {
    let path = Path::new(action_constant::FOLDER_BASE_PATH);

    // if files folder not exist, create it
    if !path.is_dir() {
        match create_dir(path) {
            Ok(_) => {},
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }

    let dir = match read_dir(path){
        Ok (dir) => dir,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let mut count = 0;
    for file in dir {
        count = count + 1;
        
        match file {
            Ok(entry) => {
                println!(" -> {:?}", entry.path());
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }

    if count == 0 {
        println!("No files");
    }

    Ok(())
}

// Operasi create file dilakukan dengan melibatkan I/O dengan user, untuk file name dan isi content
pub fn create_file() -> Result<(), String> {
    let path = Path::new(action_constant::FOLDER_BASE_PATH);

    // if files folder not exists, create it
    if !path.is_dir() {

        // error handling using basic implementation of keyword match
        match create_dir(path) {
            Err(err) => {
                return Err(err.to_string());
            },
            Ok(_) => {},
        }
    }

    print!("Enter filename : ");
    let _ = utility::stdout_flush()?;

    let filename = match utility::read_entry() {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok (txt) => txt,
    };

    println!("Enter file content : ");
    let _ = utility::stdout_flush()?;

    let content = match utility::read_entry() {
        Err(err) => {
            return Err(err.to_string());
        },
        Ok (txt) => txt,
    };

    match write(path.join(filename), content) {
        Ok(()) => {},
        Err(err) => {
            return Err(err.to_string());
        }
    }

    Ok(())
}

// Operasi baca file dilakukan dengan melibatkan I/O dengan user, input nama file untuk mendapatkan data.txt file
pub fn read_file() -> Result<(), String> {
    let path = Path::new(action_constant::FOLDER_BASE_PATH);

    print!("Enter filename : ");
    let _ = utility::stdout_flush()?;

    let filename = match utility::read_entry() {
        Ok (txt) => txt,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let content = match read_to_string(path.join(filename)) {
        Ok (content) => content,
        // ketika terjadi error, Err() tidak dikembalikan, melainkan menggunakan Ok()
        // memunculkan pesan errornya ke layar secara friendly, agar program tetap bisa running sesuai flow
        Err(err) => {
            println!("Error, {:?}", err.to_string());
            return Ok(())
        }
    };

    println!("File Content : {}", content);

    Ok(())
}

pub fn delete_file() -> Result<(), String> {
    let path = Path::new(action_constant::FOLDER_BASE_PATH);

    print!("Enter filename : ");
    let _ = utility::stdout_flush()?;

    let filename = match utility::read_entry() {
        Ok (txt) => txt,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    match remove_file(path.join(filename)) {
        Err(err) => {
            println!("ERROR. {:?}", err.to_string());
            return Ok(());
        },
        Ok(_) => {
            println!("File deleted");
        },
    }

    Ok(())
}