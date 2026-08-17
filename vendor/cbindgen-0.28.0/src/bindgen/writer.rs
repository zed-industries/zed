/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cmp;
use std::io;
use std::io::Write;

use crate::bindgen::config::{Braces, Language};
use crate::bindgen::language_backend::LanguageBackend;
use crate::bindgen::Bindings;

/// A type of way to format a list.
pub enum ListType<'a> {
    /// Join each adjacent item with a str.
    Join(&'a str),
    /// End each item with a str.
    Cap(&'a str),
}

/// A utility wrapper to write unbuffered data and correctly adjust positions.
struct InnerWriter<'a, 'b: 'a, F: 'a + Write>(&'a mut SourceWriter<'b, F>);

impl<F: Write> Write for InnerWriter<'_, '_, F> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let writer = &mut self.0;

        if !writer.line_started {
            for _ in 0..writer.spaces() {
                write!(writer.out, " ").unwrap();
            }
            writer.line_started = true;
            writer.line_length += writer.spaces();
        }

        let written = writer.out.write(buf)?;
        writer.line_length += written;
        writer.max_line_length = cmp::max(writer.max_line_length, writer.line_length);
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.out.flush()
    }
}

/// A utility writer for generating code easier.
pub struct SourceWriter<'a, F: Write> {
    out: F,
    bindings: &'a Bindings,
    spaces: Vec<usize>,
    line_started: bool,
    line_length: usize,
    line_number: usize,
    max_line_length: usize,
}

pub type MeasureWriter<'a> = SourceWriter<'a, &'a mut Vec<u8>>;

impl<'a, F: Write> SourceWriter<'a, F> {
    pub fn new(out: F, bindings: &'a Bindings) -> Self {
        SourceWriter {
            out,
            bindings,
            spaces: vec![0],
            line_started: false,
            line_length: 0,
            line_number: 1,
            max_line_length: 0,
        }
    }

    pub fn bindings(&self) -> &Bindings {
        self.bindings
    }

    /// Takes a function that writes source and returns the maximum line length
    /// written.
    pub fn try_write<T>(&mut self, func: T, max_line_length: usize) -> bool
    where
        T: FnOnce(&mut MeasureWriter),
    {
        if self.line_length > max_line_length {
            return false;
        }

        let mut buffer = Vec::new();
        let line_length = {
            let mut measurer = SourceWriter {
                out: &mut buffer,
                bindings: self.bindings,
                spaces: self.spaces.clone(),
                line_started: self.line_started,
                line_length: self.line_length,
                line_number: self.line_number,
                max_line_length: self.line_length,
            };

            func(&mut measurer);

            measurer.max_line_length
        };

        if line_length > max_line_length {
            return false;
        }
        // We don't want the extra alignment, it's already accounted for by the
        // measurer.
        self.line_started = true;
        InnerWriter(self).write_all(&buffer).unwrap();
        true
    }

    fn spaces(&self) -> usize {
        *self.spaces.last().unwrap()
    }

    pub fn push_set_spaces(&mut self, spaces: usize) {
        self.spaces.push(spaces);
    }

    pub fn pop_set_spaces(&mut self) {
        self.pop_tab()
    }

    pub fn line_length_for_align(&self) -> usize {
        if self.line_started {
            self.line_length
        } else {
            self.line_length + self.spaces()
        }
    }

    pub fn push_tab(&mut self) {
        let spaces = self.spaces() - (self.spaces() % self.bindings.config.tab_width)
            + self.bindings.config.tab_width;
        self.spaces.push(spaces);
    }

    pub fn pop_tab(&mut self) {
        assert!(!self.spaces.is_empty());
        self.spaces.pop();
    }

    pub fn new_line(&mut self) {
        self.out
            .write_all(self.bindings.config.line_endings.as_str().as_bytes())
            .unwrap();
        self.line_started = false;
        self.line_length = 0;
        self.line_number += 1;
    }

    pub fn new_line_if_not_start(&mut self) {
        if self.line_number != 1 {
            self.new_line();
        }
    }

    pub fn open_brace(&mut self) {
        match self.bindings.config.language {
            Language::Cxx | Language::C => match self.bindings.config.braces {
                Braces::SameLine => {
                    self.write(" {");
                    self.push_tab();
                    self.new_line();
                }
                Braces::NextLine => {
                    self.new_line();
                    self.write("{");
                    self.push_tab();
                    self.new_line();
                }
            },
            Language::Cython => {
                self.write(":");
                self.new_line();
                self.push_tab();
            }
        }
    }

    pub fn close_brace(&mut self, semicolon: bool) {
        self.pop_tab();
        match self.bindings.config.language {
            Language::Cxx | Language::C => {
                self.new_line();
                if semicolon {
                    self.write("};");
                } else {
                    self.write("}");
                }
            }
            Language::Cython => {}
        }
    }

    pub fn write(&mut self, text: &'static str) {
        write!(self, "{}", text);
    }

    pub fn write_raw_block(&mut self, block: &str) {
        self.line_started = true;
        write!(self, "{}", block);
    }

    pub fn write_fmt(&mut self, fmt: ::std::fmt::Arguments) {
        InnerWriter(self).write_fmt(fmt).unwrap();
    }

    pub fn write_horizontal_source_list<
        LB: LanguageBackend,
        S,
        WF: Fn(&mut LB, &mut SourceWriter<F>, &S),
    >(
        &mut self,
        language_backend: &mut LB,
        items: &[S],
        list_type: ListType<'_>,
        writer: WF,
    ) {
        for (i, item) in items.iter().enumerate() {
            writer(language_backend, self, item);

            match list_type {
                ListType::Join(text) => {
                    if i != items.len() - 1 {
                        write!(self, "{}", text);
                    }
                }
                ListType::Cap(text) => {
                    write!(self, "{}", text);
                }
            }
        }
    }

    pub fn write_vertical_source_list<
        LB: LanguageBackend,
        S,
        WF: Fn(&mut LB, &mut SourceWriter<F>, &S),
    >(
        &mut self,
        language_backend: &mut LB,
        items: &[S],
        list_type: ListType<'_>,
        writer: WF,
    ) {
        let align_length = self.line_length_for_align();
        self.push_set_spaces(align_length);
        for (i, item) in items.iter().enumerate() {
            writer(language_backend, self, item);

            match list_type {
                ListType::Join(text) => {
                    if i != items.len() - 1 {
                        write!(self, "{}", text);
                    }
                }
                ListType::Cap(text) => {
                    write!(self, "{}", text);
                }
            }

            if i != items.len() - 1 {
                self.new_line();
            }
        }
        self.pop_tab();
    }
}
