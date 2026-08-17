//! Brainfuck interpreter written in Rust, using Logos.
//!
//! Usage:
//!     cargo run --example brainfuck <path/to/file>
//!
//! Example:
//!     cargo run --example brainfuck examples/hello_word.bf
//!
//! Brainfuck is an esoteric programming language that only
//! uses 8 single-character commands:
//! - '>';
//! - '<';
//! - '+';
//! - '-';
//! - '.';
//! - ',';
//! - '[';
//! - and ']'.
//!
//! Despite being very hard to use in practice, this makes
//! this language very simple to interpret. The following code
//! defines an [`execute`] function that runs Brainfuck code.
//!
//! Logos is used here to directly transform the code stream
//! into meaningful `Op` operations (or commands).
//! Errors, i.e., unknown tokens, are discarded using `filter_map`.
//!
//! More details can be found on Wikipedia:
//! <https://en.wikipedia.org/wiki/Brainfuck>.
//!
//! or on <http://brainfuck.org/>.

/* ANCHOR: all */
use logos::Logos;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Read};

/* ANCHOR: tokens */
/// Each [`Op`] variant is a single character.
#[derive(Debug, Logos)]
enum Op {
    /// Increment pointer.
    #[token(">")]
    IncPointer,
    /// Decrement pointer.
    #[token("<")]
    DecPointer,
    /// Increment data at pointer.
    #[token("+")]
    IncData,
    /// Decrement data at pointer.
    #[token("-")]
    DecData,
    /// Output data at pointer.
    #[token(".")]
    OutData,
    /// Input (read) to data at pointer.
    #[token(",")]
    InpData,
    /// Conditionally jump to matching `']'`.
    #[token("[")]
    CondJumpForward,
    /// Conditionally jump to matching `'['`.
    #[token("]")]
    CondJumpBackward,
}
/* ANCHOR_END: tokens */

/// Print one byte to the terminal.
#[inline(always)]
fn print_byte(byte: u8) {
    print!("{}", byte as char);
}

/// Read one byte from the terminal.
#[inline(always)]
fn read_byte() -> u8 {
    let mut input = [0u8; 1];
    io::stdin()
        .read_exact(&mut input)
        .expect("An error occurred while reading byte!");
    input[0]
}

/// Execute Brainfuck code from a string slice.
pub fn execute(code: &str) {
    let operations: Vec<_> = Op::lexer(code).filter_map(|op| op.ok()).collect();
    let mut data = [0u8; 30_000]; // Minimum recommended size
    let mut pointer: usize = 0;
    let len = operations.len();

    // We pre-process matching jump commands, and we create
    // a mapping between them.
    let mut queue = Vec::new();
    let mut pairs = HashMap::new();
    let mut pairs_reverse = HashMap::new();

    for (i, op) in operations.iter().enumerate() {
        match op {
            Op::CondJumpForward => queue.push(i),
            Op::CondJumpBackward => {
                if let Some(start) = queue.pop() {
                    pairs.insert(start, i);
                    pairs_reverse.insert(i, start);
                } else {
                    panic!(
                        "Unexpected conditional backward jump at position {}, does not match any '['",
                        i
                    );
                }
            }
            _ => (),
        }
    }

    if !queue.is_empty() {
        panic!("Unmatched conditional forward jump at positions {:?}, expecting a closing ']' for each of them", queue);
    }

    /* ANCHOR: fsm */
    let mut i: usize = 0;
    // True program execution.
    loop {
        match operations[i] {
            Op::IncPointer => pointer += 1,
            Op::DecPointer => pointer -= 1,
            Op::IncData => data[pointer] = data[pointer].wrapping_add(1),
            Op::DecData => data[pointer] = data[pointer].wrapping_sub(1),
            Op::OutData => print_byte(data[pointer]),
            Op::InpData => data[pointer] = read_byte(),
            Op::CondJumpForward => {
                if data[pointer] == 0 {
                    // Skip until matching end.
                    i = *pairs.get(&i).unwrap();
                }
            }
            Op::CondJumpBackward => {
                if data[pointer] != 0 {
                    // Go back to matching start.
                    i = *pairs_reverse.get(&i).unwrap();
                }
            }
        }
        i += 1;

        if i >= len {
            break;
        }
    }
    /* ANCHOR_END: fsm */
}

fn main() {
    let src = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    execute(src.as_str());
}
/* ANCHOR_END: all */
