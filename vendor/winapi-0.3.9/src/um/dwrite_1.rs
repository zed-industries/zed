// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the content of dwrite_1.h
use shared::basetsd::{INT16, INT32, UINT16, UINT32, UINT8};
use shared::minwindef::{BOOL, FLOAT};
use um::dcommon::DWRITE_MEASURING_MODE;
use um::dwrite::{
    DWRITE_GLYPH_OFFSET, DWRITE_MATRIX, DWRITE_PIXEL_GEOMETRY, DWRITE_RENDERING_MODE,
    DWRITE_SCRIPT_ANALYSIS, DWRITE_SHAPING_GLYPH_PROPERTIES, DWRITE_TEXT_RANGE,
    IDWriteBitmapRenderTarget, IDWriteBitmapRenderTargetVtbl, IDWriteFactory, IDWriteFactoryVtbl,
    IDWriteFont, IDWriteFontCollection, IDWriteFontFace, IDWriteFontFaceVtbl, IDWriteFontVtbl,
    IDWriteRenderingParams, IDWriteRenderingParamsVtbl, IDWriteTextAnalysisSink,
    IDWriteTextAnalysisSinkVtbl, IDWriteTextAnalysisSource, IDWriteTextAnalysisSourceVtbl,
    IDWriteTextAnalyzer, IDWriteTextAnalyzerVtbl, IDWriteTextLayout, IDWriteTextLayoutVtbl,
};
use um::winnt::{HRESULT, WCHAR};
ENUM!{enum DWRITE_PANOSE_FAMILY {
    DWRITE_PANOSE_FAMILY_ANY = 0x0, // 0
    DWRITE_PANOSE_FAMILY_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_FAMILY_TEXT_DISPLAY = 0x2, // 2
    DWRITE_PANOSE_FAMILY_SCRIPT = 0x3, // 3
    DWRITE_PANOSE_FAMILY_DECORATIVE = 0x4, // 4
    DWRITE_PANOSE_FAMILY_SYMBOL = 0x5, // 5
    DWRITE_PANOSE_FAMILY_PICTORIAL = 0x5, // 5
}}
ENUM!{enum DWRITE_PANOSE_SERIF_STYLE {
    DWRITE_PANOSE_SERIF_STYLE_ANY = 0x0, // 0
    DWRITE_PANOSE_SERIF_STYLE_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_SERIF_STYLE_COVE = 0x2, // 2
    DWRITE_PANOSE_SERIF_STYLE_OBTUSE_COVE = 0x3, // 3
    DWRITE_PANOSE_SERIF_STYLE_SQUARE_COVE = 0x4, // 4
    DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SQUARE_COVE = 0x5, // 5
    DWRITE_PANOSE_SERIF_STYLE_SQUARE = 0x6, // 6
    DWRITE_PANOSE_SERIF_STYLE_THIN = 0x7, // 7
    DWRITE_PANOSE_SERIF_STYLE_OVAL = 0x8, // 8
    DWRITE_PANOSE_SERIF_STYLE_EXAGGERATED = 0x9, // 9
    DWRITE_PANOSE_SERIF_STYLE_TRIANGLE = 0xA, // 10
    DWRITE_PANOSE_SERIF_STYLE_NORMAL_SANS = 0xB, // 11
    DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SANS = 0xC, // 12
    DWRITE_PANOSE_SERIF_STYLE_PERPENDICULAR_SANS = 0xD, // 13
    DWRITE_PANOSE_SERIF_STYLE_FLARED = 0xE, // 14
    DWRITE_PANOSE_SERIF_STYLE_ROUNDED = 0xF, // 15
    DWRITE_PANOSE_SERIF_STYLE_SCRIPT = 0x10, // 16
    DWRITE_PANOSE_SERIF_STYLE_PERP_SANS = 0xD, // 13
    DWRITE_PANOSE_SERIF_STYLE_BONE = 0x8, // 8
}}
ENUM!{enum DWRITE_PANOSE_WEIGHT {
    DWRITE_PANOSE_WEIGHT_ANY = 0x0, // 0
    DWRITE_PANOSE_WEIGHT_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_WEIGHT_VERY_LIGHT = 0x2, // 2
    DWRITE_PANOSE_WEIGHT_LIGHT = 0x3, // 3
    DWRITE_PANOSE_WEIGHT_THIN = 0x4, // 4
    DWRITE_PANOSE_WEIGHT_BOOK = 0x5, // 5
    DWRITE_PANOSE_WEIGHT_MEDIUM = 0x6, // 6
    DWRITE_PANOSE_WEIGHT_DEMI = 0x7, // 7
    DWRITE_PANOSE_WEIGHT_BOLD = 0x8, // 8
    DWRITE_PANOSE_WEIGHT_HEAVY = 0x9, // 9
    DWRITE_PANOSE_WEIGHT_BLACK = 0xA, // 10
    DWRITE_PANOSE_WEIGHT_EXTRA_BLACK = 0xB, // 11
    DWRITE_PANOSE_WEIGHT_NORD = 0xB, // 11
}}
ENUM!{enum DWRITE_PANOSE_PROPORTION {
    DWRITE_PANOSE_PROPORTION_ANY = 0x0, // 0
    DWRITE_PANOSE_PROPORTION_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_PROPORTION_OLD_STYLE = 0x2, // 2
    DWRITE_PANOSE_PROPORTION_MODERN = 0x3, // 3
    DWRITE_PANOSE_PROPORTION_EVEN_WIDTH = 0x4, // 4
    DWRITE_PANOSE_PROPORTION_EXPANDED = 0x5, // 5
    DWRITE_PANOSE_PROPORTION_CONDENSED = 0x6, // 6
    DWRITE_PANOSE_PROPORTION_VERY_EXPANDED = 0x7, // 7
    DWRITE_PANOSE_PROPORTION_VERY_CONDENSED = 0x8, // 8
    DWRITE_PANOSE_PROPORTION_MONOSPACED = 0x9, // 9
}}
ENUM!{enum DWRITE_PANOSE_CONTRAST {
    DWRITE_PANOSE_CONTRAST_ANY = 0x0, // 0
    DWRITE_PANOSE_CONTRAST_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_CONTRAST_NONE = 0x2, // 2
    DWRITE_PANOSE_CONTRAST_VERY_LOW = 0x3, // 3
    DWRITE_PANOSE_CONTRAST_LOW = 0x4, // 4
    DWRITE_PANOSE_CONTRAST_MEDIUM_LOW = 0x5, // 5
    DWRITE_PANOSE_CONTRAST_MEDIUM = 0x6, // 6
    DWRITE_PANOSE_CONTRAST_MEDIUM_HIGH = 0x7, // 7
    DWRITE_PANOSE_CONTRAST_HIGH = 0x8, // 8
    DWRITE_PANOSE_CONTRAST_VERY_HIGH = 0x9, // 9
    DWRITE_PANOSE_CONTRAST_HORIZONTAL_LOW = 0xA, // 10
    DWRITE_PANOSE_CONTRAST_HORIZONTAL_MEDIUM = 0xB, // 11
    DWRITE_PANOSE_CONTRAST_HORIZONTAL_HIGH = 0xC, // 12
    DWRITE_PANOSE_CONTRAST_BROKEN = 0xD, // 13
}}
ENUM!{enum DWRITE_PANOSE_STROKE_VARIATION {
    DWRITE_PANOSE_STROKE_VARIATION_ANY = 0x0, // 0
    DWRITE_PANOSE_STROKE_VARIATION_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_STROKE_VARIATION_NO_VARIATION = 0x2, // 2
    DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_DIAGONAL = 0x3, // 3
    DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_TRANSITIONAL = 0x4, // 4
    DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_VERTICAL = 0x5, // 5
    DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_HORIZONTAL = 0x6, // 6
    DWRITE_PANOSE_STROKE_VARIATION_RAPID_VERTICAL = 0x7, // 7
    DWRITE_PANOSE_STROKE_VARIATION_RAPID_HORIZONTAL = 0x8, // 8
    DWRITE_PANOSE_STROKE_VARIATION_INSTANT_VERTICAL = 0x9, // 9
    DWRITE_PANOSE_STROKE_VARIATION_INSTANT_HORIZONTAL = 0xA, // 10
}}
ENUM!{enum DWRITE_PANOSE_ARM_STYLE {
    DWRITE_PANOSE_ARM_STYLE_ANY = 0x0, // 0
    DWRITE_PANOSE_ARM_STYLE_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORIZONTAL = 0x2, // 2
    DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_WEDGE = 0x3, // 3
    DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERTICAL = 0x4, // 4
    DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_SINGLE_SERIF = 0x5, // 5
    DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_DOUBLE_SERIF = 0x6, // 6
    DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_HORIZONTAL = 0x7, // 7
    DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_WEDGE = 0x8, // 8
    DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_VERTICAL = 0x9, // 9
    DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_SINGLE_SERIF = 0xA, // 10
    DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_DOUBLE_SERIF = 0xB, // 11
    DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORZ = 0x2, // 2
    DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERT = 0x4, // 4
    DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_HORZ = 0x7, // 7
    DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_WEDGE = 0x8, // 8
    DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_VERT = 0x9, // 9
    DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_SINGLE_SERIF = 0xA, // 10
    DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_DOUBLE_SERIF = 0xB, // 11
}}
ENUM!{enum DWRITE_PANOSE_LETTERFORM {
    DWRITE_PANOSE_LETTERFORM_ANY = 0x0, // 0
    DWRITE_PANOSE_LETTERFORM_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_LETTERFORM_NORMAL_CONTACT = 0x2, // 2
    DWRITE_PANOSE_LETTERFORM_NORMAL_WEIGHTED = 0x3, // 3
    DWRITE_PANOSE_LETTERFORM_NORMAL_BOXED = 0x4, // 4
    DWRITE_PANOSE_LETTERFORM_NORMAL_FLATTENED = 0x5, // 5
    DWRITE_PANOSE_LETTERFORM_NORMAL_ROUNDED = 0x6, // 6
    DWRITE_PANOSE_LETTERFORM_NORMAL_OFF_CENTER = 0x7, // 7
    DWRITE_PANOSE_LETTERFORM_NORMAL_SQUARE = 0x8, // 8
    DWRITE_PANOSE_LETTERFORM_OBLIQUE_CONTACT = 0x9, // 9
    DWRITE_PANOSE_LETTERFORM_OBLIQUE_WEIGHTED = 0xA, // 10
    DWRITE_PANOSE_LETTERFORM_OBLIQUE_BOXED = 0xB, // 11
    DWRITE_PANOSE_LETTERFORM_OBLIQUE_FLATTENED = 0xC, // 12
    DWRITE_PANOSE_LETTERFORM_OBLIQUE_ROUNDED = 0xD, // 13
    DWRITE_PANOSE_LETTERFORM_OBLIQUE_OFF_CENTER = 0xE, // 14
    DWRITE_PANOSE_LETTERFORM_OBLIQUE_SQUARE = 0xF, // 15
}}
ENUM!{enum DWRITE_PANOSE_MIDLINE {
    DWRITE_PANOSE_MIDLINE_ANY = 0x0, // 0
    DWRITE_PANOSE_MIDLINE_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_MIDLINE_STANDARD_TRIMMED = 0x2, // 2
    DWRITE_PANOSE_MIDLINE_STANDARD_POINTED = 0x3, // 3
    DWRITE_PANOSE_MIDLINE_STANDARD_SERIFED = 0x4, // 4
    DWRITE_PANOSE_MIDLINE_HIGH_TRIMMED = 0x5, // 5
    DWRITE_PANOSE_MIDLINE_HIGH_POINTED = 0x6, // 6
    DWRITE_PANOSE_MIDLINE_HIGH_SERIFED = 0x7, // 7
    DWRITE_PANOSE_MIDLINE_CONSTANT_TRIMMED = 0x8, // 8
    DWRITE_PANOSE_MIDLINE_CONSTANT_POINTED = 0x9, // 9
    DWRITE_PANOSE_MIDLINE_CONSTANT_SERIFED = 0xA, // 10
    DWRITE_PANOSE_MIDLINE_LOW_TRIMMED = 0xB, // 11
    DWRITE_PANOSE_MIDLINE_LOW_POINTED = 0xC, // 12
    DWRITE_PANOSE_MIDLINE_LOW_SERIFED = 0xD, // 13
}}
ENUM!{enum DWRITE_PANOSE_XHEIGHT {
    DWRITE_PANOSE_XHEIGHT_ANY = 0x0, // 0
    DWRITE_PANOSE_XHEIGHT_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_XHEIGHT_CONSTANT_SMALL = 0x2, // 2
    DWRITE_PANOSE_XHEIGHT_CONSTANT_STANDARD = 0x3, // 3
    DWRITE_PANOSE_XHEIGHT_CONSTANT_LARGE = 0x4, // 4
    DWRITE_PANOSE_XHEIGHT_DUCKING_SMALL = 0x5, // 5
    DWRITE_PANOSE_XHEIGHT_DUCKING_STANDARD = 0x6, // 6
    DWRITE_PANOSE_XHEIGHT_DUCKING_LARGE = 0x7, // 7
    DWRITE_PANOSE_XHEIGHT_CONSTANT_STD = 0x3, // 3
    DWRITE_PANOSE_XHEIGHT_DUCKING_STD = 0x6, // 6
}}
ENUM!{enum DWRITE_PANOSE_TOOL_KIND {
    DWRITE_PANOSE_TOOL_KIND_ANY = 0x0, // 0
    DWRITE_PANOSE_TOOL_KIND_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_TOOL_KIND_FLAT_NIB = 0x2, // 2
    DWRITE_PANOSE_TOOL_KIND_PRESSURE_POINT = 0x3, // 3
    DWRITE_PANOSE_TOOL_KIND_ENGRAVED = 0x4, // 4
    DWRITE_PANOSE_TOOL_KIND_BALL = 0x5, // 5
    DWRITE_PANOSE_TOOL_KIND_BRUSH = 0x6, // 6
    DWRITE_PANOSE_TOOL_KIND_ROUGH = 0x7, // 7
    DWRITE_PANOSE_TOOL_KIND_FELT_PEN_BRUSH_TIP = 0x8, // 8
    DWRITE_PANOSE_TOOL_KIND_WILD_BRUSH = 0x9, // 9
}}
ENUM!{enum DWRITE_PANOSE_SPACING {
    DWRITE_PANOSE_SPACING_ANY = 0x0, // 0
    DWRITE_PANOSE_SPACING_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_SPACING_PROPORTIONAL_SPACED = 0x2, // 2
    DWRITE_PANOSE_SPACING_MONOSPACED = 0x3, // 3
}}
ENUM!{enum DWRITE_PANOSE_ASPECT_RATIO {
    DWRITE_PANOSE_ASPECT_RATIO_ANY = 0x0, // 0
    DWRITE_PANOSE_ASPECT_RATIO_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_ASPECT_RATIO_VERY_CONDENSED = 0x2, // 2
    DWRITE_PANOSE_ASPECT_RATIO_CONDENSED = 0x3, // 3
    DWRITE_PANOSE_ASPECT_RATIO_NORMAL = 0x4, // 4
    DWRITE_PANOSE_ASPECT_RATIO_EXPANDED = 0x5, // 5
    DWRITE_PANOSE_ASPECT_RATIO_VERY_EXPANDED = 0x6, // 6
}}
ENUM!{enum DWRITE_PANOSE_SCRIPT_TOPOLOGY {
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_ANY = 0x0, // 0
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_DISCONNECTED = 0x2, // 2
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_TRAILING = 0x3, // 3
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_CONNECTED = 0x4, // 4
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_DISCONNECTED = 0x5, // 5
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_TRAILING = 0x6, // 6
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_CONNECTED = 0x7, // 7
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_DISCONNECTED = 0x8, // 8
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_TRAILING = 0x9, // 9
    DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_CONNECTED = 0xA, // 10
}}
ENUM!{enum DWRITE_PANOSE_SCRIPT_FORM {
    DWRITE_PANOSE_SCRIPT_FORM_ANY = 0x0, // 0
    DWRITE_PANOSE_SCRIPT_FORM_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_NO_WRAPPING = 0x2, // 2
    DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_SOME_WRAPPING = 0x3, // 3
    DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_MORE_WRAPPING = 0x4, // 4
    DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_EXTREME_WRAPPING = 0x5, // 5
    DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_NO_WRAPPING = 0x6, // 6
    DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_SOME_WRAPPING = 0x7, // 7
    DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_MORE_WRAPPING = 0x8, // 8
    DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_EXTREME_WRAPPING = 0x9, // 9
    DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_NO_WRAPPING = 0xA, // 10
    DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_SOME_WRAPPING = 0xB, // 11
    DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_MORE_WRAPPING = 0xC, // 12
    DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_EXTREME_WRAPPING = 0xD, // 13
}}
ENUM!{enum DWRITE_PANOSE_FINIALS {
    DWRITE_PANOSE_FINIALS_ANY = 0x0, // 0
    DWRITE_PANOSE_FINIALS_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_FINIALS_NONE_NO_LOOPS = 0x2, // 2
    DWRITE_PANOSE_FINIALS_NONE_CLOSED_LOOPS = 0x3, // 3
    DWRITE_PANOSE_FINIALS_NONE_OPEN_LOOPS = 0x4, // 4
    DWRITE_PANOSE_FINIALS_SHARP_NO_LOOPS = 0x5, // 5
    DWRITE_PANOSE_FINIALS_SHARP_CLOSED_LOOPS = 0x6, // 6
    DWRITE_PANOSE_FINIALS_SHARP_OPEN_LOOPS = 0x7, // 7
    DWRITE_PANOSE_FINIALS_TAPERED_NO_LOOPS = 0x8, // 8
    DWRITE_PANOSE_FINIALS_TAPERED_CLOSED_LOOPS = 0x9, // 9
    DWRITE_PANOSE_FINIALS_TAPERED_OPEN_LOOPS = 0xA, // 10
    DWRITE_PANOSE_FINIALS_ROUND_NO_LOOPS = 0xB, // 11
    DWRITE_PANOSE_FINIALS_ROUND_CLOSED_LOOPS = 0xC, // 12
    DWRITE_PANOSE_FINIALS_ROUND_OPEN_LOOPS = 0xD, // 13
}}
ENUM!{enum DWRITE_PANOSE_XASCENT {
    DWRITE_PANOSE_XASCENT_ANY = 0x0, // 0
    DWRITE_PANOSE_XASCENT_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_XASCENT_VERY_LOW = 0x2, // 2
    DWRITE_PANOSE_XASCENT_LOW = 0x3, // 3
    DWRITE_PANOSE_XASCENT_MEDIUM = 0x4, // 4
    DWRITE_PANOSE_XASCENT_HIGH = 0x5, // 5
    DWRITE_PANOSE_XASCENT_VERY_HIGH = 0x6, // 6
}}
ENUM!{enum DWRITE_PANOSE_DECORATIVE_CLASS {
    DWRITE_PANOSE_DECORATIVE_CLASS_ANY = 0x0, // 0
    DWRITE_PANOSE_DECORATIVE_CLASS_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_DECORATIVE_CLASS_DERIVATIVE = 0x2, // 2
    DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_TOPOLOGY = 0x3, // 3
    DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ELEMENTS = 0x4, // 4
    DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ASPECT = 0x5, // 5
    DWRITE_PANOSE_DECORATIVE_CLASS_INITIALS = 0x6, // 6
    DWRITE_PANOSE_DECORATIVE_CLASS_CARTOON = 0x7, // 7
    DWRITE_PANOSE_DECORATIVE_CLASS_PICTURE_STEMS = 0x8, // 8
    DWRITE_PANOSE_DECORATIVE_CLASS_ORNAMENTED = 0x9, // 9
    DWRITE_PANOSE_DECORATIVE_CLASS_TEXT_AND_BACKGROUND = 0xA, // 10
    DWRITE_PANOSE_DECORATIVE_CLASS_COLLAGE = 0xB, // 11
    DWRITE_PANOSE_DECORATIVE_CLASS_MONTAGE = 0xC, // 12
}}
ENUM!{enum DWRITE_PANOSE_ASPECT {
    DWRITE_PANOSE_ASPECT_ANY = 0x0, // 0
    DWRITE_PANOSE_ASPECT_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_ASPECT_SUPER_CONDENSED = 0x2, // 2
    DWRITE_PANOSE_ASPECT_VERY_CONDENSED = 0x3, // 3
    DWRITE_PANOSE_ASPECT_CONDENSED = 0x4, // 4
    DWRITE_PANOSE_ASPECT_NORMAL = 0x5, // 5
    DWRITE_PANOSE_ASPECT_EXTENDED = 0x6, // 6
    DWRITE_PANOSE_ASPECT_VERY_EXTENDED = 0x7, // 7
    DWRITE_PANOSE_ASPECT_SUPER_EXTENDED = 0x8, // 8
    DWRITE_PANOSE_ASPECT_MONOSPACED = 0x9, // 9
}}
ENUM!{enum DWRITE_PANOSE_FILL {
    DWRITE_PANOSE_FILL_ANY = 0x0, // 0
    DWRITE_PANOSE_FILL_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_FILL_STANDARD_SOLID_FILL = 0x2, // 2
    DWRITE_PANOSE_FILL_NO_FILL = 0x3, // 3
    DWRITE_PANOSE_FILL_PATTERNED_FILL = 0x4, // 4
    DWRITE_PANOSE_FILL_COMPLEX_FILL = 0x5, // 5
    DWRITE_PANOSE_FILL_SHAPED_FILL = 0x6, // 6
    DWRITE_PANOSE_FILL_DRAWN_DISTRESSED = 0x7, // 7
}}
ENUM!{enum DWRITE_PANOSE_LINING {
    DWRITE_PANOSE_LINING_ANY = 0x0, // 0
    DWRITE_PANOSE_LINING_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_LINING_NONE = 0x2, // 2
    DWRITE_PANOSE_LINING_INLINE = 0x3, // 3
    DWRITE_PANOSE_LINING_OUTLINE = 0x4, // 4
    DWRITE_PANOSE_LINING_ENGRAVED = 0x5, // 5
    DWRITE_PANOSE_LINING_SHADOW = 0x6, // 6
    DWRITE_PANOSE_LINING_RELIEF = 0x7, // 7
    DWRITE_PANOSE_LINING_BACKDROP = 0x8, // 8
}}
ENUM!{enum DWRITE_PANOSE_DECORATIVE_TOPOLOGY {
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ANY = 0x0, // 0
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_STANDARD = 0x2, // 2
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SQUARE = 0x3, // 3
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_MULTIPLE_SEGMENT = 0x4, // 4
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ART_DECO = 0x5, // 5
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UNEVEN_WEIGHTING = 0x6, // 6
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_ARMS = 0x7, // 7
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_FORMS = 0x8, // 8
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_LOMBARDIC_FORMS = 0x9, // 9
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UPPER_CASE_IN_LOWER_CASE = 0xA, // 10
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_IMPLIED_TOPOLOGY = 0xB, // 11
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_HORSESHOE_E_AND_A = 0xC, // 12
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_CURSIVE = 0xD, // 13
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_BLACKLETTER = 0xE, // 14
    DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SWASH_VARIANCE = 0xF, // 15
}}
ENUM!{enum DWRITE_PANOSE_CHARACTER_RANGES {
    DWRITE_PANOSE_CHARACTER_RANGES_ANY = 0x0, // 0
    DWRITE_PANOSE_CHARACTER_RANGES_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_CHARACTER_RANGES_EXTENDED_COLLECTION = 0x2, // 2
    DWRITE_PANOSE_CHARACTER_RANGES_LITERALS = 0x3, // 3
    DWRITE_PANOSE_CHARACTER_RANGES_NO_LOWER_CASE = 0x4, // 4
    DWRITE_PANOSE_CHARACTER_RANGES_SMALL_CAPS = 0x5, // 5
}}
ENUM!{enum DWRITE_PANOSE_SYMBOL_KIND {
    DWRITE_PANOSE_SYMBOL_KIND_ANY = 0x0, // 0
    DWRITE_PANOSE_SYMBOL_KIND_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_SYMBOL_KIND_MONTAGES = 0x2, // 2
    DWRITE_PANOSE_SYMBOL_KIND_PICTURES = 0x3, // 3
    DWRITE_PANOSE_SYMBOL_KIND_SHAPES = 0x4, // 4
    DWRITE_PANOSE_SYMBOL_KIND_SCIENTIFIC = 0x5, // 5
    DWRITE_PANOSE_SYMBOL_KIND_MUSIC = 0x6, // 6
    DWRITE_PANOSE_SYMBOL_KIND_EXPERT = 0x7, // 7
    DWRITE_PANOSE_SYMBOL_KIND_PATTERNS = 0x8, // 8
    DWRITE_PANOSE_SYMBOL_KIND_BOARDERS = 0x9, // 9
    DWRITE_PANOSE_SYMBOL_KIND_ICONS = 0xA, // 10
    DWRITE_PANOSE_SYMBOL_KIND_LOGOS = 0xB, // 11
    DWRITE_PANOSE_SYMBOL_KIND_INDUSTRY_SPECIFIC = 0xC, // 12
}}
ENUM!{enum DWRITE_PANOSE_SYMBOL_ASPECT_RATIO {
    DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_ANY = 0x0, // 0
    DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_FIT = 0x1, // 1
    DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_WIDTH = 0x2, // 2
    DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_EXCEPTIONALLY_WIDE = 0x3, // 3
    DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_SUPER_WIDE = 0x4, // 4
    DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_WIDE = 0x5, // 5
    DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_WIDE = 0x6, // 6
    DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NORMAL = 0x7, // 7
    DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NARROW = 0x8, // 8
    DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_NARROW = 0x9, // 9
}}
ENUM!{enum DWRITE_OUTLINE_THRESHOLD {
    DWRITE_OUTLINE_THRESHOLD_ANTIALIASED = 0x0, // 0
    DWRITE_OUTLINE_THRESHOLD_ALIASED = 0x1, // 1
}}
ENUM!{enum DWRITE_BASELINE {
    DWRITE_BASELINE_DEFAULT = 0x0, // 0
    DWRITE_BASELINE_ROMAN = 0x1, // 1
    DWRITE_BASELINE_CENTRAL = 0x2, // 2
    DWRITE_BASELINE_MATH = 0x3, // 3
    DWRITE_BASELINE_HANGING = 0x4, // 4
    DWRITE_BASELINE_IDEOGRAPHIC_BOTTOM = 0x5, // 5
    DWRITE_BASELINE_IDEOGRAPHIC_TOP = 0x6, // 6
    DWRITE_BASELINE_MINIMUM = 0x7, // 7
    DWRITE_BASELINE_MAXIMUM = 0x8, // 8
}}
ENUM!{enum DWRITE_VERTICAL_GLYPH_ORIENTATION {
    DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT = 0x0, // 0
    DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED = 0x1, // 1
}}
ENUM!{enum DWRITE_GLYPH_ORIENTATION_ANGLE {
    DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES = 0x0, // 0
    DWRITE_GLYPH_ORIENTATION_ANGLE_90_DEGREES = 0x1, // 1
    DWRITE_GLYPH_ORIENTATION_ANGLE_180_DEGREES = 0x2, // 2
    DWRITE_GLYPH_ORIENTATION_ANGLE_270_DEGREES = 0x3, // 3
}}
STRUCT!{struct DWRITE_FONT_METRICS1 {
    designUnitsPerEm: UINT16,
    ascent: UINT16,
    descent: UINT16,
    lineGap: INT16,
    capHeight: UINT16,
    xHeight: UINT16,
    underlinePosition: INT16,
    underlineThickness: UINT16,
    strikethroughPosition: INT16,
    strikethroughThickness: UINT16,
    glyphBoxLeft: INT16,
    glyphBoxTop: INT16,
    glyphBoxRight: INT16,
    glyphBoxBottom: INT16,
    subscriptPositionX: INT16,
    subscriptPositionY: INT16,
    subscriptSizeX: INT16,
    subscriptSizeY: INT16,
    superscriptPositionX: INT16,
    superscriptPositionY: INT16,
    superscriptSizeX: INT16,
    superscriptSizeY: INT16,
    hasTypographicMetrics: BOOL,
}}
STRUCT!{struct DWRITE_CARET_METRICS {
    slopeRise: INT16,
    slopeRun: INT16,
    offset: INT16,
}}
STRUCT!{struct DWRITE_PANOSE_text {
    familyKind: UINT8,
    serifStyle: UINT8,
    weight: UINT8,
    proportion: UINT8,
    contrast: UINT8,
    strokeVariation: UINT8,
    armStyle: UINT8,
    letterform: UINT8,
    midline: UINT8,
    xHeight: UINT8,
}}
STRUCT!{struct DWRITE_PANOSE_script {
    familyKind: UINT8,
    toolKind: UINT8,
    weight: UINT8,
    spacing: UINT8,
    aspectRatio: UINT8,
    contrast: UINT8,
    scriptTopology: UINT8,
    scriptForm: UINT8,
    finials: UINT8,
    xAscent: UINT8,
}}
STRUCT!{struct DWRITE_PANOSE_decorative {
    familyKind: UINT8,
    decorativeClass: UINT8,
    weight: UINT8,
    aspect: UINT8,
    contrast: UINT8,
    serifVariant: UINT8,
    fill: UINT8,
    lining: UINT8,
    decorativeTopology: UINT8,
    characterRange: UINT8,
}}
STRUCT!{struct DWRITE_PANOSE_symbol {
    familyKind: UINT8,
    symbolKind: UINT8,
    weight: UINT8,
    spacing: UINT8,
    aspectRatioAndContrast: UINT8,
    aspectRatio94: UINT8,
    aspectRatio119: UINT8,
    aspectRatio157: UINT8,
    aspectRatio163: UINT8,
    aspectRatio211: UINT8,
}}
UNION!{union DWRITE_PANOSE {
    [u8; 10],
    values values_mut: [UINT8; 10],
    familyKind familyKind_mut: UINT8,
    text text_mut: DWRITE_PANOSE_text,
    script script_mut: DWRITE_PANOSE_script,
    decorative decorative_mut: DWRITE_PANOSE_decorative,
    symbol symbol_mut: DWRITE_PANOSE_symbol,
}}
STRUCT!{struct DWRITE_UNICODE_RANGE {
    first: UINT32,
    last: UINT32,
}}
STRUCT!{struct DWRITE_SCRIPT_PROPERTIES {
    isoScriptCode: UINT32,
    isoScriptNumber: UINT32,
    clusterLookahead: UINT32,
    justificationCharacter: UINT32,
    bitfield0: UINT32,
}}
BITFIELD!{DWRITE_SCRIPT_PROPERTIES bitfield0: UINT32 [
    restrictCaretToClusters set_restrictCaretToClusters[0..1],
    usesWordDividers set_usesWordDividers[1..2],
    isDiscreteWriting set_isDiscreteWriting[2..3],
    isBlockWriting set_isBlockWriting[3..4],
    isDistributedWithinCluster set_isDistributedWithinCluster[4..5],
    isConnectedWriting set_isConnectedWriting[5..6],
    isCursiveWriting set_isCursiveWriting[6..7],
    reserved set_reserved[7..32],
]}
STRUCT!{struct DWRITE_JUSTIFICATION_OPPORTUNITY {
    expansionMinimum: FLOAT,
    expansionMaximum: FLOAT,
    compressionMaximum: FLOAT,
    bitfield0: UINT32,
}}
BITFIELD!{DWRITE_JUSTIFICATION_OPPORTUNITY bitfield0: UINT32 [
    expansionPriority set_expansionPriority[0..8],
    compressionPriority set_compressionPriority[8..16],
    allowResidualExpansion set_allowResidualExpansion[16..17],
    allowResidualCompression set_allowResidualCompression[17..18],
    applyToLeadingEdge set_applyToLeadingEdge[18..19],
    applyToTrailingEdge set_applyToTrailingEdge[19..20],
    reserved set_reserved[20..32],
]}
RIDL!{#[uuid(0x30572f99, 0xdac6, 0x41db, 0xa1, 0x6e, 0x04, 0x86, 0x30, 0x7e, 0x60, 0x6a)]
interface IDWriteFactory1(IDWriteFactory1Vtbl): IDWriteFactory(IDWriteFactoryVtbl) {
    fn GetEudcFontCollection(
        fontCollection: *mut *mut IDWriteFontCollection,
        checkForUpdates: BOOL,
    ) -> HRESULT,
    fn CreateCustomRenderingParams(
        gamma: FLOAT,
        enhancedContrast: FLOAT,
        enhancedContrastGrayscale: FLOAT,
        clearTypeLevel: FLOAT,
        pixelGeometry: DWRITE_PIXEL_GEOMETRY,
        renderingMode: DWRITE_RENDERING_MODE,
        renderingParams: *mut *mut IDWriteRenderingParams1,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xa71efdb4, 0x9fdb, 0x4838, 0xad, 0x90, 0xcf, 0xc3, 0xbe, 0x8c, 0x3d, 0xaf)]
interface IDWriteFontFace1(IDWriteFontFace1Vtbl): IDWriteFontFace(IDWriteFontFaceVtbl) {
    fn GetMetrics(
        fontMetrics: *mut DWRITE_FONT_METRICS1,
    ) -> (),
    fn GetGdiCompatibleMetrics(
        emSize: FLOAT,
        pixelsPerDip: FLOAT,
        transform: *const DWRITE_MATRIX,
        fontMetrics: *mut DWRITE_FONT_METRICS1,
    ) -> HRESULT,
    fn GetCaretMetrics(
        caretMetrics: *mut DWRITE_CARET_METRICS,
    ) -> (),
    fn GetUnicodeRanges(
        maxRangeCount: UINT32,
        unicodeRanges: *mut DWRITE_UNICODE_RANGE,
        actualRangeCount: *mut UINT32,
    ) -> HRESULT,
    fn IsMonospacedFont() -> BOOL,
    fn GetDesignGlyphAdvances(
        glyphCount: UINT32,
        glyphIndices: *const UINT16,
        glyphAdvances: *mut INT32,
        isSideways: BOOL,
    ) -> HRESULT,
    fn GetGdiCompatibleGlyphAdvances(
        emSize: FLOAT,
        pixelsPerDip: FLOAT,
        transform: *const DWRITE_MATRIX,
        useGdiNatural: BOOL,
        isSideways: BOOL,
        glyphCount: UINT32,
        glyphIndices: *const UINT16,
        glyphAdvances: *mut INT32,
    ) -> HRESULT,
    fn GetKerningPairAdjustments(
        glyphCount: UINT32,
        glyphIndices: *const UINT16,
        glyphAdvanceAdjustments: *mut INT32,
    ) -> HRESULT,
    fn HasKerningPairs() -> BOOL,
    fn GetRecommendedRenderingMode(
        fontEmSize: FLOAT,
        dpiX: FLOAT,
        dpiY: FLOAT,
        transform: *const DWRITE_MATRIX,
        isSideways: BOOL,
        outlineThreshold: DWRITE_OUTLINE_THRESHOLD,
        measuringMode: DWRITE_MEASURING_MODE,
        renderingMode: *mut DWRITE_RENDERING_MODE,
    ) -> HRESULT,
    fn GetVerticalGlyphVariants(
        glyphCount: UINT32,
        nominalGlyphIndices: *const UINT16,
        verticalGlyphIndices: *mut UINT16,
    ) -> HRESULT,
    fn HasVerticalGlyphVariants() -> BOOL,
}}
RIDL!{#[uuid(0xacd16696, 0x8c14, 0x4f5d, 0x87, 0x7e, 0xfe, 0x3f, 0xc1, 0xd3, 0x27, 0x38)]
interface IDWriteFont1(IDWriteFont1Vtbl): IDWriteFont(IDWriteFontVtbl) {
    fn GetMetrics(
        fontMetrics: *mut DWRITE_FONT_METRICS1,
    ) -> (),
    fn GetPanose(
        panose: *mut DWRITE_PANOSE,
    ) -> (),
    fn GetUnicodeRanges(
        maxRangeCount: UINT32,
        unicodeRanges: *mut DWRITE_UNICODE_RANGE,
        actualRangeCount: *mut UINT32,
    ) -> HRESULT,
    fn IsMonospacedFont() -> BOOL,
}}
RIDL!{#[uuid(0x94413cf4, 0xa6fc, 0x4248, 0x8b, 0x50, 0x66, 0x74, 0x34, 0x8f, 0xca, 0xd3)]
interface IDWriteRenderingParams1(IDWriteRenderingParams1Vtbl):
    IDWriteRenderingParams(IDWriteRenderingParamsVtbl) {
    fn GetGrayscaleEnhancedContrast() -> FLOAT,
}}
RIDL!{#[uuid(0x80dad800, 0xe21f, 0x4e83, 0x96, 0xce, 0xbf, 0xcc, 0xe5, 0x00, 0xdb, 0x7c)]
interface IDWriteTextAnalyzer1(IDWriteTextAnalyzer1Vtbl):
    IDWriteTextAnalyzer(IDWriteTextAnalyzerVtbl) {
    fn ApplyCharacterSpacing(
        leadingSpacing: FLOAT,
        trailingSpacing: FLOAT,
        minimumAdvanceWidth: FLOAT,
        textLength: UINT32,
        glyphCount: UINT32,
        clusterMap: *const UINT16,
        glyphAdvances: *const FLOAT,
        glyphOffsets: *const DWRITE_GLYPH_OFFSET,
        glyphProperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        modifiedGlyphAdvances: *mut FLOAT,
        modifiedGlyphOffsets: *mut DWRITE_GLYPH_OFFSET,
    ) -> HRESULT,
    fn GetBaseline(
        fontFace: *mut IDWriteFontFace,
        baseline: DWRITE_BASELINE,
        isVertical: BOOL,
        isSimulationAllowed: BOOL,
        scriptAnalysis: DWRITE_SCRIPT_ANALYSIS,
        localeName: *const WCHAR,
        baselineCoordinate: *mut INT32,
        exists: *mut BOOL,
    ) -> HRESULT,
    fn AnalyzeVerticalGlyphOrientation(
        analysisSource: *mut IDWriteTextAnalysisSource1,
        textPosition: UINT32,
        textLength: UINT32,
        analysisSink: *mut IDWriteTextAnalysisSink1,
    ) -> HRESULT,
    fn GetGlyphOrientationTransform(
        glyphOrientationAngle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        isSideways: BOOL,
        transform: *mut DWRITE_MATRIX,
    ) -> HRESULT,
    fn GetScriptProperties(
        scriptAnalysis: DWRITE_SCRIPT_ANALYSIS,
        scriptProperties: *mut DWRITE_SCRIPT_PROPERTIES,
    ) -> HRESULT,
    fn GetTextComplexity(
        textString: *const WCHAR,
        textLength: UINT32,
        fontFace: *mut IDWriteFontFace,
        isTextSimple: *mut BOOL,
        textLengthRead: *mut UINT32,
        glyphIndices: *mut UINT16,
    ) -> HRESULT,
    fn GetJustificationOpportunities(
        fontFace: *mut IDWriteFontFace,
        fontEmSize: FLOAT,
        scriptAnalysis: DWRITE_SCRIPT_ANALYSIS,
        textLength: UINT32,
        glyphCount: UINT32,
        textString: *const WCHAR,
        clusterMap: *const UINT16,
        glyphProperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        justificationOpportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY,
    ) -> HRESULT,
    fn JustifyGlyphAdvances(
        lineWidth: FLOAT,
        glyphCount: UINT32,
        justificationOpportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY,
        glyphAdvances: *const FLOAT,
        glyphOffsets: *const DWRITE_GLYPH_OFFSET,
        justifiedGlyphAdvances: *mut FLOAT,
        justifiedGlyphOffsets: *mut DWRITE_GLYPH_OFFSET,
    ) -> HRESULT,
    fn GetJustifiedGlyphs(
        fontFace: *mut IDWriteFontFace,
        fontEmSize: FLOAT,
        scriptAnalysis: DWRITE_SCRIPT_ANALYSIS,
        textLength: UINT32,
        glyphCount: UINT32,
        maxGlyphCount: UINT32,
        clusterMap: *const UINT16,
        glyphIndices: *const UINT16,
        glyphAdvances: *const FLOAT,
        justifiedGlyphAdvances: *const FLOAT,
        justifiedGlyphOffsets: *const DWRITE_GLYPH_OFFSET,
        glyphProperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        actualGlyphCount: *mut UINT32,
        modifiedClusterMap: *mut UINT16,
        modifiedGlyphIndices: *mut UINT16,
        modifiedGlyphAdvances: *mut FLOAT,
        modifiedGlyphOffsets: *mut DWRITE_GLYPH_OFFSET,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x639cfad8, 0x0fb4, 0x4b21, 0xa5, 0x8a, 0x06, 0x79, 0x20, 0x12, 0x00, 0x09)]
interface IDWriteTextAnalysisSource1(IDWriteTextAnalysisSource1Vtbl):
    IDWriteTextAnalysisSource(IDWriteTextAnalysisSourceVtbl) {
    fn GetVerticalGlyphOrientation(
        textPosition: UINT32,
        textLength: *mut UINT32,
        glyphOrientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION,
        bidiLevel: *mut UINT8,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xb0d941a0, 0x85e7, 0x4d8b, 0x9f, 0xd3, 0x5c, 0xed, 0x99, 0x34, 0x48, 0x2a)]
interface IDWriteTextAnalysisSink1(IDWriteTextAnalysisSink1Vtbl):
    IDWriteTextAnalysisSink(IDWriteTextAnalysisSinkVtbl) {
    fn SetGlyphOrientation(
        textPosition: UINT32,
        textLength: UINT32,
        glyphOrientationAngle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        adjustedBidiLevel: UINT8,
        isSideways: BOOL,
        isRightToLeft: BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x9064d822, 0x80a7, 0x465c, 0xa9, 0x86, 0xdf, 0x65, 0xf7, 0x8b, 0x8f, 0xeb)]
interface IDWriteTextLayout1(IDWriteTextLayout1Vtbl):
    IDWriteTextLayout(IDWriteTextLayoutVtbl) {
    fn SetPairKerning(
        isPairKerningEnabled: BOOL,
        textRange: DWRITE_TEXT_RANGE,
    ) -> HRESULT,
    fn GetPairKerning(
        currentPosition: UINT32,
        isPairKerningEnabled: *mut BOOL,
        textRange: *mut DWRITE_TEXT_RANGE,
    ) -> HRESULT,
    fn SetCharacterSpacing(
        leadingSpacing: FLOAT,
        trailingSpacing: FLOAT,
        minimumAdvanceWidth: FLOAT,
        textRange: DWRITE_TEXT_RANGE,
    ) -> HRESULT,
    fn GetCharacterSpacing(
        currentPosition: UINT32,
        leadingSpacing: *mut FLOAT,
        trailingSpacing: *mut FLOAT,
        minimumAdvanceWidth: *mut FLOAT,
        textRange: *mut DWRITE_TEXT_RANGE,
    ) -> HRESULT,
}}
ENUM!{enum DWRITE_TEXT_ANTIALIAS_MODE {
    DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE = 0x0, // 0
    DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE = 0x1, // 1
}}
RIDL!{#[uuid(0x791e8298, 0x3ef3, 0x4230, 0x98, 0x80, 0xc9, 0xbd, 0xec, 0xc4, 0x20, 0x64)]
interface IDWriteBitmapRenderTarget1(IDWriteBitmapRenderTarget1Vtbl):
    IDWriteBitmapRenderTarget(IDWriteBitmapRenderTargetVtbl) {
    fn GetTextAntialiasMode() -> DWRITE_TEXT_ANTIALIAS_MODE,
    fn SetTextAntialiasMode(
        antialiasMode: DWRITE_TEXT_ANTIALIAS_MODE,
    ) -> HRESULT,
}}
