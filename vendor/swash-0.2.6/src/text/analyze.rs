use super::{cluster::Boundary, Codepoint, LineBreak, Properties, WordBreak};
use core::borrow::Borrow;

/// Returns an iterator yielding unicode properties and boundary analysis for
/// each character in the specified sequence.
pub fn analyze<I>(chars: I) -> Analyze<I::IntoIter>
where
    I: IntoIterator,
    I::IntoIter: Clone,
    I::Item: Borrow<char>,
{
    Analyze {
        chars: chars.into_iter(),
        state: BoundaryState::new(),
    }
}

/// Iterator that yields Unicode properties and boundary analysis.
/// This iterator is created by the [`analyze`] function.
#[derive(Clone)]
pub struct Analyze<I> {
    chars: I,
    state: BoundaryState,
}

impl<I> Iterator for Analyze<I>
where
    I: Iterator + Clone,
    I::Item: Borrow<char>,
{
    type Item = (Properties, Boundary);

    fn next(&mut self) -> Option<Self::Item> {
        self.state.next(&mut self.chars)
    }
}

impl<I> Analyze<I> {
    /// Returns true if the analysis indicates that bidi resolution is
    /// required.
    pub fn needs_bidi_resolution(&self) -> bool {
        self.state.needs_bidi
    }

    /// Sets the word breaking strength that will be used to analyze the next character.
    pub fn set_break_strength(&mut self, strength: WordBreakStrength) {
        self.state.strength = strength;
    }
}

/// Word breaking strength (corresponds to <https://drafts.csswg.org/css-text/#word-break-property>).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[repr(u8)]
pub enum WordBreakStrength {
    /// Words can be broken according to their normal Unicode rules.
    #[default]
    Normal,
    /// Breaking treats numeric, alphabetic, and Southeast Asian classes as Ideographic. Note that this does not affect
    /// breaking punctuation.
    BreakAll,
    /// Breaking between typographic letter units or the NU, AL, AI, or ID classes is prohibited.
    KeepAll,
}

#[derive(Clone)]
struct BoundaryState {
    strength: WordBreakStrength,
    prev: WordBreak,
    prevent_next: bool,
    ri_count: u8,
    emoji: bool,
    next_emoji: bool,
    line_state: (u8, Option<LineBreak>),
    first: bool,
    needs_bidi: bool,
}

impl BoundaryState {
    fn new() -> Self {
        Self {
            strength: WordBreakStrength::default(),
            prev: WordBreak::EX,
            prevent_next: false,
            ri_count: 0,
            emoji: false,
            next_emoji: false,
            line_state: (sot, None),
            first: true,
            needs_bidi: false,
        }
    }

    fn reset_state(&mut self) {
        self.ri_count = 0;
        self.emoji = false;
    }

    fn check_word<I>(&mut self, props: Properties, iter: &mut I) -> bool
    where
        I: Iterator + Clone,
        I::Item: Borrow<char>,
    {
        use WordBreak::*;
        let b = props.word_break();
        let emoji = props.is_extended_pictographic();
        if self.first {
            self.first = false;
            self.prev = b;
            self.next_emoji = emoji;
            if b == RI {
                self.ri_count = 1;
            }
            return true;
        }
        let prev_emoji = self.emoji;
        self.emoji = self.emoji || self.next_emoji;
        self.next_emoji = emoji;
        let a = self.prev;
        self.prev = b;
        if self.prevent_next {
            self.prevent_next = false;
            return false;
        }
        if a == CR && b == LF {
            self.reset_state();
            return false;
        }
        let a_mask = a.mask();
        let b_mask = b.mask();
        const AH_LETTER: u32 = LE.mask() | HL.mask();
        const MID_NUM_LET_Q: u32 = MB.mask() | SQ.mask();
        const WB3_A: u32 = NL.mask() | CR.mask() | LF.mask();
        if a_mask & WB3_A != 0 || b_mask & WB3_A != 0 {
            // (Newline | CR | LF) ÷
            // ÷ (Newline | CR | LF)
            self.reset_state();
            return true;
        }
        if a == ZWJ && emoji {
            self.reset_state();
            return false;
        }
        const WB_4: u32 = Extend.mask() | FO.mask() | ZWJ.mask();
        if b_mask & WB_4 != 0 {
            // Ignore format and extend characters
            self.reset_state();
            self.prev = a;
            return false;
        }
        if a == WSegSpace && b == WSegSpace {
            // WSegSpace × WSegSpace
            self.reset_state();
            return false;
        }
        if a_mask & AH_LETTER != 0 {
            // AHLetter × AHLetter
            // AHLetter × Numeric
            if b_mask & (AH_LETTER | NU.mask()) != 0 {
                self.reset_state();
                return false;
            }
            if b_mask & (ML.mask() | MID_NUM_LET_Q) != 0 {
                // AHLetter	× (MidLetter | MidNumLetQ) AHLetter
                // AHLetter (MidLetter | MidNumLetQ) × AHLetter
                if let Some(c) = iter
                    .clone()
                    .next()
                    .map(|p| p.borrow().properties().word_break())
                {
                    if c.mask() & AH_LETTER != 0 {
                        self.prevent_next = true;
                        self.reset_state();
                        return false;
                    }
                }
            }
        }
        if a == HL {
            if b == SQ {
                self.reset_state();
                return false;
            }
            if b == DQ {
                // Hebrew_Letter × Double_Quote Hebrew_Letter
                // Hebrew_Letter Double_Quote × Hebrew_Letter
                if let Some(c) = iter
                    .clone()
                    .next()
                    .map(|p| p.borrow().properties().word_break())
                {
                    if c == HL {
                        self.prevent_next = true;
                        self.reset_state();
                        return false;
                    }
                }
            }
        }
        if a_mask & NU.mask() != 0 {
            // Numeric × Numeric
            // Numeric × AHLetter
            if b_mask & (NU.mask() | AH_LETTER) != 0 {
                self.reset_state();
                return false;
            }
            if b_mask & (MN.mask() | MID_NUM_LET_Q) != 0 {
                if let Some(c) = iter
                    .clone()
                    .next()
                    .map(|p| p.borrow().properties().word_break())
                {
                    // Numeric (MidNum | MidNumLetQ) × Numeric
                    // Numeric × (MidNum | MidNumLetQ) Numeric
                    if c == NU {
                        self.prevent_next = true;
                        self.reset_state();
                        return false;
                    }
                }
            }
        }
        if a == KA && b == KA {
            // Katakana × Katakana
            self.reset_state();
            return false;
        }
        const WB13_A: u32 = AH_LETTER | NU.mask() | KA.mask() | EX.mask();
        if a_mask & WB13_A != 0 && b == EX {
            // (AHLetter | Numeric | Katakana | ExtendNumLet) ×	ExtendNumLet
            self.reset_state();
            return false;
        }
        const WB13_B: u32 = AH_LETTER | NU.mask() | KA.mask();
        if a == EX && b_mask & WB13_B != 0 {
            // ExtendNumLet × (AHLetter | Numeric | Katakana)
            self.reset_state();
            return false;
        }
        if prev_emoji && a == ZWJ && emoji {
            self.ri_count = 0;
            return false;
        }
        if self.ri_count == 2 {
            self.reset_state();
            if b == RI {
                self.ri_count = 1;
            }
            return true;
        }
        if b == RI {
            self.ri_count += 1;
            if a != RI {
                self.reset_state();
                self.ri_count = 1;
                return true;
            }
            self.emoji = false;
            return false;
        }
        self.reset_state();
        true
    }

    fn check_line(&mut self, props: Properties) -> Boundary {
        let state = self.line_state;
        let lb = props.line_break();

        use LineBreak::*;

        let val = PAIR_TABLE[state.0 as usize][lb as usize];

        // word-break: break-all
        //
        // Treat the NU, AL, and SA line breaking classes as ID.
        let mode_val = if self.strength == WordBreakStrength::BreakAll {
            let left = if matches!(state.1, Some(AL | NU | SA)) {
                ID as usize
            } else {
                state.0 as usize
            };
            let right = if matches!(lb, AL | NU | SA) {
                ID as usize
            } else {
                lb as usize
            };
            PAIR_TABLE[left][right]
        } else {
            val
        };

        let mut mode = if mode_val & MANDATORY_BREAK_BIT != 0 {
            Boundary::Mandatory
        } else if mode_val & ALLOWED_BREAK_BIT != 0 && state.1 != Some(ZWJ) {
            Boundary::Line
        } else {
            Boundary::None
        };

        // word-break: keep-all
        //
        // Prohibit breaking between typographic letter units or the NU, AL, or
        // AI, or ID classes.
        // (See https://github.com/unicode-org/icu4x/blob/1e27279/components/segmenter/src/line.rs#L836-L840)
        if let (
            WordBreakStrength::KeepAll,
            Some(AI | AL | ID | NU | HY | H2 | H3 | JL | JV | JT | CJ),
            AI | AL | ID | NU | HY | H2 | H3 | JL | JV | JT | CJ,
        ) = (self.strength, state.1, lb)
        {
            mode = Boundary::None;
        }

        // Store the original value, not the modified one.
        self.line_state = (val & !(ALLOWED_BREAK_BIT | MANDATORY_BREAK_BIT), Some(lb));
        mode
    }

    fn next<I>(&mut self, iter: &mut I) -> Option<(Properties, Boundary)>
    where
        I: Iterator + Clone,
        I::Item: Borrow<char>,
    {
        let props = iter.next()?.borrow().properties();
        let mut boundary = self.check_line(props);
        let word = self.check_word(props, iter);
        if boundary as u16 == 0 && word {
            boundary = Boundary::Word;
        }
        self.needs_bidi = self.needs_bidi || props.bidi_class().needs_resolution();
        Some((props, boundary))
    }
}

const ALLOWED_BREAK_BIT: u8 = 0x80;
const MANDATORY_BREAK_BIT: u8 = 0x40;

#[allow(non_upper_case_globals)]
const sot: u8 = 44;

#[rustfmt::skip]
const PAIR_TABLE: [[u8; 44]; 53] = [
    [1,1,130,3,132,5,134,28,8,1,10,11,140,141,14,15,144,145,18,19,148,21,22,151,152,153,26,27,28,29,30,31,32,33,162,1,1,37,38,39,1,41,1,235,],
    [1,1,130,3,132,5,134,28,8,1,10,11,140,141,14,15,144,145,18,19,148,21,22,151,152,153,26,27,28,29,30,31,32,33,162,1,1,37,38,39,1,41,1,235,],
    [129,129,2,3,132,5,134,28,8,2,10,11,140,141,14,15,144,145,146,19,148,149,22,151,152,153,26,27,28,157,158,159,160,33,162,129,129,50,38,39,129,41,2,235,],
    [129,129,130,3,132,5,134,28,8,3,10,11,140,141,14,143,144,145,146,19,148,149,22,151,152,153,26,27,28,157,158,159,160,33,162,129,129,37,38,39,129,41,3,235,],
    [1,1,2,3,4,5,134,28,8,4,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,1,1,37,38,39,1,41,4,235,],
    [193,193,194,195,196,197,198,220,200,193,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,217,218,219,220,221,222,223,224,225,226,193,193,229,230,231,193,233,193,235,],
    [129,129,130,131,132,5,134,156,8,6,10,11,140,141,14,15,144,145,146,147,148,149,22,151,152,153,26,27,156,157,158,159,160,33,162,129,129,37,38,39,129,41,6,235,],
    [129,129,130,3,132,5,134,28,8,28,10,11,140,141,14,15,144,145,146,19,148,149,22,151,152,153,26,27,28,157,158,159,160,33,162,129,129,37,38,39,129,41,28,235,],
    [129,129,130,3,132,5,134,28,8,8,10,11,140,141,14,15,144,145,146,19,148,149,22,151,152,153,26,27,28,157,158,31,32,33,162,129,129,48,38,39,129,41,8,235,],
    [1,1,130,3,132,5,134,28,8,9,10,11,140,141,14,15,144,145,18,19,148,21,22,151,152,153,26,27,28,29,30,31,32,33,162,1,1,37,38,39,1,41,9,235,],
    [1,1,130,3,132,5,134,28,8,10,10,11,140,141,14,15,144,145,18,19,148,149,22,151,152,153,26,27,28,29,158,31,32,33,162,1,1,49,38,39,1,41,10,235,],
    [193,193,194,195,196,197,198,220,200,193,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,217,26,219,220,221,222,223,224,225,226,193,193,229,230,231,193,233,193,235,],
    [129,129,130,3,132,5,134,28,8,12,10,11,140,13,14,15,144,145,146,19,148,21,22,151,152,153,26,27,28,157,158,31,160,33,162,129,129,37,38,39,129,41,12,235,],
    [129,129,130,3,132,5,134,28,8,13,10,11,140,141,14,15,144,145,146,19,148,21,22,151,152,153,26,27,28,157,158,31,160,33,162,129,129,37,38,39,129,41,13,235,],
    [129,129,130,3,132,5,134,28,8,14,10,11,140,141,14,15,144,145,146,19,148,21,22,151,152,153,26,27,28,157,158,159,160,33,162,129,129,37,38,39,129,41,14,235,],
    [1,1,2,3,4,5,6,28,8,15,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,1,1,37,38,39,1,41,15,235,],
    [129,129,130,3,132,5,134,28,8,16,10,11,140,141,14,15,144,145,146,19,148,21,22,151,24,25,26,27,28,157,158,31,160,33,162,129,129,37,38,39,129,41,16,235,],
    [129,129,130,3,132,5,134,28,8,17,10,11,140,141,14,15,144,145,146,19,148,21,22,151,24,153,26,27,28,157,158,31,160,33,162,129,129,37,38,39,129,41,17,235,],
    [1,1,130,51,132,5,134,28,8,18,10,11,140,141,14,15,144,145,18,51,148,21,22,151,152,153,26,27,28,29,30,31,32,33,162,1,1,37,38,39,1,41,18,235,],
    [129,129,130,3,132,5,134,28,8,19,10,11,140,141,14,143,144,145,146,19,148,149,22,151,152,153,26,27,28,29,158,159,160,33,162,129,129,37,38,39,129,41,19,235,],
    [129,129,130,3,132,5,134,28,8,20,10,11,140,141,14,15,144,145,146,19,148,21,22,151,152,153,26,27,28,157,158,31,160,33,162,129,129,37,38,39,129,41,20,235,],
    [129,129,130,3,132,5,134,28,8,21,10,11,140,141,14,15,144,145,146,19,148,21,22,151,152,153,26,27,28,157,158,159,160,33,162,129,129,37,38,39,129,41,21,235,],
    [1,1,130,3,132,5,134,28,8,22,10,11,140,141,14,15,144,145,18,19,148,149,22,151,152,153,26,27,28,29,158,159,160,33,162,1,1,37,38,39,1,41,22,235,],
    [129,129,130,3,132,5,134,28,8,23,10,11,140,141,14,15,16,17,146,19,148,21,22,23,152,25,26,27,28,157,158,31,160,33,162,129,129,37,38,39,129,41,23,235,],
    [129,129,130,3,132,5,134,28,8,24,10,11,140,141,14,15,144,145,146,19,148,21,22,151,24,153,26,27,28,157,158,31,160,33,162,129,129,37,38,39,129,41,24,235,],
    [129,129,130,3,132,5,134,28,8,25,10,11,140,141,14,15,144,145,146,19,148,21,22,151,24,25,26,27,28,157,158,31,160,33,162,129,129,37,38,39,129,41,25,235,],
    [193,193,194,195,196,197,198,220,200,193,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,217,218,219,220,221,222,223,224,225,226,193,193,229,230,231,193,233,193,235,],
    [193,193,194,195,196,197,198,220,200,193,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,217,218,219,220,221,222,223,224,225,226,193,193,229,230,231,193,233,193,235,],
    [129,129,130,3,132,5,134,28,8,28,10,11,140,141,14,15,144,145,146,19,148,149,22,151,152,153,26,27,28,157,158,159,160,33,162,129,129,37,38,39,129,41,28,235,],
    [1,1,130,3,132,5,134,28,8,29,10,11,140,141,14,15,144,145,18,19,148,21,22,151,152,153,26,27,28,29,30,31,32,33,162,1,1,37,38,39,1,41,29,235,],
    [1,1,2,3,4,5,6,28,8,30,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,1,1,46,38,39,1,41,30,235,],
    [1,1,130,3,132,5,134,28,8,31,10,11,140,141,14,15,144,145,18,19,148,149,22,151,152,153,26,27,28,29,30,159,160,33,162,1,1,37,38,39,1,41,31,235,],
    [1,1,130,3,132,5,134,28,8,32,10,11,12,13,14,15,16,17,18,19,20,149,22,23,24,25,26,27,28,29,30,159,160,33,162,1,1,37,38,39,1,41,32,235,],
    [1,1,2,3,4,5,6,28,8,33,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,1,1,47,38,39,1,41,33,235,],
    [129,129,130,3,132,5,134,28,8,34,10,11,140,141,14,15,144,145,146,19,148,149,22,151,152,153,26,27,28,157,158,159,160,33,52,129,129,37,38,39,129,41,34,235,],
    [1,1,130,3,132,5,134,28,8,1,10,11,140,141,14,15,144,145,18,19,148,21,22,151,152,153,26,27,28,29,30,31,32,33,162,1,1,37,38,39,1,41,1,235,],
    [1,1,130,3,132,5,134,28,8,1,10,11,140,141,14,15,144,145,18,19,148,21,22,151,152,153,26,27,28,29,30,31,32,33,162,1,1,37,38,39,1,41,1,235,],
    [129,129,130,131,132,5,134,156,8,129,10,11,140,141,14,143,144,145,146,147,148,149,22,151,152,153,26,27,156,157,158,159,160,161,162,129,129,37,38,39,129,41,129,235,],
    [129,129,130,3,132,5,134,28,8,38,10,11,140,141,14,15,144,145,18,19,148,149,22,151,152,153,26,27,28,29,158,159,160,33,162,129,129,37,38,39,129,41,38,235,],
    [1,1,2,3,4,5,6,28,8,39,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,1,1,37,38,39,1,41,39,235,],
    [1,1,130,3,132,5,134,28,8,1,10,11,140,141,14,15,144,145,18,19,148,21,22,151,152,153,26,27,28,29,30,31,32,33,162,1,1,37,38,39,1,41,1,235,],
    [129,129,130,131,132,5,134,156,136,129,138,11,140,141,142,143,144,145,146,147,148,149,150,151,152,153,26,27,156,157,158,159,160,161,162,129,129,45,166,167,129,41,129,235,],
    [1,1,130,3,132,5,134,28,8,42,10,11,140,141,14,15,144,145,18,19,148,21,22,151,152,153,26,27,28,29,30,31,32,33,162,1,1,37,38,39,1,41,42,235,],
    [129,129,130,3,132,5,134,28,8,129,10,11,140,141,14,143,144,145,146,19,148,149,22,151,152,153,26,27,28,157,158,159,160,33,162,129,129,37,38,39,129,41,129,235,],
    [1,1,2,3,4,5,6,28,8,1,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,1,1,37,38,39,1,41,1,43,],
    [129,129,130,131,132,5,134,156,136,129,138,11,140,141,142,143,144,145,146,147,148,149,150,151,152,153,26,27,156,157,158,159,160,161,162,129,129,45,166,167,129,41,129,235,],
    [1,1,2,3,4,5,6,28,8,1,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,1,1,46,38,39,1,41,1,235,],
    [129,129,130,131,132,5,134,156,8,129,10,11,140,141,14,143,144,145,146,147,148,149,22,151,152,153,26,27,156,157,30,159,160,161,162,129,129,47,38,39,129,41,129,235,],
    [129,129,130,131,132,5,134,28,8,129,10,11,140,141,14,143,144,145,146,147,148,149,22,151,152,153,26,27,28,157,158,159,160,161,162,129,129,48,38,39,129,41,129,235,],
    [129,129,130,131,132,5,134,28,8,129,10,11,140,141,14,143,144,145,146,147,148,149,22,151,152,153,26,27,28,157,158,159,160,161,162,129,129,49,38,39,129,41,129,235,],
    [129,129,2,131,132,5,134,156,8,129,10,11,140,141,14,143,144,145,146,147,148,149,22,151,152,153,26,27,156,157,158,159,160,161,162,129,129,50,38,39,129,41,129,235,],
    [1,1,2,3,4,5,134,28,8,51,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,1,1,37,38,39,1,41,51,235,],
    [129,129,130,3,132,5,134,28,8,52,10,11,140,141,14,15,144,145,146,19,148,149,22,151,152,153,26,27,28,157,158,159,160,33,162,129,129,37,38,39,129,41,52,235,],
];
