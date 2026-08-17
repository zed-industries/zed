use std::ops::Range;

use crate::domains::find_authority_end;
use crate::scanner::Scanner;

/// Scan for email address starting from the trigger character "@".
///
/// Based on RFC 6531, but also accepts invalid IDNs. Doesn't try to handle IP addresses in domain part or
/// quoting in local part.
pub struct EmailScanner {
    pub domain_must_have_dot: bool,
}

impl Scanner for EmailScanner {
    fn scan(&self, s: &str, at: usize) -> Option<Range<usize>> {
        if let Some(start) = self.find_start(&s[0..at]) {
            let after = at + 1;
            if let Some(end) = self.find_end(&s[after..]) {
                let range = Range {
                    start,
                    end: after + end,
                };
                return Some(range);
            }
        }
        None
    }
}

impl EmailScanner {
    // See "Local-part" in RFC 5321, plus extensions in RFC 6531
    fn find_start(&self, s: &str) -> Option<usize> {
        let mut first = None;
        let mut atom_boundary = true;
        for (i, c) in s.char_indices().rev() {
            if Self::local_atom_allowed(c) {
                first = Some(i);
                atom_boundary = false;
            } else if c == '.' {
                if atom_boundary {
                    break;
                }
                atom_boundary = true;
            } else if c == '@' {
                // In `@me@a.com`, we don't want to extract `me@a.com`.
                return None;
            } else {
                break;
            }
        }
        first
    }

    // See "Domain" in RFC 5321, plus extension of "sub-domain" in RFC 6531
    fn find_end(&self, s: &str) -> Option<usize> {
        if let (Some(end), last_dot) = find_authority_end(s, false, true, false, true) {
            if !self.domain_must_have_dot || last_dot.is_some() {
                Some(end)
            } else {
                None
            }
        } else {
            None
        }
    }

    // See "Atom" in RFC 5321, "atext" in RFC 5322
    fn local_atom_allowed(c: char) -> bool {
        match c {
            'a'..='z'
            | 'A'..='Z'
            | '0'..='9'
            | '!'
            | '#'
            | '$'
            | '%'
            | '&'
            | '\''
            | '*'
            | '+'
            | '-'
            | '/'
            | '='
            | '?'
            | '^'
            | '_'
            | '`'
            | '{'
            | '|'
            | '}'
            | '~' => true,
            _ => c >= '\u{80}',
        }
    }
}
