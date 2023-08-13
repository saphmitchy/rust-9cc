use rust_9cc::ast::GenAssembly;
use rust_9cc::binary;
use rust_9cc::parse;
use std::env::args;

fn main() {
    let arg: Vec<String> = args().collect();
    let ast = parse::source_to_ast(arg.get(1).unwrap()).unwrap();
    let mut label_counter = 0;
    let mut operation = vec![];
    for a in ast {
        a.to_assembly(&mut operation, &mut label_counter);
    }
    binary::elf_writer(&arg[2], &operation).unwrap();
}
