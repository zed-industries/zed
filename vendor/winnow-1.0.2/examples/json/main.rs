mod json;
mod parser_alt;
mod parser_dispatch;
#[allow(dead_code)]
mod parser_partial;

use winnow::error::EmptyError;
use winnow::prelude::*;

fn main() -> Result<(), lexopt::Error> {
    let args = Args::parse()?;

    let data = args.input.as_deref().unwrap_or(if args.invalid {
        "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { 1\"hello\" : \"world\"
  }
  } "
    } else {
        "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { \"hello\" : \"world\"
  }
  } "
    });

    let result = match args.implementation {
        Impl::Naive => parser_alt::json::<EmptyError>.parse(data),
        Impl::Dispatch => parser_dispatch::json::<EmptyError>.parse(data),
    };
    match result {
        Ok(json) => {
            println!("{json:#?}");
        }
        Err(err) => {
            if args.pretty {
                println!("{err}");
            } else {
                println!("{err:#?}");
            }
        }
    }

    Ok(())
}

#[derive(Default)]
struct Args {
    input: Option<String>,
    invalid: bool,
    pretty: bool,
    implementation: Impl,
}

#[derive(Default)]
enum Impl {
    #[default]
    Naive,
    Dispatch,
}

impl Args {
    fn parse() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut res = Args::default();

        let mut args = lexopt::Parser::from_env();
        while let Some(arg) = args.next()? {
            match arg {
                Long("invalid") => {
                    res.invalid = true;
                }
                Long("pretty") => {
                    // Only case where pretty matters
                    res.pretty = true;
                    res.invalid = true;
                }
                Long("impl") => {
                    res.implementation = args.value()?.parse_with(|s| match s {
                        "naive" => Ok(Impl::Naive),
                        "dispatch" => Ok(Impl::Dispatch),
                        _ => Err("expected `naive`, `dispatch`"),
                    })?;
                }
                Value(input) => {
                    res.input = Some(input.string()?);
                }
                _ => return Err(arg.unexpected()),
            }
        }
        Ok(res)
    }
}
