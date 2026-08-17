// font-kit/tests/tests.rs
//
// Copyright © 2019 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// General tests.

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::file_type::FileType;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::outline::{Contour, Outline, OutlineBuilder, PointFlags};
use font_kit::properties::{Properties, Stretch, Weight};
use pathfinder_geometry::rect::{RectF, RectI};
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

#[cfg(feature = "source")]
use font_kit::source::SystemSource;

static TEST_FONT_FILE_PATH: &str = "resources/tests/eb-garamond/EBGaramond12-Regular.otf";
static TEST_FONT_POSTSCRIPT_NAME: &str = "EBGaramond12-Regular";
static TEST_FONT_COLLECTION_FILE_PATH: &str = "resources/tests/eb-garamond/EBGaramond12.otc";
static TEST_FONT_COLLECTION_POSTSCRIPT_NAME: [&str; 2] =
    ["EBGaramond12-Regular", "EBGaramond12-Italic"];

static FILE_PATH_EB_GARAMOND_TTF: &str = "resources/tests/eb-garamond/EBGaramond12-Regular.ttf";
static FILE_PATH_INCONSOLATA_TTF: &str = "resources/tests/inconsolata/Inconsolata-Regular.ttf";

#[cfg(not(target_os = "linux"))]
static KNOWN_SYSTEM_FONT_NAME: &'static str = "Arial";
#[cfg(target_os = "linux")]
static KNOWN_SYSTEM_FONT_NAME: &str = "DejaVu Sans";

static SFNT_VERSIONS: [[u8; 4]; 4] = [
    [0x00, 0x01, 0x00, 0x00],
    [b'O', b'T', b'T', b'O'],
    [b't', b'r', b'u', b'e'],
    [b't', b'y', b'p', b'1'],
];

const OPENTYPE_TABLE_TAG_HEAD: u32 = 0x68656164;

#[cfg(feature = "source")]
#[test]
pub fn get_font_full_name() {
    let font = SystemSource::new()
        .select_best_match(
            &[FamilyName::Title(KNOWN_SYSTEM_FONT_NAME.to_string())],
            &Properties::new(),
        )
        .unwrap()
        .load()
        .unwrap();
    assert_eq!(font.full_name(), KNOWN_SYSTEM_FONT_NAME);
}

#[cfg(feature = "source")]
#[test]
pub fn get_font_full_name_from_lowercase_family_name() {
    let font = SystemSource::new()
        .select_best_match(
            &[FamilyName::Title(
                KNOWN_SYSTEM_FONT_NAME.to_ascii_lowercase(),
            )],
            &Properties::new(),
        )
        .unwrap()
        .load()
        .unwrap();
    assert_eq!(font.full_name(), KNOWN_SYSTEM_FONT_NAME);
}

#[test]
pub fn load_font_from_file() {
    let mut file = File::open(TEST_FONT_FILE_PATH).unwrap();
    let font = Font::from_file(&mut file, 0).unwrap();
    assert_eq!(font.postscript_name().unwrap(), TEST_FONT_POSTSCRIPT_NAME);
}

#[test]
pub fn load_font_from_memory() {
    let mut file = File::open(TEST_FONT_FILE_PATH).unwrap();
    let mut font_data = vec![];
    file.read_to_end(&mut font_data).unwrap();
    let font = Font::from_bytes(Arc::new(font_data), 0).unwrap();
    assert_eq!(font.postscript_name().unwrap(), TEST_FONT_POSTSCRIPT_NAME);
}

#[test]
pub fn analyze_file() {
    let mut file = File::open(TEST_FONT_FILE_PATH).unwrap();
    assert_eq!(Font::analyze_file(&mut file).unwrap(), FileType::Single);
}

#[test]
pub fn analyze_bytes() {
    let mut file = File::open(TEST_FONT_FILE_PATH).unwrap();
    let mut font_data = vec![];
    file.read_to_end(&mut font_data).unwrap();
    assert_eq!(
        Font::analyze_bytes(Arc::new(font_data)).unwrap(),
        FileType::Single
    );
}

#[cfg(feature = "source")]
#[test]
pub fn get_glyph_for_char() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('a').expect("No glyph for char!");
    assert_eq!(glyph, 68);
}

#[cfg(all(
    feature = "source",
    any(target_family = "windows", target_os = "macos")
))]
#[test]
pub fn get_glyph_outline() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('i').expect("No glyph for char!");
    let mut outline_builder = OutlineBuilder::new();
    font.outline(glyph, HintingOptions::None, &mut outline_builder)
        .unwrap();

    let outline = outline_builder.into_outline();
    assert_eq!(
        outline,
        Outline {
            contours: vec![
                Contour {
                    positions: vec![
                        Vector2F::new(136.0, 1259.0),
                        Vector2F::new(136.0, 1466.0),
                        Vector2F::new(316.0, 1466.0),
                        Vector2F::new(316.0, 1259.0),
                    ],
                    flags: vec![PointFlags::empty(); 4],
                },
                Contour {
                    positions: vec![
                        Vector2F::new(136.0, 0.0),
                        Vector2F::new(136.0, 1062.0),
                        Vector2F::new(316.0, 1062.0),
                        Vector2F::new(316.0, 0.0),
                    ],
                    flags: vec![PointFlags::empty(); 4],
                },
            ],
        }
    );
}

#[cfg(all(
    feature = "source",
    not(any(target_family = "windows", target_os = "macos", target_os = "ios"))
))]
#[test]
pub fn get_glyph_outline() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('i').expect("No glyph for char!");
    let mut outline_builder = OutlineBuilder::new();
    font.outline(glyph, HintingOptions::None, &mut outline_builder)
        .unwrap();

    let outline = outline_builder.into_outline();
    assert_eq!(
        outline,
        Outline {
            contours: vec![
                Contour {
                    positions: vec![
                        Vector2F::new(193.0, 1120.0),
                        Vector2F::new(377.0, 1120.0),
                        Vector2F::new(377.0, 0.0),
                        Vector2F::new(193.0, 0.0),
                    ],
                    flags: vec![PointFlags::empty(); 4],
                },
                Contour {
                    positions: vec![
                        Vector2F::new(193.0, 1556.0),
                        Vector2F::new(377.0, 1556.0),
                        Vector2F::new(377.0, 1323.0),
                        Vector2F::new(193.0, 1323.0),
                    ],
                    flags: vec![PointFlags::empty(); 4],
                },
            ],
        }
    );
}

// Right now, only FreeType can do hinting.
#[cfg(all(
    not(any(target_os = "macos", target_os = "ios", target_family = "windows")),
    feature = "loader-freetype-default",
    feature = "source"
))]
#[test]
pub fn get_vertically_hinted_glyph_outline() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('i').expect("No glyph for char!");
    let mut outline_builder = OutlineBuilder::new();
    font.outline(glyph, HintingOptions::Vertical(16.0), &mut outline_builder)
        .unwrap();

    let outline = outline_builder.into_outline();
    assert_eq!(
        outline,
        Outline {
            contours: vec![
                Contour {
                    positions: vec![
                        Vector2F::new(136.0, 1316.0),
                        Vector2F::new(136.0, 1536.0),
                        Vector2F::new(316.0, 1536.0),
                        Vector2F::new(316.0, 1316.0),
                    ],
                    flags: vec![PointFlags::empty(); 4],
                },
                Contour {
                    positions: vec![
                        Vector2F::new(136.0, 0.0),
                        Vector2F::new(136.0, 1152.0),
                        Vector2F::new(316.0, 1152.0),
                        Vector2F::new(316.0, 0.0),
                    ],
                    flags: vec![PointFlags::empty(); 4],
                },
            ],
        }
    );
}

#[cfg(all(
    feature = "source",
    not(feature = "loader-freetype-default"),
    not(any(target_os = "macos", target_os = "ios", target_family = "windows"))
))]
#[test]
pub fn get_vertically_hinted_glyph_outline() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('i').expect("No glyph for char!");
    let mut outline_builder = OutlineBuilder::new();
    font.outline(glyph, HintingOptions::Vertical(16.0), &mut outline_builder)
        .unwrap();

    let outline = outline_builder.into_outline();
    assert_eq!(
        outline,
        Outline {
            contours: vec![
                Contour {
                    positions: vec![
                        Vector2F::new(256.0, 1152.0),
                        Vector2F::new(384.0, 1152.0),
                        Vector2F::new(384.0, 0.0),
                        Vector2F::new(256.0, 0.0),
                    ],
                    flags: vec![PointFlags::empty(); 4],
                },
                Contour {
                    positions: vec![
                        Vector2F::new(256.0, 1536.0),
                        Vector2F::new(384.0, 1536.0),
                        Vector2F::new(384.0, 1280.0),
                        Vector2F::new(256.0, 1280.0),
                    ],
                    flags: vec![PointFlags::empty(); 4],
                },
            ],
        }
    );
}

// Right now, only FreeType can do hinting.
#[cfg(all(
    not(any(target_os = "macos", target_os = "ios", target_family = "windows")),
    feature = "loader-freetype-default",
    feature = "source"
))]
#[test]
pub fn get_fully_hinted_glyph_outline() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('i').expect("No glyph for char!");
    let mut outline_builder = OutlineBuilder::new();
    font.outline(glyph, HintingOptions::Full(10.0), &mut outline_builder)
        .unwrap();

    let outline = outline_builder.into_outline();
    assert_eq!(
        outline,
        Outline {
            contours: vec![
                Contour {
                    positions: vec![
                        Vector2F::new(137.6, 1228.8),
                        Vector2F::new(137.6, 1433.6),
                        Vector2F::new(316.80002, 1433.6),
                        Vector2F::new(316.80002, 1228.8),
                    ],
                    flags: vec![PointFlags::empty(); 4],
                },
                Contour {
                    positions: vec![
                        Vector2F::new(137.6, 0.0),
                        Vector2F::new(137.6, 1024.0),
                        Vector2F::new(316.80002, 1024.0),
                        Vector2F::new(316.80002, 0.0),
                    ],
                    flags: vec![PointFlags::empty(); 4],
                },
            ],
        }
    );
}

#[cfg(all(
    feature = "source",
    not(feature = "loader-freetype-default"),
    not(any(target_os = "macos", target_os = "ios", target_family = "windows"))
))]
#[test]
pub fn get_fully_hinted_glyph_outline() {
    let mut file = File::open(FILE_PATH_INCONSOLATA_TTF).unwrap();
    let font = Font::from_file(&mut file, 0).unwrap();
    let glyph = font.glyph_for_char('i').expect("No glyph for char!");

    let mut outline_builder = OutlineBuilder::new();
    font.outline(glyph, HintingOptions::Full(10.0), &mut outline_builder)
        .unwrap();

    let outline = outline_builder.into_outline();
    assert_eq!(
        outline,
        Outline {
            contours: vec![
                Contour {
                    positions: vec![
                        Vector2F::new(100.0, 100.0),
                        Vector2F::new(200.0, 100.0),
                        Vector2F::new(200.0, 400.0),
                        Vector2F::new(100.0, 400.0),
                        Vector2F::new(100.0, 500.0),
                        Vector2F::new(300.0, 500.0),
                        Vector2F::new(300.0, 100.0),
                        Vector2F::new(400.0, 100.0),
                        Vector2F::new(400.0, 0.0),
                        Vector2F::new(100.0, 0.0),
                    ],
                    flags: vec![PointFlags::empty(); 10],
                },
                Contour {
                    positions: vec![
                        Vector2F::new(200.0, 600.0),
                        Vector2F::new(200.0, 600.0),
                        Vector2F::new(200.0, 600.0),
                        Vector2F::new(200.0, 600.0),
                        Vector2F::new(200.0, 600.0),
                        Vector2F::new(200.0, 700.0),
                        Vector2F::new(200.0, 700.0),
                        Vector2F::new(200.0, 700.0),
                        Vector2F::new(200.0, 700.0),
                        Vector2F::new(300.0, 700.0),
                        Vector2F::new(300.0, 700.0),
                        Vector2F::new(300.0, 700.0),
                        Vector2F::new(300.0, 600.0),
                        Vector2F::new(300.0, 600.0),
                        Vector2F::new(300.0, 600.0),
                        Vector2F::new(300.0, 600.0),
                        Vector2F::new(200.0, 600.0),
                    ],
                    flags: vec![
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                    ],
                },
            ],
        }
    );
}

#[test]
pub fn get_empty_glyph_outline() {
    let mut file = File::open(TEST_FONT_FILE_PATH).unwrap();
    let font = Font::from_file(&mut file, 0).unwrap();
    let glyph = font.glyph_for_char(' ').expect("No glyph for char!");
    let mut outline_builder = OutlineBuilder::new();
    font.outline(glyph, HintingOptions::None, &mut outline_builder)
        .unwrap();

    let outline = outline_builder.into_outline();
    assert_eq!(outline, Outline::new());
}

// https://github.com/servo/font-kit/issues/141
#[test]
pub fn get_glyph_raster_bounds() {
    let mut file = File::open(FILE_PATH_INCONSOLATA_TTF).unwrap();
    let font = Font::from_file(&mut file, 0).unwrap();
    let glyph = font.glyph_for_char('J').expect("No glyph for char!");
    let transform = Transform2F::default();
    let size = 32.0;
    let hinting_options = HintingOptions::None;
    let rasterization_options = RasterizationOptions::GrayscaleAa;
    #[cfg(not(target_family = "windows"))]
    let expected_rect = RectI::new(Vector2I::new(1, -20), Vector2I::new(14, 21));
    #[cfg(target_family = "windows")]
    let expected_rect = RectI::new(Vector2I::new(1, -20), Vector2I::new(14, 20));
    assert_eq!(
        font.raster_bounds(
            glyph,
            size,
            transform,
            hinting_options,
            rasterization_options
        ),
        Ok(expected_rect)
    );
}

#[cfg(all(
    feature = "source",
    any(target_family = "windows", target_os = "macos", target_os = "ios")
))]
#[test]
pub fn get_glyph_typographic_bounds() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('a').expect("No glyph for char!");
    assert_eq!(
        font.typographic_bounds(glyph),
        Ok(RectF::new(
            Vector2F::new(74.0, -24.0),
            Vector2F::new(978.0, 1110.0)
        ))
    );
}

#[cfg(all(
    feature = "source",
    not(any(target_family = "windows", target_os = "macos", target_os = "ios"))
))]
#[test]
pub fn get_glyph_typographic_bounds() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('a').expect("No glyph for char!");
    assert_eq!(
        font.typographic_bounds(glyph),
        Ok(RectF::new(
            Vector2F::new(123.0, -29.0),
            Vector2F::new(946.0, 1176.0)
        ))
    );
}

#[cfg(all(feature = "source", target_family = "windows"))]
#[test]
pub fn get_glyph_advance_and_origin() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('a').expect("No glyph for char!");
    assert_eq!(font.advance(glyph), Ok(Vector2F::new(1139.0, 0.0)));
    assert_eq!(font.origin(glyph), Ok(Vector2F::new(74.0, 1898.0)));
}

#[cfg(all(feature = "source", target_os = "macos"))]
#[test]
pub fn get_glyph_advance_and_origin() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('a').expect("No glyph for char!");
    assert_eq!(font.advance(glyph), Ok(Vector2F::new(1139.0, 0.0)));
    assert_eq!(font.origin(glyph), Ok(Vector2F::default()));
}

#[cfg(all(
    feature = "source",
    not(any(target_family = "windows", target_os = "macos", target_os = "ios"))
))]
#[test]
pub fn get_glyph_advance_and_origin() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph = font.glyph_for_char('a').expect("No glyph for char!");
    assert_eq!(font.advance(glyph), Ok(Vector2F::new(1255.0, 0.0)));
    assert_eq!(font.origin(glyph), Ok(Vector2F::default()));
}

#[cfg(all(
    feature = "source",
    any(target_family = "windows", target_os = "macos")
))]
#[test]
pub fn get_font_metrics() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let metrics = font.metrics();
    assert_eq!(metrics.units_per_em, 2048);
    assert_eq!(metrics.ascent, 1854.0);
    assert_eq!(metrics.descent, -434.0);
    assert_eq!(metrics.line_gap, 67.0);
    assert_eq!(metrics.underline_position, -217.0);
    assert_eq!(metrics.underline_thickness, 150.0);
    assert_eq!(metrics.cap_height, 1467.0);
    assert_eq!(metrics.x_height, 1062.0);

    // Different versions of the font can have different max heights, so ignore that.
    let bounding_box = metrics.bounding_box;
    assert_eq!(bounding_box.origin(), Vector2F::new(-1361.0, -665.0));
    assert_eq!(bounding_box.width(), 5457.0);
}

#[cfg(all(
    feature = "source",
    not(any(target_family = "windows", target_os = "macos", target_os = "ios"))
))]
#[test]
pub fn get_font_metrics() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let metrics = font.metrics();
    assert_eq!(metrics.units_per_em, 2048);
    assert_eq!(metrics.ascent, 1901.0);
    assert_eq!(metrics.descent, -483.0);
    assert_eq!(metrics.line_gap, 0.0); // FIXME(pcwalton): Huh?!
    assert_eq!(metrics.underline_position, -40.0);
    assert_eq!(metrics.underline_thickness, 90.0);
    assert_eq!(metrics.cap_height, 0.0); // FIXME(pcwalton): Huh?!
    assert_eq!(metrics.x_height, 0.0); // FIXME(pcwalton): Huh?!
    assert_eq!(
        metrics.bounding_box,
        RectF::new(
            Vector2F::new(-2090.0, -948.0),
            Vector2F::new(5763.0, 3472.0)
        )
    );
}

#[cfg(feature = "source")]
#[test]
pub fn get_font_properties() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let properties = font.properties();
    assert_eq!(properties.weight, Weight(400.0));
    assert_eq!(properties.stretch, Stretch(1.0));
}

#[cfg(feature = "source")]
#[test]
pub fn load_font_table() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let head_table = font
        .load_font_table(OPENTYPE_TABLE_TAG_HEAD)
        .expect("Where's the `head` table?");
    assert_eq!(&head_table[12..16], &[0x5f, 0x0f, 0x3c, 0xf5]);
}

#[cfg(feature = "source")]
#[test]
pub fn rasterize_glyph_with_grayscale_aa() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph_id = font.glyph_for_char('L').unwrap();
    let size = 32.0;
    let raster_rect = font
        .raster_bounds(
            glyph_id,
            size,
            Transform2F::default(),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa,
        )
        .unwrap();
    let mut canvas = Canvas::new(raster_rect.size(), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        size,
        Transform2F::from_translation(-raster_rect.origin().to_f32()),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )
    .unwrap();
    check_L_shape(&canvas);
}

#[cfg(feature = "source")]
#[test]
pub fn rasterize_glyph_bilevel() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph_id = font.glyph_for_char('L').unwrap();
    let size = 16.0;
    let raster_rect = font
        .raster_bounds(
            glyph_id,
            size,
            Transform2F::default(),
            HintingOptions::None,
            RasterizationOptions::Bilevel,
        )
        .unwrap();
    let mut canvas = Canvas::new(raster_rect.size(), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        size,
        Transform2F::from_translation(-raster_rect.origin().to_f32()),
        HintingOptions::None,
        RasterizationOptions::Bilevel,
    )
    .unwrap();
    assert!(canvas
        .pixels
        .iter()
        .all(|&value| value == 0 || value == 0xff));
    check_L_shape(&canvas);
}

#[cfg(feature = "source")]
#[test]
pub fn rasterize_glyph_bilevel_offset() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph_id = font.glyph_for_char('L').unwrap();
    let size = 32.0;
    let raster_rect = font
        .raster_bounds(
            glyph_id,
            size,
            Transform2F::from_translation(Vector2F::new(30.0, 100.0)),
            HintingOptions::None,
            RasterizationOptions::Bilevel,
        )
        .unwrap();
    let mut canvas = Canvas::new(raster_rect.size(), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        size,
        Transform2F::from_translation(-raster_rect.origin().to_f32() + Vector2F::new(30.0, 100.0)),
        HintingOptions::None,
        RasterizationOptions::Bilevel,
    )
    .unwrap();

    assert!(canvas
        .pixels
        .iter()
        .all(|&value| value == 0 || value == 0xff));
    check_L_shape(&canvas);
}

#[cfg(all(
    feature = "source",
    any(
        not(any(target_os = "macos", target_os = "ios", target_family = "windows")),
        feature = "loader-freetype-default"
    )
))]
#[test]
pub fn rasterize_glyph_with_full_hinting() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph_id = font.glyph_for_char('L').unwrap();
    let size = 32.0;
    let raster_rect = font
        .raster_bounds(
            glyph_id,
            size,
            Transform2F::default(),
            HintingOptions::None,
            RasterizationOptions::Bilevel,
        )
        .unwrap();
    let origin = -raster_rect.origin().to_f32();
    let mut canvas = Canvas::new(raster_rect.size(), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        size,
        Transform2F::from_translation(origin),
        HintingOptions::Full(size),
        RasterizationOptions::GrayscaleAa,
    )
    .unwrap();
    check_L_shape(&canvas);

    // Make sure the top and bottom (non-blank) rows have some fully black pixels in them.
    let mut top_row = &canvas.pixels[0..canvas.stride];
    if top_row.iter().all(|&value| value == 0) {
        top_row = &canvas.pixels[canvas.stride..(2 * canvas.stride)];
    }

    assert!(top_row.iter().any(|&value| value == 0xff));
    for y in (0..(canvas.size.y() as usize)).rev() {
        let bottom_row = &canvas.pixels[(y * canvas.stride)..((y + 1) * canvas.stride)];
        if bottom_row.iter().all(|&value| value == 0) {
            continue;
        }
        assert!(bottom_row.iter().any(|&value| value == 0xff));
        break;
    }
}

#[cfg(all(feature = "source", target_family = "windows"))]
#[test]
pub fn rasterize_glyph() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph_id = font.glyph_for_char('{').unwrap();
    let size = 32.0;
    let raster_rect = font
        .raster_bounds(
            glyph_id,
            size,
            Transform2F::default(),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa,
        )
        .unwrap();
    let mut canvas = Canvas::new(raster_rect.size(), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph_id,
        size,
        Transform2F::from_translation(-raster_rect.origin().to_f32()),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )
    .unwrap();
    check_curly_shape(&canvas);
}

// Tests that an empty glyph can be successfully rasterized to a 0x0 canvas (issue #7).
#[cfg(feature = "source")]
#[test]
pub fn rasterize_empty_glyph() {
    let mut file = File::open(TEST_FONT_FILE_PATH).unwrap();
    let font = Font::from_file(&mut file, 0).unwrap();
    let glyph = font.glyph_for_char(' ').expect("No glyph for char!");
    let mut canvas = Canvas::new(Vector2I::splat(16), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph,
        16.0,
        Transform2F::default(),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )
    .unwrap();
}

// Tests that an empty glyph can be successfully rasterized to a 0x0 canvas (issue #7).
#[cfg(feature = "source")]
#[test]
pub fn rasterize_empty_glyph_on_empty_canvas() {
    let mut file = File::open(TEST_FONT_FILE_PATH).unwrap();
    let font = Font::from_file(&mut file, 0).unwrap();
    let glyph = font.glyph_for_char(' ').expect("No glyph for char!");
    let size = 32.0;
    let raster_rect = font
        .raster_bounds(
            glyph,
            size,
            Transform2F::default(),
            HintingOptions::None,
            RasterizationOptions::GrayscaleAa,
        )
        .unwrap();
    let mut canvas = Canvas::new(raster_rect.size(), Format::A8);
    font.rasterize_glyph(
        &mut canvas,
        glyph,
        size,
        Transform2F::from_translation(-raster_rect.origin().to_f32()),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )
    .unwrap();
}

#[cfg(feature = "source")]
#[test]
pub fn font_transform() {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let glyph_id = font.glyph_for_char('L').unwrap();
    let size = 16.0;
    let raster_rect = font
        .raster_bounds(
            glyph_id,
            size,
            Transform2F::from_translation(Vector2F::splat(8.0)),
            HintingOptions::None,
            RasterizationOptions::Bilevel,
        )
        .unwrap();
    let raster_rect2 = font
        .raster_bounds(
            glyph_id,
            size,
            Transform2F::row_major(3.0, 0.0, 0.0, 3.0, 8.0, 8.0),
            HintingOptions::None,
            RasterizationOptions::Bilevel,
        )
        .unwrap();
    assert!((raster_rect2.width() - raster_rect.width() * 3).abs() <= 3);
    assert!((raster_rect2.height() - raster_rect.height() * 3).abs() <= 3);
    assert!((raster_rect2.origin_x() - ((raster_rect.origin_x() - 8) * 3 + 8)).abs() <= 3);
    assert!((raster_rect2.origin_y() - ((raster_rect.origin_y() - 8) * 3 + 8)).abs() <= 3);
}

#[test]
fn load_fonts_from_opentype_collection() {
    let mut file = File::open(TEST_FONT_COLLECTION_FILE_PATH).unwrap();
    {
        let font = Font::from_file(&mut file, 0).unwrap();
        assert_eq!(
            font.postscript_name().unwrap(),
            TEST_FONT_COLLECTION_POSTSCRIPT_NAME[0]
        );
    }
    let font = Font::from_file(&mut file, 1).unwrap();
    assert_eq!(
        font.postscript_name().unwrap(),
        TEST_FONT_COLLECTION_POSTSCRIPT_NAME[1]
    );
}

#[test]
fn get_glyph_count() {
    let font = Font::from_path(TEST_FONT_FILE_PATH, 0).unwrap();
    assert_eq!(font.glyph_count(), 3084);
}

// The initial off-curve point used to cause an assertion in the FreeType backend.
#[test]
fn get_glyph_outline_eb_garamond_exclam() {
    let mut file = File::open(FILE_PATH_EB_GARAMOND_TTF).unwrap();
    let font = Font::from_file(&mut file, 0).unwrap();
    let glyph = font.glyph_for_char('!').expect("No glyph for char!");
    let mut outline_builder = OutlineBuilder::new();
    font.outline(glyph, HintingOptions::None, &mut outline_builder)
        .unwrap();

    // The TrueType spec doesn't specify the rounding method for midpoints, as far as I can tell.
    // So we are lenient and accept either values rounded down (what Core Text provides if the
    // first point is off-curve, it seems) or precise floating-point values (what our FreeType
    // loader provides).
    let mut outline = outline_builder.into_outline();
    for contour in &mut outline.contours {
        for position in &mut contour.positions {
            *position = position.floor();
        }
    }

    println!("{:#?}", outline);
    assert_eq!(
        outline,
        Outline {
            contours: vec![
                Contour {
                    positions: vec![
                        Vector2F::new(114.0, 598.0),
                        Vector2F::new(114.0, 619.0),
                        Vector2F::new(127.0, 634.0),
                        Vector2F::new(141.0, 649.0),
                        Vector2F::new(161.0, 649.0),
                        Vector2F::new(181.0, 649.0),
                        Vector2F::new(193.0, 634.0),
                        Vector2F::new(206.0, 619.0),
                        Vector2F::new(206.0, 598.0),
                        Vector2F::new(206.0, 526.0),
                        Vector2F::new(176.0, 244.0),
                        Vector2F::new(172.0, 205.0),
                        Vector2F::new(158.0, 205.0),
                        Vector2F::new(144.0, 205.0),
                        Vector2F::new(140.0, 244.0),
                        Vector2F::new(114.0, 491.0),
                        Vector2F::new(114.0, 598.0),
                    ],
                    flags: vec![
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                    ],
                },
                Contour {
                    positions: vec![
                        Vector2F::new(117.0, 88.0),
                        Vector2F::new(135.0, 106.0),
                        Vector2F::new(160.0, 106.0),
                        Vector2F::new(185.0, 106.0),
                        Vector2F::new(202.0, 88.0),
                        Vector2F::new(220.0, 71.0),
                        Vector2F::new(220.0, 46.0),
                        Vector2F::new(220.0, 21.0),
                        Vector2F::new(202.0, 3.0),
                        Vector2F::new(185.0, -14.0),
                        Vector2F::new(160.0, -14.0),
                        Vector2F::new(135.0, -14.0),
                        Vector2F::new(117.0, 3.0),
                        Vector2F::new(100.0, 21.0),
                        Vector2F::new(100.0, 46.0),
                        Vector2F::new(100.0, 71.0),
                        Vector2F::new(117.0, 88.0),
                    ],
                    flags: vec![
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                        PointFlags::CONTROL_POINT_0,
                        PointFlags::empty(),
                    ],
                },
            ],
        }
    );
}

// https://github.com/pcwalton/pathfinder/issues/84
#[allow(non_snake_case)]
#[test]
fn get_glyph_outline_inconsolata_J() {
    let mut file = File::open(FILE_PATH_INCONSOLATA_TTF).unwrap();
    let font = Font::from_file(&mut file, 0).unwrap();
    let glyph = font.glyph_for_char('J').expect("No glyph for char!");
    let mut outline_builder = OutlineBuilder::new();
    font.outline(glyph, HintingOptions::None, &mut outline_builder)
        .unwrap();

    let outline = outline_builder.into_outline();
    assert_eq!(
        outline,
        Outline {
            contours: vec![Contour {
                positions: vec![
                    Vector2F::new(198.0, -11.0),
                    Vector2F::new(106.0, -11.0),
                    Vector2F::new(49.0, 58.0),
                    Vector2F::new(89.0, 108.0),
                    Vector2F::new(96.0, 116.0),
                    Vector2F::new(101.0, 112.0),
                    Vector2F::new(102.0, 102.0),
                    Vector2F::new(106.0, 95.0),
                    Vector2F::new(110.0, 88.0),
                    Vector2F::new(122.0, 78.0),
                    Vector2F::new(157.0, 51.0),
                    Vector2F::new(196.0, 51.0),
                    Vector2F::new(247.0, 51.0),
                    Vector2F::new(269.5, 86.5),
                    Vector2F::new(292.0, 122.0),
                    Vector2F::new(292.0, 208.0),
                    Vector2F::new(292.0, 564.0),
                    Vector2F::new(172.0, 564.0),
                    Vector2F::new(172.0, 623.0),
                    Vector2F::new(457.0, 623.0),
                    Vector2F::new(457.0, 564.0),
                    Vector2F::new(361.0, 564.0),
                    Vector2F::new(361.0, 209.0),
                    Vector2F::new(363.0, 133.0),
                    Vector2F::new(341.0, 84.0),
                    Vector2F::new(319.0, 35.0),
                    Vector2F::new(281.5, 12.0),
                    Vector2F::new(244.0, -11.0),
                    Vector2F::new(198.0, -11.0),
                ],
                flags: vec![
                    PointFlags::empty(),
                    PointFlags::CONTROL_POINT_0,
                    PointFlags::empty(),
                    PointFlags::empty(),
                    PointFlags::empty(),
                    PointFlags::empty(),
                    PointFlags::CONTROL_POINT_0,
                    PointFlags::empty(),
                    PointFlags::CONTROL_POINT_0,
                    PointFlags::empty(),
                    PointFlags::CONTROL_POINT_0,
                    PointFlags::empty(),
                    PointFlags::CONTROL_POINT_0,
                    PointFlags::empty(),
                    PointFlags::CONTROL_POINT_0,
                    PointFlags::empty(),
                    PointFlags::empty(),
                    PointFlags::empty(),
                    PointFlags::empty(),
                    PointFlags::empty(),
                    PointFlags::empty(),
                    PointFlags::empty(),
                    PointFlags::empty(),
                    PointFlags::CONTROL_POINT_0,
                    PointFlags::empty(),
                    PointFlags::CONTROL_POINT_0,
                    PointFlags::empty(),
                    PointFlags::CONTROL_POINT_0,
                    PointFlags::empty(),
                ],
            }],
        }
    );
}

// Makes sure that a canvas has an "L" shape in it. This is used to test rasterization.
#[allow(non_snake_case)]
fn check_L_shape(canvas: &Canvas) {
    // Find any empty rows at the start.
    let mut y = 0;
    while y < canvas.size.y() {
        let (row_start, row_end) = (canvas.stride * y as usize, canvas.stride * (y + 1) as usize);
        if canvas.pixels[row_start..row_end].iter().any(|&p| p != 0) {
            break;
        }
        y += 1;
    }
    assert!(y < canvas.size.y());

    // Find the top part of the L.
    let mut top_stripe_width = None;
    while y < canvas.size.y() {
        let (row_start, row_end) = (canvas.stride * y as usize, canvas.stride * (y + 1) as usize);
        if let Some(stripe_width) = stripe_width(&canvas.pixels[row_start..row_end]) {
            if let Some(top_stripe_width) = top_stripe_width {
                if stripe_width > top_stripe_width {
                    break;
                }
                assert_eq!(stripe_width, top_stripe_width);
            }
            top_stripe_width = Some(stripe_width);
        }
        y += 1;
    }
    assert!(y < canvas.size.y());

    // Find the bottom part of the L.
    let mut bottom_stripe_width = None;
    while y < canvas.size.y() {
        let (row_start, row_end) = (canvas.stride * y as usize, canvas.stride * (y + 1) as usize);
        y += 1;
        if let Some(stripe_width) = stripe_width(&canvas.pixels[row_start..row_end]) {
            if let Some(bottom_stripe_width) = bottom_stripe_width {
                assert!(bottom_stripe_width > top_stripe_width.unwrap());
                assert_eq!(stripe_width, bottom_stripe_width);
            }
            bottom_stripe_width = Some(stripe_width);
        }
    }

    // Find any empty rows at the end.
    while y < canvas.size.y() {
        let (row_start, row_end) = (canvas.stride * y as usize, canvas.stride * (y + 1) as usize);
        y += 1;
        if canvas.pixels[row_start..row_end].iter().any(|&p| p != 0) {
            break;
        }
    }

    // Make sure we made it to the end.
    assert_eq!(y, canvas.size.y());
}

// Makes sure that a canvas has an "{" shape in it. This is used to test rasterization.
#[cfg(target_family = "windows")]
fn check_curly_shape(canvas: &Canvas) {
    let mut y = 0;
    let height = canvas.size.y();
    // check the upper row and the lower rows are symmetrical
    while y < height / 2 {
        let (upper_row_start, upper_row_end) =
            (canvas.stride * y as usize, canvas.stride * (y + 1) as usize);
        let (lower_row_start, lower_row_end) = (
            canvas.stride * (height - y - 1) as usize,
            canvas.stride * (height - y) as usize,
        );
        let upper_row = &canvas.pixels[upper_row_start..upper_row_end];
        let lower_row = &canvas.pixels[lower_row_start..lower_row_end];
        let top_row_width = stripe_width(upper_row).unwrap();
        let lower_row_width = stripe_width(lower_row).unwrap();
        let upper_row_pixel_start = stride_pixel_start(upper_row).unwrap();
        let lower_row_pixel_start = stride_pixel_start(lower_row).unwrap();
        if top_row_width == lower_row_width {
            // check non-zero pixels start at the same index
            assert_eq!(upper_row_pixel_start, lower_row_pixel_start);
        } else {
            //if not, assert that the difference is not greater than 1
            assert_eq!((lower_row_width as i32 - top_row_width as i32).abs(), 1);
            // and assert that the non-zero pixel index difference is not greater than 1
            assert_eq!(
                (upper_row_pixel_start as i32 - lower_row_pixel_start as i32).abs(),
                1
            );
        }
        y += 1;
    }
}

// return the first non-zero pixel index
#[cfg(target_family = "windows")]
fn stride_pixel_start(pixels: &[u8]) -> Option<u32> {
    let mut index = 0;
    for x in pixels {
        if *x != 0 {
            return Some(index);
        }
        index += 1;
    }
    None
}

fn stripe_width(pixels: &[u8]) -> Option<u32> {
    let mut x = 0;
    // Find the initial empty part.
    while x < pixels.len() && pixels[x] == 0 {
        x += 1
    }
    if x == pixels.len() {
        return None;
    }
    // Find the stripe width.
    let mut stripe_width = 0;
    while x < pixels.len() && pixels[x] != 0 {
        x += 1;
        stripe_width += 1;
    }
    // Find the last empty part.
    while x < pixels.len() && pixels[x] == 0 {
        x += 1;
    }
    assert_eq!(x, pixels.len());
    Some(stripe_width)
}
