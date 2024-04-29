mod executor;
mod parser;

use std::fs;

fn main() {
    let source = fs::read_to_string("test.lmc").unwrap();

    let program = parser::parse_source(source);
    executor::execute(program);
}
