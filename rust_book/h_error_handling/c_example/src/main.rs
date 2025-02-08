mod file;

fn run_program() -> Result<(), String> {
    println!("Welcome to File Maker app!");

    loop {
        println!();
        println!("Available commands: ");
        println!("1. Print Files");
        println!("2. Create Files");
        println!("3. Read Files");
        println!("4. Delete Files");
        println!("9. Quit or Exit");
        println!();

        print!("Enter your command: ");

        // error handling use operator ?
        let _ = file::utility::stdout_flush()?;

        // error handling use guard method
        let user_entry = match file::utility::read_entry() {
            Ok(entry) => entry,
            Err(error) => {
                println!("ERROR, unable to continue the program");
                continue;
            },
        };

        // error handling using basic implementation
        let cmd_result = file::action_constant::validate_command(&user_entry);
        match cmd_result {
            _ => {},
            Err(err) => {
                println!("ERROR, {}", err);
                continue;
            }
        }
        let cmd = cmd_result.unwrap();

        // check command
        // error handling using operator ?
        match cmd {
            file::action_constant::Command::PrintFiles => {
                println!("Printing files...");
                file::action::print_files()?;
            },
            file::action_constant::Command::CreateFile => {
                println!("Creating file...");
                file::action::create_file()?;
            },
            file::action_constant::Command::ReadFile => {
                println!("Reading file...");
                file::action::read_file()?;
            },
            file::action_constant::Command::DeleteFile => {
                println!("Deleting file...");
                file::action::delete_file()?;
            },
            file::action_constant::Command::ExitProgram => {
                println!("Program exited!");
                return Ok(())
            }
        }
    }
}

fn main() {
    match run_program() {
        Ok(_) => {},
        Err(error) => {
            panic!("ERROR, {}", error);
        }
    }
}
