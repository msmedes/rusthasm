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
    file: Vec<String>,
    pub current_command_type: CommandType,
    symbol: &'a str,
    comp: &'a str,
    dest: &'a str,
    jump: &'a str,
    pub instruction_counter: i32,
}

impl<'a> Parser<'a> {
    pub fn new(file_path: String) -> Parser<'a> {
        let file = load_file(file_path.clone());
        Parser {
            file_path,
            line_number: -1,
            file,
            instruction_counter: 0,
            current_command_type: CommandType::NONE,
            symbol: "",
            comp: "",
            dest: "",
            jump: "",
        }
    }
}

fn load_file(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Can't read file");
    contents.lines().map(|s: &str| s.to_string()).collect()
}

fn process_line(line: &str) -> &str {
    let index = line.find(' ');
    match index {
        Some(index) => &line[index..],
        None => line,
    }
}
