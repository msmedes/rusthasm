use std::fs;

#[derive(Debug, PartialEq)]
pub enum Command {
    A,
    C,
    L,
    Null,
}
#[derive(Debug, PartialEq)]
struct CInstruction {
    dest: String,
    comp: String,
    jump: String,
}

impl CInstruction {
    fn new(dest: String, comp: String, jump: String) -> CInstruction {
        CInstruction {
            dest,
            comp,
            jump,
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    file_path: String,
    pub line_number: i32,
    file: Vec<String>,
    pub current_command_type: Command,
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
            current_command_type: Command::Null,
            symbol: String::from(""),
            comp: String::from(""),
            dest: String::from(""),
            jump: String::from(""),
        }
    }

    pub fn reset(&mut self) {
        self.line_number = -1 as i32;
    }

    pub fn has_more_commands(&self) -> bool {
        self.line_number < (self.file.len() as i32) - 1
    }

    fn reset_commands(&mut self) {
        self.current_command_type = Command::Null;
        self.symbol = String::from("");
        self.comp = String::from("");
        self.dest = String::from("");
        self.jump = String::from("");
    }

    fn current_command(&self) -> String {
        self.file[self.line_number as usize].clone()
    }

    fn command_type(&mut self) -> Command {
        self.reset_commands();
        let first_char = self.current_command().chars().next(); // Holy shit ok
        match first_char {
            Some('@') => Command::A,
            Some('(') => Command::L,
            _ => Command::C, 
        }
    }

    fn parse_a_command(&self) -> String {
        self.current_command()[1..].to_owned()
    }

    fn parse_l_command(&self) -> String {
        let len = self.current_command().len();
        self.current_command()[1..len-1].to_owned()
    }

    fn parse_c_command(&self) -> CInstruction {
        // I am not proud of this
        let mut dest = "NONE".to_owned();
        let mut jump = "NONE".to_owned();
        let comp: String;
        let command = self.current_command();
        let equal_index = match command.find('=') {
            Some(index) => index as i32,
            None => -1 as i32,
        };
        let semi_index = match command.find(';') {
            Some(index) => index as i32,
            None => -1 as i32,
        };

        if equal_index != -1 {
            dest = command[..equal_index as usize].to_owned();
        }

        if semi_index != -1 {
            jump = command[(semi_index+1) as usize..].to_owned();
            comp = command[(equal_index+1) as usize..semi_index as usize].to_owned();
        } else {
            comp = command[(equal_index+1) as usize..].to_owned();
        }
        CInstruction::new(dest, comp, jump)
    }
    
    pub fn advance(&mut self) {
        self.line_number += 1;
        self.current_command_type = self.command_type();
        match self.current_command_type {
            Command::A => self.symbol = self.parse_a_command(),
            Command::C => {
                let instruction = self.parse_c_command();
                self.dest = instruction.dest;
                self.comp = instruction.comp;
                self.jump = instruction.jump;
            }
            Command::L => self.symbol = self.parse_l_command(),
            _ => panic!("command type cannot be null"),
        }
    }

    pub fn comp(&self) -> String {
        match self.current_command_type {
            Command::C => self.comp.clone(),
            _ => panic!("comp command can only be returned for C commands"),
        }
    }

    pub fn dest(&self) -> String {
        match self.current_command_type {
            Command::C => self.dest.clone(),
            _ => panic!("dest command can only be returned by C commands"),
        }
    }

    pub fn jump(&self) -> String {
        match self.current_command_type {
            Command::C => self.jump.clone(),
            _ => panic!("jump command can only be returned by C commands"),
        }
    }

    pub fn symbol(&self) -> String {
        match self.current_command_type {
            // I guess this is the best way to do this in rust?
            Command::C => panic!("symbol command can only be returned by A or L commands"),
            _ => self.symbol.clone(),
        }
    }
}

fn load_file(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Can't read file");
    contents.lines().map(|s: &str| s.to_owned()).collect()
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
        assert_eq!(file_contents, vec!["@OUTPUT_D",
            "   0;JMP            // goto output_d",
         "(OUTPUT_FIRST)",]);
    }

    #[test]
    #[should_panic(expected = "No such file")]
    fn load_unexisting_file() {
        let _ = load_file(String::from("nosuchfile.txt"));
    }

    #[test]
    fn has_more_commands() {
        let parser = Parser::new(String::from("test.txt"));
        let more_commands = parser.has_more_commands();
        assert_eq!(true, more_commands);
    }

    #[test]
    fn no_more_commands() {
        let mut parser = Parser::new("test.txt".to_owned());
        parser.line_number = (parser.file.len() as i32) - 1;
        let more_commands = parser.has_more_commands();
        assert_eq!(more_commands, false);
    }

    #[test]
    fn find_a_command() {
        let mut parser = Parser::new("test.txt".to_owned());
        parser.line_number = 0;
        let result = parser.command_type();
        assert_eq!(result, Command::A);
    }
    
    #[test]
    fn find_c_command() {
        let mut parser = Parser::new("test.txt".to_owned());
        parser.line_number = 1;
        let result = parser.command_type();
        assert_eq!(result, Command::C);
    }

    #[test]
    fn find_l_command() {
        let mut parser = Parser::new("test.txt".to_owned());
        parser.line_number = 2;
        let result = parser.command_type();
        assert_eq!(result, Command::L);
    }

    #[test]
    fn parse_c_command_equals_only() {
        let mut parser = Parser::new("c_commands.txt".to_owned());
        parser.line_number = 0;
        let result = parser.parse_c_command();
        let desired = CInstruction::new("D".to_owned(), "D-M".to_owned(), "NONE".to_owned());
        assert_eq!(result, desired);
    }

    #[test]
    fn parse_c_command_semi_and_equals() {
        let mut parser = Parser::new("c_commands.txt".to_owned());
        parser.line_number = 1;
        let result = parser.parse_c_command();
        let desired = CInstruction::new("A".to_owned(), "M+1".to_owned(), "JMP".to_owned());
        assert_eq!(result, desired);
    }

    #[test]
    fn parse_c_command_no_equals() {
        let mut parser = Parser::new("c_commands.txt".to_owned());
        parser.line_number = 2;
        let result = parser.parse_c_command(); 
        let desired = CInstruction::new("NONE".to_owned(), "0".to_owned(), "JMP".to_owned());
        assert_eq!(result, desired)
    }
}
