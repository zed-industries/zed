/// An example how to execute the ANSI escape sequence.
use std::io::{Result, Write};

use anes::execute;

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    execute!(
        &mut stdout,
        anes::SaveCursorPosition,
        anes::MoveCursorTo(10, 10),
        anes::RestoreCursorPosition
    )?;
    Ok(())
}
