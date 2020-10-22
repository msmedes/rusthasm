use std::error::Error;
use std::fs;

#[derive(Debug)]
pub enum CommandType {
    A,
    C,
    L,
    NONE,
}

struct CInstruction {
    comp: String,
    dest: String,
    jump: String,
}

#[derive(Debug)]
pub struct Parser<'a> {
    file_path: String,
    pub line_number: i32,
    file: Vec<&'a str>,
    pub current_command_type: CommandType,
    symbol: &'a str,
    comp: &'a str,
    dest: &'a str,
    jump: &'a str,
    pub instruction_counter: i32,
}

impl<'a> Parser<'a> {
    pub fn new(file_path: String) -> Result<Parser<'a>, Box<dyn Error>> {
        let file = match load_file(&file_path) {
            Ok(file) => file,
            Err(e) => return e,
        };
        Ok(Parser {
            file_path,
            line_number: -1,
            file,
            instruction_counter: 0,
            current_command_type: CommandType::NONE,
            symbol: "",
            comp: "",
            dest: "",
            jump: "",
        })
    }

    // pub fn reset(self) {
    //     self.line_number = -1;
    // }
}

fn load_file(file_path: &String) -> Result<Vec<&str>, Box<dyn Error>> {
    let file_contents = fs::read_to_string(file_path)?;
    Ok(file_contents.lines().collect())
}

fn process_line(line: &str) -> &str {
    let index = line.find(' ');
    match index {
        Some(index) => &line[index..],
        None => line,
    }
}
