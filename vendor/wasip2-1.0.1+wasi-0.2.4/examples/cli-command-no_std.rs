wasip2::cli::command::export!(Example);

struct Example;

impl wasip2::exports::cli::run::Guest for Example {
    fn run() -> Result<(), ()> {
        let stdout = wasip2::cli::stdout::get_stdout();
        stdout.blocking_write_and_flush(b"Hello, WASI!").unwrap();
        Ok(())
    }
}
