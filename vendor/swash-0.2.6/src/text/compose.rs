use super::unicode_data::{
    compose_index, decompose_compat_index, decompose_index, COMPOSE0, COMPOSE1, COMPOSE1_COUNT,
    DECOMPOSE, DECOMPOSE_COMPAT,
};
use core::char::from_u32_unchecked;

/// Decomposition of a character.
#[derive(Copy, Clone)]
pub struct Decompose {
    inner: DecomposeInner,
    len: u8,
    cur: u8,
}

impl Decompose {
    /// Returns the sequence of characters that represent the
    /// decomposition.
    pub fn chars(&self) -> &[char] {
        match self.inner {
            DecomposeInner::Slice(chars) => chars,
            DecomposeInner::Array(ref chars, len) => &chars[..len as usize],
        }
    }
}

impl Iterator for Decompose {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.len {
            return None;
        }
        let item = self.chars()[self.cur as usize];
        self.cur += 1;
        Some(item)
    }
}

#[derive(Copy, Clone)]
enum DecomposeInner {
    Slice(&'static [char]),
    Array([char; 3], u32),
}

impl DecomposeInner {
    fn len(&self) -> u8 {
        match self {
            Self::Slice(s) => s.len() as u8,
            Self::Array(_, len) => *len as u8,
        }
    }
}

impl From<DecomposeInner> for Decompose {
    fn from(inner: DecomposeInner) -> Self {
        Self {
            inner,
            len: inner.len(),
            cur: 0,
        }
    }
}

pub fn compose_pair(a: char, b: char) -> Option<char> {
    if let Some(c) = compose_hangul(a, b) {
        return Some(c);
    }
    let l = pair_index(a as u32, &COMPOSE0[..])?;
    let r = pair_index(b as u32, &COMPOSE1[..])?;
    let c = compose_index(l * COMPOSE1_COUNT + r) as u32;
    if c != 0 {
        return Some(unsafe { core::char::from_u32_unchecked(c) });
    }
    None
}

fn pair_index(c: u32, table: &[(u32, u16, u16)]) -> Option<usize> {
    let c = c as usize;
    for entry in table {
        let start = entry.0 as usize;
        if start == 0 || c < start {
            return None;
        }
        let end = start + entry.1 as usize;
        if c <= end {
            return Some(entry.2 as usize + (c - start));
        }
    }
    None
}

const LBASE: u32 = 0x1100;
const VBASE: u32 = 0x1161;
const TBASE: u32 = 0x11A7;
const LCOUNT: u32 = 19;
const VCOUNT: u32 = 21;
const TCOUNT: u32 = 28;
const SBASE: u32 = 0xAC00;
const NCOUNT: u32 = VCOUNT * TCOUNT;
const SCOUNT: u32 = LCOUNT * NCOUNT;

fn is_hangul(c: char) -> bool {
    let c = c as u32;
    (SBASE..(SBASE + SCOUNT)).contains(&c)
}

fn compose_hangul(a: char, b: char) -> Option<char> {
    let a = a as u32;
    let b = b as u32;
    if !(VBASE..(TBASE + TCOUNT)).contains(&b) {
        return None;
    }
    if !(LBASE..(LBASE + LCOUNT)).contains(&a) && !(SBASE..(SBASE + SCOUNT)).contains(&a) {
        return None;
    }
    if a >= SBASE {
        if (a - SBASE) % TCOUNT == 0 {
            Some(unsafe { from_u32_unchecked(a + (b - TBASE)) })
        } else {
            None
        }
    } else {
        let li = a - LBASE;
        let vi = b - VBASE;
        Some(unsafe { from_u32_unchecked(SBASE + li * NCOUNT + vi * TCOUNT) })
    }
}

fn decompose_hangul(c: char) -> DecomposeInner {
    let si = c as u32 - SBASE;
    let li = si / NCOUNT;
    let mut chars = [' '; 3];
    let mut len = 2;
    unsafe {
        chars[0] = from_u32_unchecked(LBASE + li);
        let vi = (si % NCOUNT) / TCOUNT;
        chars[1] = from_u32_unchecked(VBASE + vi);
        let ti = si % TCOUNT;
        if ti > 0 {
            chars[2] = from_u32_unchecked(TBASE + ti);
            len += 1;
        }
    }
    DecomposeInner::Array(chars, len)
}

pub fn decompose(c: char) -> Decompose {
    if c <= '\x7F' {
        DecomposeInner::Array([c, ' ', ' '], 1).into()
    } else if is_hangul(c) {
        decompose_hangul(c).into()
    } else {
        let index = decompose_index(c as usize);
        if index == 0 {
            DecomposeInner::Array([c, ' ', ' '], 1).into()
        } else {
            let buf = &DECOMPOSE[index..];
            let end = 1 + buf[0] as usize;
            DecomposeInner::Slice(unsafe { &*(&buf[1..end] as *const [u32] as *const [char]) })
                .into()
        }
    }
}

pub fn decompose_compat(c: char) -> Decompose {
    if c <= '\x7F' {
        DecomposeInner::Array([c, ' ', ' '], 1).into()
    } else if is_hangul(c) {
        decompose_hangul(c).into()
    } else {
        let index = decompose_compat_index(c as usize);
        if index == 0 {
            DecomposeInner::Array([c, ' ', ' '], 1).into()
        } else if index == 1 {
            let index = decompose_index(c as usize);
            let buf = &DECOMPOSE[index..];
            let end = 1 + buf[0] as usize;
            DecomposeInner::Slice(unsafe { &*(&buf[1..end] as *const [u32] as *const [char]) })
                .into()
        } else {
            let buf = &DECOMPOSE_COMPAT[index..];
            let end = 1 + buf[0] as usize;
            DecomposeInner::Slice(unsafe { &*(&buf[1..end] as *const [u32] as *const [char]) })
                .into()
        }
    }
}
