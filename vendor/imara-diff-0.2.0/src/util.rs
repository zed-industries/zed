use crate::intern::Token;
use crate::Hunk;

pub fn common_prefix(file1: &[Token], file2: &[Token]) -> u32 {
    let mut off = 0;
    for (token1, token2) in file1.iter().zip(file2) {
        if token1 != token2 {
            break;
        }
        off += 1;
    }
    off
}

pub fn common_postfix(file1: &[Token], file2: &[Token]) -> u32 {
    let mut off = 0;
    for (token1, token2) in file1.iter().rev().zip(file2.iter().rev()) {
        if token1 != token2 {
            break;
        }
        off += 1;
    }
    off
}

pub fn common_edges(file1: &[Token], file2: &[Token]) -> (u32, u32) {
    let prefix = common_prefix(file1, file2);
    let postfix = common_postfix(&file1[prefix as usize..], &file2[prefix as usize..]);
    (prefix, postfix)
}

pub fn strip_common_prefix(file1: &mut &[Token], file2: &mut &[Token]) -> u32 {
    let off = common_prefix(file1, file2);
    *file1 = &file1[off as usize..];
    *file2 = &file2[off as usize..];
    off
}

pub fn strip_common_postfix(file1: &mut &[Token], file2: &mut &[Token]) -> u32 {
    let off = common_postfix(file1, file2);
    *file1 = &file1[..file1.len() - off as usize];
    *file2 = &file2[..file2.len() - off as usize];
    off
}

pub fn sqrt(val: usize) -> u32 {
    let nbits = (usize::BITS - val.leading_zeros()) / 2;
    1 << nbits
}

impl Hunk {
    pub(crate) fn next_hunk(&mut self, removed: &[bool], added: &[bool]) -> bool {
        let Some(off) = find_next_change(added, self.after.end) else {
            return false;
        };
        let mut off_before = 0;
        loop {
            debug_assert!(
                removed.len() as u32 != self.before.end || off == 0,
                "broken hunk alignment {self:?} "
            );
            let unchanged_tokens = find_next_change(removed, self.before.end)
                .unwrap_or(removed.len() as u32 - self.before.end);
            if off_before + unchanged_tokens > off {
                self.before.start = self.before.end + (off - off_before);
                self.before.end = self.before.start;
                break;
            }
            off_before += unchanged_tokens;
            self.before.start = self.before.end + unchanged_tokens;
            self.before.end = find_hunk_end(removed, self.before.end + unchanged_tokens);
            if off_before == off {
                break;
            }
        }
        self.after.start = self.after.end + off;
        self.after.end = find_hunk_end(added, self.after.start);
        true
    }
}

pub fn find_next_change(changes: &[bool], pos: u32) -> Option<u32> {
    changes[pos as usize..]
        .iter()
        .position(|&changed| changed)
        .map(|off| off as u32)
}

pub fn find_hunk_end(changes: &[bool], pos: u32) -> u32 {
    pos + changes[pos as usize..]
        .iter()
        .take_while(|&&changed| changed)
        .count() as u32
}

pub fn find_hunk_start(changes: &[bool], pos: u32) -> u32 {
    pos - changes[..pos as usize]
        .iter()
        .rev()
        .take_while(|&&changed| changed)
        .count() as u32
}
