// SPDX-License-Identifier: MIT OR Apache-2.0 OR Zlib
// Copyright 2022-2023 John Nunley
//
// Licensed under the Apache License, Version 2.0, the MIT License, and
// the Zlib license ("the Licenses"), you may not use this file except in
// compliance with one of the the Licenses, at your option. You may obtain
//  a copy of the Licenses at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//     http://opensource.org/licenses/MIT
//     http://opensource.org/licenses/Zlib
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the Licenses is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the Licenses for the specific language governing permissions and
// limitations under the Licenses.
//
// The KEYSYMTAB, CodePair, Keysym::key_char, and Keysym::from_char, including
// their comments, were taken from https://github.com/xkbcommon/libxkbcommon/blob/f3210cbf/src/keysym-utf.c
// which are under Public domain.

//! Keyboard symbols for X11.

#![no_std]
#![allow(non_upper_case_globals)]
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

use core::char;
use core::fmt;

#[rustfmt::skip]
mod automatically_generated;
pub use automatically_generated::*;

/// The type of a raw keyboard code.
pub type RawKeyCode = u32;

/// The keyboard code, often corresponding to a physical key.
///
/// Keyboard events usually return this type directly, and leave it to be the responsibility of the
/// user to convert it to a keyboard symbol.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct KeyCode(RawKeyCode);

impl KeyCode {
    /// Create a new `KeyCode` from a raw keyboard code.
    pub const fn new(raw: RawKeyCode) -> Self {
        Self(raw)
    }

    /// Get the raw keyboard code.
    pub const fn raw(self) -> RawKeyCode {
        self.0
    }
}

impl From<RawKeyCode> for KeyCode {
    fn from(raw: RawKeyCode) -> Self {
        Self::new(raw)
    }
}

impl From<KeyCode> for RawKeyCode {
    fn from(keycode: KeyCode) -> Self {
        keycode.raw()
    }
}

impl From<u16> for KeyCode {
    fn from(raw: u16) -> Self {
        Self::new(raw as RawKeyCode)
    }
}

impl From<KeyCode> for u16 {
    fn from(keycode: KeyCode) -> Self {
        keycode.raw() as u16
    }
}

impl From<u8> for KeyCode {
    fn from(raw: u8) -> Self {
        Self::new(raw as RawKeyCode)
    }
}

impl From<KeyCode> for u8 {
    fn from(keycode: KeyCode) -> Self {
        keycode.raw() as u8
    }
}

/// The type of a raw keyboard symbol.
pub type RawKeysym = u32;

/// The keyboard symbol, often corresponding to a character.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(transparent)]
pub struct Keysym(RawKeysym);

impl fmt::Debug for Keysym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.name() {
            Some(name) => f.write_str(name),
            None => write!(f, "{:#x}", self.0),
        }
    }
}

impl Keysym {
    /// Create a new `Keysym` from a raw keyboard symbol.
    pub const fn new(raw: RawKeysym) -> Self {
        Self(raw)
    }

    /// Get the raw keyboard symbol.
    pub const fn raw(self) -> RawKeysym {
        self.0
    }

    /// Get a string corresponding to the name of this keyboard symbol.
    ///
    /// The output of this function is not stable and is intended for debugging purposes.
    pub const fn name(self) -> Option<&'static str> {
        name(self)
    }

    /// Tell whether a keysym is a keypad key.
    pub const fn is_keypad_key(self) -> bool {
        matches!(self.0, key::KP_Space..=key::KP_Equal)
    }

    /// Tell whether a keysym is a private keypad key.
    pub const fn is_private_keypad_key(self) -> bool {
        matches!(self.0, 0x11000000..=0x1100FFFF)
    }

    /// Tell whether a keysym is a cursor key.
    pub const fn is_cursor_key(self) -> bool {
        matches!(self.0, key::Home..=key::Select)
    }

    /// Tell whether a keysym is a PF key.
    pub const fn is_pf_key(self) -> bool {
        matches!(self.0, key::KP_F1..=key::KP_F4)
    }

    /// Tell whether a keysym is a function key.
    pub const fn is_function_key(self) -> bool {
        matches!(self.0, key::F1..=key::F35)
    }

    /// Tell whether a key is a miscellaneous function key.
    pub const fn is_misc_function_key(self) -> bool {
        matches!(self.0, key::Select..=key::Break)
    }

    /// Tell whether a key is a modifier key.
    pub const fn is_modifier_key(self) -> bool {
        matches!(
            self.0,
            key::Shift_L..=key::Hyper_R
            | key::ISO_Lock..=key::ISO_Level5_Lock
            | key::Mode_switch
            | key::Num_Lock
        )
    }

    /// Translate a keyboard symbol to its approximate character.
    ///
    /// The translation follows the `xkb_keysym_to_utf32` function.
    pub fn key_char(self) -> Option<char> {
        let keysym = self.0;

        // First check for Latin-1 characters (1:1 mapping).
        if matches!(keysym, 0x0020..=0x007e) || matches!(keysym, 0x00a0..=0x00ff) {
            return char::from_u32(keysym);
        }

        // Patch encoding botch.
        if keysym == key::KP_Space {
            return char::from_u32(key::space & 0x7f);
        }

        // Special keysyms.
        if matches!(keysym, key::BackSpace..=key::Clear)
            || matches!(keysym, key::KP_Multiply..=key::KP_9)
            || keysym == key::Return
            || keysym == key::Escape
            || keysym == key::Delete
            || keysym == key::KP_Tab
            || keysym == key::KP_Enter
            || keysym == key::KP_Equal
        {
            return char::from_u32(keysym & 0x7f);
        }

        // Also check for directly encoded unicode codepoints.

        // Exclude surrogates: they are invalid in UTF-32.
        // See https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#G28875
        // for further details.
        if matches!(keysym, 0x0100d800..=0x0100dfff) {
            return None;
        }

        // In theory, this is supposed to start from 0x100100, such that the ASCII
        // range, which is already covered by 0x00-0xff, can't be encoded in two
        // ways. However, changing this after a couple of decades probably won't
        // go well, so it stays as it is.
        if matches!(keysym, 0x01000000..=0x0110ffff) {
            return char::from_u32(keysym - 0x01000000);
        }

        // Search the main table.
        KEYSYMTAB
            .binary_search_by_key(&keysym, |pair| pair.keysym as u32)
            .ok()
            .and_then(|idx| char::from_u32(KEYSYMTAB[idx].ucs as u32))
    }

    /// Create a [`Keysym`] from the given `char`.
    ///
    /// This function replecates the `xkb_utf32_to_keysym` behavior.
    pub fn from_char(ch: char) -> Self {
        let ucs = ch as u32;

        // First check for Latin-1 characters (1:1 mapping).
        if matches!(ucs, 0x0020..=0x007e) || matches!(ucs, 0x00a0..=0x00ff) {
            return Self::new(ucs);
        }

        // Special keysyms.
        if (key::BackSpace & 0x7f..=key::Clear & 0x7f).contains(&ucs)
            || ucs == key::Return & 0x7f
            || ucs == key::Escape & 0x7f
        {
            return Self::new(ucs | 0xff00);
        }

        if ucs == key::Delete & 0x7f {
            return Self::new(key::Delete);
        }

        // Unicode non-symbols and code points outside Unicode planes.
        if matches!(ucs, 0xd800..=0xdfff)
            || matches!(ucs, 0xfdd0..=0xfdef)
            || ucs > 0x10ffff
            || (ucs & 0xfffe == 0xfffe)
        {
            return Keysym::NoSymbol;
        }

        // Search main table.
        KEYSYMTAB
            .iter()
            .find(|pair| pair.ucs as u32 == ucs)
            .map_or(Self::new(ucs | 0x01000000), |pair| {
                Self::new(pair.keysym as u32)
            })
    }
}

impl From<u32> for Keysym {
    fn from(raw: u32) -> Self {
        Self(raw)
    }
}

impl From<Keysym> for u32 {
    fn from(keysym: Keysym) -> Self {
        keysym.0
    }
}

/// The "empty" keyboard symbol.
pub const NO_SYMBOL: Keysym = Keysym::NoSymbol;

/// Get the keyboard symbol from a keyboard code and its column.
///
/// `min_keycode` can be retrieved from the X11 setup, and `keysyms_per_keycode` and `keysyms` can be
/// retrieved from the X11 server through the `GetKeyboardMapping` request.
pub fn keysym(
    keycode: KeyCode,
    mut column: u8,
    min_keycode: KeyCode,
    keysyms_per_keycode: u8,
    keysyms: &[RawKeysym],
) -> Option<Keysym> {
    if column >= keysyms_per_keycode && column > 3 {
        return None;
    }

    // Get the keysyms to consider.
    let start = (keycode.0 - min_keycode.0) as usize * keysyms_per_keycode as usize;
    let end = start + keysyms_per_keycode as usize;
    let keysyms = &keysyms[start..end];

    // See which keysym we should get.
    let mut per = keysyms_per_keycode as usize;
    if column < 4 {
        // If we're going past the traditional upper/lower keys, we need to figure out where
        // our keysym is.
        if column >= 2 {
            // See how many keysyms we actually have in this column.
            loop {
                // There will always be at least one keysym in this column.
                if per <= 1 {
                    break;
                }

                // If the keysym we're looking at isn't NoSymbol, we're done.
                if keysyms[per - 1] != Keysym::NoSymbol.0 {
                    break;
                }

                // This column isn't as big as `per`, subtract it.
                per -= 1;
            }

            // If this keysym doesn't go past the traditional upper/lower keys, adjust column
            // accordingly.
            if per <= 2 {
                column %= 2;
            }
        }

        // Convert to upper/lower ourselves if the keysym doesn't support it.
        let alt_column = (column | 1) as usize;
        if per <= alt_column || keysyms[alt_column] == Keysym::NoSymbol.0 {
            // Convert to upper/lower case.
            let (upper, lower) = convert_case(Keysym(*keysyms.get(column as usize & !1)?));
            return Some(if column & 1 == 0 { upper } else { lower });
        }
    }

    keysyms.get(column as usize).map(|&keysym| Keysym(keysym))
}

/// Convert a keysym to its uppercase/lowercase equivalents.
const fn convert_case(keysym: Keysym) -> (Keysym, Keysym) {
    // by default, they're both the regular keysym
    let (mut upper, mut lower) = (keysym.0, keysym.0);

    // tell which language it belongs to
    #[allow(non_upper_case_globals)]
    match keysym.0 {
        key::A..=key::Z => lower += key::a - key::A,
        key::a..=key::z => upper -= key::a - key::A,
        key::Agrave..=key::Odiaeresis => lower += key::agrave - key::Agrave,
        key::agrave..=key::odiaeresis => upper -= key::agrave - key::Agrave,
        key::Ooblique..=key::Thorn => lower += key::oslash - key::Ooblique,
        key::oslash..=key::thorn => upper -= key::oslash - key::Ooblique,
        key::Aogonek => lower = key::aogonek,
        key::aogonek => upper = key::Aogonek,
        key::Lstroke..=key::Sacute => lower += key::lstroke - key::Lstroke,
        key::lstroke..=key::sacute => upper -= key::lstroke - key::Lstroke,
        key::Scaron..=key::Zacute => lower += key::scaron - key::Scaron,
        key::scaron..=key::zacute => upper -= key::scaron - key::Scaron,
        key::Zcaron..=key::Zabovedot => lower += key::zcaron - key::Zcaron,
        key::zcaron..=key::zabovedot => upper -= key::zcaron - key::Zcaron,
        key::Racute..=key::Tcedilla => lower += key::racute - key::Racute,
        key::racute..=key::tcedilla => upper -= key::racute - key::Racute,
        key::Hstroke..=key::Hcircumflex => lower += key::hstroke - key::Hstroke,
        key::hstroke..=key::hcircumflex => upper -= key::hstroke - key::Hstroke,
        key::Gbreve..=key::Jcircumflex => lower += key::gbreve - key::Gbreve,
        key::gbreve..=key::jcircumflex => upper -= key::gbreve - key::Gbreve,
        key::Cabovedot..=key::Scircumflex => lower += key::cabovedot - key::Cabovedot,
        key::cabovedot..=key::scircumflex => upper -= key::cabovedot - key::Cabovedot,
        key::Rcedilla..=key::Tslash => lower += key::rcedilla - key::Rcedilla,
        key::rcedilla..=key::tslash => upper -= key::rcedilla - key::Rcedilla,
        key::ENG => lower = key::eng,
        key::eng => upper = key::ENG,
        key::Amacron..=key::Umacron => lower += key::amacron - key::Amacron,
        key::amacron..=key::umacron => upper -= key::amacron - key::Amacron,
        key::Serbian_DJE..=key::Serbian_DZE => lower -= key::Serbian_DJE - key::Serbian_dje,
        key::Serbian_dje..=key::Serbian_dze => upper += key::Serbian_DJE - key::Serbian_dje,
        key::Cyrillic_YU..=key::Cyrillic_HARDSIGN => lower -= key::Cyrillic_YU - key::Cyrillic_yu,
        key::Cyrillic_yu..=key::Cyrillic_hardsign => upper += key::Cyrillic_YU - key::Cyrillic_yu,
        key::Greek_ALPHAaccent..=key::Greek_OMEGAaccent => {
            lower += key::Greek_alphaaccent - key::Greek_ALPHAaccent
        }
        key::Greek_alphaaccent..=key::Greek_omegaaccent
            if !matches!(
                keysym.0,
                key::Greek_iotaaccentdieresis | key::Greek_upsilonaccentdieresis
            ) =>
        {
            upper -= key::Greek_alphaaccent - key::Greek_ALPHAaccent
        }
        key::Greek_ALPHA..=key::Greek_OMEGA => lower += key::Greek_alpha - key::Greek_ALPHA,
        key::Greek_alpha..=key::Greek_omega if !matches!(keysym.0, key::Greek_finalsmallsigma) => {
            upper -= key::Greek_alpha - key::Greek_ALPHA
        }
        key::Armenian_AYB..=key::Armenian_fe => {
            lower |= 1;
            upper &= !1;
        }
        _ => {}
    }

    (Keysym(upper), Keysym(lower))
}

// We don't use the uint32_t types here, to save some space.
struct CodePair {
    keysym: u16,
    ucs: u16,
}

#[rustfmt::skip]
const KEYSYMTAB: [CodePair; 763] = [
    CodePair { keysym: 0x01a1, ucs: 0x0104 }, /*                     Aogonek Ą LATIN CAPITAL LETTER A WITH OGONEK */
    CodePair { keysym: 0x01a2, ucs: 0x02d8 }, /*                       breve ˘ BREVE */
    CodePair { keysym: 0x01a3, ucs: 0x0141 }, /*                     Lstroke Ł LATIN CAPITAL LETTER L WITH STROKE */
    CodePair { keysym: 0x01a5, ucs: 0x013d }, /*                      Lcaron Ľ LATIN CAPITAL LETTER L WITH CARON */
    CodePair { keysym: 0x01a6, ucs: 0x015a }, /*                      Sacute Ś LATIN CAPITAL LETTER S WITH ACUTE */
    CodePair { keysym: 0x01a9, ucs: 0x0160 }, /*                      Scaron Š LATIN CAPITAL LETTER S WITH CARON */
    CodePair { keysym: 0x01aa, ucs: 0x015e }, /*                    Scedilla Ş LATIN CAPITAL LETTER S WITH CEDILLA */
    CodePair { keysym: 0x01ab, ucs: 0x0164 }, /*                      Tcaron Ť LATIN CAPITAL LETTER T WITH CARON */
    CodePair { keysym: 0x01ac, ucs: 0x0179 }, /*                      Zacute Ź LATIN CAPITAL LETTER Z WITH ACUTE */
    CodePair { keysym: 0x01ae, ucs: 0x017d }, /*                      Zcaron Ž LATIN CAPITAL LETTER Z WITH CARON */
    CodePair { keysym: 0x01af, ucs: 0x017b }, /*                   Zabovedot Ż LATIN CAPITAL LETTER Z WITH DOT ABOVE */
    CodePair { keysym: 0x01b1, ucs: 0x0105 }, /*                     aogonek ą LATIN SMALL LETTER A WITH OGONEK */
    CodePair { keysym: 0x01b2, ucs: 0x02db }, /*                      ogonek ˛ OGONEK */
    CodePair { keysym: 0x01b3, ucs: 0x0142 }, /*                     lstroke ł LATIN SMALL LETTER L WITH STROKE */
    CodePair { keysym: 0x01b5, ucs: 0x013e }, /*                      lcaron ľ LATIN SMALL LETTER L WITH CARON */
    CodePair { keysym: 0x01b6, ucs: 0x015b }, /*                      sacute ś LATIN SMALL LETTER S WITH ACUTE */
    CodePair { keysym: 0x01b7, ucs: 0x02c7 }, /*                       caron ˇ CARON */
    CodePair { keysym: 0x01b9, ucs: 0x0161 }, /*                      scaron š LATIN SMALL LETTER S WITH CARON */
    CodePair { keysym: 0x01ba, ucs: 0x015f }, /*                    scedilla ş LATIN SMALL LETTER S WITH CEDILLA */
    CodePair { keysym: 0x01bb, ucs: 0x0165 }, /*                      tcaron ť LATIN SMALL LETTER T WITH CARON */
    CodePair { keysym: 0x01bc, ucs: 0x017a }, /*                      zacute ź LATIN SMALL LETTER Z WITH ACUTE */
    CodePair { keysym: 0x01bd, ucs: 0x02dd }, /*                 doubleacute ˝ DOUBLE ACUTE ACCENT */
    CodePair { keysym: 0x01be, ucs: 0x017e }, /*                      zcaron ž LATIN SMALL LETTER Z WITH CARON */
    CodePair { keysym: 0x01bf, ucs: 0x017c }, /*                   zabovedot ż LATIN SMALL LETTER Z WITH DOT ABOVE */
    CodePair { keysym: 0x01c0, ucs: 0x0154 }, /*                      Racute Ŕ LATIN CAPITAL LETTER R WITH ACUTE */
    CodePair { keysym: 0x01c3, ucs: 0x0102 }, /*                      Abreve Ă LATIN CAPITAL LETTER A WITH BREVE */
    CodePair { keysym: 0x01c5, ucs: 0x0139 }, /*                      Lacute Ĺ LATIN CAPITAL LETTER L WITH ACUTE */
    CodePair { keysym: 0x01c6, ucs: 0x0106 }, /*                      Cacute Ć LATIN CAPITAL LETTER C WITH ACUTE */
    CodePair { keysym: 0x01c8, ucs: 0x010c }, /*                      Ccaron Č LATIN CAPITAL LETTER C WITH CARON */
    CodePair { keysym: 0x01ca, ucs: 0x0118 }, /*                     Eogonek Ę LATIN CAPITAL LETTER E WITH OGONEK */
    CodePair { keysym: 0x01cc, ucs: 0x011a }, /*                      Ecaron Ě LATIN CAPITAL LETTER E WITH CARON */
    CodePair { keysym: 0x01cf, ucs: 0x010e }, /*                      Dcaron Ď LATIN CAPITAL LETTER D WITH CARON */
    CodePair { keysym: 0x01d0, ucs: 0x0110 }, /*                     Dstroke Đ LATIN CAPITAL LETTER D WITH STROKE */
    CodePair { keysym: 0x01d1, ucs: 0x0143 }, /*                      Nacute Ń LATIN CAPITAL LETTER N WITH ACUTE */
    CodePair { keysym: 0x01d2, ucs: 0x0147 }, /*                      Ncaron Ň LATIN CAPITAL LETTER N WITH CARON */
    CodePair { keysym: 0x01d5, ucs: 0x0150 }, /*                Odoubleacute Ő LATIN CAPITAL LETTER O WITH DOUBLE ACUTE */
    CodePair { keysym: 0x01d8, ucs: 0x0158 }, /*                      Rcaron Ř LATIN CAPITAL LETTER R WITH CARON */
    CodePair { keysym: 0x01d9, ucs: 0x016e }, /*                       Uring Ů LATIN CAPITAL LETTER U WITH RING ABOVE */
    CodePair { keysym: 0x01db, ucs: 0x0170 }, /*                Udoubleacute Ű LATIN CAPITAL LETTER U WITH DOUBLE ACUTE */
    CodePair { keysym: 0x01de, ucs: 0x0162 }, /*                    Tcedilla Ţ LATIN CAPITAL LETTER T WITH CEDILLA */
    CodePair { keysym: 0x01e0, ucs: 0x0155 }, /*                      racute ŕ LATIN SMALL LETTER R WITH ACUTE */
    CodePair { keysym: 0x01e3, ucs: 0x0103 }, /*                      abreve ă LATIN SMALL LETTER A WITH BREVE */
    CodePair { keysym: 0x01e5, ucs: 0x013a }, /*                      lacute ĺ LATIN SMALL LETTER L WITH ACUTE */
    CodePair { keysym: 0x01e6, ucs: 0x0107 }, /*                      cacute ć LATIN SMALL LETTER C WITH ACUTE */
    CodePair { keysym: 0x01e8, ucs: 0x010d }, /*                      ccaron č LATIN SMALL LETTER C WITH CARON */
    CodePair { keysym: 0x01ea, ucs: 0x0119 }, /*                     eogonek ę LATIN SMALL LETTER E WITH OGONEK */
    CodePair { keysym: 0x01ec, ucs: 0x011b }, /*                      ecaron ě LATIN SMALL LETTER E WITH CARON */
    CodePair { keysym: 0x01ef, ucs: 0x010f }, /*                      dcaron ď LATIN SMALL LETTER D WITH CARON */
    CodePair { keysym: 0x01f0, ucs: 0x0111 }, /*                     dstroke đ LATIN SMALL LETTER D WITH STROKE */
    CodePair { keysym: 0x01f1, ucs: 0x0144 }, /*                      nacute ń LATIN SMALL LETTER N WITH ACUTE */
    CodePair { keysym: 0x01f2, ucs: 0x0148 }, /*                      ncaron ň LATIN SMALL LETTER N WITH CARON */
    CodePair { keysym: 0x01f5, ucs: 0x0151 }, /*                odoubleacute ő LATIN SMALL LETTER O WITH DOUBLE ACUTE */
    CodePair { keysym: 0x01f8, ucs: 0x0159 }, /*                      rcaron ř LATIN SMALL LETTER R WITH CARON */
    CodePair { keysym: 0x01f9, ucs: 0x016f }, /*                       uring ů LATIN SMALL LETTER U WITH RING ABOVE */
    CodePair { keysym: 0x01fb, ucs: 0x0171 }, /*                udoubleacute ű LATIN SMALL LETTER U WITH DOUBLE ACUTE */
    CodePair { keysym: 0x01fe, ucs: 0x0163 }, /*                    tcedilla ţ LATIN SMALL LETTER T WITH CEDILLA */
    CodePair { keysym: 0x01ff, ucs: 0x02d9 }, /*                    abovedot ˙ DOT ABOVE */
    CodePair { keysym: 0x02a1, ucs: 0x0126 }, /*                     Hstroke Ħ LATIN CAPITAL LETTER H WITH STROKE */
    CodePair { keysym: 0x02a6, ucs: 0x0124 }, /*                 Hcircumflex Ĥ LATIN CAPITAL LETTER H WITH CIRCUMFLEX */
    CodePair { keysym: 0x02a9, ucs: 0x0130 }, /*                   Iabovedot İ LATIN CAPITAL LETTER I WITH DOT ABOVE */
    CodePair { keysym: 0x02ab, ucs: 0x011e }, /*                      Gbreve Ğ LATIN CAPITAL LETTER G WITH BREVE */
    CodePair { keysym: 0x02ac, ucs: 0x0134 }, /*                 Jcircumflex Ĵ LATIN CAPITAL LETTER J WITH CIRCUMFLEX */
    CodePair { keysym: 0x02b1, ucs: 0x0127 }, /*                     hstroke ħ LATIN SMALL LETTER H WITH STROKE */
    CodePair { keysym: 0x02b6, ucs: 0x0125 }, /*                 hcircumflex ĥ LATIN SMALL LETTER H WITH CIRCUMFLEX */
    CodePair { keysym: 0x02b9, ucs: 0x0131 }, /*                    idotless ı LATIN SMALL LETTER DOTLESS I */
    CodePair { keysym: 0x02bb, ucs: 0x011f }, /*                      gbreve ğ LATIN SMALL LETTER G WITH BREVE */
    CodePair { keysym: 0x02bc, ucs: 0x0135 }, /*                 jcircumflex ĵ LATIN SMALL LETTER J WITH CIRCUMFLEX */
    CodePair { keysym: 0x02c5, ucs: 0x010a }, /*                   Cabovedot Ċ LATIN CAPITAL LETTER C WITH DOT ABOVE */
    CodePair { keysym: 0x02c6, ucs: 0x0108 }, /*                 Ccircumflex Ĉ LATIN CAPITAL LETTER C WITH CIRCUMFLEX */
    CodePair { keysym: 0x02d5, ucs: 0x0120 }, /*                   Gabovedot Ġ LATIN CAPITAL LETTER G WITH DOT ABOVE */
    CodePair { keysym: 0x02d8, ucs: 0x011c }, /*                 Gcircumflex Ĝ LATIN CAPITAL LETTER G WITH CIRCUMFLEX */
    CodePair { keysym: 0x02dd, ucs: 0x016c }, /*                      Ubreve Ŭ LATIN CAPITAL LETTER U WITH BREVE */
    CodePair { keysym: 0x02de, ucs: 0x015c }, /*                 Scircumflex Ŝ LATIN CAPITAL LETTER S WITH CIRCUMFLEX */
    CodePair { keysym: 0x02e5, ucs: 0x010b }, /*                   cabovedot ċ LATIN SMALL LETTER C WITH DOT ABOVE */
    CodePair { keysym: 0x02e6, ucs: 0x0109 }, /*                 ccircumflex ĉ LATIN SMALL LETTER C WITH CIRCUMFLEX */
    CodePair { keysym: 0x02f5, ucs: 0x0121 }, /*                   gabovedot ġ LATIN SMALL LETTER G WITH DOT ABOVE */
    CodePair { keysym: 0x02f8, ucs: 0x011d }, /*                 gcircumflex ĝ LATIN SMALL LETTER G WITH CIRCUMFLEX */
    CodePair { keysym: 0x02fd, ucs: 0x016d }, /*                      ubreve ŭ LATIN SMALL LETTER U WITH BREVE */
    CodePair { keysym: 0x02fe, ucs: 0x015d }, /*                 scircumflex ŝ LATIN SMALL LETTER S WITH CIRCUMFLEX */
    CodePair { keysym: 0x03a2, ucs: 0x0138 }, /*                         kra ĸ LATIN SMALL LETTER KRA */
    CodePair { keysym: 0x03a3, ucs: 0x0156 }, /*                    Rcedilla Ŗ LATIN CAPITAL LETTER R WITH CEDILLA */
    CodePair { keysym: 0x03a5, ucs: 0x0128 }, /*                      Itilde Ĩ LATIN CAPITAL LETTER I WITH TILDE */
    CodePair { keysym: 0x03a6, ucs: 0x013b }, /*                    Lcedilla Ļ LATIN CAPITAL LETTER L WITH CEDILLA */
    CodePair { keysym: 0x03aa, ucs: 0x0112 }, /*                     Emacron Ē LATIN CAPITAL LETTER E WITH MACRON */
    CodePair { keysym: 0x03ab, ucs: 0x0122 }, /*                    Gcedilla Ģ LATIN CAPITAL LETTER G WITH CEDILLA */
    CodePair { keysym: 0x03ac, ucs: 0x0166 }, /*                      Tslash Ŧ LATIN CAPITAL LETTER T WITH STROKE */
    CodePair { keysym: 0x03b3, ucs: 0x0157 }, /*                    rcedilla ŗ LATIN SMALL LETTER R WITH CEDILLA */
    CodePair { keysym: 0x03b5, ucs: 0x0129 }, /*                      itilde ĩ LATIN SMALL LETTER I WITH TILDE */
    CodePair { keysym: 0x03b6, ucs: 0x013c }, /*                    lcedilla ļ LATIN SMALL LETTER L WITH CEDILLA */
    CodePair { keysym: 0x03ba, ucs: 0x0113 }, /*                     emacron ē LATIN SMALL LETTER E WITH MACRON */
    CodePair { keysym: 0x03bb, ucs: 0x0123 }, /*                    gcedilla ģ LATIN SMALL LETTER G WITH CEDILLA */
    CodePair { keysym: 0x03bc, ucs: 0x0167 }, /*                      tslash ŧ LATIN SMALL LETTER T WITH STROKE */
    CodePair { keysym: 0x03bd, ucs: 0x014a }, /*                         ENG Ŋ LATIN CAPITAL LETTER ENG */
    CodePair { keysym: 0x03bf, ucs: 0x014b }, /*                         eng ŋ LATIN SMALL LETTER ENG */
    CodePair { keysym: 0x03c0, ucs: 0x0100 }, /*                     Amacron Ā LATIN CAPITAL LETTER A WITH MACRON */
    CodePair { keysym: 0x03c7, ucs: 0x012e }, /*                     Iogonek Į LATIN CAPITAL LETTER I WITH OGONEK */
    CodePair { keysym: 0x03cc, ucs: 0x0116 }, /*                   Eabovedot Ė LATIN CAPITAL LETTER E WITH DOT ABOVE */
    CodePair { keysym: 0x03cf, ucs: 0x012a }, /*                     Imacron Ī LATIN CAPITAL LETTER I WITH MACRON */
    CodePair { keysym: 0x03d1, ucs: 0x0145 }, /*                    Ncedilla Ņ LATIN CAPITAL LETTER N WITH CEDILLA */
    CodePair { keysym: 0x03d2, ucs: 0x014c }, /*                     Omacron Ō LATIN CAPITAL LETTER O WITH MACRON */
    CodePair { keysym: 0x03d3, ucs: 0x0136 }, /*                    Kcedilla Ķ LATIN CAPITAL LETTER K WITH CEDILLA */
    CodePair { keysym: 0x03d9, ucs: 0x0172 }, /*                     Uogonek Ų LATIN CAPITAL LETTER U WITH OGONEK */
    CodePair { keysym: 0x03dd, ucs: 0x0168 }, /*                      Utilde Ũ LATIN CAPITAL LETTER U WITH TILDE */
    CodePair { keysym: 0x03de, ucs: 0x016a }, /*                     Umacron Ū LATIN CAPITAL LETTER U WITH MACRON */
    CodePair { keysym: 0x03e0, ucs: 0x0101 }, /*                     amacron ā LATIN SMALL LETTER A WITH MACRON */
    CodePair { keysym: 0x03e7, ucs: 0x012f }, /*                     iogonek į LATIN SMALL LETTER I WITH OGONEK */
    CodePair { keysym: 0x03ec, ucs: 0x0117 }, /*                   eabovedot ė LATIN SMALL LETTER E WITH DOT ABOVE */
    CodePair { keysym: 0x03ef, ucs: 0x012b }, /*                     imacron ī LATIN SMALL LETTER I WITH MACRON */
    CodePair { keysym: 0x03f1, ucs: 0x0146 }, /*                    ncedilla ņ LATIN SMALL LETTER N WITH CEDILLA */
    CodePair { keysym: 0x03f2, ucs: 0x014d }, /*                     omacron ō LATIN SMALL LETTER O WITH MACRON */
    CodePair { keysym: 0x03f3, ucs: 0x0137 }, /*                    kcedilla ķ LATIN SMALL LETTER K WITH CEDILLA */
    CodePair { keysym: 0x03f9, ucs: 0x0173 }, /*                     uogonek ų LATIN SMALL LETTER U WITH OGONEK */
    CodePair { keysym: 0x03fd, ucs: 0x0169 }, /*                      utilde ũ LATIN SMALL LETTER U WITH TILDE */
    CodePair { keysym: 0x03fe, ucs: 0x016b }, /*                     umacron ū LATIN SMALL LETTER U WITH MACRON */
    CodePair { keysym: 0x047e, ucs: 0x203e }, /*                    overline ‾ OVERLINE */
    CodePair { keysym: 0x04a1, ucs: 0x3002 }, /*               kana_fullstop 。 IDEOGRAPHIC FULL STOP */
    CodePair { keysym: 0x04a2, ucs: 0x300c }, /*         kana_openingbracket 「 LEFT CORNER BRACKET */
    CodePair { keysym: 0x04a3, ucs: 0x300d }, /*         kana_closingbracket 」 RIGHT CORNER BRACKET */
    CodePair { keysym: 0x04a4, ucs: 0x3001 }, /*                  kana_comma 、 IDEOGRAPHIC COMMA */
    CodePair { keysym: 0x04a5, ucs: 0x30fb }, /*            kana_conjunctive ・ KATAKANA MIDDLE DOT */
    CodePair { keysym: 0x04a6, ucs: 0x30f2 }, /*                     kana_WO ヲ KATAKANA LETTER WO */
    CodePair { keysym: 0x04a7, ucs: 0x30a1 }, /*                      kana_a ァ KATAKANA LETTER SMALL A */
    CodePair { keysym: 0x04a8, ucs: 0x30a3 }, /*                      kana_i ィ KATAKANA LETTER SMALL I */
    CodePair { keysym: 0x04a9, ucs: 0x30a5 }, /*                      kana_u ゥ KATAKANA LETTER SMALL U */
    CodePair { keysym: 0x04aa, ucs: 0x30a7 }, /*                      kana_e ェ KATAKANA LETTER SMALL E */
    CodePair { keysym: 0x04ab, ucs: 0x30a9 }, /*                      kana_o ォ KATAKANA LETTER SMALL O */
    CodePair { keysym: 0x04ac, ucs: 0x30e3 }, /*                     kana_ya ャ KATAKANA LETTER SMALL YA */
    CodePair { keysym: 0x04ad, ucs: 0x30e5 }, /*                     kana_yu ュ KATAKANA LETTER SMALL YU */
    CodePair { keysym: 0x04ae, ucs: 0x30e7 }, /*                     kana_yo ョ KATAKANA LETTER SMALL YO */
    CodePair { keysym: 0x04af, ucs: 0x30c3 }, /*                    kana_tsu ッ KATAKANA LETTER SMALL TU */
    CodePair { keysym: 0x04b0, ucs: 0x30fc }, /*              prolongedsound ー KATAKANA-HIRAGANA PROLONGED SOUND MARK */
    CodePair { keysym: 0x04b1, ucs: 0x30a2 }, /*                      kana_A ア KATAKANA LETTER A */
    CodePair { keysym: 0x04b2, ucs: 0x30a4 }, /*                      kana_I イ KATAKANA LETTER I */
    CodePair { keysym: 0x04b3, ucs: 0x30a6 }, /*                      kana_U ウ KATAKANA LETTER U */
    CodePair { keysym: 0x04b4, ucs: 0x30a8 }, /*                      kana_E エ KATAKANA LETTER E */
    CodePair { keysym: 0x04b5, ucs: 0x30aa }, /*                      kana_O オ KATAKANA LETTER O */
    CodePair { keysym: 0x04b6, ucs: 0x30ab }, /*                     kana_KA カ KATAKANA LETTER KA */
    CodePair { keysym: 0x04b7, ucs: 0x30ad }, /*                     kana_KI キ KATAKANA LETTER KI */
    CodePair { keysym: 0x04b8, ucs: 0x30af }, /*                     kana_KU ク KATAKANA LETTER KU */
    CodePair { keysym: 0x04b9, ucs: 0x30b1 }, /*                     kana_KE ケ KATAKANA LETTER KE */
    CodePair { keysym: 0x04ba, ucs: 0x30b3 }, /*                     kana_KO コ KATAKANA LETTER KO */
    CodePair { keysym: 0x04bb, ucs: 0x30b5 }, /*                     kana_SA サ KATAKANA LETTER SA */
    CodePair { keysym: 0x04bc, ucs: 0x30b7 }, /*                    kana_SHI シ KATAKANA LETTER SI */
    CodePair { keysym: 0x04bd, ucs: 0x30b9 }, /*                     kana_SU ス KATAKANA LETTER SU */
    CodePair { keysym: 0x04be, ucs: 0x30bb }, /*                     kana_SE セ KATAKANA LETTER SE */
    CodePair { keysym: 0x04bf, ucs: 0x30bd }, /*                     kana_SO ソ KATAKANA LETTER SO */
    CodePair { keysym: 0x04c0, ucs: 0x30bf }, /*                     kana_TA タ KATAKANA LETTER TA */
    CodePair { keysym: 0x04c1, ucs: 0x30c1 }, /*                    kana_CHI チ KATAKANA LETTER TI */
    CodePair { keysym: 0x04c2, ucs: 0x30c4 }, /*                    kana_TSU ツ KATAKANA LETTER TU */
    CodePair { keysym: 0x04c3, ucs: 0x30c6 }, /*                     kana_TE テ KATAKANA LETTER TE */
    CodePair { keysym: 0x04c4, ucs: 0x30c8 }, /*                     kana_TO ト KATAKANA LETTER TO */
    CodePair { keysym: 0x04c5, ucs: 0x30ca }, /*                     kana_NA ナ KATAKANA LETTER NA */
    CodePair { keysym: 0x04c6, ucs: 0x30cb }, /*                     kana_NI ニ KATAKANA LETTER NI */
    CodePair { keysym: 0x04c7, ucs: 0x30cc }, /*                     kana_NU ヌ KATAKANA LETTER NU */
    CodePair { keysym: 0x04c8, ucs: 0x30cd }, /*                     kana_NE ネ KATAKANA LETTER NE */
    CodePair { keysym: 0x04c9, ucs: 0x30ce }, /*                     kana_NO ノ KATAKANA LETTER NO */
    CodePair { keysym: 0x04ca, ucs: 0x30cf }, /*                     kana_HA ハ KATAKANA LETTER HA */
    CodePair { keysym: 0x04cb, ucs: 0x30d2 }, /*                     kana_HI ヒ KATAKANA LETTER HI */
    CodePair { keysym: 0x04cc, ucs: 0x30d5 }, /*                     kana_FU フ KATAKANA LETTER HU */
    CodePair { keysym: 0x04cd, ucs: 0x30d8 }, /*                     kana_HE ヘ KATAKANA LETTER HE */
    CodePair { keysym: 0x04ce, ucs: 0x30db }, /*                     kana_HO ホ KATAKANA LETTER HO */
    CodePair { keysym: 0x04cf, ucs: 0x30de }, /*                     kana_MA マ KATAKANA LETTER MA */
    CodePair { keysym: 0x04d0, ucs: 0x30df }, /*                     kana_MI ミ KATAKANA LETTER MI */
    CodePair { keysym: 0x04d1, ucs: 0x30e0 }, /*                     kana_MU ム KATAKANA LETTER MU */
    CodePair { keysym: 0x04d2, ucs: 0x30e1 }, /*                     kana_ME メ KATAKANA LETTER ME */
    CodePair { keysym: 0x04d3, ucs: 0x30e2 }, /*                     kana_MO モ KATAKANA LETTER MO */
    CodePair { keysym: 0x04d4, ucs: 0x30e4 }, /*                     kana_YA ヤ KATAKANA LETTER YA */
    CodePair { keysym: 0x04d5, ucs: 0x30e6 }, /*                     kana_YU ユ KATAKANA LETTER YU */
    CodePair { keysym: 0x04d6, ucs: 0x30e8 }, /*                     kana_YO ヨ KATAKANA LETTER YO */
    CodePair { keysym: 0x04d7, ucs: 0x30e9 }, /*                     kana_RA ラ KATAKANA LETTER RA */
    CodePair { keysym: 0x04d8, ucs: 0x30ea }, /*                     kana_RI リ KATAKANA LETTER RI */
    CodePair { keysym: 0x04d9, ucs: 0x30eb }, /*                     kana_RU ル KATAKANA LETTER RU */
    CodePair { keysym: 0x04da, ucs: 0x30ec }, /*                     kana_RE レ KATAKANA LETTER RE */
    CodePair { keysym: 0x04db, ucs: 0x30ed }, /*                     kana_RO ロ KATAKANA LETTER RO */
    CodePair { keysym: 0x04dc, ucs: 0x30ef }, /*                     kana_WA ワ KATAKANA LETTER WA */
    CodePair { keysym: 0x04dd, ucs: 0x30f3 }, /*                      kana_N ン KATAKANA LETTER N */
    CodePair { keysym: 0x04de, ucs: 0x309b }, /*                 voicedsound ゛ KATAKANA-HIRAGANA VOICED SOUND MARK */
    CodePair { keysym: 0x04df, ucs: 0x309c }, /*             semivoicedsound ゜ KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK */
    CodePair { keysym: 0x05ac, ucs: 0x060c }, /*                Arabic_comma ، ARABIC COMMA */
    CodePair { keysym: 0x05bb, ucs: 0x061b }, /*            Arabic_semicolon ؛ ARABIC SEMICOLON */
    CodePair { keysym: 0x05bf, ucs: 0x061f }, /*        Arabic_question_mark ؟ ARABIC QUESTION MARK */
    CodePair { keysym: 0x05c1, ucs: 0x0621 }, /*                Arabic_hamza ء ARABIC LETTER HAMZA */
    CodePair { keysym: 0x05c2, ucs: 0x0622 }, /*          Arabic_maddaonalef آ ARABIC LETTER ALEF WITH MADDA ABOVE */
    CodePair { keysym: 0x05c3, ucs: 0x0623 }, /*          Arabic_hamzaonalef أ ARABIC LETTER ALEF WITH HAMZA ABOVE */
    CodePair { keysym: 0x05c4, ucs: 0x0624 }, /*           Arabic_hamzaonwaw ؤ ARABIC LETTER WAW WITH HAMZA ABOVE */
    CodePair { keysym: 0x05c5, ucs: 0x0625 }, /*       Arabic_hamzaunderalef إ ARABIC LETTER ALEF WITH HAMZA BELOW */
    CodePair { keysym: 0x05c6, ucs: 0x0626 }, /*           Arabic_hamzaonyeh ئ ARABIC LETTER YEH WITH HAMZA ABOVE */
    CodePair { keysym: 0x05c7, ucs: 0x0627 }, /*                 Arabic_alef ا ARABIC LETTER ALEF */
    CodePair { keysym: 0x05c8, ucs: 0x0628 }, /*                  Arabic_beh ب ARABIC LETTER BEH */
    CodePair { keysym: 0x05c9, ucs: 0x0629 }, /*           Arabic_tehmarbuta ة ARABIC LETTER TEH MARBUTA */
    CodePair { keysym: 0x05ca, ucs: 0x062a }, /*                  Arabic_teh ت ARABIC LETTER TEH */
    CodePair { keysym: 0x05cb, ucs: 0x062b }, /*                 Arabic_theh ث ARABIC LETTER THEH */
    CodePair { keysym: 0x05cc, ucs: 0x062c }, /*                 Arabic_jeem ج ARABIC LETTER JEEM */
    CodePair { keysym: 0x05cd, ucs: 0x062d }, /*                  Arabic_hah ح ARABIC LETTER HAH */
    CodePair { keysym: 0x05ce, ucs: 0x062e }, /*                 Arabic_khah خ ARABIC LETTER KHAH */
    CodePair { keysym: 0x05cf, ucs: 0x062f }, /*                  Arabic_dal د ARABIC LETTER DAL */
    CodePair { keysym: 0x05d0, ucs: 0x0630 }, /*                 Arabic_thal ذ ARABIC LETTER THAL */
    CodePair { keysym: 0x05d1, ucs: 0x0631 }, /*                   Arabic_ra ر ARABIC LETTER REH */
    CodePair { keysym: 0x05d2, ucs: 0x0632 }, /*                 Arabic_zain ز ARABIC LETTER ZAIN */
    CodePair { keysym: 0x05d3, ucs: 0x0633 }, /*                 Arabic_seen س ARABIC LETTER SEEN */
    CodePair { keysym: 0x05d4, ucs: 0x0634 }, /*                Arabic_sheen ش ARABIC LETTER SHEEN */
    CodePair { keysym: 0x05d5, ucs: 0x0635 }, /*                  Arabic_sad ص ARABIC LETTER SAD */
    CodePair { keysym: 0x05d6, ucs: 0x0636 }, /*                  Arabic_dad ض ARABIC LETTER DAD */
    CodePair { keysym: 0x05d7, ucs: 0x0637 }, /*                  Arabic_tah ط ARABIC LETTER TAH */
    CodePair { keysym: 0x05d8, ucs: 0x0638 }, /*                  Arabic_zah ظ ARABIC LETTER ZAH */
    CodePair { keysym: 0x05d9, ucs: 0x0639 }, /*                  Arabic_ain ع ARABIC LETTER AIN */
    CodePair { keysym: 0x05da, ucs: 0x063a }, /*                Arabic_ghain غ ARABIC LETTER GHAIN */
    CodePair { keysym: 0x05e0, ucs: 0x0640 }, /*              Arabic_tatweel ـ ARABIC TATWEEL */
    CodePair { keysym: 0x05e1, ucs: 0x0641 }, /*                  Arabic_feh ف ARABIC LETTER FEH */
    CodePair { keysym: 0x05e2, ucs: 0x0642 }, /*                  Arabic_qaf ق ARABIC LETTER QAF */
    CodePair { keysym: 0x05e3, ucs: 0x0643 }, /*                  Arabic_kaf ك ARABIC LETTER KAF */
    CodePair { keysym: 0x05e4, ucs: 0x0644 }, /*                  Arabic_lam ل ARABIC LETTER LAM */
    CodePair { keysym: 0x05e5, ucs: 0x0645 }, /*                 Arabic_meem م ARABIC LETTER MEEM */
    CodePair { keysym: 0x05e6, ucs: 0x0646 }, /*                 Arabic_noon ن ARABIC LETTER NOON */
    CodePair { keysym: 0x05e7, ucs: 0x0647 }, /*                   Arabic_ha ه ARABIC LETTER HEH */
    CodePair { keysym: 0x05e8, ucs: 0x0648 }, /*                  Arabic_waw و ARABIC LETTER WAW */
    CodePair { keysym: 0x05e9, ucs: 0x0649 }, /*          Arabic_alefmaksura ى ARABIC LETTER ALEF MAKSURA */
    CodePair { keysym: 0x05ea, ucs: 0x064a }, /*                  Arabic_yeh ي ARABIC LETTER YEH */
    CodePair { keysym: 0x05eb, ucs: 0x064b }, /*             Arabic_fathatan ً ARABIC FATHATAN */
    CodePair { keysym: 0x05ec, ucs: 0x064c }, /*             Arabic_dammatan ٌ ARABIC DAMMATAN */
    CodePair { keysym: 0x05ed, ucs: 0x064d }, /*             Arabic_kasratan ٍ ARABIC KASRATAN */
    CodePair { keysym: 0x05ee, ucs: 0x064e }, /*                Arabic_fatha َ ARABIC FATHA */
    CodePair { keysym: 0x05ef, ucs: 0x064f }, /*                Arabic_damma ُ ARABIC DAMMA */
    CodePair { keysym: 0x05f0, ucs: 0x0650 }, /*                Arabic_kasra ِ ARABIC KASRA */
    CodePair { keysym: 0x05f1, ucs: 0x0651 }, /*               Arabic_shadda ّ ARABIC SHADDA */
    CodePair { keysym: 0x05f2, ucs: 0x0652 }, /*                Arabic_sukun ْ ARABIC SUKUN */
    CodePair { keysym: 0x06a1, ucs: 0x0452 }, /*                 Serbian_dje ђ CYRILLIC SMALL LETTER DJE */
    CodePair { keysym: 0x06a2, ucs: 0x0453 }, /*               Macedonia_gje ѓ CYRILLIC SMALL LETTER GJE */
    CodePair { keysym: 0x06a3, ucs: 0x0451 }, /*                 Cyrillic_io ё CYRILLIC SMALL LETTER IO */
    CodePair { keysym: 0x06a4, ucs: 0x0454 }, /*                Ukrainian_ie є CYRILLIC SMALL LETTER UKRAINIAN IE */
    CodePair { keysym: 0x06a5, ucs: 0x0455 }, /*               Macedonia_dse ѕ CYRILLIC SMALL LETTER DZE */
    CodePair { keysym: 0x06a6, ucs: 0x0456 }, /*                 Ukrainian_i і CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I */
    CodePair { keysym: 0x06a7, ucs: 0x0457 }, /*                Ukrainian_yi ї CYRILLIC SMALL LETTER YI */
    CodePair { keysym: 0x06a8, ucs: 0x0458 }, /*                 Cyrillic_je ј CYRILLIC SMALL LETTER JE */
    CodePair { keysym: 0x06a9, ucs: 0x0459 }, /*                Cyrillic_lje љ CYRILLIC SMALL LETTER LJE */
    CodePair { keysym: 0x06aa, ucs: 0x045a }, /*                Cyrillic_nje њ CYRILLIC SMALL LETTER NJE */
    CodePair { keysym: 0x06ab, ucs: 0x045b }, /*                Serbian_tshe ћ CYRILLIC SMALL LETTER TSHE */
    CodePair { keysym: 0x06ac, ucs: 0x045c }, /*               Macedonia_kje ќ CYRILLIC SMALL LETTER KJE */
    CodePair { keysym: 0x06ad, ucs: 0x0491 }, /*   Ukrainian_ghe_with_upturn ґ CYRILLIC SMALL LETTER GHE WITH UPTURN */
    CodePair { keysym: 0x06ae, ucs: 0x045e }, /*         Byelorussian_shortu ў CYRILLIC SMALL LETTER SHORT U */
    CodePair { keysym: 0x06af, ucs: 0x045f }, /*               Cyrillic_dzhe џ CYRILLIC SMALL LETTER DZHE */
    CodePair { keysym: 0x06b0, ucs: 0x2116 }, /*                  numerosign № NUMERO SIGN */
    CodePair { keysym: 0x06b1, ucs: 0x0402 }, /*                 Serbian_DJE Ђ CYRILLIC CAPITAL LETTER DJE */
    CodePair { keysym: 0x06b2, ucs: 0x0403 }, /*               Macedonia_GJE Ѓ CYRILLIC CAPITAL LETTER GJE */
    CodePair { keysym: 0x06b3, ucs: 0x0401 }, /*                 Cyrillic_IO Ё CYRILLIC CAPITAL LETTER IO */
    CodePair { keysym: 0x06b4, ucs: 0x0404 }, /*                Ukrainian_IE Є CYRILLIC CAPITAL LETTER UKRAINIAN IE */
    CodePair { keysym: 0x06b5, ucs: 0x0405 }, /*               Macedonia_DSE Ѕ CYRILLIC CAPITAL LETTER DZE */
    CodePair { keysym: 0x06b6, ucs: 0x0406 }, /*                 Ukrainian_I І CYRILLIC CAPITAL LETTER BYELORUSSIAN-UKRAINIAN I */
    CodePair { keysym: 0x06b7, ucs: 0x0407 }, /*                Ukrainian_YI Ї CYRILLIC CAPITAL LETTER YI */
    CodePair { keysym: 0x06b8, ucs: 0x0408 }, /*                 Cyrillic_JE Ј CYRILLIC CAPITAL LETTER JE */
    CodePair { keysym: 0x06b9, ucs: 0x0409 }, /*                Cyrillic_LJE Љ CYRILLIC CAPITAL LETTER LJE */
    CodePair { keysym: 0x06ba, ucs: 0x040a }, /*                Cyrillic_NJE Њ CYRILLIC CAPITAL LETTER NJE */
    CodePair { keysym: 0x06bb, ucs: 0x040b }, /*                Serbian_TSHE Ћ CYRILLIC CAPITAL LETTER TSHE */
    CodePair { keysym: 0x06bc, ucs: 0x040c }, /*               Macedonia_KJE Ќ CYRILLIC CAPITAL LETTER KJE */
    CodePair { keysym: 0x06bd, ucs: 0x0490 }, /*   Ukrainian_GHE_WITH_UPTURN Ґ CYRILLIC CAPITAL LETTER GHE WITH UPTURN */
    CodePair { keysym: 0x06be, ucs: 0x040e }, /*         Byelorussian_SHORTU Ў CYRILLIC CAPITAL LETTER SHORT U */
    CodePair { keysym: 0x06bf, ucs: 0x040f }, /*               Cyrillic_DZHE Џ CYRILLIC CAPITAL LETTER DZHE */
    CodePair { keysym: 0x06c0, ucs: 0x044e }, /*                 Cyrillic_yu ю CYRILLIC SMALL LETTER YU */
    CodePair { keysym: 0x06c1, ucs: 0x0430 }, /*                  Cyrillic_a а CYRILLIC SMALL LETTER A */
    CodePair { keysym: 0x06c2, ucs: 0x0431 }, /*                 Cyrillic_be б CYRILLIC SMALL LETTER BE */
    CodePair { keysym: 0x06c3, ucs: 0x0446 }, /*                Cyrillic_tse ц CYRILLIC SMALL LETTER TSE */
    CodePair { keysym: 0x06c4, ucs: 0x0434 }, /*                 Cyrillic_de д CYRILLIC SMALL LETTER DE */
    CodePair { keysym: 0x06c5, ucs: 0x0435 }, /*                 Cyrillic_ie е CYRILLIC SMALL LETTER IE */
    CodePair { keysym: 0x06c6, ucs: 0x0444 }, /*                 Cyrillic_ef ф CYRILLIC SMALL LETTER EF */
    CodePair { keysym: 0x06c7, ucs: 0x0433 }, /*                Cyrillic_ghe г CYRILLIC SMALL LETTER GHE */
    CodePair { keysym: 0x06c8, ucs: 0x0445 }, /*                 Cyrillic_ha х CYRILLIC SMALL LETTER HA */
    CodePair { keysym: 0x06c9, ucs: 0x0438 }, /*                  Cyrillic_i и CYRILLIC SMALL LETTER I */
    CodePair { keysym: 0x06ca, ucs: 0x0439 }, /*             Cyrillic_shorti й CYRILLIC SMALL LETTER SHORT I */
    CodePair { keysym: 0x06cb, ucs: 0x043a }, /*                 Cyrillic_ka к CYRILLIC SMALL LETTER KA */
    CodePair { keysym: 0x06cc, ucs: 0x043b }, /*                 Cyrillic_el л CYRILLIC SMALL LETTER EL */
    CodePair { keysym: 0x06cd, ucs: 0x043c }, /*                 Cyrillic_em м CYRILLIC SMALL LETTER EM */
    CodePair { keysym: 0x06ce, ucs: 0x043d }, /*                 Cyrillic_en н CYRILLIC SMALL LETTER EN */
    CodePair { keysym: 0x06cf, ucs: 0x043e }, /*                  Cyrillic_o о CYRILLIC SMALL LETTER O */
    CodePair { keysym: 0x06d0, ucs: 0x043f }, /*                 Cyrillic_pe п CYRILLIC SMALL LETTER PE */
    CodePair { keysym: 0x06d1, ucs: 0x044f }, /*                 Cyrillic_ya я CYRILLIC SMALL LETTER YA */
    CodePair { keysym: 0x06d2, ucs: 0x0440 }, /*                 Cyrillic_er р CYRILLIC SMALL LETTER ER */
    CodePair { keysym: 0x06d3, ucs: 0x0441 }, /*                 Cyrillic_es с CYRILLIC SMALL LETTER ES */
    CodePair { keysym: 0x06d4, ucs: 0x0442 }, /*                 Cyrillic_te т CYRILLIC SMALL LETTER TE */
    CodePair { keysym: 0x06d5, ucs: 0x0443 }, /*                  Cyrillic_u у CYRILLIC SMALL LETTER U */
    CodePair { keysym: 0x06d6, ucs: 0x0436 }, /*                Cyrillic_zhe ж CYRILLIC SMALL LETTER ZHE */
    CodePair { keysym: 0x06d7, ucs: 0x0432 }, /*                 Cyrillic_ve в CYRILLIC SMALL LETTER VE */
    CodePair { keysym: 0x06d8, ucs: 0x044c }, /*           Cyrillic_softsign ь CYRILLIC SMALL LETTER SOFT SIGN */
    CodePair { keysym: 0x06d9, ucs: 0x044b }, /*               Cyrillic_yeru ы CYRILLIC SMALL LETTER YERU */
    CodePair { keysym: 0x06da, ucs: 0x0437 }, /*                 Cyrillic_ze з CYRILLIC SMALL LETTER ZE */
    CodePair { keysym: 0x06db, ucs: 0x0448 }, /*                Cyrillic_sha ш CYRILLIC SMALL LETTER SHA */
    CodePair { keysym: 0x06dc, ucs: 0x044d }, /*                  Cyrillic_e э CYRILLIC SMALL LETTER E */
    CodePair { keysym: 0x06dd, ucs: 0x0449 }, /*              Cyrillic_shcha щ CYRILLIC SMALL LETTER SHCHA */
    CodePair { keysym: 0x06de, ucs: 0x0447 }, /*                Cyrillic_che ч CYRILLIC SMALL LETTER CHE */
    CodePair { keysym: 0x06df, ucs: 0x044a }, /*           Cyrillic_hardsign ъ CYRILLIC SMALL LETTER HARD SIGN */
    CodePair { keysym: 0x06e0, ucs: 0x042e }, /*                 Cyrillic_YU Ю CYRILLIC CAPITAL LETTER YU */
    CodePair { keysym: 0x06e1, ucs: 0x0410 }, /*                  Cyrillic_A А CYRILLIC CAPITAL LETTER A */
    CodePair { keysym: 0x06e2, ucs: 0x0411 }, /*                 Cyrillic_BE Б CYRILLIC CAPITAL LETTER BE */
    CodePair { keysym: 0x06e3, ucs: 0x0426 }, /*                Cyrillic_TSE Ц CYRILLIC CAPITAL LETTER TSE */
    CodePair { keysym: 0x06e4, ucs: 0x0414 }, /*                 Cyrillic_DE Д CYRILLIC CAPITAL LETTER DE */
    CodePair { keysym: 0x06e5, ucs: 0x0415 }, /*                 Cyrillic_IE Е CYRILLIC CAPITAL LETTER IE */
    CodePair { keysym: 0x06e6, ucs: 0x0424 }, /*                 Cyrillic_EF Ф CYRILLIC CAPITAL LETTER EF */
    CodePair { keysym: 0x06e7, ucs: 0x0413 }, /*                Cyrillic_GHE Г CYRILLIC CAPITAL LETTER GHE */
    CodePair { keysym: 0x06e8, ucs: 0x0425 }, /*                 Cyrillic_HA Х CYRILLIC CAPITAL LETTER HA */
    CodePair { keysym: 0x06e9, ucs: 0x0418 }, /*                  Cyrillic_I И CYRILLIC CAPITAL LETTER I */
    CodePair { keysym: 0x06ea, ucs: 0x0419 }, /*             Cyrillic_SHORTI Й CYRILLIC CAPITAL LETTER SHORT I */
    CodePair { keysym: 0x06eb, ucs: 0x041a }, /*                 Cyrillic_KA К CYRILLIC CAPITAL LETTER KA */
    CodePair { keysym: 0x06ec, ucs: 0x041b }, /*                 Cyrillic_EL Л CYRILLIC CAPITAL LETTER EL */
    CodePair { keysym: 0x06ed, ucs: 0x041c }, /*                 Cyrillic_EM М CYRILLIC CAPITAL LETTER EM */
    CodePair { keysym: 0x06ee, ucs: 0x041d }, /*                 Cyrillic_EN Н CYRILLIC CAPITAL LETTER EN */
    CodePair { keysym: 0x06ef, ucs: 0x041e }, /*                  Cyrillic_O О CYRILLIC CAPITAL LETTER O */
    CodePair { keysym: 0x06f0, ucs: 0x041f }, /*                 Cyrillic_PE П CYRILLIC CAPITAL LETTER PE */
    CodePair { keysym: 0x06f1, ucs: 0x042f }, /*                 Cyrillic_YA Я CYRILLIC CAPITAL LETTER YA */
    CodePair { keysym: 0x06f2, ucs: 0x0420 }, /*                 Cyrillic_ER Р CYRILLIC CAPITAL LETTER ER */
    CodePair { keysym: 0x06f3, ucs: 0x0421 }, /*                 Cyrillic_ES С CYRILLIC CAPITAL LETTER ES */
    CodePair { keysym: 0x06f4, ucs: 0x0422 }, /*                 Cyrillic_TE Т CYRILLIC CAPITAL LETTER TE */
    CodePair { keysym: 0x06f5, ucs: 0x0423 }, /*                  Cyrillic_U У CYRILLIC CAPITAL LETTER U */
    CodePair { keysym: 0x06f6, ucs: 0x0416 }, /*                Cyrillic_ZHE Ж CYRILLIC CAPITAL LETTER ZHE */
    CodePair { keysym: 0x06f7, ucs: 0x0412 }, /*                 Cyrillic_VE В CYRILLIC CAPITAL LETTER VE */
    CodePair { keysym: 0x06f8, ucs: 0x042c }, /*           Cyrillic_SOFTSIGN Ь CYRILLIC CAPITAL LETTER SOFT SIGN */
    CodePair { keysym: 0x06f9, ucs: 0x042b }, /*               Cyrillic_YERU Ы CYRILLIC CAPITAL LETTER YERU */
    CodePair { keysym: 0x06fa, ucs: 0x0417 }, /*                 Cyrillic_ZE З CYRILLIC CAPITAL LETTER ZE */
    CodePair { keysym: 0x06fb, ucs: 0x0428 }, /*                Cyrillic_SHA Ш CYRILLIC CAPITAL LETTER SHA */
    CodePair { keysym: 0x06fc, ucs: 0x042d }, /*                  Cyrillic_E Э CYRILLIC CAPITAL LETTER E */
    CodePair { keysym: 0x06fd, ucs: 0x0429 }, /*              Cyrillic_SHCHA Щ CYRILLIC CAPITAL LETTER SHCHA */
    CodePair { keysym: 0x06fe, ucs: 0x0427 }, /*                Cyrillic_CHE Ч CYRILLIC CAPITAL LETTER CHE */
    CodePair { keysym: 0x06ff, ucs: 0x042a }, /*           Cyrillic_HARDSIGN Ъ CYRILLIC CAPITAL LETTER HARD SIGN */
    CodePair { keysym: 0x07a1, ucs: 0x0386 }, /*           Greek_ALPHAaccent Ά GREEK CAPITAL LETTER ALPHA WITH TONOS */
    CodePair { keysym: 0x07a2, ucs: 0x0388 }, /*         Greek_EPSILONaccent Έ GREEK CAPITAL LETTER EPSILON WITH TONOS */
    CodePair { keysym: 0x07a3, ucs: 0x0389 }, /*             Greek_ETAaccent Ή GREEK CAPITAL LETTER ETA WITH TONOS */
    CodePair { keysym: 0x07a4, ucs: 0x038a }, /*            Greek_IOTAaccent Ί GREEK CAPITAL LETTER IOTA WITH TONOS */
    CodePair { keysym: 0x07a5, ucs: 0x03aa }, /*         Greek_IOTAdiaeresis Ϊ GREEK CAPITAL LETTER IOTA WITH DIALYTIKA */
    CodePair { keysym: 0x07a7, ucs: 0x038c }, /*         Greek_OMICRONaccent Ό GREEK CAPITAL LETTER OMICRON WITH TONOS */
    CodePair { keysym: 0x07a8, ucs: 0x038e }, /*         Greek_UPSILONaccent Ύ GREEK CAPITAL LETTER UPSILON WITH TONOS */
    CodePair { keysym: 0x07a9, ucs: 0x03ab }, /*       Greek_UPSILONdieresis Ϋ GREEK CAPITAL LETTER UPSILON WITH DIALYTIKA */
    CodePair { keysym: 0x07ab, ucs: 0x038f }, /*           Greek_OMEGAaccent Ώ GREEK CAPITAL LETTER OMEGA WITH TONOS */
    CodePair { keysym: 0x07ae, ucs: 0x0385 }, /*        Greek_accentdieresis ΅ GREEK DIALYTIKA TONOS */
    CodePair { keysym: 0x07af, ucs: 0x2015 }, /*              Greek_horizbar ― HORIZONTAL BAR */
    CodePair { keysym: 0x07b1, ucs: 0x03ac }, /*           Greek_alphaaccent ά GREEK SMALL LETTER ALPHA WITH TONOS */
    CodePair { keysym: 0x07b2, ucs: 0x03ad }, /*         Greek_epsilonaccent έ GREEK SMALL LETTER EPSILON WITH TONOS */
    CodePair { keysym: 0x07b3, ucs: 0x03ae }, /*             Greek_etaaccent ή GREEK SMALL LETTER ETA WITH TONOS */
    CodePair { keysym: 0x07b4, ucs: 0x03af }, /*            Greek_iotaaccent ί GREEK SMALL LETTER IOTA WITH TONOS */
    CodePair { keysym: 0x07b5, ucs: 0x03ca }, /*          Greek_iotadieresis ϊ GREEK SMALL LETTER IOTA WITH DIALYTIKA */
    CodePair { keysym: 0x07b6, ucs: 0x0390 }, /*    Greek_iotaaccentdieresis ΐ GREEK SMALL LETTER IOTA WITH DIALYTIKA AND TONOS */
    CodePair { keysym: 0x07b7, ucs: 0x03cc }, /*         Greek_omicronaccent ό GREEK SMALL LETTER OMICRON WITH TONOS */
    CodePair { keysym: 0x07b8, ucs: 0x03cd }, /*         Greek_upsilonaccent ύ GREEK SMALL LETTER UPSILON WITH TONOS */
    CodePair { keysym: 0x07b9, ucs: 0x03cb }, /*       Greek_upsilondieresis ϋ GREEK SMALL LETTER UPSILON WITH DIALYTIKA */
    CodePair { keysym: 0x07ba, ucs: 0x03b0 }, /* Greek_upsilonaccentdieresis ΰ GREEK SMALL LETTER UPSILON WITH DIALYTIKA AND TONOS */
    CodePair { keysym: 0x07bb, ucs: 0x03ce }, /*           Greek_omegaaccent ώ GREEK SMALL LETTER OMEGA WITH TONOS */
    CodePair { keysym: 0x07c1, ucs: 0x0391 }, /*                 Greek_ALPHA Α GREEK CAPITAL LETTER ALPHA */
    CodePair { keysym: 0x07c2, ucs: 0x0392 }, /*                  Greek_BETA Β GREEK CAPITAL LETTER BETA */
    CodePair { keysym: 0x07c3, ucs: 0x0393 }, /*                 Greek_GAMMA Γ GREEK CAPITAL LETTER GAMMA */
    CodePair { keysym: 0x07c4, ucs: 0x0394 }, /*                 Greek_DELTA Δ GREEK CAPITAL LETTER DELTA */
    CodePair { keysym: 0x07c5, ucs: 0x0395 }, /*               Greek_EPSILON Ε GREEK CAPITAL LETTER EPSILON */
    CodePair { keysym: 0x07c6, ucs: 0x0396 }, /*                  Greek_ZETA Ζ GREEK CAPITAL LETTER ZETA */
    CodePair { keysym: 0x07c7, ucs: 0x0397 }, /*                   Greek_ETA Η GREEK CAPITAL LETTER ETA */
    CodePair { keysym: 0x07c8, ucs: 0x0398 }, /*                 Greek_THETA Θ GREEK CAPITAL LETTER THETA */
    CodePair { keysym: 0x07c9, ucs: 0x0399 }, /*                  Greek_IOTA Ι GREEK CAPITAL LETTER IOTA */
    CodePair { keysym: 0x07ca, ucs: 0x039a }, /*                 Greek_KAPPA Κ GREEK CAPITAL LETTER KAPPA */
    CodePair { keysym: 0x07cb, ucs: 0x039b }, /*                Greek_LAMBDA Λ GREEK CAPITAL LETTER LAMDA */
    CodePair { keysym: 0x07cc, ucs: 0x039c }, /*                    Greek_MU Μ GREEK CAPITAL LETTER MU */
    CodePair { keysym: 0x07cd, ucs: 0x039d }, /*                    Greek_NU Ν GREEK CAPITAL LETTER NU */
    CodePair { keysym: 0x07ce, ucs: 0x039e }, /*                    Greek_XI Ξ GREEK CAPITAL LETTER XI */
    CodePair { keysym: 0x07cf, ucs: 0x039f }, /*               Greek_OMICRON Ο GREEK CAPITAL LETTER OMICRON */
    CodePair { keysym: 0x07d0, ucs: 0x03a0 }, /*                    Greek_PI Π GREEK CAPITAL LETTER PI */
    CodePair { keysym: 0x07d1, ucs: 0x03a1 }, /*                   Greek_RHO Ρ GREEK CAPITAL LETTER RHO */
    CodePair { keysym: 0x07d2, ucs: 0x03a3 }, /*                 Greek_SIGMA Σ GREEK CAPITAL LETTER SIGMA */
    CodePair { keysym: 0x07d4, ucs: 0x03a4 }, /*                   Greek_TAU Τ GREEK CAPITAL LETTER TAU */
    CodePair { keysym: 0x07d5, ucs: 0x03a5 }, /*               Greek_UPSILON Υ GREEK CAPITAL LETTER UPSILON */
    CodePair { keysym: 0x07d6, ucs: 0x03a6 }, /*                   Greek_PHI Φ GREEK CAPITAL LETTER PHI */
    CodePair { keysym: 0x07d7, ucs: 0x03a7 }, /*                   Greek_CHI Χ GREEK CAPITAL LETTER CHI */
    CodePair { keysym: 0x07d8, ucs: 0x03a8 }, /*                   Greek_PSI Ψ GREEK CAPITAL LETTER PSI */
    CodePair { keysym: 0x07d9, ucs: 0x03a9 }, /*                 Greek_OMEGA Ω GREEK CAPITAL LETTER OMEGA */
    CodePair { keysym: 0x07e1, ucs: 0x03b1 }, /*                 Greek_alpha α GREEK SMALL LETTER ALPHA */
    CodePair { keysym: 0x07e2, ucs: 0x03b2 }, /*                  Greek_beta β GREEK SMALL LETTER BETA */
    CodePair { keysym: 0x07e3, ucs: 0x03b3 }, /*                 Greek_gamma γ GREEK SMALL LETTER GAMMA */
    CodePair { keysym: 0x07e4, ucs: 0x03b4 }, /*                 Greek_delta δ GREEK SMALL LETTER DELTA */
    CodePair { keysym: 0x07e5, ucs: 0x03b5 }, /*               Greek_epsilon ε GREEK SMALL LETTER EPSILON */
    CodePair { keysym: 0x07e6, ucs: 0x03b6 }, /*                  Greek_zeta ζ GREEK SMALL LETTER ZETA */
    CodePair { keysym: 0x07e7, ucs: 0x03b7 }, /*                   Greek_eta η GREEK SMALL LETTER ETA */
    CodePair { keysym: 0x07e8, ucs: 0x03b8 }, /*                 Greek_theta θ GREEK SMALL LETTER THETA */
    CodePair { keysym: 0x07e9, ucs: 0x03b9 }, /*                  Greek_iota ι GREEK SMALL LETTER IOTA */
    CodePair { keysym: 0x07ea, ucs: 0x03ba }, /*                 Greek_kappa κ GREEK SMALL LETTER KAPPA */
    CodePair { keysym: 0x07eb, ucs: 0x03bb }, /*                Greek_lambda λ GREEK SMALL LETTER LAMDA */
    CodePair { keysym: 0x07ec, ucs: 0x03bc }, /*                    Greek_mu μ GREEK SMALL LETTER MU */
    CodePair { keysym: 0x07ed, ucs: 0x03bd }, /*                    Greek_nu ν GREEK SMALL LETTER NU */
    CodePair { keysym: 0x07ee, ucs: 0x03be }, /*                    Greek_xi ξ GREEK SMALL LETTER XI */
    CodePair { keysym: 0x07ef, ucs: 0x03bf }, /*               Greek_omicron ο GREEK SMALL LETTER OMICRON */
    CodePair { keysym: 0x07f0, ucs: 0x03c0 }, /*                    Greek_pi π GREEK SMALL LETTER PI */
    CodePair { keysym: 0x07f1, ucs: 0x03c1 }, /*                   Greek_rho ρ GREEK SMALL LETTER RHO */
    CodePair { keysym: 0x07f2, ucs: 0x03c3 }, /*                 Greek_sigma σ GREEK SMALL LETTER SIGMA */
    CodePair { keysym: 0x07f3, ucs: 0x03c2 }, /*       Greek_finalsmallsigma ς GREEK SMALL LETTER FINAL SIGMA */
    CodePair { keysym: 0x07f4, ucs: 0x03c4 }, /*                   Greek_tau τ GREEK SMALL LETTER TAU */
    CodePair { keysym: 0x07f5, ucs: 0x03c5 }, /*               Greek_upsilon υ GREEK SMALL LETTER UPSILON */
    CodePair { keysym: 0x07f6, ucs: 0x03c6 }, /*                   Greek_phi φ GREEK SMALL LETTER PHI */
    CodePair { keysym: 0x07f7, ucs: 0x03c7 }, /*                   Greek_chi χ GREEK SMALL LETTER CHI */
    CodePair { keysym: 0x07f8, ucs: 0x03c8 }, /*                   Greek_psi ψ GREEK SMALL LETTER PSI */
    CodePair { keysym: 0x07f9, ucs: 0x03c9 }, /*                 Greek_omega ω GREEK SMALL LETTER OMEGA */
    CodePair { keysym: 0x08a1, ucs: 0x23b7 }, /*                 leftradical ⎷ ??? */
    CodePair { keysym: 0x08a2, ucs: 0x250c }, /*              topleftradical ┌ BOX DRAWINGS LIGHT DOWN AND RIGHT */
    CodePair { keysym: 0x08a3, ucs: 0x2500 }, /*              horizconnector ─ BOX DRAWINGS LIGHT HORIZONTAL */
    CodePair { keysym: 0x08a4, ucs: 0x2320 }, /*                 topintegral ⌠ TOP HALF INTEGRAL */
    CodePair { keysym: 0x08a5, ucs: 0x2321 }, /*                 botintegral ⌡ BOTTOM HALF INTEGRAL */
    CodePair { keysym: 0x08a6, ucs: 0x2502 }, /*               vertconnector │ BOX DRAWINGS LIGHT VERTICAL */
    CodePair { keysym: 0x08a7, ucs: 0x23a1 }, /*            topleftsqbracket ⎡ ??? */
    CodePair { keysym: 0x08a8, ucs: 0x23a3 }, /*            botleftsqbracket ⎣ ??? */
    CodePair { keysym: 0x08a9, ucs: 0x23a4 }, /*           toprightsqbracket ⎤ ??? */
    CodePair { keysym: 0x08aa, ucs: 0x23a6 }, /*           botrightsqbracket ⎦ ??? */
    CodePair { keysym: 0x08ab, ucs: 0x239b }, /*               topleftparens ⎛ ??? */
    CodePair { keysym: 0x08ac, ucs: 0x239d }, /*               botleftparens ⎝ ??? */
    CodePair { keysym: 0x08ad, ucs: 0x239e }, /*              toprightparens ⎞ ??? */
    CodePair { keysym: 0x08ae, ucs: 0x23a0 }, /*              botrightparens ⎠ ??? */
    CodePair { keysym: 0x08af, ucs: 0x23a8 }, /*        leftmiddlecurlybrace ⎨ ??? */
    CodePair { keysym: 0x08b0, ucs: 0x23ac }, /*       rightmiddlecurlybrace ⎬ ??? */
    /*  0x08b1                                              topleftsummation ? ??? */
    /*  0x08b2                                              botleftsummation ? ??? */
    /*  0x08b3                                     topvertsummationconnector ? ??? */
    /*  0x08b4                                     botvertsummationconnector ? ??? */
    /*  0x08b5                                             toprightsummation ? ??? */
    /*  0x08b6                                             botrightsummation ? ??? */
    /*  0x08b7                                          rightmiddlesummation ? ??? */
    CodePair { keysym: 0x08bc, ucs: 0x2264 }, /*               lessthanequal ≤ LESS-THAN OR EQUAL TO */
    CodePair { keysym: 0x08bd, ucs: 0x2260 }, /*                    notequal ≠ NOT EQUAL TO */
    CodePair { keysym: 0x08be, ucs: 0x2265 }, /*            greaterthanequal ≥ GREATER-THAN OR EQUAL TO */
    CodePair { keysym: 0x08bf, ucs: 0x222b }, /*                    integral ∫ INTEGRAL */
    CodePair { keysym: 0x08c0, ucs: 0x2234 }, /*                   therefore ∴ THEREFORE */
    CodePair { keysym: 0x08c1, ucs: 0x221d }, /*                   variation ∝ PROPORTIONAL TO */
    CodePair { keysym: 0x08c2, ucs: 0x221e }, /*                    infinity ∞ INFINITY */
    CodePair { keysym: 0x08c5, ucs: 0x2207 }, /*                       nabla ∇ NABLA */
    CodePair { keysym: 0x08c8, ucs: 0x223c }, /*                 approximate ∼ TILDE OPERATOR */
    CodePair { keysym: 0x08c9, ucs: 0x2243 }, /*                similarequal ≃ ASYMPTOTICALLY EQUAL TO */
    CodePair { keysym: 0x08cd, ucs: 0x21d4 }, /*                    ifonlyif ⇔ LEFT RIGHT DOUBLE ARROW */
    CodePair { keysym: 0x08ce, ucs: 0x21d2 }, /*                     implies ⇒ RIGHTWARDS DOUBLE ARROW */
    CodePair { keysym: 0x08cf, ucs: 0x2261 }, /*                   identical ≡ IDENTICAL TO */
    CodePair { keysym: 0x08d6, ucs: 0x221a }, /*                     radical √ SQUARE ROOT */
    CodePair { keysym: 0x08da, ucs: 0x2282 }, /*                  includedin ⊂ SUBSET OF */
    CodePair { keysym: 0x08db, ucs: 0x2283 }, /*                    includes ⊃ SUPERSET OF */
    CodePair { keysym: 0x08dc, ucs: 0x2229 }, /*                intersection ∩ INTERSECTION */
    CodePair { keysym: 0x08dd, ucs: 0x222a }, /*                       union ∪ UNION */
    CodePair { keysym: 0x08de, ucs: 0x2227 }, /*                  logicaland ∧ LOGICAL AND */
    CodePair { keysym: 0x08df, ucs: 0x2228 }, /*                   logicalor ∨ LOGICAL OR */
    CodePair { keysym: 0x08ef, ucs: 0x2202 }, /*           partialderivative ∂ PARTIAL DIFFERENTIAL */
    CodePair { keysym: 0x08f6, ucs: 0x0192 }, /*                    function ƒ LATIN SMALL LETTER F WITH HOOK */
    CodePair { keysym: 0x08fb, ucs: 0x2190 }, /*                   leftarrow ← LEFTWARDS ARROW */
    CodePair { keysym: 0x08fc, ucs: 0x2191 }, /*                     uparrow ↑ UPWARDS ARROW */
    CodePair { keysym: 0x08fd, ucs: 0x2192 }, /*                  rightarrow → RIGHTWARDS ARROW */
    CodePair { keysym: 0x08fe, ucs: 0x2193 }, /*                   downarrow ↓ DOWNWARDS ARROW */
    /*  0x09df                                                         blank ? ??? */
    CodePair { keysym: 0x09e0, ucs: 0x25c6 }, /*                soliddiamond ◆ BLACK DIAMOND */
    CodePair { keysym: 0x09e1, ucs: 0x2592 }, /*                checkerboard ▒ MEDIUM SHADE */
    CodePair { keysym: 0x09e2, ucs: 0x2409 }, /*                          ht ␉ SYMBOL FOR HORIZONTAL TABULATION */
    CodePair { keysym: 0x09e3, ucs: 0x240c }, /*                          ff ␌ SYMBOL FOR FORM FEED */
    CodePair { keysym: 0x09e4, ucs: 0x240d }, /*                          cr ␍ SYMBOL FOR CARRIAGE RETURN */
    CodePair { keysym: 0x09e5, ucs: 0x240a }, /*                          lf ␊ SYMBOL FOR LINE FEED */
    CodePair { keysym: 0x09e8, ucs: 0x2424 }, /*                          nl ␤ SYMBOL FOR NEWLINE */
    CodePair { keysym: 0x09e9, ucs: 0x240b }, /*                          vt ␋ SYMBOL FOR VERTICAL TABULATION */
    CodePair { keysym: 0x09ea, ucs: 0x2518 }, /*              lowrightcorner ┘ BOX DRAWINGS LIGHT UP AND LEFT */
    CodePair { keysym: 0x09eb, ucs: 0x2510 }, /*               uprightcorner ┐ BOX DRAWINGS LIGHT DOWN AND LEFT */
    CodePair { keysym: 0x09ec, ucs: 0x250c }, /*                upleftcorner ┌ BOX DRAWINGS LIGHT DOWN AND RIGHT */
    CodePair { keysym: 0x09ed, ucs: 0x2514 }, /*               lowleftcorner └ BOX DRAWINGS LIGHT UP AND RIGHT */
    CodePair { keysym: 0x09ee, ucs: 0x253c }, /*               crossinglines ┼ BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL */
    CodePair { keysym: 0x09ef, ucs: 0x23ba }, /*              horizlinescan1 ⎺ HORIZONTAL SCAN LINE-1 (Unicode 3.2 draft) */
    CodePair { keysym: 0x09f0, ucs: 0x23bb }, /*              horizlinescan3 ⎻ HORIZONTAL SCAN LINE-3 (Unicode 3.2 draft) */
    CodePair { keysym: 0x09f1, ucs: 0x2500 }, /*              horizlinescan5 ─ BOX DRAWINGS LIGHT HORIZONTAL */
    CodePair { keysym: 0x09f2, ucs: 0x23bc }, /*              horizlinescan7 ⎼ HORIZONTAL SCAN LINE-7 (Unicode 3.2 draft) */
    CodePair { keysym: 0x09f3, ucs: 0x23bd }, /*              horizlinescan9 ⎽ HORIZONTAL SCAN LINE-9 (Unicode 3.2 draft) */
    CodePair { keysym: 0x09f4, ucs: 0x251c }, /*                       leftt ├ BOX DRAWINGS LIGHT VERTICAL AND RIGHT */
    CodePair { keysym: 0x09f5, ucs: 0x2524 }, /*                      rightt ┤ BOX DRAWINGS LIGHT VERTICAL AND LEFT */
    CodePair { keysym: 0x09f6, ucs: 0x2534 }, /*                        bott ┴ BOX DRAWINGS LIGHT UP AND HORIZONTAL */
    CodePair { keysym: 0x09f7, ucs: 0x252c }, /*                        topt ┬ BOX DRAWINGS LIGHT DOWN AND HORIZONTAL */
    CodePair { keysym: 0x09f8, ucs: 0x2502 }, /*                     vertbar │ BOX DRAWINGS LIGHT VERTICAL */
    CodePair { keysym: 0x0aa1, ucs: 0x2003 }, /*                     emspace   EM SPACE */
    CodePair { keysym: 0x0aa2, ucs: 0x2002 }, /*                     enspace   EN SPACE */
    CodePair { keysym: 0x0aa3, ucs: 0x2004 }, /*                    em3space   THREE-PER-EM SPACE */
    CodePair { keysym: 0x0aa4, ucs: 0x2005 }, /*                    em4space   FOUR-PER-EM SPACE */
    CodePair { keysym: 0x0aa5, ucs: 0x2007 }, /*                  digitspace   FIGURE SPACE */
    CodePair { keysym: 0x0aa6, ucs: 0x2008 }, /*                  punctspace   PUNCTUATION SPACE */
    CodePair { keysym: 0x0aa7, ucs: 0x2009 }, /*                   thinspace   THIN SPACE */
    CodePair { keysym: 0x0aa8, ucs: 0x200a }, /*                   hairspace   HAIR SPACE */
    CodePair { keysym: 0x0aa9, ucs: 0x2014 }, /*                      emdash — EM DASH */
    CodePair { keysym: 0x0aaa, ucs: 0x2013 }, /*                      endash – EN DASH */
    CodePair { keysym: 0x0aac, ucs: 0x2423 }, /*                 signifblank ␣ OPEN BOX */
    CodePair { keysym: 0x0aae, ucs: 0x2026 }, /*                    ellipsis … HORIZONTAL ELLIPSIS */
    CodePair { keysym: 0x0aaf, ucs: 0x2025 }, /*             doubbaselinedot ‥ TWO DOT LEADER */
    CodePair { keysym: 0x0ab0, ucs: 0x2153 }, /*                    onethird ⅓ VULGAR FRACTION ONE THIRD */
    CodePair { keysym: 0x0ab1, ucs: 0x2154 }, /*                   twothirds ⅔ VULGAR FRACTION TWO THIRDS */
    CodePair { keysym: 0x0ab2, ucs: 0x2155 }, /*                    onefifth ⅕ VULGAR FRACTION ONE FIFTH */
    CodePair { keysym: 0x0ab3, ucs: 0x2156 }, /*                   twofifths ⅖ VULGAR FRACTION TWO FIFTHS */
    CodePair { keysym: 0x0ab4, ucs: 0x2157 }, /*                 threefifths ⅗ VULGAR FRACTION THREE FIFTHS */
    CodePair { keysym: 0x0ab5, ucs: 0x2158 }, /*                  fourfifths ⅘ VULGAR FRACTION FOUR FIFTHS */
    CodePair { keysym: 0x0ab6, ucs: 0x2159 }, /*                    onesixth ⅙ VULGAR FRACTION ONE SIXTH */
    CodePair { keysym: 0x0ab7, ucs: 0x215a }, /*                  fivesixths ⅚ VULGAR FRACTION FIVE SIXTHS */
    CodePair { keysym: 0x0ab8, ucs: 0x2105 }, /*                      careof ℅ CARE OF */
    CodePair { keysym: 0x0abb, ucs: 0x2012 }, /*                     figdash ‒ FIGURE DASH */
    CodePair { keysym: 0x0abc, ucs: 0x27e8 }, /*            leftanglebracket ⟨ MATHEMATICAL LEFT ANGLE BRACKET */
    CodePair { keysym: 0x0abd, ucs: 0x002e }, /*                decimalpoint . FULL STOP */
    CodePair { keysym: 0x0abe, ucs: 0x27e9 }, /*           rightanglebracket ⟩ MATHEMATICAL RIGHT ANGLE BRACKET */
    /*  0x0abf                                                        marker ? ??? */
    CodePair { keysym: 0x0ac3, ucs: 0x215b }, /*                   oneeighth ⅛ VULGAR FRACTION ONE EIGHTH */
    CodePair { keysym: 0x0ac4, ucs: 0x215c }, /*                threeeighths ⅜ VULGAR FRACTION THREE EIGHTHS */
    CodePair { keysym: 0x0ac5, ucs: 0x215d }, /*                 fiveeighths ⅝ VULGAR FRACTION FIVE EIGHTHS */
    CodePair { keysym: 0x0ac6, ucs: 0x215e }, /*                seveneighths ⅞ VULGAR FRACTION SEVEN EIGHTHS */
    CodePair { keysym: 0x0ac9, ucs: 0x2122 }, /*                   trademark ™ TRADE MARK SIGN */
    CodePair { keysym: 0x0aca, ucs: 0x2613 }, /*               signaturemark ☓ SALTIRE */
    /*  0x0acb                                             trademarkincircle ? ??? */
    CodePair { keysym: 0x0acc, ucs: 0x25c1 }, /*            leftopentriangle ◁ WHITE LEFT-POINTING TRIANGLE */
    CodePair { keysym: 0x0acd, ucs: 0x25b7 }, /*           rightopentriangle ▷ WHITE RIGHT-POINTING TRIANGLE */
    CodePair { keysym: 0x0ace, ucs: 0x25cb }, /*                emopencircle ○ WHITE CIRCLE */
    CodePair { keysym: 0x0acf, ucs: 0x25af }, /*             emopenrectangle ▯ WHITE VERTICAL RECTANGLE */
    CodePair { keysym: 0x0ad0, ucs: 0x2018 }, /*         leftsinglequotemark ‘ LEFT SINGLE QUOTATION MARK */
    CodePair { keysym: 0x0ad1, ucs: 0x2019 }, /*        rightsinglequotemark ’ RIGHT SINGLE QUOTATION MARK */
    CodePair { keysym: 0x0ad2, ucs: 0x201c }, /*         leftdoublequotemark “ LEFT DOUBLE QUOTATION MARK */
    CodePair { keysym: 0x0ad3, ucs: 0x201d }, /*        rightdoublequotemark ” RIGHT DOUBLE QUOTATION MARK */
    CodePair { keysym: 0x0ad4, ucs: 0x211e }, /*                prescription ℞ PRESCRIPTION TAKE */
    CodePair { keysym: 0x0ad5, ucs: 0x2030 }, /*                    permille ‰ PER MILLE SIGN */
    CodePair { keysym: 0x0ad6, ucs: 0x2032 }, /*                     minutes ′ PRIME */
    CodePair { keysym: 0x0ad7, ucs: 0x2033 }, /*                     seconds ″ DOUBLE PRIME */
    CodePair { keysym: 0x0ad9, ucs: 0x271d }, /*                  latincross ✝ LATIN CROSS */
    /* 0x0ada                                                       hexagram ? ??? */
    CodePair { keysym: 0x0adb, ucs: 0x25ac }, /*            filledrectbullet ▬ BLACK RECTANGLE */
    CodePair { keysym: 0x0adc, ucs: 0x25c0 }, /*         filledlefttribullet ◀ BLACK LEFT-POINTING TRIANGLE */
    CodePair { keysym: 0x0add, ucs: 0x25b6 }, /*        filledrighttribullet ▶ BLACK RIGHT-POINTING TRIANGLE */
    CodePair { keysym: 0x0ade, ucs: 0x25cf }, /*              emfilledcircle ● BLACK CIRCLE */
    CodePair { keysym: 0x0adf, ucs: 0x25ae }, /*                emfilledrect ▮ BLACK VERTICAL RECTANGLE */
    CodePair { keysym: 0x0ae0, ucs: 0x25e6 }, /*            enopencircbullet ◦ WHITE BULLET */
    CodePair { keysym: 0x0ae1, ucs: 0x25ab }, /*          enopensquarebullet ▫ WHITE SMALL SQUARE */
    CodePair { keysym: 0x0ae2, ucs: 0x25ad }, /*              openrectbullet ▭ WHITE RECTANGLE */
    CodePair { keysym: 0x0ae3, ucs: 0x25b3 }, /*             opentribulletup △ WHITE UP-POINTING TRIANGLE */
    CodePair { keysym: 0x0ae4, ucs: 0x25bd }, /*           opentribulletdown ▽ WHITE DOWN-POINTING TRIANGLE */
    CodePair { keysym: 0x0ae5, ucs: 0x2606 }, /*                    openstar ☆ WHITE STAR */
    CodePair { keysym: 0x0ae6, ucs: 0x2022 }, /*          enfilledcircbullet • BULLET */
    CodePair { keysym: 0x0ae7, ucs: 0x25aa }, /*            enfilledsqbullet ▪ BLACK SMALL SQUARE */
    CodePair { keysym: 0x0ae8, ucs: 0x25b2 }, /*           filledtribulletup ▲ BLACK UP-POINTING TRIANGLE */
    CodePair { keysym: 0x0ae9, ucs: 0x25bc }, /*         filledtribulletdown ▼ BLACK DOWN-POINTING TRIANGLE */
    CodePair { keysym: 0x0aea, ucs: 0x261c }, /*                 leftpointer ☜ WHITE LEFT POINTING INDEX */
    CodePair { keysym: 0x0aeb, ucs: 0x261e }, /*                rightpointer ☞ WHITE RIGHT POINTING INDEX */
    CodePair { keysym: 0x0aec, ucs: 0x2663 }, /*                        club ♣ BLACK CLUB SUIT */
    CodePair { keysym: 0x0aed, ucs: 0x2666 }, /*                     diamond ♦ BLACK DIAMOND SUIT */
    CodePair { keysym: 0x0aee, ucs: 0x2665 }, /*                       heart ♥ BLACK HEART SUIT */
    CodePair { keysym: 0x0af0, ucs: 0x2720 }, /*                maltesecross ✠ MALTESE CROSS */
    CodePair { keysym: 0x0af1, ucs: 0x2020 }, /*                      dagger † DAGGER */
    CodePair { keysym: 0x0af2, ucs: 0x2021 }, /*                doubledagger ‡ DOUBLE DAGGER */
    CodePair { keysym: 0x0af3, ucs: 0x2713 }, /*                   checkmark ✓ CHECK MARK */
    CodePair { keysym: 0x0af4, ucs: 0x2717 }, /*                 ballotcross ✗ BALLOT X */
    CodePair { keysym: 0x0af5, ucs: 0x266f }, /*                musicalsharp ♯ MUSIC SHARP SIGN */
    CodePair { keysym: 0x0af6, ucs: 0x266d }, /*                 musicalflat ♭ MUSIC FLAT SIGN */
    CodePair { keysym: 0x0af7, ucs: 0x2642 }, /*                  malesymbol ♂ MALE SIGN */
    CodePair { keysym: 0x0af8, ucs: 0x2640 }, /*                femalesymbol ♀ FEMALE SIGN */
    CodePair { keysym: 0x0af9, ucs: 0x260e }, /*                   telephone ☎ BLACK TELEPHONE */
    CodePair { keysym: 0x0afa, ucs: 0x2315 }, /*           telephonerecorder ⌕ TELEPHONE RECORDER */
    CodePair { keysym: 0x0afb, ucs: 0x2117 }, /*         phonographcopyright ℗ SOUND RECORDING COPYRIGHT */
    CodePair { keysym: 0x0afc, ucs: 0x2038 }, /*                       caret ‸ CARET */
    CodePair { keysym: 0x0afd, ucs: 0x201a }, /*          singlelowquotemark ‚ SINGLE LOW-9 QUOTATION MARK */
    CodePair { keysym: 0x0afe, ucs: 0x201e }, /*          doublelowquotemark „ DOUBLE LOW-9 QUOTATION MARK */
    /* 0x0aff                                                         cursor ? ??? */
    CodePair { keysym: 0x0ba3, ucs: 0x003c }, /*                   leftcaret < LESS-THAN SIGN */
    CodePair { keysym: 0x0ba6, ucs: 0x003e }, /*                  rightcaret > GREATER-THAN SIGN */
    CodePair { keysym: 0x0ba8, ucs: 0x2228 }, /*                   downcaret ∨ LOGICAL OR */
    CodePair { keysym: 0x0ba9, ucs: 0x2227 }, /*                     upcaret ∧ LOGICAL AND */
    CodePair { keysym: 0x0bc0, ucs: 0x00af }, /*                     overbar ¯ MACRON */
    CodePair { keysym: 0x0bc2, ucs: 0x22a4 }, /*                    downtack ⊤ DOWN TACK */
    CodePair { keysym: 0x0bc3, ucs: 0x2229 }, /*                      upshoe ∩ INTERSECTION */
    CodePair { keysym: 0x0bc4, ucs: 0x230a }, /*                   downstile ⌊ LEFT FLOOR */
    CodePair { keysym: 0x0bc6, ucs: 0x005f }, /*                    underbar _ LOW LINE */
    CodePair { keysym: 0x0bca, ucs: 0x2218 }, /*                         jot ∘ RING OPERATOR */
    CodePair { keysym: 0x0bcc, ucs: 0x2395 }, /*                        quad ⎕ APL FUNCTIONAL SYMBOL QUAD (Unicode 3.0) */
    CodePair { keysym: 0x0bce, ucs: 0x22a5 }, /*                      uptack ⊥ UP TACK */
    CodePair { keysym: 0x0bcf, ucs: 0x25cb }, /*                      circle ○ WHITE CIRCLE */
    CodePair { keysym: 0x0bd3, ucs: 0x2308 }, /*                     upstile ⌈ LEFT CEILING */
    CodePair { keysym: 0x0bd6, ucs: 0x222a }, /*                    downshoe ∪ UNION */
    CodePair { keysym: 0x0bd8, ucs: 0x2283 }, /*                   rightshoe ⊃ SUPERSET OF */
    CodePair { keysym: 0x0bda, ucs: 0x2282 }, /*                    leftshoe ⊂ SUBSET OF */
    CodePair { keysym: 0x0bdc, ucs: 0x22a3 }, /*                    lefttack ⊣ LEFT TACK */
    CodePair { keysym: 0x0bfc, ucs: 0x22a2 }, /*                   righttack ⊢ RIGHT TACK */
    CodePair { keysym: 0x0cdf, ucs: 0x2017 }, /*        hebrew_doublelowline ‗ DOUBLE LOW LINE */
    CodePair { keysym: 0x0ce0, ucs: 0x05d0 }, /*                hebrew_aleph א HEBREW LETTER ALEF */
    CodePair { keysym: 0x0ce1, ucs: 0x05d1 }, /*                  hebrew_bet ב HEBREW LETTER BET */
    CodePair { keysym: 0x0ce2, ucs: 0x05d2 }, /*                hebrew_gimel ג HEBREW LETTER GIMEL */
    CodePair { keysym: 0x0ce3, ucs: 0x05d3 }, /*                hebrew_dalet ד HEBREW LETTER DALET */
    CodePair { keysym: 0x0ce4, ucs: 0x05d4 }, /*                   hebrew_he ה HEBREW LETTER HE */
    CodePair { keysym: 0x0ce5, ucs: 0x05d5 }, /*                  hebrew_waw ו HEBREW LETTER VAV */
    CodePair { keysym: 0x0ce6, ucs: 0x05d6 }, /*                 hebrew_zain ז HEBREW LETTER ZAYIN */
    CodePair { keysym: 0x0ce7, ucs: 0x05d7 }, /*                 hebrew_chet ח HEBREW LETTER HET */
    CodePair { keysym: 0x0ce8, ucs: 0x05d8 }, /*                  hebrew_tet ט HEBREW LETTER TET */
    CodePair { keysym: 0x0ce9, ucs: 0x05d9 }, /*                  hebrew_yod י HEBREW LETTER YOD */
    CodePair { keysym: 0x0cea, ucs: 0x05da }, /*            hebrew_finalkaph ך HEBREW LETTER FINAL KAF */
    CodePair { keysym: 0x0ceb, ucs: 0x05db }, /*                 hebrew_kaph כ HEBREW LETTER KAF */
    CodePair { keysym: 0x0cec, ucs: 0x05dc }, /*                hebrew_lamed ל HEBREW LETTER LAMED */
    CodePair { keysym: 0x0ced, ucs: 0x05dd }, /*             hebrew_finalmem ם HEBREW LETTER FINAL MEM */
    CodePair { keysym: 0x0cee, ucs: 0x05de }, /*                  hebrew_mem מ HEBREW LETTER MEM */
    CodePair { keysym: 0x0cef, ucs: 0x05df }, /*             hebrew_finalnun ן HEBREW LETTER FINAL NUN */
    CodePair { keysym: 0x0cf0, ucs: 0x05e0 }, /*                  hebrew_nun נ HEBREW LETTER NUN */
    CodePair { keysym: 0x0cf1, ucs: 0x05e1 }, /*               hebrew_samech ס HEBREW LETTER SAMEKH */
    CodePair { keysym: 0x0cf2, ucs: 0x05e2 }, /*                 hebrew_ayin ע HEBREW LETTER AYIN */
    CodePair { keysym: 0x0cf3, ucs: 0x05e3 }, /*              hebrew_finalpe ף HEBREW LETTER FINAL PE */
    CodePair { keysym: 0x0cf4, ucs: 0x05e4 }, /*                   hebrew_pe פ HEBREW LETTER PE */
    CodePair { keysym: 0x0cf5, ucs: 0x05e5 }, /*            hebrew_finalzade ץ HEBREW LETTER FINAL TSADI */
    CodePair { keysym: 0x0cf6, ucs: 0x05e6 }, /*                 hebrew_zade צ HEBREW LETTER TSADI */
    CodePair { keysym: 0x0cf7, ucs: 0x05e7 }, /*                 hebrew_qoph ק HEBREW LETTER QOF */
    CodePair { keysym: 0x0cf8, ucs: 0x05e8 }, /*                 hebrew_resh ר HEBREW LETTER RESH */
    CodePair { keysym: 0x0cf9, ucs: 0x05e9 }, /*                 hebrew_shin ש HEBREW LETTER SHIN */
    CodePair { keysym: 0x0cfa, ucs: 0x05ea }, /*                  hebrew_taw ת HEBREW LETTER TAV */
    CodePair { keysym: 0x0da1, ucs: 0x0e01 }, /*                  Thai_kokai ก THAI CHARACTER KO KAI */
    CodePair { keysym: 0x0da2, ucs: 0x0e02 }, /*                Thai_khokhai ข THAI CHARACTER KHO KHAI */
    CodePair { keysym: 0x0da3, ucs: 0x0e03 }, /*               Thai_khokhuat ฃ THAI CHARACTER KHO KHUAT */
    CodePair { keysym: 0x0da4, ucs: 0x0e04 }, /*               Thai_khokhwai ค THAI CHARACTER KHO KHWAI */
    CodePair { keysym: 0x0da5, ucs: 0x0e05 }, /*                Thai_khokhon ฅ THAI CHARACTER KHO KHON */
    CodePair { keysym: 0x0da6, ucs: 0x0e06 }, /*             Thai_khorakhang ฆ THAI CHARACTER KHO RAKHANG */
    CodePair { keysym: 0x0da7, ucs: 0x0e07 }, /*                 Thai_ngongu ง THAI CHARACTER NGO NGU */
    CodePair { keysym: 0x0da8, ucs: 0x0e08 }, /*                Thai_chochan จ THAI CHARACTER CHO CHAN */
    CodePair { keysym: 0x0da9, ucs: 0x0e09 }, /*               Thai_choching ฉ THAI CHARACTER CHO CHING */
    CodePair { keysym: 0x0daa, ucs: 0x0e0a }, /*               Thai_chochang ช THAI CHARACTER CHO CHANG */
    CodePair { keysym: 0x0dab, ucs: 0x0e0b }, /*                   Thai_soso ซ THAI CHARACTER SO SO */
    CodePair { keysym: 0x0dac, ucs: 0x0e0c }, /*                Thai_chochoe ฌ THAI CHARACTER CHO CHOE */
    CodePair { keysym: 0x0dad, ucs: 0x0e0d }, /*                 Thai_yoying ญ THAI CHARACTER YO YING */
    CodePair { keysym: 0x0dae, ucs: 0x0e0e }, /*                Thai_dochada ฎ THAI CHARACTER DO CHADA */
    CodePair { keysym: 0x0daf, ucs: 0x0e0f }, /*                Thai_topatak ฏ THAI CHARACTER TO PATAK */
    CodePair { keysym: 0x0db0, ucs: 0x0e10 }, /*                Thai_thothan ฐ THAI CHARACTER THO THAN */
    CodePair { keysym: 0x0db1, ucs: 0x0e11 }, /*          Thai_thonangmontho ฑ THAI CHARACTER THO NANGMONTHO */
    CodePair { keysym: 0x0db2, ucs: 0x0e12 }, /*             Thai_thophuthao ฒ THAI CHARACTER THO PHUTHAO */
    CodePair { keysym: 0x0db3, ucs: 0x0e13 }, /*                  Thai_nonen ณ THAI CHARACTER NO NEN */
    CodePair { keysym: 0x0db4, ucs: 0x0e14 }, /*                  Thai_dodek ด THAI CHARACTER DO DEK */
    CodePair { keysym: 0x0db5, ucs: 0x0e15 }, /*                  Thai_totao ต THAI CHARACTER TO TAO */
    CodePair { keysym: 0x0db6, ucs: 0x0e16 }, /*               Thai_thothung ถ THAI CHARACTER THO THUNG */
    CodePair { keysym: 0x0db7, ucs: 0x0e17 }, /*              Thai_thothahan ท THAI CHARACTER THO THAHAN */
    CodePair { keysym: 0x0db8, ucs: 0x0e18 }, /*               Thai_thothong ธ THAI CHARACTER THO THONG */
    CodePair { keysym: 0x0db9, ucs: 0x0e19 }, /*                   Thai_nonu น THAI CHARACTER NO NU */
    CodePair { keysym: 0x0dba, ucs: 0x0e1a }, /*               Thai_bobaimai บ THAI CHARACTER BO BAIMAI */
    CodePair { keysym: 0x0dbb, ucs: 0x0e1b }, /*                  Thai_popla ป THAI CHARACTER PO PLA */
    CodePair { keysym: 0x0dbc, ucs: 0x0e1c }, /*               Thai_phophung ผ THAI CHARACTER PHO PHUNG */
    CodePair { keysym: 0x0dbd, ucs: 0x0e1d }, /*                   Thai_fofa ฝ THAI CHARACTER FO FA */
    CodePair { keysym: 0x0dbe, ucs: 0x0e1e }, /*                Thai_phophan พ THAI CHARACTER PHO PHAN */
    CodePair { keysym: 0x0dbf, ucs: 0x0e1f }, /*                  Thai_fofan ฟ THAI CHARACTER FO FAN */
    CodePair { keysym: 0x0dc0, ucs: 0x0e20 }, /*             Thai_phosamphao ภ THAI CHARACTER PHO SAMPHAO */
    CodePair { keysym: 0x0dc1, ucs: 0x0e21 }, /*                   Thai_moma ม THAI CHARACTER MO MA */
    CodePair { keysym: 0x0dc2, ucs: 0x0e22 }, /*                  Thai_yoyak ย THAI CHARACTER YO YAK */
    CodePair { keysym: 0x0dc3, ucs: 0x0e23 }, /*                  Thai_rorua ร THAI CHARACTER RO RUA */
    CodePair { keysym: 0x0dc4, ucs: 0x0e24 }, /*                     Thai_ru ฤ THAI CHARACTER RU */
    CodePair { keysym: 0x0dc5, ucs: 0x0e25 }, /*                 Thai_loling ล THAI CHARACTER LO LING */
    CodePair { keysym: 0x0dc6, ucs: 0x0e26 }, /*                     Thai_lu ฦ THAI CHARACTER LU */
    CodePair { keysym: 0x0dc7, ucs: 0x0e27 }, /*                 Thai_wowaen ว THAI CHARACTER WO WAEN */
    CodePair { keysym: 0x0dc8, ucs: 0x0e28 }, /*                 Thai_sosala ศ THAI CHARACTER SO SALA */
    CodePair { keysym: 0x0dc9, ucs: 0x0e29 }, /*                 Thai_sorusi ษ THAI CHARACTER SO RUSI */
    CodePair { keysym: 0x0dca, ucs: 0x0e2a }, /*                  Thai_sosua ส THAI CHARACTER SO SUA */
    CodePair { keysym: 0x0dcb, ucs: 0x0e2b }, /*                  Thai_hohip ห THAI CHARACTER HO HIP */
    CodePair { keysym: 0x0dcc, ucs: 0x0e2c }, /*                Thai_lochula ฬ THAI CHARACTER LO CHULA */
    CodePair { keysym: 0x0dcd, ucs: 0x0e2d }, /*                   Thai_oang อ THAI CHARACTER O ANG */
    CodePair { keysym: 0x0dce, ucs: 0x0e2e }, /*               Thai_honokhuk ฮ THAI CHARACTER HO NOKHUK */
    CodePair { keysym: 0x0dcf, ucs: 0x0e2f }, /*              Thai_paiyannoi ฯ THAI CHARACTER PAIYANNOI */
    CodePair { keysym: 0x0dd0, ucs: 0x0e30 }, /*                  Thai_saraa ะ THAI CHARACTER SARA A */
    CodePair { keysym: 0x0dd1, ucs: 0x0e31 }, /*             Thai_maihanakat ั THAI CHARACTER MAI HAN-AKAT */
    CodePair { keysym: 0x0dd2, ucs: 0x0e32 }, /*                 Thai_saraaa า THAI CHARACTER SARA AA */
    CodePair { keysym: 0x0dd3, ucs: 0x0e33 }, /*                 Thai_saraam ำ THAI CHARACTER SARA AM */
    CodePair { keysym: 0x0dd4, ucs: 0x0e34 }, /*                  Thai_sarai ิ THAI CHARACTER SARA I */
    CodePair { keysym: 0x0dd5, ucs: 0x0e35 }, /*                 Thai_saraii ี THAI CHARACTER SARA II */
    CodePair { keysym: 0x0dd6, ucs: 0x0e36 }, /*                 Thai_saraue ึ THAI CHARACTER SARA UE */
    CodePair { keysym: 0x0dd7, ucs: 0x0e37 }, /*                Thai_sarauee ื THAI CHARACTER SARA UEE */
    CodePair { keysym: 0x0dd8, ucs: 0x0e38 }, /*                  Thai_sarau ุ THAI CHARACTER SARA U */
    CodePair { keysym: 0x0dd9, ucs: 0x0e39 }, /*                 Thai_sarauu ู THAI CHARACTER SARA UU */
    CodePair { keysym: 0x0dda, ucs: 0x0e3a }, /*                Thai_phinthu ฺ THAI CHARACTER PHINTHU */
    CodePair { keysym: 0x0dde, ucs: 0x0e3e }, /*      Thai_maihanakat_maitho ฾ ??? */
    CodePair { keysym: 0x0ddf, ucs: 0x0e3f }, /*                   Thai_baht ฿ THAI CURRENCY SYMBOL BAHT */
    CodePair { keysym: 0x0de0, ucs: 0x0e40 }, /*                  Thai_sarae เ THAI CHARACTER SARA E */
    CodePair { keysym: 0x0de1, ucs: 0x0e41 }, /*                 Thai_saraae แ THAI CHARACTER SARA AE */
    CodePair { keysym: 0x0de2, ucs: 0x0e42 }, /*                  Thai_sarao โ THAI CHARACTER SARA O */
    CodePair { keysym: 0x0de3, ucs: 0x0e43 }, /*          Thai_saraaimaimuan ใ THAI CHARACTER SARA AI MAIMUAN */
    CodePair { keysym: 0x0de4, ucs: 0x0e44 }, /*         Thai_saraaimaimalai ไ THAI CHARACTER SARA AI MAIMALAI */
    CodePair { keysym: 0x0de5, ucs: 0x0e45 }, /*            Thai_lakkhangyao ๅ THAI CHARACTER LAKKHANGYAO */
    CodePair { keysym: 0x0de6, ucs: 0x0e46 }, /*               Thai_maiyamok ๆ THAI CHARACTER MAIYAMOK */
    CodePair { keysym: 0x0de7, ucs: 0x0e47 }, /*              Thai_maitaikhu ็ THAI CHARACTER MAITAIKHU */
    CodePair { keysym: 0x0de8, ucs: 0x0e48 }, /*                  Thai_maiek ่ THAI CHARACTER MAI EK */
    CodePair { keysym: 0x0de9, ucs: 0x0e49 }, /*                 Thai_maitho ้ THAI CHARACTER MAI THO */
    CodePair { keysym: 0x0dea, ucs: 0x0e4a }, /*                 Thai_maitri ๊ THAI CHARACTER MAI TRI */
    CodePair { keysym: 0x0deb, ucs: 0x0e4b }, /*            Thai_maichattawa ๋ THAI CHARACTER MAI CHATTAWA */
    CodePair { keysym: 0x0dec, ucs: 0x0e4c }, /*            Thai_thanthakhat ์ THAI CHARACTER THANTHAKHAT */
    CodePair { keysym: 0x0ded, ucs: 0x0e4d }, /*               Thai_nikhahit ํ THAI CHARACTER NIKHAHIT */
    CodePair { keysym: 0x0df0, ucs: 0x0e50 }, /*                 Thai_leksun ๐ THAI DIGIT ZERO */
    CodePair { keysym: 0x0df1, ucs: 0x0e51 }, /*                Thai_leknung ๑ THAI DIGIT ONE */
    CodePair { keysym: 0x0df2, ucs: 0x0e52 }, /*                Thai_leksong ๒ THAI DIGIT TWO */
    CodePair { keysym: 0x0df3, ucs: 0x0e53 }, /*                 Thai_leksam ๓ THAI DIGIT THREE */
    CodePair { keysym: 0x0df4, ucs: 0x0e54 }, /*                  Thai_leksi ๔ THAI DIGIT FOUR */
    CodePair { keysym: 0x0df5, ucs: 0x0e55 }, /*                  Thai_lekha ๕ THAI DIGIT FIVE */
    CodePair { keysym: 0x0df6, ucs: 0x0e56 }, /*                 Thai_lekhok ๖ THAI DIGIT SIX */
    CodePair { keysym: 0x0df7, ucs: 0x0e57 }, /*                Thai_lekchet ๗ THAI DIGIT SEVEN */
    CodePair { keysym: 0x0df8, ucs: 0x0e58 }, /*                Thai_lekpaet ๘ THAI DIGIT EIGHT */
    CodePair { keysym: 0x0df9, ucs: 0x0e59 }, /*                 Thai_lekkao ๙ THAI DIGIT NINE */
    CodePair { keysym: 0x0ea1, ucs: 0x3131 }, /*               Hangul_Kiyeog ㄱ HANGUL LETTER KIYEOK */
    CodePair { keysym: 0x0ea2, ucs: 0x3132 }, /*          Hangul_SsangKiyeog ㄲ HANGUL LETTER SSANGKIYEOK */
    CodePair { keysym: 0x0ea3, ucs: 0x3133 }, /*           Hangul_KiyeogSios ㄳ HANGUL LETTER KIYEOK-SIOS */
    CodePair { keysym: 0x0ea4, ucs: 0x3134 }, /*                Hangul_Nieun ㄴ HANGUL LETTER NIEUN */
    CodePair { keysym: 0x0ea5, ucs: 0x3135 }, /*           Hangul_NieunJieuj ㄵ HANGUL LETTER NIEUN-CIEUC */
    CodePair { keysym: 0x0ea6, ucs: 0x3136 }, /*           Hangul_NieunHieuh ㄶ HANGUL LETTER NIEUN-HIEUH */
    CodePair { keysym: 0x0ea7, ucs: 0x3137 }, /*               Hangul_Dikeud ㄷ HANGUL LETTER TIKEUT */
    CodePair { keysym: 0x0ea8, ucs: 0x3138 }, /*          Hangul_SsangDikeud ㄸ HANGUL LETTER SSANGTIKEUT */
    CodePair { keysym: 0x0ea9, ucs: 0x3139 }, /*                Hangul_Rieul ㄹ HANGUL LETTER RIEUL */
    CodePair { keysym: 0x0eaa, ucs: 0x313a }, /*          Hangul_RieulKiyeog ㄺ HANGUL LETTER RIEUL-KIYEOK */
    CodePair { keysym: 0x0eab, ucs: 0x313b }, /*           Hangul_RieulMieum ㄻ HANGUL LETTER RIEUL-MIEUM */
    CodePair { keysym: 0x0eac, ucs: 0x313c }, /*           Hangul_RieulPieub ㄼ HANGUL LETTER RIEUL-PIEUP */
    CodePair { keysym: 0x0ead, ucs: 0x313d }, /*            Hangul_RieulSios ㄽ HANGUL LETTER RIEUL-SIOS */
    CodePair { keysym: 0x0eae, ucs: 0x313e }, /*           Hangul_RieulTieut ㄾ HANGUL LETTER RIEUL-THIEUTH */
    CodePair { keysym: 0x0eaf, ucs: 0x313f }, /*          Hangul_RieulPhieuf ㄿ HANGUL LETTER RIEUL-PHIEUPH */
    CodePair { keysym: 0x0eb0, ucs: 0x3140 }, /*           Hangul_RieulHieuh ㅀ HANGUL LETTER RIEUL-HIEUH */
    CodePair { keysym: 0x0eb1, ucs: 0x3141 }, /*                Hangul_Mieum ㅁ HANGUL LETTER MIEUM */
    CodePair { keysym: 0x0eb2, ucs: 0x3142 }, /*                Hangul_Pieub ㅂ HANGUL LETTER PIEUP */
    CodePair { keysym: 0x0eb3, ucs: 0x3143 }, /*           Hangul_SsangPieub ㅃ HANGUL LETTER SSANGPIEUP */
    CodePair { keysym: 0x0eb4, ucs: 0x3144 }, /*            Hangul_PieubSios ㅄ HANGUL LETTER PIEUP-SIOS */
    CodePair { keysym: 0x0eb5, ucs: 0x3145 }, /*                 Hangul_Sios ㅅ HANGUL LETTER SIOS */
    CodePair { keysym: 0x0eb6, ucs: 0x3146 }, /*            Hangul_SsangSios ㅆ HANGUL LETTER SSANGSIOS */
    CodePair { keysym: 0x0eb7, ucs: 0x3147 }, /*                Hangul_Ieung ㅇ HANGUL LETTER IEUNG */
    CodePair { keysym: 0x0eb8, ucs: 0x3148 }, /*                Hangul_Jieuj ㅈ HANGUL LETTER CIEUC */
    CodePair { keysym: 0x0eb9, ucs: 0x3149 }, /*           Hangul_SsangJieuj ㅉ HANGUL LETTER SSANGCIEUC */
    CodePair { keysym: 0x0eba, ucs: 0x314a }, /*                Hangul_Cieuc ㅊ HANGUL LETTER CHIEUCH */
    CodePair { keysym: 0x0ebb, ucs: 0x314b }, /*               Hangul_Khieuq ㅋ HANGUL LETTER KHIEUKH */
    CodePair { keysym: 0x0ebc, ucs: 0x314c }, /*                Hangul_Tieut ㅌ HANGUL LETTER THIEUTH */
    CodePair { keysym: 0x0ebd, ucs: 0x314d }, /*               Hangul_Phieuf ㅍ HANGUL LETTER PHIEUPH */
    CodePair { keysym: 0x0ebe, ucs: 0x314e }, /*                Hangul_Hieuh ㅎ HANGUL LETTER HIEUH */
    CodePair { keysym: 0x0ebf, ucs: 0x314f }, /*                    Hangul_A ㅏ HANGUL LETTER A */
    CodePair { keysym: 0x0ec0, ucs: 0x3150 }, /*                   Hangul_AE ㅐ HANGUL LETTER AE */
    CodePair { keysym: 0x0ec1, ucs: 0x3151 }, /*                   Hangul_YA ㅑ HANGUL LETTER YA */
    CodePair { keysym: 0x0ec2, ucs: 0x3152 }, /*                  Hangul_YAE ㅒ HANGUL LETTER YAE */
    CodePair { keysym: 0x0ec3, ucs: 0x3153 }, /*                   Hangul_EO ㅓ HANGUL LETTER EO */
    CodePair { keysym: 0x0ec4, ucs: 0x3154 }, /*                    Hangul_E ㅔ HANGUL LETTER E */
    CodePair { keysym: 0x0ec5, ucs: 0x3155 }, /*                  Hangul_YEO ㅕ HANGUL LETTER YEO */
    CodePair { keysym: 0x0ec6, ucs: 0x3156 }, /*                   Hangul_YE ㅖ HANGUL LETTER YE */
    CodePair { keysym: 0x0ec7, ucs: 0x3157 }, /*                    Hangul_O ㅗ HANGUL LETTER O */
    CodePair { keysym: 0x0ec8, ucs: 0x3158 }, /*                   Hangul_WA ㅘ HANGUL LETTER WA */
    CodePair { keysym: 0x0ec9, ucs: 0x3159 }, /*                  Hangul_WAE ㅙ HANGUL LETTER WAE */
    CodePair { keysym: 0x0eca, ucs: 0x315a }, /*                   Hangul_OE ㅚ HANGUL LETTER OE */
    CodePair { keysym: 0x0ecb, ucs: 0x315b }, /*                   Hangul_YO ㅛ HANGUL LETTER YO */
    CodePair { keysym: 0x0ecc, ucs: 0x315c }, /*                    Hangul_U ㅜ HANGUL LETTER U */
    CodePair { keysym: 0x0ecd, ucs: 0x315d }, /*                  Hangul_WEO ㅝ HANGUL LETTER WEO */
    CodePair { keysym: 0x0ece, ucs: 0x315e }, /*                   Hangul_WE ㅞ HANGUL LETTER WE */
    CodePair { keysym: 0x0ecf, ucs: 0x315f }, /*                   Hangul_WI ㅟ HANGUL LETTER WI */
    CodePair { keysym: 0x0ed0, ucs: 0x3160 }, /*                   Hangul_YU ㅠ HANGUL LETTER YU */
    CodePair { keysym: 0x0ed1, ucs: 0x3161 }, /*                   Hangul_EU ㅡ HANGUL LETTER EU */
    CodePair { keysym: 0x0ed2, ucs: 0x3162 }, /*                   Hangul_YI ㅢ HANGUL LETTER YI */
    CodePair { keysym: 0x0ed3, ucs: 0x3163 }, /*                    Hangul_I ㅣ HANGUL LETTER I */
    CodePair { keysym: 0x0ed4, ucs: 0x11a8 }, /*             Hangul_J_Kiyeog ᆨ HANGUL JONGSEONG KIYEOK */
    CodePair { keysym: 0x0ed5, ucs: 0x11a9 }, /*        Hangul_J_SsangKiyeog ᆩ HANGUL JONGSEONG SSANGKIYEOK */
    CodePair { keysym: 0x0ed6, ucs: 0x11aa }, /*         Hangul_J_KiyeogSios ᆪ HANGUL JONGSEONG KIYEOK-SIOS */
    CodePair { keysym: 0x0ed7, ucs: 0x11ab }, /*              Hangul_J_Nieun ᆫ HANGUL JONGSEONG NIEUN */
    CodePair { keysym: 0x0ed8, ucs: 0x11ac }, /*         Hangul_J_NieunJieuj ᆬ HANGUL JONGSEONG NIEUN-CIEUC */
    CodePair { keysym: 0x0ed9, ucs: 0x11ad }, /*         Hangul_J_NieunHieuh ᆭ HANGUL JONGSEONG NIEUN-HIEUH */
    CodePair { keysym: 0x0eda, ucs: 0x11ae }, /*             Hangul_J_Dikeud ᆮ HANGUL JONGSEONG TIKEUT */
    CodePair { keysym: 0x0edb, ucs: 0x11af }, /*              Hangul_J_Rieul ᆯ HANGUL JONGSEONG RIEUL */
    CodePair { keysym: 0x0edc, ucs: 0x11b0 }, /*        Hangul_J_RieulKiyeog ᆰ HANGUL JONGSEONG RIEUL-KIYEOK */
    CodePair { keysym: 0x0edd, ucs: 0x11b1 }, /*         Hangul_J_RieulMieum ᆱ HANGUL JONGSEONG RIEUL-MIEUM */
    CodePair { keysym: 0x0ede, ucs: 0x11b2 }, /*         Hangul_J_RieulPieub ᆲ HANGUL JONGSEONG RIEUL-PIEUP */
    CodePair { keysym: 0x0edf, ucs: 0x11b3 }, /*          Hangul_J_RieulSios ᆳ HANGUL JONGSEONG RIEUL-SIOS */
    CodePair { keysym: 0x0ee0, ucs: 0x11b4 }, /*         Hangul_J_RieulTieut ᆴ HANGUL JONGSEONG RIEUL-THIEUTH */
    CodePair { keysym: 0x0ee1, ucs: 0x11b5 }, /*        Hangul_J_RieulPhieuf ᆵ HANGUL JONGSEONG RIEUL-PHIEUPH */
    CodePair { keysym: 0x0ee2, ucs: 0x11b6 }, /*         Hangul_J_RieulHieuh ᆶ HANGUL JONGSEONG RIEUL-HIEUH */
    CodePair { keysym: 0x0ee3, ucs: 0x11b7 }, /*              Hangul_J_Mieum ᆷ HANGUL JONGSEONG MIEUM */
    CodePair { keysym: 0x0ee4, ucs: 0x11b8 }, /*              Hangul_J_Pieub ᆸ HANGUL JONGSEONG PIEUP */
    CodePair { keysym: 0x0ee5, ucs: 0x11b9 }, /*          Hangul_J_PieubSios ᆹ HANGUL JONGSEONG PIEUP-SIOS */
    CodePair { keysym: 0x0ee6, ucs: 0x11ba }, /*               Hangul_J_Sios ᆺ HANGUL JONGSEONG SIOS */
    CodePair { keysym: 0x0ee7, ucs: 0x11bb }, /*          Hangul_J_SsangSios ᆻ HANGUL JONGSEONG SSANGSIOS */
    CodePair { keysym: 0x0ee8, ucs: 0x11bc }, /*              Hangul_J_Ieung ᆼ HANGUL JONGSEONG IEUNG */
    CodePair { keysym: 0x0ee9, ucs: 0x11bd }, /*              Hangul_J_Jieuj ᆽ HANGUL JONGSEONG CIEUC */
    CodePair { keysym: 0x0eea, ucs: 0x11be }, /*              Hangul_J_Cieuc ᆾ HANGUL JONGSEONG CHIEUCH */
    CodePair { keysym: 0x0eeb, ucs: 0x11bf }, /*             Hangul_J_Khieuq ᆿ HANGUL JONGSEONG KHIEUKH */
    CodePair { keysym: 0x0eec, ucs: 0x11c0 }, /*              Hangul_J_Tieut ᇀ HANGUL JONGSEONG THIEUTH */
    CodePair { keysym: 0x0eed, ucs: 0x11c1 }, /*             Hangul_J_Phieuf ᇁ HANGUL JONGSEONG PHIEUPH */
    CodePair { keysym: 0x0eee, ucs: 0x11c2 }, /*              Hangul_J_Hieuh ᇂ HANGUL JONGSEONG HIEUH */
    CodePair { keysym: 0x0eef, ucs: 0x316d }, /*     Hangul_RieulYeorinHieuh ㅭ HANGUL LETTER RIEUL-YEORINHIEUH */
    CodePair { keysym: 0x0ef0, ucs: 0x3171 }, /*    Hangul_SunkyeongeumMieum ㅱ HANGUL LETTER KAPYEOUNMIEUM */
    CodePair { keysym: 0x0ef1, ucs: 0x3178 }, /*    Hangul_SunkyeongeumPieub ㅸ HANGUL LETTER KAPYEOUNPIEUP */
    CodePair { keysym: 0x0ef2, ucs: 0x317f }, /*              Hangul_PanSios ㅿ HANGUL LETTER PANSIOS */
    CodePair { keysym: 0x0ef3, ucs: 0x3181 }, /*    Hangul_KkogjiDalrinIeung ㆁ HANGUL LETTER YESIEUNG */
    CodePair { keysym: 0x0ef4, ucs: 0x3184 }, /*   Hangul_SunkyeongeumPhieuf ㆄ HANGUL LETTER KAPYEOUNPHIEUPH */
    CodePair { keysym: 0x0ef5, ucs: 0x3186 }, /*          Hangul_YeorinHieuh ㆆ HANGUL LETTER YEORINHIEUH */
    CodePair { keysym: 0x0ef6, ucs: 0x318d }, /*                Hangul_AraeA ㆍ HANGUL LETTER ARAEA */
    CodePair { keysym: 0x0ef7, ucs: 0x318e }, /*               Hangul_AraeAE ㆎ HANGUL LETTER ARAEAE */
    CodePair { keysym: 0x0ef8, ucs: 0x11eb }, /*            Hangul_J_PanSios ᇫ HANGUL JONGSEONG PANSIOS */
    CodePair { keysym: 0x0ef9, ucs: 0x11f0 }, /*  Hangul_J_KkogjiDalrinIeung ᇰ HANGUL JONGSEONG YESIEUNG */
    CodePair { keysym: 0x0efa, ucs: 0x11f9 }, /*        Hangul_J_YeorinHieuh ᇹ HANGUL JONGSEONG YEORINHIEUH */
    CodePair { keysym: 0x0eff, ucs: 0x20a9 }, /*                  Korean_Won ₩ WON SIGN */
    CodePair { keysym: 0x13bc, ucs: 0x0152 }, /*                          OE Œ LATIN CAPITAL LIGATURE OE */
    CodePair { keysym: 0x13bd, ucs: 0x0153 }, /*                          oe œ LATIN SMALL LIGATURE OE */
    CodePair { keysym: 0x13be, ucs: 0x0178 }, /*                  Ydiaeresis Ÿ LATIN CAPITAL LETTER Y WITH DIAERESIS */
    CodePair { keysym: 0x20ac, ucs: 0x20ac }, /*                    EuroSign € EURO SIGN */
];

#[cfg(test)]
mod tests {
    use crate::*;

    // NOTE: Tests were taken from https://github.com/xkbcommon/libxkbcommon/blob/f3210cbf/test/keysym.c#L229
    // and are under MIT license.

    /*
     * Copyright © 2009 Dan Nicholson
     *
     * Permission is hereby granted, free of charge, to any person obtaining a
     * copy of this software and associated documentation files (the "Software"),
     * to deal in the Software without restriction, including without limitation
     * the rights to use, copy, modify, merge, publish, distribute, sublicense,
     * and/or sell copies of the Software, and to permit persons to whom the
     * Software is furnished to do so, subject to the following conditions:
     *
     * The above copyright notice and this permission notice (including the next
     * paragraph) shall be included in all copies or substantial portions of the
     * Software.
     *
     * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
     * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
     * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
     * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
     * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
     * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
     * DEALINGS IN THE SOFTWARE.
     */
    #[test]
    fn keysym_from_char() {
        // Test conversion back and forth.
        fn test_from_to(ch: char, keysym: Keysym) {
            assert_eq!(Keysym::from_char(ch), keysym);
            assert_eq!(keysym.key_char(), Some(ch));
        }
        test_from_to('y', Keysym::new(key::y));
        test_from_to('u', Keysym::new(key::u));
        test_from_to('m', Keysym::new(key::m));
        test_from_to('\u{43c}', Keysym::new(key::Cyrillic_em));
        test_from_to('\u{443}', Keysym::new(key::Cyrillic_u));
        test_from_to('!', Keysym::new(key::exclam));
        test_from_to('\u{F8}', Keysym::new(key::oslash));
        test_from_to('\u{5D0}', Keysym::new(key::hebrew_aleph));
        test_from_to('\u{634}', Keysym::new(key::Arabic_sheen));
        test_from_to('\u{1F609}', Keysym::new(0x0101F609)); // Keysym::emoji
        test_from_to('\u{08}', Keysym::new(key::BackSpace));
        test_from_to('\t', Keysym::new(key::Tab));
        test_from_to('\n', Keysym::new(key::Linefeed));
        test_from_to('\u{0b}', Keysym::new(key::Clear));
        test_from_to('\r', Keysym::new(key::Return));
        test_from_to('\u{1b}', Keysym::new(key::Escape));
        test_from_to('\u{7f}', Keysym::new(key::Delete));
        test_from_to(' ', Keysym::new(key::space));
        test_from_to(',', Keysym::new(key::comma));
        test_from_to('.', Keysym::new(key::period));
        test_from_to('=', Keysym::new(key::equal));
        test_from_to('9', Keysym::new(key::_9));
        test_from_to('*', Keysym::new(key::asterisk));
        test_from_to('\u{d7}', Keysym::new(key::multiply));
        test_from_to('-', Keysym::new(key::minus));
        test_from_to('\u{10fffd}', Keysym::new(0x110fffd));
        test_from_to('\u{20ac}', Keysym::new(key::EuroSign));

        // Unicode non-characters.

        // rust doesn't allow building the char from the surrogates.
        // assert_eq!(Keysym::from_char('\u{d800}'), Keysym::NoSymbol)); // Unicode surrogates
        // assert_eq!(Keysym::from_char('\u{dfff}'), Keysym::NoSymbol)); // Unicode surrogates

        assert_eq!(Keysym::from_char('\u{fdd0}'), Keysym::NoSymbol);
        assert_eq!(Keysym::from_char('\u{fdef}'), Keysym::NoSymbol);
        assert_eq!(Keysym::from_char('\u{fffe}'), Keysym::NoSymbol);
        assert_eq!(Keysym::from_char('\u{ffff}'), Keysym::NoSymbol);
        assert_eq!(Keysym::from_char('\u{7fffe}'), Keysym::NoSymbol);
        assert_eq!(Keysym::from_char('\u{7ffff}'), Keysym::NoSymbol);
        assert_eq!(Keysym::from_char('\u{afffe}'), Keysym::NoSymbol);
        assert_eq!(Keysym::from_char('\u{affff}'), Keysym::NoSymbol);

        // Rust doesn't allow codepoints outside the Unicode planes for char.
        // assert_eq!(Keysym::from_char('\u{110000}', Keysym::NoSymbol);
        // assert_eq!(Keysym::from_char('\u{deadbeef}', Keysym::NoSymbol);
    }
}
