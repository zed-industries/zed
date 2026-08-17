use std::io::Write as _;

fn main() {
    let mut stdout = wasip2::cli::stdout::get_stdout();
    stdout.write_all(b"Hello, world!\n").unwrap();
    stdout.flush().unwrap();
}
