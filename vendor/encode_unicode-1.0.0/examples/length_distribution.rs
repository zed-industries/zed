/* Copyright 2018 Torbjørn Birch Moltu
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */

//! Counts the number of codepoints of each UTF-8 length in files

use std::env::args_os;
use std::fs::File;
use std::io::{self, Read, stdin};
use std::borrow::Cow;
extern crate encode_unicode;
use encode_unicode::U8UtfExt;

#[derive(Default)]
struct Distribution {
    bytes: usize,
    utf8: [usize; 4],
}

fn read(file: &mut dyn Read) -> (Distribution, Option<io::Error>) {
    let mut r = Distribution::default();
    let mut buf = [0u8; 4096];
    loop {
        let read = match file.read(&mut buf) {
            Ok(0) => return (r, None),
            Ok(n) => n,
            Err(e) => return (r, Some(e)),
        };
        r.bytes += read;
        for (o, &b) in buf[..read].iter().enumerate() {
            if let Ok(i) = b.extra_utf8_bytes() {
                r.utf8[i] += 1;
                if i == 3 {
                    let min = o.saturating_sub(20);
                    let max = if o+23 <= read {o+23} else {read};
                    println!("{}", String::from_utf8_lossy(&buf[min..max]));
                }
            }
        }
    }
}

fn display(name_pad: usize,  name: Cow<str>,
           r: Distribution,  err: Option<io::Error>) {
    let c = r.utf8;
    let characters = c[0]+c[1]+c[2]+c[3];
    let s = [c[0], c[1]*2, c[2]*3, c[3]*4];
    let p = [
        (s[0]*100) as f32 / r.bytes as f32,
        (s[1]*100) as f32 / r.bytes as f32,
        (s[2]*100) as f32 / r.bytes as f32,
        (s[3]*100) as f32 / r.bytes as f32,
    ];
    println!("{:>6$}: bytes: {:7}, UTF-8 distribution: [{:7}, {:6}, {:6}, {:6}]",
        name, r.bytes, s[0], s[1], s[2], s[3], name_pad
    );
    println!("{5:6$}  chars: {:7}, UTF-8 percentages:  [{:>6.2}%, {:>5.2}%, {:>5.2}%, {:>5.2}%]",
        characters, p[0], p[1], p[2], p[3], "", name_pad
    );
    if let Some(err) = err {
        println!("{1:2$}  {}", err, "", name_pad);
    }
}

fn main() {
    let name_length = args_os().skip(1)
        .map(|path| path.to_string_lossy().chars().count() )
        .max();
    for path in args_os().skip(1) {
        let name = path.to_string_lossy();
        let (r,err) = match File::open(&path) {
            Ok(mut file) => read(&mut file),
            Err(err) => {
                eprintln!("{}:\t{}", name, err);
                continue;
            }
        };
        display(name_length.unwrap(), name, r, err);
    }
    if name_length.is_none() {
        let stdin = stdin();
        let (r,err) = read(&mut stdin.lock());
        display(0, Cow::Borrowed("stdin"), r, err);
    }
}
