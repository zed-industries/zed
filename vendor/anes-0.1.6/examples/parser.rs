/// An example how to use the ANSI escape sequence parser.
use std::io::{Read, Result, Write};

use anes::{
    self, execute,
    parser::{KeyCode, Parser, Sequence},
    queue,
};
use libc::termios as Termios;

const HELP: &str = r#"ANES parser example

* Hit `Esc` to quit
* Hit 'c' to ask for cursor position
* Use your mouse or type anything
"#;

fn main() -> Result<()> {
    let mut w = std::io::stdout();
    queue!(
        w,
        anes::SwitchBufferToAlternate,
        anes::HideCursor,
        anes::EnableMouseEvents
    )?;
    for line in HELP.split('\n') {
        queue!(w, line, anes::MoveCursorToNextLine(1))?;
    }
    w.flush()?;

    let saved_attributes = get_termios()?;
    let mut attributes = saved_attributes;
    make_raw(&mut attributes);
    set_termios(attributes)?;

    let mut stdin = std::io::stdin();
    let mut stdin_buffer = [0u8; 1024];
    let mut parser = Parser::default();

    loop {
        if let Ok(size) = stdin.read(&mut stdin_buffer) {
            parser.advance(&stdin_buffer[..size], false);

            let mut break_outer_loop = false;

            while let Some(sequence) = parser.next() {
                match sequence {
                    Sequence::Key(KeyCode::Esc, _) => {
                        break_outer_loop = true;
                        break;
                    }
                    Sequence::Key(KeyCode::Char('c'), _) => {
                        execute!(w, anes::ReportCursorPosition)?
                    }
                    _ => execute!(
                        w,
                        anes::ClearLine::Left,
                        anes::MoveCursorToColumn(1),
                        format!("{:?}", sequence),
                    )?,
                }
            }

            if break_outer_loop {
                break;
            }
        }
    }

    set_termios(saved_attributes)?;

    execute!(
        w,
        anes::DisableMouseEvents,
        anes::ShowCursor,
        anes::SwitchBufferToNormal
    )?;
    Ok(())
}

//
// RAW mode
//

fn get_termios() -> Result<Termios> {
    unsafe {
        let mut termios = std::mem::zeroed();
        if libc::tcgetattr(libc::STDIN_FILENO, &mut termios) != -1 {
            Ok(termios)
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

fn set_termios(termios: Termios) -> Result<()> {
    if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &termios) } != -1 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

fn make_raw(termios: &mut Termios) {
    unsafe { libc::cfmakeraw(termios) }
}
