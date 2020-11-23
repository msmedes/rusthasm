mod parser;
mod code;
mod symbol;

use std::convert::TryInto;

pub struct Assembler<'a> {
    file_path: String,
    parser: parser::Parser,
    code: code::Code<'a>,
    symbol: symbol::SymbolTable<'a>,
    buffer: Vec<String>,
    assembled: bool,
}

impl <'a> Assembler<'a> {
    pub fn new(file_path: String) -> Assembler<'a> {
        Assembler {
            file_path: file_path.clone(),
            parser: parser::Parser::new(file_path),
            code: code::Code::new(),
            symbol: symbol::SymbolTable::new(),
            buffer: vec![],
            assembled: false,
        }
    }

    pub fn assemble(&mut self) {
        self.process_l_commands();
        while self.parser.has_more_commands() {
            self.parser.advance();
            match self.parser.current_command_type {
                parser::Command::A => self.process_a_command(),
                parser::Command::C => self.process_c_command(),
                _ => panic!("invalid command type"),
            }
        }
        self.assembled = true;
    }

    fn process_a_command(&mut self) {
        // let symbol = self.parser.symbol();
        todo!();
    }

    fn process_c_command(&mut self) {
        let parser_dest = self.parser.dest();
        let parser_comp = self.parser.comp();
        let parser_jump = self.parser.jump();
        let dest = self.code.dest(&parser_dest);
        let comp = self.code.comp(&parser_comp);
        let jump = self.code.jump(&parser_jump);
        let command = format!("111{}{}{}", comp,dest,jump);
        self.buffer.push(command);
    }

    fn process_l_commands(&mut self) {
        while self.parser.has_more_commands() {
            self.parser.advance();
            match self.parser.current_command_type {
                parser::Command::L => {
                    let symbol = self.parser.symbol();
                    if !self.symbol.contains(&symbol) {
                        self.symbol.add_entry(&symbol.clone(), self.parser.instruction_counter.try_into().unwrap());
                    }
                }
                _ => self.parser.instruction_counter += 1
            }
            self.parser.reset();
        }
    }

    pub fn write_to_file(){
        todo!();
    }
}