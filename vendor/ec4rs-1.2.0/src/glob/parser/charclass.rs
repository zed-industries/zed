use std::collections::BTreeSet;

use super::Chars;
use crate::glob::{Glob, Matcher};

#[inline]
fn grow_char_class(chars: &mut Chars<'_>, charclass: &mut BTreeSet<char>) -> Option<()> {
    // Previous character.
    let mut pc = '[';
    let mut not_at_start = false;
    loop {
        match chars.next()? {
            ']' => return Some(()),
            '\\' => {
                pc = chars.next()?;
                charclass.insert(pc);
            }
            // The spec says nothing about char ranges,
            // but the test suite tests for them.
            // Therefore, EC has them in practice.
            '-' if not_at_start => {
                let nc = match chars.next()? {
                    ']' => {
                        charclass.insert('-');
                        return Some(());
                    }
                    '\\' => chars.next()?,
                    other => other,
                };
                charclass.extend(pc..=nc);
                pc = nc;
            }
            c => {
                charclass.insert(c);
                pc = c;
            }
        }
        not_at_start = true;
    }
}

pub fn parse(mut glob: Glob, mut chars: Chars<'_>) -> (Glob, Chars<'_>) {
    let invert = if let Some(c) = chars.peek() {
        *c == '!'
    } else {
        glob.append_char('[');
        return (glob, chars);
    };
    let restore = chars.clone();
    if invert {
        chars.next();
    }
    let mut charclass = BTreeSet::<char>::new();
    if grow_char_class(&mut chars, &mut charclass).is_some() {
        // Remove slashes for the sake of consistent behavior.
        charclass.remove(&'/');
        match charclass.len() {
            0 => {
                if invert {
                    glob.append(Matcher::AnyChar);
                } else {
                    glob.append_char('[');
                    glob.append_char(']');
                }
            }
            // Don't use BTreeSet::first here (stable: 1.66).
            1 => glob.append_char(*charclass.iter().next().unwrap()),
            _ => glob.append(Matcher::CharClass(charclass.into(), !invert)),
        }
        (glob, chars)
    } else {
        glob.append_char('[');
        (glob, restore)
    }
}
