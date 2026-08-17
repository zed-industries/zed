//! Parser for Myanmar clusters.

use super::unicode_data::{Category, ClusterBreak, MyanmarClass};
use super::{CharCluster, Emoji, ShapeClass, Token, Whitespace, MAX_CLUSTER_SIZE};

type Kind = MyanmarClass;

pub struct MyanmarState<I> {
    chars: I,
    cur: Token,
    cur_kind: Kind,
    cur_emoji: bool,
    done: bool,
}

impl<I> MyanmarState<I>
where
    I: Iterator<Item = Token> + Clone,
{
    pub fn new(mut chars: I) -> Self {
        if let Some(first) = chars.by_ref().next() {
            let (kind, emoji) = first.info.myanmar_class();
            Self {
                chars,
                cur: first,
                cur_kind: kind,
                cur_emoji: emoji,
                done: false,
            }
        } else {
            Self {
                chars,
                cur: Token::default(),
                cur_kind: MyanmarClass::O,
                cur_emoji: false,
                done: true,
            }
        }
    }

    pub fn next(&mut self, cluster: &mut CharCluster) -> bool {
        if self.done {
            return false;
        }
        Parser::new(self, cluster).parse();
        true
    }
}

struct Parser<'a, I> {
    s: &'a mut MyanmarState<I>,
    cluster: &'a mut CharCluster,
    vt: bool,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token> + Clone,
{
    fn new(s: &'a mut MyanmarState<I>, cluster: &'a mut CharCluster) -> Self {
        Self {
            s,
            cluster,
            vt: false,
        }
    }

    fn parse(&mut self) -> Option<()> {
        use MyanmarClass::*;
        if self.s.done {
            return Some(());
        }
        if self.emoji() {
            self.cluster.info_mut().set_emoji(Emoji::Default);
            while self.emoji() {
                self.accept_any_as(ShapeClass::Base)?;
                if !self.parse_emoji_extension()? {
                    break;
                }
            }
            return Some(());
        }
        match self.kind() {
            O => {
                // This is not in the Myanmar spec, but added to support uniform
                // clustering of CRLF across the parsers.
                match self.s.cur.ch {
                    '\r' => {
                        self.cluster.info_mut().set_space(Whitespace::Newline);
                        self.accept_any_as(ShapeClass::Control)?;
                        if self.s.cur.ch == '\n' {
                            self.accept_any_as(ShapeClass::Control)?;
                        }
                    }
                    '\n' => {
                        self.cluster.info_mut().set_space(Whitespace::Newline);
                        self.accept_any_as(ShapeClass::Control)?;
                    }
                    _ => {
                        self.cluster.info_mut().set_space_from_char(self.s.cur.ch);
                        let class = match self.s.cur.info.category() {
                            Category::Format => match self.s.cur.ch as u32 {
                                0x200C => ShapeClass::Zwnj,
                                0x200D => ShapeClass::Zwj,
                                _ => ShapeClass::Control,
                            },
                            Category::Control => ShapeClass::Control,
                            _ => ShapeClass::Base,
                        };
                        self.accept_any_as(class)?;
                    }
                }
            }
            P | S | R | WJ | D0 => {
                self.accept_any()?;
            }
            _ => {
                match self.s.cur.ch as u32 {
                    0x1004 | 0x101B | 0x105A => {
                        let mut iter = self.s.chars.clone();
                        if let Some(b) = iter.next() {
                            if b.ch == '\u{103A}' {
                                if let Some(c) = iter.next() {
                                    if c.ch == '\u{1039}' {
                                        self.cluster.push(&self.s.cur, ShapeClass::Kinzi);
                                        self.cluster.push(&b, ShapeClass::Kinzi);
                                        self.cluster.push(&c, ShapeClass::Kinzi);
                                        self.advance();
                                        self.advance();
                                        self.advance();
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                match self.kind() {
                    C | IV | D | DB => {
                        self.accept_any_as(ShapeClass::Base)?;
                        self.accept_as(VS, ShapeClass::Vs)?;
                        while self.parse_stacked_consonant_or_vowel()? {}
                        if self.vt {
                            return Some(());
                        }
                        self.accept_zero_or_many(As)?;
                        if self.accept(MY)? {
                            self.accept(As)?;
                        }
                        self.accept_as(MR, ShapeClass::MedialRa)?;
                        if self.accept(MW)? {
                            self.accept(MH)?;
                            self.accept(As)?;
                        } else if self.accept(MH)? {
                            self.accept(As)?;
                        }
                        self.accept_zero_or_many_as(VPre, ShapeClass::VPre)?;
                        self.accept_zero_or_many(VAbv)?;
                        self.accept_zero_or_many_as(VBlw, ShapeClass::VBlw)?;
                        self.accept_zero_or_many_as(A, ShapeClass::Anusvara)?;
                        if self.accept(DB)? {
                            self.accept(As)?;
                        }
                        while self.parse_post_base_vowel()? {}
                        while self.parse_pwo_tone_mark()? {}
                        self.accept_zero_or_many(V)?;
                        self.accept(J)?;
                        return Some(());
                    }
                    _ => {
                        self.cluster.info_mut().set_broken();
                        self.accept_any()?;
                        return Some(());
                    }
                }
            }
        }
        None
    }

    fn parse_stacked_consonant_or_vowel(&mut self) -> Option<bool> {
        use MyanmarClass::*;
        match self.kind() {
            H => {
                self.vt = true;
                self.accept_any_as(ShapeClass::Halant)?;
                match self.kind() {
                    C | IV => {
                        self.vt = false;
                        self.accept_any_as(ShapeClass::Base)?;
                        self.accept_as(VS, ShapeClass::Vs)?;
                        Some(true)
                    }
                    _ => Some(false),
                }
            }
            _ => Some(false),
        }
    }

    fn parse_post_base_vowel(&mut self) -> Option<bool> {
        use MyanmarClass::*;
        match self.kind() {
            VPst => {
                self.accept_any()?;
                self.accept(MH)?;
                self.accept_zero_or_many(As)?;
                self.accept_zero_or_many(VAbv)?;
                self.accept_zero_or_many_as(A, ShapeClass::Anusvara)?;
                if self.accept(DB)? {
                    self.accept(As)?;
                }
                Some(true)
            }
            _ => Some(false),
        }
    }

    fn parse_pwo_tone_mark(&mut self) -> Option<bool> {
        use MyanmarClass::*;
        match self.kind() {
            PT => {
                self.accept_any()?;
                if self.accept(As)? {
                    self.accept_as(A, ShapeClass::Anusvara)?;
                } else {
                    // This goes against the spec, but seems to be necessary to handle the actual
                    // example of a complex cluster here:
                    // https://docs.microsoft.com/en-us/typography/script-development/myanmar#well-formed-clusters
                    self.accept_zero_or_many_as(A, ShapeClass::Anusvara)?; // self.accept(A)?;
                    self.accept(DB)?;
                    self.accept(As)?;
                }
                Some(true)
            }
            _ => Some(false),
        }
    }

    fn parse_emoji_extension(&mut self) -> Option<bool> {
        use ClusterBreak::*;
        loop {
            match self.s.cur.info.cluster_break() {
                EX => match self.s.cur.ch as u32 {
                    0x200C => self.accept_any_as(ShapeClass::Zwnj)?,
                    0xFE0F => {
                        self.cluster.info_mut().set_emoji(Emoji::Color);
                        self.cluster.note_char(&self.s.cur);
                        self.advance()?;
                    }
                    0xFE0E => {
                        self.cluster.info_mut().set_emoji(Emoji::Text);
                        self.cluster.note_char(&self.s.cur);
                        self.advance()?;
                    }
                    _ => self.accept_any_as(ShapeClass::Mark)?,
                },
                ZWJ => {
                    self.accept_any_as(ShapeClass::Zwj)?;
                    return Some(true);
                }
                _ => break,
            }
        }
        Some(false)
    }

    #[inline(always)]
    fn emoji(&self) -> bool {
        self.s.cur_emoji
    }

    #[inline(always)]
    fn kind(&self) -> Kind {
        self.s.cur_kind
    }

    fn accept(&mut self, kind: Kind) -> Option<bool> {
        self.accept_as(kind, ShapeClass::Other)
    }

    fn accept_as(&mut self, kind: Kind, as_class: ShapeClass) -> Option<bool> {
        if self.s.cur_kind == kind {
            self.accept_any_as(as_class)?;
            Some(true)
        } else {
            Some(false)
        }
    }

    fn accept_zero_or_many(&mut self, kind: Kind) -> Option<bool> {
        let mut some = false;
        while self.accept(kind)? {
            some = true;
        }
        Some(some)
    }

    fn accept_zero_or_many_as(&mut self, kind: Kind, as_class: ShapeClass) -> Option<bool> {
        let mut some = false;
        while self.accept_as(kind, as_class)? {
            some = true;
        }
        Some(some)
    }

    fn accept_any(&mut self) -> Option<()> {
        self.cluster.push(&self.s.cur, ShapeClass::Other);
        self.advance()?;
        Some(())
    }

    fn accept_any_as(&mut self, as_class: ShapeClass) -> Option<()> {
        self.cluster.push(&self.s.cur, as_class);
        self.advance()?;
        Some(())
    }

    fn advance(&mut self) -> Option<()> {
        if self.cluster.len() as usize == MAX_CLUSTER_SIZE {
            return None;
        }
        if let Some(input) = self.s.chars.next() {
            let (kind, emoji) = input.info.myanmar_class();
            self.s.cur = input;
            self.s.cur_emoji = emoji;
            self.s.cur_kind = kind;
            if input.ch == '\u{34f}' {
                self.accept_any()?;
            }
            Some(())
        } else {
            self.s.done = true;
            None
        }
    }
}
