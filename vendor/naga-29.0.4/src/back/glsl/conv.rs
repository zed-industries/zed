use crate::back::glsl::{BackendResult, Error, VaryingOptions};

/// Structure returned by [`glsl_scalar`]
///
/// It contains both a prefix used in other types and the full type name
pub(in crate::back::glsl) struct ScalarString<'a> {
    /// The prefix used to compose other types
    pub prefix: &'a str,
    /// The name of the scalar type
    pub full: &'a str,
}

/// Helper function that returns scalar related strings
///
/// Check [`ScalarString`] for the information provided
///
/// # Errors
/// If a [`Float`](crate::ScalarKind::Float) with an width that isn't 4 or 8
pub(in crate::back::glsl) const fn glsl_scalar(
    scalar: crate::Scalar,
) -> Result<ScalarString<'static>, Error> {
    use crate::ScalarKind as Sk;

    Ok(match scalar.kind {
        Sk::Sint => ScalarString {
            prefix: "i",
            full: "int",
        },
        Sk::Uint => ScalarString {
            prefix: "u",
            full: "uint",
        },
        Sk::Float => match scalar.width {
            4 => ScalarString {
                prefix: "",
                full: "float",
            },
            8 => ScalarString {
                prefix: "d",
                full: "double",
            },
            _ => return Err(Error::UnsupportedScalar(scalar)),
        },
        Sk::Bool => ScalarString {
            prefix: "b",
            full: "bool",
        },
        Sk::AbstractInt | Sk::AbstractFloat => {
            return Err(Error::UnsupportedScalar(scalar));
        }
    })
}

/// Helper function that returns the glsl variable name for a builtin
pub(in crate::back::glsl) const fn glsl_built_in(
    built_in: crate::BuiltIn,
    options: VaryingOptions,
) -> &'static str {
    use crate::BuiltIn as Bi;

    match built_in {
        Bi::Position { .. } => {
            if options.output {
                "gl_Position"
            } else {
                "gl_FragCoord"
            }
        }
        Bi::ViewIndex => {
            if options.targeting_webgl {
                "gl_ViewID_OVR"
            } else {
                "uint(gl_ViewIndex)"
            }
        }
        // vertex
        Bi::BaseInstance => "uint(gl_BaseInstance)",
        Bi::BaseVertex => "uint(gl_BaseVertex)",
        Bi::ClipDistance => "gl_ClipDistance",
        Bi::CullDistance => "gl_CullDistance",
        Bi::InstanceIndex => {
            if options.draw_parameters {
                "(uint(gl_InstanceID) + uint(gl_BaseInstanceARB))"
            } else {
                // Must match FIRST_INSTANCE_BINDING
                "(uint(gl_InstanceID) + naga_vs_first_instance)"
            }
        }
        Bi::PointSize => "gl_PointSize",
        Bi::VertexIndex => "uint(gl_VertexID)",
        Bi::DrawIndex => "gl_DrawID",
        // fragment
        Bi::FragDepth => "gl_FragDepth",
        Bi::PointCoord => "gl_PointCoord",
        Bi::FrontFacing => "gl_FrontFacing",
        Bi::PrimitiveIndex => "uint(gl_PrimitiveID)",
        Bi::Barycentric { perspective: true } => "gl_BaryCoordEXT",
        Bi::Barycentric { perspective: false } => "gl_BaryCoordNoPerspEXT",
        Bi::SampleIndex => "gl_SampleID",
        Bi::SampleMask => {
            if options.output {
                "gl_SampleMask"
            } else {
                "gl_SampleMaskIn"
            }
        }
        // compute
        Bi::GlobalInvocationId => "gl_GlobalInvocationID",
        Bi::LocalInvocationId => "gl_LocalInvocationID",
        Bi::LocalInvocationIndex => "gl_LocalInvocationIndex",
        Bi::WorkGroupId => "gl_WorkGroupID",
        Bi::WorkGroupSize => "gl_WorkGroupSize",
        Bi::NumWorkGroups => "gl_NumWorkGroups",
        // subgroup
        Bi::NumSubgroups => "gl_NumSubgroups",
        Bi::SubgroupId => "gl_SubgroupID",
        Bi::SubgroupSize => "gl_SubgroupSize",
        Bi::SubgroupInvocationId => "gl_SubgroupInvocationID",
        // mesh
        // TODO: figure out how to map these to glsl things as glsl treats them as arrays
        Bi::CullPrimitive
        | Bi::PointIndex
        | Bi::LineIndices
        | Bi::TriangleIndices
        | Bi::MeshTaskSize
        | Bi::VertexCount
        | Bi::PrimitiveCount
        | Bi::Vertices
        | Bi::Primitives
        | Bi::RayInvocationId
        | Bi::NumRayInvocations
        | Bi::InstanceCustomData
        | Bi::GeometryIndex
        | Bi::WorldRayOrigin
        | Bi::WorldRayDirection
        | Bi::ObjectRayOrigin
        | Bi::ObjectRayDirection
        | Bi::RayTmin
        | Bi::RayTCurrentMax
        | Bi::ObjectToWorld
        | Bi::WorldToObject
        | Bi::HitKind => {
            unimplemented!()
        }
    }
}

/// Helper function that returns the string corresponding to the address space
pub(in crate::back::glsl) const fn glsl_storage_qualifier(
    space: crate::AddressSpace,
) -> Option<&'static str> {
    use crate::AddressSpace as As;

    match space {
        As::Function => None,
        As::Private => None,
        As::Storage { .. } => Some("buffer"),
        As::Uniform => Some("uniform"),
        As::Handle => Some("uniform"),
        As::WorkGroup => Some("shared"),
        As::Immediate => Some("uniform"),
        As::TaskPayload | As::RayPayload | As::IncomingRayPayload => unreachable!(),
    }
}

/// Helper function that returns the string corresponding to the glsl interpolation qualifier
pub(in crate::back::glsl) const fn glsl_interpolation(
    interpolation: crate::Interpolation,
) -> &'static str {
    use crate::Interpolation as I;

    match interpolation {
        I::Perspective => "smooth",
        I::Linear => "noperspective",
        I::Flat => "flat",
        I::PerVertex => unreachable!(),
    }
}

/// Return the GLSL auxiliary qualifier for the given sampling value.
pub(in crate::back::glsl) const fn glsl_sampling(
    sampling: crate::Sampling,
) -> BackendResult<Option<&'static str>> {
    use crate::Sampling as S;

    Ok(match sampling {
        S::First => return Err(Error::FirstSamplingNotSupported),
        S::Center | S::Either => None,
        S::Centroid => Some("centroid"),
        S::Sample => Some("sample"),
    })
}

/// Helper function that returns the glsl dimension string of [`ImageDimension`](crate::ImageDimension)
pub(in crate::back::glsl) const fn glsl_dimension(dim: crate::ImageDimension) -> &'static str {
    use crate::ImageDimension as IDim;

    match dim {
        IDim::D1 => "1D",
        IDim::D2 => "2D",
        IDim::D3 => "3D",
        IDim::Cube => "Cube",
    }
}

/// Helper function that returns the glsl storage format string of [`StorageFormat`](crate::StorageFormat)
pub(in crate::back::glsl) fn glsl_storage_format(
    format: crate::StorageFormat,
) -> Result<&'static str, Error> {
    use crate::StorageFormat as Sf;

    Ok(match format {
        Sf::R8Unorm => "r8",
        Sf::R8Snorm => "r8_snorm",
        Sf::R8Uint => "r8ui",
        Sf::R8Sint => "r8i",
        Sf::R16Uint => "r16ui",
        Sf::R16Sint => "r16i",
        Sf::R16Float => "r16f",
        Sf::Rg8Unorm => "rg8",
        Sf::Rg8Snorm => "rg8_snorm",
        Sf::Rg8Uint => "rg8ui",
        Sf::Rg8Sint => "rg8i",
        Sf::R32Uint => "r32ui",
        Sf::R32Sint => "r32i",
        Sf::R32Float => "r32f",
        Sf::Rg16Uint => "rg16ui",
        Sf::Rg16Sint => "rg16i",
        Sf::Rg16Float => "rg16f",
        Sf::Rgba8Unorm => "rgba8",
        Sf::Rgba8Snorm => "rgba8_snorm",
        Sf::Rgba8Uint => "rgba8ui",
        Sf::Rgba8Sint => "rgba8i",
        Sf::Rgb10a2Uint => "rgb10_a2ui",
        Sf::Rgb10a2Unorm => "rgb10_a2",
        Sf::Rg11b10Ufloat => "r11f_g11f_b10f",
        Sf::R64Uint => "r64ui",
        Sf::Rg32Uint => "rg32ui",
        Sf::Rg32Sint => "rg32i",
        Sf::Rg32Float => "rg32f",
        Sf::Rgba16Uint => "rgba16ui",
        Sf::Rgba16Sint => "rgba16i",
        Sf::Rgba16Float => "rgba16f",
        Sf::Rgba32Uint => "rgba32ui",
        Sf::Rgba32Sint => "rgba32i",
        Sf::Rgba32Float => "rgba32f",
        Sf::R16Unorm => "r16",
        Sf::R16Snorm => "r16_snorm",
        Sf::Rg16Unorm => "rg16",
        Sf::Rg16Snorm => "rg16_snorm",
        Sf::Rgba16Unorm => "rgba16",
        Sf::Rgba16Snorm => "rgba16_snorm",

        Sf::Bgra8Unorm => {
            return Err(Error::Custom(
                "Support format BGRA8 is not implemented".into(),
            ))
        }
    })
}
