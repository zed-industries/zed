use winnow::prelude::*;

mod parser;
#[cfg(test)]
mod test;

fn main() -> Result<(), lexopt::Error> {
    let args = Args::parse()?;

    let input = args.input.as_deref().unwrap_or("1 + 1");
    match parser::pratt_parser.parse(input) {
        Ok(result) => {
            println!("{result}");
        }
        Err(err) => {
            println!("FAILED");
            println!("{err}");
        }
    }

    Ok(())
}

#[derive(Default)]
struct Args {
    input: Option<String>,
}

impl Args {
    fn parse() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut res = Args::default();

        let mut args = lexopt::Parser::from_env();
        while let Some(arg) = args.next()? {
            match arg {
                Value(input) => {
                    res.input = Some(input.string()?);
                }
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(res)
    }
}
