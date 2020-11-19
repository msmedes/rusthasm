mod code;
mod parser;
mod symbol;

fn main() {
    let symbol_table = symbol::Table::new();
    println!("{:?}", symbol_table);
    let code = code::Code::new();
    println!("{:?}", code);
    let parser = parser::Parser::new(String::from("test.txt"));
    println!("{:?}", parser);
}
