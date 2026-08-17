use super::super::{Codepoint as _, JoiningType};
use super::char::{Char, ShapeClass};
use super::token::Token;
use super::{ClusterInfo, UserData};
use crate::GlyphId;

use core::fmt;
use core::ops::Range;

/// The maximum number of characters in a single cluster.
pub const MAX_CLUSTER_SIZE: usize = 32;

/// Character cluster; output from the parser and input to the shaper.
#[derive(Copy, Clone)]
pub struct CharCluster {
    info: ClusterInfo,
    chars: [Char; MAX_CLUSTER_SIZE],
    len: u8,
    map_len: u8,
    start: u32,
    end: u32,
    force_normalize: bool,
    comp: Form,
    decomp: Form,
    form: FormKind,
    best_ratio: f32,
}

impl CharCluster {
    /// Creates a new empty cluster.
    pub fn new() -> Self {
        Self {
            info: ClusterInfo(0),
            chars: [DEFAULT_CHAR; MAX_CLUSTER_SIZE],
            len: 0,
            map_len: 0,
            start: 0,
            end: 0,
            force_normalize: false,
            comp: Form::new(),
            decomp: Form::new(),
            form: FormKind::Original,
            best_ratio: 0.,
        }
    }

    /// Returns the cluster information.
    pub fn info(&self) -> ClusterInfo {
        self.info
    }

    /// Returns the primary user data for the cluster.
    pub fn user_data(&self) -> UserData {
        self.chars[0].data
    }

    /// Returns the source range for the cluster in code units.
    pub fn range(&self) -> SourceRange {
        SourceRange {
            start: self.start,
            end: self.end,
        }
    }

    /// Returns true if the cluster is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the sequence of characters in the cluster.
    pub fn chars(&self) -> &[Char] {
        &self.chars[..self.len as usize]
    }

    /// Returns the currently mapped sequence of characters in the cluster.
    pub fn mapped_chars(&self) -> &[Char] {
        match self.form {
            FormKind::Original => &self.chars[..self.len as usize],
            FormKind::NFD => self.decomp.chars(),
            FormKind::NFC => self.comp.chars(),
        }
    }

    /// Applies a nominal glyph identifier mapping to the cluster, returning
    /// a result indicating the status of the mapping.
    pub fn map(&mut self, f: impl Fn(char) -> GlyphId) -> Status {
        let len = self.len;
        if len == 0 {
            return Status::Complete;
        }
        let mut glyph_ids = [0u16; MAX_CLUSTER_SIZE];
        let prev_ratio = self.best_ratio;
        let mut ratio;
        if self.force_normalize && self.composed().is_some() {
            ratio = self.comp.map(&f, &mut glyph_ids, self.best_ratio);
            if ratio > self.best_ratio {
                self.best_ratio = ratio;
                self.form = FormKind::NFC;
                if ratio >= 1. {
                    return Status::Complete;
                }
            }
        }
        ratio = Mapper {
            chars: &mut self.chars[..self.len as usize],
            map_len: self.map_len.max(1),
        }
        .map(&f, &mut glyph_ids, self.best_ratio);
        if ratio > self.best_ratio {
            self.best_ratio = ratio;
            self.form = FormKind::Original;
            if ratio >= 1. {
                return Status::Complete;
            }
        }
        if len > 1 && self.decomposed().is_some() {
            ratio = self.decomp.map(&f, &mut glyph_ids, self.best_ratio);
            if ratio > self.best_ratio {
                self.best_ratio = ratio;
                self.form = FormKind::NFD;
                if ratio >= 1. {
                    return Status::Complete;
                }
            }
            if !self.force_normalize && self.composed().is_some() {
                ratio = self.comp.map(&f, &mut glyph_ids, self.best_ratio);
                if ratio > self.best_ratio {
                    self.best_ratio = ratio;
                    self.form = FormKind::NFC;
                    if ratio >= 1. {
                        return Status::Complete;
                    }
                }
            }
        }
        if self.best_ratio > prev_ratio {
            Status::Keep
        } else {
            Status::Discard
        }
    }

    /// Resets the cluster to the initial empty state.
    pub fn clear(&mut self) {
        self.info = ClusterInfo(0);
        self.len = 0;
        self.map_len = 0;
        self.start = 0;
        self.end = 0;
        self.force_normalize = false;
        self.comp.clear();
        self.decomp.clear();
        self.form = FormKind::Original;
        self.best_ratio = 0.;
    }

    /// Returns the sequence of decomposed characters for the cluster.
    fn decomposed(&mut self) -> Option<&[Char]> {
        match self.decomp.state {
            FormState::Invalid => None,
            FormState::None => {
                self.decomp.state = FormState::Invalid;
                let mut i = 0;
                for ch in &self.chars[..self.len as usize] {
                    let mut end = i;
                    let mut copy = *ch;
                    for c in ch.ch.decompose() {
                        if end == MAX_CLUSTER_SIZE {
                            return None;
                        }
                        copy.ch = c;
                        self.decomp.chars[end] = copy;
                        end += 1;
                    }
                    i = end;
                }
                if i == 0 {
                    return None;
                }
                self.decomp.len = i as u8;
                self.decomp.state = FormState::Valid;
                self.decomp.setup();
                Some(self.decomp.chars())
            }
            FormState::Valid => Some(self.decomp.chars()),
        }
    }

    /// Returns the sequence of composed characters for the cluster.
    fn composed(&mut self) -> Option<&[Char]> {
        match self.comp.state {
            FormState::Invalid => None,
            FormState::None => {
                if self.decomposed().map(|chars| chars.len()).unwrap_or(0) == 0 {
                    self.comp.state = FormState::Invalid;
                    return None;
                }
                self.comp.state = FormState::Invalid;
                let mut last = self.decomp.chars[0];
                let mut i = 0;
                for ch in &self.decomp.chars()[1..] {
                    if let Some(comp) = char::compose(last.ch, ch.ch) {
                        last.ch = comp;
                    } else {
                        self.comp.chars[i] = last;
                        i += 1;
                        last = *ch;
                    }
                }
                self.comp.chars[i] = last;
                self.comp.len = i as u8 + 1;
                self.comp.state = FormState::Valid;
                self.comp.setup();
                Some(self.comp.chars())
            }
            FormState::Valid => Some(self.comp.chars()),
        }
    }
}

impl Default for CharCluster {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for cluster building.
impl CharCluster {
    pub(super) fn info_mut(&mut self) -> &mut ClusterInfo {
        &mut self.info
    }

    pub(super) fn len(&self) -> u8 {
        self.len
    }

    pub(super) fn force_normalize(&mut self) {
        self.force_normalize = true;
    }

    pub(super) fn push(&mut self, input: &Token, class: ShapeClass) {
        let contributes_to_shaping = input.info.contributes_to_shaping();
        self.chars[self.len as usize] = Char {
            ch: input.ch,
            shape_class: class,
            joining_type: input.info.joining_type(),
            ignorable: input.info.is_ignorable(),
            contributes_to_shaping,
            glyph_id: 0,
            offset: input.offset,
            data: input.data,
        };
        if self.len == 0 {
            self.start = input.offset;
        }
        self.info.merge_boundary(input.info.boundary() as u16);
        self.end = input.offset + input.len as u32;
        self.len += 1;
        self.map_len += contributes_to_shaping as u8;
    }

    /// This function records the attributes and range information for
    /// a character but does not add it to the cluster. It is used when
    /// characters such as emoji variation selectors are dropped from
    /// shaping but should still be included in the cluster range.
    pub(super) fn note_char(&mut self, input: &Token) {
        if self.len == 0 {
            self.start = input.offset;
        }
        self.info.merge_boundary(input.info.boundary() as u16);
        self.end = input.offset + input.len as u32;
    }
}

/// Iterative status of mapping a character cluster to nominal glyph identifiers.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Status {
    /// Mapping should be skipped.
    Discard,
    /// The best mapping so far.
    Keep,
    /// Complete mapping.
    Complete,
}

/// Source range of a cluster in code units.
#[derive(Copy, Clone)]
pub struct SourceRange {
    pub start: u32,
    pub end: u32,
}

impl SourceRange {
    /// Converts the source range into a `usize` range.
    pub fn to_range(self) -> Range<usize> {
        self.start as usize..self.end as usize
    }
}

impl From<SourceRange> for Range<usize> {
    fn from(s: SourceRange) -> Self {
        s.to_range()
    }
}

impl fmt::Debug for SourceRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl fmt::Display for SourceRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
enum FormKind {
    Original,
    NFD,
    NFC,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum FormState {
    None,
    Valid,
    Invalid,
}

#[derive(Copy, Clone)]
struct Form {
    pub chars: [Char; MAX_CLUSTER_SIZE],
    pub len: u8,
    pub map_len: u8,
    pub state: FormState,
}

impl Form {
    fn new() -> Self {
        Self {
            chars: [DEFAULT_CHAR; MAX_CLUSTER_SIZE],
            len: 0,
            map_len: 0,
            state: FormState::None,
        }
    }

    fn clear(&mut self) {
        self.len = 0;
        self.map_len = 0;
        self.state = FormState::None;
    }

    fn chars(&self) -> &[Char] {
        &self.chars[..self.len as usize]
    }

    fn setup(&mut self) {
        self.map_len = (self
            .chars()
            .iter()
            .filter(|c| c.shape_class != ShapeClass::Control)
            .count() as u8)
            .max(1);
    }

    fn map(
        &mut self,
        f: &impl Fn(char) -> u16,
        glyphs: &mut [u16; MAX_CLUSTER_SIZE],
        best_ratio: f32,
    ) -> f32 {
        Mapper {
            chars: &mut self.chars[..self.len as usize],
            map_len: self.map_len,
        }
        .map(f, glyphs, best_ratio)
    }
}

struct Mapper<'a> {
    chars: &'a mut [Char],
    map_len: u8,
}

impl<'a> Mapper<'a> {
    fn map(
        &mut self,
        f: &impl Fn(char) -> u16,
        glyphs: &mut [u16; MAX_CLUSTER_SIZE],
        best_ratio: f32,
    ) -> f32 {
        if self.map_len == 0 {
            return 1.;
        }
        let mut mapped = 0;
        for (c, g) in self.chars.iter().zip(glyphs.iter_mut()) {
            if !c.contributes_to_shaping {
                *g = f(c.ch);
                if self.map_len == 1 {
                    mapped += 1;
                }
            } else {
                let gid = f(c.ch);
                *g = gid;
                if gid != 0 {
                    mapped += 1;
                }
            }
        }
        let ratio = mapped as f32 / self.map_len as f32;
        if ratio > best_ratio {
            for (ch, glyph) in self.chars.iter_mut().zip(glyphs) {
                ch.glyph_id = *glyph;
            }
        }
        ratio
    }
}

const DEFAULT_CHAR: Char = Char {
    ch: ' ',
    shape_class: ShapeClass::Base,
    joining_type: JoiningType::U,
    ignorable: false,
    contributes_to_shaping: true,
    glyph_id: 0,
    data: 0,
    offset: 0,
};
