/// An example how to queue & flush the ANSI escape sequence.
use std::io::{Result, Write};

use anes::queue;

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    queue!(
        &mut stdout,
        anes::SaveCursorPosition,
        anes::MoveCursorTo(10, 10)
    )?;

    queue!(&mut stdout, anes::RestoreCursorPosition,)?;

    // ANSI sequences are not executed until you flush it!
    stdout.flush()
}
