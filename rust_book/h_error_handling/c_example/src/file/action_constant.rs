pub const FOLDER_BASE_PATH :&str = "files";

pub enum Command {
    PrintFiles,
    CreateFile,
    ReadFile,
    DeleteFile,
    ExitProgram
}

pub fn validate_command(cmd :&str) -> Result<Command, &'static str> {
    match cmd {
        "1" => Ok(Command::PrintFiles),
        "2" => Ok(Command::CreateFile),
        "3" => Ok(Command::ReadFile),
        "4" => Ok(Command::DeleteFile),
        "9" => Ok(Command::ExitProgram),
        _ => Err("Unknown command")
    }
}