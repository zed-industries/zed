use shingles::Shingles;
use itertools::Itertools;
use crate::text::multi_shingles::MultiShingles;

pub fn whitespace_split<'a>(text: &'a str) -> impl Iterator<Item = &'a str> {
    text
        .split(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace())
        .filter(|&x| !x.is_empty())
}

pub fn whitespace_split_boxed<'a>(text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(text
        .split(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace())
        .filter(|&x| !x.is_empty()))
}

pub fn shingle_text<'a>(text: &'a str, size: usize) -> impl Iterator<Item = &'a str> {
    Shingles::new(text, size)
}


pub fn shingle_text_range<'a>(text: &'a str, from: usize, to: usize) -> impl Iterator<Item = &'a str> {
    MultiShingles::new(text, from, to)
}

pub fn shingle_text_boxed<'a>(text: &'a str, size: usize) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    Box::new(Shingles::new(text, size))
}


pub fn shingle_tokens<'a>(tokens: &'a Vec<&'a str>, size: usize) -> impl Iterator<Item = String> {
    Shingles::new(tokens.as_slice(), size)
        .into_iter().map(|tokens| tokens.join(""))
        .collect_vec()
        .into_iter()
}




