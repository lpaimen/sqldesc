

/// Run with `cargo run --example sqldesc` 
use std::fs;

use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() {

    let filename = std::env::args().nth(1).expect(
        "No filename provided!\n\n
        Usage: cargo run --example sqldesc FILENAME.sql"
    );

    let contents = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Unable to read the file {}", &filename));

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let ast = Parser::parse_sql(&dialect, contents.to_string()).unwrap();

    println!("AST: {:?}", ast);
}
