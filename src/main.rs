use rust_9cc::{elf_writer, source_to_ast};
use std::env::args;

fn main() {
    let arg: Vec<String> = args().collect();
    let ast = source_to_ast(arg.get(1).unwrap()).unwrap();
    let operation = ast.to_assembly();
    elf_writer(&arg[2], &operation).unwrap();
}
