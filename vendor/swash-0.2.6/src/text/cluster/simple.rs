//! Simple cluster formation (unicode grapheme cluster algorithm).

use super::super::ClusterBreak;
use super::{CharCluster, Emoji, ShapeClass, Token, Whitespace, MAX_CLUSTER_SIZE};

pub struct SimpleState<I> {
    chars: I,
    cur: Token,
    cur_kind: ClusterBreak,
    cur_emoji: bool,
    done: bool,
}

impl<I> SimpleState<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(mut chars: I) -> Self {
        if let Some(first) = chars.by_ref().next() {
            let (kind, emoji) = first.info.cluster_class();
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
                cur_kind: ClusterBreak::XX,
                cur_emoji: false,
                done: true,
            }
        }
    }

    pub fn next(&mut self, cluster: &mut CharCluster) -> bool {
        if self.done {
            return false;
        }
        Parser { s: self, cluster }.parse();
        true
    }
}

pub struct Parser<'a, I> {
    s: &'a mut SimpleState<I>,
    cluster: &'a mut CharCluster,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    fn parse(&mut self) -> Option<()> {
        use ClusterBreak::*;
        while self.accept(PP)? {}
        if self.emoji() {
            self.cluster.info_mut().set_emoji(Emoji::Default);
            while self.emoji() {
                self.accept_any()?;
                if !self.parse_emoji_extension()? {
                    break;
                }
            }
        } else {
            match self.kind() {
                CN => {
                    self.accept_any_as(ShapeClass::Control)?;
                }
                LF => {
                    self.cluster.info_mut().set_space(Whitespace::Newline);
                    self.accept_any_as(ShapeClass::Control)?;
                }
                CR => {
                    self.cluster.info_mut().set_space(Whitespace::Newline);
                    self.accept_any_as(ShapeClass::Control)?;
                    self.accept_as(LF, ShapeClass::Control)?;
                }
                L => {
                    self.accept_any()?;
                    match self.kind() {
                        L | V | LV | LVT => {
                            self.accept_any()?;
                        }
                        _ => {}
                    }
                }
                LV | V => {
                    self.accept_any()?;
                    match self.kind() {
                        V | T => {
                            self.accept_any()?;
                        }
                        _ => {}
                    }
                }
                LVT | T => {
                    self.accept_any()?;
                    self.accept(T)?;
                }
                RI => {
                    self.accept(RI)?;
                }
                EX | SM | ZWJ => {
                    self.cluster.info_mut().set_broken();
                }
                _ => {
                    self.cluster.info_mut().set_space_from_char(self.s.cur.ch);
                    self.accept_any()?;
                }
            }
        }
        while self.parse_extension()? {}
        Some(())
    }

    fn parse_emoji_extension(&mut self) -> Option<bool> {
        use ClusterBreak::*;
        loop {
            match self.kind() {
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

    fn parse_extension(&mut self) -> Option<bool> {
        use ClusterBreak::*;
        Some(match self.kind() {
            EX => {
                if self.s.cur.ch as u32 == 0x200C {
                    self.accept_any_as(ShapeClass::Zwnj)?;
                } else if self.s.cur.info.is_variation_selector() {
                    self.accept_any_as(ShapeClass::Vs)?;
                } else {
                    self.cluster.force_normalize();
                    self.accept_any_as(ShapeClass::Mark)?;
                }
                true
            }
            SM => {
                self.cluster.force_normalize();
                self.accept_any_as(ShapeClass::Mark)?;
                true
            }
            ZWJ => {
                self.accept_any_as(ShapeClass::Zwj)?;
                true
            }
            _ => false,
        })
    }

    #[inline(always)]
    fn emoji(&self) -> bool {
        self.s.cur_emoji
    }

    #[inline(always)]
    fn kind(&self) -> ClusterBreak {
        self.s.cur_kind
    }

    fn accept(&mut self, kind: ClusterBreak) -> Option<bool> {
        if self.s.cur_kind == kind {
            self.accept_any()?;
            Some(true)
        } else {
            Some(false)
        }
    }

    fn accept_as(&mut self, kind: ClusterBreak, as_kind: ShapeClass) -> Option<bool> {
        if self.s.cur_kind == kind {
            self.accept_any_as(as_kind)?;
            Some(true)
        } else {
            Some(false)
        }
    }

    fn accept_any(&mut self) -> Option<()> {
        self.push_cur();
        self.advance()?;
        Some(())
    }

    fn accept_any_as(&mut self, as_kind: ShapeClass) -> Option<()> {
        self.cluster.push(&self.s.cur, as_kind);
        self.advance()?;
        Some(())
    }

    fn advance(&mut self) -> Option<()> {
        if self.cluster.len() as usize == MAX_CLUSTER_SIZE {
            return None;
        }
        if let Some(input) = self.s.chars.next() {
            let (kind, emoji) = input.info.cluster_class();
            self.s.cur = input;
            self.s.cur_emoji = emoji;
            self.s.cur_kind = kind;
            Some(())
        } else {
            self.s.done = true;
            None
        }
    }

    #[inline]
    fn push_cur(&mut self) {
        self.cluster.push(&self.s.cur, ShapeClass::Base);
    }
}
