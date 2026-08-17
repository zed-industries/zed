//! Conversions between [`naga`] and [`wgpu_types`].

#![no_std]

use naga::valid::Capabilities as Caps;
use wgpu_types as wgt;

/// Map [`wgt::Features`] and [`wgt::DownlevelFlags`] to [`naga::valid::Capabilities`].
pub fn features_to_naga_capabilities(
    features: wgt::Features,
    downlevel: wgt::DownlevelFlags,
) -> Caps {
    let mut caps = Caps::empty();
    caps.set(
        Caps::IMMEDIATES,
        features.contains(wgt::Features::IMMEDIATES),
    );
    caps.set(Caps::FLOAT64, features.contains(wgt::Features::SHADER_F64));
    caps.set(
        Caps::SHADER_FLOAT16,
        features.contains(wgt::Features::SHADER_F16),
    );
    caps.set(
        Caps::SHADER_FLOAT16_IN_FLOAT32,
        downlevel.contains(wgt::DownlevelFlags::SHADER_F16_IN_F32),
    );
    caps.set(
        Caps::PRIMITIVE_INDEX,
        features.contains(wgt::Features::PRIMITIVE_INDEX),
    );
    caps.set(
        Caps::TEXTURE_AND_SAMPLER_BINDING_ARRAY,
        features.contains(wgt::Features::TEXTURE_BINDING_ARRAY),
    );
    caps.set(
        Caps::BUFFER_BINDING_ARRAY,
        features.contains(wgt::Features::BUFFER_BINDING_ARRAY),
    );
    caps.set(
        Caps::STORAGE_TEXTURE_BINDING_ARRAY,
        features.contains(wgt::Features::TEXTURE_BINDING_ARRAY)
            && features.contains(wgt::Features::STORAGE_RESOURCE_BINDING_ARRAY),
    );
    caps.set(
        Caps::STORAGE_BUFFER_BINDING_ARRAY,
        features.contains(wgt::Features::BUFFER_BINDING_ARRAY)
            && features.contains(wgt::Features::STORAGE_RESOURCE_BINDING_ARRAY),
    );
    caps.set(
        Caps::TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING,
        features
            .contains(wgt::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING),
    );
    caps.set(
        Caps::BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING,
        features.contains(wgt::Features::UNIFORM_BUFFER_BINDING_ARRAYS),
    );
    caps.set(
        Caps::STORAGE_TEXTURE_BINDING_ARRAY_NON_UNIFORM_INDEXING,
        features.contains(wgt::Features::STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING),
    );
    caps.set(
        Caps::STORAGE_BUFFER_BINDING_ARRAY_NON_UNIFORM_INDEXING,
        features
            .contains(wgt::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING),
    );
    caps.set(
        Caps::ACCELERATION_STRUCTURE_BINDING_ARRAY,
        features.contains(wgt::Features::ACCELERATION_STRUCTURE_BINDING_ARRAY),
    );
    caps.set(
        Caps::STORAGE_TEXTURE_16BIT_NORM_FORMATS,
        features.contains(wgt::Features::TEXTURE_FORMAT_16BIT_NORM),
    );
    caps.set(Caps::MULTIVIEW, features.contains(wgt::Features::MULTIVIEW));
    caps.set(
        Caps::EARLY_DEPTH_TEST,
        features.contains(wgt::Features::SHADER_EARLY_DEPTH_TEST),
    );
    caps.set(
        Caps::SHADER_INT64,
        features.contains(wgt::Features::SHADER_INT64),
    );
    caps.set(
        Caps::SHADER_INT64_ATOMIC_MIN_MAX,
        features.intersects(
            wgt::Features::SHADER_INT64_ATOMIC_MIN_MAX | wgt::Features::SHADER_INT64_ATOMIC_ALL_OPS,
        ),
    );
    caps.set(
        Caps::SHADER_INT64_ATOMIC_ALL_OPS,
        features.contains(wgt::Features::SHADER_INT64_ATOMIC_ALL_OPS),
    );
    caps.set(
        Caps::TEXTURE_ATOMIC,
        features.contains(wgt::Features::TEXTURE_ATOMIC),
    );
    caps.set(
        Caps::TEXTURE_INT64_ATOMIC,
        features.contains(wgt::Features::TEXTURE_INT64_ATOMIC),
    );
    caps.set(
        Caps::SHADER_FLOAT32_ATOMIC,
        features.contains(wgt::Features::SHADER_FLOAT32_ATOMIC),
    );
    caps.set(
        Caps::MULTISAMPLED_SHADING,
        downlevel.contains(wgt::DownlevelFlags::MULTISAMPLED_SHADING),
    );
    caps.set(
        Caps::DUAL_SOURCE_BLENDING,
        features.contains(wgt::Features::DUAL_SOURCE_BLENDING),
    );
    caps.set(
        Caps::CLIP_DISTANCE,
        features.contains(wgt::Features::CLIP_DISTANCES),
    );
    caps.set(
        Caps::CUBE_ARRAY_TEXTURES,
        downlevel.contains(wgt::DownlevelFlags::CUBE_ARRAY_TEXTURES),
    );
    caps.set(
        Caps::SUBGROUP,
        features.intersects(wgt::Features::SUBGROUP | wgt::Features::SUBGROUP_VERTEX),
    );
    caps.set(
        Caps::SUBGROUP_BARRIER,
        features.intersects(wgt::Features::SUBGROUP_BARRIER),
    );
    caps.set(
        Caps::RAY_QUERY,
        features.intersects(wgt::Features::EXPERIMENTAL_RAY_QUERY),
    );
    caps.set(
        Caps::SUBGROUP_VERTEX_STAGE,
        features.contains(wgt::Features::SUBGROUP_VERTEX),
    );
    caps.set(
        Caps::RAY_HIT_VERTEX_POSITION,
        features.intersects(wgt::Features::EXPERIMENTAL_RAY_HIT_VERTEX_RETURN),
    );
    caps.set(
        Caps::TEXTURE_EXTERNAL,
        features.intersects(wgt::Features::EXTERNAL_TEXTURE),
    );
    caps.set(
        Caps::SHADER_BARYCENTRICS,
        features.intersects(wgt::Features::SHADER_BARYCENTRICS),
    );
    caps.set(
        Caps::MESH_SHADER,
        features.intersects(wgt::Features::EXPERIMENTAL_MESH_SHADER),
    );
    caps.set(
        Caps::MESH_SHADER_POINT_TOPOLOGY,
        features.intersects(wgt::Features::EXPERIMENTAL_MESH_SHADER_POINTS),
    );
    caps.set(
        Caps::COOPERATIVE_MATRIX,
        features.intersects(wgt::Features::EXPERIMENTAL_COOPERATIVE_MATRIX),
    );
    caps.set(
        Caps::PER_VERTEX,
        features.intersects(wgt::Features::SHADER_PER_VERTEX),
    );
    caps.set(
        Caps::DRAW_INDEX,
        features.intersects(wgt::Features::SHADER_DRAW_INDEX),
    );
    caps.set(
        Caps::MEMORY_DECORATION_COHERENT,
        features.contains(wgt::Features::MEMORY_DECORATION_COHERENT),
    );
    caps.set(
        Caps::MEMORY_DECORATION_VOLATILE,
        features.contains(wgt::Features::MEMORY_DECORATION_VOLATILE),
    );
    caps
}

/// Create a [`naga::valid::Validator`] configured for the given feature set.
pub fn create_validator(
    features: wgt::Features,
    downlevel: wgt::DownlevelFlags,
    flags: naga::valid::ValidationFlags,
) -> naga::valid::Validator {
    let caps = features_to_naga_capabilities(features, downlevel);
    naga::valid::Validator::new(flags, caps)
}

/// Map a [`wgt::TextureFormat`] to the corresponding [`naga::StorageFormat`], if any.
pub fn map_storage_format_to_naga(format: wgt::TextureFormat) -> Option<naga::StorageFormat> {
    use naga::StorageFormat as Sf;
    use wgt::TextureFormat as Tf;

    Some(match format {
        Tf::R8Unorm => Sf::R8Unorm,
        Tf::R8Snorm => Sf::R8Snorm,
        Tf::R8Uint => Sf::R8Uint,
        Tf::R8Sint => Sf::R8Sint,

        Tf::R16Uint => Sf::R16Uint,
        Tf::R16Sint => Sf::R16Sint,
        Tf::R16Float => Sf::R16Float,
        Tf::Rg8Unorm => Sf::Rg8Unorm,
        Tf::Rg8Snorm => Sf::Rg8Snorm,
        Tf::Rg8Uint => Sf::Rg8Uint,
        Tf::Rg8Sint => Sf::Rg8Sint,

        Tf::R32Uint => Sf::R32Uint,
        Tf::R32Sint => Sf::R32Sint,
        Tf::R32Float => Sf::R32Float,
        Tf::Rg16Uint => Sf::Rg16Uint,
        Tf::Rg16Sint => Sf::Rg16Sint,
        Tf::Rg16Float => Sf::Rg16Float,
        Tf::Rgba8Unorm => Sf::Rgba8Unorm,
        Tf::Rgba8Snorm => Sf::Rgba8Snorm,
        Tf::Rgba8Uint => Sf::Rgba8Uint,
        Tf::Rgba8Sint => Sf::Rgba8Sint,
        Tf::Bgra8Unorm => Sf::Bgra8Unorm,

        Tf::Rgb10a2Uint => Sf::Rgb10a2Uint,
        Tf::Rgb10a2Unorm => Sf::Rgb10a2Unorm,
        Tf::Rg11b10Ufloat => Sf::Rg11b10Ufloat,

        Tf::R64Uint => Sf::R64Uint,
        Tf::Rg32Uint => Sf::Rg32Uint,
        Tf::Rg32Sint => Sf::Rg32Sint,
        Tf::Rg32Float => Sf::Rg32Float,
        Tf::Rgba16Uint => Sf::Rgba16Uint,
        Tf::Rgba16Sint => Sf::Rgba16Sint,
        Tf::Rgba16Float => Sf::Rgba16Float,

        Tf::Rgba32Uint => Sf::Rgba32Uint,
        Tf::Rgba32Sint => Sf::Rgba32Sint,
        Tf::Rgba32Float => Sf::Rgba32Float,

        Tf::R16Unorm => Sf::R16Unorm,
        Tf::R16Snorm => Sf::R16Snorm,
        Tf::Rg16Unorm => Sf::Rg16Unorm,
        Tf::Rg16Snorm => Sf::Rg16Snorm,
        Tf::Rgba16Unorm => Sf::Rgba16Unorm,
        Tf::Rgba16Snorm => Sf::Rgba16Snorm,

        _ => return None,
    })
}

/// Map a [`naga::StorageFormat`] to the corresponding [`wgt::TextureFormat`].
pub fn map_storage_format_from_naga(format: naga::StorageFormat) -> wgt::TextureFormat {
    use naga::StorageFormat as Sf;
    use wgt::TextureFormat as Tf;

    match format {
        Sf::R8Unorm => Tf::R8Unorm,
        Sf::R8Snorm => Tf::R8Snorm,
        Sf::R8Uint => Tf::R8Uint,
        Sf::R8Sint => Tf::R8Sint,

        Sf::R16Uint => Tf::R16Uint,
        Sf::R16Sint => Tf::R16Sint,
        Sf::R16Float => Tf::R16Float,
        Sf::Rg8Unorm => Tf::Rg8Unorm,
        Sf::Rg8Snorm => Tf::Rg8Snorm,
        Sf::Rg8Uint => Tf::Rg8Uint,
        Sf::Rg8Sint => Tf::Rg8Sint,

        Sf::R32Uint => Tf::R32Uint,
        Sf::R32Sint => Tf::R32Sint,
        Sf::R32Float => Tf::R32Float,
        Sf::Rg16Uint => Tf::Rg16Uint,
        Sf::Rg16Sint => Tf::Rg16Sint,
        Sf::Rg16Float => Tf::Rg16Float,
        Sf::Rgba8Unorm => Tf::Rgba8Unorm,
        Sf::Rgba8Snorm => Tf::Rgba8Snorm,
        Sf::Rgba8Uint => Tf::Rgba8Uint,
        Sf::Rgba8Sint => Tf::Rgba8Sint,
        Sf::Bgra8Unorm => Tf::Bgra8Unorm,

        Sf::Rgb10a2Uint => Tf::Rgb10a2Uint,
        Sf::Rgb10a2Unorm => Tf::Rgb10a2Unorm,
        Sf::Rg11b10Ufloat => Tf::Rg11b10Ufloat,

        Sf::R64Uint => Tf::R64Uint,
        Sf::Rg32Uint => Tf::Rg32Uint,
        Sf::Rg32Sint => Tf::Rg32Sint,
        Sf::Rg32Float => Tf::Rg32Float,
        Sf::Rgba16Uint => Tf::Rgba16Uint,
        Sf::Rgba16Sint => Tf::Rgba16Sint,
        Sf::Rgba16Float => Tf::Rgba16Float,

        Sf::Rgba32Uint => Tf::Rgba32Uint,
        Sf::Rgba32Sint => Tf::Rgba32Sint,
        Sf::Rgba32Float => Tf::Rgba32Float,

        Sf::R16Unorm => Tf::R16Unorm,
        Sf::R16Snorm => Tf::R16Snorm,
        Sf::Rg16Unorm => Tf::Rg16Unorm,
        Sf::Rg16Snorm => Tf::Rg16Snorm,
        Sf::Rgba16Unorm => Tf::Rgba16Unorm,
        Sf::Rgba16Snorm => Tf::Rgba16Snorm,
    }
}

/// Map a [`naga::ShaderStage`] to the corresponding [`wgt::ShaderStages`] flag.
pub fn map_naga_stage(stage: naga::ShaderStage) -> wgt::ShaderStages {
    match stage {
        naga::ShaderStage::Vertex => wgt::ShaderStages::VERTEX,
        naga::ShaderStage::Fragment => wgt::ShaderStages::FRAGMENT,
        naga::ShaderStage::Compute => wgt::ShaderStages::COMPUTE,
        naga::ShaderStage::Task => wgt::ShaderStages::TASK,
        naga::ShaderStage::Mesh => wgt::ShaderStages::MESH,
        naga::ShaderStage::RayGeneration => wgt::ShaderStages::RAY_GENERATION,
        naga::ShaderStage::AnyHit => wgt::ShaderStages::ANY_HIT,
        naga::ShaderStage::ClosestHit => wgt::ShaderStages::CLOSEST_HIT,
        naga::ShaderStage::Miss => wgt::ShaderStages::MISS,
    }
}
