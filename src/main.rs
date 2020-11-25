mod assembler;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut a = assembler::Assembler::new(file_path.to_string());
    a.assemble();
    a.write_to_file();
}
