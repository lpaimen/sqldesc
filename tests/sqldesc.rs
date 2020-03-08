use sqlparser::ast::*;
use sqlparser::test_utils::{all_dialects };

#[test]
fn parse_create_table_comments_different() {
    let sql = "
        -- Normal comment, not doc
        --* First doc line
        --* Second doc line
        /* Multi line, not doc */
        /** Multi line 1 */
        /*
         * Multi line, not doc
         */
        /**
         * Multi line 2
         * line 3
         */
        CREATE TABLE foo()";
    let ast = all_dialects().one_statement_parses_to(sql, "CREATE TABLE foo ()");

    match ast {
        Statement::CreateTable {
            name: _,
            columns: _,
            constraints: _,
            with_options: _,
            external: _,
            file_format: None,
            location: None,
            doc,
        } => {
            assert_eq!(doc.doc_string(), "First doc line\nSecond doc line\nMulti line 1\nMulti line 2\nline 3")
        },
        _ => unreachable!(),
    }
}

#[test]
fn parse_create_table_column_comments() {
    let sql = "
    /** Table comment */
    CREATE TABLE foo(
        /** Column 1 */
        col_1 TEXT,
        --* Column 2
        col_2 TEXT,
        /**
         * Column 3
         * Line 2
         */
        col_3 TEXT,
        -- No desc
        col_4 TEXT
    )";

    let ast = all_dialects().one_statement_parses_to(sql, "CREATE TABLE foo (col_1 text, col_2 text, col_3 text, col_4 text)");

    match ast {
        Statement::CreateTable {
            name: _,
            columns,
            constraints: _,
            with_options: _,
            external: _,
            file_format: None,
            location: None,
            doc,
        } => {
            assert_eq!(doc.doc_string(), "Table comment");
            assert_eq!(columns[0].doc.doc_string(), "Column 1");
            assert_eq!(columns[1].doc.doc_string(), "Column 2");
            assert_eq!(columns[2].doc.doc_string(), "Column 3\nLine 2");
            assert_eq!(columns[3].doc.doc_string(), "");
            assert_eq!(columns[3].doc.is_useful(), false);
        },
        _ => unreachable!(),
    }
}