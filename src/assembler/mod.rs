mod parser;
mod code;
mod symbol;

use std::convert::TryInto;
use std::fs::File;
use std::io::Write;

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
            self.parser.advance(false);
            match self.parser.current_command_type {
                parser::Command::A => self.process_a_command(),
                parser::Command::C => self.process_c_command(),
                _ => (),
            }
        }
        self.assembled = true;
    }

    fn process_a_command(&mut self) {
        // let binary: String;
        let symbol = self.parser.symbol();
        let address = symbol.parse::<u16>();
        let binary = match address {
            Ok(address) => format!("{:016b}", address),
            Err(_) => {
                let address = match self.symbol.get_addr(&symbol) {
                    Some(address) => address,
                    None => self.symbol.add_variable(symbol.clone()),
                };
                format!("{:016b}", address)
            }
        };
        self.buffer.push(binary);
    }

    fn process_c_command(&mut self) {
        let parser_dest = self.parser.dest();
        let parser_comp = self.parser.comp();
        let parser_jump = self.parser.jump();
        let dest = self.code.dest(parser_dest);
        let comp = self.code.comp(parser_comp);
        let jump = self.code.jump(parser_jump);
        let command = format!("111{}{}{}", comp,dest,jump);
        self.buffer.push(command);
    }

    fn process_l_commands(&mut self) {
        while self.parser.has_more_commands() {
            self.parser.advance(true);
            match self.parser.current_command_type {
                parser::Command::L => {
                    let symbol = self.parser.symbol();
                    if !self.symbol.contains(symbol.clone()) {
                        self.symbol.add_entry(symbol, self.parser.instruction_counter.try_into().unwrap());
                    }
                }
                _ => self.parser.instruction_counter += 1
            }
        }
        self.parser.reset();
    }

    pub fn write_to_file(&self){
        let filename = self.file_path.strip_suffix(".asm").unwrap();
        let filename = format!("{}.hack", filename);
        let mut file = File::create(filename).expect("unable to create file");
       for i in &self.buffer {
           writeln!(file, "{}", i).unwrap();
       }
    }
}