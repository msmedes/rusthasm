mod parser;
mod code;
mod symbol;

pub struct Assembler<'a> {
    file_path: String,
    parser: parser::Parser,
    code: code::Code<'a>,
    symbol: symbol::SymbolTable<'a>,
    pub buffer: Vec<String>,
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
}