#[cfg(test)]
pub mod test_glyph_defs;

use read_fonts::{
    tables::colr::{CompositeMode, Extend},
    types::{BoundingBox, GlyphId, Point},
    FontRef,
};
use serde::{Deserialize, Serialize};

use std::{
    env,
    fs::OpenOptions,
    io::{self, BufRead, Write},
    string::String,
};

use crate::{
    alloc::vec::Vec,
    color::{
        transform::Transform, traversal_tests::test_glyph_defs::*, Brush, ColorPainter, ColorStop,
    },
    setting::VariationSetting,
    MetadataProvider,
};

#[derive(Serialize, Deserialize, Default, PartialEq)]
struct PaintDump {
    glyph_id: u32,
    ops: Vec<PaintOps>,
}

impl From<Brush<'_>> for BrushParams {
    fn from(brush: Brush) -> Self {
        match brush {
            Brush::Solid {
                palette_index,
                alpha,
            } => BrushParams::Solid {
                palette_index,
                alpha,
            },
            Brush::LinearGradient {
                p0,
                p1,
                color_stops,
                extend,
            } => BrushParams::LinearGradient {
                p0,
                p1,
                color_stops: color_stops.to_vec(),
                extend,
            },
            Brush::RadialGradient {
                c0,
                r0,
                c1,
                r1,
                color_stops,
                extend,
            } => BrushParams::RadialGradient {
                c0,
                r0,
                c1,
                r1,
                color_stops: color_stops.to_vec(),
                extend,
            },
            Brush::SweepGradient {
                c0,
                start_angle,
                end_angle,
                color_stops,
                extend,
            } => BrushParams::SweepGradient {
                c0,
                start_angle,
                end_angle,
                color_stops: color_stops.to_vec(),
                extend,
            },
        }
    }
}

// Needed as a mirror struct with owned ColorStops for serialization, deserialization.
#[derive(Serialize, Deserialize, PartialEq)]
pub enum BrushParams {
    Solid {
        palette_index: u16,
        alpha: f32,
    },
    // Normalized to a straight line between points p0 and p1,
    // color stops normalized to align with both points.
    LinearGradient {
        p0: Point<f32>,
        p1: Point<f32>,
        color_stops: Vec<ColorStop>,
        extend: Extend,
    },
    RadialGradient {
        c0: Point<f32>,
        r0: f32,
        c1: Point<f32>,
        r1: f32,
        color_stops: Vec<ColorStop>,
        extend: Extend,
    },
    SweepGradient {
        c0: Point<f32>,
        start_angle: f32,
        end_angle: f32,
        color_stops: Vec<ColorStop>,
        extend: Extend,
    },
}

// Wrapping Transform for tests, as the results of trigonometric functions, in
// particular the tan() cases in PaintSkew need floating point PartialEq
// comparisons with an epsilon because the result of the tan() function differs
// on different platforms/archictectures.
#[derive(Serialize, Deserialize)]
struct DumpTransform(Transform);

// Using the same value as in SK_ScalarNearlyZero from Skia (see SkScalar.h).
const NEARLY_EQUAL_TOLERANCE: f32 = 1.0 / (1 << 12) as f32;

fn nearly_equal(a: f32, b: f32) -> bool {
    (a - b).abs() < NEARLY_EQUAL_TOLERANCE
}

impl PartialEq<DumpTransform> for DumpTransform {
    fn eq(&self, other: &DumpTransform) -> bool {
        nearly_equal(self.0.xx, other.0.xx)
            && nearly_equal(self.0.xy, other.0.xy)
            && nearly_equal(self.0.yx, other.0.yx)
            && nearly_equal(self.0.yy, other.0.yy)
            && nearly_equal(self.0.dx, other.0.dx)
            && nearly_equal(self.0.dy, other.0.dy)
    }
}

impl From<Transform> for DumpTransform {
    fn from(value: Transform) -> Self {
        Self(value)
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
enum PaintOps {
    PushTransform {
        transform: DumpTransform,
    },
    PopTransform,
    PushClipGlyph {
        gid: u32,
    },
    PushClipBox {
        clip_box: BoundingBox<f32>,
    },
    PopClip,
    FillBrush {
        brush: BrushParams,
    },
    FillGlyph {
        gid: u32,
        transform: DumpTransform,
        brush: BrushParams,
    },
    PushLayer {
        composite_mode: u8,
    },
    PopLayer,
}

impl ColorPainter for PaintDump {
    fn push_transform(&mut self, transform: Transform) {
        self.ops.push(PaintOps::PushTransform {
            transform: transform.into(),
        });
    }
    fn pop_transform(&mut self) {
        self.ops.push(PaintOps::PopTransform);
    }

    fn push_clip_glyph(&mut self, glyph: GlyphId) {
        self.ops.push(PaintOps::PushClipGlyph {
            gid: glyph.to_u32(),
        });
    }

    fn push_clip_box(&mut self, clip_box: BoundingBox<f32>) {
        self.ops.push(PaintOps::PushClipBox { clip_box });
    }

    fn pop_clip(&mut self) {
        self.ops.push(PaintOps::PopClip);
    }

    fn fill(&mut self, brush: Brush) {
        self.ops.push(PaintOps::FillBrush {
            brush: brush.into(),
        });
    }

    fn fill_glyph(&mut self, glyph_id: GlyphId, transform: Option<Transform>, brush: Brush) {
        self.ops.push(PaintOps::FillGlyph {
            gid: glyph_id.to_u32(),
            transform: transform.unwrap_or_default().into(),
            brush: brush.into(),
        });
    }

    fn push_layer(&mut self, composite_mode: CompositeMode) {
        self.ops.push(PaintOps::PushLayer {
            composite_mode: composite_mode as u8,
        });
    }
    fn pop_layer(&mut self) {
        self.ops.push(PaintOps::PopLayer);
    }
}

impl PaintDump {
    pub fn new(gid: u32) -> Self {
        Self {
            glyph_id: gid,
            ..Default::default()
        }
    }
}

fn location_to_filename<I>(set_name: &str, settings: I) -> String
where
    I: IntoIterator,
    I::Item: Into<VariationSetting>,
{
    let formatted_settings: Vec<String> = settings
        .into_iter()
        .map(|entry| {
            let entry_setting = entry.into();
            format!("{:}_{:}", entry_setting.selector, entry_setting.value)
        })
        .collect();
    let suffix = match formatted_settings.len() {
        0 => String::new(),
        _ => format!("_{}", formatted_settings.join("_")),
    };
    format!("colrv1_{}{}", set_name.to_lowercase(), suffix)
}

fn should_rebaseline() -> bool {
    env::var("REBASELINE_COLRV1_TESTS").is_ok()
}

// To regenerate the baselines, set the environment variable `REBASELINE_COLRV1_TESTS`
// when running tests, for example like this:
// $ REBASELINE_COLRV1_TESTS=1 cargo test color::traversal
fn colrv1_traversal_test(
    set_name: &str,
    test_chars: &[char],
    settings: &[(&str, f32)],
    required_format: crate::color::ColorGlyphFormat,
) {
    let colr_font = font_test_data::COLRV0V1_VARIABLE;
    let font = FontRef::new(colr_font).unwrap();

    let location = font.axes().location(settings);

    let dumpfile_path = format!(
        "../font-test-data/test_data/colrv1_json/{}",
        location_to_filename(set_name, settings)
    );

    let test_gids = test_chars
        .iter()
        .map(|codepoint| font.charmap().map(*codepoint).unwrap());

    let paint_dumps_iter = test_gids.map(|gid| {
        let mut color_painter = PaintDump::new(gid.to_u32());

        let color_glyph = font.color_glyphs().get_with_format(gid, required_format);

        assert!(color_glyph.is_some());

        let result = color_glyph
            .unwrap()
            .paint(location.coords(), &mut color_painter);

        assert!(result.is_ok());

        color_painter
    });

    if should_rebaseline() {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(dumpfile_path)
            .unwrap();

        paint_dumps_iter.for_each(|dump| {
            writeln!(file, "{}", serde_json::to_string(&dump).unwrap())
                .expect("Writing to file failed.")
        });
    } else {
        let expected = font_test_data::colrv1_json::expected(set_name, settings);
        let mut lines = io::BufReader::new(expected.as_bytes()).lines();
        for dump in paint_dumps_iter {
            match lines.next() {
                Some(line) => {
                    assert!(
                        dump == serde_json::from_str(
                            line.as_ref().expect("Failed to read expectations line from file.")
                        )
                        .expect("Failed to parse expectations line."),
                        "Result did not match expectation for glyph id: {}\nActual: {}\nExpected: {}\n",
                        dump.glyph_id, serde_json::to_string(&dump).unwrap(), &line.unwrap()
                    )
                }
                None => panic!("Expectation not found for glyph id: {}", dump.glyph_id),
            }
        }
    }
}

macro_rules! colrv1_traversal_tests {
        ($($test_name:ident: $glyph_set:ident, $settings:expr,)*) => {
        $(
            #[test]
            fn $test_name() {
                colrv1_traversal_test(stringify!($glyph_set), $glyph_set, $settings, crate::color::ColorGlyphFormat::ColrV1);
            }
        )*
    }
}

colrv1_traversal_tests!(
clipbox_default:CLIPBOX,&[],
clipbox_var_1:CLIPBOX, &[("CLIO", 200.0)],
comp_mode_default:COMPOSITE_MODE,&[],
extend_mode_default:EXTEND_MODE,&[],
extend_mode_var1:EXTEND_MODE,&[("COL1", -0.25), ("COL3", 0.25)],
extend_mode_var2:EXTEND_MODE,&[("COL1", 0.5), ("COL3", -0.5)],
extend_mode_var3:EXTEND_MODE,&[("COL3", 0.5)],
extend_mode_var4:EXTEND_MODE,&[("COL3", 1.0)],
extend_mode_var5:EXTEND_MODE,&[("COL1", -1.5)],
extend_mode_var6:EXTEND_MODE,&[("GRR0", -200.0), ("GRR1", -300.0)],
extend_mode_var7:EXTEND_MODE,&[("GRX0", -1000.0), ("GRX1", -1000.0), ("GRR0", -1000.0), ("GRR1", -900.0)],
extend_mode_var8:EXTEND_MODE,&[("GRX0", 1000.0), ("GRX1", -1000.0), ("GRR0", -1000.0), ("GRR1", 200.0)],
extend_mode_var9:EXTEND_MODE,&[("GRR0", -50.0), ("COL3", -2.0), ("COL2", -2.0), ("COL1", -0.9)],
extend_mode_var10:EXTEND_MODE,&[("GRR0", -50.0), ("COL3", -2.0), ("COL2", -2.0), ("COL1", -1.1)],
extend_mode_var11:EXTEND_MODE,&[("COL3", 1.0), ("COL2", 1.5), ("COL1", 2.0)],
extend_mode_var12:EXTEND_MODE,&[("COL2", -0.3)],
extend_mode_var13:EXTEND_MODE,&[("GRR0", 430.0), ("GRR1", 40.0)],
foreground_color_default:FOREGROUND_COLOR,&[],
gradient_skewed:GRADIENT_P2_SKEWED,&[],
gradient_stops_repeat:GRADIENT_STOPS_REPEAT,&[],
paint_rotate_default:PAINT_ROTATE,&[],
paint_rotate_var1:PAINT_ROTATE,&[("ROTA", 40.0)],
paint_rotate_var2:PAINT_ROTATE,&[("ROTX", -250.0), ("ROTY", -250.0)],
paint_scale_default:PAINT_SCALE,&[],
paint_scale_var1:PAINT_SCALE,&[("SCOX", 200.0), ("SCOY", 200.0)],
paint_scale_var2:PAINT_SCALE,&[("SCSX", 0.25), ("SCOY", 0.25)],
paint_scale_var3:PAINT_SCALE,&[("SCSX", -1.0), ("SCOY", -1.0)],
paint_skew_default:PAINT_SKEW,&[],
paint_skew_var1:PAINT_SKEW,&[("SKXA", 20.0)],
paint_skew_var2:PAINT_SKEW,&[("SKYA", 20.0)],
paint_skew_var3:PAINT_SKEW,&[("SKCX", 200.0),("SKCY", 200.0)],
paint_transform_default:PAINT_TRANSFORM,&[],
paint_translate_default:PAINT_TRANSLATE,&[],
paint_translate_var_1:PAINT_TRANSLATE,&[("TLDX", 100.0), ("TLDY", 100.0)],
paint_sweep_default:SWEEP_VARSWEEP,&[],
paint_sweep_var1:SWEEP_VARSWEEP,&[("SWPS", 0.0)],
paint_sweep_var2:SWEEP_VARSWEEP,&[("SWPS", 90.0)],
paint_sweep_var3:SWEEP_VARSWEEP,&[("SWPE", -90.0)],
paint_sweep_var4:SWEEP_VARSWEEP,&[("SWPE", -45.0)],
paint_sweep_var5:SWEEP_VARSWEEP,&[("SWPS", -45.0),("SWPE", 45.0)],
paint_sweep_var6:SWEEP_VARSWEEP,&[("SWC1", -0.25), ("SWC2", 0.083333333), ("SWC3", 0.083333333), ("SWC4", 0.25)],
paint_sweep_var7:SWEEP_VARSWEEP,&[("SWPS", 45.0), ("SWPE", -45.0), ("SWC1", -0.25), ("SWC2", -0.416687), ("SWC3", -0.583313), ("SWC4", -0.75)],
variable_alpha_default:VARIABLE_ALPHA,&[],
variable_alpha_var1:VARIABLE_ALPHA,&[("APH1", -0.7)],
variable_alpha_var2:VARIABLE_ALPHA,&[("APH2", -0.7), ("APH3", -0.2)],
nocycle_multi_colrglyph:NO_CYCLE_MULTI_COLRGLYPH,&[],
sweep_coincident:SWEEP_COINCIDENT,&[],
paint_glyph_nested:PAINT_GLYPH_NESTED,&[],
);

macro_rules! colrv0_traversal_tests {
    ($($test_name:ident: $glyph_set:ident,)*) => {
    $(
        #[test]
        fn $test_name() {
            colrv1_traversal_test(stringify!($glyph_set), $glyph_set, &[], crate::color::ColorGlyphFormat::ColrV0);
        }
    )*
}
}

colrv0_traversal_tests!(
    colored_circles:COLORED_CIRCLES_V0,
);
