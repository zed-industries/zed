// font-kit/examples/match-font.rs
//
// Copyright © 2020 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Looks up fonts by name.

extern crate font_kit;

use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\tmatch-font \"Times New Roman, Arial, serif\"");
        std::process::exit(1);
    }

    let mut families = Vec::new();
    for family in args[1].split(',') {
        let family = family.replace('\'', "");
        let family = family.trim();
        families.push(match family {
            "serif" => FamilyName::Serif,
            "sans-serif" => FamilyName::SansSerif,
            "monospace" => FamilyName::Monospace,
            "cursive" => FamilyName::Cursive,
            "fantasy" => FamilyName::Fantasy,
            _ => FamilyName::Title(family.to_string()),
        });
    }

    let properties = Properties::default();
    let handle = SystemSource::new().select_best_match(&families, &properties)?;

    if let Handle::Path {
        ref path,
        font_index,
    } = handle
    {
        println!("Path: {}", path.display());
        println!("Index: {}", font_index);
    }

    let font = handle.load()?;

    println!("Family name: {}", font.family_name());
    println!(
        "PostScript name: {}",
        font.postscript_name().unwrap_or("?".to_string())
    );
    println!("Style: {:?}", font.properties().style);
    println!("Weight: {:?}", font.properties().weight);
    println!("Stretch: {:?}", font.properties().stretch);

    Ok(())
}
