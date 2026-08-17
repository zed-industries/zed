//! Simple example of miette usage

use std::io::Cursor;

use brush_parser::Parser;
use miette::{IntoDiagnostic, miette};

fn main() -> miette::Result<()> {
    let f = std::env::args()
        .nth(1)
        .ok_or_else(|| miette!("Please provide a file name"))?;

    let source = std::fs::read_to_string(&f).into_diagnostic()?;
    let reader = Cursor::new(&source);
    let mut parser = Parser::builder().reader(reader).build();

    let ast = parser
        .parse_program()
        .map_err(|e| e.to_pretty_error(&source))?;

    println!("{ast:#?}");

    Ok(())
}
