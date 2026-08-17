use alloc::string::String;
use std::{ffi::OsString, os::windows::ffi::OsStringExt};

use windows::Win32::Graphics::Dxgi;

// Helper to convert DXGI adapter name to a normal string
pub fn map_adapter_name(name: [u16; 128]) -> String {
    let len = name.iter().take_while(|&&c| c != 0).count();
    let name = OsString::from_wide(&name[..len]);
    name.to_string_lossy().into_owned()
}

pub fn map_texture_format_failable(
    format: wgt::TextureFormat,
) -> Option<Dxgi::Common::DXGI_FORMAT> {
    use wgt::TextureFormat as Tf;
    use Dxgi::Common::*;

    Some(match format {
        Tf::R8Unorm => DXGI_FORMAT_R8_UNORM,
        Tf::R8Snorm => DXGI_FORMAT_R8_SNORM,
        Tf::R8Uint => DXGI_FORMAT_R8_UINT,
        Tf::R8Sint => DXGI_FORMAT_R8_SINT,
        Tf::R16Uint => DXGI_FORMAT_R16_UINT,
        Tf::R16Sint => DXGI_FORMAT_R16_SINT,
        Tf::R16Unorm => DXGI_FORMAT_R16_UNORM,
        Tf::R16Snorm => DXGI_FORMAT_R16_SNORM,
        Tf::R16Float => DXGI_FORMAT_R16_FLOAT,
        Tf::Rg8Unorm => DXGI_FORMAT_R8G8_UNORM,
        Tf::Rg8Snorm => DXGI_FORMAT_R8G8_SNORM,
        Tf::Rg8Uint => DXGI_FORMAT_R8G8_UINT,
        Tf::Rg8Sint => DXGI_FORMAT_R8G8_SINT,
        Tf::Rg16Unorm => DXGI_FORMAT_R16G16_UNORM,
        Tf::Rg16Snorm => DXGI_FORMAT_R16G16_SNORM,
        Tf::R32Uint => DXGI_FORMAT_R32_UINT,
        Tf::R32Sint => DXGI_FORMAT_R32_SINT,
        Tf::R32Float => DXGI_FORMAT_R32_FLOAT,
        Tf::Rg16Uint => DXGI_FORMAT_R16G16_UINT,
        Tf::Rg16Sint => DXGI_FORMAT_R16G16_SINT,
        Tf::Rg16Float => DXGI_FORMAT_R16G16_FLOAT,
        Tf::Rgba8Unorm => DXGI_FORMAT_R8G8B8A8_UNORM,
        Tf::Rgba8UnormSrgb => DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
        Tf::Bgra8UnormSrgb => DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
        Tf::Rgba8Snorm => DXGI_FORMAT_R8G8B8A8_SNORM,
        Tf::Bgra8Unorm => DXGI_FORMAT_B8G8R8A8_UNORM,
        Tf::Rgba8Uint => DXGI_FORMAT_R8G8B8A8_UINT,
        Tf::Rgba8Sint => DXGI_FORMAT_R8G8B8A8_SINT,
        Tf::Rgb9e5Ufloat => DXGI_FORMAT_R9G9B9E5_SHAREDEXP,
        Tf::Rgb10a2Uint => DXGI_FORMAT_R10G10B10A2_UINT,
        Tf::Rgb10a2Unorm => DXGI_FORMAT_R10G10B10A2_UNORM,
        Tf::Rg11b10Ufloat => DXGI_FORMAT_R11G11B10_FLOAT,
        Tf::R64Uint => DXGI_FORMAT_R32G32_UINT, // R64 emulated by R32G32
        Tf::Rg32Uint => DXGI_FORMAT_R32G32_UINT,
        Tf::Rg32Sint => DXGI_FORMAT_R32G32_SINT,
        Tf::Rg32Float => DXGI_FORMAT_R32G32_FLOAT,
        Tf::Rgba16Uint => DXGI_FORMAT_R16G16B16A16_UINT,
        Tf::Rgba16Sint => DXGI_FORMAT_R16G16B16A16_SINT,
        Tf::Rgba16Unorm => DXGI_FORMAT_R16G16B16A16_UNORM,
        Tf::Rgba16Snorm => DXGI_FORMAT_R16G16B16A16_SNORM,
        Tf::Rgba16Float => DXGI_FORMAT_R16G16B16A16_FLOAT,
        Tf::Rgba32Uint => DXGI_FORMAT_R32G32B32A32_UINT,
        Tf::Rgba32Sint => DXGI_FORMAT_R32G32B32A32_SINT,
        Tf::Rgba32Float => DXGI_FORMAT_R32G32B32A32_FLOAT,
        Tf::Stencil8 => DXGI_FORMAT_D24_UNORM_S8_UINT,
        Tf::Depth16Unorm => DXGI_FORMAT_D16_UNORM,
        Tf::Depth24Plus => DXGI_FORMAT_D24_UNORM_S8_UINT,
        Tf::Depth24PlusStencil8 => DXGI_FORMAT_D24_UNORM_S8_UINT,
        Tf::Depth32Float => DXGI_FORMAT_D32_FLOAT,
        Tf::Depth32FloatStencil8 => DXGI_FORMAT_D32_FLOAT_S8X24_UINT,
        Tf::NV12 => DXGI_FORMAT_NV12,
        Tf::P010 => DXGI_FORMAT_P010,
        Tf::Bc1RgbaUnorm => DXGI_FORMAT_BC1_UNORM,
        Tf::Bc1RgbaUnormSrgb => DXGI_FORMAT_BC1_UNORM_SRGB,
        Tf::Bc2RgbaUnorm => DXGI_FORMAT_BC2_UNORM,
        Tf::Bc2RgbaUnormSrgb => DXGI_FORMAT_BC2_UNORM_SRGB,
        Tf::Bc3RgbaUnorm => DXGI_FORMAT_BC3_UNORM,
        Tf::Bc3RgbaUnormSrgb => DXGI_FORMAT_BC3_UNORM_SRGB,
        Tf::Bc4RUnorm => DXGI_FORMAT_BC4_UNORM,
        Tf::Bc4RSnorm => DXGI_FORMAT_BC4_SNORM,
        Tf::Bc5RgUnorm => DXGI_FORMAT_BC5_UNORM,
        Tf::Bc5RgSnorm => DXGI_FORMAT_BC5_SNORM,
        Tf::Bc6hRgbUfloat => DXGI_FORMAT_BC6H_UF16,
        Tf::Bc6hRgbFloat => DXGI_FORMAT_BC6H_SF16,
        Tf::Bc7RgbaUnorm => DXGI_FORMAT_BC7_UNORM,
        Tf::Bc7RgbaUnormSrgb => DXGI_FORMAT_BC7_UNORM_SRGB,
        Tf::Etc2Rgb8Unorm
        | Tf::Etc2Rgb8UnormSrgb
        | Tf::Etc2Rgb8A1Unorm
        | Tf::Etc2Rgb8A1UnormSrgb
        | Tf::Etc2Rgba8Unorm
        | Tf::Etc2Rgba8UnormSrgb
        | Tf::EacR11Unorm
        | Tf::EacR11Snorm
        | Tf::EacRg11Unorm
        | Tf::EacRg11Snorm
        | Tf::Astc {
            block: _,
            channel: _,
        } => return None,
    })
}

pub fn map_texture_format(format: wgt::TextureFormat) -> Dxgi::Common::DXGI_FORMAT {
    match map_texture_format_failable(format) {
        Some(f) => f,
        None => unreachable!(),
    }
}

// Note: DXGI doesn't allow sRGB format on the swapchain,
// but creating RTV of swapchain buffers with sRGB works.
pub fn map_texture_format_nosrgb(format: wgt::TextureFormat) -> Dxgi::Common::DXGI_FORMAT {
    match format {
        wgt::TextureFormat::Bgra8UnormSrgb => Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM,
        wgt::TextureFormat::Rgba8UnormSrgb => Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM,
        _ => map_texture_format(format),
    }
}

// SRV and UAV can't use the depth or typeless formats
// see https://microsoft.github.io/DirectX-Specs/d3d/PlanarDepthStencilDDISpec.html#view-creation
pub fn map_texture_format_for_srv_uav(
    format: wgt::TextureFormat,
    aspect: crate::FormatAspects,
) -> Option<Dxgi::Common::DXGI_FORMAT> {
    Some(match (format, aspect) {
        (wgt::TextureFormat::Depth16Unorm, crate::FormatAspects::DEPTH) => {
            Dxgi::Common::DXGI_FORMAT_R16_UNORM
        }
        (wgt::TextureFormat::Depth32Float, crate::FormatAspects::DEPTH) => {
            Dxgi::Common::DXGI_FORMAT_R32_FLOAT
        }
        (wgt::TextureFormat::Depth32FloatStencil8, crate::FormatAspects::DEPTH) => {
            Dxgi::Common::DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS
        }
        (
            wgt::TextureFormat::Depth24Plus | wgt::TextureFormat::Depth24PlusStencil8,
            crate::FormatAspects::DEPTH,
        ) => Dxgi::Common::DXGI_FORMAT_R24_UNORM_X8_TYPELESS,

        (wgt::TextureFormat::Depth32FloatStencil8, crate::FormatAspects::STENCIL) => {
            Dxgi::Common::DXGI_FORMAT_X32_TYPELESS_G8X24_UINT
        }
        (
            wgt::TextureFormat::Stencil8 | wgt::TextureFormat::Depth24PlusStencil8,
            crate::FormatAspects::STENCIL,
        ) => Dxgi::Common::DXGI_FORMAT_X24_TYPELESS_G8_UINT,

        (_, crate::FormatAspects::DEPTH)
        | (_, crate::FormatAspects::STENCIL)
        | (_, crate::FormatAspects::DEPTH_STENCIL) => return None,

        _ => map_texture_format(format),
    })
}

// see https://microsoft.github.io/DirectX-Specs/d3d/PlanarDepthStencilDDISpec.html#planar-layout-for-staging-from-buffer
pub fn map_texture_format_for_copy(
    format: wgt::TextureFormat,
    aspect: crate::FormatAspects,
) -> Option<Dxgi::Common::DXGI_FORMAT> {
    Some(match (format, aspect) {
        (wgt::TextureFormat::Depth16Unorm, crate::FormatAspects::DEPTH) => {
            Dxgi::Common::DXGI_FORMAT_R16_UNORM
        }
        (
            wgt::TextureFormat::Depth32Float | wgt::TextureFormat::Depth32FloatStencil8,
            crate::FormatAspects::DEPTH,
        ) => Dxgi::Common::DXGI_FORMAT_R32_FLOAT,

        (
            wgt::TextureFormat::Stencil8
            | wgt::TextureFormat::Depth24PlusStencil8
            | wgt::TextureFormat::Depth32FloatStencil8,
            crate::FormatAspects::STENCIL,
        ) => Dxgi::Common::DXGI_FORMAT_R8_UINT,

        (format, crate::FormatAspects::COLOR) => map_texture_format(format),

        _ => return None,
    })
}

pub fn map_texture_format_for_resource(
    format: wgt::TextureFormat,
    usage: wgt::TextureUses,
    has_view_formats: bool,
    casting_fully_typed_format_supported: bool,
) -> Dxgi::Common::DXGI_FORMAT {
    use wgt::TextureFormat as Tf;
    use Dxgi::Common::*;

    if casting_fully_typed_format_supported {
        map_texture_format(format)

    // We might view this resource as srgb or non-srgb
    } else if has_view_formats {
        match format {
            Tf::Rgba8Unorm | Tf::Rgba8UnormSrgb => DXGI_FORMAT_R8G8B8A8_TYPELESS,
            Tf::Bgra8Unorm | Tf::Bgra8UnormSrgb => DXGI_FORMAT_B8G8R8A8_TYPELESS,
            Tf::Bc1RgbaUnorm | Tf::Bc1RgbaUnormSrgb => DXGI_FORMAT_BC1_TYPELESS,
            Tf::Bc2RgbaUnorm | Tf::Bc2RgbaUnormSrgb => DXGI_FORMAT_BC2_TYPELESS,
            Tf::Bc3RgbaUnorm | Tf::Bc3RgbaUnormSrgb => DXGI_FORMAT_BC3_TYPELESS,
            Tf::Bc7RgbaUnorm | Tf::Bc7RgbaUnormSrgb => DXGI_FORMAT_BC7_TYPELESS,
            format => map_texture_format(format),
        }

    // We might view this resource as SRV/UAV but also as DSV
    } else if format.is_depth_stencil_format()
        && usage.intersects(
            wgt::TextureUses::RESOURCE
                | wgt::TextureUses::STORAGE_READ_ONLY
                | wgt::TextureUses::STORAGE_WRITE_ONLY
                | wgt::TextureUses::STORAGE_READ_WRITE,
        )
    {
        match format {
            Tf::Depth16Unorm => DXGI_FORMAT_R16_TYPELESS,
            Tf::Depth32Float => DXGI_FORMAT_R32_TYPELESS,
            Tf::Depth32FloatStencil8 => DXGI_FORMAT_R32G8X24_TYPELESS,
            Tf::Stencil8 | Tf::Depth24Plus | Tf::Depth24PlusStencil8 => DXGI_FORMAT_R24G8_TYPELESS,
            _ => unreachable!(),
        }
    } else {
        map_texture_format(format)
    }
}

pub fn map_index_format(format: wgt::IndexFormat) -> Dxgi::Common::DXGI_FORMAT {
    match format {
        wgt::IndexFormat::Uint16 => Dxgi::Common::DXGI_FORMAT_R16_UINT,
        wgt::IndexFormat::Uint32 => Dxgi::Common::DXGI_FORMAT_R32_UINT,
    }
}

pub fn map_vertex_format(format: wgt::VertexFormat) -> Dxgi::Common::DXGI_FORMAT {
    use wgt::VertexFormat as Vf;
    use Dxgi::Common::*;

    match format {
        Vf::Unorm8 => DXGI_FORMAT_R8_UNORM,
        Vf::Snorm8 => DXGI_FORMAT_R8_SNORM,
        Vf::Uint8 => DXGI_FORMAT_R8_UINT,
        Vf::Sint8 => DXGI_FORMAT_R8_SINT,
        Vf::Unorm8x2 => DXGI_FORMAT_R8G8_UNORM,
        Vf::Snorm8x2 => DXGI_FORMAT_R8G8_SNORM,
        Vf::Uint8x2 => DXGI_FORMAT_R8G8_UINT,
        Vf::Sint8x2 => DXGI_FORMAT_R8G8_SINT,
        Vf::Unorm8x4 => DXGI_FORMAT_R8G8B8A8_UNORM,
        Vf::Snorm8x4 => DXGI_FORMAT_R8G8B8A8_SNORM,
        Vf::Uint8x4 => DXGI_FORMAT_R8G8B8A8_UINT,
        Vf::Sint8x4 => DXGI_FORMAT_R8G8B8A8_SINT,
        Vf::Unorm16 => DXGI_FORMAT_R16_UNORM,
        Vf::Snorm16 => DXGI_FORMAT_R16_SNORM,
        Vf::Uint16 => DXGI_FORMAT_R16_UINT,
        Vf::Sint16 => DXGI_FORMAT_R16_SINT,
        Vf::Float16 => DXGI_FORMAT_R16_FLOAT,
        Vf::Unorm16x2 => DXGI_FORMAT_R16G16_UNORM,
        Vf::Snorm16x2 => DXGI_FORMAT_R16G16_SNORM,
        Vf::Uint16x2 => DXGI_FORMAT_R16G16_UINT,
        Vf::Sint16x2 => DXGI_FORMAT_R16G16_SINT,
        Vf::Float16x2 => DXGI_FORMAT_R16G16_FLOAT,
        Vf::Unorm16x4 => DXGI_FORMAT_R16G16B16A16_UNORM,
        Vf::Snorm16x4 => DXGI_FORMAT_R16G16B16A16_SNORM,
        Vf::Uint16x4 => DXGI_FORMAT_R16G16B16A16_UINT,
        Vf::Sint16x4 => DXGI_FORMAT_R16G16B16A16_SINT,
        Vf::Float16x4 => DXGI_FORMAT_R16G16B16A16_FLOAT,
        Vf::Uint32 => DXGI_FORMAT_R32_UINT,
        Vf::Sint32 => DXGI_FORMAT_R32_SINT,
        Vf::Float32 => DXGI_FORMAT_R32_FLOAT,
        Vf::Uint32x2 => DXGI_FORMAT_R32G32_UINT,
        Vf::Sint32x2 => DXGI_FORMAT_R32G32_SINT,
        Vf::Float32x2 => DXGI_FORMAT_R32G32_FLOAT,
        Vf::Uint32x3 => DXGI_FORMAT_R32G32B32_UINT,
        Vf::Sint32x3 => DXGI_FORMAT_R32G32B32_SINT,
        Vf::Float32x3 => DXGI_FORMAT_R32G32B32_FLOAT,
        Vf::Uint32x4 => DXGI_FORMAT_R32G32B32A32_UINT,
        Vf::Sint32x4 => DXGI_FORMAT_R32G32B32A32_SINT,
        Vf::Float32x4 => DXGI_FORMAT_R32G32B32A32_FLOAT,
        Vf::Unorm10_10_10_2 => DXGI_FORMAT_R10G10B10A2_UNORM,
        Vf::Unorm8x4Bgra => DXGI_FORMAT_B8G8R8A8_UNORM,
        Vf::Float64 | Vf::Float64x2 | Vf::Float64x3 | Vf::Float64x4 => unimplemented!(),
    }
}

pub fn map_acomposite_alpha_mode(mode: wgt::CompositeAlphaMode) -> Dxgi::Common::DXGI_ALPHA_MODE {
    match mode {
        wgt::CompositeAlphaMode::PreMultiplied => Dxgi::Common::DXGI_ALPHA_MODE_PREMULTIPLIED,
        wgt::CompositeAlphaMode::PostMultiplied => Dxgi::Common::DXGI_ALPHA_MODE_STRAIGHT,
        wgt::CompositeAlphaMode::Opaque => Dxgi::Common::DXGI_ALPHA_MODE_IGNORE,
        wgt::CompositeAlphaMode::Auto | wgt::CompositeAlphaMode::Inherit => {
            Dxgi::Common::DXGI_ALPHA_MODE_UNSPECIFIED
        }
    }
}
