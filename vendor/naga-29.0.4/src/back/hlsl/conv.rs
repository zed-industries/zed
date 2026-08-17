use crate::common;

use alloc::{borrow::Cow, format, string::String};

use super::Error;
use crate::proc::Alignment;

impl crate::ScalarKind {
    pub(super) fn to_hlsl_cast(self) -> &'static str {
        match self {
            Self::Float => "asfloat",
            Self::Sint => "asint",
            Self::Uint => "asuint",
            Self::Bool | Self::AbstractInt | Self::AbstractFloat => unreachable!(),
        }
    }
}

impl crate::Scalar {
    /// Helper function that returns scalar related strings
    ///
    /// <https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-scalar>
    pub(super) const fn to_hlsl_str(self) -> Result<&'static str, Error> {
        match self.kind {
            crate::ScalarKind::Sint => match self.width {
                4 => Ok("int"),
                8 => Ok("int64_t"),
                _ => Err(Error::UnsupportedScalar(self)),
            },
            crate::ScalarKind::Uint => match self.width {
                4 => Ok("uint"),
                8 => Ok("uint64_t"),
                _ => Err(Error::UnsupportedScalar(self)),
            },
            crate::ScalarKind::Float => match self.width {
                2 => Ok("half"),
                4 => Ok("float"),
                8 => Ok("double"),
                _ => Err(Error::UnsupportedScalar(self)),
            },
            crate::ScalarKind::Bool => Ok("bool"),
            crate::ScalarKind::AbstractInt | crate::ScalarKind::AbstractFloat => {
                Err(Error::UnsupportedScalar(self))
            }
        }
    }
}

impl crate::TypeInner {
    pub(super) const fn is_matrix(&self) -> bool {
        match *self {
            Self::Matrix { .. } => true,
            _ => false,
        }
    }

    pub(super) fn size_hlsl(&self, gctx: crate::proc::GlobalCtx) -> Result<u32, Error> {
        match *self {
            Self::Matrix {
                columns,
                rows,
                scalar,
            } => {
                let stride = Alignment::from(rows) * scalar.width as u32;
                let last_row_size = rows as u32 * scalar.width as u32;
                Ok(((columns as u32 - 1) * stride) + last_row_size)
            }
            Self::Array { base, size, stride } => {
                let count = match size.resolve(gctx)? {
                    crate::proc::IndexableLength::Known(size) => size,
                    // A dynamically-sized array has to have at least one element
                    crate::proc::IndexableLength::Dynamic => 1,
                };
                let last_el_size = gctx.types[base].inner.size_hlsl(gctx)?;
                Ok(((count - 1) * stride) + last_el_size)
            }
            _ => Ok(self.size(gctx)),
        }
    }

    /// Used to generate the name of the wrapped type constructor
    pub(super) fn hlsl_type_id<'a>(
        base: crate::Handle<crate::Type>,
        gctx: crate::proc::GlobalCtx,
        names: &'a crate::FastHashMap<crate::proc::NameKey, String>,
    ) -> Result<Cow<'a, str>, Error> {
        Ok(match gctx.types[base].inner {
            crate::TypeInner::Scalar(scalar) => Cow::Borrowed(scalar.to_hlsl_str()?),
            crate::TypeInner::Vector { size, scalar } => Cow::Owned(format!(
                "{}{}",
                scalar.to_hlsl_str()?,
                common::vector_size_str(size)
            )),
            crate::TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } => Cow::Owned(format!(
                "{}{}x{}",
                scalar.to_hlsl_str()?,
                common::vector_size_str(columns),
                common::vector_size_str(rows),
            )),
            crate::TypeInner::Array {
                base,
                size: crate::ArraySize::Constant(size),
                ..
            } => Cow::Owned(format!(
                "array{size}_{}_",
                Self::hlsl_type_id(base, gctx, names)?
            )),
            crate::TypeInner::Struct { .. } => {
                Cow::Borrowed(&names[&crate::proc::NameKey::Type(base)])
            }
            _ => unreachable!(),
        })
    }
}

impl crate::StorageFormat {
    pub(super) const fn to_hlsl_str(self) -> &'static str {
        match self {
            Self::R16Float | Self::R32Float => "float",
            Self::R8Unorm | Self::R16Unorm => "unorm float",
            Self::R8Snorm | Self::R16Snorm => "snorm float",
            Self::R8Uint | Self::R16Uint | Self::R32Uint => "uint",
            Self::R8Sint | Self::R16Sint | Self::R32Sint => "int",
            Self::R64Uint => "uint64_t",

            Self::Rg16Float | Self::Rg32Float => "float4",
            Self::Rg8Unorm | Self::Rg16Unorm => "unorm float4",
            Self::Rg8Snorm | Self::Rg16Snorm => "snorm float4",

            Self::Rg8Sint | Self::Rg16Sint | Self::Rg32Uint => "int4",
            Self::Rg8Uint | Self::Rg16Uint | Self::Rg32Sint => "uint4",

            Self::Rg11b10Ufloat => "float4",

            Self::Rgba16Float | Self::Rgba32Float => "float4",
            Self::Rgba8Unorm | Self::Bgra8Unorm | Self::Rgba16Unorm | Self::Rgb10a2Unorm => {
                "unorm float4"
            }
            Self::Rgba8Snorm | Self::Rgba16Snorm => "snorm float4",

            Self::Rgba8Uint | Self::Rgba16Uint | Self::Rgba32Uint | Self::Rgb10a2Uint => "uint4",
            Self::Rgba8Sint | Self::Rgba16Sint | Self::Rgba32Sint => "int4",
        }
    }
}

impl crate::BuiltIn {
    pub(super) fn to_hlsl_str(self) -> Result<&'static str, Error> {
        Ok(match self {
            Self::Position { .. } => "SV_Position",
            // vertex
            Self::ClipDistance => "SV_ClipDistance",
            Self::CullDistance => "SV_CullDistance",
            Self::InstanceIndex => "SV_InstanceID",
            Self::VertexIndex => "SV_VertexID",
            // fragment
            Self::FragDepth => "SV_Depth",
            Self::FrontFacing => "SV_IsFrontFace",
            Self::PrimitiveIndex => "SV_PrimitiveID",
            Self::Barycentric { .. } => "SV_Barycentrics",
            Self::SampleIndex => "SV_SampleIndex",
            Self::SampleMask => "SV_Coverage",
            // compute
            Self::GlobalInvocationId => "SV_DispatchThreadID",
            Self::LocalInvocationId => "SV_GroupThreadID",
            Self::LocalInvocationIndex => "SV_GroupIndex",
            Self::WorkGroupId => "SV_GroupID",
            // The specific semantic we use here doesn't matter, because references
            // to this field will get replaced with references to `SPECIAL_CBUF_VAR`
            // in `Writer::write_expr`.
            Self::NumWorkGroups => "SV_GroupID",
            Self::ViewIndex => "SV_ViewID",
            // These builtins map to functions
            Self::SubgroupSize
            | Self::SubgroupInvocationId
            | Self::NumSubgroups
            | Self::SubgroupId => unreachable!(),
            Self::BaseInstance | Self::BaseVertex | Self::WorkGroupSize => {
                return Err(Error::Unimplemented(format!("builtin {self:?}")))
            }
            Self::PointSize | Self::PointCoord | Self::DrawIndex => {
                return Err(Error::Custom(format!("Unsupported builtin {self:?}")))
            }
            Self::CullPrimitive => "SV_CullPrimitive",
            Self::PointIndex | Self::LineIndices | Self::TriangleIndices => unimplemented!(),
            Self::MeshTaskSize
            | Self::VertexCount
            | Self::PrimitiveCount
            | Self::Vertices
            | Self::Primitives => unreachable!(),
            Self::RayInvocationId
            | Self::NumRayInvocations
            | Self::InstanceCustomData
            | Self::GeometryIndex
            | Self::WorldRayOrigin
            | Self::WorldRayDirection
            | Self::ObjectRayOrigin
            | Self::ObjectRayDirection
            | Self::RayTmin
            | Self::RayTCurrentMax
            | Self::ObjectToWorld
            | Self::WorldToObject
            | Self::HitKind => unreachable!(),
        })
    }
}

impl crate::Interpolation {
    /// Return the string corresponding to the HLSL interpolation qualifier.
    pub(super) const fn to_hlsl_str(self) -> Option<&'static str> {
        match self {
            // Would be "linear", but it's the default interpolation in SM4 and up
            // https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-struct#interpolation-modifiers-introduced-in-shader-model-4
            Self::Perspective => None,
            Self::Linear => Some("noperspective"),
            Self::Flat => Some("nointerpolation"),
            Self::PerVertex => unreachable!(),
        }
    }
}

impl crate::Sampling {
    /// Return the HLSL auxiliary qualifier for the given sampling value.
    pub(super) const fn to_hlsl_str(self) -> Option<&'static str> {
        match self {
            Self::Center | Self::First | Self::Either => None,
            Self::Centroid => Some("centroid"),
            Self::Sample => Some("sample"),
        }
    }
}

impl crate::AtomicFunction {
    /// Return the HLSL suffix for the `InterlockedXxx` method.
    pub(super) const fn to_hlsl_suffix(self) -> &'static str {
        match self {
            Self::Add | Self::Subtract => "Add",
            Self::And => "And",
            Self::InclusiveOr => "Or",
            Self::ExclusiveOr => "Xor",
            Self::Min => "Min",
            Self::Max => "Max",
            Self::Exchange { compare: None } => "Exchange",
            Self::Exchange { .. } => "CompareExchange",
        }
    }
}
