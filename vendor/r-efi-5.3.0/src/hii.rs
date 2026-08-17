//! Human Interface Infrastructure (HII)
//!
//! This module contains bindings and definitions copied from Section 33.3 of
//! the UEFI spec, as well as the core HII related definitions.

//
// Core HII Definitions
//

// This is the exception to the rule. It's defined in 34.8 (HII_DATABASE
// protocol), not 33.3, but it's used throughout the HII protocols, so it makes
// sense to be defined at the base.
pub type Handle = *mut core::ffi::c_void;

//
// 33.3.1 Package Lists and Package Headers
//

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PackageHeader<const N: usize = 0> {
    pub length: [u8; 3],
    pub r#type: u8,
    pub data: [u8; N],
}

pub const PACKAGE_TYPE_ALL: u8 = 0x00;
pub const PACKAGE_TYPE_GUID: u8 = 0x01;
pub const PACKAGE_FORMS: u8 = 0x02;
pub const PACKAGE_STRINGS: u8 = 0x04;
pub const PACKAGE_FONTS: u8 = 0x05;
pub const PACKAGE_IMAGES: u8 = 0x06;
pub const PACKAGE_SIMPLE_FONTS: u8 = 0x07;
pub const PACKAGE_DEVICE_PATH: u8 = 0x08;
pub const PACKAGE_KEYBOARD_LAYOUT: u8 = 0x09;
pub const PACKAGE_ANIMATIONS: u8 = 0x0A;
pub const PACKAGE_END: u8 = 0xDF;
pub const PACKAGE_TYPE_SYSTEM_BEGIN: u8 = 0xE0;
pub const PACKAGE_TYPE_SYSTEM_END: u8 = 0xFF;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PackageListHeader {
    pub package_list_guid: crate::base::Guid,
    pub package_length: u32,
}

//
// 33.3.3 Font Package
//

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FontPackageHdr<const N: usize = 0> {
    pub header: PackageHeader,
    pub hdr_size: u32,
    pub glyph_block_offset: u32,
    pub cell: GlyphInfo,
    pub font_style: FontStyle,
    pub font_family: [crate::base::Char16; N],
}

pub type FontStyle = u32;

pub const FONT_STYLE_NORMAL: FontStyle = 0x00000000;
pub const FONT_STYLE_BOLD: FontStyle = 0x00000001;
pub const FONT_STYLE_ITALIC: FontStyle = 0x00000002;
pub const FONT_STYLE_EMBOSS: FontStyle = 0x00010000;
pub const FONT_STYLE_OUTLINE: FontStyle = 0x00020000;
pub const FONT_STYLE_SHADOW: FontStyle = 0x00040000;
pub const FONT_STYLE_UNDERLINE: FontStyle = 0x00080000;
pub const FONT_STYLE_DBL_UNDER: FontStyle = 0x00100000;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GlyphBlock<const N: usize = 0> {
    pub block_type: u8,
    pub block_body: [u8; N],
}

pub const GIBT_END: u8 = 0x00;
pub const GIBT_GLYPH: u8 = 0x10;
pub const GIBT_GLYPHS: u8 = 0x11;
pub const GIBT_GLYPH_DEFAULT: u8 = 0x12;
pub const GIBT_GLYPHS_DEFAULT: u8 = 0x13;
pub const GIBT_GLYPH_VARIABILITY: u8 = 0x14;
pub const GIBT_DUPLICATE: u8 = 0x20;
pub const GIBT_SKIP2: u8 = 0x21;
pub const GIBT_SKIP1: u8 = 0x22;
pub const GIBT_DEFAULTS: u8 = 0x23;
pub const GIBT_EXT1: u8 = 0x30;
pub const GIBT_EXT2: u8 = 0x31;
pub const GIBT_EXT4: u8 = 0x32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GlyphInfo {
    pub width: u16,
    pub height: u16,
    pub offset_x: i16,
    pub offset_y: i16,
    pub advance_x: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtDefaultsBlock {
    pub header: GlyphBlock,
    pub cell: GlyphInfo,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtDuplicateBlock {
    pub header: GlyphBlock,
    pub char_value: crate::base::Char16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GlyphGibtEndBlock {
    pub header: GlyphBlock,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtExt1Block {
    pub header: GlyphBlock,
    pub block_type_2: u8,
    pub length: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtExt2Block {
    pub header: GlyphBlock,
    pub block_type_2: u8,
    pub length: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtExt4Block {
    pub header: GlyphBlock,
    pub block_type_2: u8,
    pub length: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtGlyphBlock<const N: usize = 0> {
    pub header: GlyphBlock,
    pub cell: GlyphInfo,
    pub bitmap_data: [u8; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtGlyphsBlock<const N: usize = 0> {
    pub header: GlyphBlock,
    pub cell: GlyphInfo,
    pub count: u16,
    pub bitmap_data: [u8; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtGlyphDefaultBlock<const N: usize = 0> {
    pub header: GlyphBlock,
    pub bitmap_data: [u8; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtGlypshDefaultBlock<const N: usize = 0> {
    pub header: GlyphBlock,
    pub count: u16,
    pub bitmap_data: [u8; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtSkip2Block {
    pub header: GlyphBlock,
    pub skip_count: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtSkip1Block {
    pub header: GlyphBlock,
    pub skip_count: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GibtVariabilityBlock<const N: usize = 0> {
    pub header: GlyphBlock,
    pub cell: GlyphInfo,
    pub glyph_pack_in_bits: u8,
    pub bitmap_data: [u8; N],
}

//
// 33.3.8 Forms Package
//

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FormPackageHdr {
    pub header: PackageHeader,
    pub op_code_header: IfrOpHeader,
    // Op-Codes Follow...
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrOpHeader {
    pub op_code: u8,
    pub length_and_scope: u8, // Length:7, Scope:1
}

pub type QuestionId = u16;
pub type ImageId = u16;
pub type StringId = u16;
pub type FormId = u16;
pub type VarstoreId = u16;
pub type AnimationId = u16;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrQuestionHeader {
    pub header: IfrStatementHeader,
    pub question_id: QuestionId,
    pub var_store_id: VarstoreId,
    pub var_store_info: IfrQuestionHeaderVarstoreInfo,
    pub flags: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union IfrQuestionHeaderVarstoreInfo {
    pub var_name: StringId,
    pub var_offset: u16,
}

pub const IFR_FLAG_READ_ONLY: u8 = 0x01;
pub const IFR_FLAG_CALLBACK: u8 = 0x04;
pub const IFR_FLAG_RESET_REQUIRED: u8 = 0x10;
pub const IFR_FLAG_REST_STYLE: u8 = 0x20;
pub const IFR_FLAG_RECONNECT_REQUIRED: u8 = 0x40;
pub const IFR_FLAG_OPTIONS_ONLY: u8 = 0x80;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrStatementHeader {
    pub prompt: StringId,
    pub help: StringId,
}

pub const IFR_FORM_OP: u8 = 0x01;
pub const IFR_SUBTITLE_OP: u8 = 0x02;
pub const IFR_TEXT_OP: u8 = 0x03;
pub const IFR_IMAGE_OP: u8 = 0x04;
pub const IFR_ONE_OF_OP: u8 = 0x05;
pub const IFR_CHECKBOX_OP: u8 = 0x06;
pub const IFR_NUMERIC_OP: u8 = 0x07;
pub const IFR_PASSWORD_OP: u8 = 0x08;
pub const IFR_ONE_OF_OPTION_OP: u8 = 0x09;
pub const IFR_SUPPRESS_IF_OP: u8 = 0x0A;
pub const IFR_LOCKED_OP: u8 = 0x0B;
pub const IFR_ACTION_OP: u8 = 0x0C;
pub const IFR_RESET_BUTTON_OP: u8 = 0x0D;
pub const IFR_FORM_SET_OP: u8 = 0x0E;
pub const IFR_REF_OP: u8 = 0x0F;
pub const IFR_NO_SUBMIT_IF_OP: u8 = 0x10;
pub const IFR_INCONSISTENT_IF_OP: u8 = 0x11;
pub const IFR_EQ_ID_VAL_OP: u8 = 0x12;
pub const IFR_EQ_ID_ID_OP: u8 = 0x13;
pub const IFR_EQ_ID_VAL_LIST_OP: u8 = 0x14;
pub const IFR_AND_OP: u8 = 0x15;
pub const IFR_OR_OP: u8 = 0x16;
pub const IFR_NOT_OP: u8 = 0x17;
pub const IFR_RULE_OP: u8 = 0x18;
pub const IFR_GRAY_OUT_IF_OP: u8 = 0x19;
pub const IFR_DATE_OP: u8 = 0x1A;
pub const IFR_TIME_OP: u8 = 0x1B;
pub const IFR_STRING_OP: u8 = 0x1C;
pub const IFR_REFRESH_OP: u8 = 0x1D;
pub const IFR_DISABLE_IF_OP: u8 = 0x1E;
pub const IFR_ANIMATION_OP: u8 = 0x1F;
pub const IFR_TO_LOWER_OP: u8 = 0x20;
pub const IFR_TO_UPPER_OP: u8 = 0x21;
pub const IFR_MAP_OP: u8 = 0x22;
pub const IFR_ORDERED_LIST_OP: u8 = 0x23;
pub const IFR_VARSTORE_OP: u8 = 0x24;
pub const IFR_VARSTORE_NAME_VALUE_OP: u8 = 0x25;
pub const IFR_VARSTORE_EFI_OP: u8 = 0x26;
pub const IFR_VARSTORE_DEVICE_OP: u8 = 0x27;
pub const IFR_VERSION_OP: u8 = 0x28;
pub const IFR_END_OP: u8 = 0x29;
pub const IFR_MATCH_OP: u8 = 0x2A;
pub const IFR_GET_OP: u8 = 0x2B;
pub const IFR_SET_OP: u8 = 0x2C;
pub const IFR_READ_OP: u8 = 0x2D;
pub const IFR_WRITE_OP: u8 = 0x2E;
pub const IFR_EQUAL_OP: u8 = 0x2F;
pub const IFR_NOT_EQUAL_OP: u8 = 0x30;
pub const IFR_GREATER_THAN_OP: u8 = 0x31;
pub const IFR_GREATER_EQUAL_OP: u8 = 0x32;
pub const IFR_LESS_THAN_OP: u8 = 0x33;
pub const IFR_LESS_EQUAL_OP: u8 = 0x34;
pub const IFR_BITWISE_AND_OP: u8 = 0x35;
pub const IFR_BITWISE_OR_OP: u8 = 0x36;
pub const IFR_BITWISE_NOT_OP: u8 = 0x37;
pub const IFR_SHIFT_LEFT_OP: u8 = 0x38;
pub const IFR_SHIFT_RIGHT_OP: u8 = 0x39;
pub const IFR_ADD_OP: u8 = 0x3A;
pub const IFR_SUBTRACT_OP: u8 = 0x3B;
pub const IFR_MULTIPLY_OP: u8 = 0x3C;
pub const IFR_DIVIDE_OP: u8 = 0x3D;
pub const IFR_MODULO_OP: u8 = 0x3E;
pub const IFR_RULE_REF_OP: u8 = 0x3F;
pub const IFR_QUESTION_REF1_OP: u8 = 0x40;
pub const IFR_QUESTION_REF2_OP: u8 = 0x41;
pub const IFR_UINT8_OP: u8 = 0x42;
pub const IFR_UINT16_OP: u8 = 0x43;
pub const IFR_UINT32_OP: u8 = 0x44;
pub const IFR_UINT64_OP: u8 = 0x45;
pub const IFR_TRUE_OP: u8 = 0x46;
pub const IFR_FALSE_OP: u8 = 0x47;
pub const IFR_TO_UINT_OP: u8 = 0x48;
pub const IFR_TO_STRING_OP: u8 = 0x49;
pub const IFR_TO_BOOLEAN_OP: u8 = 0x4A;
pub const IFR_MID_OP: u8 = 0x4B;
pub const IFR_FIND_OP: u8 = 0x4C;
pub const IFR_TOKEN_OP: u8 = 0x4D;
pub const IFR_STRING_REF1_OP: u8 = 0x4E;
pub const IFR_STRING_REF2_OP: u8 = 0x4F;
pub const IFR_CONDITIONAL_OP: u8 = 0x50;
pub const IFR_QUESTION_REF3_OP: u8 = 0x51;
pub const IFR_ZERO_OP: u8 = 0x52;
pub const IFR_ONE_OP: u8 = 0x53;
pub const IFR_ONES_OP: u8 = 0x54;
pub const IFR_UNDEFINED_OP: u8 = 0x55;
pub const IFR_LENGTH_OP: u8 = 0x56;
pub const IFR_DUP_OP: u8 = 0x57;
pub const IFR_THIS_OP: u8 = 0x58;
pub const IFR_SPAN_OP: u8 = 0x59;
pub const IFR_VALUE_OP: u8 = 0x5A;
pub const IFR_DEFAULT_OP: u8 = 0x5B;
pub const IFR_DEFAULTSTORE_OP: u8 = 0x5C;
pub const IFR_FORM_MAP_OP: u8 = 0x5D;
pub const IFR_CATENATE_OP: u8 = 0x5E;
pub const IFR_GUID_OP: u8 = 0x5F;
pub const IFR_SECURITY_OP: u8 = 0x60;
pub const IFR_MODAL_TAG_OP: u8 = 0x61;
pub const IFR_REFRESH_ID_OP: u8 = 0x62;
pub const IFR_WARNING_IF_OP: u8 = 0x63;
pub const IFR_MATCH2_OP: u8 = 0x64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrAction {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub question_config: StringId,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrAction1 {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrAnimation {
    pub header: IfrOpHeader,
    pub id: AnimationId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrAdd {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrAnd {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrBitwiseAnd {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrBitwiseNot {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrBitwiseOr {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrCatenate {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrCheckbox {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub flags: u8,
}

pub const IFR_CHECKBOX_DEFAULT: u8 = 0x01;
pub const IFR_CHECKBOX_DEFAULT_MFG: u8 = 0x02;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrConditional {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrDate {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub flags: u8,
}

pub const QF_DATE_YEAR_SUPPRESS: u8 = 0x01;
pub const QF_DATE_MONTH_SUPPRESS: u8 = 0x02;
pub const QF_DATE_DAY_SUPPRESS: u8 = 0x04;
pub const QF_DATE_STORAGE: u8 = 0x30;

pub const QF_DATE_STORAGE_NORMAL: u8 = 0x00;
pub const QF_DATE_STORAGE_TIME: u8 = 0x10;
pub const QF_DATE_STORAGE_WAKEUP: u8 = 0x20;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrDefault {
    pub header: IfrOpHeader,
    pub default_id: u16,
    pub r#type: u8,
    pub value: IfrTypeValue,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrDefault2 {
    pub header: IfrOpHeader,
    pub default_id: u16,
    pub r#type: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrDefaultstore {
    pub header: IfrOpHeader,
    pub default_name: StringId,
    pub default_id: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrDisableIf {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrDivide {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrDup {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrEnd {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrEqual {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrEqIdId {
    pub header: IfrOpHeader,
    pub question_id_1: QuestionId,
    pub question_id_2: QuestionId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrEqIdValList<const N: usize = 0> {
    pub header: IfrOpHeader,
    pub question_id: QuestionId,
    pub list_length: u16,
    pub value_list: [u16; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrEqIdVal {
    pub header: IfrOpHeader,
    pub question_id: QuestionId,
    pub value: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrFalse {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrFind {
    pub header: IfrOpHeader,
    pub format: u8,
}

pub const IFR_FF_CASE_SENSITIVE: u8 = 0x00;
pub const IFR_FF_CASE_INSENSITIVE: u8 = 0x01;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrForm {
    pub header: IfrOpHeader,
    pub form_id: FormId,
    pub form_title: StringId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrFormMapMethod {
    pub method_title: StringId,
    pub method_identifier: crate::base::Guid,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrFormMap<const N: usize = 0> {
    pub header: IfrOpHeader,
    pub form_id: FormId,
    pub methods: [IfrFormMapMethod; N],
}

pub const STANDARD_FORM_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0x3bd2f4ec,
    0xe524,
    0x46e4,
    0xa9,
    0xd8,
    &[0x51, 0x01, 0x17, 0x42, 0x55, 0x62],
);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrFormSet<const N: usize = 0> {
    pub header: IfrOpHeader,
    pub guid: crate::base::Guid,
    pub form_set_title: StringId,
    pub help: StringId,
    pub flags: u8,
    pub class_guid: [crate::base::Guid; N],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrGet {
    pub header: IfrOpHeader,
    pub var_store_id: VarstoreId,
    pub var_store_info: IfrGetVarStoreInfo,
    pub var_store_type: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union IfrGetVarStoreInfo {
    pub var_name: StringId,
    pub var_offset: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrGrayOutIf {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrGreaterEqual {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrGreaterThan {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrGuid {
    pub header: IfrOpHeader,
    pub guid: crate::base::Guid,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrImage {
    pub id: ImageId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrInconsistentIf {
    pub header: IfrOpHeader,
    pub error: StringId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrLength {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrLessEqual {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrLessThan {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrLocked {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrMap {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrMatch {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrMid {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrModalTag {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrModulo {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrMultiply {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrNot {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrNotEqual {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrNoSubmitIf {
    pub header: IfrOpHeader,
    pub error: StringId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrNumericDataU8 {
    pub min_value: u8,
    pub max_value: u8,
    pub step: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrNumericDataU16 {
    pub min_value: u16,
    pub max_value: u16,
    pub step: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrNumericDataU32 {
    pub min_value: u32,
    pub max_value: u32,
    pub step: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrNumericDataU64 {
    pub min_value: u64,
    pub max_value: u64,
    pub step: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union IfrNumericData {
    pub r#u8: IfrNumericDataU8,
    pub r#u16: IfrNumericDataU16,
    pub r#u32: IfrNumericDataU32,
    pub r#u64: IfrNumericDataU64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrNumeric {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub flags: u8,
    pub data: IfrNumericData,
}

pub const IFR_NUMERIC_SIZE: u8 = 0x03;
pub const IFR_NUMERIC_SIZE_1: u8 = 0x00;
pub const IFR_NUMERIC_SIZE_2: u8 = 0x01;
pub const IFR_NUMERIC_SIZE_4: u8 = 0x02;
pub const IFR_NUMERIC_SIZE_8: u8 = 0x03;

pub const IFR_DISPLAY: u8 = 0x30;
pub const IFR_DISPLAY_INT_DEC: u8 = 0x00;
pub const IFR_DISPLAY_UINT_DEC: u8 = 0x10;
pub const IFR_DISPLAY_UINT_HEX: u8 = 0x20;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrOne {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrOnes {
    pub header: IfrOpHeader,
}

type IfrOneOfData = IfrNumericData;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrOneOf {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub flags: u8,
    pub data: IfrOneOfData,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrOneOfOption {
    pub header: IfrOpHeader,
    pub option: StringId,
    pub flags: u8,
    pub r#type: u8,
    pub value: IfrTypeValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union IfrTypeValue<const N: usize = 0> {
    pub r#u8: u8,
    pub r#u16: u16,
    pub r#u32: u32,
    pub r#u64: u64,
    pub b: crate::base::Boolean,
    pub time: Time,
    pub date: Date,
    pub string: StringId,
    pub r#ref: Ref,
    pub buffer: [u8; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Ref {
    pub question_id: QuestionId,
    pub form_id: FormId,
    pub form_set_guid: crate::base::Guid,
    pub device_path: StringId,
}

pub const IFR_TYPE_NUM_SIZE_8: u8 = 0x00;
pub const IFR_TYPE_NUM_SIZE_16: u8 = 0x01;
pub const IFR_TYPE_NUM_SIZE_32: u8 = 0x02;
pub const IFR_TYPE_NUM_SIZE_64: u8 = 0x03;
pub const IFR_TYPE_BOOLEAN: u8 = 0x04;
pub const IFR_TYPE_TIME: u8 = 0x05;
pub const IFR_TYPE_DATE: u8 = 0x06;
pub const IFR_TYPE_STRING: u8 = 0x07;
pub const IFR_TYPE_OTHER: u8 = 0x08;
pub const IFR_TYPE_UNDEFINED: u8 = 0x09;
pub const IFR_TYPE_ACTION: u8 = 0x0A;
pub const IFR_TYPE_BUFFER: u8 = 0x0B;
pub const IFR_TYPE_REF: u8 = 0x0C;

pub const IFR_OPTION_DEFAULT: u8 = 0x10;
pub const IFR_OPTION_DEFAULT_MFG: u8 = 0x20;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrOr {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrOrderedList {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub max_containers: u8,
    pub flags: u8,
}

pub const IFR_UNIQUE_SET: u8 = 0x01;
pub const IFR_NO_EMPTY_SET: u8 = 0x02;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrPassword {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub min_size: u16,
    pub max_size: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrQuestionRef1 {
    pub header: IfrOpHeader,
    pub question_id: QuestionId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrQuestionRef2 {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrQuestionRef3 {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrQuestionRef32 {
    pub header: IfrOpHeader,
    pub device_path: StringId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrQuestionRef33 {
    pub header: IfrOpHeader,
    pub device_path: StringId,
    pub guid: crate::base::Guid,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrRead {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrRef {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub form_id: FormId,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrRef2 {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub form_id: FormId,
    pub question_id: QuestionId,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrRef3 {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub form_id: FormId,
    pub question_id: QuestionId,
    pub form_set_id: crate::base::Guid,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrRef4 {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub form_id: FormId,
    pub question_id: QuestionId,
    pub form_set_id: crate::base::Guid,
    pub device_path: StringId,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrRef5 {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrRefresh {
    pub header: IfrOpHeader,
    pub refresh_interval: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrRefreshId {
    pub header: IfrOpHeader,
    pub refresh_event_group_id: crate::base::Guid,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrResetButton {
    pub header: IfrOpHeader,
    pub statement: IfrStatementHeader,
    pub deafult_id: DefaultId,
}

pub type DefaultId = u16;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrRule {
    pub header: IfrOpHeader,
    pub rule_id: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrRuleRef {
    pub header: IfrOpHeader,
    pub rule_id: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrSecurity {
    pub header: IfrOpHeader,
    pub permissions: crate::base::Guid,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union IfrSetVarStoreInfo {
    pub var_name: StringId,
    pub var_offset: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrSet {
    pub header: IfrOpHeader,
    pub var_store_id: VarstoreId,
    pub var_store_info: IfrSetVarStoreInfo,
    pub var_store_type: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrShiftLeft {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrShiftRight {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrSpan {
    pub header: IfrOpHeader,
    pub flags: u8,
}

pub const IFR_FLAGS_FIRST_MATCHING: u8 = 0x00;
pub const IFR_FLAGS_FIRST_NON_MATCHING: u8 = 0x01;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrString {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub min_size: u8,
    pub max_size: u8,
    pub flags: u8,
}

pub const IFR_STRING_MULTI_LINE: u8 = 0x01;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrStringRef1 {
    pub header: IfrOpHeader,
    pub string_id: StringId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrStringRef2 {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrSubtitle {
    pub header: IfrOpHeader,
    pub statement: IfrStatementHeader,
    pub flags: u8,
}

pub const IFR_FLAGS_HORIZONTAL: u8 = 0x01;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrSubtract {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrSuppressIf {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrText {
    pub header: IfrOpHeader,
    pub statement: IfrStatementHeader,
    pub text_two: StringId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrThis {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IfrTime {
    pub header: IfrOpHeader,
    pub question: IfrQuestionHeader,
    pub flags: u8,
}

pub const QF_TIME_HOUR_SUPPRESS: u8 = 0x01;
pub const QF_TIME_MINUTE_SUPPRESS: u8 = 0x02;
pub const QF_TIME_SECOND_SUPPRESS: u8 = 0x04;
pub const QF_TIME_STORAGE: u8 = 0x30;

pub const QF_TIME_STORAGE_NORMAL: u8 = 0x00;
pub const QF_TIME_STORAGE_TIME: u8 = 0x10;
pub const QF_TIME_STORAGE_WAKEUP: u8 = 0x20;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrToken {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrToBoolean {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrToLower {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrToString {
    pub header: IfrOpHeader,
    pub format: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrToUint {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrToUpper {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrTrue {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrUint8 {
    pub header: IfrOpHeader,
    pub value: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrUint16 {
    pub header: IfrOpHeader,
    pub value: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrUint32 {
    pub header: IfrOpHeader,
    pub value: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrUint64 {
    pub header: IfrOpHeader,
    pub value: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrUndefined {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrValue {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrVarstore<const N: usize = 0> {
    pub header: IfrOpHeader,
    pub guid: crate::base::Guid,
    pub var_store_id: VarstoreId,
    pub size: u16,
    pub name: [u8; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrVarstoreNameValue {
    pub header: IfrOpHeader,
    pub var_store_id: VarstoreId,
    pub guid: crate::base::Guid,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrVarstoreEfi<const N: usize = 0> {
    pub header: IfrOpHeader,
    pub var_store_id: VarstoreId,
    pub guid: crate::base::Guid,
    pub attributes: u32,
    pub size: u16,
    pub name: [u8; N],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrVarstoreDevice {
    pub header: IfrOpHeader,
    pub device_path: StringId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrVersion {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrWrite {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrZero {
    pub header: IfrOpHeader,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrWarningIf {
    pub header: IfrOpHeader,
    pub warning: StringId,
    pub time_out: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IfrMatch2 {
    pub header: IfrOpHeader,
    pub syntax_type: crate::base::Guid,
}
