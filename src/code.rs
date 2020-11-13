use std::collections::HashMap;

#[derive(Debug)]
pub struct Code<'a> {
    comp_table: HashMap<&'a str, &'a str>,
    dest_table: HashMap<&'a str, &'a str>,
    jump_table: HashMap<&'a str, &'a str>,
}

impl<'a> Code<'a> {
    pub fn new() -> Code<'a> {
        let comp_table: HashMap<&'a str, &'a str> = [
            ("0", "0101010"),
            ("1", "0111111"),
            ("-1", "0111010"),
            ("D", "0001100"),
            ("A", "0110000"),
            ("M", "1110000"),
            ("!D", "0001101"),
            ("!A", "0110001"),
            ("!M", "1110001"),
            ("-D", "0001111"),
            ("-A", "0110011"),
            ("-M", "1110011"),
            ("D+1", "0011111"),
            ("A+1", "0110111"),
            ("M+1", "1110111"),
            ("D-1", "0001110"),
            ("A-1", "0110010"),
            ("M-1", "1110010"),
            ("D+A", "0000010"),
            ("D+M", "1000010"),
            ("D-A", "0010011"),
            ("D-M", "1010011"),
            ("A-D", "0000111"),
            ("M-D", "1000111"),
            ("D&A", "0000000"),
            ("D&M", "1000000"),
            ("D|A", "0010101"),
            ("D|M", "1010101"),
        ]
        .iter()
        .cloned()
        .collect();
        let dest_table: HashMap<&'a str, &'a str> = [
            ("NIL", "000"),
            ("M", "001"),
            ("D", "010"),
            ("MD", "011"),
            ("A", "100"),
            ("AM", "101"),
            ("AD", "110"),
            ("AMD", "111"),
        ]
        .iter()
        .cloned()
        .collect();

        let jump_table: HashMap<&'a str, &'a str> = [
            ("NIL", "000"),
            ("JGT", "001"),
            ("JEQ", "010"),
            ("JGE", "011"),
            ("JLT", "100"),
            ("JNE", "101"),
            ("JLE", "110"),
            ("JMP", "111"),
        ]
        .iter()
        .cloned()
        .collect();
        Code {
            comp_table,
            dest_table,
            jump_table,
        }
    }
}
