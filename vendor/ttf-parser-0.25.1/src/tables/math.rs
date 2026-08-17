//! A [Math Table](https://docs.microsoft.com/en-us/typography/opentype/spec/math) implementation.

use crate::gpos::Device;
use crate::opentype_layout::Coverage;
use crate::parser::{
    FromData, FromSlice, LazyArray16, LazyOffsetArray16, Offset, Offset16, Stream,
};
use crate::GlyphId;

/// A [Math Value](https://docs.microsoft.com/en-us/typography/opentype/spec/math#mathvaluerecord)
/// with optional device corrections.
#[derive(Clone, Copy, Debug)]
pub struct MathValue<'a> {
    /// The X or Y value in font design units.
    pub value: i16,
    /// Device corrections for this value.
    pub device: Option<Device<'a>>,
}

impl<'a> MathValue<'a> {
    fn parse(data: &'a [u8], parent: &'a [u8]) -> Option<Self> {
        Some(MathValueRecord::parse(data)?.get(parent))
    }
}

/// A math value record with unresolved offset.
#[derive(Clone, Copy)]
struct MathValueRecord {
    value: i16,
    device_offset: Option<Offset16>,
}

impl FromData for MathValueRecord {
    const SIZE: usize = 4;

    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let value = s.read::<i16>()?;
        let device_offset = s.read::<Option<Offset16>>()?;
        Some(MathValueRecord {
            value,
            device_offset,
        })
    }
}

impl MathValueRecord {
    fn get(self, data: &[u8]) -> MathValue {
        let device = self
            .device_offset
            .and_then(|offset| data.get(offset.to_usize()..))
            .and_then(Device::parse);
        MathValue {
            value: self.value,
            device,
        }
    }
}

/// A mapping from glyphs to
/// [Math Values](https://docs.microsoft.com/en-us/typography/opentype/spec/math#mathvaluerecord).
#[derive(Clone, Copy)]
pub struct MathValues<'a> {
    data: &'a [u8],
    coverage: Coverage<'a>,
    records: LazyArray16<'a, MathValueRecord>,
}

impl<'a> FromSlice<'a> for MathValues<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let coverage = s.parse_at_offset16::<Coverage>(data)?;
        let count = s.read::<u16>()?;
        let records = s.read_array16::<MathValueRecord>(count)?;
        Some(MathValues {
            data,
            coverage,
            records,
        })
    }
}

impl<'a> MathValues<'a> {
    /// Returns the value for the glyph or `None` if it is not covered.
    #[inline]
    pub fn get(&self, glyph: GlyphId) -> Option<MathValue<'a>> {
        let index = self.coverage.get(glyph)?;
        Some(self.records.get(index)?.get(self.data))
    }
}

impl core::fmt::Debug for MathValues<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "MathValues {{ ... }}")
    }
}

/// A [Math Constants Table](https://learn.microsoft.com/en-us/typography/opentype/spec/math#mathconstants-table).
#[derive(Clone, Copy)]
pub struct Constants<'a> {
    data: &'a [u8],
}

impl<'a> FromSlice<'a> for Constants<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        Some(Constants { data })
    }
}

impl core::fmt::Debug for Constants<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Constants {{ ... }}")
    }
}

const SCRIPT_PERCENT_SCALE_DOWN_OFFSET: usize = 0;
const SCRIPT_SCRIPT_PERCENT_SCALE_DOWN_OFFSET: usize = 2;
const DELIMITED_SUB_FORMULA_MIN_HEIGHT_OFFSET: usize = 4;
const DISPLAY_OPERATOR_MIN_HEIGHT_OFFSET: usize = 6;
const MATH_LEADING_OFFSET: usize = 8;
const AXIS_HEIGHT_OFFSET: usize = 12;
const ACCENT_BASE_HEIGHT_OFFSET: usize = 16;
const FLATTENED_ACCENT_BASE_HEIGHT_OFFSET: usize = 20;
const SUBSCRIPT_SHIFT_DOWN_OFFSET: usize = 24;
const SUBSCRIPT_TOP_MAX_OFFSET: usize = 28;
const SUBSCRIPT_BASELINE_DROP_MIN_OFFSET: usize = 32;
const SUPERSCRIPT_SHIFT_UP_OFFSET: usize = 36;
const SUPERSCRIPT_SHIFT_UP_CRAMPED_OFFSET: usize = 40;
const SUPERSCRIPT_BOTTOM_MIN_OFFSET: usize = 44;
const SUPERSCRIPT_BASELINE_DROP_MAX_OFFSET: usize = 48;
const SUB_SUPERSCRIPT_GAP_MIN_OFFSET: usize = 52;
const SUPERSCRIPT_BOTTOM_MAX_WITH_SUBSCRIPT_OFFSET: usize = 56;
const SPACE_AFTER_SCRIPT_OFFSET: usize = 60;
const UPPER_LIMIT_GAP_MIN_OFFSET: usize = 64;
const UPPER_LIMIT_BASELINE_RISE_MIN_OFFSET: usize = 68;
const LOWER_LIMIT_GAP_MIN_OFFSET: usize = 72;
const LOWER_LIMIT_BASELINE_DROP_MIN_OFFSET: usize = 76;
const STACK_TOP_SHIFT_UP_OFFSET: usize = 80;
const STACK_TOP_DISPLAY_STYLE_SHIFT_UP_OFFSET: usize = 84;
const STACK_BOTTOM_SHIFT_DOWN_OFFSET: usize = 88;
const STACK_BOTTOM_DISPLAY_STYLE_SHIFT_DOWN_OFFSET: usize = 92;
const STACK_GAP_MIN_OFFSET: usize = 96;
const STACK_DISPLAY_STYLE_GAP_MIN_OFFSET: usize = 100;
const STRETCH_STACK_TOP_SHIFT_UP_OFFSET: usize = 104;
const STRETCH_STACK_BOTTOM_SHIFT_DOWN_OFFSET: usize = 108;
const STRETCH_STACK_GAP_ABOVE_MIN_OFFSET: usize = 112;
const STRETCH_STACK_GAP_BELOW_MIN_OFFSET: usize = 116;
const FRACTION_NUMERATOR_SHIFT_UP_OFFSET: usize = 120;
const FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP_OFFSET: usize = 124;
const FRACTION_DENOMINATOR_SHIFT_DOWN_OFFSET: usize = 128;
const FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN_OFFSET: usize = 132;
const FRACTION_NUMERATOR_GAP_MIN_OFFSET: usize = 136;
const FRACTION_NUM_DISPLAY_STYLE_GAP_MIN_OFFSET: usize = 140;
const FRACTION_RULE_THICKNESS_OFFSET: usize = 144;
const FRACTION_DENOMINATOR_GAP_MIN_OFFSET: usize = 148;
const FRACTION_DENOM_DISPLAY_STYLE_GAP_MIN_OFFSET: usize = 152;
const SKEWED_FRACTION_HORIZONTAL_GAP_OFFSET: usize = 156;
const SKEWED_FRACTION_VERTICAL_GAP_OFFSET: usize = 160;
const OVERBAR_VERTICAL_GAP_OFFSET: usize = 164;
const OVERBAR_RULE_THICKNESS_OFFSET: usize = 168;
const OVERBAR_EXTRA_ASCENDER_OFFSET: usize = 172;
const UNDERBAR_VERTICAL_GAP_OFFSET: usize = 176;
const UNDERBAR_RULE_THICKNESS_OFFSET: usize = 180;
const UNDERBAR_EXTRA_DESCENDER_OFFSET: usize = 184;
const RADICAL_VERTICAL_GAP_OFFSET: usize = 188;
const RADICAL_DISPLAY_STYLE_VERTICAL_GAP_OFFSET: usize = 192;
const RADICAL_RULE_THICKNESS_OFFSET: usize = 196;
const RADICAL_EXTRA_ASCENDER_OFFSET: usize = 200;
const RADICAL_KERN_BEFORE_DEGREE_OFFSET: usize = 204;
const RADICAL_KERN_AFTER_DEGREE_OFFSET: usize = 208;
const RADICAL_DEGREE_BOTTOM_RAISE_PERCENT_OFFSET: usize = 212;

impl<'a> Constants<'a> {
    /// Percentage of scaling down for level 1 superscripts and subscripts.
    #[inline]
    pub fn script_percent_scale_down(&self) -> i16 {
        self.read_i16(SCRIPT_PERCENT_SCALE_DOWN_OFFSET)
    }

    /// Percentage of scaling down for level 2 (scriptScript) superscripts and subscripts.
    #[inline]
    pub fn script_script_percent_scale_down(&self) -> i16 {
        self.read_i16(SCRIPT_SCRIPT_PERCENT_SCALE_DOWN_OFFSET)
    }

    /// Minimum height required for a delimited expression (contained within parentheses, etc.) to
    /// be treated as a sub-formula.
    #[inline]
    pub fn delimited_sub_formula_min_height(&self) -> u16 {
        self.read_u16(DELIMITED_SUB_FORMULA_MIN_HEIGHT_OFFSET)
    }

    /// Minimum height of n-ary operators (such as integral and summation) for formulas in display
    /// mode (that is, appearing as standalone page elements, not embedded inline within text).
    #[inline]
    pub fn display_operator_min_height(&self) -> u16 {
        self.read_u16(DISPLAY_OPERATOR_MIN_HEIGHT_OFFSET)
    }

    /// White space to be left between math formulas to ensure proper line spacing.
    #[inline]
    pub fn math_leading(&self) -> MathValue<'a> {
        self.read_record(MATH_LEADING_OFFSET)
    }

    /// Axis height of the font.
    #[inline]
    pub fn axis_height(&self) -> MathValue<'a> {
        self.read_record(AXIS_HEIGHT_OFFSET)
    }

    /// Maximum (ink) height of accent base that does not require raising the accents.
    #[inline]
    pub fn accent_base_height(&self) -> MathValue<'a> {
        self.read_record(ACCENT_BASE_HEIGHT_OFFSET)
    }

    /// Maximum (ink) height of accent base that does not require flattening the accents.
    #[inline]
    pub fn flattened_accent_base_height(&self) -> MathValue<'a> {
        self.read_record(FLATTENED_ACCENT_BASE_HEIGHT_OFFSET)
    }

    /// The standard shift down applied to subscript elements.
    #[inline]
    pub fn subscript_shift_down(&self) -> MathValue<'a> {
        self.read_record(SUBSCRIPT_SHIFT_DOWN_OFFSET)
    }

    /// Maximum allowed height of the (ink) top of subscripts that does not require moving
    /// subscripts further down.
    #[inline]
    pub fn subscript_top_max(&self) -> MathValue<'a> {
        self.read_record(SUBSCRIPT_TOP_MAX_OFFSET)
    }

    /// Minimum allowed drop of the baseline of subscripts relative to the (ink) bottom of the
    /// base.
    #[inline]
    pub fn subscript_baseline_drop_min(&self) -> MathValue<'a> {
        self.read_record(SUBSCRIPT_BASELINE_DROP_MIN_OFFSET)
    }

    /// Standard shift up applied to superscript elements.
    #[inline]
    pub fn superscript_shift_up(&self) -> MathValue<'a> {
        self.read_record(SUPERSCRIPT_SHIFT_UP_OFFSET)
    }

    /// Standard shift of superscripts relative to the base, in cramped style.
    #[inline]
    pub fn superscript_shift_up_cramped(&self) -> MathValue<'a> {
        self.read_record(SUPERSCRIPT_SHIFT_UP_CRAMPED_OFFSET)
    }

    /// Minimum allowed height of the (ink) bottom of superscripts that does not require moving
    /// subscripts further up.
    #[inline]
    pub fn superscript_bottom_min(&self) -> MathValue<'a> {
        self.read_record(SUPERSCRIPT_BOTTOM_MIN_OFFSET)
    }

    /// Maximum allowed drop of the baseline of superscripts relative to the (ink) top of the
    /// base.
    #[inline]
    pub fn superscript_baseline_drop_max(&self) -> MathValue<'a> {
        self.read_record(SUPERSCRIPT_BASELINE_DROP_MAX_OFFSET)
    }

    /// Minimum gap between the superscript and subscript ink.
    #[inline]
    pub fn sub_superscript_gap_min(&self) -> MathValue<'a> {
        self.read_record(SUB_SUPERSCRIPT_GAP_MIN_OFFSET)
    }

    /// The maximum level to which the (ink) bottom of superscript can be pushed to increase the
    /// gap between superscript and subscript, before subscript starts being moved down.
    #[inline]
    pub fn superscript_bottom_max_with_subscript(&self) -> MathValue<'a> {
        self.read_record(SUPERSCRIPT_BOTTOM_MAX_WITH_SUBSCRIPT_OFFSET)
    }

    /// Extra white space to be added after each subscript and superscript.
    #[inline]
    pub fn space_after_script(&self) -> MathValue<'a> {
        self.read_record(SPACE_AFTER_SCRIPT_OFFSET)
    }

    /// Minimum gap between the (ink) bottom of the upper limit, and the (ink) top of the base
    /// operator.
    #[inline]
    pub fn upper_limit_gap_min(&self) -> MathValue<'a> {
        self.read_record(UPPER_LIMIT_GAP_MIN_OFFSET)
    }

    /// Minimum distance between baseline of upper limit and (ink) top of the base operator.
    #[inline]
    pub fn upper_limit_baseline_rise_min(&self) -> MathValue<'a> {
        self.read_record(UPPER_LIMIT_BASELINE_RISE_MIN_OFFSET)
    }

    /// Minimum gap between (ink) top of the lower limit, and (ink) bottom of the base operator.
    #[inline]
    pub fn lower_limit_gap_min(&self) -> MathValue<'a> {
        self.read_record(LOWER_LIMIT_GAP_MIN_OFFSET)
    }

    /// Minimum distance between baseline of the lower limit and (ink) bottom of the base operator.
    #[inline]
    pub fn lower_limit_baseline_drop_min(&self) -> MathValue<'a> {
        self.read_record(LOWER_LIMIT_BASELINE_DROP_MIN_OFFSET)
    }

    /// Standard shift up applied to the top element of a stack.
    #[inline]
    pub fn stack_top_shift_up(&self) -> MathValue<'a> {
        self.read_record(STACK_TOP_SHIFT_UP_OFFSET)
    }

    /// Standard shift up applied to the top element of a stack in display style.
    #[inline]
    pub fn stack_top_display_style_shift_up(&self) -> MathValue<'a> {
        self.read_record(STACK_TOP_DISPLAY_STYLE_SHIFT_UP_OFFSET)
    }

    /// Standard shift down applied to the bottom element of a stack.
    #[inline]
    pub fn stack_bottom_shift_down(&self) -> MathValue<'a> {
        self.read_record(STACK_BOTTOM_SHIFT_DOWN_OFFSET)
    }

    /// Standard shift down applied to the bottom element of a stack in display style.
    #[inline]
    pub fn stack_bottom_display_style_shift_down(&self) -> MathValue<'a> {
        self.read_record(STACK_BOTTOM_DISPLAY_STYLE_SHIFT_DOWN_OFFSET)
    }

    /// Minimum gap between (ink) bottom of the top element of a stack, and the (ink) top of the
    /// bottom element.
    #[inline]
    pub fn stack_gap_min(&self) -> MathValue<'a> {
        self.read_record(STACK_GAP_MIN_OFFSET)
    }

    /// Minimum gap between (ink) bottom of the top element of a stack, and the (ink) top of the
    /// bottom element in display style.
    #[inline]
    pub fn stack_display_style_gap_min(&self) -> MathValue<'a> {
        self.read_record(STACK_DISPLAY_STYLE_GAP_MIN_OFFSET)
    }

    /// Standard shift up applied to the top element of the stretch stack.
    #[inline]
    pub fn stretch_stack_top_shift_up(&self) -> MathValue<'a> {
        self.read_record(STRETCH_STACK_TOP_SHIFT_UP_OFFSET)
    }

    /// Standard shift down applied to the bottom element of the stretch stack.
    #[inline]
    pub fn stretch_stack_bottom_shift_down(&self) -> MathValue<'a> {
        self.read_record(STRETCH_STACK_BOTTOM_SHIFT_DOWN_OFFSET)
    }

    /// Minimum gap between the ink of the stretched element, and the (ink) bottom of the element above.
    #[inline]
    pub fn stretch_stack_gap_above_min(&self) -> MathValue<'a> {
        self.read_record(STRETCH_STACK_GAP_ABOVE_MIN_OFFSET)
    }

    /// Minimum gap between the ink of the stretched element, and the (ink) top of the element below.
    #[inline]
    pub fn stretch_stack_gap_below_min(&self) -> MathValue<'a> {
        self.read_record(STRETCH_STACK_GAP_BELOW_MIN_OFFSET)
    }

    /// Standard shift up applied to the numerator.
    #[inline]
    pub fn fraction_numerator_shift_up(&self) -> MathValue<'a> {
        self.read_record(FRACTION_NUMERATOR_SHIFT_UP_OFFSET)
    }

    /// Standard shift up applied to the numerator in display style.
    #[inline]
    pub fn fraction_numerator_display_style_shift_up(&self) -> MathValue<'a> {
        self.read_record(FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP_OFFSET)
    }

    /// Standard shift down applied to the denominator.
    #[inline]
    pub fn fraction_denominator_shift_down(&self) -> MathValue<'a> {
        self.read_record(FRACTION_DENOMINATOR_SHIFT_DOWN_OFFSET)
    }

    /// Standard shift down applied to the denominator in display style.
    #[inline]
    pub fn fraction_denominator_display_style_shift_down(&self) -> MathValue<'a> {
        self.read_record(FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN_OFFSET)
    }

    /// Minimum tolerated gap between the (ink) bottom of the numerator and the ink of the
    /// fraction bar.
    #[inline]
    pub fn fraction_numerator_gap_min(&self) -> MathValue<'a> {
        self.read_record(FRACTION_NUMERATOR_GAP_MIN_OFFSET)
    }

    /// Minimum tolerated gap between the (ink) bottom of the numerator and the ink of the
    /// fraction bar in display style.
    #[inline]
    pub fn fraction_num_display_style_gap_min(&self) -> MathValue<'a> {
        self.read_record(FRACTION_NUM_DISPLAY_STYLE_GAP_MIN_OFFSET)
    }

    /// Thickness of the fraction bar.
    #[inline]
    pub fn fraction_rule_thickness(&self) -> MathValue<'a> {
        self.read_record(FRACTION_RULE_THICKNESS_OFFSET)
    }

    /// Minimum tolerated gap between the (ink) top of the denominator and the ink of the fraction bar.
    #[inline]
    pub fn fraction_denominator_gap_min(&self) -> MathValue<'a> {
        self.read_record(FRACTION_DENOMINATOR_GAP_MIN_OFFSET)
    }

    /// Minimum tolerated gap between the (ink) top of the denominator and the ink of the fraction
    /// bar in display style.
    #[inline]
    pub fn fraction_denom_display_style_gap_min(&self) -> MathValue<'a> {
        self.read_record(FRACTION_DENOM_DISPLAY_STYLE_GAP_MIN_OFFSET)
    }

    /// Horizontal distance between the top and bottom elements of a skewed fraction.
    #[inline]
    pub fn skewed_fraction_horizontal_gap(&self) -> MathValue<'a> {
        self.read_record(SKEWED_FRACTION_HORIZONTAL_GAP_OFFSET)
    }

    /// Vertical distance between the ink of the top and bottom elements of a skewed fraction.
    #[inline]
    pub fn skewed_fraction_vertical_gap(&self) -> MathValue<'a> {
        self.read_record(SKEWED_FRACTION_VERTICAL_GAP_OFFSET)
    }

    /// Distance between the overbar and the (ink) top of he base.
    #[inline]
    pub fn overbar_vertical_gap(&self) -> MathValue<'a> {
        self.read_record(OVERBAR_VERTICAL_GAP_OFFSET)
    }

    /// Thickness of overbar.
    #[inline]
    pub fn overbar_rule_thickness(&self) -> MathValue<'a> {
        self.read_record(OVERBAR_RULE_THICKNESS_OFFSET)
    }

    /// Extra white space reserved above the overbar.
    #[inline]
    pub fn overbar_extra_ascender(&self) -> MathValue<'a> {
        self.read_record(OVERBAR_EXTRA_ASCENDER_OFFSET)
    }

    /// Distance between underbar and (ink) bottom of the base.
    #[inline]
    pub fn underbar_vertical_gap(&self) -> MathValue<'a> {
        self.read_record(UNDERBAR_VERTICAL_GAP_OFFSET)
    }

    /// Thickness of underbar.
    #[inline]
    pub fn underbar_rule_thickness(&self) -> MathValue<'a> {
        self.read_record(UNDERBAR_RULE_THICKNESS_OFFSET)
    }

    /// Extra white space reserved below the underbar.
    #[inline]
    pub fn underbar_extra_descender(&self) -> MathValue<'a> {
        self.read_record(UNDERBAR_EXTRA_DESCENDER_OFFSET)
    }

    /// Space between the (ink) top of the expression and the bar over it.
    #[inline]
    pub fn radical_vertical_gap(&self) -> MathValue<'a> {
        self.read_record(RADICAL_VERTICAL_GAP_OFFSET)
    }

    /// Space between the (ink) top of the expression and the bar over it.
    #[inline]
    pub fn radical_display_style_vertical_gap(&self) -> MathValue<'a> {
        self.read_record(RADICAL_DISPLAY_STYLE_VERTICAL_GAP_OFFSET)
    }

    /// Thickness of the radical rule.
    #[inline]
    pub fn radical_rule_thickness(&self) -> MathValue<'a> {
        self.read_record(RADICAL_RULE_THICKNESS_OFFSET)
    }

    /// Extra white space reserved above the radical.
    #[inline]
    pub fn radical_extra_ascender(&self) -> MathValue<'a> {
        self.read_record(RADICAL_EXTRA_ASCENDER_OFFSET)
    }

    /// Extra horizontal kern before the degree of a radical, if such is present.
    #[inline]
    pub fn radical_kern_before_degree(&self) -> MathValue<'a> {
        self.read_record(RADICAL_KERN_BEFORE_DEGREE_OFFSET)
    }

    /// Negative kern after the degree of a radical, if such is present.
    #[inline]
    pub fn radical_kern_after_degree(&self) -> MathValue<'a> {
        self.read_record(RADICAL_KERN_AFTER_DEGREE_OFFSET)
    }

    /// Height of the bottom of the radical degree, if such is present, in proportion to the
    /// ascender of the radical sign.
    #[inline]
    pub fn radical_degree_bottom_raise_percent(&self) -> i16 {
        self.read_i16(RADICAL_DEGREE_BOTTOM_RAISE_PERCENT_OFFSET)
    }

    /// Read an `i16` at an offset into the table.
    #[inline]
    fn read_i16(&self, offset: usize) -> i16 {
        Stream::read_at(self.data, offset).unwrap_or(0)
    }

    /// Read an `u16` at an offset into the table.
    #[inline]
    fn read_u16(&self, offset: usize) -> u16 {
        Stream::read_at(self.data, offset).unwrap_or(0)
    }

    /// Read a `MathValueRecord` at an offset into the table.
    #[inline]
    fn read_record(&self, offset: usize) -> MathValue<'a> {
        self.data
            .get(offset..)
            .and_then(|data| MathValue::parse(data, self.data))
            .unwrap_or(MathValue {
                value: 0,
                device: None,
            })
    }
}

/// A [Math Kern Table](https://learn.microsoft.com/en-us/typography/opentype/spec/math#mathkern-table).
#[derive(Clone)]
pub struct Kern<'a> {
    data: &'a [u8],
    heights: LazyArray16<'a, MathValueRecord>,
    kerns: LazyArray16<'a, MathValueRecord>,
}

impl<'a> Kern<'a> {
    /// Number of heights at which the kern value changes.
    pub fn count(&self) -> u16 {
        self.heights.len()
    }

    /// The correction height at the given index.
    ///
    /// The index must be smaller than `count()`.
    pub fn height(&self, index: u16) -> Option<MathValue<'a>> {
        Some(self.heights.get(index)?.get(self.data))
    }

    /// The kern value at the given index.
    ///
    /// The index must be smaller than or equal to `count()`.
    pub fn kern(&self, index: u16) -> Option<MathValue<'a>> {
        Some(self.kerns.get(index)?.get(self.data))
    }
}

impl<'a> FromSlice<'a> for Kern<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let count = s.read::<u16>()?;
        let heights = s.read_array16::<MathValueRecord>(count)?;
        let kerns = s.read_array16::<MathValueRecord>(count + 1)?;
        Some(Kern {
            data,
            heights,
            kerns,
        })
    }
}

impl core::fmt::Debug for Kern<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Kern {{ ... }}")
    }
}

#[derive(Clone, Copy)]
struct KernInfoRecord {
    top_right: Option<Offset16>,
    top_left: Option<Offset16>,
    bottom_right: Option<Offset16>,
    bottom_left: Option<Offset16>,
}

impl FromData for KernInfoRecord {
    const SIZE: usize = 8;

    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(KernInfoRecord {
            top_right: s.read::<Option<Offset16>>()?,
            top_left: s.read::<Option<Offset16>>()?,
            bottom_right: s.read::<Option<Offset16>>()?,
            bottom_left: s.read::<Option<Offset16>>()?,
        })
    }
}

impl KernInfoRecord {
    fn get<'a>(&self, data: &'a [u8]) -> KernInfo<'a> {
        let parse_field = |offset: Option<Offset16>| {
            offset
                .and_then(|offset| data.get(offset.to_usize()..))
                .and_then(Kern::parse)
        };
        KernInfo {
            top_right: parse_field(self.top_right),
            top_left: parse_field(self.top_left),
            bottom_right: parse_field(self.bottom_right),
            bottom_left: parse_field(self.bottom_left),
        }
    }
}

/// An [entry in a Math Kern Info Table](
/// https://learn.microsoft.com/en-us/typography/opentype/spec/math#mathkerninforecord).
#[derive(Clone, Debug)]
pub struct KernInfo<'a> {
    /// The kerning data for the top-right corner.
    pub top_right: Option<Kern<'a>>,
    /// The kerning data for the top-left corner.
    pub top_left: Option<Kern<'a>>,
    /// The kerning data for the bottom-right corner.
    pub bottom_right: Option<Kern<'a>>,
    /// The kerning data for the bottom-left corner.
    pub bottom_left: Option<Kern<'a>>,
}

/// A [Math Kern Info Table](https://docs.microsoft.com/en-us/typography/opentype/spec/math#mathkerninfo-table).
#[derive(Clone, Copy)]
pub struct KernInfos<'a> {
    data: &'a [u8],
    coverage: Coverage<'a>,
    records: LazyArray16<'a, KernInfoRecord>,
}

impl<'a> FromSlice<'a> for KernInfos<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let coverage = s.parse_at_offset16::<Coverage>(data)?;
        let count = s.read::<u16>()?;
        let records = s.read_array16::<KernInfoRecord>(count)?;
        Some(KernInfos {
            data,
            coverage,
            records,
        })
    }
}

impl<'a> KernInfos<'a> {
    /// Returns the kerning info for the glyph or `None` if it is not covered.
    #[inline]
    pub fn get(&self, glyph: GlyphId) -> Option<KernInfo<'a>> {
        let index = self.coverage.get(glyph)?;
        Some(self.records.get(index)?.get(self.data))
    }
}

impl core::fmt::Debug for KernInfos<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "KernInfos {{ ... }}")
    }
}

/// A [Math Glyph Info Table](https://learn.microsoft.com/en-us/typography/opentype/spec/math#mathglyphinfo-table).
#[derive(Clone, Copy, Debug)]
pub struct GlyphInfo<'a> {
    /// Per-glyph italics correction values.
    pub italic_corrections: Option<MathValues<'a>>,
    /// Per-glyph horizontal positions for attaching mathematical accents.
    pub top_accent_attachments: Option<MathValues<'a>>,
    /// Glyphs which are _extended shapes_.
    pub extended_shapes: Option<Coverage<'a>>,
    /// Per-glyph information for mathematical kerning.
    pub kern_infos: Option<KernInfos<'a>>,
}

impl<'a> FromSlice<'a> for GlyphInfo<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(GlyphInfo {
            italic_corrections: s.parse_at_offset16::<MathValues>(data),
            top_accent_attachments: s.parse_at_offset16::<MathValues>(data),
            extended_shapes: s.parse_at_offset16::<Coverage>(data),
            kern_infos: s.parse_at_offset16::<KernInfos>(data),
        })
    }
}

/// Glyph part flags.
#[derive(Clone, Copy, Debug)]
pub struct PartFlags(pub u16);

#[allow(missing_docs)]
impl PartFlags {
    #[inline]
    pub fn extender(self) -> bool {
        self.0 & 0x0001 != 0
    }
}

impl FromData for PartFlags {
    const SIZE: usize = 2;

    fn parse(data: &[u8]) -> Option<Self> {
        u16::parse(data).map(PartFlags)
    }
}

/// Details for a glyph part in an assembly.
#[derive(Clone, Copy, Debug)]
pub struct GlyphPart {
    /// Glyph ID for the part.
    pub glyph_id: GlyphId,
    /// Lengths of the connectors on the start of the glyph, in font design units.
    pub start_connector_length: u16,
    /// Lengths of the connectors on the end of the glyph, in font design units.
    pub end_connector_length: u16,
    /// The full advance of the part, in font design units.
    pub full_advance: u16,
    /// Part flags.
    pub part_flags: PartFlags,
}

impl FromData for GlyphPart {
    const SIZE: usize = 10;

    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(GlyphPart {
            glyph_id: s.read::<GlyphId>()?,
            start_connector_length: s.read::<u16>()?,
            end_connector_length: s.read::<u16>()?,
            full_advance: s.read::<u16>()?,
            part_flags: s.read::<PartFlags>()?,
        })
    }
}

/// A [Glyph Assembly Table](https://learn.microsoft.com/en-us/typography/opentype/spec/math#glyphassembly-table).
#[derive(Clone, Copy, Debug)]
pub struct GlyphAssembly<'a> {
    /// The italics correction of the assembled glyph.
    pub italics_correction: MathValue<'a>,
    /// Parts the assembly is composed of.
    pub parts: LazyArray16<'a, GlyphPart>,
}

impl<'a> FromSlice<'a> for GlyphAssembly<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let italics_correction = s.read::<MathValueRecord>()?.get(data);
        let count = s.read::<u16>()?;
        let parts = s.read_array16::<GlyphPart>(count)?;
        Some(GlyphAssembly {
            italics_correction,
            parts,
        })
    }
}

/// Description of math glyph variants.
#[derive(Clone, Copy, Debug)]
pub struct GlyphVariant {
    /// The ID of the variant glyph.
    pub variant_glyph: GlyphId,
    /// Advance width/height, in design units, of the variant glyph.
    pub advance_measurement: u16,
}

impl FromData for GlyphVariant {
    const SIZE: usize = 4;

    fn parse(data: &[u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        Some(GlyphVariant {
            variant_glyph: s.read::<GlyphId>()?,
            advance_measurement: s.read::<u16>()?,
        })
    }
}

/// A [Math Glyph Construction Table](
/// https://learn.microsoft.com/en-us/typography/opentype/spec/math#mathglyphconstruction-table).
#[derive(Clone, Copy, Debug)]
pub struct GlyphConstruction<'a> {
    /// A general recipe on how to construct a variant with large advance width/height.
    pub assembly: Option<GlyphAssembly<'a>>,
    /// Prepared variants of the glyph with varying advances.
    pub variants: LazyArray16<'a, GlyphVariant>,
}

impl<'a> FromSlice<'a> for GlyphConstruction<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let assembly = s.parse_at_offset16::<GlyphAssembly>(data);
        let variant_count = s.read::<u16>()?;
        let variants = s.read_array16::<GlyphVariant>(variant_count)?;
        Some(GlyphConstruction { assembly, variants })
    }
}

/// A mapping from glyphs to
/// [Math Glyph Construction Tables](
/// https://learn.microsoft.com/en-us/typography/opentype/spec/math#mathglyphconstruction-table).
#[derive(Clone, Copy)]
pub struct GlyphConstructions<'a> {
    coverage: Coverage<'a>,
    constructions: LazyOffsetArray16<'a, GlyphConstruction<'a>>,
}

impl<'a> GlyphConstructions<'a> {
    fn new(
        data: &'a [u8],
        coverage: Option<Coverage<'a>>,
        offsets: LazyArray16<'a, Option<Offset16>>,
    ) -> Self {
        GlyphConstructions {
            coverage: coverage.unwrap_or(Coverage::Format1 {
                glyphs: LazyArray16::new(&[]),
            }),
            constructions: LazyOffsetArray16::new(data, offsets),
        }
    }

    /// Returns the construction for the glyph or `None` if it is not covered.
    #[inline]
    pub fn get(&self, glyph: GlyphId) -> Option<GlyphConstruction<'a>> {
        let index = self.coverage.get(glyph)?;
        self.constructions.get(index)
    }
}

impl core::fmt::Debug for GlyphConstructions<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "GlyphConstructions {{ ... }}")
    }
}

/// A [Math Variants Table](
/// https://learn.microsoft.com/en-us/typography/opentype/spec/math#mathvariants-table).
#[derive(Clone, Copy, Debug)]
pub struct Variants<'a> {
    /// Minimum overlap of connecting glyphs during glyph construction, in design units.
    pub min_connector_overlap: u16,
    /// Constructions for shapes growing in the vertical direction.
    pub vertical_constructions: GlyphConstructions<'a>,
    /// Constructions for shapes growing in the horizontal direction.
    pub horizontal_constructions: GlyphConstructions<'a>,
}

impl<'a> FromSlice<'a> for Variants<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let min_connector_overlap = s.read::<u16>()?;
        let vertical_coverage = s.parse_at_offset16::<Coverage>(data);
        let horizontal_coverage = s.parse_at_offset16::<Coverage>(data);
        let vertical_count = s.read::<u16>()?;
        let horizontal_count = s.read::<u16>()?;
        let vertical_offsets = s.read_array16::<Option<Offset16>>(vertical_count)?;
        let horizontal_offsets = s.read_array16::<Option<Offset16>>(horizontal_count)?;
        Some(Variants {
            min_connector_overlap,
            vertical_constructions: GlyphConstructions::new(
                data,
                vertical_coverage,
                vertical_offsets,
            ),
            horizontal_constructions: GlyphConstructions::new(
                data,
                horizontal_coverage,
                horizontal_offsets,
            ),
        })
    }
}

/// A [Math Table](https://docs.microsoft.com/en-us/typography/opentype/spec/math).
#[derive(Clone, Copy, Debug)]
pub struct Table<'a> {
    /// Math positioning constants.
    pub constants: Option<Constants<'a>>,
    /// Per-glyph positioning information.
    pub glyph_info: Option<GlyphInfo<'a>>,
    /// Variants and assembly recipes for growable glyphs.
    pub variants: Option<Variants<'a>>,
}

impl<'a> Table<'a> {
    /// Parses a table from raw data.
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let mut s = Stream::new(data);
        let major_version = s.read::<u16>()? as u8;
        s.skip::<u16>(); // minor version
        if major_version != 1 {
            return None;
        }

        Some(Table {
            constants: s.parse_at_offset16::<Constants>(data),
            glyph_info: s.parse_at_offset16::<GlyphInfo>(data),
            variants: s.parse_at_offset16::<Variants>(data),
        })
    }
}

trait StreamExt<'a> {
    fn parse_at_offset16<T: FromSlice<'a>>(&mut self, data: &'a [u8]) -> Option<T>;
}

impl<'a> StreamExt<'a> for Stream<'a> {
    fn parse_at_offset16<T: FromSlice<'a>>(&mut self, data: &'a [u8]) -> Option<T> {
        let offset = self.read::<Option<Offset16>>()??.to_usize();
        data.get(offset..).and_then(T::parse)
    }
}
