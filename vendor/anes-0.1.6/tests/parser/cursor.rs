use anes::parser::Sequence;

use crate::test_sequences;

#[test]
fn position() {
    test_sequences!(b"\x1B[20;10R", Sequence::CursorPosition(10, 20),);
}
