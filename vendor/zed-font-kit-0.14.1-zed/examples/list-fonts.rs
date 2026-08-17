// font-kit/examples/list-fonts.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Lists all fonts on the system.

extern crate font_kit;
extern crate pbr;
extern crate prettytable;

use font_kit::source::SystemSource;
use pbr::ProgressBar;
use prettytable::{Attr, Cell, Row, Table};

fn main() {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(Row::new(vec![
        Cell::new("PostScript Name").with_style(Attr::Bold),
        Cell::new("Name").with_style(Attr::Bold),
        Cell::new("Family").with_style(Attr::Bold),
        Cell::new("Style").with_style(Attr::Bold),
        Cell::new("Weight").with_style(Attr::Bold),
        Cell::new("Stretch").with_style(Attr::Bold),
    ]));

    let source = SystemSource::new();
    let fonts = source.all_fonts().unwrap();
    let mut progress_bar = ProgressBar::new(fonts.len() as u64);
    progress_bar.message("Loading fonts… ");

    for font in fonts {
        if let Ok(font) = font.load() {
            let properties = font.properties();
            table.add_row(Row::new(vec![
                Cell::new(&font.postscript_name().unwrap_or_else(|| "".to_owned())),
                Cell::new(&font.full_name()),
                Cell::new(&font.family_name()),
                Cell::new(&properties.style.to_string()),
                Cell::new(&properties.weight.0.to_string()),
                Cell::new(&properties.stretch.0.to_string()),
            ]));
        }

        progress_bar.inc();
    }

    progress_bar.finish_print("");
    table.printstd();
}
