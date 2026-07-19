//! OSC 133 ("semantic prompt") tracking for click-to-move-cursor.
//!
//! `vte 0.15` — the exact crate this terminal builds against — silently drops
//! OSC 133: there is no `vte::ansi::Handler` hook for it, so it falls through to
//! an internal `unhandled` log arm and never reaches the parsed grid. That means
//! the grid we render carries no record of the shell's prompt / input / output
//! boundaries.
//!
//! To recover them we tee the raw PTY byte stream (see
//! [`crate::alacritty::ScanningPty`]) into an [`Osc133Scanner`], which detects
//! the `A`/`B`/`C`/`D` marks and drives the small [`SemanticPromptState`] phase
//! machine below. The scanner runs on the PTY reader thread; the state is shared
//! with the UI thread, which consults it when a click lands.
//!
//! Phase 1 is deliberately **position-free**. The click gate is purely
//! `phase == Input`, and for shells that advertise `click_events` (fish,
//! nushell) the click is reported to the shell with the live mouse coordinate so
//! the shell moves its own cursor — correct for wide characters, combining
//! marks and multi-line input alike. Tracking prompt/input *positions* (needed
//! for the `cl` arrow-synthesis fallback that non-`click_events` shells would
//! use, and for rejecting clicks placed before the prompt) is left to a later
//! phase; fish never needs it.

/// How the shell wants clicks handled, negotiated from the `OSC 133 ; A` options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClickMode {
    /// The shell advertised no click support; clicking must be a no-op.
    #[default]
    None,
    /// `click_events=N`: the shell repositions its own cursor when we report a
    /// click. `N == 1` is absolute screen coordinates (fish); `N == 2` is
    /// prompt-relative (handled in a later phase, since it needs the prompt
    /// position).
    ClickEvents(u8),
    /// `cl=...`: the terminal is expected to synthesize arrow keys across the
    /// input cells. Recorded for completeness; arrow synthesis is a later phase
    /// and no-ops for now.
    Cl,
}

/// Where the shell cursor sits in the prompt/command lifecycle, per OSC 133.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Phase {
    /// No prompt seen yet, or the last command finished (`D`).
    #[default]
    Idle,
    /// Between prompt-start (`A`) and input-start (`B`): the prompt is drawing.
    Prompt,
    /// Between input-start (`B`) and command-start (`C`): the user is editing the
    /// command line. This is the only phase where click-to-move fires.
    Input,
    /// Between command-start (`C`) and command-end (`D`): command output. A click
    /// here (a REPL, `cat`, a pager on the primary screen) must never move the
    /// cursor — this is what closes the misfire the naive implementation had.
    Output,
}

/// Live, position-free semantic state consulted by the click handler. Shared
/// between the PTY reader thread (which mutates it through [`Osc133Scanner`]) and
/// the UI thread (which reads it in `mouse_up`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SemanticPromptState {
    pub phase: Phase,
    pub click: ClickMode,
}

impl SemanticPromptState {
    /// The click-to-move gate: true only while the user is editing the command
    /// line at a prompt whose shell asked us to report clicks. With no OSC 133
    /// input region — every shell without shell integration, and every full-screen
    /// program — this is always false, so a click is a plain no-op.
    pub fn accepts_click(&self) -> bool {
        self.phase == Phase::Input && self.click != ClickMode::None
    }

    // Only the OSC 133 tee (`#[cfg(unix)]`) and the unit tests drive the phase
    // machine, so gate `apply` to match its consumers: on a non-test Windows
    // build there is no tee, and leaving it in would be dead code that Zed's
    // `-D warnings` CI rejects.
    #[cfg(any(unix, test))]
    fn apply(&mut self, command: u8, options: &[u8]) {
        match command {
            b'A' => {
                self.phase = Phase::Prompt;
                self.click = parse_click(options);
            }
            b'B' => self.phase = Phase::Input,
            b'C' => {
                self.phase = Phase::Output;
                // Click support is negotiated per prompt (on `A`). Clearing it
                // when the command starts keeps a later stray `B` — from command
                // output that itself contains `ESC]133;B` — from reopening the
                // click gate with the previous prompt's mode.
                self.click = ClickMode::None;
            }
            b'D' => {
                self.phase = Phase::Idle;
                self.click = ClickMode::None;
            }
            _ => {}
        }
    }
}

/// `ESC ] 1 3 3 ;` — the introducer every OSC 133 mark starts with.
#[cfg(any(unix, test))]
const INTRO: &[u8] = b"\x1b]133;";

/// Cap on how much of an OSC 133 payload we buffer. `A`/`B`/`C`/`D` plus the
/// short options we parse (`click_events=…`, `cl=…`) fit easily; the only long
/// payload is fish's `C;cmdline_url=…`, whose contents we never read, so
/// truncating past this bound is safe.
#[cfg(any(unix, test))]
const MAX_PAYLOAD: usize = 1024;

// The scanner below is driven only by the `#[cfg(unix)]` PTY tee and the unit
// tests; gate it to those consumers so a non-test Windows build carries no dead
// code (Zed CI builds with `-D warnings`).
#[cfg(any(unix, test))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ScanState {
    /// Outside any OSC 133 sequence.
    #[default]
    Ground,
    /// Matched the first `n` bytes of [`INTRO`] (`1 <= n < INTRO.len()`).
    Intro(usize),
    /// Inside the payload, collecting bytes until the string terminator.
    Payload,
    /// Saw `ESC` inside the payload; a following `\` completes an `ESC \` ST.
    PayloadEsc,
}

/// Streaming detector for OSC 133 marks. Fed the raw PTY bytes in whatever
/// arbitrary chunks the reader hands it — a single mark may straddle two reads —
/// and applies each completed mark to the shared [`SemanticPromptState`]. Only
/// the in-progress-sequence state persists between calls, so splits are handled
/// for free.
#[cfg(any(unix, test))]
#[derive(Debug, Default)]
pub struct Osc133Scanner {
    state: ScanState,
    payload: Vec<u8>,
}

#[cfg(any(unix, test))]
impl Osc133Scanner {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed a chunk of raw PTY output, updating `state` for every OSC 133 mark
    /// completed within it.
    pub fn feed(&mut self, bytes: &[u8], state: &mut SemanticPromptState) {
        for &byte in bytes {
            self.step(byte, state);
        }
    }

    fn step(&mut self, byte: u8, state: &mut SemanticPromptState) {
        match self.state {
            ScanState::Ground => self.enter_intro(byte),
            ScanState::Intro(n) => {
                if byte == INTRO[n] {
                    if n + 1 == INTRO.len() {
                        self.state = ScanState::Payload;
                        self.payload.clear();
                    } else {
                        self.state = ScanState::Intro(n + 1);
                    }
                } else {
                    // Mismatch. `INTRO` has no repeated prefix, so the only
                    // partial match a byte can revive is a fresh `ESC`; anything
                    // else drops us back to the ground state.
                    self.state = ScanState::Ground;
                    self.enter_intro(byte);
                }
            }
            ScanState::Payload => match byte {
                0x07 => self.finish(state), // BEL terminator
                0x1b => self.state = ScanState::PayloadEsc,
                _ => {
                    // Past the cap we stop storing but keep scanning for the
                    // terminator so we still resynchronize (only the unused
                    // `cmdline_url` payload is ever this long).
                    if self.payload.len() < MAX_PAYLOAD {
                        self.payload.push(byte);
                    }
                }
            },
            ScanState::PayloadEsc => {
                if byte == b'\\' {
                    self.finish(state); // ESC \ (ST) terminator
                } else {
                    // A bare ESC that is not part of an ST aborts the string
                    // (ECMA-48). That ESC is itself the start of the next control
                    // sequence, so treat it as a fresh introducer (we already
                    // consumed it) and re-feed the current byte as the one that
                    // follows it. This resynchronizes even when the aborting ESC
                    // is the introducer of an immediately-following `ESC]133;…`
                    // with no bytes in between.
                    self.reset();
                    self.state = ScanState::Intro(1);
                    self.step(byte, state);
                }
            }
        }
    }

    /// Begin matching a new introducer if `byte` is its first character.
    fn enter_intro(&mut self, byte: u8) {
        self.state = if byte == INTRO[0] {
            ScanState::Intro(1)
        } else {
            ScanState::Ground
        };
    }

    /// A terminator was reached: parse the buffered payload and apply the mark.
    fn finish(&mut self, state: &mut SemanticPromptState) {
        let command_end = self
            .payload
            .iter()
            .position(|&b| b == b';')
            .unwrap_or(self.payload.len());
        let command = &self.payload[..command_end];
        // Marks are a single letter; ignore anything else (including the empty
        // payload of a bare `ESC ] 1 3 3 ; ST`).
        if command.len() == 1 {
            let options = self.payload.get(command_end + 1..).unwrap_or(&[]);
            state.apply(command[0], options);
        }
        self.reset();
    }

    fn reset(&mut self) {
        self.state = ScanState::Ground;
        self.payload.clear();
    }
}

/// Negotiate the click mode from an `OSC 133 ; A` option string.
/// `click_events` takes priority over `cl`, matching Ghostty.
#[cfg(any(unix, test))]
fn parse_click(options: &[u8]) -> ClickMode {
    let mut cl = false;
    for option in options.split(|&b| b == b';') {
        if let Some(value) = option.strip_prefix(b"click_events=") {
            // An empty, non-numeric or out-of-range value is not a valid
            // negotiation. Return `None` rather than defaulting to
            // `ClickEvents(1)`, so garbage can never fire as an absolute-mode
            // press. `click_events` still takes priority over `cl`, so we do not
            // fall through to the `cl` fallback here.
            return match std::str::from_utf8(value)
                .ok()
                .and_then(|s| s.trim().parse::<u8>().ok())
            {
                Some(n) => ClickMode::ClickEvents(n),
                None => ClickMode::None,
            };
        }
        if option.starts_with(b"cl=") {
            cl = true;
        }
    }
    if cl { ClickMode::Cl } else { ClickMode::None }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Feed a whole byte stream through a fresh scanner and return the resulting
    /// state.
    fn scan(bytes: &[u8]) -> SemanticPromptState {
        let mut scanner = Osc133Scanner::new();
        let mut state = SemanticPromptState::default();
        scanner.feed(bytes, &mut state);
        state
    }

    #[test]
    fn marks_drive_the_phase_machine() {
        // A -> Prompt, B -> Input, C -> Output, D -> Idle.
        assert_eq!(scan(b"\x1b]133;A\x07").phase, Phase::Prompt);
        assert_eq!(scan(b"\x1b]133;A\x07\x1b]133;B\x07").phase, Phase::Input);
        assert_eq!(
            scan(b"\x1b]133;A\x07\x1b]133;B\x07\x1b]133;C\x07").phase,
            Phase::Output
        );
        assert_eq!(
            scan(b"\x1b]133;A\x07\x1b]133;B\x07\x1b]133;C\x07\x1b]133;D;0\x07").phase,
            Phase::Idle
        );
    }

    #[test]
    fn click_mode_negotiation() {
        // fish: advertises click_events=1 on prompt-start.
        assert_eq!(
            scan(b"\x1b]133;A;click_events=1\x07").click,
            ClickMode::ClickEvents(1)
        );
        // click_events=2 (prompt-relative) is parsed even though Phase 1 no-ops it.
        assert_eq!(
            scan(b"\x1b]133;A;click_events=2\x07").click,
            ClickMode::ClickEvents(2)
        );
        // cl fallback.
        assert_eq!(scan(b"\x1b]133;A;cl=line\x07").click, ClickMode::Cl);
        // click_events beats cl when both are advertised.
        assert_eq!(
            scan(b"\x1b]133;A;cl=line;click_events=1\x07").click,
            ClickMode::ClickEvents(1)
        );
        // No options -> None -> never fires.
        assert_eq!(scan(b"\x1b]133;A\x07").click, ClickMode::None);
    }

    #[test]
    fn gate_open_in_input_closed_otherwise() {
        // Open: prompt advertised click_events, cursor in the input region.
        let open = scan(b"\x1b]133;A;click_events=1\x07$ \x1b]133;B\x07");
        assert!(open.accepts_click());

        // Closed after command-start: a click during output must not fire.
        let output = scan(b"\x1b]133;A;click_events=1\x07$ \x1b]133;B\x07ls\r\n\x1b]133;C\x07");
        assert!(!output.accepts_click());

        // Closed with no click support even while in the input phase.
        let no_click = scan(b"\x1b]133;A\x07$ \x1b]133;B\x07");
        assert_eq!(no_click.phase, Phase::Input);
        assert!(!no_click.accepts_click());
    }

    #[test]
    fn no_osc_133_leaves_state_idle() {
        let state = scan(b"just some plain output\r\n$ echo hi\r\n");
        assert_eq!(state.phase, Phase::Idle);
        assert!(!state.accepts_click());
    }

    #[test]
    fn accepts_st_terminator() {
        // fish terminates prompt-start with ESC \ (ST), not BEL.
        let state = scan(b"\x1b]133;A;click_events=1\x1b\\$ \x1b]133;B\x1b\\");
        assert_eq!(state.click, ClickMode::ClickEvents(1));
        assert!(state.accepts_click());
    }

    #[test]
    fn osc_split_across_feeds() {
        let mut scanner = Osc133Scanner::new();
        let mut state = SemanticPromptState::default();
        // Split the introducer itself across two reads.
        scanner.feed(b"\x1b]13", &mut state);
        scanner.feed(b"3;A;click_events=1\x07", &mut state);
        assert_eq!(state.click, ClickMode::ClickEvents(1));
        assert_eq!(state.phase, Phase::Prompt);

        // Split inside the payload across two reads.
        let mut scanner = Osc133Scanner::new();
        let mut state = SemanticPromptState::default();
        scanner.feed(b"\x1b]133;A;click_ev", &mut state);
        scanner.feed(b"ents=1\x07", &mut state);
        assert_eq!(state.click, ClickMode::ClickEvents(1));

        // Split right before the terminator.
        let mut scanner = Osc133Scanner::new();
        let mut state = SemanticPromptState::default();
        scanner.feed(b"\x1b]133;B", &mut state);
        scanner.feed(b"\x07", &mut state);
        assert_eq!(state.phase, Phase::Input);
    }

    #[test]
    fn full_fish_lifecycle_reopens_each_prompt() {
        // Two prompts in a row: after the first command ends (D) and the second
        // prompt opens (A;B), the gate is open again with click_events intact.
        let stream: &[u8] = b"\x1b]133;A;click_events=1\x07$ \x1b]133;B\x07ls\r\n\
            \x1b]133;C;cmdline_url=file%3A%2F%2Fls\x07file.txt\r\n\x1b]133;D;0\x07\
            \x1b]133;A;click_events=1\x07$ \x1b]133;B\x07";
        let state = scan(stream);
        assert_eq!(state.phase, Phase::Input);
        assert_eq!(state.click, ClickMode::ClickEvents(1));
        assert!(state.accepts_click());
    }

    #[test]
    fn long_cmdline_url_payload_is_tolerated() {
        // A pathological C payload longer than MAX_PAYLOAD must not break the
        // scanner: it truncates the (unused) payload and still resynchronizes on
        // the terminator, so the following D is seen.
        let mut stream = b"\x1b]133;C;cmdline_url=".to_vec();
        stream.extend(std::iter::repeat(b'x').take(MAX_PAYLOAD * 3));
        stream.extend_from_slice(b"\x07\x1b]133;D;0\x07");
        let state = scan(&stream);
        assert_eq!(state.phase, Phase::Idle);
    }

    #[test]
    fn embedded_esc_aborts_and_resyncs() {
        // A bare ESC that is not an ST aborts the string; the next real mark is
        // still detected.
        let state = scan(b"\x1b]133;A;click_events=1\x1bX garbage \x1b]133;B\x07");
        assert_eq!(state.phase, Phase::Input);
    }

    #[test]
    fn back_to_back_osc_after_unterminated_resyncs() {
        // A bare ESC inside a payload aborts the OSC *and* begins the next control
        // sequence (ECMA-48). When that ESC is the introducer of an
        // immediately-following `ESC]133;…` — with no bytes in between — the next
        // mark must still be detected. `embedded_esc_aborts_and_resyncs` only
        // passes because the ` garbage ` gap sidesteps this path.

        // Unterminated `A` directly followed by `B`: the `B` is detected (phase
        // reaches `Input`). The gate stays closed because the aborted `A` never
        // applied, so `click` was never negotiated.
        let dropped = scan(b"\x1b]133;A;click_events=1\x1b]133;B\x07");
        assert_eq!(dropped.phase, Phase::Input);
        assert!(!dropped.accepts_click());

        // A completed `A` (click negotiated), then an empty unterminated mark
        // whose aborting ESC introduces a real `B`: the gate reopens with the
        // negotiated click intact.
        let recovered = scan(b"\x1b]133;A;click_events=1\x07\x1b]133;\x1b]133;B\x07");
        assert_eq!(recovered.phase, Phase::Input);
        assert!(recovered.accepts_click());
    }

    #[test]
    fn command_start_closes_the_click_gate() {
        // Once command output begins (`C`), the negotiated click mode is cleared,
        // so a later stray `B` — e.g. a file `cat`'d to the terminal that contains
        // `ESC]133;B` — flips the phase back to `Input` but cannot reopen the gate
        // without a fresh `A;click_events`.
        let reopened = scan(
            b"\x1b]133;A;click_events=1\x07$ \x1b]133;B\x07ls\r\n\
            \x1b]133;C\x07output\r\n\x1b]133;B\x07",
        );
        assert_eq!(reopened.phase, Phase::Input);
        assert_eq!(reopened.click, ClickMode::None);
        assert!(!reopened.accepts_click());

        // `D` clears it too, defensively (it already is after `C`).
        let after_d = scan(
            b"\x1b]133;A;click_events=1\x07\x1b]133;B\x07\x1b]133;C\x07\
            \x1b]133;D;0\x07\x1b]133;B\x07",
        );
        assert_eq!(after_d.click, ClickMode::None);
        assert!(!after_d.accepts_click());
    }

    #[test]
    fn malformed_click_events_do_not_fire() {
        // Empty, non-numeric and out-of-range values are not valid negotiations:
        // they must yield `None`, never a default `ClickEvents(1)` that would fire
        // as an absolute-mode press. No real shell emits these; this is purely
        // defensive.
        assert_eq!(scan(b"\x1b]133;A;click_events=\x07").click, ClickMode::None);
        assert_eq!(
            scan(b"\x1b]133;A;click_events=abc\x07").click,
            ClickMode::None
        );
        assert_eq!(
            scan(b"\x1b]133;A;click_events=256\x07").click,
            ClickMode::None
        );
        assert_eq!(
            scan(b"\x1b]133;A;click_events=999\x07").click,
            ClickMode::None
        );
        // Valid values still parse.
        assert_eq!(
            scan(b"\x1b]133;A;click_events=0\x07").click,
            ClickMode::ClickEvents(0)
        );
        assert_eq!(
            scan(b"\x1b]133;A;click_events=2\x07").click,
            ClickMode::ClickEvents(2)
        );
    }
}
