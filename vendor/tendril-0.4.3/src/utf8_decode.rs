// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use fmt;
use tendril::{Atomicity, Tendril};
use utf8;

pub struct IncompleteUtf8(utf8::Incomplete);

impl<A> Tendril<fmt::Bytes, A>
where
    A: Atomicity,
{
    pub fn decode_utf8_lossy<F>(mut self, mut push_utf8: F) -> Option<IncompleteUtf8>
    where
        F: FnMut(Tendril<fmt::UTF8, A>),
    {
        loop {
            if self.is_empty() {
                return None;
            }
            let unborrowed_result = match utf8::decode(&self) {
                Ok(s) => {
                    debug_assert!(s.as_ptr() == self.as_ptr());
                    debug_assert!(s.len() == self.len());
                    Ok(())
                }
                Err(utf8::DecodeError::Invalid {
                    valid_prefix,
                    invalid_sequence,
                    ..
                }) => {
                    debug_assert!(valid_prefix.as_ptr() == self.as_ptr());
                    debug_assert!(valid_prefix.len() <= self.len());
                    Err((
                        valid_prefix.len(),
                        Err(valid_prefix.len() + invalid_sequence.len()),
                    ))
                }
                Err(utf8::DecodeError::Incomplete {
                    valid_prefix,
                    incomplete_suffix,
                }) => {
                    debug_assert!(valid_prefix.as_ptr() == self.as_ptr());
                    debug_assert!(valid_prefix.len() <= self.len());
                    Err((valid_prefix.len(), Ok(incomplete_suffix)))
                }
            };
            match unborrowed_result {
                Ok(()) => {
                    unsafe { push_utf8(self.reinterpret_without_validating()) }
                    return None;
                }
                Err((valid_len, and_then)) => {
                    if valid_len > 0 {
                        let subtendril = self.subtendril(0, valid_len as u32);
                        unsafe { push_utf8(subtendril.reinterpret_without_validating()) }
                    }
                    match and_then {
                        Ok(incomplete) => return Some(IncompleteUtf8(incomplete)),
                        Err(offset) => {
                            push_utf8(Tendril::from_slice(utf8::REPLACEMENT_CHARACTER));
                            self.pop_front(offset as u32)
                        }
                    }
                }
            }
        }
    }
}

impl IncompleteUtf8 {
    pub fn try_complete<A, F>(
        &mut self,
        mut input: Tendril<fmt::Bytes, A>,
        mut push_utf8: F,
    ) -> Result<Tendril<fmt::Bytes, A>, ()>
    where
        A: Atomicity,
        F: FnMut(Tendril<fmt::UTF8, A>),
    {
        let resume_at;
        match self.0.try_complete(&input) {
            None => return Err(()),
            Some((result, rest)) => {
                push_utf8(Tendril::from_slice(
                    result.unwrap_or(utf8::REPLACEMENT_CHARACTER),
                ));
                resume_at = input.len() - rest.len();
            }
        }
        input.pop_front(resume_at as u32);
        Ok(input)
    }
}
