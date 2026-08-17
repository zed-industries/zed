// font-kit/examples/fallback.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate clap;
extern crate font_kit;

use clap::{Arg, ArgMatches, Command};

use font_kit::loader::Loader;
use font_kit::source::SystemSource;

#[cfg(any(target_family = "windows", target_os = "macos"))]
static SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME: &'static str = "ArialMT";
#[cfg(not(any(target_family = "windows", target_os = "macos")))]
static SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME: &str = "DejaVuSans";

fn get_args() -> ArgMatches {
    let postscript_name_arg = Arg::new("POSTSCRIPT-NAME")
        .help("PostScript name of the font")
        .default_value(SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME)
        .index(1);
    let text_arg = Arg::new("TEXT")
        .help("Text to query")
        .default_value("A")
        .index(2);
    let locale_arg = Arg::new("LOCALE")
        .help("Locale for fallback query")
        .default_value("en-US")
        .index(3);
    Command::new("fallback")
        .version("0.1")
        .arg(postscript_name_arg)
        .arg(text_arg)
        .arg(locale_arg)
        .get_matches()
}

fn main() {
    let matches = get_args();
    let postscript_name = matches
        .get_one::<String>("POSTSCRIPT-NAME")
        .map(|s| s.as_str())
        .unwrap();
    let text = matches
        .get_one::<String>("TEXT")
        .map(|s| s.as_str())
        .unwrap();
    let locale = matches
        .get_one::<String>("LOCALE")
        .map(|s| s.as_str())
        .unwrap();
    let font = SystemSource::new()
        .select_by_postscript_name(postscript_name)
        .expect("Font not found")
        .load()
        .unwrap();
    println!("{}: text: {:?}", postscript_name, text);
    let fallback_result = font.get_fallbacks(text, locale);
    println!(
        "fallback valid substring length: {}",
        fallback_result.valid_len
    );
    for font in &fallback_result.fonts {
        println!("font: {}", font.font.full_name());
    }
}
