

/// Run with `cargo run --example sqldesc` 
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() {
    println!("Hello, wordld!");

    let sql = "--* Foobar
    --* Barbar
    --* babar
    /**
     * multiline comment
     */
            CREATE TABLE foob(
                --* id: Primary key of foob
                id integer primary key,
                --* foo: never set this
                foo text)";

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let ast = Parser::parse_sql(&dialect, sql.to_string()).unwrap();

    println!("AST: {:?}", ast);
}
