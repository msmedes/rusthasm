use std::fs;

#[derive(Debug)]
pub enum CommandType {
    A,
    C,
    L,
    Null,
}

struct CInstruction {
    comp: String,
    dest: String,
    jump: String,
}

#[derive(Debug)]
pub struct Parser {
    file_path: String,
    pub line_number: i32,
    file: Vec<String>,
    pub current_command_type: CommandType,
    symbol: String,
    comp: String,
    dest: String,
    jump: String,
    pub instruction_counter: i32,
}

impl Parser {
    pub fn new(file_path: String) -> Parser {
        let file = load_file(file_path.clone());
        Parser {
            file_path,
            line_number: -1,
            file,
            instruction_counter: 0,
            current_command_type: CommandType::Null,
            symbol: String::from(""),
            comp: String::from(""),
            dest: String::from(""),
            jump: String::from(""),
        }
    }
}

fn load_file(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Can't read file");
    contents.lines().map(|s: &str| s.to_string()).collect()
}

fn process_line(line: String) -> String {
    let line = line.trim();
    if line.starts_with('/') || line == "\n" {
        return String::from("")
    }
    let index = line.find(' ');
    match index {
        Some(index) => String::from(&line[..index]),
        None => String::from(line),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_at_command(){
        let line = String::from(" @RO ");
        let result = String::from("@RO");
        assert_eq!(result, process_line(line));
    }

    #[test]
    fn process_strip_comment() {
        let line = String::from("D;JGT            // if D>0 (first is greater) goto output_first");
        let result = String::from("D;JGT");
        assert_eq!(result, process_line(line));
    }

    #[test]
    fn process_empty_line() {
        let line = String::from("");
        let result = String::from("");
        assert_eq!(result, process_line(line));
    }

    #[test]
    fn process_new_line() {
        let line = String::from("\n");
        let result = String::from("");
        assert_eq!(result, process_line(line));
    }

    #[test]
    fn load_existing_file() {
        let file_contents = load_file(String::from("test.txt"));
        assert_eq!(file_contents, vec!["foo","bar","baz","bax",])
    }

    #[test]
    #[should_panic(expected = "No such file")]
    fn load_unexisting_file() {
        let _ = load_file(String::from("nosuchfile.txt"));
    }
}