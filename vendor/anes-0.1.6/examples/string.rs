//! An example how to retrieve the ANSI escape sequence as a `String`.
use anes::SaveCursorPosition;

fn main() {
    let string = format!("{}", SaveCursorPosition);
    assert_eq!(&string, "\x1B7");
}
