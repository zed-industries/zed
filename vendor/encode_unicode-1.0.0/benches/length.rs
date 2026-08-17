/* Copyright 2018-2022 Torbjørn Birch Moltu
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */

// Run with -- --nocapture to show error messages if setup fails.
// (or use ./do.sh)

#![cfg(feature="std")]
#![feature(test)]
extern crate test;
use test::{Bencher, black_box};

use std::fs;
use std::path::Path;
use std::io::ErrorKind;
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
extern crate minreq;
#[macro_use] extern crate lazy_static;
extern crate encode_unicode;
use encode_unicode::{CharExt, Utf8Char, U8UtfExt, Utf16Char, U16UtfExt};

// Setup; need longish strings to make benchmarks representative and
// reduce overhead (might get cache misses now though)
// Therefore we download a few wikipedia articles in different languages.
// Downloading a fixed revision of the articles doesn't prevent the HTML from
// changing due to changes in templates or rendering.

fn load_wikipedia(language: &str,  article: &str,  english: &str,  revision: usize) -> String {
    let cache_path = Path::new("benches").join("texts");
    let cache_path = cache_path.to_str().unwrap();
    let name = format!("{}_{}.html", language, english);
    let path = Path::new(cache_path).join(&name);
    let path = path.to_str().unwrap();
    match fs::read_to_string(path) {
        Ok(content) => return content,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {},//continue
        Err(ref e) if e.kind() == ErrorKind::InvalidData => {
            panic!("{} exists but is not UTF-8", &name);
        },
        Err(e) => panic!("{} exists but cannot be read ({})", path, e),
    }
    let mut article_ascii = String::new();
    for c in article.chars() {
        if c.is_ascii() {
            article_ascii.push(c);
        } else {
            let encoded = format!("%{:2X}", c as u32);
            article_ascii.push_str(encoded.as_str());
        }
    }
    let url = format!("https://{}.m.wikipedia.org/w/index.php?title={}&oldid={}",
        language, article_ascii, revision
    );
    println!("Downloading {} and saving to {}", &url, path);
    let response = minreq::get(&url).send().unwrap_or_else(|e| {
        panic!("Cannot get {}: {}", url, e);
    });
    if response.status_code != 200 {
        panic!("Bad URL {}: {} {}", url, response.status_code, response.reason_phrase);
    }
    let content = String::from_utf8(response.into_bytes()).unwrap_or_else(|_| {
        panic!("Response from {} is not UTF-8", url);
    });
    if let Err(e) = fs::create_dir_all(cache_path) {
        eprintln!("Warning: failed to create directory {}: {}", cache_path, e);
    } else if let Err(e) = fs::write(&path, &content) {
        eprintln!("Warning: failed to save {}: {}", path, e);
    }
    sleep(Duration::from_secs(1));
    content
}
const ARTICLES: &[(&str, &str, &str, usize)] = &[
    ("en", "United_Kingdom", "United_Kingdom", 855522252),// 99,7% ASCII
    ("es", "España", "Spain", 109861222),// 1,75% 2-byte characters
    ("ru", "Россия", "Russia", 94607243),// 36% 2-byte characters
    ("zh", "中國", "China", 50868604),// 30% 3-byte characters
];
lazy_static!{
    static ref STRINGS: HashMap<&'static str, String> = {
        let mut content = HashMap::new();
        for &(language, article, english, revision) in ARTICLES {
            content.insert(language, load_wikipedia(language, article, english, revision));
        }
        // make one string with only ASCII
        let only_ascii = content.values()
            .map(|v| (v, v.bytes().filter(|b| b.is_ascii() ).count()) )
            .max_by_key(|&(_,len)| len )
            .map(|(v,_)| v.bytes().filter(|b| b.is_ascii() ).map(|b| b as char ).collect() )
            .unwrap();
        content.insert("ascii", only_ascii);
        content
    };
    static ref EQUAL_CHARS: HashMap<&'static str, &'static str> = {
        let (least, chars) = STRINGS.iter()
            .map(|(l,s)| (l, s.chars().count()) )
            .min_by_key(|&(_,chars)| chars )
            .unwrap();
        println!("chars: {} (limited by {})", chars, least);
        STRINGS.iter().map(|(&language, string)| {
            let cut = string.char_indices()
                .nth(chars)
                .map_or(string.len(), |(i,_)| i );
            let string = &string[..cut];
            assert_eq!(string.chars().count(), chars);
            (language, string)
        }).collect()
    };
    static ref EQUAL_BYTES: HashMap<&'static str, String> = {
        let (least, bytes) = STRINGS.iter()
            .map(|(l,s)| (l, s.len()) )
            .min_by_key(|&(_,bytes)| bytes )
            .unwrap();
        println!("bytes: {} (limited by {})", bytes, least);
        STRINGS.iter().map(|(&language, string)| {
            let mut remaining = bytes;
            // take just so many characters that their length is exactly $bytes
            // slicing won't if !string.is_char_boundary(bytes),
            let string = string.chars().filter(|c| {
                match remaining.checked_sub(c.len_utf8()) {
                    Some(after) => {remaining = after; true},
                    None => false
                }
            }).collect::<String>();
            assert_eq!(string.len(), bytes);
            (language, string)
        }).collect()
    };
    static ref EQUAL_UNITS: HashMap<&'static str, String> = {
        let (least, units) = STRINGS.iter()
            .map(|(l,s)| (l, s.chars().map(|c| c.len_utf16() ).sum::<usize>()) )
            .min_by_key(|&(_,units)| units )
            .unwrap();
        println!("units: {} (limited by {})", units, least);
        STRINGS.iter().map(|(&language, string)| {
            let mut remaining = units;
            let string = string.chars().filter(|c| {
                match remaining.checked_sub(c.len_utf16()) {
                    Some(after) => {remaining = after; true},
                    None => false
                }
            }).collect::<String>();
            assert_eq!(string.chars().map(|c| c.len_utf16() ).sum::<usize>(), units);
            (language, string)
        }).collect()
    };
}



  ///////////////////////////
 // benchmarks begin here //
///////////////////////////

fn utf8char_len(language: &str,  b: &mut Bencher) {
    let string = &EQUAL_BYTES[language];
    let chars: Vec<Utf8Char> = string.chars().map(|c| c.to_utf8() ).collect();
    let bytes = string.len();
    b.iter(|| {
        let sum: usize = black_box(&chars).iter().map(|u8c| u8c.len() ).sum();
        assert_eq!(sum, bytes);
    });
}
#[bench] fn utf8char_len_ascii(b: &mut Bencher) {utf8char_len("ascii", b)}
#[bench] fn utf8char_len_en(b: &mut Bencher) {utf8char_len("en", b)}
#[bench] fn utf8char_len_es(b: &mut Bencher) {utf8char_len("es", b)}
#[bench] fn utf8char_len_ru(b: &mut Bencher) {utf8char_len("ru", b)}
#[bench] fn utf8char_len_zh(b: &mut Bencher) {utf8char_len("zh", b)}

fn utf8_extra_bytes_unchecked(language: &str,  b: &mut Bencher) {
    let string = &EQUAL_CHARS[language];
    let chars = string.chars().count();
    let string = string.as_bytes();
    b.iter(|| {
        let mut i = 0;
        let mut loops = 0;
        while i < string.len() {
            i += string[i].extra_utf8_bytes_unchecked();
            i += 1;
            loops += 1;
        }
        assert_eq!(loops, chars);
    });
}
#[bench] fn utf8_extra_bytes_unchecked_ascii(b: &mut Bencher) {utf8_extra_bytes_unchecked("ascii", b)}
#[bench] fn utf8_extra_bytes_unchecked_en(b: &mut Bencher) {utf8_extra_bytes_unchecked("en", b)}
#[bench] fn utf8_extra_bytes_unchecked_es(b: &mut Bencher) {utf8_extra_bytes_unchecked("es", b)}
#[bench] fn utf8_extra_bytes_unchecked_ru(b: &mut Bencher) {utf8_extra_bytes_unchecked("ru", b)}
#[bench] fn utf8_extra_bytes_unchecked_zh(b: &mut Bencher) {utf8_extra_bytes_unchecked("zh", b)}

fn utf8_extra_bytes(language: &str,  b: &mut Bencher) {
    let string = &EQUAL_CHARS[language];
    let chars = string.chars().count();
    let string = string.as_bytes();
    b.iter(|| {
        let mut i = 0;
        let mut loops = 0;
        let mut errors = 0;
        while i < string.len() {
            match string[i].extra_utf8_bytes() {
                Ok(n) => i += n,
                Err(_) => errors += 1,
            }
            i += 1;
            loops += 1;
        }
        assert_eq!(loops, chars);
        assert_eq!(errors, 0);
    });
}
#[bench] fn utf8_extra_bytes_ascii(b: &mut Bencher) {utf8_extra_bytes("ascii", b)}
#[bench] fn utf8_extra_bytes_en(b: &mut Bencher) {utf8_extra_bytes("en", b)}
#[bench] fn utf8_extra_bytes_es(b: &mut Bencher) {utf8_extra_bytes("es", b)}
#[bench] fn utf8_extra_bytes_ru(b: &mut Bencher) {utf8_extra_bytes("ru", b)}
#[bench] fn utf8_extra_bytes_zh(b: &mut Bencher) {utf8_extra_bytes("zh", b)}


fn utf16char_len(language: &str,  b: &mut Bencher) {
    let string = &EQUAL_UNITS[language];
    let chars: Vec<Utf16Char> = string.chars().map(|c| c.to_utf16() ).collect();
    let units = string.chars().map(|c| c.len_utf16() ).sum::<usize>();
    b.iter(|| {
        let sum: usize = black_box(&chars).iter().map(|u8c| u8c.len() ).sum();
        assert_eq!(sum, units);
    });
}
#[bench] fn utf16char_len_ascii(b: &mut Bencher) {utf16char_len("ascii", b)}
#[bench] fn utf16char_len_en(b: &mut Bencher) {utf16char_len("en", b)}
#[bench] fn utf16char_len_es(b: &mut Bencher) {utf16char_len("en", b)}
#[bench] fn utf16char_len_ru(b: &mut Bencher) {utf16char_len("ru", b)}
#[bench] fn utf16char_len_zh(b: &mut Bencher) {utf16char_len("zh", b)}

fn utf16_is_leading_surrogate(language: &str,  b: &mut Bencher) {
    let string = &EQUAL_UNITS[language];
    let chars = string.chars().count();
    let string: Vec<u16> = string.chars().map(|c| c.to_utf16() ).collect();
    b.iter(|| {
        let mut i = 0;
        let mut loops = 0;
        while i < string.len() {
            i += if string[i].is_utf16_leading_surrogate() {2} else {1};
            loops += 1;
        }
        assert_eq!(loops, chars);
    });
}
#[bench] fn utf16_is_leading_surrogate_ascii(b: &mut Bencher) {utf16_is_leading_surrogate("ascii", b)}
#[bench] fn utf16_is_leading_surrogate_en(b: &mut Bencher) {utf16_is_leading_surrogate("en", b)}
#[bench] fn utf16_is_leading_surrogate_es(b: &mut Bencher) {utf16_is_leading_surrogate("es", b)}
#[bench] fn utf16_is_leading_surrogate_ru(b: &mut Bencher) {utf16_is_leading_surrogate("ru", b)}
#[bench] fn utf16_is_leading_surrogate_zh(b: &mut Bencher) {utf16_is_leading_surrogate("zh", b)}

fn utf16_needs_extra_unit(language: &str,  b: &mut Bencher) {
    let string = &EQUAL_UNITS[language];
    let chars = string.chars().count();
    let string: Vec<u16> = string.chars().map(|c| c.to_utf16() ).collect();
    b.iter(|| {
        let mut i = 0;
        let mut loops = 0;
        let mut errors = 0;
        while i < string.len() {
            i += match string[i].utf16_needs_extra_unit() {
                Ok(true) => 2,
                Ok(false) => 1,
                Err(_) => {errors+=1; 1}
            };
            loops += 1;
        }
        assert_eq!(loops, chars);
        assert_eq!(errors, 0);
    });
}
#[bench] fn utf16_needs_extra_unit_ascii(b: &mut Bencher) {utf16_needs_extra_unit("ascii", b)}
#[bench] fn utf16_needs_extra_unit_en(b: &mut Bencher) {utf16_needs_extra_unit("en", b)}
#[bench] fn utf16_needs_extra_unit_es(b: &mut Bencher) {utf16_needs_extra_unit("es", b)}
#[bench] fn utf16_needs_extra_unit_ru(b: &mut Bencher) {utf16_needs_extra_unit("ru", b)}
#[bench] fn utf16_needs_extra_unit_zh(b: &mut Bencher) {utf16_needs_extra_unit("zh", b)}
