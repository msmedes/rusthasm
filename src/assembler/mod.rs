mod parser;
mod code;
mod symbol;

use std::convert::TryInto;

pub struct Assembler {
    file_path: String,
    parser: parser::Parser,
    code: code::Code,
    symbol: symbol::SymbolTable,
    buffer: Vec<String>,
    assembled: bool,
}

impl  Assembler {
    pub fn new(file_path: String) -> Assembler {
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
        // let binary: String;
        let symbol = self.parser.symbol();
        let address = symbol.parse::<u16>();
        let binary = match address {
            Ok(address) => format!("{:#16b}", address),
            Err(_) => {
                let address = match self.symbol.get_addr(&symbol) {
                    Some(address) => address,
                    None => self.symbol.add_variable(symbol.clone()),
                };
                format!("{:#16b}", address)
            }
        };
        self.buffer.push(binary);
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
                    if !self.symbol.contains(symbol.clone()) {
                        self.symbol.add_entry(symbol, self.parser.instruction_counter.try_into().unwrap());
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