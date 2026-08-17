/// An example how to use the ANSI escape sequence.
use std::io::{Result, Write};

use anes;

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    write!(stdout, "{}", anes::SaveCursorPosition)?;
    write!(stdout, "{}", anes::RestoreCursorPosition)?;
    stdout.flush()?;
    Ok(())
}
