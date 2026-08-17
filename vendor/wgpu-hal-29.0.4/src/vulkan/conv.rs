use alloc::vec::Vec;

use ash::vk;

impl super::PrivateCapabilities {
    pub fn map_texture_format(&self, format: wgt::TextureFormat) -> vk::Format {
        use ash::vk::Format as F;
        use wgt::TextureFormat as Tf;
        use wgt::{AstcBlock, AstcChannel};
        match format {
            Tf::R8Unorm => F::R8_UNORM,
            Tf::R8Snorm => F::R8_SNORM,
            Tf::R8Uint => F::R8_UINT,
            Tf::R8Sint => F::R8_SINT,
            Tf::R16Uint => F::R16_UINT,
            Tf::R16Sint => F::R16_SINT,
            Tf::R16Unorm => F::R16_UNORM,
            Tf::R16Snorm => F::R16_SNORM,
            Tf::R16Float => F::R16_SFLOAT,
            Tf::Rg8Unorm => F::R8G8_UNORM,
            Tf::Rg8Snorm => F::R8G8_SNORM,
            Tf::Rg8Uint => F::R8G8_UINT,
            Tf::Rg8Sint => F::R8G8_SINT,
            Tf::Rg16Unorm => F::R16G16_UNORM,
            Tf::Rg16Snorm => F::R16G16_SNORM,
            Tf::R32Uint => F::R32_UINT,
            Tf::R32Sint => F::R32_SINT,
            Tf::R32Float => F::R32_SFLOAT,
            Tf::Rg16Uint => F::R16G16_UINT,
            Tf::Rg16Sint => F::R16G16_SINT,
            Tf::Rg16Float => F::R16G16_SFLOAT,
            Tf::Rgba8Unorm => F::R8G8B8A8_UNORM,
            Tf::Rgba8UnormSrgb => F::R8G8B8A8_SRGB,
            Tf::Bgra8UnormSrgb => F::B8G8R8A8_SRGB,
            Tf::Rgba8Snorm => F::R8G8B8A8_SNORM,
            Tf::Bgra8Unorm => F::B8G8R8A8_UNORM,
            Tf::Rgba8Uint => F::R8G8B8A8_UINT,
            Tf::Rgba8Sint => F::R8G8B8A8_SINT,
            Tf::Rgb10a2Uint => F::A2B10G10R10_UINT_PACK32,
            Tf::Rgb10a2Unorm => F::A2B10G10R10_UNORM_PACK32,
            Tf::Rg11b10Ufloat => F::B10G11R11_UFLOAT_PACK32,
            Tf::R64Uint => F::R64_UINT,
            Tf::Rg32Uint => F::R32G32_UINT,
            Tf::Rg32Sint => F::R32G32_SINT,
            Tf::Rg32Float => F::R32G32_SFLOAT,
            Tf::Rgba16Uint => F::R16G16B16A16_UINT,
            Tf::Rgba16Sint => F::R16G16B16A16_SINT,
            Tf::Rgba16Unorm => F::R16G16B16A16_UNORM,
            Tf::Rgba16Snorm => F::R16G16B16A16_SNORM,
            Tf::Rgba16Float => F::R16G16B16A16_SFLOAT,
            Tf::Rgba32Uint => F::R32G32B32A32_UINT,
            Tf::Rgba32Sint => F::R32G32B32A32_SINT,
            Tf::Rgba32Float => F::R32G32B32A32_SFLOAT,
            Tf::Depth32Float => F::D32_SFLOAT,
            Tf::Depth32FloatStencil8 => F::D32_SFLOAT_S8_UINT,
            Tf::Depth24Plus => {
                if self.texture_d24 {
                    F::X8_D24_UNORM_PACK32
                } else {
                    F::D32_SFLOAT
                }
            }
            Tf::Depth24PlusStencil8 => {
                if self.texture_d24_s8 {
                    F::D24_UNORM_S8_UINT
                } else {
                    F::D32_SFLOAT_S8_UINT
                }
            }
            Tf::Stencil8 => {
                if self.texture_s8 {
                    F::S8_UINT
                } else if self.texture_d24_s8 {
                    F::D24_UNORM_S8_UINT
                } else {
                    F::D32_SFLOAT_S8_UINT
                }
            }
            Tf::Depth16Unorm => F::D16_UNORM,
            Tf::NV12 => F::G8_B8R8_2PLANE_420_UNORM,
            Tf::P010 => F::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16,
            Tf::Rgb9e5Ufloat => F::E5B9G9R9_UFLOAT_PACK32,
            Tf::Bc1RgbaUnorm => F::BC1_RGBA_UNORM_BLOCK,
            Tf::Bc1RgbaUnormSrgb => F::BC1_RGBA_SRGB_BLOCK,
            Tf::Bc2RgbaUnorm => F::BC2_UNORM_BLOCK,
            Tf::Bc2RgbaUnormSrgb => F::BC2_SRGB_BLOCK,
            Tf::Bc3RgbaUnorm => F::BC3_UNORM_BLOCK,
            Tf::Bc3RgbaUnormSrgb => F::BC3_SRGB_BLOCK,
            Tf::Bc4RUnorm => F::BC4_UNORM_BLOCK,
            Tf::Bc4RSnorm => F::BC4_SNORM_BLOCK,
            Tf::Bc5RgUnorm => F::BC5_UNORM_BLOCK,
            Tf::Bc5RgSnorm => F::BC5_SNORM_BLOCK,
            Tf::Bc6hRgbUfloat => F::BC6H_UFLOAT_BLOCK,
            Tf::Bc6hRgbFloat => F::BC6H_SFLOAT_BLOCK,
            Tf::Bc7RgbaUnorm => F::BC7_UNORM_BLOCK,
            Tf::Bc7RgbaUnormSrgb => F::BC7_SRGB_BLOCK,
            Tf::Etc2Rgb8Unorm => F::ETC2_R8G8B8_UNORM_BLOCK,
            Tf::Etc2Rgb8UnormSrgb => F::ETC2_R8G8B8_SRGB_BLOCK,
            Tf::Etc2Rgb8A1Unorm => F::ETC2_R8G8B8A1_UNORM_BLOCK,
            Tf::Etc2Rgb8A1UnormSrgb => F::ETC2_R8G8B8A1_SRGB_BLOCK,
            Tf::Etc2Rgba8Unorm => F::ETC2_R8G8B8A8_UNORM_BLOCK,
            Tf::Etc2Rgba8UnormSrgb => F::ETC2_R8G8B8A8_SRGB_BLOCK,
            Tf::EacR11Unorm => F::EAC_R11_UNORM_BLOCK,
            Tf::EacR11Snorm => F::EAC_R11_SNORM_BLOCK,
            Tf::EacRg11Unorm => F::EAC_R11G11_UNORM_BLOCK,
            Tf::EacRg11Snorm => F::EAC_R11G11_SNORM_BLOCK,
            Tf::Astc { block, channel } => match channel {
                AstcChannel::Unorm => match block {
                    AstcBlock::B4x4 => F::ASTC_4X4_UNORM_BLOCK,
                    AstcBlock::B5x4 => F::ASTC_5X4_UNORM_BLOCK,
                    AstcBlock::B5x5 => F::ASTC_5X5_UNORM_BLOCK,
                    AstcBlock::B6x5 => F::ASTC_6X5_UNORM_BLOCK,
                    AstcBlock::B6x6 => F::ASTC_6X6_UNORM_BLOCK,
                    AstcBlock::B8x5 => F::ASTC_8X5_UNORM_BLOCK,
                    AstcBlock::B8x6 => F::ASTC_8X6_UNORM_BLOCK,
                    AstcBlock::B8x8 => F::ASTC_8X8_UNORM_BLOCK,
                    AstcBlock::B10x5 => F::ASTC_10X5_UNORM_BLOCK,
                    AstcBlock::B10x6 => F::ASTC_10X6_UNORM_BLOCK,
                    AstcBlock::B10x8 => F::ASTC_10X8_UNORM_BLOCK,
                    AstcBlock::B10x10 => F::ASTC_10X10_UNORM_BLOCK,
                    AstcBlock::B12x10 => F::ASTC_12X10_UNORM_BLOCK,
                    AstcBlock::B12x12 => F::ASTC_12X12_UNORM_BLOCK,
                },
                AstcChannel::UnormSrgb => match block {
                    AstcBlock::B4x4 => F::ASTC_4X4_SRGB_BLOCK,
                    AstcBlock::B5x4 => F::ASTC_5X4_SRGB_BLOCK,
                    AstcBlock::B5x5 => F::ASTC_5X5_SRGB_BLOCK,
                    AstcBlock::B6x5 => F::ASTC_6X5_SRGB_BLOCK,
                    AstcBlock::B6x6 => F::ASTC_6X6_SRGB_BLOCK,
                    AstcBlock::B8x5 => F::ASTC_8X5_SRGB_BLOCK,
                    AstcBlock::B8x6 => F::ASTC_8X6_SRGB_BLOCK,
                    AstcBlock::B8x8 => F::ASTC_8X8_SRGB_BLOCK,
                    AstcBlock::B10x5 => F::ASTC_10X5_SRGB_BLOCK,
                    AstcBlock::B10x6 => F::ASTC_10X6_SRGB_BLOCK,
                    AstcBlock::B10x8 => F::ASTC_10X8_SRGB_BLOCK,
                    AstcBlock::B10x10 => F::ASTC_10X10_SRGB_BLOCK,
                    AstcBlock::B12x10 => F::ASTC_12X10_SRGB_BLOCK,
                    AstcBlock::B12x12 => F::ASTC_12X12_SRGB_BLOCK,
                },
                AstcChannel::Hdr => match block {
                    AstcBlock::B4x4 => F::ASTC_4X4_SFLOAT_BLOCK_EXT,
                    AstcBlock::B5x4 => F::ASTC_5X4_SFLOAT_BLOCK_EXT,
                    AstcBlock::B5x5 => F::ASTC_5X5_SFLOAT_BLOCK_EXT,
                    AstcBlock::B6x5 => F::ASTC_6X5_SFLOAT_BLOCK_EXT,
                    AstcBlock::B6x6 => F::ASTC_6X6_SFLOAT_BLOCK_EXT,
                    AstcBlock::B8x5 => F::ASTC_8X5_SFLOAT_BLOCK_EXT,
                    AstcBlock::B8x6 => F::ASTC_8X6_SFLOAT_BLOCK_EXT,
                    AstcBlock::B8x8 => F::ASTC_8X8_SFLOAT_BLOCK_EXT,
                    AstcBlock::B10x5 => F::ASTC_10X5_SFLOAT_BLOCK_EXT,
                    AstcBlock::B10x6 => F::ASTC_10X6_SFLOAT_BLOCK_EXT,
                    AstcBlock::B10x8 => F::ASTC_10X8_SFLOAT_BLOCK_EXT,
                    AstcBlock::B10x10 => F::ASTC_10X10_SFLOAT_BLOCK_EXT,
                    AstcBlock::B12x10 => F::ASTC_12X10_SFLOAT_BLOCK_EXT,
                    AstcBlock::B12x12 => F::ASTC_12X12_SFLOAT_BLOCK_EXT,
                },
            },
        }
    }
}

pub fn map_vk_surface_formats(sf: vk::SurfaceFormatKHR) -> Option<wgt::TextureFormat> {
    use ash::vk::Format as F;
    use wgt::TextureFormat as Tf;
    // List we care about pulled from https://vulkan.gpuinfo.org/listsurfaceformats.php.
    // Device::create_swapchain() hardcodes linear scRGB for fp16, non-linear sRGB otherwise.
    Some(match sf.color_space {
        vk::ColorSpaceKHR::SRGB_NONLINEAR => match sf.format {
            F::B8G8R8A8_UNORM => Tf::Bgra8Unorm,
            F::B8G8R8A8_SRGB => Tf::Bgra8UnormSrgb,
            F::R8G8B8A8_SNORM => Tf::Rgba8Snorm,
            F::R8G8B8A8_UNORM => Tf::Rgba8Unorm,
            F::R8G8B8A8_SRGB => Tf::Rgba8UnormSrgb,
            F::R16G16B16A16_SNORM => Tf::Rgba16Snorm,
            F::R16G16B16A16_UNORM => Tf::Rgba16Unorm,
            F::A2B10G10R10_UNORM_PACK32 => Tf::Rgb10a2Unorm,
            _ => return None,
        },
        vk::ColorSpaceKHR::EXTENDED_SRGB_LINEAR_EXT => match sf.format {
            F::R16G16B16A16_SFLOAT => Tf::Rgba16Float,
            _ => return None,
        },
        _ => return None,
    })
}

impl crate::Attachment<'_, super::TextureView> {
    pub(super) fn make_attachment_key(&self, ops: crate::AttachmentOps) -> super::AttachmentKey {
        super::AttachmentKey {
            format: self.view.raw_format,
            layout: derive_image_layout(self.usage, self.view.format),
            ops,
        }
    }
}

impl crate::ColorAttachment<'_, super::TextureView> {
    pub(super) unsafe fn make_vk_clear_color(&self) -> vk::ClearColorValue {
        let cv = &self.clear_value;
        match self.target.view.format.sample_type(None, None).unwrap() {
            wgt::TextureSampleType::Float { .. } => vk::ClearColorValue {
                float32: [cv.r as f32, cv.g as f32, cv.b as f32, cv.a as f32],
            },
            wgt::TextureSampleType::Sint => vk::ClearColorValue {
                int32: [cv.r as i32, cv.g as i32, cv.b as i32, cv.a as i32],
            },
            wgt::TextureSampleType::Uint => vk::ClearColorValue {
                uint32: [cv.r as u32, cv.g as u32, cv.b as u32, cv.a as u32],
            },
            wgt::TextureSampleType::Depth => unreachable!(),
        }
    }
}

pub fn derive_image_layout(usage: wgt::TextureUses, format: wgt::TextureFormat) -> vk::ImageLayout {
    // Note: depth textures are always sampled with RODS layout
    let is_color = !format.is_depth_stencil_format();
    match usage {
        wgt::TextureUses::UNINITIALIZED => vk::ImageLayout::UNDEFINED,
        wgt::TextureUses::COPY_SRC => vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        wgt::TextureUses::COPY_DST => vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        wgt::TextureUses::RESOURCE if is_color => vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        wgt::TextureUses::COLOR_TARGET => vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        wgt::TextureUses::DEPTH_STENCIL_WRITE => vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        _ => {
            if usage == wgt::TextureUses::PRESENT {
                vk::ImageLayout::PRESENT_SRC_KHR
            } else if is_color {
                vk::ImageLayout::GENERAL
            } else {
                vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL
            }
        }
    }
}

pub fn map_texture_usage(usage: wgt::TextureUses) -> vk::ImageUsageFlags {
    let mut flags = vk::ImageUsageFlags::empty();
    if usage.contains(wgt::TextureUses::COPY_SRC) {
        flags |= vk::ImageUsageFlags::TRANSFER_SRC;
    }
    if usage.contains(wgt::TextureUses::COPY_DST) {
        flags |= vk::ImageUsageFlags::TRANSFER_DST;
    }
    if usage.contains(wgt::TextureUses::RESOURCE) {
        flags |= vk::ImageUsageFlags::SAMPLED;
    }
    if usage.contains(wgt::TextureUses::COLOR_TARGET) {
        flags |= vk::ImageUsageFlags::COLOR_ATTACHMENT;
    }
    if usage
        .intersects(wgt::TextureUses::DEPTH_STENCIL_READ | wgt::TextureUses::DEPTH_STENCIL_WRITE)
    {
        flags |= vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;
    }
    if usage.intersects(
        wgt::TextureUses::STORAGE_READ_ONLY
            | wgt::TextureUses::STORAGE_WRITE_ONLY
            | wgt::TextureUses::STORAGE_READ_WRITE
            | wgt::TextureUses::STORAGE_ATOMIC,
    ) {
        flags |= vk::ImageUsageFlags::STORAGE;
    }
    if usage.contains(wgt::TextureUses::TRANSIENT) {
        flags |= vk::ImageUsageFlags::TRANSIENT_ATTACHMENT;
    }
    flags
}

pub fn map_texture_usage_to_barrier(
    usage: wgt::TextureUses,
) -> (vk::PipelineStageFlags, vk::AccessFlags) {
    let mut stages = vk::PipelineStageFlags::empty();
    let mut access = vk::AccessFlags::empty();
    let shader_stages = vk::PipelineStageFlags::VERTEX_SHADER
        | vk::PipelineStageFlags::FRAGMENT_SHADER
        | vk::PipelineStageFlags::COMPUTE_SHADER;

    if usage.contains(wgt::TextureUses::COPY_SRC) {
        stages |= vk::PipelineStageFlags::TRANSFER;
        access |= vk::AccessFlags::TRANSFER_READ;
    }
    if usage.contains(wgt::TextureUses::COPY_DST) {
        stages |= vk::PipelineStageFlags::TRANSFER;
        access |= vk::AccessFlags::TRANSFER_WRITE;
    }
    if usage.contains(wgt::TextureUses::RESOURCE) {
        stages |= shader_stages;
        access |= vk::AccessFlags::SHADER_READ;
    }
    if usage.contains(wgt::TextureUses::COLOR_TARGET) {
        stages |= vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        access |= vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE;
    }
    if usage.intersects(wgt::TextureUses::DEPTH_STENCIL_READ) {
        stages |= vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
            | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS;
        access |= vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ;
    }
    if usage.intersects(wgt::TextureUses::DEPTH_STENCIL_WRITE) {
        stages |= vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
            | vk::PipelineStageFlags::LATE_FRAGMENT_TESTS;
        access |= vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
            | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE;
    }
    if usage.intersects(
        wgt::TextureUses::STORAGE_READ_ONLY
            | wgt::TextureUses::STORAGE_READ_WRITE
            | wgt::TextureUses::STORAGE_ATOMIC,
    ) {
        stages |= shader_stages;
        access |= vk::AccessFlags::SHADER_READ;
    }
    if usage.intersects(
        wgt::TextureUses::STORAGE_WRITE_ONLY
            | wgt::TextureUses::STORAGE_READ_WRITE
            | wgt::TextureUses::STORAGE_ATOMIC,
    ) {
        stages |= shader_stages;
        access |= vk::AccessFlags::SHADER_WRITE;
    }

    if usage == wgt::TextureUses::UNINITIALIZED || usage == wgt::TextureUses::PRESENT {
        (
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::AccessFlags::empty(),
        )
    } else {
        (stages, access)
    }
}

pub fn map_vk_image_usage(usage: vk::ImageUsageFlags) -> wgt::TextureUses {
    let mut bits = wgt::TextureUses::empty();
    if usage.contains(vk::ImageUsageFlags::TRANSFER_SRC) {
        bits |= wgt::TextureUses::COPY_SRC;
    }
    if usage.contains(vk::ImageUsageFlags::TRANSFER_DST) {
        bits |= wgt::TextureUses::COPY_DST;
    }
    if usage.contains(vk::ImageUsageFlags::SAMPLED) {
        bits |= wgt::TextureUses::RESOURCE;
    }
    if usage.contains(vk::ImageUsageFlags::COLOR_ATTACHMENT) {
        bits |= wgt::TextureUses::COLOR_TARGET;
    }
    if usage.contains(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT) {
        bits |= wgt::TextureUses::DEPTH_STENCIL_READ | wgt::TextureUses::DEPTH_STENCIL_WRITE;
    }
    if usage.contains(vk::ImageUsageFlags::STORAGE) {
        bits |= wgt::TextureUses::STORAGE_READ_ONLY
            | wgt::TextureUses::STORAGE_WRITE_ONLY
            | wgt::TextureUses::STORAGE_READ_WRITE
            | wgt::TextureUses::STORAGE_ATOMIC;
    }
    if usage.contains(vk::ImageUsageFlags::TRANSIENT_ATTACHMENT) {
        bits |= wgt::TextureUses::TRANSIENT;
    }
    bits
}

pub fn map_texture_dimension(dim: wgt::TextureDimension) -> vk::ImageType {
    match dim {
        wgt::TextureDimension::D1 => vk::ImageType::TYPE_1D,
        wgt::TextureDimension::D2 => vk::ImageType::TYPE_2D,
        wgt::TextureDimension::D3 => vk::ImageType::TYPE_3D,
    }
}

pub fn map_index_format(index_format: wgt::IndexFormat) -> vk::IndexType {
    match index_format {
        wgt::IndexFormat::Uint16 => vk::IndexType::UINT16,
        wgt::IndexFormat::Uint32 => vk::IndexType::UINT32,
    }
}

pub fn map_vertex_format(vertex_format: wgt::VertexFormat) -> vk::Format {
    use wgt::VertexFormat as Vf;
    match vertex_format {
        Vf::Uint8 => vk::Format::R8_UINT,
        Vf::Uint8x2 => vk::Format::R8G8_UINT,
        Vf::Uint8x4 => vk::Format::R8G8B8A8_UINT,
        Vf::Sint8 => vk::Format::R8_SINT,
        Vf::Sint8x2 => vk::Format::R8G8_SINT,
        Vf::Sint8x4 => vk::Format::R8G8B8A8_SINT,
        Vf::Unorm8 => vk::Format::R8_UNORM,
        Vf::Unorm8x2 => vk::Format::R8G8_UNORM,
        Vf::Unorm8x4 => vk::Format::R8G8B8A8_UNORM,
        Vf::Snorm8 => vk::Format::R8_SNORM,
        Vf::Snorm8x2 => vk::Format::R8G8_SNORM,
        Vf::Snorm8x4 => vk::Format::R8G8B8A8_SNORM,
        Vf::Uint16 => vk::Format::R16_UINT,
        Vf::Uint16x2 => vk::Format::R16G16_UINT,
        Vf::Uint16x4 => vk::Format::R16G16B16A16_UINT,
        Vf::Sint16 => vk::Format::R16_SINT,
        Vf::Sint16x2 => vk::Format::R16G16_SINT,
        Vf::Sint16x4 => vk::Format::R16G16B16A16_SINT,
        Vf::Unorm16 => vk::Format::R16_UNORM,
        Vf::Unorm16x2 => vk::Format::R16G16_UNORM,
        Vf::Unorm16x4 => vk::Format::R16G16B16A16_UNORM,
        Vf::Snorm16 => vk::Format::R16_SNORM,
        Vf::Snorm16x2 => vk::Format::R16G16_SNORM,
        Vf::Snorm16x4 => vk::Format::R16G16B16A16_SNORM,
        Vf::Float16 => vk::Format::R16_SFLOAT,
        Vf::Float16x2 => vk::Format::R16G16_SFLOAT,
        Vf::Float16x4 => vk::Format::R16G16B16A16_SFLOAT,
        Vf::Float32 => vk::Format::R32_SFLOAT,
        Vf::Float32x2 => vk::Format::R32G32_SFLOAT,
        Vf::Float32x3 => vk::Format::R32G32B32_SFLOAT,
        Vf::Float32x4 => vk::Format::R32G32B32A32_SFLOAT,
        Vf::Uint32 => vk::Format::R32_UINT,
        Vf::Uint32x2 => vk::Format::R32G32_UINT,
        Vf::Uint32x3 => vk::Format::R32G32B32_UINT,
        Vf::Uint32x4 => vk::Format::R32G32B32A32_UINT,
        Vf::Sint32 => vk::Format::R32_SINT,
        Vf::Sint32x2 => vk::Format::R32G32_SINT,
        Vf::Sint32x3 => vk::Format::R32G32B32_SINT,
        Vf::Sint32x4 => vk::Format::R32G32B32A32_SINT,
        Vf::Float64 => vk::Format::R64_SFLOAT,
        Vf::Float64x2 => vk::Format::R64G64_SFLOAT,
        Vf::Float64x3 => vk::Format::R64G64B64_SFLOAT,
        Vf::Float64x4 => vk::Format::R64G64B64A64_SFLOAT,
        Vf::Unorm10_10_10_2 => vk::Format::A2B10G10R10_UNORM_PACK32,
        Vf::Unorm8x4Bgra => vk::Format::B8G8R8A8_UNORM,
    }
}

pub fn map_aspects(aspects: crate::FormatAspects) -> vk::ImageAspectFlags {
    let mut flags = vk::ImageAspectFlags::empty();
    if aspects.contains(crate::FormatAspects::COLOR) {
        flags |= vk::ImageAspectFlags::COLOR;
    }
    if aspects.contains(crate::FormatAspects::DEPTH) {
        flags |= vk::ImageAspectFlags::DEPTH;
    }
    if aspects.contains(crate::FormatAspects::STENCIL) {
        flags |= vk::ImageAspectFlags::STENCIL;
    }
    if aspects.contains(crate::FormatAspects::PLANE_0) {
        flags |= vk::ImageAspectFlags::PLANE_0;
    }
    if aspects.contains(crate::FormatAspects::PLANE_1) {
        flags |= vk::ImageAspectFlags::PLANE_1;
    }
    if aspects.contains(crate::FormatAspects::PLANE_2) {
        flags |= vk::ImageAspectFlags::PLANE_2;
    }
    flags
}

pub fn map_attachment_ops(
    op: crate::AttachmentOps,
) -> (vk::AttachmentLoadOp, vk::AttachmentStoreOp) {
    let load_op = if op.contains(crate::AttachmentOps::LOAD) {
        vk::AttachmentLoadOp::LOAD
    } else if op.contains(crate::AttachmentOps::LOAD_DONT_CARE) {
        vk::AttachmentLoadOp::DONT_CARE
    } else if op.contains(crate::AttachmentOps::LOAD_CLEAR) {
        vk::AttachmentLoadOp::CLEAR
    } else {
        unreachable!()
    };
    let store_op = if op.contains(crate::AttachmentOps::STORE) {
        vk::AttachmentStoreOp::STORE
    } else if op.contains(crate::AttachmentOps::STORE_DISCARD) {
        vk::AttachmentStoreOp::DONT_CARE
    } else {
        unreachable!()
    };
    (load_op, store_op)
}

pub fn map_present_mode(mode: wgt::PresentMode) -> vk::PresentModeKHR {
    match mode {
        wgt::PresentMode::Immediate => vk::PresentModeKHR::IMMEDIATE,
        wgt::PresentMode::Mailbox => vk::PresentModeKHR::MAILBOX,
        wgt::PresentMode::Fifo => vk::PresentModeKHR::FIFO,
        wgt::PresentMode::FifoRelaxed => vk::PresentModeKHR::FIFO_RELAXED,
        wgt::PresentMode::AutoNoVsync | wgt::PresentMode::AutoVsync => {
            unreachable!("Cannot create swapchain with Auto PresentationMode")
        }
    }
}

pub fn map_vk_present_mode(mode: vk::PresentModeKHR) -> Option<wgt::PresentMode> {
    // Not exposed in Ash yet.
    const FIFO_LATEST_READY: vk::PresentModeKHR = vk::PresentModeKHR::from_raw(1_000_361_000);

    // See https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentModeKHR.html
    match mode {
        vk::PresentModeKHR::IMMEDIATE => Some(wgt::PresentMode::Immediate),
        vk::PresentModeKHR::MAILBOX => Some(wgt::PresentMode::Mailbox),
        vk::PresentModeKHR::FIFO => Some(wgt::PresentMode::Fifo),
        vk::PresentModeKHR::FIFO_RELAXED => Some(wgt::PresentMode::FifoRelaxed),

        // Modes that aren't exposed yet.
        vk::PresentModeKHR::SHARED_DEMAND_REFRESH => None,
        vk::PresentModeKHR::SHARED_CONTINUOUS_REFRESH => None,
        FIFO_LATEST_READY => None,

        _ => {
            log::debug!("Unrecognized present mode {mode:?}");
            None
        }
    }
}

pub fn map_composite_alpha_mode(mode: wgt::CompositeAlphaMode) -> vk::CompositeAlphaFlagsKHR {
    match mode {
        wgt::CompositeAlphaMode::Opaque => vk::CompositeAlphaFlagsKHR::OPAQUE,
        wgt::CompositeAlphaMode::PreMultiplied => vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED,
        wgt::CompositeAlphaMode::PostMultiplied => vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED,
        wgt::CompositeAlphaMode::Inherit => vk::CompositeAlphaFlagsKHR::INHERIT,
        wgt::CompositeAlphaMode::Auto => unreachable!(),
    }
}

pub fn map_vk_composite_alpha(flags: vk::CompositeAlphaFlagsKHR) -> Vec<wgt::CompositeAlphaMode> {
    let mut modes = Vec::new();
    if flags.contains(vk::CompositeAlphaFlagsKHR::OPAQUE) {
        modes.push(wgt::CompositeAlphaMode::Opaque);
    }
    if flags.contains(vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED) {
        modes.push(wgt::CompositeAlphaMode::PreMultiplied);
    }
    if flags.contains(vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED) {
        modes.push(wgt::CompositeAlphaMode::PostMultiplied);
    }
    if flags.contains(vk::CompositeAlphaFlagsKHR::INHERIT) {
        modes.push(wgt::CompositeAlphaMode::Inherit);
    }
    modes
}

pub fn map_buffer_usage(usage: wgt::BufferUses) -> vk::BufferUsageFlags {
    let mut flags = vk::BufferUsageFlags::empty();
    if usage.contains(wgt::BufferUses::COPY_SRC) {
        flags |= vk::BufferUsageFlags::TRANSFER_SRC;
    }
    if usage.contains(wgt::BufferUses::COPY_DST) {
        flags |= vk::BufferUsageFlags::TRANSFER_DST;
    }
    if usage.contains(wgt::BufferUses::UNIFORM) {
        flags |= vk::BufferUsageFlags::UNIFORM_BUFFER;
    }
    if usage.intersects(wgt::BufferUses::STORAGE_READ_ONLY | wgt::BufferUses::STORAGE_READ_WRITE) {
        flags |= vk::BufferUsageFlags::STORAGE_BUFFER;
    }
    if usage.contains(wgt::BufferUses::INDEX) {
        flags |= vk::BufferUsageFlags::INDEX_BUFFER;
    }
    if usage.contains(wgt::BufferUses::VERTEX) {
        flags |= vk::BufferUsageFlags::VERTEX_BUFFER;
    }
    if usage.contains(wgt::BufferUses::INDIRECT) {
        flags |= vk::BufferUsageFlags::INDIRECT_BUFFER;
    }
    if usage.contains(wgt::BufferUses::ACCELERATION_STRUCTURE_SCRATCH) {
        flags |= vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS;
    }
    if usage.intersects(
        wgt::BufferUses::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT
            | wgt::BufferUses::TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT,
    ) {
        flags |= vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
            | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS;
    }
    if usage.intersects(wgt::BufferUses::ACCELERATION_STRUCTURE_QUERY) {
        flags |= vk::BufferUsageFlags::TRANSFER_DST;
    }
    flags
}

pub fn map_buffer_usage_to_barrier(
    usage: wgt::BufferUses,
) -> (vk::PipelineStageFlags, vk::AccessFlags) {
    let mut stages = vk::PipelineStageFlags::empty();
    let mut access = vk::AccessFlags::empty();
    let shader_stages = vk::PipelineStageFlags::VERTEX_SHADER
        | vk::PipelineStageFlags::FRAGMENT_SHADER
        | vk::PipelineStageFlags::COMPUTE_SHADER;

    if usage.contains(wgt::BufferUses::MAP_READ) {
        stages |= vk::PipelineStageFlags::HOST;
        access |= vk::AccessFlags::HOST_READ;
    }
    if usage.contains(wgt::BufferUses::MAP_WRITE) {
        stages |= vk::PipelineStageFlags::HOST;
        access |= vk::AccessFlags::HOST_WRITE;
    }
    if usage.contains(wgt::BufferUses::COPY_SRC) {
        stages |= vk::PipelineStageFlags::TRANSFER;
        access |= vk::AccessFlags::TRANSFER_READ;
    }
    if usage.contains(wgt::BufferUses::COPY_DST) {
        stages |= vk::PipelineStageFlags::TRANSFER;
        access |= vk::AccessFlags::TRANSFER_WRITE;
    }
    if usage.contains(wgt::BufferUses::UNIFORM) {
        stages |= shader_stages;
        access |= vk::AccessFlags::UNIFORM_READ;
    }
    if usage.intersects(wgt::BufferUses::STORAGE_READ_ONLY) {
        stages |= shader_stages;
        access |= vk::AccessFlags::SHADER_READ;
    }
    if usage.intersects(wgt::BufferUses::STORAGE_READ_WRITE) {
        stages |= shader_stages;
        access |= vk::AccessFlags::SHADER_READ | vk::AccessFlags::SHADER_WRITE;
    }
    if usage.contains(wgt::BufferUses::INDEX) {
        stages |= vk::PipelineStageFlags::VERTEX_INPUT;
        access |= vk::AccessFlags::INDEX_READ;
    }
    if usage.contains(wgt::BufferUses::VERTEX) {
        stages |= vk::PipelineStageFlags::VERTEX_INPUT;
        access |= vk::AccessFlags::VERTEX_ATTRIBUTE_READ;
    }
    if usage.contains(wgt::BufferUses::INDIRECT) {
        stages |= vk::PipelineStageFlags::DRAW_INDIRECT;
        access |= vk::AccessFlags::INDIRECT_COMMAND_READ;
    }
    if usage.intersects(
        wgt::BufferUses::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT
            | wgt::BufferUses::TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT
            | wgt::BufferUses::ACCELERATION_STRUCTURE_SCRATCH,
    ) {
        stages |= vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR;
        access |= vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR
            | vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR
            | vk::AccessFlags::SHADER_READ;
    }
    if usage.contains(wgt::BufferUses::ACCELERATION_STRUCTURE_QUERY) {
        stages |= vk::PipelineStageFlags::TRANSFER;
        access |= vk::AccessFlags::TRANSFER_WRITE;
    }

    (stages, access)
}

pub fn map_view_dimension(dim: wgt::TextureViewDimension) -> vk::ImageViewType {
    match dim {
        wgt::TextureViewDimension::D1 => vk::ImageViewType::TYPE_1D,
        wgt::TextureViewDimension::D2 => vk::ImageViewType::TYPE_2D,
        wgt::TextureViewDimension::D2Array => vk::ImageViewType::TYPE_2D_ARRAY,
        wgt::TextureViewDimension::Cube => vk::ImageViewType::CUBE,
        wgt::TextureViewDimension::CubeArray => vk::ImageViewType::CUBE_ARRAY,
        wgt::TextureViewDimension::D3 => vk::ImageViewType::TYPE_3D,
    }
}

pub fn map_copy_extent(extent: &crate::CopyExtent) -> vk::Extent3D {
    vk::Extent3D {
        width: extent.width,
        height: extent.height,
        depth: extent.depth,
    }
}

pub fn map_subresource_range(
    range: &wgt::ImageSubresourceRange,
    format: wgt::TextureFormat,
) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange {
        aspect_mask: map_aspects(crate::FormatAspects::new(format, range.aspect)),
        base_mip_level: range.base_mip_level,
        level_count: range.mip_level_count.unwrap_or(vk::REMAINING_MIP_LEVELS),
        base_array_layer: range.base_array_layer,
        layer_count: range
            .array_layer_count
            .unwrap_or(vk::REMAINING_ARRAY_LAYERS),
    }
}

// Special subresource range mapping for dealing with barriers
// so that we account for the "hidden" depth aspect in emulated Stencil8.
pub(super) fn map_subresource_range_combined_aspect(
    range: &wgt::ImageSubresourceRange,
    format: wgt::TextureFormat,
    private_caps: &super::PrivateCapabilities,
) -> vk::ImageSubresourceRange {
    let mut range = map_subresource_range(range, format);
    if !private_caps.texture_s8 && format == wgt::TextureFormat::Stencil8 {
        range.aspect_mask |= vk::ImageAspectFlags::DEPTH;
    }
    range
}

pub fn map_subresource_layers(
    base: &crate::TextureCopyBase,
) -> (vk::ImageSubresourceLayers, vk::Offset3D) {
    let offset = vk::Offset3D {
        x: base.origin.x as i32,
        y: base.origin.y as i32,
        z: base.origin.z as i32,
    };
    let subresource = vk::ImageSubresourceLayers {
        aspect_mask: map_aspects(base.aspect),
        mip_level: base.mip_level,
        base_array_layer: base.array_layer,
        layer_count: 1,
    };
    (subresource, offset)
}

pub fn map_filter_mode(mode: wgt::FilterMode) -> vk::Filter {
    match mode {
        wgt::FilterMode::Nearest => vk::Filter::NEAREST,
        wgt::FilterMode::Linear => vk::Filter::LINEAR,
    }
}

pub fn map_mip_filter_mode(mode: wgt::MipmapFilterMode) -> vk::SamplerMipmapMode {
    match mode {
        wgt::MipmapFilterMode::Nearest => vk::SamplerMipmapMode::NEAREST,
        wgt::MipmapFilterMode::Linear => vk::SamplerMipmapMode::LINEAR,
    }
}

pub fn map_address_mode(mode: wgt::AddressMode) -> vk::SamplerAddressMode {
    match mode {
        wgt::AddressMode::ClampToEdge => vk::SamplerAddressMode::CLAMP_TO_EDGE,
        wgt::AddressMode::Repeat => vk::SamplerAddressMode::REPEAT,
        wgt::AddressMode::MirrorRepeat => vk::SamplerAddressMode::MIRRORED_REPEAT,
        wgt::AddressMode::ClampToBorder => vk::SamplerAddressMode::CLAMP_TO_BORDER,
        // wgt::AddressMode::MirrorClamp => vk::SamplerAddressMode::MIRROR_CLAMP_TO_EDGE,
    }
}

pub fn map_border_color(border_color: wgt::SamplerBorderColor) -> vk::BorderColor {
    match border_color {
        wgt::SamplerBorderColor::TransparentBlack | wgt::SamplerBorderColor::Zero => {
            vk::BorderColor::FLOAT_TRANSPARENT_BLACK
        }
        wgt::SamplerBorderColor::OpaqueBlack => vk::BorderColor::FLOAT_OPAQUE_BLACK,
        wgt::SamplerBorderColor::OpaqueWhite => vk::BorderColor::FLOAT_OPAQUE_WHITE,
    }
}

pub fn map_comparison(fun: wgt::CompareFunction) -> vk::CompareOp {
    use wgt::CompareFunction as Cf;
    match fun {
        Cf::Never => vk::CompareOp::NEVER,
        Cf::Less => vk::CompareOp::LESS,
        Cf::LessEqual => vk::CompareOp::LESS_OR_EQUAL,
        Cf::Equal => vk::CompareOp::EQUAL,
        Cf::GreaterEqual => vk::CompareOp::GREATER_OR_EQUAL,
        Cf::Greater => vk::CompareOp::GREATER,
        Cf::NotEqual => vk::CompareOp::NOT_EQUAL,
        Cf::Always => vk::CompareOp::ALWAYS,
    }
}

pub fn map_shader_stage(stage: wgt::ShaderStages) -> vk::ShaderStageFlags {
    let mut flags = vk::ShaderStageFlags::empty();
    if stage.contains(wgt::ShaderStages::VERTEX) {
        flags |= vk::ShaderStageFlags::VERTEX;
    }
    if stage.contains(wgt::ShaderStages::FRAGMENT) {
        flags |= vk::ShaderStageFlags::FRAGMENT;
    }
    if stage.contains(wgt::ShaderStages::COMPUTE) {
        flags |= vk::ShaderStageFlags::COMPUTE;
    }
    if stage.contains(wgt::ShaderStages::TASK) {
        flags |= vk::ShaderStageFlags::TASK_EXT;
    }
    if stage.contains(wgt::ShaderStages::MESH) {
        flags |= vk::ShaderStageFlags::MESH_EXT;
    }
    flags
}

pub fn map_binding_type(ty: wgt::BindingType) -> vk::DescriptorType {
    match ty {
        wgt::BindingType::Buffer {
            ty,
            has_dynamic_offset,
            ..
        } => match ty {
            wgt::BufferBindingType::Storage { .. } => match has_dynamic_offset {
                true => vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
                false => vk::DescriptorType::STORAGE_BUFFER,
            },
            wgt::BufferBindingType::Uniform => match has_dynamic_offset {
                true => vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
                false => vk::DescriptorType::UNIFORM_BUFFER,
            },
        },
        wgt::BindingType::Sampler { .. } => vk::DescriptorType::SAMPLER,
        wgt::BindingType::Texture { .. } => vk::DescriptorType::SAMPLED_IMAGE,
        wgt::BindingType::StorageTexture { .. } => vk::DescriptorType::STORAGE_IMAGE,
        wgt::BindingType::AccelerationStructure { .. } => {
            vk::DescriptorType::ACCELERATION_STRUCTURE_KHR
        }
        wgt::BindingType::ExternalTexture => unimplemented!(),
    }
}

pub fn map_topology(topology: wgt::PrimitiveTopology) -> vk::PrimitiveTopology {
    use wgt::PrimitiveTopology as Pt;
    match topology {
        Pt::PointList => vk::PrimitiveTopology::POINT_LIST,
        Pt::LineList => vk::PrimitiveTopology::LINE_LIST,
        Pt::LineStrip => vk::PrimitiveTopology::LINE_STRIP,
        Pt::TriangleList => vk::PrimitiveTopology::TRIANGLE_LIST,
        Pt::TriangleStrip => vk::PrimitiveTopology::TRIANGLE_STRIP,
    }
}

pub fn map_polygon_mode(mode: wgt::PolygonMode) -> vk::PolygonMode {
    match mode {
        wgt::PolygonMode::Fill => vk::PolygonMode::FILL,
        wgt::PolygonMode::Line => vk::PolygonMode::LINE,
        wgt::PolygonMode::Point => vk::PolygonMode::POINT,
    }
}

pub fn map_front_face(front_face: wgt::FrontFace) -> vk::FrontFace {
    match front_face {
        wgt::FrontFace::Cw => vk::FrontFace::CLOCKWISE,
        wgt::FrontFace::Ccw => vk::FrontFace::COUNTER_CLOCKWISE,
    }
}

pub fn map_cull_face(face: wgt::Face) -> vk::CullModeFlags {
    match face {
        wgt::Face::Front => vk::CullModeFlags::FRONT,
        wgt::Face::Back => vk::CullModeFlags::BACK,
    }
}

pub fn map_stencil_op(op: wgt::StencilOperation) -> vk::StencilOp {
    use wgt::StencilOperation as So;
    match op {
        So::Keep => vk::StencilOp::KEEP,
        So::Zero => vk::StencilOp::ZERO,
        So::Replace => vk::StencilOp::REPLACE,
        So::Invert => vk::StencilOp::INVERT,
        So::IncrementClamp => vk::StencilOp::INCREMENT_AND_CLAMP,
        So::IncrementWrap => vk::StencilOp::INCREMENT_AND_WRAP,
        So::DecrementClamp => vk::StencilOp::DECREMENT_AND_CLAMP,
        So::DecrementWrap => vk::StencilOp::DECREMENT_AND_WRAP,
    }
}

pub fn map_stencil_face(
    face: &wgt::StencilFaceState,
    compare_mask: u32,
    write_mask: u32,
) -> vk::StencilOpState {
    vk::StencilOpState {
        fail_op: map_stencil_op(face.fail_op),
        pass_op: map_stencil_op(face.pass_op),
        depth_fail_op: map_stencil_op(face.depth_fail_op),
        compare_op: map_comparison(face.compare),
        compare_mask,
        write_mask,
        reference: 0,
    }
}

fn map_blend_factor(factor: wgt::BlendFactor) -> vk::BlendFactor {
    use wgt::BlendFactor as Bf;
    match factor {
        Bf::Zero => vk::BlendFactor::ZERO,
        Bf::One => vk::BlendFactor::ONE,
        Bf::Src => vk::BlendFactor::SRC_COLOR,
        Bf::OneMinusSrc => vk::BlendFactor::ONE_MINUS_SRC_COLOR,
        Bf::SrcAlpha => vk::BlendFactor::SRC_ALPHA,
        Bf::OneMinusSrcAlpha => vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        Bf::Dst => vk::BlendFactor::DST_COLOR,
        Bf::OneMinusDst => vk::BlendFactor::ONE_MINUS_DST_COLOR,
        Bf::DstAlpha => vk::BlendFactor::DST_ALPHA,
        Bf::OneMinusDstAlpha => vk::BlendFactor::ONE_MINUS_DST_ALPHA,
        Bf::SrcAlphaSaturated => vk::BlendFactor::SRC_ALPHA_SATURATE,
        Bf::Constant => vk::BlendFactor::CONSTANT_COLOR,
        Bf::OneMinusConstant => vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR,
        Bf::Src1 => vk::BlendFactor::SRC1_COLOR,
        Bf::OneMinusSrc1 => vk::BlendFactor::ONE_MINUS_SRC1_COLOR,
        Bf::Src1Alpha => vk::BlendFactor::SRC1_ALPHA,
        Bf::OneMinusSrc1Alpha => vk::BlendFactor::ONE_MINUS_SRC1_ALPHA,
    }
}

fn map_blend_op(operation: wgt::BlendOperation) -> vk::BlendOp {
    use wgt::BlendOperation as Bo;
    match operation {
        Bo::Add => vk::BlendOp::ADD,
        Bo::Subtract => vk::BlendOp::SUBTRACT,
        Bo::ReverseSubtract => vk::BlendOp::REVERSE_SUBTRACT,
        Bo::Min => vk::BlendOp::MIN,
        Bo::Max => vk::BlendOp::MAX,
    }
}

pub fn map_blend_component(
    component: &wgt::BlendComponent,
) -> (vk::BlendOp, vk::BlendFactor, vk::BlendFactor) {
    let op = map_blend_op(component.operation);
    let src = map_blend_factor(component.src_factor);
    let dst = map_blend_factor(component.dst_factor);
    (op, src, dst)
}

pub fn map_pipeline_statistics(
    types: wgt::PipelineStatisticsTypes,
) -> vk::QueryPipelineStatisticFlags {
    use wgt::PipelineStatisticsTypes as Pst;
    let mut flags = vk::QueryPipelineStatisticFlags::empty();
    if types.contains(Pst::VERTEX_SHADER_INVOCATIONS) {
        flags |= vk::QueryPipelineStatisticFlags::VERTEX_SHADER_INVOCATIONS;
    }
    if types.contains(Pst::CLIPPER_INVOCATIONS) {
        flags |= vk::QueryPipelineStatisticFlags::CLIPPING_INVOCATIONS;
    }
    if types.contains(Pst::CLIPPER_PRIMITIVES_OUT) {
        flags |= vk::QueryPipelineStatisticFlags::CLIPPING_PRIMITIVES;
    }
    if types.contains(Pst::FRAGMENT_SHADER_INVOCATIONS) {
        flags |= vk::QueryPipelineStatisticFlags::FRAGMENT_SHADER_INVOCATIONS;
    }
    if types.contains(Pst::COMPUTE_SHADER_INVOCATIONS) {
        flags |= vk::QueryPipelineStatisticFlags::COMPUTE_SHADER_INVOCATIONS;
    }
    flags
}

pub fn map_acceleration_structure_format(
    format: crate::AccelerationStructureFormat,
) -> vk::AccelerationStructureTypeKHR {
    match format {
        crate::AccelerationStructureFormat::TopLevel => vk::AccelerationStructureTypeKHR::TOP_LEVEL,
        crate::AccelerationStructureFormat::BottomLevel => {
            vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL
        }
    }
}

pub fn map_acceleration_structure_build_mode(
    format: crate::AccelerationStructureBuildMode,
) -> vk::BuildAccelerationStructureModeKHR {
    match format {
        crate::AccelerationStructureBuildMode::Build => {
            vk::BuildAccelerationStructureModeKHR::BUILD
        }
        crate::AccelerationStructureBuildMode::Update => {
            vk::BuildAccelerationStructureModeKHR::UPDATE
        }
    }
}

pub fn map_acceleration_structure_flags(
    flags: crate::AccelerationStructureBuildFlags,
) -> vk::BuildAccelerationStructureFlagsKHR {
    let mut vk_flags = vk::BuildAccelerationStructureFlagsKHR::empty();

    if flags.contains(crate::AccelerationStructureBuildFlags::PREFER_FAST_TRACE) {
        vk_flags |= vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE;
    }

    if flags.contains(crate::AccelerationStructureBuildFlags::PREFER_FAST_BUILD) {
        vk_flags |= vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_BUILD;
    }

    if flags.contains(crate::AccelerationStructureBuildFlags::ALLOW_UPDATE) {
        vk_flags |= vk::BuildAccelerationStructureFlagsKHR::ALLOW_UPDATE;
    }

    if flags.contains(crate::AccelerationStructureBuildFlags::LOW_MEMORY) {
        vk_flags |= vk::BuildAccelerationStructureFlagsKHR::LOW_MEMORY;
    }

    if flags.contains(crate::AccelerationStructureBuildFlags::ALLOW_COMPACTION) {
        vk_flags |= vk::BuildAccelerationStructureFlagsKHR::ALLOW_COMPACTION
    }

    if flags.contains(crate::AccelerationStructureBuildFlags::ALLOW_RAY_HIT_VERTEX_RETURN) {
        vk_flags |= vk::BuildAccelerationStructureFlagsKHR::ALLOW_DATA_ACCESS
    }

    vk_flags
}

pub fn map_acceleration_structure_geometry_flags(
    flags: crate::AccelerationStructureGeometryFlags,
) -> vk::GeometryFlagsKHR {
    let mut vk_flags = vk::GeometryFlagsKHR::empty();

    if flags.contains(crate::AccelerationStructureGeometryFlags::OPAQUE) {
        vk_flags |= vk::GeometryFlagsKHR::OPAQUE;
    }

    if flags.contains(crate::AccelerationStructureGeometryFlags::NO_DUPLICATE_ANY_HIT_INVOCATION) {
        vk_flags |= vk::GeometryFlagsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION;
    }

    vk_flags
}

pub fn map_acceleration_structure_usage_to_barrier(
    usage: crate::AccelerationStructureUses,
    features: wgt::Features,
) -> (vk::PipelineStageFlags, vk::AccessFlags) {
    let mut stages = vk::PipelineStageFlags::empty();
    let mut access = vk::AccessFlags::empty();

    if usage.contains(crate::AccelerationStructureUses::BUILD_INPUT) {
        stages |= vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR;
        access |= vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR;
    }
    if usage.contains(crate::AccelerationStructureUses::QUERY_INPUT) {
        stages |= vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR;
        access |= vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR;
    }
    if usage.contains(crate::AccelerationStructureUses::BUILD_OUTPUT) {
        stages |= vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR;
        access |= vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR;
    }
    if usage.contains(crate::AccelerationStructureUses::SHADER_INPUT)
        && features.contains(wgt::Features::EXPERIMENTAL_RAY_QUERY)
    {
        stages |= vk::PipelineStageFlags::VERTEX_SHADER
            | vk::PipelineStageFlags::FRAGMENT_SHADER
            | vk::PipelineStageFlags::COMPUTE_SHADER;
        access |= vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR;
    }
    if usage.contains(crate::AccelerationStructureUses::COPY_SRC) {
        stages |= vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR;
        access |= vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR;
    }
    if usage.contains(crate::AccelerationStructureUses::COPY_DST) {
        stages |= vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR;
        access |= vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR;
    }

    (stages, access)
}
