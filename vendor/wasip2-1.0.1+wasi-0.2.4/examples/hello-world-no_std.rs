fn main() {
    let stdout = wasip2::cli::stdout::get_stdout();
    stdout.blocking_write_and_flush(b"Hello, world!\n").unwrap();
}
