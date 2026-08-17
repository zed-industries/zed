use std::io::Write as _;

wasip2::cli::command::export!(Example);

struct Example;

impl wasip2::exports::cli::run::Guest for Example {
    fn run() -> Result<(), ()> {
        let mut stdout = wasip2::cli::stdout::get_stdout();
        stdout.write_all(b"Hello, WASI!").unwrap();
        stdout.flush().unwrap();
        Ok(())
    }
}
