//! Sound card enumeration
use libc::{c_int, c_char};
use super::error::*;
use crate::alsa;
use std::ffi::CStr;

/// An ALSA sound card, uniquely identified by its index.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card(c_int);

/// Iterate over existing sound cards.
pub struct Iter(c_int);

impl Iter {
    pub fn new() -> Iter { Iter(-1) }
}

impl Iterator for Iter {
    type Item = Result<Card>;

    fn next(&mut self) -> Option<Result<Card>> {
        match acheck!(snd_card_next(&mut self.0)) {
            Ok(_) if self.0 == -1 => None,
            Ok(_) => Some(Ok(Card(self.0))),
            Err(e) => Some(Err(e)),
        }
    }
}

impl Card {
    pub fn new(index: c_int) -> Card { Card(index) }
    pub fn from_str(s: &CStr) -> Result<Card> {
        acheck!(snd_card_get_index(s.as_ptr())).map(Card)
    }
    pub fn get_name(&self) -> Result<String> {
        let mut c: *mut c_char = ::std::ptr::null_mut();
        acheck!(snd_card_get_name(self.0, &mut c))
            .and_then(|_| from_alloc("snd_card_get_name", c)) 
    }
    pub fn get_longname(&self) -> Result<String> {
        let mut c: *mut c_char = ::std::ptr::null_mut();
        acheck!(snd_card_get_longname(self.0, &mut c))
            .and_then(|_| from_alloc("snd_card_get_longname", c)) 
    }

    pub fn get_index(&self) -> c_int { self.0 }
}

#[test]
fn print_cards() {
    for a in Iter::new().map(|a| a.unwrap()) {
        println!("Card #{}: {} ({})", a.get_index(), a.get_name().unwrap(), a.get_longname().unwrap())
    }
}
