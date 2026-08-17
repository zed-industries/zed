use euclid::default::Point2D;
use euclid::Trig;
use num_traits::{Float, FromPrimitive};
use palette::Srgba;
use rand::random;
use std::fmt;

#[derive(Clone, PartialEq, Debug, Copy, Eq)]
pub enum FillStyle {
    Solid,
    Hachure,
    ZigZag,
    CrossHatch,
    Dots,
    Dashed,
    ZigZagLine,
}

impl fmt::Display for FillStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FillStyle::Solid => "Solid",
            FillStyle::Hachure => "Hachure",
            FillStyle::ZigZag => "ZigZag",
            FillStyle::CrossHatch => "CrossHatch",
            FillStyle::Dots => "Dots",
            FillStyle::Dashed => "Dashed",
            FillStyle::ZigZagLine => "ZigZagLine",
        };
        f.write_str(s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LineCap {
    #[default]
    Butt,
    Round,
    Square,
}

/// Options for angled joins in strokes.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LineJoin {
    Miter { limit: f64 },
    Round,
    Bevel,
}
impl LineJoin {
    pub const DEFAULT_MITER_LIMIT: f64 = 10.0;
}
impl Default for LineJoin {
    fn default() -> Self {
        LineJoin::Miter {
            limit: LineJoin::DEFAULT_MITER_LIMIT,
        }
    }
}

#[derive(Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Options {
    #[builder(default = "Some(2.0)")]
    pub max_randomness_offset: Option<f32>,
    #[builder(default = "Some(1.0)")]
    pub roughness: Option<f32>,
    #[builder(default = "Some(2.0)")]
    pub bowing: Option<f32>,
    #[builder(default = "Some(Srgba::new(0.0, 0.0, 0.0, 1.0))")]
    pub stroke: Option<Srgba>,
    #[builder(default = "Some(1.0)")]
    pub stroke_width: Option<f32>,
    #[builder(default = "Some(0.95)")]
    pub curve_fitting: Option<f32>,
    #[builder(default = "Some(0.0)")]
    pub curve_tightness: Option<f32>,
    #[builder(default = "Some(9.0)")]
    pub curve_step_count: Option<f32>,
    #[builder(default = "None")]
    pub fill: Option<Srgba>,
    #[builder(default = "None")]
    pub fill_style: Option<FillStyle>,
    #[builder(default = "Some(-1.0)")]
    pub fill_weight: Option<f32>,
    #[builder(default = "Some(-41.0)")]
    pub hachure_angle: Option<f32>,
    #[builder(default = "Some(-1.0)")]
    pub hachure_gap: Option<f32>,
    #[builder(default = "Some(1.0)")]
    pub simplification: Option<f32>,
    #[builder(default = "Some(-1.0)")]
    pub dash_offset: Option<f32>,
    #[builder(default = "Some(-1.0)")]
    pub dash_gap: Option<f32>,
    #[builder(default = "Some(-1.0)")]
    pub zigzag_offset: Option<f32>,
    #[builder(default = "Some(0_u64)")]
    pub seed: Option<u64>,
    #[builder(default = "None")]
    pub stroke_line_dash: Option<Vec<f64>>,
    #[builder(default = "None")]
    pub stroke_line_dash_offset: Option<f64>,
    #[builder(default = "None")]
    pub line_cap: Option<LineCap>,
    #[builder(default = "None")]
    pub line_join: Option<LineJoin>,
    #[builder(default = "None")]
    pub fill_line_dash: Option<Vec<f64>>,
    #[builder(default = "None")]
    pub fill_line_dash_offset: Option<f64>,
    #[builder(default = "Some(false)")]
    pub disable_multi_stroke: Option<bool>,
    #[builder(default = "Some(false)")]
    pub disable_multi_stroke_fill: Option<bool>,
    #[builder(default = "Some(false)")]
    pub preserve_vertices: Option<bool>,
    #[builder(default = "None")]
    pub fixed_decimal_place_digits: Option<f32>,
    // Rough.js stores the evolving PRNG state in `ops.randomizer` (not in `ops.seed`).
    // This is internal-only and must not be user-set.
    #[builder(default = "None", setter(skip))]
    pub(crate) randomizer: Option<i32>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            max_randomness_offset: Some(2.0),
            roughness: Some(1.0),
            bowing: Some(1.0),
            stroke: Some(Srgba::new(0.0, 0.0, 0.0, 1.0)),
            stroke_width: Some(1.0),
            curve_tightness: Some(0.0),
            curve_fitting: Some(0.95),
            curve_step_count: Some(9.0),
            fill: None,
            fill_style: None,
            fill_weight: Some(-1.0),
            hachure_angle: Some(-41.0),
            hachure_gap: Some(-1.0),
            dash_offset: Some(-1.0),
            dash_gap: Some(-1.0),
            zigzag_offset: Some(-1.0),
            seed: Some(0_u64),
            disable_multi_stroke: Some(false),
            disable_multi_stroke_fill: Some(false),
            preserve_vertices: Some(false),
            simplification: Some(1.0),
            stroke_line_dash: None,
            stroke_line_dash_offset: None,
            line_cap: None,
            line_join: None,
            fill_line_dash: None,
            fill_line_dash_offset: None,
            fixed_decimal_place_digits: None,
            randomizer: None,
        }
    }
}

impl Options {
    pub fn random(&mut self) -> f64 {
        // Match Rough.js `random(ops)` in `bin/renderer.js`:
        //
        // - `ops.seed` is the *base seed* (stable across calls).
        // - `ops.randomizer` is lazily created and holds the evolving 32-bit state.
        // - If seed is `0` (falsy), `Random.next()` falls back to `Math.random()` without
        //   advancing state (but `ops.randomizer` still exists and stays falsy).
        if self.randomizer.is_none() {
            let seed_bits = self.seed.unwrap_or(0) as u32;
            self.randomizer = Some(seed_bits as i32);
        }

        let state = self.randomizer.unwrap_or(0);
        if state != 0 {
            // Match Rough.js `Random.next()` from `bin/math.js`:
            //
            // `return ((2 ** 31 - 1) & (this.seed = Math.imul(48271, this.seed))) / 2 ** 31;`
            //
            // - `Math.imul` is a signed 32-bit multiply
            // - assignment stores the raw signed 32-bit result
            // - returned value is masked with `& 0x7fffffff`
            let next = state.wrapping_mul(48271);
            self.randomizer = Some(next);
            let out = next & 0x7fffffff;
            return (out as f64) / 2147483648.0;
        }

        random::<f64>()
    }

    pub fn set_hachure_angle(&mut self, angle: Option<f32>) -> &mut Self {
        self.hachure_angle = angle;
        self
    }

    pub fn set_hachure_gap(&mut self, gap: Option<f32>) -> &mut Self {
        self.hachure_gap = gap;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Options, OptionsBuilder};

    #[test]
    fn roughjs_random_seed_1_matches_known_sequence() {
        // Matches Rough.js `Random.next()` from `bin/math.js` with `seed = 1`.
        let denom = 2147483648.0_f64; // 2^31
        let expected_out: [u32; 10] = [
            48_271,
            182_605_793,
            1_291_342_511,
            1_533_981_633,
            1_591_223_503,
            902_075_297,
            1_698_214_639,
            773_027_713,
            144_866_575,
            647_683_937,
        ];
        let expected: Vec<f64> = expected_out.iter().map(|&n| (n as f64) / denom).collect();

        let mut opts: Options = OptionsBuilder::default().seed(1_u64).build().unwrap();
        let got: Vec<f64> = (0..expected.len()).map(|_| opts.random()).collect();

        assert_eq!(got, expected);
    }
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum OpType {
    Move,
    BCurveTo,
    LineTo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpSetType {
    Path,
    FillPath,
    FillSketch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Op<F: Float + Trig> {
    pub op: OpType,
    pub data: Vec<F>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpSet<F: Float + Trig> {
    pub op_set_type: OpSetType,
    pub ops: Vec<Op<F>>,
    pub size: Option<Point2D<F>>,
    pub path: Option<String>,
}

pub struct Drawable<F: Float + Trig> {
    pub shape: String,
    pub options: Options,
    pub sets: Vec<OpSet<F>>,
}

pub struct PathInfo {
    pub d: String,
    pub stroke: Option<Srgba>,
    pub stroke_width: Option<f32>,
    pub fill: Option<Srgba>,
}

pub fn _c<U: Float + FromPrimitive>(inp: f32) -> U {
    U::from(inp).expect("can not parse from f32")
}

pub fn _cc<U: Float + FromPrimitive>(inp: f64) -> U {
    U::from(inp).expect("can not parse from f64")
}
