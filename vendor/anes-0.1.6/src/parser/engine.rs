//
// https://vt100.net/emu/dec_ansi_parser
//
// The parser is heavily inspired by the vte (https://crates.io/crates/vte) crate.
// Tried to use this crate, but it doesn't work for opposite way (terminal -> sequence),
// because there're couple of exceptions we have to handle and it doesn't make much
// sense to add them to the vte crate. An example is Esc key where we need to know if
// there's additional input available or not and then the decision is made if the
// Esc char is dispatched immediately (user hits just Esc key) or if it's an escape/csi/...
// sequence.
//
const MAX_PARAMETERS: usize = 30;
const DEFAULT_PARAMETER_VALUE: u64 = 0;
const MAX_UTF8_CODE_POINTS: usize = 4;

/// A parser engine state.
///
/// All these variant names come from the
/// [A parser for DEC’s ANSI-compatible video terminals](https://vt100.net/emu/dec_ansi_parser)
/// description.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    /// Initial state.
    Ground,
    /// Escape sequence started.
    ///
    /// `Esc` received with a flag that there's more data available.
    Escape,
    /// Escape sequence and we're collecting intermediates.
    ///
    /// # Notes
    ///
    /// This implementation doesn't collect intermediates. It just handles the state
    /// to distinguish between (im)proper sequences.
    EscapeIntermediate,
    /// CSI sequence started.
    ///
    /// `Esc` followed by the `[` received.
    CsiEntry,
    /// CSI sequence should be consumed, but not dispatched.
    CsiIgnore,
    /// CSI sequence and we're collecting parameters.
    CsiParameter,
    /// CSI sequence and we're collecting intermediates.
    ///
    /// # Notes
    ///
    /// This implementation doesn't collect intermediates. It just handles the state
    /// to distinguish between (im)proper sequences.
    CsiIntermediate,
    /// Possible UTF-8 sequence and we're collecting UTF-8 code points.
    Utf8,
}

pub(crate) trait Provide {
    fn provide_char(&mut self, ch: char);

    fn provide_esc_sequence(&mut self, ch: char);

    fn provide_csi_sequence(&mut self, parameters: &[u64], ignored_count: usize, ch: char);
}

pub(crate) struct Engine {
    parameters: [u64; MAX_PARAMETERS],
    parameters_count: usize,
    parameter: u64,
    ignored_parameters_count: usize,
    state: State,
    utf8_points: [u8; MAX_UTF8_CODE_POINTS],
    utf8_points_count: usize,
    utf8_points_expected_count: usize,
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            parameters: [DEFAULT_PARAMETER_VALUE; MAX_PARAMETERS],
            parameters_count: 0,
            parameter: DEFAULT_PARAMETER_VALUE,
            ignored_parameters_count: 0,
            state: State::Ground,
            utf8_points: [0; MAX_UTF8_CODE_POINTS],
            utf8_points_count: 0,
            utf8_points_expected_count: 0,
        }
    }
}

impl Engine {
    fn set_state(&mut self, state: State) {
        if let State::Ground = state {
            self.parameters_count = 0;
            self.parameter = DEFAULT_PARAMETER_VALUE;
            self.ignored_parameters_count = 0;
            self.utf8_points_count = 0;
            self.utf8_points_expected_count = 0;
        }
        self.state = state;
    }

    fn store_parameter(&mut self) {
        if self.parameters_count < MAX_PARAMETERS {
            self.parameters[self.parameters_count] = self.parameter;
            self.parameters_count += 1;
        } else {
            self.ignored_parameters_count += 1;
        }
        self.parameter = DEFAULT_PARAMETER_VALUE;
    }

    fn handle_possible_esc(&mut self, provider: &mut dyn Provide, byte: u8, more: bool) -> bool {
        if byte != 0x1B {
            return false;
        }

        match (self.state, more) {
            // More input means possible Esc sequence, just switch state and wait
            (State::Ground, true) => self.set_state(State::Escape),

            // No more input means Esc key, dispatch it
            (State::Ground, false) => provider.provide_char('\x1B'),

            // More input means possible Esc sequence, dispatch the previous Esc char
            (State::Escape, true) => provider.provide_char('\x1B'),

            // No more input means Esc key, dispatch the previous & current Esc char
            (State::Escape, false) => {
                provider.provide_char('\x1B');
                provider.provide_char('\x1B');
                self.set_state(State::Ground);
            }

            // Discard any state
            // More input means possible Esc sequence
            (_, true) => self.set_state(State::Escape),

            // Discard any state
            // No more input means Esc key, dispatch it
            (_, false) => {
                provider.provide_char('\x1B');
                self.set_state(State::Ground);
            }
        }

        true
    }

    fn handle_possible_utf8_code_points(&mut self, provider: &mut dyn Provide, byte: u8) -> bool {
        if byte & 0b1000_0000 == 0b0000_0000 {
            provider.provide_char(byte as char);
            true
        } else if byte & 0b1110_0000 == 0b1100_0000 {
            self.utf8_points_count = 1;
            self.utf8_points[0] = byte;
            self.utf8_points_expected_count = 2;
            self.set_state(State::Utf8);
            true
        } else if byte & 0b1111_0000 == 0b1110_0000 {
            self.utf8_points_count = 1;
            self.utf8_points[0] = byte;
            self.utf8_points_expected_count = 3;
            self.set_state(State::Utf8);
            true
        } else if byte & 0b1111_1000 == 0b1111_0000 {
            self.utf8_points_count = 1;
            self.utf8_points[0] = byte;
            self.utf8_points_expected_count = 4;
            self.set_state(State::Utf8);
            true
        } else {
            false
        }
    }

    fn advance_ground_state(&mut self, provider: &mut dyn Provide, byte: u8) {
        if self.handle_possible_utf8_code_points(provider, byte) {
            return;
        }

        match byte {
            0x1B => unreachable!(),

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => provider.provide_char(byte as char),

            // Print
            0x20..=0x7F => provider.provide_char(byte as char),

            _ => {}
        };
    }

    fn advance_escape_state(&mut self, provider: &mut dyn Provide, byte: u8) {
        match byte {
            0x1B => unreachable!(),

            // Intermediate bytes to collect
            0x20..=0x2F => {
                self.set_state(State::EscapeIntermediate);
            }

            // Escape followed by '[' (0x5B)
            //   -> CSI sequence start
            0x5B => self.set_state(State::CsiEntry),

            // Escape sequence final character
            0x30..=0x4F | 0x51..=0x57 | 0x59 | 0x5A | 0x5C | 0x60..=0x7E => {
                provider.provide_esc_sequence(byte as char);
                self.set_state(State::Ground);
            }

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => provider.provide_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x7F => {}

            // Other bytes are considered as invalid -> cancel whatever we have
            _ => self.set_state(State::Ground),
        };
    }

    fn advance_escape_intermediate_state(&mut self, provider: &mut dyn Provide, byte: u8) {
        match byte {
            0x1B => unreachable!(),

            // Intermediate bytes to collect
            0x20..=0x2F => {}

            // Escape followed by '[' (0x5B)
            //   -> CSI sequence start
            0x5B => self.set_state(State::CsiEntry),

            // Escape sequence final character
            0x30..=0x5A | 0x5C..=0x7E => {
                provider.provide_esc_sequence(byte as char);
                self.set_state(State::Ground);
            }

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => provider.provide_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x7F => {}

            // Other bytes are considered as invalid -> cancel whatever we have
            _ => self.set_state(State::Ground),
        };
    }

    fn advance_csi_entry_state(&mut self, provider: &mut dyn Provide, byte: u8) {
        match byte {
            0x1B => unreachable!(),

            // Semicolon = parameter delimiter
            0x3B => {
                self.store_parameter();
                self.set_state(State::CsiParameter);
            }

            // '0' ..= '9' = parameter value
            0x30..=0x39 => {
                self.parameter = (byte as u64) - 0x30;
                self.set_state(State::CsiParameter);
            }

            0x3A => self.set_state(State::CsiIgnore),

            // CSI sequence final character
            //   -> dispatch CSI sequence
            0x40..=0x7E => {
                provider.provide_csi_sequence(
                    &self.parameters[..self.parameters_count],
                    self.ignored_parameters_count,
                    byte as char,
                );

                self.set_state(State::Ground);
            }

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => provider.provide_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x7F => {}

            // Collect rest as parameters
            _ => {
                self.parameter = byte as u64;
                self.store_parameter();
            }
        };
    }

    fn advance_csi_ignore_state(&mut self, provider: &mut dyn Provide, byte: u8) {
        match byte {
            0x1B => unreachable!(),

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => provider.provide_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x20..=0x3F | 0x7F => {}

            0x40..=0x7E => self.set_state(State::Ground),

            // Other bytes are considered as invalid -> cancel whatever we have
            _ => self.set_state(State::Ground),
        };
    }

    fn advance_csi_parameter_state(&mut self, provider: &mut dyn Provide, byte: u8) {
        match byte {
            0x1B => unreachable!(),

            // '0' ..= '9' = parameter value
            0x30..=0x39 => {
                self.parameter = self.parameter.saturating_mul(10);
                self.parameter = self.parameter.saturating_add((byte as u64) - 0x30);
            }

            // Semicolon = parameter delimiter
            0x3B => self.store_parameter(),

            // CSI sequence final character
            //   -> dispatch CSI sequence
            0x40..=0x7E => {
                self.store_parameter();
                provider.provide_csi_sequence(
                    &self.parameters[..self.parameters_count],
                    self.ignored_parameters_count,
                    byte as char,
                );

                self.set_state(State::Ground);
            }

            // Intermediates to collect
            0x20..=0x2F => {
                self.store_parameter();
                self.set_state(State::CsiIntermediate);
            }

            // Ignore
            0x3A | 0x3C..=0x3F => self.set_state(State::CsiIgnore),

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => provider.provide_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x7F => {}

            // Other bytes are considered as invalid -> cancel whatever we have
            _ => self.set_state(State::Ground),
        };
    }

    fn advance_csi_intermediate_state(&mut self, provider: &mut dyn Provide, byte: u8) {
        match byte {
            0x1B => unreachable!(),

            // Intermediates to collect
            0x20..=0x2F => {}

            // CSI sequence final character
            //   -> dispatch CSI sequence
            0x40..=0x7E => {
                provider.provide_csi_sequence(
                    &self.parameters[..self.parameters_count],
                    self.ignored_parameters_count,
                    byte as char,
                );

                self.set_state(State::Ground);
            }

            // Execute
            0x00..=0x17 | 0x19 | 0x1C..=0x1F => provider.provide_char(byte as char),

            // TODO Does it mean we should ignore the whole sequence?
            // Ignore
            0x7F => {}

            // Other bytes are considered as invalid -> cancel whatever we have
            _ => self.set_state(State::Ground),
        }
    }

    fn advance_utf8_state(&mut self, provider: &mut dyn Provide, byte: u8) {
        if byte & 0b1100_0000 != 0b1000_0000 {
            self.set_state(State::Ground);
            return;
        }

        self.utf8_points[self.utf8_points_count] = byte;
        self.utf8_points_count += 1;

        if self.utf8_points_count == self.utf8_points_expected_count {
            if let Some(ch) = std::str::from_utf8(&self.utf8_points[..self.utf8_points_count])
                .ok()
                .and_then(|s| s.chars().next())
            {
                provider.provide_char(ch);
            }
            self.set_state(State::Ground);
        }
    }

    pub(crate) fn advance(&mut self, provider: &mut dyn Provide, byte: u8, more: bool) {
        // eprintln!("advance: {:?} {} {}", self.state, byte, more);

        if self.handle_possible_esc(provider, byte, more) {
            return;
        }

        match self.state {
            State::Ground => self.advance_ground_state(provider, byte),
            State::Escape => self.advance_escape_state(provider, byte),
            State::EscapeIntermediate => self.advance_escape_intermediate_state(provider, byte),
            State::CsiEntry => self.advance_csi_entry_state(provider, byte),
            State::CsiIgnore => self.advance_csi_ignore_state(provider, byte),
            State::CsiParameter => self.advance_csi_parameter_state(provider, byte),
            State::CsiIntermediate => self.advance_csi_intermediate_state(provider, byte),
            State::Utf8 => self.advance_utf8_state(provider, byte),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn esc_char() {
        let mut engine = Engine::default();
        let mut provider = CharProvider::default();

        // No more input means that the Esc character should be dispatched immediately
        engine.advance(&mut provider, 0x1B, false);
        assert_eq!(provider.chars, &['\x1B']);

        // There's more input so the machine should wait before dispatching Esc character
        engine.advance(&mut provider, 0x1B, true);
        assert_eq!(provider.chars, &['\x1B']);

        // Another Esc character, but no more input, machine should dispatch the postponed Esc
        // character and the new one too.
        engine.advance(&mut provider, 0x1B, false);
        assert_eq!(provider.chars, &['\x1B', '\x1B', '\x1B']);
    }

    #[test]
    fn esc_without_intermediates() {
        let mut engine = Engine::default();
        let mut provider = EscProvider::default();

        let input = b"\x1B0\x1B~";
        advance(&mut engine, &mut provider, input, false);

        assert_eq!(provider.chars.len(), 2);

        assert_eq!(provider.chars[0], '0');

        assert_eq!(provider.chars[1], '~');
    }

    #[test]
    fn csi_without_parameters() {
        let mut engine = Engine::default();
        let mut provider = CsiProvider::default();

        let input = b"\x1B\x5Bm";
        advance(&mut engine, &mut provider, input, false);

        assert_eq!(provider.parameters.len(), 1);
        assert_eq!(provider.parameters[0], &[]);
        assert_eq!(provider.chars.len(), 1);
        assert_eq!(provider.chars[0], 'm');
    }

    #[test]
    fn csi_with_two_default_parameters() {
        let mut engine = Engine::default();
        let mut provider = CsiProvider::default();

        let input = b"\x1B\x5B;m";
        advance(&mut engine, &mut provider, input, false);

        assert_eq!(provider.parameters.len(), 1);
        assert_eq!(
            provider.parameters[0],
            &[DEFAULT_PARAMETER_VALUE, DEFAULT_PARAMETER_VALUE]
        );
        assert_eq!(provider.chars.len(), 1);
        assert_eq!(provider.chars[0], 'm');
    }

    #[test]
    fn csi_with_trailing_semicolon() {
        let mut engine = Engine::default();
        let mut provider = CsiProvider::default();

        let input = b"\x1B\x5B123;m";
        advance(&mut engine, &mut provider, input, false);

        assert_eq!(provider.parameters.len(), 1);
        assert_eq!(provider.parameters[0], &[123, DEFAULT_PARAMETER_VALUE]);
        assert_eq!(provider.chars.len(), 1);
        assert_eq!(provider.chars[0], 'm');
    }

    #[test]
    fn csi_max_parameters() {
        let mut engine = Engine::default();
        let mut provider = CsiProvider::default();

        let input = b"\x1B\x5B1;2;3;4;5;6;7;8;9;10;11;12;13;14;15;16;17;18;19;20;21;22;23;24;25;26;27;28;29;30m";
        advance(&mut engine, &mut provider, input, false);

        assert_eq!(provider.parameters.len(), 1);
        assert_eq!(provider.parameters[0].len(), MAX_PARAMETERS);
        assert_eq!(
            provider.parameters[0],
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30
            ]
        );
        assert_eq!(provider.chars.len(), 1);
        assert_eq!(provider.chars[0], 'm');
    }

    #[test]
    fn test_parse_utf8_character() {
        let mut engine = Engine::default();
        let mut provider = CharProvider::default();

        advance(&mut engine, &mut provider, &['a' as u8], false);
        assert_eq!(provider.chars.len(), 1);
        assert_eq!(provider.chars[0], 'a');

        advance(&mut engine, &mut provider, &[0xC3, 0xB1], false);
        assert_eq!(provider.chars.len(), 2);
        assert_eq!(provider.chars[1], 'ñ');

        advance(&mut engine, &mut provider, &[0xE2, 0x81, 0xA1], false);
        assert_eq!(provider.chars.len(), 3);
        assert_eq!(provider.chars[2], '\u{2061}');

        advance(&mut engine, &mut provider, &[0xF0, 0x90, 0x8C, 0xBC], false);
        assert_eq!(provider.chars.len(), 4);
        assert_eq!(provider.chars[3], '𐌼');
    }

    fn advance(engine: &mut Engine, provider: &mut dyn Provide, bytes: &[u8], more: bool) {
        let len = bytes.len();

        for (i, byte) in bytes.iter().enumerate() {
            engine.advance(provider, *byte, i < len - 1 || more);
        }
    }

    #[derive(Default)]
    struct CharProvider {
        chars: Vec<char>,
    }

    impl Provide for CharProvider {
        fn provide_char(&mut self, ch: char) {
            self.chars.push(ch);
        }

        fn provide_esc_sequence(&mut self, _ch: char) {}

        fn provide_csi_sequence(&mut self, _parameters: &[u64], _ignored_count: usize, _ch: char) {}
    }

    #[derive(Default)]
    struct CsiProvider {
        parameters: Vec<Vec<u64>>,
        chars: Vec<char>,
    }

    impl Provide for CsiProvider {
        fn provide_char(&mut self, _ch: char) {}

        fn provide_esc_sequence(&mut self, _ch: char) {}

        fn provide_csi_sequence(&mut self, parameters: &[u64], _ignored_count: usize, ch: char) {
            self.parameters.push(parameters.to_vec());
            self.chars.push(ch);
        }
    }

    #[derive(Default)]
    struct EscProvider {
        chars: Vec<char>,
    }

    impl Provide for EscProvider {
        fn provide_char(&mut self, _ch: char) {}

        fn provide_esc_sequence(&mut self, ch: char) {
            self.chars.push(ch);
        }

        fn provide_csi_sequence(&mut self, _parameters: &[u64], _ignored_count: usize, _ch: char) {}
    }
}
