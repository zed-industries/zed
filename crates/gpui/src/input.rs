mod input;
mod input_handler;

pub use input::{Input, InputLineLayout};
// todo: move to keymap
pub use input::{
    Backspace, Copy, Cut, Delete, Down, End, Enter, Home, Left, MoveToBeginning, MoveToEnd, Paste,
    Right, SelectAll, SelectDown, SelectLeft, SelectRight, SelectToBeginning, SelectToEnd,
    SelectUp, SelectWordLeft, SelectWordRight, Tab, Up, WordLeft, WordRight,
};
pub use input_handler::*;
