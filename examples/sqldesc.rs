

/// Run with `cargo run --example sqldesc` 
use std::fs;

use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{ Statement, Doc, ObjectName, Ident };

fn doc_table(name: &ObjectName, doc: &Doc) {
    println!("INSERT INTO sqldesc(type, target, description) VALUES('table', '{}', '{}');", name, doc.doc_string());
}

fn doc_column(table: &ObjectName, name: &Ident, doc: &Doc) {
    println!("INSERT INTO sqldesc(type, target, description) VALUES('column', '{}.{}', '{}');", table, name, doc.doc_string());
}



fn main() {

    let filename = std::env::args().nth(1).expect(
        "No filename provided!\n\n
        Usage: cargo run --example sqldesc FILENAME.sql"
    );

    let contents = fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Unable to read the file {}", &filename));

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let ast = Parser::parse_sql(&dialect, contents.to_string()).unwrap();

    println!(
        "Round-trip:\n'{}'",
        ast
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
    println!("Parse results:\n{:#?}", ast);

    for expr in ast.iter() {
        match expr {
            Statement::CreateTable { name, doc, columns, .. } => {
                if doc.is_useful() {
                    doc_table(name, doc);
                }
                for col in columns.iter() {
                    if col.doc.is_useful() {
                        doc_column(name, &col.name, &col.doc);
                    }
                }
            }
            _ => ()
        }
    }
}
