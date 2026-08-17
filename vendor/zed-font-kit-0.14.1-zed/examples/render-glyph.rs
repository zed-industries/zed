// font-kit/examples/render-glyph.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate clap;
extern crate colored;
extern crate font_kit;
extern crate pathfinder_geometry;

use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};
use colored::Colorize;
use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::hinting::HintingOptions;
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use std::fmt::Write;

#[cfg(any(target_family = "windows", target_os = "macos"))]
static SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME: &'static str = "ArialMT";
#[cfg(not(any(target_family = "windows", target_os = "macos")))]
static SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME: &str = "DejaVuSans";

fn get_args() -> ArgMatches {
    let postscript_name_arg = Arg::new("POSTSCRIPT-NAME")
        .help("PostScript name of the font")
        .default_value(SANS_SERIF_FONT_REGULAR_POSTSCRIPT_NAME)
        .index(1);
    let glyph_arg = Arg::new("GLYPH")
        .help("Character to render")
        .default_value("A")
        .index(2);
    let size_arg = Arg::new("SIZE")
        .help("Font size in blocks")
        .default_value("32")
        .index(3);
    let grayscale_arg = Arg::new("grayscale")
        .long("grayscale")
        .help("Use grayscale antialiasing (default)");
    let bilevel_arg = Arg::new("bilevel")
        .help("Use bilevel (black & white) rasterization")
        .short('b')
        .long("bilevel")
        .action(ArgAction::SetTrue);
    let subpixel_arg = Arg::new("subpixel")
        .help("Use subpixel (LCD) rasterization")
        .short('s')
        .long("subpixel")
        .action(ArgAction::SetTrue);
    let hinting_value_parser =
        clap::builder::PossibleValuesParser::new(["none", "vertical", "full"]);
    let hinting_arg = Arg::new("hinting")
        .help("Select hinting type")
        .short('H')
        .long("hinting")
        .value_parser(hinting_value_parser)
        .value_names(&["TYPE"]);
    let transform_arg = Arg::new("transform")
        .help("Transform to apply to glyph when rendering")
        .long("transform")
        .num_args(4);
    let rasterization_mode_group =
        ArgGroup::new("rasterization-mode").args(&["grayscale", "bilevel", "subpixel"]);
    Command::new("render-glyph")
        .version("0.1")
        .author("The Pathfinder Project Developers")
        .about("Simple example tool to render glyphs with `font-kit`")
        .arg(postscript_name_arg)
        .arg(glyph_arg)
        .arg(size_arg)
        .arg(grayscale_arg)
        .arg(bilevel_arg)
        .arg(subpixel_arg)
        .group(rasterization_mode_group)
        .arg(hinting_arg)
        .arg(transform_arg)
        .get_matches()
}

fn main() {
    let matches = get_args();

    let postscript_name = matches
        .get_one::<String>("POSTSCRIPT-NAME")
        .map(|s| s.as_str())
        .unwrap();
    let character = matches
        .get_one::<String>("GLYPH")
        .map(|s| s.as_str())
        .unwrap()
        .chars()
        .next()
        .unwrap();
    let size: f32 = matches
        .get_one::<String>("SIZE")
        .map(|s| s.as_str())
        .unwrap()
        .parse()
        .unwrap();

    let (canvas_format, rasterization_options) = if matches.get_flag("bilevel") {
        (Format::A8, RasterizationOptions::Bilevel)
    } else if matches.get_flag("subpixel") {
        (Format::Rgb24, RasterizationOptions::SubpixelAa)
    } else {
        (Format::A8, RasterizationOptions::GrayscaleAa)
    };

    let mut transform = Transform2F::default();
    if let Some(values) = matches.get_many::<String>("transform") {
        if let [Ok(a), Ok(b), Ok(c), Ok(d)] = values.map(|x| x.parse()).collect::<Vec<_>>()[..] {
            transform = Transform2F::row_major(a, b, c, d, 0.0, 0.0)
        }
    }

    let hinting_options = match matches.get_one::<String>("hinting").map(|s| s.as_str()) {
        Some("vertical") => HintingOptions::Vertical(size),
        Some("full") => HintingOptions::Full(size),
        _ => HintingOptions::None,
    };

    let font = SystemSource::new()
        .select_by_postscript_name(postscript_name)
        .unwrap()
        .load()
        .unwrap();
    let glyph_id = font.glyph_for_char(character).unwrap();

    let raster_rect = font
        .raster_bounds(
            glyph_id,
            size,
            transform,
            hinting_options,
            rasterization_options,
        )
        .unwrap();

    let mut canvas = Canvas::new(raster_rect.size(), canvas_format);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        size,
        Transform2F::from_translation(-raster_rect.origin().to_f32()) * transform,
        hinting_options,
        rasterization_options,
    )
    .unwrap();

    println!("glyph {}:", glyph_id);
    for y in 0..raster_rect.height() {
        let mut line = String::new();
        let (row_start, row_end) = (y as usize * canvas.stride, (y + 1) as usize * canvas.stride);
        let row = &canvas.pixels[row_start..row_end];
        for x in 0..raster_rect.width() {
            match canvas.format {
                Format::Rgba32 => unimplemented!(),
                Format::Rgb24 => {
                    write!(
                        &mut line,
                        "{}{}{}",
                        shade(row[x as usize * 3]).to_string().red(),
                        shade(row[x as usize * 3 + 1]).to_string().green(),
                        shade(row[x as usize * 3 + 2]).to_string().blue()
                    )
                    .unwrap();
                }
                Format::A8 => {
                    let shade = shade(row[x as usize]);
                    line.push(shade);
                    line.push(shade);
                }
            }
        }
        println!("{}", line);
    }
}

fn shade(value: u8) -> char {
    match value {
        0 => ' ',
        1..=84 => '░',
        85..=169 => '▒',
        170..=254 => '▓',
        _ => '█',
    }
}
