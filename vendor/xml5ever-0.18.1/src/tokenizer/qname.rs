// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum QualNameState {
    BeforeName,
    InName,
    AfterColon,
}

pub struct QualNameTokenizer<'a> {
    state: QualNameState,
    slice: &'a [u8],
    valid_index: Option<u32>,
    curr_ind: usize,
}

impl<'a> QualNameTokenizer<'a> {
    pub fn new(tag: &[u8]) -> QualNameTokenizer {
        QualNameTokenizer {
            state: QualNameState::BeforeName,
            slice: tag,
            valid_index: None,
            curr_ind: 0,
        }
    }

    pub fn run(&mut self) -> Option<u32> {
        if !self.slice.is_empty() {
            loop {
                if !self.step() {
                    break;
                }
            }
        }
        self.valid_index
    }

    fn incr(&mut self) -> bool {
        if self.curr_ind + 1 < self.slice.len() {
            self.curr_ind += 1;
            return true;
        }
        false
    }

    fn step(&mut self) -> bool {
        match self.state {
            QualNameState::BeforeName => self.do_before_name(),
            QualNameState::InName => self.do_in_name(),
            QualNameState::AfterColon => self.do_after_colon(),
        }
    }

    fn do_before_name(&mut self) -> bool {
        if self.slice[self.curr_ind] == b':' {
            false
        } else {
            self.state = QualNameState::InName;
            self.incr()
        }
    }

    fn do_in_name(&mut self) -> bool {
        if self.slice[self.curr_ind] == b':' && self.curr_ind + 1 < self.slice.len() {
            self.valid_index = Some(self.curr_ind as u32);
            self.state = QualNameState::AfterColon;
        }
        self.incr()
    }

    fn do_after_colon(&mut self) -> bool {
        if self.slice[self.curr_ind] == b':' {
            self.valid_index = None;
            return false;
        }
        self.incr()
    }
}
