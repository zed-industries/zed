use gpui::{font, AnyElement, StyledText, TextRun};
use theme::Theme;
use ui::{div, IntoElement, ParentElement as _, Styled};

use core::iter;

use alacritty_terminal::vte::{
    ansi::{Attr, Color, NamedColor, Rgb},
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
            self.parser.advance(&mut self.state.handler, *byte);
        }
    }

    pub fn num_lines(&self) -> u8 {
        // todo!(): Track this over time with our parser and just return it when needed
        self.state.handler.buffer.lines().count() as u8
    }

    pub fn render(&self, theme: &Theme) -> AnyElement {
        let mut text_runs = self.state.handler.text_runs.clone();
        text_runs.push(self.state.handler.current_text_run.clone());

        let runs = text_runs
            .iter()
            .map(|ansi_run| {
                let color = terminal_view::terminal_element::convert_color(&ansi_run.fg, theme);
                let background_color = Some(terminal_view::terminal_element::convert_color(
                    &ansi_run.bg,
                    theme,
                ));

                TextRun {
                    len: ansi_run.len,
                    color,
                    background_color,
                    underline: Default::default(),
                    font: font("Zed Mono"),
                    strikethrough: None,
                }
            })
            .collect::<Vec<TextRun>>();

        let text =
            StyledText::new(self.state.handler.buffer.trim_end().to_string()).with_runs(runs);
        div().child(text).into_any_element()
    }
}

pub struct ParserState {
    handler: TerminalHandler,
}

impl ParserState {
    fn new() -> Self {
        Self {
            handler: TerminalHandler::new(),
        }
    }
}

#[derive(Clone)]
struct AnsiTextRun {
    pub len: usize,
    pub fg: alacritty_terminal::vte::ansi::Color,
    pub bg: alacritty_terminal::vte::ansi::Color,
}

impl AnsiTextRun {
    fn default() -> Self {
        Self {
            len: 0,
            fg: Color::Named(NamedColor::Foreground),
            bg: Color::Named(NamedColor::Background),
        }
    }

    fn push(&mut self, c: char) {
        self.len += 1;
    }
}

// This should instead gather `TextRun`s
struct TerminalHandler {
    text_runs: Vec<AnsiTextRun>,
    current_text_run: AnsiTextRun,
    buffer: String,
}

impl TerminalHandler {
    fn new() -> Self {
        Self {
            text_runs: Vec::new(),
            current_text_run: AnsiTextRun {
                len: 0,
                fg: Color::Named(NamedColor::Foreground),
                bg: Color::Named(NamedColor::Background),
            },
            buffer: String::new(),
        }
    }

    fn add_text(&mut self, c: char) {
        self.buffer.push(c);
        self.current_text_run.len += 1;

        // TODO: Handle newlines and carriage returns
    }

    fn reset(&mut self) {
        if self.current_text_run.len > 0 {
            self.text_runs.push(self.current_text_run.clone());
        }

        self.current_text_run = AnsiTextRun::default();
    }

    fn terminal_attribute(&mut self, attr: Attr) {
        // println!("[terminal_attribute] attr={:?}", attr);
        if Attr::Reset == attr {
            self.reset();
            return;
        }

        if self.current_text_run.len > 0 {
            self.text_runs.push(self.current_text_run.clone());
        }

        let mut text_run = AnsiTextRun {
            len: 0,
            fg: self.current_text_run.fg,
            bg: self.current_text_run.bg,
        };

        match attr {
            Attr::Foreground(color) => text_run.fg = color,
            Attr::Background(color) => text_run.bg = color,
            _ => {}
        }

        self.current_text_run = text_run;
    }

    fn process_carriage_return(&mut self) {
        // Find last carriage return's position
        let last_cr = self.buffer.rfind('\r').unwrap_or(0);
        self.buffer = self.buffer.chars().take(last_cr).collect();

        // First work through our current text run
        let mut total_len = self.current_text_run.len;
        if total_len > last_cr {
            // We are in the current text run
            self.current_text_run.len = self.current_text_run.len - last_cr;
        } else {
            let mut last_cr_run = 0;
            // Find the last run before the last carriage return
            for (i, run) in self.text_runs.iter().enumerate() {
                total_len += run.len;
                if total_len > last_cr {
                    last_cr_run = i;
                    break;
                }
            }
            self.text_runs = self.text_runs[..last_cr_run].to_vec();
            self.current_text_run = self.text_runs.pop().unwrap_or(AnsiTextRun::default());
        }

        self.buffer.push('\r');
        self.current_text_run.len += 1;
    }
}

impl Perform for TerminalHandler {
    fn print(&mut self, c: char) {
        // println!("[print] c={:?}", c);
        self.add_text(c);
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.add_text('\n');
            }
            b'\r' => {
                self.process_carriage_return();
            }
            _ => {
                // Format as hex
                println!("[execute] byte={:02x}", byte);
            }
        }
    }

    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {
        // noop
        // println!(
        //     "[hook] params={:?}, intermediates={:?}, c={:?}",
        //     _params, _intermediates, _c
        // );
    }

    fn put(&mut self, _byte: u8) {
        // noop
        // println!("[put] byte={:02x}", _byte);
    }

    fn unhook(&mut self) {
        // noop
    }

    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {
        // noop
        // println!("[osc_dispatch] params={:?}", _params);
    }

    fn csi_dispatch(
        &mut self,
        params: &alacritty_terminal::vte::Params,
        intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        // println!(
        //     "[csi_dispatch] action={:?}, params={:?}, intermediates={:?}",
        //     action, params, intermediates
        // );

        let mut params_iter = params.iter();
        // Collect colors
        match (action, intermediates) {
            ('m', []) => {
                if params.is_empty() {
                    self.terminal_attribute(Attr::Reset);
                } else {
                    for attr in attrs_from_sgr_parameters(&mut params_iter) {
                        match attr {
                            Some(attr) => self.terminal_attribute(attr),
                            None => return,
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {
        // noop
        // println!(
        //     "[esc_dispatch] intermediates={:?}, byte={:?}",
        //     _intermediates, _byte
        // );
    }
}

// The following was pulled from vte::ansi
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
