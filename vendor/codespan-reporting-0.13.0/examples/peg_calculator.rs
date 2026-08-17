//! An example of using `peg` with `codespan_reporting`.
//!
//! To run this example, execute the following command from the top level of
//! this repository:
//!
//! ```sh
//! cargo run --example peg_calculator
//! ```

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::{self, Config};

use rustyline::error::ReadlineError;
use rustyline::Editor;

peg::parser! {
    grammar arithmetic() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule calculate() -> i64 = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "-" y:@ { x - y }
                  "-" v:@ { - v }
            --
            x:(@) "*" y:@ { x * y }
            x:(@) "/" y:@ { x / y }
            --
            x:@   "^" y:(@) { i64::pow(x, y as u32) }
            v:@   "!"       { (1..v+1).product() }
            --
            "(" v:calculate() ")" { v }
            n:number() { n }
        }
    }
}

#[cfg(not(feature = "termcolor"))]
fn main() {
    panic!("this example requires termcolor feature");
}

#[cfg(feature = "termcolor")]
fn main() -> anyhow::Result<()> {
    use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = Config::default();
    let mut editor = Editor::<()>::new();

    loop {
        let line = match editor.readline("> ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => return Ok(()),
            Err(error) => return Err(error.into()),
        };

        match arithmetic::calculate(&line) {
            Ok(number) => println!("{}", number),
            Err(error) => {
                let file = SimpleFile::new("<repl>", line);

                let start = error.location.offset;
                let diagnostic = Diagnostic::error()
                    .with_message("parse error")
                    .with_label(Label::primary((), start..start).with_message("parse error"))
                    .with_notes(vec![format!("expected: {}", error.expected)]);

                term::emit_to_write_style(&mut writer.lock(), &config, &file, &diagnostic)?;
            }
        }
    }
}
