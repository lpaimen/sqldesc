

/// Run with `cargo run --example sqldesc` 
use std::fs;

use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{ Statement, Doc, ObjectName, Ident };

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Parses SQL description comments")]
struct Opt {
    #[structopt(long, help = "Print debug info while running")]
    debug: bool,

    #[structopt(parse(from_os_str), help = "SQL file to parse")]
    input_file: PathBuf,

    #[structopt(long, help = "Output type, sql or comment", default_value = "comment")]
    output: String,
}

fn doc_table(opt: &Opt, name: &ObjectName, doc: &Doc) {
    if opt.output == "sql" {
        println!("INSERT INTO sqldesc(type, target, description) VALUES('table', '{}', '{}');", name, doc.doc_string());
    }
    if opt.output == "comment" {
        println!("COMMENT ON TABLE {} IS '{}';", name, doc.doc_string());
    }
}

fn doc_column(opt: &Opt, table: &ObjectName, name: &Ident, doc: &Doc) {
    if opt.output == "sql" {
        println!("INSERT INTO sqldesc(type, target, description) VALUES('column', '{}.{}', '{}');", table, name, doc.doc_string());
    }
    if opt.output == "comment" {
        println!("COMMENT ON COLUMN {}.{} IS '{}';", table, name, doc.doc_string());
    }
}

fn main() {

    let opt = Opt::from_args();
    
    let contents = fs::read_to_string(&opt.input_file)
        .unwrap_or_else(|_| panic!("Unable to read the file"));

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let ast = Parser::parse_sql(&dialect, contents.to_string()).unwrap();

    if opt.debug {
        println!(
            "Round-trip:\n'{}'",
            ast
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        );
        println!("Parse results:\n{:#?}", ast);
    }

    for expr in ast.iter() {
        match expr {
            Statement::CreateTable { name, doc, columns, .. } => {
                if doc.is_useful() {
                    doc_table(&opt, name, doc);
                }
                for col in columns.iter() {
                    if col.doc.is_useful() {
                        doc_column(&opt, name, &col.name, &col.doc);
                    }
                }
            }
            _ => ()
        }
    }
}
