//! Layout of Wasm GC objects in the deferred reference-counting collector.

use super::*;

/// The size of the `VMDrcHeader` header for GC objects.
pub const HEADER_SIZE: u32 = 24;

/// The align of the `VMDrcHeader` header for GC objects.
pub const HEADER_ALIGN: u32 = 8;

/// The offset of the length field in a `VMDrcArrayHeader`.
pub const ARRAY_LENGTH_OFFSET: u32 = HEADER_SIZE;

/// The bit within a `VMDrcHeader`'s reserved bits that is the mark
/// bit. Collectively, this bit in all the heap's objects' headers implements
/// the precise-stack-roots set.
pub const HEADER_MARK_BIT: u32 = 1 << 0;

/// The bit within a `VMDrcHeader`'s reserved bits that is the
/// in-the-over-approximated-stack-roots list bit.
pub const HEADER_IN_OVER_APPROX_LIST_BIT: u32 = 1 << 1;

/// The layout of Wasm GC objects in the deferred reference-counting collector.
#[derive(Default)]
pub struct DrcTypeLayouts;

impl GcTypeLayouts for DrcTypeLayouts {
    fn array_length_field_offset(&self) -> u32 {
        ARRAY_LENGTH_OFFSET
    }

    fn array_layout(&self, ty: &WasmArrayType) -> GcArrayLayout {
        common_array_layout(ty, HEADER_SIZE, HEADER_ALIGN, ARRAY_LENGTH_OFFSET)
    }

    fn struct_layout(&self, ty: &WasmStructType) -> GcStructLayout {
        common_struct_layout(ty, HEADER_SIZE, HEADER_ALIGN)
    }

    fn exn_layout(&self, ty: &WasmExnType) -> GcExceptionLayout {
        common_exn_layout(ty, HEADER_SIZE, HEADER_ALIGN)
    }
}
