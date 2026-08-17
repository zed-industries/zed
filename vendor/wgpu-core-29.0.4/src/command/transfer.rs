use alloc::{format, string::String, sync::Arc, vec::Vec};

use arrayvec::ArrayVec;
use thiserror::Error;
use wgt::{
    error::{ErrorType, WebGpuError},
    BufferAddress, BufferTextureCopyInfoError, BufferUsages, Extent3d, TextureSelector,
    TextureUsages,
};

use crate::{
    api_log,
    command::{
        clear_texture, encoder::EncodingState, ArcCommand, CommandEncoderError, EncoderStateError,
    },
    device::MissingDownlevelFlags,
    global::Global,
    id::{BufferId, CommandEncoderId, TextureId},
    init_tracker::{
        has_copy_partial_init_tracker_coverage, MemoryInitKind, TextureInitRange,
        TextureInitTrackerAction,
    },
    resource::{
        Buffer, MissingBufferUsageError, MissingTextureUsageError, ParentDevice, RawResourceAccess,
        Texture, TextureErrorDimension,
    },
};

use super::ClearError;

type TexelCopyBufferInfo = wgt::TexelCopyBufferInfo<BufferId>;
type TexelCopyTextureInfo = wgt::TexelCopyTextureInfo<Arc<Texture>>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CopySide {
    Source,
    Destination,
}

/// Error encountered while attempting a data transfer.
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum TransferError {
    #[error("Source and destination cannot be the same buffer")]
    SameSourceDestinationBuffer,
    #[error(transparent)]
    MissingBufferUsage(#[from] MissingBufferUsageError),
    #[error(transparent)]
    MissingTextureUsage(#[from] MissingTextureUsageError),
    #[error(
        "Copy at offset {start_offset} bytes would end up overrunning the bounds of the {side:?} buffer of size {buffer_size}"
    )]
    BufferStartOffsetOverrun {
        start_offset: BufferAddress,
        buffer_size: BufferAddress,
        side: CopySide,
    },
    #[error(
        "Copy at offset {start_offset} for {size} bytes would end up overrunning the bounds of the {side:?} buffer of size {buffer_size}"
    )]
    BufferEndOffsetOverrun {
        start_offset: BufferAddress,
        size: BufferAddress,
        buffer_size: BufferAddress,
        side: CopySide,
    },
    #[error("Copy of {dimension:?} {start_offset}..{end_offset} would end up overrunning the bounds of the {side:?} texture of {dimension:?} size {texture_size}")]
    TextureOverrun {
        start_offset: u32,
        end_offset: u32,
        texture_size: u32,
        dimension: TextureErrorDimension,
        side: CopySide,
    },
    #[error("Partial copy of {start_offset}..{end_offset} on {dimension:?} dimension with size {texture_size} \
             is not supported for the {side:?} texture format {format:?} with {sample_count} samples")]
    UnsupportedPartialTransfer {
        format: wgt::TextureFormat,
        sample_count: u32,
        start_offset: u32,
        end_offset: u32,
        texture_size: u32,
        dimension: TextureErrorDimension,
        side: CopySide,
    },
    #[error(
        "Copying{} layers {}..{} to{} layers {}..{} of the same texture is not allowed",
        if *src_aspects == wgt::TextureAspect::All { String::new() } else { format!(" {src_aspects:?}") },
        src_origin_z,
        src_origin_z + array_layer_count,
        if *dst_aspects == wgt::TextureAspect::All { String::new() } else { format!(" {dst_aspects:?}") },
        dst_origin_z,
        dst_origin_z + array_layer_count,
    )]
    InvalidCopyWithinSameTexture {
        src_aspects: wgt::TextureAspect,
        dst_aspects: wgt::TextureAspect,
        src_origin_z: u32,
        dst_origin_z: u32,
        array_layer_count: u32,
    },
    #[error("Unable to select texture aspect {aspect:?} from format {format:?}")]
    InvalidTextureAspect {
        format: wgt::TextureFormat,
        aspect: wgt::TextureAspect,
    },
    #[error("Unable to select texture mip level {level} out of {total}")]
    InvalidTextureMipLevel { level: u32, total: u32 },
    #[error("Texture dimension must be 2D when copying from an external texture")]
    InvalidDimensionExternal,
    #[error("Buffer offset {0} is not aligned to block size or `COPY_BUFFER_ALIGNMENT`")]
    UnalignedBufferOffset(BufferAddress),
    #[error("Copy size {0} does not respect `COPY_BUFFER_ALIGNMENT`")]
    UnalignedCopySize(BufferAddress),
    #[error("Copy width is not a multiple of block width")]
    UnalignedCopyWidth,
    #[error("Copy height is not a multiple of block height")]
    UnalignedCopyHeight,
    #[error("Copy origin's x component is not a multiple of block width")]
    UnalignedCopyOriginX,
    #[error("Copy origin's y component is not a multiple of block height")]
    UnalignedCopyOriginY,
    #[error("Bytes per row does not respect `COPY_BYTES_PER_ROW_ALIGNMENT`")]
    UnalignedBytesPerRow,
    #[error("Number of bytes per row needs to be specified since more than one row is copied")]
    UnspecifiedBytesPerRow,
    #[error("Number of rows per image needs to be specified since more than one image is copied")]
    UnspecifiedRowsPerImage,
    #[error("Number of bytes per row is less than the number of bytes in a complete row")]
    InvalidBytesPerRow,
    #[error("Number of rows per image is invalid")]
    InvalidRowsPerImage,
    #[error("Overflow while computing the size of the copy")]
    SizeOverflow,
    #[error("Copy source aspects must refer to all aspects of the source texture format")]
    CopySrcMissingAspects,
    #[error(
        "Copy destination aspects must refer to all aspects of the destination texture format"
    )]
    CopyDstMissingAspects,
    #[error("Copy aspect must refer to a single aspect of texture format")]
    CopyAspectNotOne,
    #[error("Copying from textures with format {0:?} is forbidden")]
    CopyFromForbiddenTextureFormat(wgt::TextureFormat),
    #[error("Copying from textures with format {format:?} and aspect {aspect:?} is forbidden")]
    CopyFromForbiddenTextureFormatAspect {
        format: wgt::TextureFormat,
        aspect: wgt::TextureAspect,
    },
    #[error("Copying to textures with format {0:?} is forbidden")]
    CopyToForbiddenTextureFormat(wgt::TextureFormat),
    #[error("Copying to textures with format {format:?} and aspect {aspect:?} is forbidden")]
    CopyToForbiddenTextureFormatAspect {
        format: wgt::TextureFormat,
        aspect: wgt::TextureAspect,
    },
    #[error(
        "Copying to textures with format {0:?} is forbidden when copying from external texture"
    )]
    ExternalCopyToForbiddenTextureFormat(wgt::TextureFormat),
    #[error(
        "Source format ({src_format:?}) and destination format ({dst_format:?}) are not copy-compatible (they may only differ in srgb-ness)"
    )]
    TextureFormatsNotCopyCompatible {
        src_format: wgt::TextureFormat,
        dst_format: wgt::TextureFormat,
    },
    #[error(transparent)]
    MemoryInitFailure(#[from] ClearError),
    #[error("Cannot encode this copy because of a missing downelevel flag")]
    MissingDownlevelFlags(#[from] MissingDownlevelFlags),
    #[error("Source texture sample count must be 1, got {sample_count}")]
    InvalidSampleCount { sample_count: u32 },
    #[error(
        "Source sample count ({src_sample_count:?}) and destination sample count ({dst_sample_count:?}) are not equal"
    )]
    SampleCountNotEqual {
        src_sample_count: u32,
        dst_sample_count: u32,
    },
    #[error("Requested mip level {requested} does not exist (count: {count})")]
    InvalidMipLevel { requested: u32, count: u32 },
    #[error("Buffer is expected to be unmapped, but was not")]
    BufferNotAvailable,
}

impl WebGpuError for TransferError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::MissingBufferUsage(e) => e.webgpu_error_type(),
            Self::MissingTextureUsage(e) => e.webgpu_error_type(),
            Self::MemoryInitFailure(e) => e.webgpu_error_type(),

            Self::BufferEndOffsetOverrun { .. }
            | Self::TextureOverrun { .. }
            | Self::BufferStartOffsetOverrun { .. }
            | Self::UnsupportedPartialTransfer { .. }
            | Self::InvalidCopyWithinSameTexture { .. }
            | Self::InvalidTextureAspect { .. }
            | Self::InvalidTextureMipLevel { .. }
            | Self::InvalidDimensionExternal
            | Self::UnalignedBufferOffset(..)
            | Self::UnalignedCopySize(..)
            | Self::UnalignedCopyWidth
            | Self::UnalignedCopyHeight
            | Self::UnalignedCopyOriginX
            | Self::UnalignedCopyOriginY
            | Self::UnalignedBytesPerRow
            | Self::UnspecifiedBytesPerRow
            | Self::UnspecifiedRowsPerImage
            | Self::InvalidBytesPerRow
            | Self::InvalidRowsPerImage
            | Self::SizeOverflow
            | Self::CopySrcMissingAspects
            | Self::CopyDstMissingAspects
            | Self::CopyAspectNotOne
            | Self::CopyFromForbiddenTextureFormat(..)
            | Self::CopyFromForbiddenTextureFormatAspect { .. }
            | Self::CopyToForbiddenTextureFormat(..)
            | Self::CopyToForbiddenTextureFormatAspect { .. }
            | Self::ExternalCopyToForbiddenTextureFormat(..)
            | Self::TextureFormatsNotCopyCompatible { .. }
            | Self::MissingDownlevelFlags(..)
            | Self::InvalidSampleCount { .. }
            | Self::SampleCountNotEqual { .. }
            | Self::InvalidMipLevel { .. }
            | Self::SameSourceDestinationBuffer
            | Self::BufferNotAvailable => ErrorType::Validation,
        }
    }
}

impl From<BufferTextureCopyInfoError> for TransferError {
    fn from(value: BufferTextureCopyInfoError) -> Self {
        match value {
            BufferTextureCopyInfoError::InvalidBytesPerRow => Self::InvalidBytesPerRow,
            BufferTextureCopyInfoError::InvalidRowsPerImage => Self::InvalidRowsPerImage,
            BufferTextureCopyInfoError::ImageStrideOverflow
            | BufferTextureCopyInfoError::ImageBytesOverflow(_)
            | BufferTextureCopyInfoError::ArraySizeOverflow(_) => Self::SizeOverflow,
        }
    }
}

pub(crate) fn extract_texture_selector<T>(
    copy_texture: &wgt::TexelCopyTextureInfo<T>,
    copy_size: &Extent3d,
    texture: &Texture,
) -> Result<(TextureSelector, hal::TextureCopyBase), TransferError> {
    let format = texture.desc.format;
    let copy_aspect = hal::FormatAspects::new(format, copy_texture.aspect);
    if copy_aspect.is_empty() {
        return Err(TransferError::InvalidTextureAspect {
            format,
            aspect: copy_texture.aspect,
        });
    }

    let (layers, origin_z) = match texture.desc.dimension {
        wgt::TextureDimension::D1 => (0..1, 0),
        wgt::TextureDimension::D2 => (
            copy_texture.origin.z..copy_texture.origin.z + copy_size.depth_or_array_layers,
            0,
        ),
        wgt::TextureDimension::D3 => (0..1, copy_texture.origin.z),
    };
    let base = hal::TextureCopyBase {
        origin: wgt::Origin3d {
            x: copy_texture.origin.x,
            y: copy_texture.origin.y,
            z: origin_z,
        },
        // this value will be incremented per copied layer
        array_layer: layers.start,
        mip_level: copy_texture.mip_level,
        aspect: copy_aspect,
    };
    let selector = TextureSelector {
        mips: copy_texture.mip_level..copy_texture.mip_level + 1,
        layers,
    };

    Ok((selector, base))
}

/// WebGPU's [validating linear texture data][vltd] algorithm.
///
/// Copied with some modifications from WebGPU standard.
///
/// If successful, returns a tuple `(bytes, stride, is_contiguous)`, where:
/// - `bytes` is the number of buffer bytes required for this copy, and
/// - `stride` number of bytes between array layers.
/// - `is_contiguous` is true if the linear texture data does not have padding
///   between rows or between images.
///
/// [vltd]: https://gpuweb.github.io/gpuweb/#abstract-opdef-validating-linear-texture-data
pub(crate) fn validate_linear_texture_data(
    layout: &wgt::TexelCopyBufferLayout,
    format: wgt::TextureFormat,
    aspect: wgt::TextureAspect,
    buffer_size: BufferAddress,
    buffer_side: CopySide,
    copy_size: &Extent3d,
) -> Result<(BufferAddress, BufferAddress, bool), TransferError> {
    let wgt::BufferTextureCopyInfo {
        copy_width,
        copy_height,
        depth_or_array_layers,

        offset,

        block_size_bytes: _,
        block_width_texels,
        block_height_texels,

        width_blocks: _,
        height_blocks,

        row_bytes_dense,
        row_stride_bytes,

        image_stride_rows: _,
        image_stride_bytes,

        image_rows_dense: _,
        image_bytes_dense,

        bytes_in_copy,
    } = layout.get_buffer_texture_copy_info(format, aspect, copy_size)?;

    if !copy_width.is_multiple_of(block_width_texels) {
        return Err(TransferError::UnalignedCopyWidth);
    }
    if !copy_height.is_multiple_of(block_height_texels) {
        return Err(TransferError::UnalignedCopyHeight);
    }

    let requires_multiple_rows = depth_or_array_layers > 1 || height_blocks > 1;
    let requires_multiple_images = depth_or_array_layers > 1;

    // `get_buffer_texture_copy_info()` already proceeded with defaults if these
    // were not specified, and ensured that the values satisfy the minima if
    // they were, but now we enforce the WebGPU requirement that they be
    // specified any time they apply.
    if layout.bytes_per_row.is_none() && requires_multiple_rows {
        return Err(TransferError::UnspecifiedBytesPerRow);
    }

    if layout.rows_per_image.is_none() && requires_multiple_images {
        return Err(TransferError::UnspecifiedRowsPerImage);
    };

    if offset > buffer_size {
        return Err(TransferError::BufferStartOffsetOverrun {
            start_offset: offset,
            buffer_size,
            side: buffer_side,
        });
    }
    // NOTE: Should never underflow because of our earlier check.
    if bytes_in_copy > buffer_size - offset {
        return Err(TransferError::BufferEndOffsetOverrun {
            start_offset: offset,
            size: bytes_in_copy,
            buffer_size,
            side: buffer_side,
        });
    }

    let is_contiguous = (row_stride_bytes == row_bytes_dense || !requires_multiple_rows)
        && (image_stride_bytes == image_bytes_dense || !requires_multiple_images);

    Ok((bytes_in_copy, image_stride_bytes, is_contiguous))
}

/// Validate the source format of a texture copy.
///
/// This performs the check from WebGPU's [validating texture buffer copy][vtbc]
/// algorithm that ensures that the format and aspect form a valid texel copy source
/// as defined in the [depth-stencil formats][dsf].
///
/// [vtbc]: https://gpuweb.github.io/gpuweb/#abstract-opdef-validating-texture-buffer-copy
/// [dsf]: https://gpuweb.github.io/gpuweb/#depth-formats
pub(crate) fn validate_texture_copy_src_format(
    format: wgt::TextureFormat,
    aspect: wgt::TextureAspect,
) -> Result<(), TransferError> {
    use wgt::TextureAspect as Ta;
    use wgt::TextureFormat as Tf;
    match (format, aspect) {
        (Tf::Depth24Plus, _) => Err(TransferError::CopyFromForbiddenTextureFormat(format)),
        (Tf::Depth24PlusStencil8, Ta::DepthOnly) => {
            Err(TransferError::CopyFromForbiddenTextureFormatAspect { format, aspect })
        }
        _ => Ok(()),
    }
}

/// Validate the destination format of a texture copy.
///
/// This performs the check from WebGPU's [validating texture buffer copy][vtbc]
/// algorithm that ensures that the format and aspect form a valid texel copy destination
/// as defined in the [depth-stencil formats][dsf].
///
/// [vtbc]: https://gpuweb.github.io/gpuweb/#abstract-opdef-validating-texture-buffer-copy
/// [dsf]: https://gpuweb.github.io/gpuweb/#depth-formats
pub(crate) fn validate_texture_copy_dst_format(
    format: wgt::TextureFormat,
    aspect: wgt::TextureAspect,
) -> Result<(), TransferError> {
    use wgt::TextureAspect as Ta;
    use wgt::TextureFormat as Tf;
    match (format, aspect) {
        (Tf::Depth24Plus | Tf::Depth32Float, _) => {
            Err(TransferError::CopyToForbiddenTextureFormat(format))
        }
        (Tf::Depth24PlusStencil8 | Tf::Depth32FloatStencil8, Ta::DepthOnly) => {
            Err(TransferError::CopyToForbiddenTextureFormatAspect { format, aspect })
        }
        _ => Ok(()),
    }
}

/// Validation for texture/buffer copies.
///
/// This implements the following checks from WebGPU's [validating texture buffer copy][vtbc]
/// algorithm:
///  * The texture must not be multisampled.
///  * The copy must be from/to a single aspect of the texture.
///  * If `aligned` is true, the buffer offset must be aligned appropriately.
///
/// And implements the following check from WebGPU's [validating GPUTexelCopyBufferInfo][vtcbi]
/// algorithm:
///  * If `aligned` is true, `bytesPerRow` must be a multiple of 256.
///
/// Note that the `bytesPerRow` alignment check is enforced whenever
/// `bytesPerRow` is specified, even if the transfer is not multiple rows and
/// `bytesPerRow` could have been omitted.
///
/// The following steps in [validating texture buffer copy][vtbc] are implemented elsewhere:
///  * Invocation of other validation algorithms.
///  * The texture usage (COPY_DST / COPY_SRC) check.
///  * The check for non-copyable depth/stencil formats. The caller must perform
///    this check using `validate_texture_copy_src_format` / `validate_texture_copy_dst_format`
///    before calling this function. This function will panic if
///    [`wgt::TextureFormat::block_copy_size`] returns `None` due to a
///    non-copyable format.
///
/// [vtbc]: https://gpuweb.github.io/gpuweb/#abstract-opdef-validating-texture-buffer-copy
/// [vtcbi]: https://www.w3.org/TR/webgpu/#abstract-opdef-validating-gputexelcopybufferinfo
pub(crate) fn validate_texture_buffer_copy<T>(
    texture_copy_view: &wgt::TexelCopyTextureInfo<T>,
    aspect: hal::FormatAspects,
    desc: &wgt::TextureDescriptor<(), Vec<wgt::TextureFormat>>,
    layout: &wgt::TexelCopyBufferLayout,
    aligned: bool,
) -> Result<(), TransferError> {
    if desc.sample_count != 1 {
        return Err(TransferError::InvalidSampleCount {
            sample_count: desc.sample_count,
        });
    }

    if !aspect.is_one() {
        return Err(TransferError::CopyAspectNotOne);
    }

    let offset_alignment = if desc.format.is_depth_stencil_format() {
        4
    } else {
        // The case where `block_copy_size` returns `None` is currently
        // unreachable both for the reason in the expect message, and also
        // because the currently-defined non-copyable formats are depth/stencil
        // formats so would take the `if` branch.
        desc.format
            .block_copy_size(Some(texture_copy_view.aspect))
            .expect("non-copyable formats should have been rejected previously")
    };

    if aligned && !layout.offset.is_multiple_of(u64::from(offset_alignment)) {
        return Err(TransferError::UnalignedBufferOffset(layout.offset));
    }

    if let Some(bytes_per_row) = layout.bytes_per_row {
        if aligned && bytes_per_row % wgt::COPY_BYTES_PER_ROW_ALIGNMENT != 0 {
            return Err(TransferError::UnalignedBytesPerRow);
        }
    }

    Ok(())
}

/// Validate the extent and alignment of a texture copy.
///
/// Copied with minor modifications from WebGPU standard. This mostly follows
/// the [validating GPUTexelCopyTextureInfo][vtcti] and [validating texture copy
/// range][vtcr] algorithms.
///
/// Returns the HAL copy extent and the layer count.
///
/// [vtcti]: https://gpuweb.github.io/gpuweb/#abstract-opdef-validating-gputexelcopytextureinfo
/// [vtcr]: https://gpuweb.github.io/gpuweb/#abstract-opdef-validating-texture-copy-range
pub(crate) fn validate_texture_copy_range<T>(
    texture_copy_view: &wgt::TexelCopyTextureInfo<T>,
    desc: &wgt::TextureDescriptor<(), Vec<wgt::TextureFormat>>,
    texture_side: CopySide,
    copy_size: &Extent3d,
) -> Result<(hal::CopyExtent, u32), TransferError> {
    let (block_width, block_height) = desc.format.block_dimensions();

    let extent_virtual = desc.mip_level_size(texture_copy_view.mip_level).ok_or(
        TransferError::InvalidTextureMipLevel {
            level: texture_copy_view.mip_level,
            total: desc.mip_level_count,
        },
    )?;
    // physical size can be larger than the virtual
    let extent = extent_virtual.physical_size(desc.format);

    // Multisampled and depth-stencil formats do not support partial copies
    // on x and y dimensions, but do support copying a subset of layers.
    let requires_exact_size = desc.format.is_depth_stencil_format() || desc.sample_count > 1;

    // Return `Ok` if a run `size` texels long starting at `start_offset` is
    // valid for `texture_size`. Otherwise, return an appropriate a`Err`.
    let check_dimension = |dimension: TextureErrorDimension,
                           start_offset: u32,
                           size: u32,
                           texture_size: u32,
                           requires_exact_size: bool|
     -> Result<(), TransferError> {
        if requires_exact_size && (start_offset != 0 || size != texture_size) {
            Err(TransferError::UnsupportedPartialTransfer {
                format: desc.format,
                sample_count: desc.sample_count,
                start_offset,
                end_offset: start_offset.wrapping_add(size),
                texture_size,
                dimension,
                side: texture_side,
            })
        // Avoid underflow in the subtraction by checking start_offset against
        // texture_size first.
        } else if start_offset > texture_size || texture_size - start_offset < size {
            Err(TransferError::TextureOverrun {
                start_offset,
                end_offset: start_offset.wrapping_add(size),
                texture_size,
                dimension,
                side: texture_side,
            })
        } else {
            Ok(())
        }
    };

    check_dimension(
        TextureErrorDimension::X,
        texture_copy_view.origin.x,
        copy_size.width,
        extent.width,
        requires_exact_size,
    )?;
    check_dimension(
        TextureErrorDimension::Y,
        texture_copy_view.origin.y,
        copy_size.height,
        extent.height,
        requires_exact_size,
    )?;
    check_dimension(
        TextureErrorDimension::Z,
        texture_copy_view.origin.z,
        copy_size.depth_or_array_layers,
        extent.depth_or_array_layers,
        false, // partial copy always allowed on Z/layer dimension
    )?;

    if !texture_copy_view.origin.x.is_multiple_of(block_width) {
        return Err(TransferError::UnalignedCopyOriginX);
    }
    if !texture_copy_view.origin.y.is_multiple_of(block_height) {
        return Err(TransferError::UnalignedCopyOriginY);
    }
    if !copy_size.width.is_multiple_of(block_width) {
        return Err(TransferError::UnalignedCopyWidth);
    }
    if !copy_size.height.is_multiple_of(block_height) {
        return Err(TransferError::UnalignedCopyHeight);
    }

    let (depth, array_layer_count) = match desc.dimension {
        wgt::TextureDimension::D1 => (1, 1),
        wgt::TextureDimension::D2 => (1, copy_size.depth_or_array_layers),
        wgt::TextureDimension::D3 => (copy_size.depth_or_array_layers, 1),
    };

    let copy_extent = hal::CopyExtent {
        width: copy_size.width,
        height: copy_size.height,
        depth,
    };
    Ok((copy_extent, array_layer_count))
}

/// Validate a copy within the same texture.
///
/// This implements the WebGPU requirement that the [sets of subresources for
/// texture copy][srtc] of the source and destination be disjoint, i.e. that the
/// source and destination do not overlap.
///
/// This function assumes that the copy ranges have already been validated with
/// `validate_texture_copy_range`.
///
/// [srtc]: https://gpuweb.github.io/gpuweb/#abstract-opdef-set-of-subresources-for-texture-copy
pub(crate) fn validate_copy_within_same_texture<T>(
    src: &wgt::TexelCopyTextureInfo<T>,
    dst: &wgt::TexelCopyTextureInfo<T>,
    format: wgt::TextureFormat,
    array_layer_count: u32,
) -> Result<(), TransferError> {
    let src_aspects = hal::FormatAspects::new(format, src.aspect);
    let dst_aspects = hal::FormatAspects::new(format, dst.aspect);
    if (src_aspects & dst_aspects).is_empty() {
        // Copying between different aspects (if it even makes sense), is okay.
        return Ok(());
    }

    if src.origin.z >= dst.origin.z + array_layer_count
        || dst.origin.z >= src.origin.z + array_layer_count
    {
        // Copying between non-overlapping layer ranges is okay.
        return Ok(());
    }

    if src.mip_level != dst.mip_level {
        // Copying between different mip levels is okay.
        return Ok(());
    }

    Err(TransferError::InvalidCopyWithinSameTexture {
        src_aspects: src.aspect,
        dst_aspects: dst.aspect,
        src_origin_z: src.origin.z,
        dst_origin_z: dst.origin.z,
        array_layer_count,
    })
}

fn handle_texture_init(
    state: &mut EncodingState,
    init_kind: MemoryInitKind,
    copy_texture: &TexelCopyTextureInfo,
    copy_size: &Extent3d,
    texture: &Arc<Texture>,
) -> Result<(), ClearError> {
    let init_action = TextureInitTrackerAction {
        texture: texture.clone(),
        range: TextureInitRange {
            mip_range: copy_texture.mip_level..copy_texture.mip_level + 1,
            layer_range: copy_texture.origin.z
                ..(copy_texture.origin.z + copy_size.depth_or_array_layers),
        },
        kind: init_kind,
    };

    // Register the init action.
    let immediate_inits = state
        .texture_memory_actions
        .register_init_action(&{ init_action });

    // In rare cases we may need to insert an init operation immediately onto the command buffer.
    if !immediate_inits.is_empty() {
        for init in immediate_inits {
            clear_texture(
                &init.texture,
                TextureInitRange {
                    mip_range: init.mip_level..(init.mip_level + 1),
                    layer_range: init.layer..(init.layer + 1),
                },
                state.raw_encoder,
                &mut state.tracker.textures,
                &state.device.alignments,
                state.device.zero_buffer.as_ref(),
                state.snatch_guard,
                state.device.instance_flags,
            )?;
        }
    }

    Ok(())
}

/// Prepare a transfer's source texture.
///
/// Ensure the source texture of a transfer is in the right initialization
/// state, and record the state for after the transfer operation.
fn handle_src_texture_init(
    state: &mut EncodingState,
    source: &TexelCopyTextureInfo,
    copy_size: &Extent3d,
    texture: &Arc<Texture>,
) -> Result<(), TransferError> {
    handle_texture_init(
        state,
        MemoryInitKind::NeedsInitializedMemory,
        source,
        copy_size,
        texture,
    )?;
    Ok(())
}

/// Prepare a transfer's destination texture.
///
/// Ensure the destination texture of a transfer is in the right initialization
/// state, and record the state for after the transfer operation.
fn handle_dst_texture_init(
    state: &mut EncodingState,
    destination: &wgt::TexelCopyTextureInfo<Arc<Texture>>,
    copy_size: &Extent3d,
    texture: &Arc<Texture>,
) -> Result<(), TransferError> {
    // Attention: If we don't write full texture subresources, we need to a full
    // clear first since we don't track subrects. This means that in rare cases
    // even a *destination* texture of a transfer may need an immediate texture
    // init.
    let dst_init_kind = if has_copy_partial_init_tracker_coverage(
        copy_size,
        destination.mip_level,
        &texture.desc,
    ) {
        MemoryInitKind::NeedsInitializedMemory
    } else {
        MemoryInitKind::ImplicitlyInitialized
    };

    handle_texture_init(state, dst_init_kind, destination, copy_size, texture)?;
    Ok(())
}

/// Handle initialization tracking for a transfer's source or destination buffer.
///
/// Ensures that the transfer will not read from uninitialized memory, and updates
/// the initialization state information to reflect the transfer.
fn handle_buffer_init(
    state: &mut EncodingState,
    info: &wgt::TexelCopyBufferInfo<Arc<Buffer>>,
    direction: CopySide,
    required_buffer_bytes_in_copy: BufferAddress,
    is_contiguous: bool,
) {
    const ALIGN_SIZE: BufferAddress = wgt::COPY_BUFFER_ALIGNMENT;
    const ALIGN_MASK: BufferAddress = wgt::COPY_BUFFER_ALIGNMENT - 1;

    let buffer = &info.buffer;
    let start = info.layout.offset;
    let end = info.layout.offset + required_buffer_bytes_in_copy;
    if !is_contiguous || direction == CopySide::Source {
        // If the transfer will read the buffer, then the whole region needs to
        // be initialized.
        //
        // If the transfer will not write a contiguous region of the buffer,
        // then we need to make sure the padding areas are initialized. For now,
        // initialize the whole region, although this could be improved to
        // initialize only the necessary parts if doing so is likely to be
        // faster than initializing the whole thing.
        //
        // Adjust the start/end outwards to 4B alignment.
        let aligned_start = start & !ALIGN_MASK;
        let aligned_end = (end + ALIGN_MASK) & !ALIGN_MASK;
        state
            .buffer_memory_init_actions
            .extend(buffer.initialization_status.read().create_action(
                buffer,
                aligned_start..aligned_end,
                MemoryInitKind::NeedsInitializedMemory,
            ));
    } else {
        // If the transfer will write a contiguous region of the buffer, then we
        // don't need to initialize that region.
        //
        // However, if the start and end are not 4B aligned, we need to make
        // sure that we don't end up trying to initialize non-4B-aligned regions
        // later.
        //
        // Adjust the start/end inwards to 4B alignment, we will handle the
        // first/last pieces differently.
        let aligned_start = (start + ALIGN_MASK) & !ALIGN_MASK;
        let aligned_end = end & !ALIGN_MASK;
        if aligned_start != start {
            state.buffer_memory_init_actions.extend(
                buffer.initialization_status.read().create_action(
                    buffer,
                    aligned_start - ALIGN_SIZE..aligned_start,
                    MemoryInitKind::NeedsInitializedMemory,
                ),
            );
        }
        if aligned_start != aligned_end {
            state.buffer_memory_init_actions.extend(
                buffer.initialization_status.read().create_action(
                    buffer,
                    aligned_start..aligned_end,
                    MemoryInitKind::ImplicitlyInitialized,
                ),
            );
        }
        if aligned_end != end {
            // It is possible that `aligned_end + ALIGN_SIZE > dst_buffer.size`,
            // because `dst_buffer.size` is the user-requested size, not the
            // final size of the buffer. The final size of the buffer is not
            // readily available, but was rounded up to COPY_BUFFER_ALIGNMENT,
            // so no overrun is possible.
            state.buffer_memory_init_actions.extend(
                buffer.initialization_status.read().create_action(
                    buffer,
                    aligned_end..aligned_end + ALIGN_SIZE,
                    MemoryInitKind::NeedsInitializedMemory,
                ),
            );
        }
    }
}

impl Global {
    pub fn command_encoder_copy_buffer_to_buffer(
        &self,
        command_encoder_id: CommandEncoderId,
        source: BufferId,
        source_offset: BufferAddress,
        destination: BufferId,
        destination_offset: BufferAddress,
        size: Option<BufferAddress>,
    ) -> Result<(), EncoderStateError> {
        profiling::scope!("CommandEncoder::copy_buffer_to_buffer");
        api_log!(
            "CommandEncoder::copy_buffer_to_buffer {source:?} -> {destination:?} {size:?}bytes"
        );

        let hub = &self.hub;

        let cmd_enc = hub.command_encoders.get(command_encoder_id);
        let mut cmd_buf_data = cmd_enc.data.lock();

        cmd_buf_data.push_with(|| -> Result<_, CommandEncoderError> {
            Ok(ArcCommand::CopyBufferToBuffer {
                src: self.resolve_buffer_id(source)?,
                src_offset: source_offset,
                dst: self.resolve_buffer_id(destination)?,
                dst_offset: destination_offset,
                size,
            })
        })
    }

    pub fn command_encoder_copy_buffer_to_texture(
        &self,
        command_encoder_id: CommandEncoderId,
        source: &TexelCopyBufferInfo,
        destination: &wgt::TexelCopyTextureInfo<TextureId>,
        copy_size: &Extent3d,
    ) -> Result<(), EncoderStateError> {
        profiling::scope!("CommandEncoder::copy_buffer_to_texture");
        api_log!(
            "CommandEncoder::copy_buffer_to_texture {:?} -> {:?} {copy_size:?}",
            source.buffer,
            destination.texture
        );

        let cmd_enc = self.hub.command_encoders.get(command_encoder_id);
        let mut cmd_buf_data = cmd_enc.data.lock();

        cmd_buf_data.push_with(|| -> Result<_, CommandEncoderError> {
            Ok(ArcCommand::CopyBufferToTexture {
                src: wgt::TexelCopyBufferInfo::<Arc<Buffer>> {
                    buffer: self.resolve_buffer_id(source.buffer)?,
                    layout: source.layout,
                },
                dst: wgt::TexelCopyTextureInfo::<Arc<Texture>> {
                    texture: self.resolve_texture_id(destination.texture)?,
                    mip_level: destination.mip_level,
                    origin: destination.origin,
                    aspect: destination.aspect,
                },
                size: *copy_size,
            })
        })
    }

    pub fn command_encoder_copy_texture_to_buffer(
        &self,
        command_encoder_id: CommandEncoderId,
        source: &wgt::TexelCopyTextureInfo<TextureId>,
        destination: &TexelCopyBufferInfo,
        copy_size: &Extent3d,
    ) -> Result<(), EncoderStateError> {
        profiling::scope!("CommandEncoder::copy_texture_to_buffer");
        api_log!(
            "CommandEncoder::copy_texture_to_buffer {:?} -> {:?} {copy_size:?}",
            source.texture,
            destination.buffer
        );

        let cmd_enc = self.hub.command_encoders.get(command_encoder_id);
        let mut cmd_buf_data = cmd_enc.data.lock();

        cmd_buf_data.push_with(|| -> Result<_, CommandEncoderError> {
            Ok(ArcCommand::CopyTextureToBuffer {
                src: wgt::TexelCopyTextureInfo::<Arc<Texture>> {
                    texture: self.resolve_texture_id(source.texture)?,
                    mip_level: source.mip_level,
                    origin: source.origin,
                    aspect: source.aspect,
                },
                dst: wgt::TexelCopyBufferInfo::<Arc<Buffer>> {
                    buffer: self.resolve_buffer_id(destination.buffer)?,
                    layout: destination.layout,
                },
                size: *copy_size,
            })
        })
    }

    pub fn command_encoder_copy_texture_to_texture(
        &self,
        command_encoder_id: CommandEncoderId,
        source: &wgt::TexelCopyTextureInfo<TextureId>,
        destination: &wgt::TexelCopyTextureInfo<TextureId>,
        copy_size: &Extent3d,
    ) -> Result<(), EncoderStateError> {
        profiling::scope!("CommandEncoder::copy_texture_to_texture");
        api_log!(
            "CommandEncoder::copy_texture_to_texture {:?} -> {:?} {copy_size:?}",
            source.texture,
            destination.texture
        );

        let cmd_enc = self.hub.command_encoders.get(command_encoder_id);
        let mut cmd_buf_data = cmd_enc.data.lock();

        cmd_buf_data.push_with(|| -> Result<_, CommandEncoderError> {
            Ok(ArcCommand::CopyTextureToTexture {
                src: wgt::TexelCopyTextureInfo {
                    texture: self.resolve_texture_id(source.texture)?,
                    mip_level: source.mip_level,
                    origin: source.origin,
                    aspect: source.aspect,
                },
                dst: wgt::TexelCopyTextureInfo {
                    texture: self.resolve_texture_id(destination.texture)?,
                    mip_level: destination.mip_level,
                    origin: destination.origin,
                    aspect: destination.aspect,
                },
                size: *copy_size,
            })
        })
    }
}

pub(super) fn copy_buffer_to_buffer(
    state: &mut EncodingState,
    src_buffer: &Arc<Buffer>,
    source_offset: BufferAddress,
    dst_buffer: &Arc<Buffer>,
    destination_offset: BufferAddress,
    size: Option<BufferAddress>,
) -> Result<(), CommandEncoderError> {
    if src_buffer.is_equal(dst_buffer) {
        return Err(TransferError::SameSourceDestinationBuffer.into());
    }

    src_buffer.same_device(state.device)?;

    let src_pending = state
        .tracker
        .buffers
        .set_single(src_buffer, wgt::BufferUses::COPY_SRC);

    let src_raw = src_buffer.try_raw(state.snatch_guard)?;
    src_buffer
        .check_usage(BufferUsages::COPY_SRC)
        .map_err(TransferError::MissingBufferUsage)?;
    // expecting only a single barrier
    let src_barrier = src_pending.map(|pending| pending.into_hal(src_buffer, state.snatch_guard));

    dst_buffer.same_device(state.device)?;

    let dst_pending = state
        .tracker
        .buffers
        .set_single(dst_buffer, wgt::BufferUses::COPY_DST);

    let dst_raw = dst_buffer.try_raw(state.snatch_guard)?;
    dst_buffer
        .check_usage(BufferUsages::COPY_DST)
        .map_err(TransferError::MissingBufferUsage)?;
    let dst_barrier = dst_pending.map(|pending| pending.into_hal(dst_buffer, state.snatch_guard));

    if source_offset > src_buffer.size {
        return Err(TransferError::BufferStartOffsetOverrun {
            start_offset: source_offset,
            buffer_size: src_buffer.size,
            side: CopySide::Source,
        }
        .into());
    }
    let size = size.unwrap_or_else(|| {
        // NOTE: Should never underflow because of our earlier check.
        src_buffer.size - source_offset
    });

    if !size.is_multiple_of(wgt::COPY_BUFFER_ALIGNMENT) {
        return Err(TransferError::UnalignedCopySize(size).into());
    }
    if !source_offset.is_multiple_of(wgt::COPY_BUFFER_ALIGNMENT) {
        return Err(TransferError::UnalignedBufferOffset(source_offset).into());
    }
    if !destination_offset.is_multiple_of(wgt::COPY_BUFFER_ALIGNMENT) {
        return Err(TransferError::UnalignedBufferOffset(destination_offset).into());
    }
    if !state
        .device
        .downlevel
        .flags
        .contains(wgt::DownlevelFlags::UNRESTRICTED_INDEX_BUFFER)
        && (src_buffer.usage.contains(BufferUsages::INDEX)
            || dst_buffer.usage.contains(BufferUsages::INDEX))
    {
        let forbidden_usages = BufferUsages::VERTEX
            | BufferUsages::UNIFORM
            | BufferUsages::INDIRECT
            | BufferUsages::STORAGE;
        if src_buffer.usage.intersects(forbidden_usages)
            || dst_buffer.usage.intersects(forbidden_usages)
        {
            return Err(TransferError::MissingDownlevelFlags(MissingDownlevelFlags(
                wgt::DownlevelFlags::UNRESTRICTED_INDEX_BUFFER,
            ))
            .into());
        }
    }

    if size > src_buffer.size - source_offset {
        return Err(TransferError::BufferEndOffsetOverrun {
            start_offset: source_offset,
            size,
            buffer_size: src_buffer.size,
            side: CopySide::Source,
        }
        .into());
    }
    // NOTE: Should never overflow because of our earlier check.
    let source_end_offset = source_offset + size;

    if destination_offset > dst_buffer.size {
        return Err(TransferError::BufferStartOffsetOverrun {
            start_offset: destination_offset,
            buffer_size: dst_buffer.size,
            side: CopySide::Destination,
        }
        .into());
    }
    // NOTE: Should never underflow because of our earlier check.
    if size > dst_buffer.size - destination_offset {
        return Err(TransferError::BufferEndOffsetOverrun {
            start_offset: destination_offset,
            size,
            buffer_size: dst_buffer.size,
            side: CopySide::Destination,
        }
        .into());
    }
    // NOTE: Should never overflow because of our earlier check.
    let destination_end_offset = destination_offset + size;

    // This must happen after parameter validation (so that errors are reported
    // as required by the spec), but before any side effects.
    if size == 0 {
        log::trace!("Ignoring copy_buffer_to_buffer of size 0");
        return Ok(());
    }

    // Make sure source is initialized memory and mark dest as initialized.
    state
        .buffer_memory_init_actions
        .extend(dst_buffer.initialization_status.read().create_action(
            dst_buffer,
            destination_offset..destination_end_offset,
            MemoryInitKind::ImplicitlyInitialized,
        ));
    state
        .buffer_memory_init_actions
        .extend(src_buffer.initialization_status.read().create_action(
            src_buffer,
            source_offset..source_end_offset,
            MemoryInitKind::NeedsInitializedMemory,
        ));

    let region = hal::BufferCopy {
        src_offset: source_offset,
        dst_offset: destination_offset,
        size: wgt::BufferSize::new(size).unwrap(),
    };
    let barriers = src_barrier
        .into_iter()
        .chain(dst_barrier)
        .collect::<Vec<_>>();
    unsafe {
        state.raw_encoder.transition_buffers(&barriers);
        state
            .raw_encoder
            .copy_buffer_to_buffer(src_raw, dst_raw, &[region]);
    }

    Ok(())
}

pub(super) fn copy_buffer_to_texture(
    state: &mut EncodingState,
    source: &wgt::TexelCopyBufferInfo<Arc<Buffer>>,
    destination: &wgt::TexelCopyTextureInfo<Arc<Texture>>,
    copy_size: &Extent3d,
) -> Result<(), CommandEncoderError> {
    let dst_texture = &destination.texture;
    let src_buffer = &source.buffer;

    dst_texture.same_device(state.device)?;
    src_buffer.same_device(state.device)?;

    let (hal_copy_size, array_layer_count) = validate_texture_copy_range(
        destination,
        &dst_texture.desc,
        CopySide::Destination,
        copy_size,
    )?;

    let (dst_range, dst_base) = extract_texture_selector(destination, copy_size, dst_texture)?;

    let src_raw = src_buffer.try_raw(state.snatch_guard)?;
    src_buffer
        .check_usage(BufferUsages::COPY_SRC)
        .map_err(TransferError::MissingBufferUsage)?;

    let dst_raw = dst_texture.try_raw(state.snatch_guard)?;
    dst_texture
        .check_usage(TextureUsages::COPY_DST)
        .map_err(TransferError::MissingTextureUsage)?;

    validate_texture_copy_dst_format(dst_texture.desc.format, destination.aspect)?;

    validate_texture_buffer_copy(
        destination,
        dst_base.aspect,
        &dst_texture.desc,
        &source.layout,
        true, // alignment required for buffer offset
    )?;

    let (required_buffer_bytes_in_copy, bytes_per_array_layer, is_contiguous) =
        validate_linear_texture_data(
            &source.layout,
            dst_texture.desc.format,
            destination.aspect,
            src_buffer.size,
            CopySide::Source,
            copy_size,
        )?;

    if dst_texture.desc.format.is_depth_stencil_format() {
        state
            .device
            .require_downlevel_flags(wgt::DownlevelFlags::DEPTH_TEXTURE_AND_BUFFER_COPIES)
            .map_err(TransferError::from)?;
    }

    // This must happen after parameter validation (so that errors are reported
    // as required by the spec), but before any side effects.
    if copy_size.width == 0 || copy_size.height == 0 || copy_size.depth_or_array_layers == 0 {
        log::trace!("Ignoring copy_buffer_to_texture of size 0");
        return Ok(());
    }

    // Handle texture init *before* dealing with barrier transitions so we
    // have an easier time inserting "immediate-inits" that may be required
    // by prior discards in rare cases.
    handle_dst_texture_init(state, destination, copy_size, dst_texture)?;

    let src_pending = state
        .tracker
        .buffers
        .set_single(src_buffer, wgt::BufferUses::COPY_SRC);
    let src_barrier = src_pending.map(|pending| pending.into_hal(src_buffer, state.snatch_guard));

    let dst_pending =
        state
            .tracker
            .textures
            .set_single(dst_texture, dst_range, wgt::TextureUses::COPY_DST);
    let dst_barrier = dst_pending
        .map(|pending| pending.into_hal(dst_raw))
        .collect::<Vec<_>>();

    handle_buffer_init(
        state,
        source,
        CopySide::Source,
        required_buffer_bytes_in_copy,
        is_contiguous,
    );

    let regions = (0..array_layer_count)
        .map(|rel_array_layer| {
            let mut texture_base = dst_base.clone();
            texture_base.array_layer += rel_array_layer;
            let mut buffer_layout = source.layout;
            buffer_layout.offset += rel_array_layer as u64 * bytes_per_array_layer;
            hal::BufferTextureCopy {
                buffer_layout,
                texture_base,
                size: hal_copy_size,
            }
        })
        .collect::<Vec<_>>();

    unsafe {
        state.raw_encoder.transition_textures(&dst_barrier);
        state.raw_encoder.transition_buffers(src_barrier.as_slice());
        state
            .raw_encoder
            .copy_buffer_to_texture(src_raw, dst_raw, &regions);
    }

    Ok(())
}

pub(super) fn copy_texture_to_buffer(
    state: &mut EncodingState,
    source: &TexelCopyTextureInfo,
    destination: &wgt::TexelCopyBufferInfo<Arc<Buffer>>,
    copy_size: &Extent3d,
) -> Result<(), CommandEncoderError> {
    let src_texture = &source.texture;
    let dst_buffer = &destination.buffer;

    src_texture.same_device(state.device)?;
    dst_buffer.same_device(state.device)?;

    let (hal_copy_size, array_layer_count) =
        validate_texture_copy_range(source, &src_texture.desc, CopySide::Source, copy_size)?;

    let (src_range, src_base) = extract_texture_selector(source, copy_size, src_texture)?;

    let src_raw = src_texture.try_raw(state.snatch_guard)?;
    src_texture
        .check_usage(TextureUsages::COPY_SRC)
        .map_err(TransferError::MissingTextureUsage)?;

    if source.mip_level >= src_texture.desc.mip_level_count {
        return Err(TransferError::InvalidMipLevel {
            requested: source.mip_level,
            count: src_texture.desc.mip_level_count,
        }
        .into());
    }

    validate_texture_copy_src_format(src_texture.desc.format, source.aspect)?;

    validate_texture_buffer_copy(
        source,
        src_base.aspect,
        &src_texture.desc,
        &destination.layout,
        true, // alignment required for buffer offset
    )?;

    let (required_buffer_bytes_in_copy, bytes_per_array_layer, is_contiguous) =
        validate_linear_texture_data(
            &destination.layout,
            src_texture.desc.format,
            source.aspect,
            dst_buffer.size,
            CopySide::Destination,
            copy_size,
        )?;

    if src_texture.desc.format.is_depth_stencil_format() {
        state
            .device
            .require_downlevel_flags(wgt::DownlevelFlags::DEPTH_TEXTURE_AND_BUFFER_COPIES)
            .map_err(TransferError::from)?;
    }

    let dst_raw = dst_buffer.try_raw(state.snatch_guard)?;
    dst_buffer
        .check_usage(BufferUsages::COPY_DST)
        .map_err(TransferError::MissingBufferUsage)?;

    // This must happen after parameter validation (so that errors are reported
    // as required by the spec), but before any side effects.
    if copy_size.width == 0 || copy_size.height == 0 || copy_size.depth_or_array_layers == 0 {
        log::trace!("Ignoring copy_texture_to_buffer of size 0");
        return Ok(());
    }

    // Handle texture init *before* dealing with barrier transitions so we
    // have an easier time inserting "immediate-inits" that may be required
    // by prior discards in rare cases.
    handle_src_texture_init(state, source, copy_size, src_texture)?;

    let src_pending =
        state
            .tracker
            .textures
            .set_single(src_texture, src_range, wgt::TextureUses::COPY_SRC);
    let src_barrier = src_pending
        .map(|pending| pending.into_hal(src_raw))
        .collect::<Vec<_>>();

    let dst_pending = state
        .tracker
        .buffers
        .set_single(dst_buffer, wgt::BufferUses::COPY_DST);

    let dst_barrier = dst_pending.map(|pending| pending.into_hal(dst_buffer, state.snatch_guard));

    handle_buffer_init(
        state,
        destination,
        CopySide::Destination,
        required_buffer_bytes_in_copy,
        is_contiguous,
    );

    let regions = (0..array_layer_count)
        .map(|rel_array_layer| {
            let mut texture_base = src_base.clone();
            texture_base.array_layer += rel_array_layer;
            let mut buffer_layout = destination.layout;
            buffer_layout.offset += rel_array_layer as u64 * bytes_per_array_layer;
            hal::BufferTextureCopy {
                buffer_layout,
                texture_base,
                size: hal_copy_size,
            }
        })
        .collect::<Vec<_>>();
    unsafe {
        state.raw_encoder.transition_buffers(dst_barrier.as_slice());
        state.raw_encoder.transition_textures(&src_barrier);
        state.raw_encoder.copy_texture_to_buffer(
            src_raw,
            wgt::TextureUses::COPY_SRC,
            dst_raw,
            &regions,
        );
    }

    Ok(())
}

pub(super) fn copy_texture_to_texture(
    state: &mut EncodingState,
    source: &TexelCopyTextureInfo,
    destination: &TexelCopyTextureInfo,
    copy_size: &Extent3d,
) -> Result<(), CommandEncoderError> {
    let src_texture = &source.texture;
    let dst_texture = &destination.texture;

    src_texture.same_device(state.device)?;
    dst_texture.same_device(state.device)?;

    // src and dst texture format must be copy-compatible
    // https://gpuweb.github.io/gpuweb/#copy-compatible
    if src_texture.desc.format.remove_srgb_suffix() != dst_texture.desc.format.remove_srgb_suffix()
    {
        return Err(TransferError::TextureFormatsNotCopyCompatible {
            src_format: src_texture.desc.format,
            dst_format: dst_texture.desc.format,
        }
        .into());
    }

    let (src_copy_size, array_layer_count) =
        validate_texture_copy_range(source, &src_texture.desc, CopySide::Source, copy_size)?;
    let (dst_copy_size, _) = validate_texture_copy_range(
        destination,
        &dst_texture.desc,
        CopySide::Destination,
        copy_size,
    )?;

    if Arc::as_ptr(src_texture) == Arc::as_ptr(dst_texture) {
        validate_copy_within_same_texture(
            source,
            destination,
            src_texture.desc.format,
            array_layer_count,
        )?;
    }

    let (src_range, src_tex_base) = extract_texture_selector(source, copy_size, src_texture)?;
    let (dst_range, dst_tex_base) = extract_texture_selector(destination, copy_size, dst_texture)?;
    let src_texture_aspects = hal::FormatAspects::from(src_texture.desc.format);
    let dst_texture_aspects = hal::FormatAspects::from(dst_texture.desc.format);
    if src_tex_base.aspect != src_texture_aspects {
        return Err(TransferError::CopySrcMissingAspects.into());
    }
    if dst_tex_base.aspect != dst_texture_aspects {
        return Err(TransferError::CopyDstMissingAspects.into());
    }

    if src_texture.desc.sample_count != dst_texture.desc.sample_count {
        return Err(TransferError::SampleCountNotEqual {
            src_sample_count: src_texture.desc.sample_count,
            dst_sample_count: dst_texture.desc.sample_count,
        }
        .into());
    }

    let src_raw = src_texture.try_raw(state.snatch_guard)?;
    src_texture
        .check_usage(TextureUsages::COPY_SRC)
        .map_err(TransferError::MissingTextureUsage)?;
    let dst_raw = dst_texture.try_raw(state.snatch_guard)?;
    dst_texture
        .check_usage(TextureUsages::COPY_DST)
        .map_err(TransferError::MissingTextureUsage)?;

    // This must happen after parameter validation (so that errors are reported
    // as required by the spec), but before any side effects.
    if copy_size.width == 0 || copy_size.height == 0 || copy_size.depth_or_array_layers == 0 {
        log::trace!("Ignoring copy_texture_to_texture of size 0");
        return Ok(());
    }

    // Handle texture init *before* dealing with barrier transitions so we
    // have an easier time inserting "immediate-inits" that may be required
    // by prior discards in rare cases.
    handle_src_texture_init(state, source, copy_size, src_texture)?;
    handle_dst_texture_init(state, destination, copy_size, dst_texture)?;

    let src_pending =
        state
            .tracker
            .textures
            .set_single(src_texture, src_range, wgt::TextureUses::COPY_SRC);

    //TODO: try to avoid this the collection. It's needed because both
    // `src_pending` and `dst_pending` try to hold `trackers.textures` mutably.
    let mut barriers: ArrayVec<_, 2> = src_pending
        .map(|pending| pending.into_hal(src_raw))
        .collect();

    let dst_pending =
        state
            .tracker
            .textures
            .set_single(dst_texture, dst_range, wgt::TextureUses::COPY_DST);
    barriers.extend(dst_pending.map(|pending| pending.into_hal(dst_raw)));

    let hal_copy_size = hal::CopyExtent {
        width: src_copy_size.width.min(dst_copy_size.width),
        height: src_copy_size.height.min(dst_copy_size.height),
        depth: src_copy_size.depth.min(dst_copy_size.depth),
    };

    let dst_format = dst_texture.desc.format;

    let regions = (0..array_layer_count).map(|rel_array_layer| {
        let mut src_base = src_tex_base.clone();
        let mut dst_base = dst_tex_base.clone();
        src_base.array_layer += rel_array_layer;
        dst_base.array_layer += rel_array_layer;
        hal::TextureCopy {
            src_base,
            dst_base,
            size: hal_copy_size,
        }
    });

    let regions = if dst_tex_base.aspect == hal::FormatAspects::DEPTH_STENCIL {
        regions
            .flat_map(|region| {
                let (mut depth, mut stencil) = (region.clone(), region);
                depth.src_base.aspect = hal::FormatAspects::DEPTH;
                depth.dst_base.aspect = hal::FormatAspects::DEPTH;
                stencil.src_base.aspect = hal::FormatAspects::STENCIL;
                stencil.dst_base.aspect = hal::FormatAspects::STENCIL;
                [depth, stencil]
            })
            .collect::<Vec<_>>()
    } else if let Some(plane_count) = dst_format.planes() {
        regions
            .into_iter()
            .flat_map(|region| {
                (0..plane_count).map(move |plane| {
                    let mut plane_region = region.clone();

                    let plane_aspect = wgt::TextureAspect::from_plane(plane)
                        .expect("expected texture aspect to exist for the plane");
                    let plane_aspect = hal::FormatAspects::new(dst_format, plane_aspect);
                    plane_region.src_base.aspect = plane_aspect;
                    plane_region.dst_base.aspect = plane_aspect;

                    let (w_subsampling, h_subsampling) =
                        dst_format.subsampling_factors(Some(plane));
                    plane_region.src_base.origin.x /= w_subsampling;
                    plane_region.src_base.origin.y /= h_subsampling;
                    plane_region.dst_base.origin.x /= w_subsampling;
                    plane_region.dst_base.origin.y /= h_subsampling;

                    plane_region.size.width /= w_subsampling;
                    plane_region.size.height /= h_subsampling;

                    plane_region
                })
            })
            .collect::<Vec<_>>()
    } else {
        regions.collect::<Vec<_>>()
    };
    unsafe {
        state.raw_encoder.transition_textures(&barriers);
        state.raw_encoder.copy_texture_to_texture(
            src_raw,
            wgt::TextureUses::COPY_SRC,
            dst_raw,
            &regions,
        );
    }

    Ok(())
}
