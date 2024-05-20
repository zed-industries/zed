use gpui::AnyElement;
use ui::{IntoElement, SharedString};

use alacritty_terminal::vte::{Params, Parser, Perform};

pub struct TerminalOutput {
    parser: Parser,
    state: ParserState,
}

pub struct ParserState {
    buffer: Vec<char>,
}

impl ParserState {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    // Pseudocode
    //
    // fn render_buffer(&self) -> Vec<AnyElement> {
    //     let mut elements = Vec::new();
    //     // Convert the buffer to GPUI elements.
    //     // This can involve parsing the buffer for ANSI color codes
    //     // and creating Styled elements for each piece of text.
    //     elements
    // }
}

impl TerminalOutput {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            state: ParserState::new(),
        }
    }

    pub fn append_text(&mut self, text: &str) {
        for byte in text.as_bytes() {
            self.parser.advance(&mut self.state, *byte);
        }
    }

    pub fn num_lines(&self) -> u8 {
        // todo!(): Track this over time with our parser and just return it when needed
        self.state.buffer.iter().filter(|c| **c == '\n').count() as u8 // the line itself
    }

    pub fn render(&self) -> AnyElement {
        // todo!(): be less hacky
        let trimmed_buffer: String = self
            .state
            .buffer
            .iter()
            .collect::<String>()
            .trim_end()
            .to_string();
        SharedString::from(trimmed_buffer).into_any_element()
    }
}

impl Perform for ParserState {
    fn print(&mut self, c: char) {
        self.buffer.push(c);
    }

    fn execute(&mut self, byte: u8) {
        println!("[execute] {:02x}", byte);
        match byte {
            b'\n' => {
                self.buffer.push('\n');
            }
            b'\r' => {
                self.buffer.retain(|b| *b != '\r');
                self.buffer.push('\r');
            }
            _ => {
                // self.buffer.push(byte as char);
            }
        }
    }

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        println!(
            "[hook] params={:?}, intermediates={:?}, ignore={:?}, char={:?}",
            params, intermediates, ignore, c
        );
    }

    fn put(&mut self, byte: u8) {
        println!("[put] {:02x}", byte);
    }

    fn unhook(&mut self) {
        println!("[unhook]");
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        println!(
            "[osc_dispatch] params={:?} bell_terminated={}",
            params, bell_terminated
        );
    }

    fn csi_dispatch(
        &mut self,
        params: &alacritty_terminal::vte::Params,
        intermediates: &[u8],
        ignore: bool,
        action: char,
    ) {
        // Handle control sequences like colors
        println!(
            "[csi_dispatch] params={:#?}, intermediates={:?}, ignore={:?}, action={:?}",
            params, intermediates, ignore, action
        );
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        println!(
            "[esc_dispatch] intermediates={:?}, ignore={:?}, byte={:02x}",
            intermediates, ignore, byte
        );
    }
}
