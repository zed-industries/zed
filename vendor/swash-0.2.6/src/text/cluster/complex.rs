//! A complex cluster parser based on Microsoft's Universal Shaping Engine
//! specification.

use super::super::{Category, Codepoint, Script};
use super::unicode_data::{ClusterBreak, UseClass};
use super::{CharCluster, Emoji, ShapeClass, Token, Whitespace, MAX_CLUSTER_SIZE};

type Kind = UseClass;

pub struct ComplexState<I> {
    chars: Tokens<I>,
    cur: Token,
    cur_kind: Kind,
    cur_emoji: bool,
    done: bool,
}

impl<I> ComplexState<I>
where
    I: Iterator<Item = Token> + Clone,
{
    pub fn new(script: Script, chars: I) -> Self {
        let mut chars = Tokens::new(script, chars);
        if let Some((first, kind, emoji)) = chars.by_ref().next() {
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
                cur_kind: UseClass::O,
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
    s: &'a mut ComplexState<I>,
    cluster: &'a mut CharCluster,
    vt: bool,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token> + Clone,
{
    fn new(s: &'a mut ComplexState<I>, cluster: &'a mut CharCluster) -> Self {
        Self {
            s,
            cluster,
            vt: false,
        }
    }

    fn parse(&mut self) -> Option<()> {
        use UseClass::*;
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
                // This is not in the USE spec, but added to support uniform
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
            IND | Rsv | WJ => {
                self.accept_any_as(ShapeClass::Base)?;
                self.accept_as(VS, ShapeClass::Vs)?;
            }
            R => {
                self.accept_any_as(ShapeClass::Reph)?;
                self.parse_standard(false)?;
            }
            CS => {
                self.accept_any()?;
                self.parse_standard(false)?;
            }
            B | GB => {
                let is_potential_symbol = self.kind() == GB;
                self.parse_standard(is_potential_symbol)?;
            }
            N => {
                self.accept_any_as(ShapeClass::Base)?;
                self.accept_as(VS, ShapeClass::Vs)?;
                while self.parse_halant_number()? {}
            }
            S => {
                self.accept_any_as(ShapeClass::Base)?;
                self.accept_as(VS, ShapeClass::Vs)?;
                self.accept_zero_or_many(SMAbv)?;
                self.accept_zero_or_many(SMBlw)?;
            }
            _ => {
                self.parse_standard(false)?;
            }
        }
        None
    }

    fn parse_standard(&mut self, is_potential_symbol: bool) -> Option<()> {
        use UseClass::*;
        match self.kind() {
            B | GB => {
                self.accept_any_as(ShapeClass::Base)?;
                self.parse_standard_tail(is_potential_symbol)?;
            }
            _ => {
                self.cluster.info_mut().set_broken();
                self.accept_any_as(self.kind().to_shape_class())?;
            }
        }
        Some(())
    }

    fn parse_standard_tail(&mut self, is_potential_symbol: bool) -> Option<()> {
        use UseClass::*;
        self.accept_as(VS, ShapeClass::Vs)?;
        let k = self.kind();
        if is_potential_symbol && (k == SMAbv || k == SMBlw) {
            self.accept_zero_or_many(SMAbv)?;
            self.accept_zero_or_many(SMBlw)?;
            return Some(());
        }
        self.accept_zero_or_many(CMAbv);
        self.accept_zero_or_many(CMBlw);
        while self.parse_halant_base()? {}
        if self.vt {
            return Some(());
        }
        self.accept(MPre)?;
        self.accept(MAbv)?;
        self.accept(MBlw)?;
        self.accept(MBlw)?;
        self.accept(MPst)?;
        self.accept_zero_or_many_as(VPre, ShapeClass::VPre)?;
        self.accept_zero_or_many(VAbv)?;
        self.accept_zero_or_many(VBlw)?;
        self.accept_zero_or_many(VPst)?;
        while self.parse_vowel_modifier()? {}
        self.accept_zero_or_many(FAbv)?;
        self.accept_zero_or_many(FBlw)?;
        self.accept_zero_or_many(FPst)?;
        self.accept(FM)?;
        Some(())
    }

    fn parse_vowel_modifier(&mut self) -> Option<bool> {
        use UseClass::*;
        Some(match self.kind() {
            VMPre => {
                self.accept_any_as(ShapeClass::VMPre)?;
                true
            }
            VMAbv | VMBlw | VMPst => {
                self.accept_any()?;
                true
            }
            // Spec break: some scripts allow a virama as a vowel modifier and
            // there are particular cases of split vowel characters that
            // decompose into vowel + halant. Accept a halant here, but emit
            // it as Other to avoid any effects on reordering.
            H => {
                self.accept_any()?;
                true
            }
            _ => false,
        })
    }

    fn parse_halant_base(&mut self) -> Option<bool> {
        use UseClass::*;
        self.vt = false;
        match self.kind() {
            SUB => {
                self.accept_any()?;
                self.accept_zero_or_many(CMAbv)?;
                self.accept_zero_or_many(CMBlw)?;
                return Some(true);
            }
            H => {
                self.vt = true;
                if self.s.chars.script == Script::Khmer && self.s.cur.ch == '\u{17d2}' {
                    self.accept_any_as(ShapeClass::Other)?;
                } else {
                    self.accept_any_as(ShapeClass::Halant)?;
                }
                match self.kind() {
                    B => {
                        self.vt = false;
                        self.accept_any_as(ShapeClass::Base)?;
                        self.accept_as(VS, ShapeClass::Vs)?;
                        self.accept_zero_or_many(CMAbv)?;
                        self.accept_zero_or_many(CMBlw)?;
                        return Some(true);
                    }
                    _ => {
                        return Some(false);
                    }
                }
            }
            _ => {}
        }
        Some(false)
    }

    fn parse_halant_number(&mut self) -> Option<bool> {
        use UseClass::*;
        match self.kind() {
            HN => {
                self.accept_any_as(ShapeClass::Halant)?;
                match self.kind() {
                    N => {
                        self.accept_any_as(ShapeClass::Base)?;
                        self.accept_as(VS, ShapeClass::Vs)?;
                        Some(true)
                    }
                    _ => Some(false),
                }
            }
            _ => None,
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
        if let Some((input, kind, emoji)) = self.s.chars.next() {
            self.s.cur = input;
            self.s.cur_emoji = emoji;
            self.s.cur_kind = kind;
            if input.ch == '\u{34f}' {
                self.accept_any_as(ShapeClass::Other)?;
            }
            Some(())
        } else {
            self.s.done = true;
            None
        }
    }
}

impl UseClass {
    pub fn to_shape_class(self) -> ShapeClass {
        match self {
            Self::B => ShapeClass::Base,
            Self::H => ShapeClass::Halant,
            Self::VPre => ShapeClass::VPre,
            Self::VMPre => ShapeClass::VMPre,
            Self::VBlw => ShapeClass::VBlw,
            Self::R => ShapeClass::Reph,
            Self::ZWNJ => ShapeClass::Zwnj,
            Self::ZWJ => ShapeClass::Zwj,
            _ => ShapeClass::Other,
        }
    }
}

#[derive(Clone)]
struct Tokens<I> {
    iter: I,
    decomp: [(Token, UseClass); 3],
    decomp_len: u8,
    decomp_offset: u8,
    script: Script,
}

impl<I> Tokens<I> {
    fn new(script: Script, iter: I) -> Self {
        Self {
            iter,
            decomp: [(Token::default(), UseClass::O); 3],
            decomp_len: 0,
            decomp_offset: 0,
            script,
        }
    }
}

impl<I> Iterator for Tokens<I>
where
    I: Iterator<Item = Token> + Clone,
{
    type Item = (Token, UseClass, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.decomp_offset < self.decomp_len {
            let (input, class) = self.decomp[self.decomp_offset as usize];
            self.decomp_offset += 1;
            Some((input, class, false))
        } else {
            let input = self.iter.next()?;
            let (class, needs_decomp, emoji) = input.info.use_class();
            if needs_decomp {
                self.decomp_offset = 0;
                self.decomp_len = 0;
                for c in input.ch.decompose().chars() {
                    if self.decomp_len == 3 {
                        // shouldn't happen
                        break;
                    }
                    let props = c.properties();
                    let (class, ..) = props.use_class();
                    let c2 = Token {
                        ch: *c,
                        info: input.info.with_properties(props),
                        ..input
                    };
                    self.decomp[self.decomp_len as usize] = (c2, class);
                    self.decomp_len += 1;
                }
                //self.decomp[..self.decomp_len as usize].reverse(); //.sort_unstable_by(|a, b| a.3.cmp(&b.3));
                return self.next();
            } else if self.script == Script::Khmer {
                match input.ch as u32 {
                    0x17BE | 0x17BF | 0x17C0 | 0x17C4 | 0x17C5 => {
                        let a = '\u{17C1}';
                        let props = a.properties();
                        let a_class = props.use_class().0;
                        let a = Token {
                            ch: a,
                            info: input.info.with_properties(props),
                            ..input
                        };
                        self.decomp[0] = (a, a_class);
                        self.decomp[1] = (input, class);
                        self.decomp_len = 2;
                        self.decomp_offset = 0;
                        return self.next();
                    }
                    _ => {}
                }
            }
            Some((input, class, emoji))
        }
    }
}
