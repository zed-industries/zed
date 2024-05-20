use gpui::AnyElement;
use ui::{IntoElement, SharedString};

use core::iter;

use alacritty_terminal::vte::{
    ansi::{Attr, Color, ModifyOtherKeys, NamedColor, Rgb},
    Params, ParamsIter, Parser, Perform,
};

pub struct TerminalOutput {
    parser: Parser,
    state: ParserState,
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

pub struct ParserState {
    buffer: Vec<char>,
    handler: SimpleHandler,
}

impl ParserState {
    fn new() -> Self {
        Self {
            buffer: Vec::new(),
            handler: SimpleHandler::new(),
        }
    }
}

struct SimpleHandler {
    fg: Color,
    bg: Color,
}

// ansi::Handler requires way more for a trait than we need
impl SimpleHandler {
    fn new() -> Self {
        Self {
            fg: Color::Named(NamedColor::Foreground),
            bg: Color::Named(NamedColor::Background),
        }
    }

    fn reset(&mut self) {
        self.fg = Color::Named(NamedColor::Foreground);
        self.bg = Color::Named(NamedColor::Background);
    }

    fn terminal_attribute(&mut self, attr: Attr) {
        // println!("[terminal_attribute] attr={:?}", attr);

        match attr {
            Attr::Reset => {
                self.reset();
            }
            Attr::Foreground(color) => {
                self.fg = color;
            }
            Attr::Background(color) => {
                self.bg = color;
            }
            Attr::UnderlineColor(_) => todo!(),
            _ => {
                // Skipping Dim, Italic, etc. for now
            }
        }
    }

    fn set_modify_other_keys(&mut self, mode: ModifyOtherKeys) {
        // println!("[set_modify_other_keys] mode={:?}", mode);
    }

    fn report_modify_other_keys(&mut self) {
        // println!("[report_modify_other_keys]");
    }
}

impl Perform for ParserState {
    fn print(&mut self, c: char) {
        println!("[print] c={:?}", c);
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
        // println!("[put] {:02x}", byte);
    }

    fn unhook(&mut self) {
        // println!("[unhook]");
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

        let mut params_iter = params.iter();

        let mut next_param_or = |default: u16| match params_iter.next() {
            Some(&[param, ..]) if param != 0 => param,
            _ => default,
        };

        match (action, intermediates) {
            ('m', []) => {
                if params.is_empty() {
                    self.handler.terminal_attribute(Attr::Reset);
                } else {
                    for attr in attrs_from_sgr_parameters(&mut params_iter) {
                        match attr {
                            Some(attr) => self.handler.terminal_attribute(attr),
                            None => return,
                        }
                    }
                }
            }
            ('m', [b'>']) => {
                let mode = match (next_param_or(1) == 4).then(|| next_param_or(0)) {
                    Some(0) => ModifyOtherKeys::Reset,
                    Some(1) => ModifyOtherKeys::EnableExceptWellDefined,
                    Some(2) => ModifyOtherKeys::EnableAll,
                    _ => return,
                };
                self.handler.set_modify_other_keys(mode);
            }
            ('m', [b'?']) => {
                if params_iter.next() == Some(&[4]) {
                    self.handler.report_modify_other_keys();
                } else {
                    return;
                }
            }
            _ => {}
        }

        ()
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        // println!(
        //     "[esc_dispatch] intermediates={:?}, ignore={:?}, byte={:02x}",
        //     intermediates, ignore, byte
        // );
    }
}

#[inline]
fn attrs_from_sgr_parameters(params: &mut ParamsIter<'_>) -> Vec<Option<Attr>> {
    let mut attrs = Vec::with_capacity(params.size_hint().0);

    while let Some(param) = params.next() {
        let attr = match param {
            [0] => Some(Attr::Reset),
            [1] => Some(Attr::Bold),
            [2] => Some(Attr::Dim),
            [3] => Some(Attr::Italic),
            [4, 0] => Some(Attr::CancelUnderline),
            [4, 2] => Some(Attr::DoubleUnderline),
            [4, 3] => Some(Attr::Undercurl),
            [4, 4] => Some(Attr::DottedUnderline),
            [4, 5] => Some(Attr::DashedUnderline),
            [4, ..] => Some(Attr::Underline),
            [5] => Some(Attr::BlinkSlow),
            [6] => Some(Attr::BlinkFast),
            [7] => Some(Attr::Reverse),
            [8] => Some(Attr::Hidden),
            [9] => Some(Attr::Strike),
            [21] => Some(Attr::CancelBold),
            [22] => Some(Attr::CancelBoldDim),
            [23] => Some(Attr::CancelItalic),
            [24] => Some(Attr::CancelUnderline),
            [25] => Some(Attr::CancelBlink),
            [27] => Some(Attr::CancelReverse),
            [28] => Some(Attr::CancelHidden),
            [29] => Some(Attr::CancelStrike),
            [30] => Some(Attr::Foreground(Color::Named(NamedColor::Black))),
            [31] => Some(Attr::Foreground(Color::Named(NamedColor::Red))),
            [32] => Some(Attr::Foreground(Color::Named(NamedColor::Green))),
            [33] => Some(Attr::Foreground(Color::Named(NamedColor::Yellow))),
            [34] => Some(Attr::Foreground(Color::Named(NamedColor::Blue))),
            [35] => Some(Attr::Foreground(Color::Named(NamedColor::Magenta))),
            [36] => Some(Attr::Foreground(Color::Named(NamedColor::Cyan))),
            [37] => Some(Attr::Foreground(Color::Named(NamedColor::White))),
            [38] => {
                let mut iter = params.map(|param| param[0]);
                parse_sgr_color(&mut iter).map(Attr::Foreground)
            }
            [38, params @ ..] => handle_colon_rgb(params).map(Attr::Foreground),
            [39] => Some(Attr::Foreground(Color::Named(NamedColor::Foreground))),
            [40] => Some(Attr::Background(Color::Named(NamedColor::Black))),
            [41] => Some(Attr::Background(Color::Named(NamedColor::Red))),
            [42] => Some(Attr::Background(Color::Named(NamedColor::Green))),
            [43] => Some(Attr::Background(Color::Named(NamedColor::Yellow))),
            [44] => Some(Attr::Background(Color::Named(NamedColor::Blue))),
            [45] => Some(Attr::Background(Color::Named(NamedColor::Magenta))),
            [46] => Some(Attr::Background(Color::Named(NamedColor::Cyan))),
            [47] => Some(Attr::Background(Color::Named(NamedColor::White))),
            [48] => {
                let mut iter = params.map(|param| param[0]);
                parse_sgr_color(&mut iter).map(Attr::Background)
            }
            [48, params @ ..] => handle_colon_rgb(params).map(Attr::Background),
            [49] => Some(Attr::Background(Color::Named(NamedColor::Background))),
            [58] => {
                let mut iter = params.map(|param| param[0]);
                parse_sgr_color(&mut iter).map(|color| Attr::UnderlineColor(Some(color)))
            }
            [58, params @ ..] => {
                handle_colon_rgb(params).map(|color| Attr::UnderlineColor(Some(color)))
            }
            [59] => Some(Attr::UnderlineColor(None)),
            [90] => Some(Attr::Foreground(Color::Named(NamedColor::BrightBlack))),
            [91] => Some(Attr::Foreground(Color::Named(NamedColor::BrightRed))),
            [92] => Some(Attr::Foreground(Color::Named(NamedColor::BrightGreen))),
            [93] => Some(Attr::Foreground(Color::Named(NamedColor::BrightYellow))),
            [94] => Some(Attr::Foreground(Color::Named(NamedColor::BrightBlue))),
            [95] => Some(Attr::Foreground(Color::Named(NamedColor::BrightMagenta))),
            [96] => Some(Attr::Foreground(Color::Named(NamedColor::BrightCyan))),
            [97] => Some(Attr::Foreground(Color::Named(NamedColor::BrightWhite))),
            [100] => Some(Attr::Background(Color::Named(NamedColor::BrightBlack))),
            [101] => Some(Attr::Background(Color::Named(NamedColor::BrightRed))),
            [102] => Some(Attr::Background(Color::Named(NamedColor::BrightGreen))),
            [103] => Some(Attr::Background(Color::Named(NamedColor::BrightYellow))),
            [104] => Some(Attr::Background(Color::Named(NamedColor::BrightBlue))),
            [105] => Some(Attr::Background(Color::Named(NamedColor::BrightMagenta))),
            [106] => Some(Attr::Background(Color::Named(NamedColor::BrightCyan))),
            [107] => Some(Attr::Background(Color::Named(NamedColor::BrightWhite))),
            _ => None,
        };
        attrs.push(attr);
    }

    attrs
}

/// Handle colon separated rgb color escape sequence.
#[inline]
fn handle_colon_rgb(params: &[u16]) -> Option<Color> {
    let rgb_start = if params.len() > 4 { 2 } else { 1 };
    let rgb_iter = params[rgb_start..].iter().copied();
    let mut iter = iter::once(params[0]).chain(rgb_iter);

    parse_sgr_color(&mut iter)
}

/// Parse a color specifier from list of attributes.
fn parse_sgr_color(params: &mut dyn Iterator<Item = u16>) -> Option<Color> {
    match params.next() {
        Some(2) => Some(Color::Spec(Rgb {
            r: u8::try_from(params.next()?).ok()?,
            g: u8::try_from(params.next()?).ok()?,
            b: u8::try_from(params.next()?).ok()?,
        })),
        Some(5) => Some(Color::Indexed(u8::try_from(params.next()?).ok()?)),
        _ => None,
    }
}
