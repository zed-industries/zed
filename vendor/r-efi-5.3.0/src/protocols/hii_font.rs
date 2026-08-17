//! HII Font Protocol

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xe9ca4775,
    0x8657,
    0x47fc,
    0x97,
    0xe7,
    &[0x7e, 0xd6, 0x5a, 0x08, 0x43, 0x24],
);

pub type ProtocolStringToImage = eficall! {fn(
    *const Protocol,
    OutFlags,
    String,
    *const super::hii_font_ex::DisplayInfo,
    *mut *mut super::hii_font_ex::ImageOutput,
    usize,
    usize,
    *mut *mut RowInfo,
    *mut usize,
    *mut usize,
) -> crate::base::Status};

pub type ProtocolStringIdToImage = eficall! {fn(
    *const Protocol,
    OutFlags,
    crate::hii::Handle,
    crate::hii::StringId,
    *const crate::base::Char8,
    *const super::hii_font_ex::DisplayInfo,
    *mut *mut super::hii_font_ex::ImageOutput,
    usize,
    usize,
    *mut *mut RowInfo,
    *mut usize,
    *mut usize,
) -> crate::base::Status};

pub type ProtocolGetGlyph = eficall! {fn(
    *const Protocol,
    crate::base::Char16,
    *const super::hii_font_ex::DisplayInfo,
    *mut *mut super::hii_font_ex::ImageOutput,
    *mut usize,
) -> crate::base::Status};

pub type ProtocolGetFontInfo = eficall! {fn(
    *const Protocol,
    *mut Handle,
    *const super::hii_font_ex::DisplayInfo,
    *mut *mut super::hii_font_ex::DisplayInfo,
    String,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub string_to_image: ProtocolStringToImage,
    pub string_id_to_image: ProtocolStringIdToImage,
    pub get_glyph: ProtocolGetGlyph,
    pub get_font_info: ProtocolGetFontInfo,
}

pub type OutFlags = u32;

pub const OUT_FLAG_CLIP: OutFlags = 0x00000001;
pub const OUT_FLAG_WRAP: OutFlags = 0x00000002;
pub const OUT_FLAG_CLIP_CLEAN_Y: OutFlags = 0x00000004;
pub const OUT_FLAG_CLIP_CLEAN_X: OutFlags = 0x00000008;
pub const OUT_FLAG_TRANSPARENT: OutFlags = 0x00000010;
pub const IGNORE_IF_NO_GLYPH: OutFlags = 0x00000020;
pub const IGNORE_LINE_BREAK: OutFlags = 0x00000040;
pub const DIRECT_TO_SCREEN: OutFlags = 0x00000080;

pub type String = *mut crate::base::Char16;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RowInfo {
    pub start_index: usize,
    pub end_index: usize,
    pub line_height: usize,
    pub line_width: usize,
    pub baseline_offset: usize,
}

pub type Handle = *mut core::ffi::c_void;
