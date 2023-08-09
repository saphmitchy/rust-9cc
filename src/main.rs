use rust_9cc::binary;
use rust_9cc::parse;
use std::env::args;

fn main() {
    let arg: Vec<String> = args().collect();
    let ast = parse::source_to_ast(arg.get(1).unwrap()).unwrap();
    let mut label_counter = 0;
    let operation = ast
        .into_iter()
        .map(|v| v.to_assembly(&mut label_counter))
        .flatten()
        .collect();
    binary::elf_writer(&arg[2], &operation).unwrap();
}
