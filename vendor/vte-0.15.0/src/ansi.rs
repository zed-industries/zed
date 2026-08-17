// SPDX-License-Identifier: Apache-2.0
//
// This module was originally part of the `alacritty_terminal` crate, which is
// licensed under the Apache License, Version 2.0 and is part of the Alacritty
// project (https://github.com/alacritty/alacritty).

//! ANSI Terminal Stream Parsing.

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::fmt::{self, Display, Formatter, Write};
#[cfg(feature = "std")]
use core::ops::Mul;
use core::ops::{Add, Sub};
use core::str::FromStr;
use core::time::Duration;
use core::{iter, mem, str};
#[cfg(feature = "std")]
use std::time::Instant;

use bitflags::bitflags;
#[doc(inline)]
pub use cursor_icon;
use cursor_icon::CursorIcon;
use log::debug;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Params, ParamsIter};

/// Maximum time before a synchronized update is aborted.
const SYNC_UPDATE_TIMEOUT: Duration = Duration::from_millis(150);

/// Maximum number of bytes read in one synchronized update (2MiB).
const SYNC_BUFFER_SIZE: usize = 0x20_0000;

/// Number of bytes in the BSU/ESU CSI sequences.
const SYNC_ESCAPE_LEN: usize = 8;

/// BSU CSI sequence for beginning or extending synchronized updates.
const BSU_CSI: [u8; SYNC_ESCAPE_LEN] = *b"\x1b[?2026h";

/// ESU CSI sequence for terminating synchronized updates.
const ESU_CSI: [u8; SYNC_ESCAPE_LEN] = *b"\x1b[?2026l";

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Hyperlink {
    /// Identifier for the given hyperlink.
    pub id: Option<String>,
    /// Resource identifier of the hyperlink.
    pub uri: String,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Implementation of [W3C's luminance algorithm].
    ///
    /// [W3C's luminance algorithm]: https://www.w3.org/TR/WCAG20/#relativeluminancedef
    #[cfg(feature = "std")]
    pub fn luminance(self) -> f64 {
        let channel_luminance = |channel| {
            let channel = channel as f64 / 255.;
            if channel <= 0.03928 {
                channel / 12.92
            } else {
                f64::powf((channel + 0.055) / 1.055, 2.4)
            }
        };

        let r_luminance = channel_luminance(self.r);
        let g_luminance = channel_luminance(self.g);
        let b_luminance = channel_luminance(self.b);

        0.2126 * r_luminance + 0.7152 * g_luminance + 0.0722 * b_luminance
    }

    /// Implementation of [W3C's contrast algorithm].
    ///
    /// [W3C's contrast algorithm]: https://www.w3.org/TR/WCAG20/#contrast-ratiodef
    #[cfg(feature = "std")]
    pub fn contrast(self, other: Rgb) -> f64 {
        let self_luminance = self.luminance();
        let other_luminance = other.luminance();

        let (darker, lighter) = if self_luminance > other_luminance {
            (other_luminance, self_luminance)
        } else {
            (self_luminance, other_luminance)
        };

        (lighter + 0.05) / (darker + 0.05)
    }
}

// A multiply function for Rgb, as the default dim is just *2/3.
#[cfg(feature = "std")]
impl Mul<f32> for Rgb {
    type Output = Rgb;

    fn mul(self, rhs: f32) -> Rgb {
        let result = Rgb {
            r: (f32::from(self.r) * rhs).clamp(0.0, 255.0) as u8,
            g: (f32::from(self.g) * rhs).clamp(0.0, 255.0) as u8,
            b: (f32::from(self.b) * rhs).clamp(0.0, 255.0) as u8,
        };

        log::trace!("Scaling RGB by {} from {:?} to {:?}", rhs, self, result);
        result
    }
}

impl Add<Rgb> for Rgb {
    type Output = Rgb;

    fn add(self, rhs: Rgb) -> Rgb {
        Rgb {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
        }
    }
}

impl Sub<Rgb> for Rgb {
    type Output = Rgb;

    fn sub(self, rhs: Rgb) -> Rgb {
        Rgb {
            r: self.r.saturating_sub(rhs.r),
            g: self.g.saturating_sub(rhs.g),
            b: self.b.saturating_sub(rhs.b),
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl FromStr for Rgb {
    type Err = ();

    fn from_str(s: &str) -> Result<Rgb, ()> {
        let chars = if s.starts_with("0x") && s.len() == 8 {
            &s[2..]
        } else if s.starts_with('#') && s.len() == 7 {
            &s[1..]
        } else {
            return Err(());
        };

        match u32::from_str_radix(chars, 16) {
            Ok(mut color) => {
                let b = (color & 0xFF) as u8;
                color >>= 8;
                let g = (color & 0xFF) as u8;
                color >>= 8;
                let r = color as u8;
                Ok(Rgb { r, g, b })
            },
            Err(_) => Err(()),
        }
    }
}

/// Parse colors in XParseColor format.
fn xparse_color(color: &[u8]) -> Option<Rgb> {
    if !color.is_empty() && color[0] == b'#' {
        parse_legacy_color(&color[1..])
    } else if color.len() >= 4 && &color[..4] == b"rgb:" {
        parse_rgb_color(&color[4..])
    } else {
        None
    }
}

/// Parse colors in `rgb:r(rrr)/g(ggg)/b(bbb)` format.
fn parse_rgb_color(color: &[u8]) -> Option<Rgb> {
    let colors = str::from_utf8(color).ok()?.split('/').collect::<Vec<_>>();

    if colors.len() != 3 {
        return None;
    }

    // Scale values instead of filling with `0`s.
    let scale = |input: &str| {
        if input.len() > 4 {
            None
        } else {
            let max = u32::pow(16, input.len() as u32) - 1;
            let value = u32::from_str_radix(input, 16).ok()?;
            Some((255 * value / max) as u8)
        }
    };

    Some(Rgb { r: scale(colors[0])?, g: scale(colors[1])?, b: scale(colors[2])? })
}

/// Parse colors in `#r(rrr)g(ggg)b(bbb)` format.
fn parse_legacy_color(color: &[u8]) -> Option<Rgb> {
    let item_len = color.len() / 3;

    // Truncate/Fill to two byte precision.
    let color_from_slice = |slice: &[u8]| {
        let col = usize::from_str_radix(str::from_utf8(slice).ok()?, 16).ok()? << 4;
        Some((col >> (4 * slice.len().saturating_sub(1))) as u8)
    };

    Some(Rgb {
        r: color_from_slice(&color[0..item_len])?,
        g: color_from_slice(&color[item_len..item_len * 2])?,
        b: color_from_slice(&color[item_len * 2..])?,
    })
}

fn parse_number(input: &[u8]) -> Option<u8> {
    if input.is_empty() {
        return None;
    }
    let mut num: u8 = 0;
    for c in input {
        let c = *c as char;
        let digit = c.to_digit(10)?;
        num = num.checked_mul(10).and_then(|v| v.checked_add(digit as u8))?;
    }
    Some(num)
}

/// Internal state for VTE processor.
#[derive(Debug, Default)]
struct ProcessorState<T: Timeout> {
    /// Last processed character for repetition.
    preceding_char: Option<char>,

    /// State for synchronized terminal updates.
    sync_state: SyncState<T>,
}

#[derive(Debug)]
struct SyncState<T: Timeout> {
    /// Handler for synchronized updates.
    timeout: T,

    /// Bytes read during the synchronized update.
    buffer: Vec<u8>,
}

impl<T: Timeout> Default for SyncState<T> {
    fn default() -> Self {
        Self { buffer: Vec::with_capacity(SYNC_BUFFER_SIZE), timeout: Default::default() }
    }
}

/// The processor wraps a `crate::Parser` to ultimately call methods on a
/// Handler.
#[cfg(feature = "std")]
#[derive(Default)]
pub struct Processor<T: Timeout = StdSyncHandler> {
    state: ProcessorState<T>,
    parser: crate::Parser,
}

/// The processor wraps a `crate::Parser` to ultimately call methods on a
/// Handler.
#[cfg(not(feature = "std"))]
#[derive(Default)]
pub struct Processor<T: Timeout> {
    state: ProcessorState<T>,
    parser: crate::Parser,
}

impl<T: Timeout> Processor<T> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Synchronized update timeout.
    pub fn sync_timeout(&self) -> &T {
        &self.state.sync_state.timeout
    }

    /// Process a new byte from the PTY.
    #[inline]
    pub fn advance<H>(&mut self, handler: &mut H, bytes: &[u8])
    where
        H: Handler,
    {
        let mut processed = 0;
        while processed != bytes.len() {
            if self.state.sync_state.timeout.pending_timeout() {
                processed += self.advance_sync(handler, &bytes[processed..]);
            } else {
                let mut performer = Performer::new(&mut self.state, handler);
                processed +=
                    self.parser.advance_until_terminated(&mut performer, &bytes[processed..]);
            }
        }
    }

    /// End a synchronized update.
    pub fn stop_sync<H>(&mut self, handler: &mut H)
    where
        H: Handler,
    {
        self.stop_sync_internal(handler, None);
    }

    /// End a synchronized update.
    ///
    /// The `bsu_offset` parameter should be passed if the sync buffer contains
    /// a new BSU escape that is not part of the current synchronized
    /// update.
    fn stop_sync_internal<H>(&mut self, handler: &mut H, bsu_offset: Option<usize>)
    where
        H: Handler,
    {
        // Process all synchronized bytes.
        //
        // NOTE: We do not use `advance_until_terminated` here since BSU sequences are
        // processed automatically during the synchronized update.
        let buffer = mem::take(&mut self.state.sync_state.buffer);
        let offset = bsu_offset.unwrap_or(buffer.len());
        let mut performer = Performer::new(&mut self.state, handler);
        self.parser.advance(&mut performer, &buffer[..offset]);
        self.state.sync_state.buffer = buffer;

        match bsu_offset {
            // Just clear processed bytes if there is a new BSU.
            //
            // NOTE: We do not need to re-process for a new ESU since the `advance_sync`
            // function checks for BSUs in reverse.
            Some(bsu_offset) => {
                let new_len = self.state.sync_state.buffer.len() - bsu_offset;
                self.state.sync_state.buffer.copy_within(bsu_offset.., 0);
                self.state.sync_state.buffer.truncate(new_len);
            },
            // Report mode and clear state if no new BSU is present.
            None => {
                handler.unset_private_mode(NamedPrivateMode::SyncUpdate.into());
                self.state.sync_state.timeout.clear_timeout();
                self.state.sync_state.buffer.clear();
            },
        }
    }

    /// Number of bytes in the synchronization buffer.
    #[inline]
    pub fn sync_bytes_count(&self) -> usize {
        self.state.sync_state.buffer.len()
    }

    /// Process a new byte during a synchronized update.
    ///
    /// Returns the number of bytes processed.
    #[cold]
    fn advance_sync<H>(&mut self, handler: &mut H, bytes: &[u8]) -> usize
    where
        H: Handler,
    {
        // Advance sync parser or stop sync if we'd exceed the maximum buffer size.
        if self.state.sync_state.buffer.len() + bytes.len() >= SYNC_BUFFER_SIZE - 1 {
            // Terminate the synchronized update.
            self.stop_sync_internal(handler, None);

            // Just parse the bytes normally.
            let mut performer = Performer::new(&mut self.state, handler);
            self.parser.advance_until_terminated(&mut performer, bytes)
        } else {
            self.state.sync_state.buffer.extend(bytes);
            self.advance_sync_csi(handler, bytes.len());
            bytes.len()
        }
    }

    /// Handle BSU/ESU CSI sequences during synchronized update.
    fn advance_sync_csi<H>(&mut self, handler: &mut H, new_bytes: usize)
    where
        H: Handler,
    {
        // Get constraints within which a new escape character might be relevant.
        let buffer_len = self.state.sync_state.buffer.len();
        let start_offset = (buffer_len - new_bytes).saturating_sub(SYNC_ESCAPE_LEN - 1);
        let end_offset = buffer_len.saturating_sub(SYNC_ESCAPE_LEN - 1);
        let search_buffer = &self.state.sync_state.buffer[start_offset..end_offset];

        // Search for termination/extension escapes in the added bytes.
        //
        // NOTE: It is technically legal to specify multiple private modes in the same
        // escape, but we only allow EXACTLY `\e[?2026h`/`\e[?2026l` to keep the parser
        // more simple.
        let mut bsu_offset = None;
        for index in memchr::memchr_iter(0x1B, search_buffer).rev() {
            let offset = start_offset + index;
            let escape = &self.state.sync_state.buffer[offset..offset + SYNC_ESCAPE_LEN];

            if escape == BSU_CSI {
                self.state.sync_state.timeout.set_timeout(SYNC_UPDATE_TIMEOUT);
                bsu_offset = Some(offset);
            } else if escape == ESU_CSI {
                self.stop_sync_internal(handler, bsu_offset);
                break;
            }
        }
    }
}

/// Helper type that implements `crate::Perform`.
///
/// Processor creates a Performer when running advance and passes the Performer
/// to `crate::Parser`.
struct Performer<'a, H: Handler, T: Timeout> {
    state: &'a mut ProcessorState<T>,
    handler: &'a mut H,

    /// Whether the parser should be prematurely terminated.
    terminated: bool,
}

impl<'a, H: Handler + 'a, T: Timeout> Performer<'a, H, T> {
    /// Create a performer.
    #[inline]
    pub fn new<'b>(state: &'b mut ProcessorState<T>, handler: &'b mut H) -> Performer<'b, H, T> {
        Performer { state, handler, terminated: Default::default() }
    }
}

#[cfg(feature = "std")]
#[derive(Default)]
pub struct StdSyncHandler {
    timeout: Option<Instant>,
}

#[cfg(feature = "std")]
impl StdSyncHandler {
    /// Synchronized update expiration time.
    #[inline]
    pub fn sync_timeout(&self) -> Option<Instant> {
        self.timeout
    }
}

#[cfg(feature = "std")]
impl Timeout for StdSyncHandler {
    #[inline]
    fn set_timeout(&mut self, duration: Duration) {
        self.timeout = Some(Instant::now() + duration);
    }

    #[inline]
    fn clear_timeout(&mut self) {
        self.timeout = None;
    }

    #[inline]
    fn pending_timeout(&self) -> bool {
        self.timeout.is_some()
    }
}

/// Interface for creating timeouts and checking their expiry.
///
/// This is internally used by the [`Processor`] to handle synchronized
/// updates.
pub trait Timeout: Default {
    /// Sets the timeout for the next synchronized update.
    ///
    /// The `duration` parameter specifies the duration of the timeout. Once the
    /// specified duration has elapsed, the synchronized update rotuine can be
    /// performed.
    fn set_timeout(&mut self, duration: Duration);
    /// Clear the current timeout.
    fn clear_timeout(&mut self);
    /// Returns whether a timeout is currently active and has not yet expired.
    fn pending_timeout(&self) -> bool;
}

/// Type that handles actions from the parser.
///
/// XXX Should probably not provide default impls for everything, but it makes
/// writing specific handler impls for tests far easier.
pub trait Handler {
    /// OSC to set window title.
    fn set_title(&mut self, _: Option<String>) {}

    /// Set the cursor style.
    fn set_cursor_style(&mut self, _: Option<CursorStyle>) {}

    /// Set the cursor shape.
    fn set_cursor_shape(&mut self, _shape: CursorShape) {}

    /// A character to be displayed.
    fn input(&mut self, _c: char) {}

    /// Set cursor to position.
    fn goto(&mut self, _line: i32, _col: usize) {}

    /// Set cursor to specific row.
    fn goto_line(&mut self, _line: i32) {}

    /// Set cursor to specific column.
    fn goto_col(&mut self, _col: usize) {}

    /// Insert blank characters in current line starting from cursor.
    fn insert_blank(&mut self, _: usize) {}

    /// Move cursor up `rows`.
    fn move_up(&mut self, _: usize) {}

    /// Move cursor down `rows`.
    fn move_down(&mut self, _: usize) {}

    /// Identify the terminal (should write back to the pty stream).
    fn identify_terminal(&mut self, _intermediate: Option<char>) {}

    /// Report device status.
    fn device_status(&mut self, _: usize) {}

    /// Move cursor forward `cols`.
    fn move_forward(&mut self, _col: usize) {}

    /// Move cursor backward `cols`.
    fn move_backward(&mut self, _col: usize) {}

    /// Move cursor down `rows` and set to column 1.
    fn move_down_and_cr(&mut self, _row: usize) {}

    /// Move cursor up `rows` and set to column 1.
    fn move_up_and_cr(&mut self, _row: usize) {}

    /// Put `count` tabs.
    fn put_tab(&mut self, _count: u16) {}

    /// Backspace `count` characters.
    fn backspace(&mut self) {}

    /// Carriage return.
    fn carriage_return(&mut self) {}

    /// Linefeed.
    fn linefeed(&mut self) {}

    /// Ring the bell.
    ///
    /// Hopefully this is never implemented.
    fn bell(&mut self) {}

    /// Substitute char under cursor.
    fn substitute(&mut self) {}

    /// Newline.
    fn newline(&mut self) {}

    /// Set current position as a tabstop.
    fn set_horizontal_tabstop(&mut self) {}

    /// Scroll up `rows` rows.
    fn scroll_up(&mut self, _: usize) {}

    /// Scroll down `rows` rows.
    fn scroll_down(&mut self, _: usize) {}

    /// Insert `count` blank lines.
    fn insert_blank_lines(&mut self, _: usize) {}

    /// Delete `count` lines.
    fn delete_lines(&mut self, _: usize) {}

    /// Erase `count` chars in current line following cursor.
    ///
    /// Erase means resetting to the default state (default colors, no content,
    /// no mode flags).
    fn erase_chars(&mut self, _: usize) {}

    /// Delete `count` chars.
    ///
    /// Deleting a character is like the delete key on the keyboard - everything
    /// to the right of the deleted things is shifted left.
    fn delete_chars(&mut self, _: usize) {}

    /// Move backward `count` tabs.
    fn move_backward_tabs(&mut self, _count: u16) {}

    /// Move forward `count` tabs.
    fn move_forward_tabs(&mut self, _count: u16) {}

    /// Save current cursor position.
    fn save_cursor_position(&mut self) {}

    /// Restore cursor position.
    fn restore_cursor_position(&mut self) {}

    /// Clear current line.
    fn clear_line(&mut self, _mode: LineClearMode) {}

    /// Clear screen.
    fn clear_screen(&mut self, _mode: ClearMode) {}

    /// Clear tab stops.
    fn clear_tabs(&mut self, _mode: TabulationClearMode) {}

    /// Set tab stops at every `interval`.
    fn set_tabs(&mut self, _interval: u16) {}

    /// Reset terminal state.
    fn reset_state(&mut self) {}

    /// Reverse Index.
    ///
    /// Move the active position to the same horizontal position on the
    /// preceding line. If the active position is at the top margin, a scroll
    /// down is performed.
    fn reverse_index(&mut self) {}

    /// Set a terminal attribute.
    fn terminal_attribute(&mut self, _attr: Attr) {}

    /// Set mode.
    fn set_mode(&mut self, _mode: Mode) {}

    /// Unset mode.
    fn unset_mode(&mut self, _mode: Mode) {}

    /// DECRPM - report mode.
    fn report_mode(&mut self, _mode: Mode) {}

    /// Set private mode.
    fn set_private_mode(&mut self, _mode: PrivateMode) {}

    /// Unset private mode.
    fn unset_private_mode(&mut self, _mode: PrivateMode) {}

    /// DECRPM - report private mode.
    fn report_private_mode(&mut self, _mode: PrivateMode) {}

    /// DECSTBM - Set the terminal scrolling region.
    fn set_scrolling_region(&mut self, _top: usize, _bottom: Option<usize>) {}

    /// DECKPAM - Set keypad to applications mode (ESCape instead of digits).
    fn set_keypad_application_mode(&mut self) {}

    /// DECKPNM - Set keypad to numeric mode (digits instead of ESCape seq).
    fn unset_keypad_application_mode(&mut self) {}

    /// Set one of the graphic character sets, G0 to G3, as the active charset.
    ///
    /// 'Invoke' one of G0 to G3 in the GL area. Also referred to as shift in,
    /// shift out and locking shift depending on the set being activated.
    fn set_active_charset(&mut self, _: CharsetIndex) {}

    /// Assign a graphic character set to G0, G1, G2 or G3.
    ///
    /// 'Designate' a graphic character set as one of G0 to G3, so that it can
    /// later be 'invoked' by `set_active_charset`.
    fn configure_charset(&mut self, _: CharsetIndex, _: StandardCharset) {}

    /// Set an indexed color value.
    fn set_color(&mut self, _: usize, _: Rgb) {}

    /// Respond to a color query escape sequence.
    fn dynamic_color_sequence(&mut self, _: String, _: usize, _: &str) {}

    /// Reset an indexed color to original value.
    fn reset_color(&mut self, _: usize) {}

    /// Store data into clipboard.
    fn clipboard_store(&mut self, _: u8, _: &[u8]) {}

    /// Load data from clipboard.
    fn clipboard_load(&mut self, _: u8, _: &str) {}

    /// Run the decaln routine.
    fn decaln(&mut self) {}

    /// Push a title onto the stack.
    fn push_title(&mut self) {}

    /// Pop the last title from the stack.
    fn pop_title(&mut self) {}

    /// Report text area size in pixels.
    fn text_area_size_pixels(&mut self) {}

    /// Report text area size in characters.
    fn text_area_size_chars(&mut self) {}

    /// Set hyperlink.
    fn set_hyperlink(&mut self, _: Option<Hyperlink>) {}

    /// Set mouse cursor icon.
    fn set_mouse_cursor_icon(&mut self, _: CursorIcon) {}

    /// Report current keyboard mode.
    fn report_keyboard_mode(&mut self) {}

    /// Push keyboard mode into the keyboard mode stack.
    fn push_keyboard_mode(&mut self, _mode: KeyboardModes) {}

    /// Pop the given amount of keyboard modes from the
    /// keyboard mode stack.
    fn pop_keyboard_modes(&mut self, _to_pop: u16) {}

    /// Set the [`keyboard mode`] using the given [`behavior`].
    ///
    /// [`keyboard mode`]: crate::ansi::KeyboardModes
    /// [`behavior`]: crate::ansi::KeyboardModesApplyBehavior
    fn set_keyboard_mode(&mut self, _mode: KeyboardModes, _behavior: KeyboardModesApplyBehavior) {}

    /// Set XTerm's [`ModifyOtherKeys`] option.
    fn set_modify_other_keys(&mut self, _mode: ModifyOtherKeys) {}

    /// Report XTerm's [`ModifyOtherKeys`] state.
    ///
    /// The output is of form `CSI > 4 ; mode m`.
    fn report_modify_other_keys(&mut self) {}

    // Set SCP control.
    fn set_scp(&mut self, _char_path: ScpCharPath, _update_mode: ScpUpdateMode) {}
}

bitflags! {
    /// A set of [`kitty keyboard protocol'] modes.
    ///
    /// [`kitty keyboard protocol']: https://sw.kovidgoyal.net/kitty/keyboard-protocol
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct KeyboardModes : u8 {
        /// No keyboard protocol mode is set.
        const NO_MODE                 = 0b0000_0000;
        /// Report `Esc`, `alt` + `key`, `ctrl` + `key`, `ctrl` + `alt` + `key`, `shift`
        /// + `alt` + `key` keys using `CSI u` sequence instead of raw ones.
        const DISAMBIGUATE_ESC_CODES  = 0b0000_0001;
        /// Report key presses, release, and repetition alongside the escape. Key events
        /// that result in text are reported as plain UTF-8, unless the
        /// [`Self::REPORT_ALL_KEYS_AS_ESC`] is enabled.
        const REPORT_EVENT_TYPES      = 0b0000_0010;
        /// Additionally report shifted key an dbase layout key.
        const REPORT_ALTERNATE_KEYS   = 0b0000_0100;
        /// Report every key as an escape sequence.
        const REPORT_ALL_KEYS_AS_ESC  = 0b0000_1000;
        /// Report the text generated by the key event.
        const REPORT_ASSOCIATED_TEXT  = 0b0001_0000;
    }
}

/// XTMODKEYS modifyOtherKeys state.
///
/// This only applies to keys corresponding to ascii characters.
///
/// For the details on how to implement the mode handling correctly, consult
/// [`XTerm's implementation`] and the [`output`] of XTerm's provided [`perl
/// script`]. Some libraries and implementations also use the [`fixterms`]
/// definition of the `CSI u`.
///
/// The end escape sequence has a `CSI char; modifiers u` form while the
/// original `CSI 27 ; modifier ; char ~`. The clients should prefer the `CSI
/// u`, since it has more adoption.
///
/// [`XTerm's implementation`]: https://invisible-island.net/xterm/modified-keys.html
/// [`perl script`]: https://github.com/ThomasDickey/xterm-snapshots/blob/master/vttests/modify-keys.pl
/// [`output`]: https://github.com/alacritty/vte/blob/master/doc/modifyOtherKeys-example.txt
/// [`fixterms`]: http://www.leonerd.org.uk/hacks/fixterms/
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifyOtherKeys {
    /// Reset the state.
    Reset,
    /// Enables this feature except for keys with well-known behavior, e.g.,
    /// Tab, Backspace and some special control character cases which are
    /// built into the X11 library (e.g., Control-Space to make a NUL, or
    /// Control-3 to make an Escape character).
    ///
    /// Escape sequences shouldn't be emitted under the following circumstances:
    /// - When the key is in range of `[64;127]` and the modifier is either
    ///   Control or Shift
    /// - When the key combination is a known control combination alias
    ///
    /// For more details, consult the [`example`] for the suggested translation.
    ///
    /// [`example`]: https://github.com/alacritty/vte/blob/master/doc/modifyOtherKeys-example.txt
    EnableExceptWellDefined,
    /// Enables this feature for all keys including the exceptions of
    /// [`Self::EnableExceptWellDefined`].  XTerm still ignores the special
    /// cases built into the X11 library. Any shifted (modified) ordinary
    /// key send an escape sequence. The Alt- and Meta- modifiers cause
    /// XTerm to send escape sequences.
    ///
    /// For more details, consult the [`example`] for the suggested translation.
    ///
    /// [`example`]: https://github.com/alacritty/vte/blob/master/doc/modifyOtherKeys-example.txt
    EnableAll,
}

/// Describes how the new [`KeyboardModes`] should be applied.
#[repr(u8)]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardModesApplyBehavior {
    /// Replace the active flags with the new ones.
    #[default]
    Replace,
    /// Merge the given flags with currently active ones.
    Union,
    /// Remove the given flags from the active ones.
    Difference,
}

/// Terminal cursor configuration.
#[derive(Default, Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct CursorStyle {
    pub shape: CursorShape,
    pub blinking: bool,
}

/// Terminal cursor shape.
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub enum CursorShape {
    /// Cursor is a block like `▒`.
    #[default]
    Block,

    /// Cursor is an underscore like `_`.
    Underline,

    /// Cursor is a vertical bar `⎸`.
    Beam,

    /// Cursor is a box like `☐`.
    HollowBlock,

    /// Invisible cursor.
    Hidden,
}

/// Wrapper for the ANSI modes.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    /// Known ANSI mode.
    Named(NamedMode),
    /// Unidentified publc mode.
    Unknown(u16),
}

impl Mode {
    fn new(mode: u16) -> Self {
        match mode {
            4 => Self::Named(NamedMode::Insert),
            20 => Self::Named(NamedMode::LineFeedNewLine),
            _ => Self::Unknown(mode),
        }
    }

    /// Get the raw value of the mode.
    pub fn raw(self) -> u16 {
        match self {
            Self::Named(named) => named as u16,
            Self::Unknown(mode) => mode,
        }
    }
}

impl From<NamedMode> for Mode {
    fn from(value: NamedMode) -> Self {
        Self::Named(value)
    }
}

/// ANSI modes.
#[repr(u16)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NamedMode {
    /// IRM Insert Mode.
    Insert = 4,
    LineFeedNewLine = 20,
}

/// Wrapper for the private DEC modes.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PrivateMode {
    /// Known private mode.
    Named(NamedPrivateMode),
    /// Unknown private mode.
    Unknown(u16),
}

impl PrivateMode {
    fn new(mode: u16) -> Self {
        match mode {
            1 => Self::Named(NamedPrivateMode::CursorKeys),
            3 => Self::Named(NamedPrivateMode::ColumnMode),
            6 => Self::Named(NamedPrivateMode::Origin),
            7 => Self::Named(NamedPrivateMode::LineWrap),
            12 => Self::Named(NamedPrivateMode::BlinkingCursor),
            25 => Self::Named(NamedPrivateMode::ShowCursor),
            1000 => Self::Named(NamedPrivateMode::ReportMouseClicks),
            1002 => Self::Named(NamedPrivateMode::ReportCellMouseMotion),
            1003 => Self::Named(NamedPrivateMode::ReportAllMouseMotion),
            1004 => Self::Named(NamedPrivateMode::ReportFocusInOut),
            1005 => Self::Named(NamedPrivateMode::Utf8Mouse),
            1006 => Self::Named(NamedPrivateMode::SgrMouse),
            1007 => Self::Named(NamedPrivateMode::AlternateScroll),
            1042 => Self::Named(NamedPrivateMode::UrgencyHints),
            1049 => Self::Named(NamedPrivateMode::SwapScreenAndSetRestoreCursor),
            2004 => Self::Named(NamedPrivateMode::BracketedPaste),
            2026 => Self::Named(NamedPrivateMode::SyncUpdate),
            _ => Self::Unknown(mode),
        }
    }

    /// Get the raw value of the mode.
    pub fn raw(self) -> u16 {
        match self {
            Self::Named(named) => named as u16,
            Self::Unknown(mode) => mode,
        }
    }
}

impl From<NamedPrivateMode> for PrivateMode {
    fn from(value: NamedPrivateMode) -> Self {
        Self::Named(value)
    }
}

/// Private DEC modes.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NamedPrivateMode {
    CursorKeys = 1,
    /// Select 80 or 132 columns per page (DECCOLM).
    ///
    /// CSI ? 3 h -> set 132 column font.
    /// CSI ? 3 l -> reset 80 column font.
    ///
    /// Additionally,
    ///
    /// * set margins to default positions
    /// * erases all data in page memory
    /// * resets DECLRMM to unavailable
    /// * clears data from the status line (if set to host-writable)
    ColumnMode = 3,
    Origin = 6,
    LineWrap = 7,
    BlinkingCursor = 12,
    ShowCursor = 25,
    ReportMouseClicks = 1000,
    ReportCellMouseMotion = 1002,
    ReportAllMouseMotion = 1003,
    ReportFocusInOut = 1004,
    Utf8Mouse = 1005,
    SgrMouse = 1006,
    AlternateScroll = 1007,
    UrgencyHints = 1042,
    SwapScreenAndSetRestoreCursor = 1049,
    BracketedPaste = 2004,
    /// The mode is handled automatically by [`Processor`].
    SyncUpdate = 2026,
}

/// Mode for clearing line.
///
/// Relative to cursor.
#[derive(Debug)]
pub enum LineClearMode {
    /// Clear right of cursor.
    Right,
    /// Clear left of cursor.
    Left,
    /// Clear entire line.
    All,
}

/// Mode for clearing terminal.
///
/// Relative to cursor.
#[derive(Debug)]
pub enum ClearMode {
    /// Clear below cursor.
    Below,
    /// Clear above cursor.
    Above,
    /// Clear entire terminal.
    All,
    /// Clear 'saved' lines (scrollback).
    Saved,
}

/// Mode for clearing tab stops.
#[derive(Debug)]
pub enum TabulationClearMode {
    /// Clear stop under cursor.
    Current,
    /// Clear all stops.
    All,
}

/// Standard colors.
///
/// The order here matters since the enum should be castable to a `usize` for
/// indexing a color list.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NamedColor {
    /// Black.
    Black = 0,
    /// Red.
    Red,
    /// Green.
    Green,
    /// Yellow.
    Yellow,
    /// Blue.
    Blue,
    /// Magenta.
    Magenta,
    /// Cyan.
    Cyan,
    /// White.
    White,
    /// Bright black.
    BrightBlack,
    /// Bright red.
    BrightRed,
    /// Bright green.
    BrightGreen,
    /// Bright yellow.
    BrightYellow,
    /// Bright blue.
    BrightBlue,
    /// Bright magenta.
    BrightMagenta,
    /// Bright cyan.
    BrightCyan,
    /// Bright white.
    BrightWhite,
    /// The foreground color.
    Foreground = 256,
    /// The background color.
    Background,
    /// Color for the cursor itself.
    Cursor,
    /// Dim black.
    DimBlack,
    /// Dim red.
    DimRed,
    /// Dim green.
    DimGreen,
    /// Dim yellow.
    DimYellow,
    /// Dim blue.
    DimBlue,
    /// Dim magenta.
    DimMagenta,
    /// Dim cyan.
    DimCyan,
    /// Dim white.
    DimWhite,
    /// The bright foreground color.
    BrightForeground,
    /// Dim foreground.
    DimForeground,
}

impl NamedColor {
    #[must_use]
    pub fn to_bright(self) -> Self {
        match self {
            NamedColor::Foreground => NamedColor::BrightForeground,
            NamedColor::Black => NamedColor::BrightBlack,
            NamedColor::Red => NamedColor::BrightRed,
            NamedColor::Green => NamedColor::BrightGreen,
            NamedColor::Yellow => NamedColor::BrightYellow,
            NamedColor::Blue => NamedColor::BrightBlue,
            NamedColor::Magenta => NamedColor::BrightMagenta,
            NamedColor::Cyan => NamedColor::BrightCyan,
            NamedColor::White => NamedColor::BrightWhite,
            NamedColor::DimForeground => NamedColor::Foreground,
            NamedColor::DimBlack => NamedColor::Black,
            NamedColor::DimRed => NamedColor::Red,
            NamedColor::DimGreen => NamedColor::Green,
            NamedColor::DimYellow => NamedColor::Yellow,
            NamedColor::DimBlue => NamedColor::Blue,
            NamedColor::DimMagenta => NamedColor::Magenta,
            NamedColor::DimCyan => NamedColor::Cyan,
            NamedColor::DimWhite => NamedColor::White,
            val => val,
        }
    }

    #[must_use]
    pub fn to_dim(self) -> Self {
        match self {
            NamedColor::Black => NamedColor::DimBlack,
            NamedColor::Red => NamedColor::DimRed,
            NamedColor::Green => NamedColor::DimGreen,
            NamedColor::Yellow => NamedColor::DimYellow,
            NamedColor::Blue => NamedColor::DimBlue,
            NamedColor::Magenta => NamedColor::DimMagenta,
            NamedColor::Cyan => NamedColor::DimCyan,
            NamedColor::White => NamedColor::DimWhite,
            NamedColor::Foreground => NamedColor::DimForeground,
            NamedColor::BrightBlack => NamedColor::Black,
            NamedColor::BrightRed => NamedColor::Red,
            NamedColor::BrightGreen => NamedColor::Green,
            NamedColor::BrightYellow => NamedColor::Yellow,
            NamedColor::BrightBlue => NamedColor::Blue,
            NamedColor::BrightMagenta => NamedColor::Magenta,
            NamedColor::BrightCyan => NamedColor::Cyan,
            NamedColor::BrightWhite => NamedColor::White,
            NamedColor::BrightForeground => NamedColor::Foreground,
            val => val,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Color {
    Named(NamedColor),
    Spec(Rgb),
    Indexed(u8),
}

/// Terminal character attributes.
#[derive(Debug, Eq, PartialEq)]
pub enum Attr {
    /// Clear all special abilities.
    Reset,
    /// Bold text.
    Bold,
    /// Dim or secondary color.
    Dim,
    /// Italic text.
    Italic,
    /// Underline text.
    Underline,
    /// Underlined twice.
    DoubleUnderline,
    /// Undercurled text.
    Undercurl,
    /// Dotted underlined text.
    DottedUnderline,
    /// Dashed underlined text.
    DashedUnderline,
    /// Blink cursor slowly.
    BlinkSlow,
    /// Blink cursor fast.
    BlinkFast,
    /// Invert colors.
    Reverse,
    /// Do not display characters.
    Hidden,
    /// Strikeout text.
    Strike,
    /// Cancel bold.
    CancelBold,
    /// Cancel bold and dim.
    CancelBoldDim,
    /// Cancel italic.
    CancelItalic,
    /// Cancel all underlines.
    CancelUnderline,
    /// Cancel blink.
    CancelBlink,
    /// Cancel inversion.
    CancelReverse,
    /// Cancel text hiding.
    CancelHidden,
    /// Cancel strikeout.
    CancelStrike,
    /// Set indexed foreground color.
    Foreground(Color),
    /// Set indexed background color.
    Background(Color),
    /// Underline color.
    UnderlineColor(Option<Color>),
}

/// Identifiers which can be assigned to a graphic character set.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CharsetIndex {
    /// Default set, is designated as ASCII at startup.
    #[default]
    G0,
    G1,
    G2,
    G3,
}

/// Standard or common character sets which can be designated as G0-G3.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StandardCharset {
    #[default]
    Ascii,
    SpecialCharacterAndLineDrawing,
}

impl StandardCharset {
    /// Switch/Map character to the active charset. Ascii is the common case and
    /// for that we want to do as little as possible.
    #[inline]
    pub fn map(self, c: char) -> char {
        match self {
            StandardCharset::Ascii => c,
            StandardCharset::SpecialCharacterAndLineDrawing => match c {
                '_' => ' ',
                '`' => '◆',
                'a' => '▒',
                'b' => '\u{2409}', // Symbol for horizontal tabulation
                'c' => '\u{240c}', // Symbol for form feed
                'd' => '\u{240d}', // Symbol for carriage return
                'e' => '\u{240a}', // Symbol for line feed
                'f' => '°',
                'g' => '±',
                'h' => '\u{2424}', // Symbol for newline
                'i' => '\u{240b}', // Symbol for vertical tabulation
                'j' => '┘',
                'k' => '┐',
                'l' => '┌',
                'm' => '└',
                'n' => '┼',
                'o' => '⎺',
                'p' => '⎻',
                'q' => '─',
                'r' => '⎼',
                's' => '⎽',
                't' => '├',
                'u' => '┤',
                'v' => '┴',
                'w' => '┬',
                'x' => '│',
                'y' => '≤',
                'z' => '≥',
                '{' => 'π',
                '|' => '≠',
                '}' => '£',
                '~' => '·',
                _ => c,
            },
        }
    }
}

/// SCP control's first parameter which determines character path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScpCharPath {
    /// SCP's first parameter value of 0. Behavior is implementation defined.
    Default,
    /// SCP's first parameter value of 1 which sets character path to
    /// LEFT-TO-RIGHT.
    LTR,
    /// SCP's first parameter value of 2 which sets character path to
    /// RIGHT-TO-LEFT.
    RTL,
}

/// SCP control's second parameter which determines update mode/direction
/// between components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScpUpdateMode {
    /// SCP's second parameter value of 0 (the default). Implementation
    /// dependant update.
    ImplementationDependant,
    /// SCP's second parameter value of 1.
    ///
    /// Reflect data component changes in the presentation component.
    DataToPresentation,
    /// SCP's second parameter value of 2.
    ///
    /// Reflect presentation component changes in the data component.
    PresentationToData,
}

impl<'a, H, T> crate::Perform for Performer<'a, H, T>
where
    H: Handler + 'a,
    T: Timeout,
{
    #[inline]
    fn print(&mut self, c: char) {
        self.handler.input(c);
        self.state.preceding_char = Some(c);
    }

    #[inline]
    fn execute(&mut self, byte: u8) {
        match byte {
            C0::HT => self.handler.put_tab(1),
            C0::BS => self.handler.backspace(),
            C0::CR => self.handler.carriage_return(),
            C0::LF | C0::VT | C0::FF => self.handler.linefeed(),
            C0::BEL => self.handler.bell(),
            C0::SUB => self.handler.substitute(),
            C0::SI => self.handler.set_active_charset(CharsetIndex::G0),
            C0::SO => self.handler.set_active_charset(CharsetIndex::G1),
            _ => debug!("[unhandled] execute byte={:02x}", byte),
        }
    }

    #[inline]
    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, action: char) {
        debug!(
            "[unhandled hook] params={:?}, ints: {:?}, ignore: {:?}, action: {:?}",
            params, intermediates, ignore, action
        );
    }

    #[inline]
    fn put(&mut self, byte: u8) {
        debug!("[unhandled put] byte={:?}", byte);
    }

    #[inline]
    fn unhook(&mut self) {
        debug!("[unhandled unhook]");
    }

    #[inline]
    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        let terminator = if bell_terminated { "\x07" } else { "\x1b\\" };

        fn unhandled(params: &[&[u8]]) {
            let mut buf = String::new();
            for items in params {
                buf.push('[');
                for item in *items {
                    let _ = write!(buf, "{:?}", *item as char);
                }
                buf.push_str("],");
            }
            debug!("[unhandled osc_dispatch]: [{}] at line {}", &buf, line!());
        }

        if params.is_empty() || params[0].is_empty() {
            return;
        }

        match params[0] {
            // Set window title.
            b"0" | b"2" => {
                if params.len() >= 2 {
                    let title = params[1..]
                        .iter()
                        .flat_map(|x| str::from_utf8(x))
                        .collect::<Vec<&str>>()
                        .join(";")
                        .trim()
                        .to_owned();
                    self.handler.set_title(Some(title));
                    return;
                }
                unhandled(params);
            },

            // Set color index.
            b"4" => {
                if params.len() <= 1 || params.len() % 2 == 0 {
                    unhandled(params);
                    return;
                }

                for chunk in params[1..].chunks(2) {
                    let index = match parse_number(chunk[0]) {
                        Some(index) => index,
                        None => {
                            unhandled(params);
                            continue;
                        },
                    };

                    if let Some(c) = xparse_color(chunk[1]) {
                        self.handler.set_color(index as usize, c);
                    } else if chunk[1] == b"?" {
                        let prefix = alloc::format!("4;{index}");
                        self.handler.dynamic_color_sequence(prefix, index as usize, terminator);
                    } else {
                        unhandled(params);
                    }
                }
            },

            // Hyperlink.
            b"8" if params.len() > 2 => {
                let link_params = params[1];

                // NOTE: The escape sequence is of form 'OSC 8 ; params ; URI ST', where
                // URI is URL-encoded. However `;` is a special character and might be
                // passed as is, thus we need to rebuild the URI.
                let mut uri = str::from_utf8(params[2]).unwrap_or_default().to_string();
                for param in params[3..].iter() {
                    uri.push(';');
                    uri.push_str(str::from_utf8(param).unwrap_or_default());
                }

                // The OSC 8 escape sequence must be stopped when getting an empty `uri`.
                if uri.is_empty() {
                    self.handler.set_hyperlink(None);
                    return;
                }

                // Link parameters are in format of `key1=value1:key2=value2`. Currently only
                // key `id` is defined.
                let id = link_params
                    .split(|&b| b == b':')
                    .find_map(|kv| kv.strip_prefix(b"id="))
                    .and_then(|kv| str::from_utf8(kv).ok().map(|e| e.to_owned()));

                self.handler.set_hyperlink(Some(Hyperlink { id, uri }));
            },

            // Get/set Foreground, Background, Cursor colors.
            b"10" | b"11" | b"12" => {
                if params.len() >= 2 {
                    if let Some(mut dynamic_code) = parse_number(params[0]) {
                        for param in &params[1..] {
                            // 10 is the first dynamic color, also the foreground.
                            let offset = dynamic_code as usize - 10;
                            let index = NamedColor::Foreground as usize + offset;

                            // End of setting dynamic colors.
                            if index > NamedColor::Cursor as usize {
                                unhandled(params);
                                break;
                            }

                            if let Some(color) = xparse_color(param) {
                                self.handler.set_color(index, color);
                            } else if param == b"?" {
                                self.handler.dynamic_color_sequence(
                                    dynamic_code.to_string(),
                                    index,
                                    terminator,
                                );
                            } else {
                                unhandled(params);
                            }
                            dynamic_code += 1;
                        }
                        return;
                    }
                }
                unhandled(params);
            },

            // Set mouse cursor shape.
            b"22" if params.len() == 2 => {
                let shape = String::from_utf8_lossy(params[1]);
                match CursorIcon::from_str(&shape) {
                    Ok(cursor_icon) => self.handler.set_mouse_cursor_icon(cursor_icon),
                    Err(_) => debug!("[osc 22] unrecognized cursor icon shape: {shape:?}"),
                }
            },

            // Set cursor style.
            b"50" => {
                if params.len() >= 2
                    && params[1].len() >= 13
                    && params[1][0..12] == *b"CursorShape="
                {
                    let shape = match params[1][12] as char {
                        '0' => CursorShape::Block,
                        '1' => CursorShape::Beam,
                        '2' => CursorShape::Underline,
                        _ => return unhandled(params),
                    };
                    self.handler.set_cursor_shape(shape);
                    return;
                }
                unhandled(params);
            },

            // Set clipboard.
            b"52" => {
                if params.len() < 3 {
                    return unhandled(params);
                }

                let clipboard = params[1].first().unwrap_or(&b'c');
                match params[2] {
                    b"?" => self.handler.clipboard_load(*clipboard, terminator),
                    base64 => self.handler.clipboard_store(*clipboard, base64),
                }
            },

            // Reset color index.
            b"104" => {
                // Reset all color indexes when no parameters are given.
                if params.len() == 1 || params[1].is_empty() {
                    for i in 0..256 {
                        self.handler.reset_color(i);
                    }
                    return;
                }

                // Reset color indexes given as parameters.
                for param in &params[1..] {
                    match parse_number(param) {
                        Some(index) => self.handler.reset_color(index as usize),
                        None => unhandled(params),
                    }
                }
            },

            // Reset foreground color.
            b"110" => self.handler.reset_color(NamedColor::Foreground as usize),

            // Reset background color.
            b"111" => self.handler.reset_color(NamedColor::Background as usize),

            // Reset text cursor color.
            b"112" => self.handler.reset_color(NamedColor::Cursor as usize),

            _ => unhandled(params),
        }
    }

    #[allow(clippy::cognitive_complexity)]
    #[inline]
    fn csi_dispatch(
        &mut self,
        params: &Params,
        intermediates: &[u8],
        has_ignored_intermediates: bool,
        action: char,
    ) {
        macro_rules! unhandled {
            () => {{
                debug!(
                    "[Unhandled CSI] action={:?}, params={:?}, intermediates={:?}",
                    action, params, intermediates
                );
            }};
        }

        if has_ignored_intermediates || intermediates.len() > 2 {
            unhandled!();
            return;
        }

        let mut params_iter = params.iter();
        let handler = &mut self.handler;

        let mut next_param_or = |default: u16| match params_iter.next() {
            Some(&[param, ..]) if param != 0 => param,
            _ => default,
        };

        match (action, intermediates) {
            ('@', []) => handler.insert_blank(next_param_or(1) as usize),
            ('A', []) => handler.move_up(next_param_or(1) as usize),
            ('B', []) | ('e', []) => handler.move_down(next_param_or(1) as usize),
            ('b', []) => {
                if let Some(c) = self.state.preceding_char {
                    for _ in 0..next_param_or(1) {
                        handler.input(c);
                    }
                } else {
                    debug!("tried to repeat with no preceding char");
                }
            },
            ('C', []) | ('a', []) => handler.move_forward(next_param_or(1) as usize),
            ('c', intermediates) if next_param_or(0) == 0 => {
                handler.identify_terminal(intermediates.first().map(|&i| i as char))
            },
            ('D', []) => handler.move_backward(next_param_or(1) as usize),
            ('d', []) => handler.goto_line(next_param_or(1) as i32 - 1),
            ('E', []) => handler.move_down_and_cr(next_param_or(1) as usize),
            ('F', []) => handler.move_up_and_cr(next_param_or(1) as usize),
            ('G', []) | ('`', []) => handler.goto_col(next_param_or(1) as usize - 1),
            ('W', [b'?']) if next_param_or(0) == 5 => handler.set_tabs(8),
            ('g', []) => {
                let mode = match next_param_or(0) {
                    0 => TabulationClearMode::Current,
                    3 => TabulationClearMode::All,
                    _ => {
                        unhandled!();
                        return;
                    },
                };

                handler.clear_tabs(mode);
            },
            ('H', []) | ('f', []) => {
                let y = next_param_or(1) as i32;
                let x = next_param_or(1) as usize;
                handler.goto(y - 1, x - 1);
            },
            ('h', []) => {
                for param in params_iter.map(|param| param[0]) {
                    handler.set_mode(Mode::new(param))
                }
            },
            ('h', [b'?']) => {
                for param in params_iter.map(|param| param[0]) {
                    // Handle sync updates opaquely.
                    if param == NamedPrivateMode::SyncUpdate as u16 {
                        self.state.sync_state.timeout.set_timeout(SYNC_UPDATE_TIMEOUT);
                        self.terminated = true;
                    }

                    handler.set_private_mode(PrivateMode::new(param))
                }
            },
            ('I', []) => handler.move_forward_tabs(next_param_or(1)),
            ('J', []) => {
                let mode = match next_param_or(0) {
                    0 => ClearMode::Below,
                    1 => ClearMode::Above,
                    2 => ClearMode::All,
                    3 => ClearMode::Saved,
                    _ => {
                        unhandled!();
                        return;
                    },
                };

                handler.clear_screen(mode);
            },
            ('K', []) => {
                let mode = match next_param_or(0) {
                    0 => LineClearMode::Right,
                    1 => LineClearMode::Left,
                    2 => LineClearMode::All,
                    _ => {
                        unhandled!();
                        return;
                    },
                };

                handler.clear_line(mode);
            },
            ('k', [b' ']) => {
                // SCP control.
                let char_path = match next_param_or(0) {
                    0 => ScpCharPath::Default,
                    1 => ScpCharPath::LTR,
                    2 => ScpCharPath::RTL,
                    _ => {
                        unhandled!();
                        return;
                    },
                };

                let update_mode = match next_param_or(0) {
                    0 => ScpUpdateMode::ImplementationDependant,
                    1 => ScpUpdateMode::DataToPresentation,
                    2 => ScpUpdateMode::PresentationToData,
                    _ => {
                        unhandled!();
                        return;
                    },
                };

                handler.set_scp(char_path, update_mode);
            },
            ('L', []) => handler.insert_blank_lines(next_param_or(1) as usize),
            ('l', []) => {
                for param in params_iter.map(|param| param[0]) {
                    handler.unset_mode(Mode::new(param))
                }
            },
            ('l', [b'?']) => {
                for param in params_iter.map(|param| param[0]) {
                    handler.unset_private_mode(PrivateMode::new(param))
                }
            },
            ('M', []) => handler.delete_lines(next_param_or(1) as usize),
            ('m', []) => {
                if params.is_empty() {
                    handler.terminal_attribute(Attr::Reset);
                } else {
                    attrs_from_sgr_parameters(*handler, &mut params_iter);
                }
            },
            ('m', [b'>']) => {
                let mode = match (next_param_or(1) == 4).then(|| next_param_or(0)) {
                    Some(0) => ModifyOtherKeys::Reset,
                    Some(1) => ModifyOtherKeys::EnableExceptWellDefined,
                    Some(2) => ModifyOtherKeys::EnableAll,
                    _ => return unhandled!(),
                };
                handler.set_modify_other_keys(mode);
            },
            ('m', [b'?']) => {
                if params_iter.next() == Some(&[4]) {
                    handler.report_modify_other_keys();
                } else {
                    unhandled!()
                }
            },
            ('n', []) => handler.device_status(next_param_or(0) as usize),
            ('P', []) => handler.delete_chars(next_param_or(1) as usize),
            ('p', [b'$']) => {
                let mode = next_param_or(0);
                handler.report_mode(Mode::new(mode));
            },
            ('p', [b'?', b'$']) => {
                let mode = next_param_or(0);
                handler.report_private_mode(PrivateMode::new(mode));
            },
            ('q', [b' ']) => {
                // DECSCUSR (CSI Ps SP q) -- Set Cursor Style.
                let cursor_style_id = next_param_or(0);
                let shape = match cursor_style_id {
                    0 => None,
                    1 | 2 => Some(CursorShape::Block),
                    3 | 4 => Some(CursorShape::Underline),
                    5 | 6 => Some(CursorShape::Beam),
                    _ => {
                        unhandled!();
                        return;
                    },
                };
                let cursor_style =
                    shape.map(|shape| CursorStyle { shape, blinking: cursor_style_id % 2 == 1 });

                handler.set_cursor_style(cursor_style);
            },
            ('r', []) => {
                let top = next_param_or(1) as usize;
                let bottom =
                    params_iter.next().map(|param| param[0] as usize).filter(|&param| param != 0);

                handler.set_scrolling_region(top, bottom);
            },
            ('S', []) => handler.scroll_up(next_param_or(1) as usize),
            ('s', []) => handler.save_cursor_position(),
            ('T', []) => handler.scroll_down(next_param_or(1) as usize),
            ('t', []) => match next_param_or(1) as usize {
                14 => handler.text_area_size_pixels(),
                18 => handler.text_area_size_chars(),
                22 => handler.push_title(),
                23 => handler.pop_title(),
                _ => unhandled!(),
            },
            ('u', [b'?']) => handler.report_keyboard_mode(),
            ('u', [b'=']) => {
                let mode = KeyboardModes::from_bits_truncate(next_param_or(0) as u8);
                let behavior = match next_param_or(1) {
                    3 => KeyboardModesApplyBehavior::Difference,
                    2 => KeyboardModesApplyBehavior::Union,
                    // Default is replace.
                    _ => KeyboardModesApplyBehavior::Replace,
                };
                handler.set_keyboard_mode(mode, behavior);
            },
            ('u', [b'>']) => {
                let mode = KeyboardModes::from_bits_truncate(next_param_or(0) as u8);
                handler.push_keyboard_mode(mode);
            },
            ('u', [b'<']) => {
                // The default is 1.
                handler.pop_keyboard_modes(next_param_or(1));
            },
            ('u', []) => handler.restore_cursor_position(),
            ('X', []) => handler.erase_chars(next_param_or(1) as usize),
            ('Z', []) => handler.move_backward_tabs(next_param_or(1)),
            _ => unhandled!(),
        }
    }

    #[inline]
    fn esc_dispatch(&mut self, intermediates: &[u8], _ignore: bool, byte: u8) {
        macro_rules! unhandled {
            () => {{
                debug!(
                    "[unhandled] esc_dispatch ints={:?}, byte={:?} ({:02x})",
                    intermediates, byte as char, byte
                );
            }};
        }

        macro_rules! configure_charset {
            ($charset:path, $intermediates:expr) => {{
                let index: CharsetIndex = match $intermediates {
                    [b'('] => CharsetIndex::G0,
                    [b')'] => CharsetIndex::G1,
                    [b'*'] => CharsetIndex::G2,
                    [b'+'] => CharsetIndex::G3,
                    _ => {
                        unhandled!();
                        return;
                    },
                };
                self.handler.configure_charset(index, $charset)
            }};
        }

        match (byte, intermediates) {
            (b'B', intermediates) => configure_charset!(StandardCharset::Ascii, intermediates),
            (b'D', []) => self.handler.linefeed(),
            (b'E', []) => {
                self.handler.linefeed();
                self.handler.carriage_return();
            },
            (b'H', []) => self.handler.set_horizontal_tabstop(),
            (b'M', []) => self.handler.reverse_index(),
            (b'Z', []) => self.handler.identify_terminal(None),
            (b'c', []) => self.handler.reset_state(),
            (b'0', intermediates) => {
                configure_charset!(StandardCharset::SpecialCharacterAndLineDrawing, intermediates)
            },
            (b'7', []) => self.handler.save_cursor_position(),
            (b'8', [b'#']) => self.handler.decaln(),
            (b'8', []) => self.handler.restore_cursor_position(),
            (b'=', []) => self.handler.set_keypad_application_mode(),
            (b'>', []) => self.handler.unset_keypad_application_mode(),
            // String terminator, do nothing (parser handles as string terminator).
            (b'\\', []) => (),
            _ => unhandled!(),
        }
    }

    #[inline]
    fn terminated(&self) -> bool {
        self.terminated
    }
}

#[inline]
fn attrs_from_sgr_parameters<H: Handler>(handler: &mut H, params: &mut ParamsIter<'_>) {
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
            },
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
            },
            [48, params @ ..] => handle_colon_rgb(params).map(Attr::Background),
            [49] => Some(Attr::Background(Color::Named(NamedColor::Background))),
            [58] => {
                let mut iter = params.map(|param| param[0]);
                parse_sgr_color(&mut iter).map(|color| Attr::UnderlineColor(Some(color)))
            },
            [58, params @ ..] => {
                handle_colon_rgb(params).map(|color| Attr::UnderlineColor(Some(color)))
            },
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

        match attr {
            Some(attr) => handler.terminal_attribute(attr),
            None => continue,
        }
    }
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

/// C0 set of 7-bit control characters (from ANSI X3.4-1977).
#[allow(non_snake_case)]
pub mod C0 {
    /// Null filler, terminal should ignore this character.
    pub const NUL: u8 = 0x00;
    /// Start of Header.
    pub const SOH: u8 = 0x01;
    /// Start of Text, implied end of header.
    pub const STX: u8 = 0x02;
    /// End of Text, causes some terminal to respond with ACK or NAK.
    pub const ETX: u8 = 0x03;
    /// End of Transmission.
    pub const EOT: u8 = 0x04;
    /// Enquiry, causes terminal to send ANSWER-BACK ID.
    pub const ENQ: u8 = 0x05;
    /// Acknowledge, usually sent by terminal in response to ETX.
    pub const ACK: u8 = 0x06;
    /// Bell, triggers the bell, buzzer, or beeper on the terminal.
    pub const BEL: u8 = 0x07;
    /// Backspace, can be used to define overstruck characters.
    pub const BS: u8 = 0x08;
    /// Horizontal Tabulation, move to next predetermined position.
    pub const HT: u8 = 0x09;
    /// Linefeed, move to same position on next line (see also NL).
    pub const LF: u8 = 0x0A;
    /// Vertical Tabulation, move to next predetermined line.
    pub const VT: u8 = 0x0B;
    /// Form Feed, move to next form or page.
    pub const FF: u8 = 0x0C;
    /// Carriage Return, move to first character of current line.
    pub const CR: u8 = 0x0D;
    /// Shift Out, switch to G1 (other half of character set).
    pub const SO: u8 = 0x0E;
    /// Shift In, switch to G0 (normal half of character set).
    pub const SI: u8 = 0x0F;
    /// Data Link Escape, interpret next control character specially.
    pub const DLE: u8 = 0x10;
    /// (DC1) Terminal is allowed to resume transmitting.
    pub const XON: u8 = 0x11;
    /// Device Control 2, causes ASR-33 to activate paper-tape reader.
    pub const DC2: u8 = 0x12;
    /// (DC2) Terminal must pause and refrain from transmitting.
    pub const XOFF: u8 = 0x13;
    /// Device Control 4, causes ASR-33 to deactivate paper-tape reader.
    pub const DC4: u8 = 0x14;
    /// Negative Acknowledge, used sometimes with ETX and ACK.
    pub const NAK: u8 = 0x15;
    /// Synchronous Idle, used to maintain timing in Sync communication.
    pub const SYN: u8 = 0x16;
    /// End of Transmission block.
    pub const ETB: u8 = 0x17;
    /// Cancel (makes VT100 abort current escape sequence if any).
    pub const CAN: u8 = 0x18;
    /// End of Medium.
    pub const EM: u8 = 0x19;
    /// Substitute (VT100 uses this to display parity errors).
    pub const SUB: u8 = 0x1A;
    /// Prefix to an escape sequence.
    pub const ESC: u8 = 0x1B;
    /// File Separator.
    pub const FS: u8 = 0x1C;
    /// Group Separator.
    pub const GS: u8 = 0x1D;
    /// Record Separator (sent by VT132 in block-transfer mode).
    pub const RS: u8 = 0x1E;
    /// Unit Separator.
    pub const US: u8 = 0x1F;
    /// Delete, should be ignored by terminal.
    pub const DEL: u8 = 0x7F;
}

// Tests for parsing escape sequences.
//
// Byte sequences used in these tests are recording of pty stdout.
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    pub struct TestSyncHandler {
        is_sync: usize,
    }

    impl Timeout for TestSyncHandler {
        #[inline]
        fn set_timeout(&mut self, _: Duration) {
            self.is_sync += 1;
        }

        #[inline]
        fn clear_timeout(&mut self) {
            self.is_sync = 0;
        }

        #[inline]
        fn pending_timeout(&self) -> bool {
            self.is_sync != 0
        }
    }

    struct MockHandler {
        index: CharsetIndex,
        charset: StandardCharset,
        attr: Option<Attr>,
        identity_reported: bool,
        color: Option<Rgb>,
        reset_colors: Vec<usize>,
    }

    impl Handler for MockHandler {
        fn terminal_attribute(&mut self, attr: Attr) {
            self.attr = Some(attr);
        }

        fn configure_charset(&mut self, index: CharsetIndex, charset: StandardCharset) {
            self.index = index;
            self.charset = charset;
        }

        fn set_active_charset(&mut self, index: CharsetIndex) {
            self.index = index;
        }

        fn identify_terminal(&mut self, _intermediate: Option<char>) {
            self.identity_reported = true;
        }

        fn reset_state(&mut self) {
            *self = Self::default();
        }

        fn set_color(&mut self, _: usize, c: Rgb) {
            self.color = Some(c);
        }

        fn reset_color(&mut self, index: usize) {
            self.reset_colors.push(index)
        }
    }

    impl Default for MockHandler {
        fn default() -> MockHandler {
            MockHandler {
                index: CharsetIndex::G0,
                charset: StandardCharset::Ascii,
                attr: None,
                identity_reported: false,
                color: None,
                reset_colors: Vec::new(),
            }
        }
    }

    #[test]
    fn parse_control_attribute() {
        static BYTES: &[u8] = &[0x1B, b'[', b'1', b'm'];

        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, BYTES);

        assert_eq!(handler.attr, Some(Attr::Bold));
    }

    #[test]
    fn parse_terminal_identity_csi() {
        let bytes: &[u8] = &[0x1B, b'[', b'1', b'c'];

        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, bytes);

        assert!(!handler.identity_reported);
        handler.reset_state();

        let bytes: &[u8] = &[0x1B, b'[', b'c'];

        parser.advance(&mut handler, bytes);

        assert!(handler.identity_reported);
        handler.reset_state();

        let bytes: &[u8] = &[0x1B, b'[', b'0', b'c'];

        parser.advance(&mut handler, bytes);

        assert!(handler.identity_reported);
    }

    #[test]
    fn parse_terminal_identity_esc() {
        let bytes: &[u8] = &[0x1B, b'Z'];

        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, bytes);

        assert!(handler.identity_reported);
        handler.reset_state();

        let bytes: &[u8] = &[0x1B, b'#', b'Z'];

        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, bytes);

        assert!(!handler.identity_reported);
        handler.reset_state();
    }

    #[test]
    fn parse_truecolor_attr() {
        static BYTES: &[u8] = &[
            0x1B, b'[', b'3', b'8', b';', b'2', b';', b'1', b'2', b'8', b';', b'6', b'6', b';',
            b'2', b'5', b'5', b'm',
        ];

        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, BYTES);

        let spec = Rgb { r: 128, g: 66, b: 255 };

        assert_eq!(handler.attr, Some(Attr::Foreground(Color::Spec(spec))));
    }

    /// No exactly a test; useful for debugging.
    #[test]
    fn parse_zsh_startup() {
        static BYTES: &[u8] = &[
            0x1B, b'[', b'1', b'm', 0x1B, b'[', b'7', b'm', b'%', 0x1B, b'[', b'2', b'7', b'm',
            0x1B, b'[', b'1', b'm', 0x1B, b'[', b'0', b'm', b' ', b' ', b' ', b' ', b' ', b' ',
            b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
            b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
            b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
            b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
            b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
            b' ', b' ', b' ', b'\r', b' ', b'\r', b'\r', 0x1B, b'[', b'0', b'm', 0x1B, b'[', b'2',
            b'7', b'm', 0x1B, b'[', b'2', b'4', b'm', 0x1B, b'[', b'J', b'j', b'w', b'i', b'l',
            b'm', b'@', b'j', b'w', b'i', b'l', b'm', b'-', b'd', b'e', b's', b'k', b' ', 0x1B,
            b'[', b'0', b'1', b';', b'3', b'2', b'm', 0xE2, 0x9E, 0x9C, b' ', 0x1B, b'[', b'0',
            b'1', b';', b'3', b'2', b'm', b' ', 0x1B, b'[', b'3', b'6', b'm', b'~', b'/', b'c',
            b'o', b'd', b'e',
        ];

        let mut handler = MockHandler::default();
        let mut parser = Processor::<TestSyncHandler>::new();

        parser.advance(&mut handler, BYTES);
    }

    #[test]
    fn parse_designate_g0_as_line_drawing() {
        static BYTES: &[u8] = &[0x1B, b'(', b'0'];
        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, BYTES);

        assert_eq!(handler.index, CharsetIndex::G0);
        assert_eq!(handler.charset, StandardCharset::SpecialCharacterAndLineDrawing);
    }

    #[test]
    fn parse_designate_g1_as_line_drawing_and_invoke() {
        static BYTES: &[u8] = &[0x1B, b')', b'0', 0x0E];
        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, &BYTES[..3]);

        assert_eq!(handler.index, CharsetIndex::G1);
        assert_eq!(handler.charset, StandardCharset::SpecialCharacterAndLineDrawing);

        let mut handler = MockHandler::default();
        parser.advance(&mut handler, &[BYTES[3]]);

        assert_eq!(handler.index, CharsetIndex::G1);
    }

    #[test]
    fn parse_valid_rgb_colors() {
        assert_eq!(xparse_color(b"rgb:f/e/d"), Some(Rgb { r: 0xFF, g: 0xEE, b: 0xDD }));
        assert_eq!(xparse_color(b"rgb:11/aa/ff"), Some(Rgb { r: 0x11, g: 0xAA, b: 0xFF }));
        assert_eq!(xparse_color(b"rgb:f/ed1/cb23"), Some(Rgb { r: 0xFF, g: 0xEC, b: 0xCA }));
        assert_eq!(xparse_color(b"rgb:ffff/0/0"), Some(Rgb { r: 0xFF, g: 0x0, b: 0x0 }));
    }

    #[test]
    fn parse_valid_legacy_rgb_colors() {
        assert_eq!(xparse_color(b"#1af"), Some(Rgb { r: 0x10, g: 0xA0, b: 0xF0 }));
        assert_eq!(xparse_color(b"#11aaff"), Some(Rgb { r: 0x11, g: 0xAA, b: 0xFF }));
        assert_eq!(xparse_color(b"#110aa0ff0"), Some(Rgb { r: 0x11, g: 0xAA, b: 0xFF }));
        assert_eq!(xparse_color(b"#1100aa00ff00"), Some(Rgb { r: 0x11, g: 0xAA, b: 0xFF }));
    }

    #[test]
    fn parse_invalid_rgb_colors() {
        assert_eq!(xparse_color(b"rgb:0//"), None);
        assert_eq!(xparse_color(b"rgb://///"), None);
    }

    #[test]
    fn parse_invalid_legacy_rgb_colors() {
        assert_eq!(xparse_color(b"#"), None);
        assert_eq!(xparse_color(b"#f"), None);
    }

    #[test]
    fn parse_invalid_number() {
        assert_eq!(parse_number(b"1abc"), None);
    }

    #[test]
    fn parse_valid_number() {
        assert_eq!(parse_number(b"123"), Some(123));
    }

    #[test]
    fn parse_number_too_large() {
        assert_eq!(parse_number(b"321"), None);
    }

    #[test]
    fn parse_osc4_set_color() {
        let bytes: &[u8] = b"\x1b]4;0;#fff\x1b\\";

        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, bytes);

        assert_eq!(handler.color, Some(Rgb { r: 0xF0, g: 0xF0, b: 0xF0 }));
    }

    #[test]
    fn parse_osc104_reset_color() {
        let bytes: &[u8] = b"\x1b]104;1;\x1b\\";

        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, bytes);

        assert_eq!(handler.reset_colors, vec![1]);
    }

    #[test]
    fn parse_osc104_reset_all_colors() {
        let bytes: &[u8] = b"\x1b]104;\x1b\\";

        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, bytes);

        let expected: Vec<usize> = (0..256).collect();
        assert_eq!(handler.reset_colors, expected);
    }

    #[test]
    fn parse_osc104_reset_all_colors_no_semicolon() {
        let bytes: &[u8] = b"\x1b]104\x1b\\";

        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        parser.advance(&mut handler, bytes);

        let expected: Vec<usize> = (0..256).collect();
        assert_eq!(handler.reset_colors, expected);
    }

    #[test]
    fn partial_sync_updates() {
        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        assert_eq!(parser.state.sync_state.timeout.is_sync, 0);
        assert!(handler.attr.is_none());

        // Start synchronized update.

        parser.advance(&mut handler, b"\x1b[?20");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 0);
        assert!(handler.attr.is_none());

        parser.advance(&mut handler, b"26h");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 1);
        assert!(handler.attr.is_none());

        // Dispatch some data.

        parser.advance(&mut handler, b"random \x1b[31m stuff");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 1);
        assert!(handler.attr.is_none());

        // Extend synchronized update.

        parser.advance(&mut handler, b"\x1b[?20");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 1);
        assert!(handler.attr.is_none());

        parser.advance(&mut handler, b"26h");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 2);
        assert!(handler.attr.is_none());

        // Terminate synchronized update.

        parser.advance(&mut handler, b"\x1b[?20");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 2);
        assert!(handler.attr.is_none());

        parser.advance(&mut handler, b"26l");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 0);
        assert!(handler.attr.is_some());
    }

    #[test]
    fn sync_bursts_buffer() {
        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        assert_eq!(parser.state.sync_state.timeout.is_sync, 0);
        assert!(handler.attr.is_none());

        // Repeat test twice to ensure internal state is reset properly.
        for _ in 0..2 {
            // Start synchronized update.
            parser.advance(&mut handler, b"\x1b[?2026h");
            assert_eq!(parser.state.sync_state.timeout.is_sync, 1);
            assert!(handler.attr.is_none());

            // Ensure sync works.
            parser.advance(&mut handler, b"\x1b[31m");
            assert_eq!(parser.state.sync_state.timeout.is_sync, 1);
            assert!(handler.attr.is_none());

            // Exceed sync buffer dimensions.
            parser.advance(&mut handler, "a".repeat(SYNC_BUFFER_SIZE).as_bytes());
            assert_eq!(parser.state.sync_state.timeout.is_sync, 0);
            assert!(handler.attr.take().is_some());

            // Ensure new events are dispatched directly.
            parser.advance(&mut handler, b"\x1b[31m");
            assert_eq!(parser.state.sync_state.timeout.is_sync, 0);
            assert!(handler.attr.take().is_some());
        }
    }

    #[test]
    fn mixed_sync_escape() {
        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        assert_eq!(parser.state.sync_state.timeout.is_sync, 0);
        assert!(handler.attr.is_none());

        // Start synchronized update with immediate SGR.
        parser.advance(&mut handler, b"\x1b[?2026h\x1b[31m");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 1);
        assert!(handler.attr.is_none());

        // Terminate synchronized update and check for SGR.
        parser.advance(&mut handler, b"\x1b[?2026l");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 0);
        assert!(handler.attr.is_some());
    }

    #[test]
    fn sync_bsu_with_esu() {
        let mut parser = Processor::<TestSyncHandler>::new();
        let mut handler = MockHandler::default();

        assert_eq!(parser.state.sync_state.timeout.is_sync, 0);
        assert!(handler.attr.is_none());

        // Start synchronized update with immediate SGR.
        parser.advance(&mut handler, b"\x1b[?2026h\x1b[1m");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 1);
        assert!(handler.attr.is_none());

        // Terminate synchronized update, but immediately start a new one.
        parser.advance(&mut handler, b"\x1b[?2026l\x1b[?2026h\x1b[4m");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 2);
        assert_eq!(handler.attr.take(), Some(Attr::Bold));

        // Terminate again, expecting one buffered SGR.
        parser.advance(&mut handler, b"\x1b[?2026l");
        assert_eq!(parser.state.sync_state.timeout.is_sync, 0);
        assert_eq!(handler.attr.take(), Some(Attr::Underline));
    }

    #[test]
    #[cfg(feature = "std")]
    fn contrast() {
        let rgb1 = Rgb { r: 0xFF, g: 0xFF, b: 0xFF };
        let rgb2 = Rgb { r: 0x00, g: 0x00, b: 0x00 };
        assert!((rgb1.contrast(rgb2) - 21.).abs() < f64::EPSILON);

        let rgb1 = Rgb { r: 0xFF, g: 0xFF, b: 0xFF };
        assert!((rgb1.contrast(rgb1) - 1.).abs() < f64::EPSILON);

        let rgb1 = Rgb { r: 0xFF, g: 0x00, b: 0xFF };
        let rgb2 = Rgb { r: 0x00, g: 0xFF, b: 0x00 };
        assert!((rgb1.contrast(rgb2) - 2.285_543_608_124_253_3).abs() < f64::EPSILON);

        let rgb1 = Rgb { r: 0x12, g: 0x34, b: 0x56 };
        let rgb2 = Rgb { r: 0xFE, g: 0xDC, b: 0xBA };
        assert!((rgb1.contrast(rgb2) - 9.786_558_997_257_74).abs() < f64::EPSILON);
    }
}
