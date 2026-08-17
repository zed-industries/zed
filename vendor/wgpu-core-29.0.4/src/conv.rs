use wgt::TextureFormatFeatures;

use crate::resource::{self, TextureDescriptor};

// Some core-only texture format helpers. The helper methods on `TextureFormat`
// defined in wgpu-types may need to be modified along with the ones here.

#[cfg_attr(any(not(webgl)), expect(unused))]
pub fn is_valid_external_image_copy_dst_texture_format(format: wgt::TextureFormat) -> bool {
    use wgt::TextureFormat as Tf;
    match format {
        Tf::R8Unorm
        | Tf::R16Float
        | Tf::R32Float
        | Tf::Rg8Unorm
        | Tf::Rg16Float
        | Tf::Rg32Float
        | Tf::Rgba8Unorm
        | Tf::Rgba8UnormSrgb
        | Tf::Bgra8Unorm
        | Tf::Bgra8UnormSrgb
        | Tf::Rgb10a2Unorm
        | Tf::Rgba16Float
        | Tf::Rgba32Float => true,
        _ => false,
    }
}

pub fn map_buffer_usage(usage: wgt::BufferUsages) -> wgt::BufferUses {
    let mut u = wgt::BufferUses::empty();
    u.set(
        wgt::BufferUses::MAP_READ,
        usage.contains(wgt::BufferUsages::MAP_READ),
    );
    u.set(
        wgt::BufferUses::MAP_WRITE,
        usage.contains(wgt::BufferUsages::MAP_WRITE),
    );
    u.set(
        wgt::BufferUses::COPY_SRC,
        usage.contains(wgt::BufferUsages::COPY_SRC),
    );
    u.set(
        wgt::BufferUses::COPY_DST,
        usage.contains(wgt::BufferUsages::COPY_DST),
    );
    u.set(
        wgt::BufferUses::INDEX,
        usage.contains(wgt::BufferUsages::INDEX),
    );
    u.set(
        wgt::BufferUses::VERTEX,
        usage.contains(wgt::BufferUsages::VERTEX),
    );
    u.set(
        wgt::BufferUses::UNIFORM,
        usage.contains(wgt::BufferUsages::UNIFORM),
    );
    u.set(
        wgt::BufferUses::STORAGE_READ_ONLY | wgt::BufferUses::STORAGE_READ_WRITE,
        usage.contains(wgt::BufferUsages::STORAGE),
    );
    u.set(
        wgt::BufferUses::INDIRECT,
        usage.contains(wgt::BufferUsages::INDIRECT),
    );
    u.set(
        wgt::BufferUses::QUERY_RESOLVE,
        usage.contains(wgt::BufferUsages::QUERY_RESOLVE),
    );
    u.set(
        wgt::BufferUses::BOTTOM_LEVEL_ACCELERATION_STRUCTURE_INPUT,
        usage.contains(wgt::BufferUsages::BLAS_INPUT),
    );
    u.set(
        wgt::BufferUses::TOP_LEVEL_ACCELERATION_STRUCTURE_INPUT,
        usage.contains(wgt::BufferUsages::TLAS_INPUT),
    );
    u
}

pub fn map_texture_usage(
    usage: wgt::TextureUsages,
    aspect: hal::FormatAspects,
    flags: wgt::TextureFormatFeatureFlags,
) -> wgt::TextureUses {
    let mut u = wgt::TextureUses::empty();
    u.set(
        wgt::TextureUses::COPY_SRC,
        usage.contains(wgt::TextureUsages::COPY_SRC),
    );
    u.set(
        wgt::TextureUses::COPY_DST,
        usage.contains(wgt::TextureUsages::COPY_DST),
    );
    u.set(
        wgt::TextureUses::RESOURCE,
        usage.contains(wgt::TextureUsages::TEXTURE_BINDING),
    );
    if usage.contains(wgt::TextureUsages::STORAGE_BINDING) {
        u.set(
            wgt::TextureUses::STORAGE_READ_ONLY,
            flags.contains(wgt::TextureFormatFeatureFlags::STORAGE_READ_ONLY),
        );
        u.set(
            wgt::TextureUses::STORAGE_WRITE_ONLY,
            flags.contains(wgt::TextureFormatFeatureFlags::STORAGE_WRITE_ONLY),
        );
        u.set(
            wgt::TextureUses::STORAGE_READ_WRITE,
            flags.contains(wgt::TextureFormatFeatureFlags::STORAGE_READ_WRITE),
        );
    }
    let is_color = aspect.intersects(
        hal::FormatAspects::COLOR
            | hal::FormatAspects::PLANE_0
            | hal::FormatAspects::PLANE_1
            | hal::FormatAspects::PLANE_2,
    );
    u.set(
        wgt::TextureUses::COLOR_TARGET,
        usage.contains(wgt::TextureUsages::RENDER_ATTACHMENT) && is_color,
    );
    u.set(
        wgt::TextureUses::DEPTH_STENCIL_READ | wgt::TextureUses::DEPTH_STENCIL_WRITE,
        usage.contains(wgt::TextureUsages::RENDER_ATTACHMENT) && !is_color,
    );
    u.set(
        wgt::TextureUses::STORAGE_ATOMIC,
        usage.contains(wgt::TextureUsages::STORAGE_ATOMIC),
    );
    u.set(
        wgt::TextureUses::TRANSIENT,
        usage.contains(wgt::TextureUsages::TRANSIENT),
    );
    u
}

pub fn map_texture_usage_for_texture(
    desc: &TextureDescriptor,
    format_features: &TextureFormatFeatures,
) -> wgt::TextureUses {
    // Enforce having COPY_DST/DEPTH_STENCIL_WRITE/COLOR_TARGET otherwise we
    // wouldn't be able to initialize the texture.
    map_texture_usage(desc.usage, desc.format.into(), format_features.flags)
        | if desc.format.is_depth_stencil_format() {
            wgt::TextureUses::DEPTH_STENCIL_WRITE
        } else if desc.usage.contains(wgt::TextureUsages::COPY_DST) {
            wgt::TextureUses::COPY_DST // (set already)
        } else {
            // Use COPY_DST only if we can't use COLOR_TARGET
            if format_features
                .allowed_usages
                .contains(wgt::TextureUsages::RENDER_ATTACHMENT)
                && desc.dimension == wgt::TextureDimension::D2
            // Render targets dimension must be 2d
            {
                wgt::TextureUses::COLOR_TARGET
            } else {
                wgt::TextureUses::COPY_DST
            }
        }
}

pub fn map_texture_usage_from_hal(uses: wgt::TextureUses) -> wgt::TextureUsages {
    let mut u = wgt::TextureUsages::empty();
    u.set(
        wgt::TextureUsages::COPY_SRC,
        uses.contains(wgt::TextureUses::COPY_SRC),
    );
    u.set(
        wgt::TextureUsages::COPY_DST,
        uses.contains(wgt::TextureUses::COPY_DST),
    );
    u.set(
        wgt::TextureUsages::TEXTURE_BINDING,
        uses.contains(wgt::TextureUses::RESOURCE),
    );
    u.set(
        wgt::TextureUsages::STORAGE_BINDING,
        uses.intersects(
            wgt::TextureUses::STORAGE_READ_ONLY
                | wgt::TextureUses::STORAGE_WRITE_ONLY
                | wgt::TextureUses::STORAGE_READ_WRITE,
        ),
    );
    u.set(
        wgt::TextureUsages::RENDER_ATTACHMENT,
        uses.contains(wgt::TextureUses::COLOR_TARGET),
    );
    u.set(
        wgt::TextureUsages::STORAGE_ATOMIC,
        uses.contains(wgt::TextureUses::STORAGE_ATOMIC),
    );
    u.set(
        wgt::TextureUsages::TRANSIENT,
        uses.contains(wgt::TextureUses::TRANSIENT),
    );
    u
}

/// Check the requested texture size against the supported limits.
///
/// This function implements the texture size and sample count checks in [vtd
/// dimension step]. The format checks are elsewhere in [`create_texture`]`.
///
/// Note that while there is some basic checking of the sample count here, there
/// is an additional set of checks when `sample_count > 1` elsewhere in
/// [`create_texture`]`.
///
/// [vtd dimension step]: https://www.w3.org/TR/2025/CRD-webgpu-20251120/#:~:text=or%204.-,If%20descriptor.dimension%20is
/// [`create_texture`]: crate::device::Device::create_texture
pub fn check_texture_dimension_size(
    dimension: wgt::TextureDimension,
    wgt::Extent3d {
        width,
        height,
        depth_or_array_layers,
    }: wgt::Extent3d,
    sample_size: u32,
    limits: &wgt::Limits,
) -> Result<(), resource::TextureDimensionError> {
    use resource::{TextureDimensionError as Tde, TextureErrorDimension as Ted};
    use wgt::TextureDimension::*;

    let (extent_limits, sample_limit) = match dimension {
        D1 => ([limits.max_texture_dimension_1d, 1, 1], 1),
        D2 => (
            [
                limits.max_texture_dimension_2d,
                limits.max_texture_dimension_2d,
                limits.max_texture_array_layers,
            ],
            32,
        ),
        D3 => (
            [
                limits.max_texture_dimension_3d,
                limits.max_texture_dimension_3d,
                limits.max_texture_dimension_3d,
            ],
            1,
        ),
    };

    for (&dim, (&given, &limit)) in [Ted::X, Ted::Y, Ted::Z].iter().zip(
        [width, height, depth_or_array_layers]
            .iter()
            .zip(extent_limits.iter()),
    ) {
        if given == 0 {
            return Err(Tde::Zero(dim));
        }
        if given > limit {
            return Err(Tde::LimitExceeded { dim, given, limit });
        }
    }
    if sample_size == 0 || sample_size > sample_limit || !sample_size.is_power_of_two() {
        return Err(Tde::InvalidSampleCount(sample_size));
    }

    Ok(())
}

pub fn bind_group_layout_flags(features: wgt::Features) -> hal::BindGroupLayoutFlags {
    let mut flags = hal::BindGroupLayoutFlags::empty();
    flags.set(
        hal::BindGroupLayoutFlags::PARTIALLY_BOUND,
        features.contains(wgt::Features::PARTIALLY_BOUND_BINDING_ARRAY),
    );
    flags
}
