/*!
Helpers for the hlsl backend

Important note about `Expression::ImageQuery`/`Expression::ArrayLength` and hlsl backend:

Due to implementation of `GetDimensions` function in hlsl (<https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-to-getdimensions>)
backend can't work with it as an expression.
Instead, it generates a unique wrapped function per `Expression::ImageQuery`, based on texture info and query function.
See `WrappedImageQuery` struct that represents a unique function and will be generated before writing all statements and expressions.
This allowed to works with `Expression::ImageQuery` as expression and write wrapped function.

For example:
```wgsl
let dim_1d = textureDimensions(image_1d);
```

```hlsl
int NagaDimensions1D(Texture1D<float4>)
{
   uint4 ret;
   image_1d.GetDimensions(ret.x);
   return ret.x;
}

int dim_1d = NagaDimensions1D(image_1d);
```
*/

use alloc::format;
use core::fmt::Write;

use super::{
    super::FunctionCtx,
    writer::{
        ABS_FUNCTION, DIV_FUNCTION, EXTRACT_BITS_FUNCTION, F2I32_FUNCTION, F2I64_FUNCTION,
        F2U32_FUNCTION, F2U64_FUNCTION, IMAGE_LOAD_EXTERNAL_FUNCTION,
        IMAGE_SAMPLE_BASE_CLAMP_TO_EDGE_FUNCTION, INSERT_BITS_FUNCTION, MOD_FUNCTION, NEG_FUNCTION,
    },
    BackendResult, WrappedType,
};
use crate::{arena::Handle, proc::NameKey, ScalarKind};

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedArrayLength {
    pub(super) writable: bool,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedImageLoad {
    pub(super) class: crate::ImageClass,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedImageSample {
    pub(super) class: crate::ImageClass,
    pub(super) clamp_to_edge: bool,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedImageQuery {
    pub(super) dim: crate::ImageDimension,
    pub(super) arrayed: bool,
    pub(super) class: crate::ImageClass,
    pub(super) query: ImageQuery,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedConstructor {
    pub(super) ty: Handle<crate::Type>,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedStructMatrixAccess {
    pub(super) ty: Handle<crate::Type>,
    pub(super) index: u32,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedMatCx2 {
    pub(super) columns: crate::VectorSize,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedMath {
    pub(super) fun: crate::MathFunction,
    pub(super) scalar: crate::Scalar,
    pub(super) components: Option<u32>,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedZeroValue {
    pub(super) ty: Handle<crate::Type>,
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedUnaryOp {
    pub(super) op: crate::UnaryOperator,
    // This can only represent scalar or vector types. If we ever need to wrap
    // unary ops with other types, we'll need a better representation.
    pub(super) ty: (Option<crate::VectorSize>, crate::Scalar),
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedBinaryOp {
    pub(super) op: crate::BinaryOperator,
    // This can only represent scalar or vector types. If we ever need to wrap
    // binary ops with other types, we'll need a better representation.
    pub(super) left_ty: (Option<crate::VectorSize>, crate::Scalar),
    pub(super) right_ty: (Option<crate::VectorSize>, crate::Scalar),
}

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WrappedCast {
    // This can only represent scalar or vector types. If we ever need to wrap
    // casts with other types, we'll need a better representation.
    pub(super) vector_size: Option<crate::VectorSize>,
    pub(super) src_scalar: crate::Scalar,
    pub(super) dst_scalar: crate::Scalar,
}

/// HLSL backend requires its own `ImageQuery` enum.
///
/// It is used inside `WrappedImageQuery` and should be unique per ImageQuery function.
/// IR version can't be unique per function, because it's store mipmap level as an expression.
///
/// For example:
/// ```wgsl
/// let dim_cube_array_lod = textureDimensions(image_cube_array, 1);
/// let dim_cube_array_lod2 = textureDimensions(image_cube_array, 1);
/// ```
///
/// ```ir
/// ImageQuery {
///  image: [1],
///  query: Size {
///      level: Some(
///          [1],
///      ),
///  },
/// },
/// ImageQuery {
///  image: [1],
///  query: Size {
///      level: Some(
///          [2],
///      ),
///  },
/// },
/// ```
///
/// HLSL should generate only 1 function for this case.
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum ImageQuery {
    Size,
    SizeLevel,
    NumLevels,
    NumLayers,
    NumSamples,
}

impl From<crate::ImageQuery> for ImageQuery {
    fn from(q: crate::ImageQuery) -> Self {
        use crate::ImageQuery as Iq;
        match q {
            Iq::Size { level: Some(_) } => ImageQuery::SizeLevel,
            Iq::Size { level: None } => ImageQuery::Size,
            Iq::NumLevels => ImageQuery::NumLevels,
            Iq::NumLayers => ImageQuery::NumLayers,
            Iq::NumSamples => ImageQuery::NumSamples,
        }
    }
}

pub(super) const IMAGE_STORAGE_LOAD_SCALAR_WRAPPER: &str = "LoadedStorageValueFrom";

impl<W: Write> super::Writer<'_, W> {
    pub(super) fn write_image_type(
        &mut self,
        dim: crate::ImageDimension,
        arrayed: bool,
        class: crate::ImageClass,
    ) -> BackendResult {
        let access_str = match class {
            crate::ImageClass::Storage { .. } => "RW",
            _ => "",
        };
        let dim_str = dim.to_hlsl_str();
        let arrayed_str = if arrayed { "Array" } else { "" };
        write!(self.out, "{access_str}Texture{dim_str}{arrayed_str}")?;
        match class {
            crate::ImageClass::Depth { multi } => {
                let multi_str = if multi { "MS" } else { "" };
                write!(self.out, "{multi_str}<float>")?
            }
            crate::ImageClass::Sampled { kind, multi } => {
                let multi_str = if multi { "MS" } else { "" };
                let scalar_kind_str = crate::Scalar { kind, width: 4 }.to_hlsl_str()?;
                write!(self.out, "{multi_str}<{scalar_kind_str}4>")?
            }
            crate::ImageClass::Storage { format, .. } => {
                let storage_format_str = format.to_hlsl_str();
                write!(self.out, "<{storage_format_str}>")?
            }
            crate::ImageClass::External => {
                unreachable!(
                    "external images should be handled by `write_global_external_texture`"
                );
            }
        }
        Ok(())
    }

    pub(super) fn write_wrapped_array_length_function_name(
        &mut self,
        query: WrappedArrayLength,
    ) -> BackendResult {
        let access_str = if query.writable { "RW" } else { "" };
        write!(self.out, "NagaBufferLength{access_str}",)?;

        Ok(())
    }

    /// Helper function that write wrapped function for `Expression::ArrayLength`
    ///
    /// <https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/sm5-object-rwbyteaddressbuffer-getdimensions>
    pub(super) fn write_wrapped_array_length_function(
        &mut self,
        wal: WrappedArrayLength,
    ) -> BackendResult {
        use crate::back::INDENT;

        const ARGUMENT_VARIABLE_NAME: &str = "buffer";
        const RETURN_VARIABLE_NAME: &str = "ret";

        // Write function return type and name
        write!(self.out, "uint ")?;
        self.write_wrapped_array_length_function_name(wal)?;

        // Write function parameters
        write!(self.out, "(")?;
        let access_str = if wal.writable { "RW" } else { "" };
        writeln!(
            self.out,
            "{access_str}ByteAddressBuffer {ARGUMENT_VARIABLE_NAME})"
        )?;
        // Write function body
        writeln!(self.out, "{{")?;

        // Write `GetDimensions` function.
        writeln!(self.out, "{INDENT}uint {RETURN_VARIABLE_NAME};")?;
        writeln!(
            self.out,
            "{INDENT}{ARGUMENT_VARIABLE_NAME}.GetDimensions({RETURN_VARIABLE_NAME});"
        )?;

        // Write return value
        writeln!(self.out, "{INDENT}return {RETURN_VARIABLE_NAME};")?;

        // End of function body
        writeln!(self.out, "}}")?;
        // Write extra new line
        writeln!(self.out)?;

        Ok(())
    }

    /// Helper function used by [`Self::write_wrapped_image_load_function`] and
    /// [`Self::write_wrapped_image_sample_function`] to write the shared YUV
    /// to RGB conversion code for external textures. Expects the preceding
    /// code to declare the Y component as a `float` variable of name `y`, the
    /// UV components as a `float2` variable of name `uv`, and the external
    /// texture params as a variable of name `params`. The emitted code will
    /// return the result.
    fn write_convert_yuv_to_rgb_and_return(
        &mut self,
        level: crate::back::Level,
        y: &str,
        uv: &str,
        params: &str,
    ) -> BackendResult {
        let l1 = level;
        let l2 = l1.next();

        // Convert from YUV to non-linear RGB in the source color space. We
        // declare our matrices as row_major in HLSL, therefore we must reverse
        // the order of this multiplication
        writeln!(
            self.out,
            "{l1}float3 srcGammaRgb = mul(float4({y}, {uv}, 1.0), {params}.yuv_conversion_matrix).rgb;"
        )?;

        // Apply the inverse of the source transfer function to convert to
        // linear RGB in the source color space.
        writeln!(
            self.out,
            "{l1}float3 srcLinearRgb = srcGammaRgb < {params}.src_tf.k * {params}.src_tf.b ?"
        )?;
        writeln!(self.out, "{l2}srcGammaRgb / {params}.src_tf.k :")?;
        writeln!(self.out, "{l2}pow((srcGammaRgb + {params}.src_tf.a - 1.0) / {params}.src_tf.a, {params}.src_tf.g);")?;

        // Multiply by the gamut conversion matrix to convert to linear RGB in
        // the destination color space. We declare our matrices as row_major in
        // HLSL, therefore we must reverse the order of this multiplication.
        writeln!(
            self.out,
            "{l1}float3 dstLinearRgb = mul(srcLinearRgb, {params}.gamut_conversion_matrix);"
        )?;

        // Finally, apply the dest transfer function to convert to non-linear
        // RGB in the destination color space, and return the result.
        writeln!(
            self.out,
            "{l1}float3 dstGammaRgb = dstLinearRgb < {params}.dst_tf.b ?"
        )?;
        writeln!(self.out, "{l2}{params}.dst_tf.k * dstLinearRgb :")?;
        writeln!(self.out, "{l2}{params}.dst_tf.a * pow(dstLinearRgb, 1.0 / {params}.dst_tf.g) - ({params}.dst_tf.a - 1);")?;

        writeln!(self.out, "{l1}return float4(dstGammaRgb, 1.0);")?;
        Ok(())
    }

    pub(super) fn write_wrapped_image_load_function(
        &mut self,
        module: &crate::Module,
        load: WrappedImageLoad,
    ) -> BackendResult {
        match load {
            WrappedImageLoad {
                class: crate::ImageClass::External,
            } => {
                let l1 = crate::back::Level(1);
                let l2 = l1.next();
                let l3 = l2.next();
                let params_ty_name = &self.names
                    [&NameKey::Type(module.special_types.external_texture_params.unwrap())];
                writeln!(self.out, "float4 {IMAGE_LOAD_EXTERNAL_FUNCTION}(")?;
                writeln!(self.out, "{l1}Texture2D<float4> plane0,")?;
                writeln!(self.out, "{l1}Texture2D<float4> plane1,")?;
                writeln!(self.out, "{l1}Texture2D<float4> plane2,")?;
                writeln!(self.out, "{l1}{params_ty_name} params,")?;
                writeln!(self.out, "{l1}uint2 coords)")?;
                writeln!(self.out, "{{")?;
                writeln!(self.out, "{l1}uint2 plane0_size;")?;
                writeln!(
                    self.out,
                    "{l1}plane0.GetDimensions(plane0_size.x, plane0_size.y);"
                )?;
                // Clamp coords to provided size of external texture to prevent OOB read.
                // If params.size is zero then clamp to the actual size of the texture.
                writeln!(
                    self.out,
                    "{l1}uint2 cropped_size = any(params.size) ? params.size : plane0_size;"
                )?;
                writeln!(self.out, "{l1}coords = min(coords, cropped_size - 1);")?;

                // Apply load transformation. We declare our matrices as row_major in
                // HLSL, therefore we must reverse the order of this multiplication
                writeln!(self.out, "{l1}float3x2 load_transform = float3x2(")?;
                writeln!(self.out, "{l2}params.load_transform_0,")?;
                writeln!(self.out, "{l2}params.load_transform_1,")?;
                writeln!(self.out, "{l2}params.load_transform_2")?;
                writeln!(self.out, "{l1});")?;
                writeln!(self.out, "{l1}uint2 plane0_coords = uint2(round(mul(float3(coords, 1.0), load_transform)));")?;
                writeln!(self.out, "{l1}if (params.num_planes == 1u) {{")?;
                // For single plane, simply read from plane0
                writeln!(
                    self.out,
                    "{l2}return plane0.Load(uint3(plane0_coords, 0u));"
                )?;
                writeln!(self.out, "{l1}}} else {{")?;

                // Chroma planes may be subsampled so we must scale the coords accordingly.
                writeln!(self.out, "{l2}uint2 plane1_size;")?;
                writeln!(
                    self.out,
                    "{l2}plane1.GetDimensions(plane1_size.x, plane1_size.y);"
                )?;
                writeln!(self.out, "{l2}uint2 plane1_coords = uint2(floor(float2(plane0_coords) * float2(plane1_size) / float2(plane0_size)));")?;

                // For multi-plane, read the Y value from plane 0
                writeln!(
                    self.out,
                    "{l2}float y = plane0.Load(uint3(plane0_coords, 0u)).x;"
                )?;

                writeln!(self.out, "{l2}float2 uv;")?;
                writeln!(self.out, "{l2}if (params.num_planes == 2u) {{")?;
                // Read UV from interleaved plane 1
                writeln!(
                    self.out,
                    "{l3}uv = plane1.Load(uint3(plane1_coords, 0u)).xy;"
                )?;
                writeln!(self.out, "{l2}}} else {{")?;
                // Read U and V from planes 1 and 2 respectively
                writeln!(self.out, "{l3}uint2 plane2_size;")?;
                writeln!(
                    self.out,
                    "{l3}plane2.GetDimensions(plane2_size.x, plane2_size.y);"
                )?;
                writeln!(self.out, "{l3}uint2 plane2_coords = uint2(floor(float2(plane0_coords) * float2(plane2_size) / float2(plane0_size)));")?;
                writeln!(self.out, "{l3}uv = float2(plane1.Load(uint3(plane1_coords, 0u)).x, plane2.Load(uint3(plane2_coords, 0u)).x);")?;
                writeln!(self.out, "{l2}}}")?;

                self.write_convert_yuv_to_rgb_and_return(l2, "y", "uv", "params")?;

                writeln!(self.out, "{l1}}}")?;
                writeln!(self.out, "}}")?;
                writeln!(self.out)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub(super) fn write_wrapped_image_sample_function(
        &mut self,
        module: &crate::Module,
        sample: WrappedImageSample,
    ) -> BackendResult {
        match sample {
            WrappedImageSample {
                class: crate::ImageClass::External,
                clamp_to_edge: true,
            } => {
                let l1 = crate::back::Level(1);
                let l2 = l1.next();
                let l3 = l2.next();
                let params_ty_name = &self.names
                    [&NameKey::Type(module.special_types.external_texture_params.unwrap())];
                writeln!(
                    self.out,
                    "float4 {IMAGE_SAMPLE_BASE_CLAMP_TO_EDGE_FUNCTION}("
                )?;
                writeln!(self.out, "{l1}Texture2D<float4> plane0,")?;
                writeln!(self.out, "{l1}Texture2D<float4> plane1,")?;
                writeln!(self.out, "{l1}Texture2D<float4> plane2,")?;
                writeln!(self.out, "{l1}{params_ty_name} params,")?;
                writeln!(self.out, "{l1}SamplerState samp,")?;
                writeln!(self.out, "{l1}float2 coords)")?;
                writeln!(self.out, "{{")?;
                writeln!(self.out, "{l1}float2 plane0_size;")?;
                writeln!(
                    self.out,
                    "{l1}plane0.GetDimensions(plane0_size.x, plane0_size.y);"
                )?;
                writeln!(self.out, "{l1}float3x2 sample_transform = float3x2(")?;
                writeln!(self.out, "{l2}params.sample_transform_0,")?;
                writeln!(self.out, "{l2}params.sample_transform_1,")?;
                writeln!(self.out, "{l2}params.sample_transform_2")?;
                writeln!(self.out, "{l1});")?;
                // Apply sample transformation. We declare our matrices as row_major in
                // HLSL, therefore we must reverse the order of this multiplication
                writeln!(
                    self.out,
                    "{l1}coords = mul(float3(coords, 1.0), sample_transform);"
                )?;
                // Calculate the sample bounds. The purported size of the texture
                // (params.size) is irrelevant here as we are dealing with normalized
                // coordinates. Usually we would clamp to (0,0)..(1,1). However, we must
                // apply the sample transformation to that, also bearing in mind that it
                // may contain a flip on either axis. We calculate and adjust for the
                // half-texel separately for each plane as it depends on the actual
                // texture size which may vary between planes.
                writeln!(
                    self.out,
                    "{l1}float2 bounds_min = mul(float3(0.0, 0.0, 1.0), sample_transform);"
                )?;
                writeln!(
                    self.out,
                    "{l1}float2 bounds_max = mul(float3(1.0, 1.0, 1.0), sample_transform);"
                )?;
                writeln!(self.out, "{l1}float4 bounds = float4(min(bounds_min, bounds_max), max(bounds_min, bounds_max));")?;
                writeln!(
                    self.out,
                    "{l1}float2 plane0_half_texel = float2(0.5, 0.5) / plane0_size;"
                )?;
                writeln!(
                    self.out,
                    "{l1}float2 plane0_coords = clamp(coords, bounds.xy + plane0_half_texel, bounds.zw - plane0_half_texel);"
                )?;
                writeln!(self.out, "{l1}if (params.num_planes == 1u) {{")?;
                // For single plane, simply sample from plane0
                writeln!(
                    self.out,
                    "{l2}return plane0.SampleLevel(samp, plane0_coords, 0.0f);"
                )?;
                writeln!(self.out, "{l1}}} else {{")?;

                writeln!(self.out, "{l2}float2 plane1_size;")?;
                writeln!(
                    self.out,
                    "{l2}plane1.GetDimensions(plane1_size.x, plane1_size.y);"
                )?;
                writeln!(
                    self.out,
                    "{l2}float2 plane1_half_texel = float2(0.5, 0.5) / plane1_size;"
                )?;
                writeln!(
                    self.out,
                    "{l2}float2 plane1_coords = clamp(coords, bounds.xy + plane1_half_texel, bounds.zw - plane1_half_texel);"
                )?;

                // For multi-plane, sample the Y value from plane 0
                writeln!(
                    self.out,
                    "{l2}float y = plane0.SampleLevel(samp, plane0_coords, 0.0f).x;"
                )?;
                writeln!(self.out, "{l2}float2 uv;")?;
                writeln!(self.out, "{l2}if (params.num_planes == 2u) {{")?;
                // Sample UV from interleaved plane 1
                writeln!(
                    self.out,
                    "{l3}uv = plane1.SampleLevel(samp, plane1_coords, 0.0f).xy;"
                )?;
                writeln!(self.out, "{l2}}} else {{")?;
                // Sample U and V from planes 1 and 2 respectively
                writeln!(self.out, "{l3}float2 plane2_size;")?;
                writeln!(
                    self.out,
                    "{l3}plane2.GetDimensions(plane2_size.x, plane2_size.y);"
                )?;
                writeln!(
                    self.out,
                    "{l3}float2 plane2_half_texel = float2(0.5, 0.5) / plane2_size;"
                )?;
                writeln!(self.out, "{l3}float2 plane2_coords = clamp(coords, bounds.xy + plane2_half_texel, bounds.zw - plane2_half_texel);")?;
                writeln!(self.out, "{l3}uv = float2(plane1.SampleLevel(samp, plane1_coords, 0.0f).x, plane2.SampleLevel(samp, plane2_coords, 0.0f).x);")?;
                writeln!(self.out, "{l2}}}")?;

                self.write_convert_yuv_to_rgb_and_return(l2, "y", "uv", "params")?;

                writeln!(self.out, "{l1}}}")?;
                writeln!(self.out, "}}")?;
                writeln!(self.out)?;
            }
            WrappedImageSample {
                class:
                    crate::ImageClass::Sampled {
                        kind: ScalarKind::Float,
                        multi: false,
                    },
                clamp_to_edge: true,
            } => {
                writeln!(self.out, "float4 {IMAGE_SAMPLE_BASE_CLAMP_TO_EDGE_FUNCTION}(Texture2D<float4> tex, SamplerState samp, float2 coords) {{")?;
                let l1 = crate::back::Level(1);
                writeln!(self.out, "{l1}float2 size;")?;
                writeln!(self.out, "{l1}tex.GetDimensions(size.x, size.y);")?;
                writeln!(self.out, "{l1}float2 half_texel = float2(0.5, 0.5) / size;")?;
                writeln!(
                    self.out,
                    "{l1}return tex.SampleLevel(samp, clamp(coords, half_texel, 1.0 - half_texel), 0.0);"
                )?;
                writeln!(self.out, "}}")?;
                writeln!(self.out)?;
            }
            _ => {}
        }

        Ok(())
    }

    pub(super) fn write_wrapped_image_query_function_name(
        &mut self,
        query: WrappedImageQuery,
    ) -> BackendResult {
        let dim_str = query.dim.to_hlsl_str();
        let class_str = match query.class {
            crate::ImageClass::Sampled { multi: true, .. } => "MS",
            crate::ImageClass::Depth { multi: true } => "DepthMS",
            crate::ImageClass::Depth { multi: false } => "Depth",
            crate::ImageClass::Sampled { multi: false, .. } => "",
            crate::ImageClass::Storage { .. } => "RW",
            crate::ImageClass::External => "External",
        };
        let arrayed_str = if query.arrayed { "Array" } else { "" };
        let query_str = match query.query {
            ImageQuery::Size => "Dimensions",
            ImageQuery::SizeLevel => "MipDimensions",
            ImageQuery::NumLevels => "NumLevels",
            ImageQuery::NumLayers => "NumLayers",
            ImageQuery::NumSamples => "NumSamples",
        };

        write!(self.out, "Naga{class_str}{query_str}{dim_str}{arrayed_str}")?;

        Ok(())
    }

    /// Helper function that write wrapped function for `Expression::ImageQuery`
    ///
    /// <https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-to-getdimensions>
    pub(super) fn write_wrapped_image_query_function(
        &mut self,
        module: &crate::Module,
        wiq: WrappedImageQuery,
        expr_handle: Handle<crate::Expression>,
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        use crate::{
            back::{COMPONENTS, INDENT},
            ImageDimension as IDim,
        };

        match wiq.class {
            crate::ImageClass::External => {
                if wiq.query != ImageQuery::Size {
                    return Err(super::Error::Custom(
                        "External images only support `Size` queries".into(),
                    ));
                }

                write!(self.out, "uint2 ")?;
                self.write_wrapped_image_query_function_name(wiq)?;
                let params_name = &self.names
                    [&NameKey::Type(module.special_types.external_texture_params.unwrap())];
                // Only plane0 and params are used by this implementation, but it's easier to
                // always take all of them as arguments so that we can unconditionally expand an
                // external texture expression each of its parts.
                writeln!(self.out, "(Texture2D<float4> plane0, Texture2D<float4> plane1, Texture2D<float4> plane2, {params_name} params) {{")?;
                let l1 = crate::back::Level(1);
                let l2 = l1.next();
                writeln!(self.out, "{l1}if (any(params.size)) {{")?;
                writeln!(self.out, "{l2}return params.size;")?;
                writeln!(self.out, "{l1}}} else {{")?;
                // params.size == (0, 0) indicates to query and return plane 0's actual size
                writeln!(self.out, "{l2}uint2 ret;")?;
                writeln!(self.out, "{l2}plane0.GetDimensions(ret.x, ret.y);")?;
                writeln!(self.out, "{l2}return ret;")?;
                writeln!(self.out, "{l1}}}")?;
                writeln!(self.out, "}}")?;
                writeln!(self.out)?;
            }
            _ => {
                const ARGUMENT_VARIABLE_NAME: &str = "tex";
                const RETURN_VARIABLE_NAME: &str = "ret";
                const MIP_LEVEL_PARAM: &str = "mip_level";

                // Write function return type and name
                let ret_ty = func_ctx.resolve_type(expr_handle, &module.types);
                self.write_value_type(module, ret_ty)?;
                write!(self.out, " ")?;
                self.write_wrapped_image_query_function_name(wiq)?;

                // Write function parameters
                write!(self.out, "(")?;
                // Texture always first parameter
                self.write_image_type(wiq.dim, wiq.arrayed, wiq.class)?;
                write!(self.out, " {ARGUMENT_VARIABLE_NAME}")?;
                // Mipmap is a second parameter if exists
                if let ImageQuery::SizeLevel = wiq.query {
                    write!(self.out, ", uint {MIP_LEVEL_PARAM}")?;
                }
                writeln!(self.out, ")")?;

                // Write function body
                writeln!(self.out, "{{")?;

                let array_coords = usize::from(wiq.arrayed);
                // extra parameter is the mip level count or the sample count
                let extra_coords = match wiq.class {
                    crate::ImageClass::Storage { .. } => 0,
                    crate::ImageClass::Sampled { .. } | crate::ImageClass::Depth { .. } => 1,
                    crate::ImageClass::External => unreachable!(),
                };

                // GetDimensions Overloaded Methods
                // https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl-to-getdimensions#overloaded-methods
                let (ret_swizzle, number_of_params) = match wiq.query {
                    ImageQuery::Size | ImageQuery::SizeLevel => {
                        let ret = match wiq.dim {
                            IDim::D1 => "x",
                            IDim::D2 => "xy",
                            IDim::D3 => "xyz",
                            IDim::Cube => "xy",
                        };
                        (ret, ret.len() + array_coords + extra_coords)
                    }
                    ImageQuery::NumLevels | ImageQuery::NumSamples | ImageQuery::NumLayers => {
                        if wiq.arrayed || wiq.dim == IDim::D3 {
                            ("w", 4)
                        } else {
                            ("z", 3)
                        }
                    }
                };

                // Write `GetDimensions` function.
                writeln!(self.out, "{INDENT}uint4 {RETURN_VARIABLE_NAME};")?;
                write!(self.out, "{INDENT}{ARGUMENT_VARIABLE_NAME}.GetDimensions(")?;
                match wiq.query {
                    ImageQuery::SizeLevel => {
                        write!(self.out, "{MIP_LEVEL_PARAM}, ")?;
                    }
                    _ => match wiq.class {
                        crate::ImageClass::Sampled { multi: true, .. }
                        | crate::ImageClass::Depth { multi: true }
                        | crate::ImageClass::Storage { .. } => {}
                        _ => {
                            // Write zero mipmap level for supported types
                            write!(self.out, "0, ")?;
                        }
                    },
                }

                for component in COMPONENTS[..number_of_params - 1].iter() {
                    write!(self.out, "{RETURN_VARIABLE_NAME}.{component}, ")?;
                }

                // write last parameter without comma and space for last parameter
                write!(
                    self.out,
                    "{}.{}",
                    RETURN_VARIABLE_NAME,
                    COMPONENTS[number_of_params - 1]
                )?;

                writeln!(self.out, ");")?;

                // Write return value
                writeln!(
                    self.out,
                    "{INDENT}return {RETURN_VARIABLE_NAME}.{ret_swizzle};"
                )?;

                // End of function body
                writeln!(self.out, "}}")?;
                // Write extra new line
                writeln!(self.out)?;
            }
        }
        Ok(())
    }

    pub(super) fn write_wrapped_constructor_function_name(
        &mut self,
        module: &crate::Module,
        constructor: WrappedConstructor,
    ) -> BackendResult {
        let name = crate::TypeInner::hlsl_type_id(constructor.ty, module.to_ctx(), &self.names)?;
        write!(self.out, "Construct{name}")?;
        Ok(())
    }

    /// Helper function that write wrapped function for `Expression::Compose` for structures.
    fn write_wrapped_constructor_function(
        &mut self,
        module: &crate::Module,
        constructor: WrappedConstructor,
    ) -> BackendResult {
        use crate::back::INDENT;

        const ARGUMENT_VARIABLE_NAME: &str = "arg";
        const RETURN_VARIABLE_NAME: &str = "ret";

        // Write function return type and name
        if let crate::TypeInner::Array { base, size, .. } = module.types[constructor.ty].inner {
            write!(self.out, "typedef ")?;
            self.write_type(module, constructor.ty)?;
            write!(self.out, " ret_")?;
            self.write_wrapped_constructor_function_name(module, constructor)?;
            self.write_array_size(module, base, size)?;
            writeln!(self.out, ";")?;

            write!(self.out, "ret_")?;
            self.write_wrapped_constructor_function_name(module, constructor)?;
        } else {
            self.write_type(module, constructor.ty)?;
        }
        write!(self.out, " ")?;
        self.write_wrapped_constructor_function_name(module, constructor)?;

        // Write function parameters
        write!(self.out, "(")?;

        let mut write_arg = |i, ty| -> BackendResult {
            if i != 0 {
                write!(self.out, ", ")?;
            }
            self.write_type(module, ty)?;
            write!(self.out, " {ARGUMENT_VARIABLE_NAME}{i}")?;
            if let crate::TypeInner::Array { base, size, .. } = module.types[ty].inner {
                self.write_array_size(module, base, size)?;
            }
            Ok(())
        };

        match module.types[constructor.ty].inner {
            crate::TypeInner::Struct { ref members, .. } => {
                for (i, member) in members.iter().enumerate() {
                    write_arg(i, member.ty)?;
                }
            }
            crate::TypeInner::Array {
                base,
                size: crate::ArraySize::Constant(size),
                ..
            } => {
                for i in 0..size.get() as usize {
                    write_arg(i, base)?;
                }
            }
            _ => unreachable!(),
        };

        write!(self.out, ")")?;

        // Write function body
        writeln!(self.out, " {{")?;

        match module.types[constructor.ty].inner {
            crate::TypeInner::Struct { ref members, .. } => {
                let struct_name = &self.names[&NameKey::Type(constructor.ty)];
                writeln!(
                    self.out,
                    "{INDENT}{struct_name} {RETURN_VARIABLE_NAME} = ({struct_name})0;"
                )?;
                for (i, member) in members.iter().enumerate() {
                    let field_name = &self.names[&NameKey::StructMember(constructor.ty, i as u32)];

                    match module.types[member.ty].inner {
                        crate::TypeInner::Matrix {
                            columns,
                            rows: crate::VectorSize::Bi,
                            ..
                        } if member.binding.is_none() => {
                            for j in 0..columns as u8 {
                                writeln!(
                                    self.out,
                                    "{INDENT}{RETURN_VARIABLE_NAME}.{field_name}_{j} = {ARGUMENT_VARIABLE_NAME}{i}[{j}];"
                                )?;
                            }
                        }
                        ref other => {
                            // We cast arrays of native HLSL `floatCx2`s to arrays of `matCx2`s
                            // (where the inner matrix is represented by a struct with C `float2` members).
                            // See the module-level block comment in mod.rs for details.
                            if let Some(super::writer::MatrixType {
                                columns,
                                rows: crate::VectorSize::Bi,
                                width: 4,
                            }) = super::writer::get_inner_matrix_data(module, member.ty)
                            {
                                write!(
                                    self.out,
                                    "{}{}.{} = (__mat{}x2",
                                    INDENT, RETURN_VARIABLE_NAME, field_name, columns as u8
                                )?;
                                if let crate::TypeInner::Array { base, size, .. } = *other {
                                    self.write_array_size(module, base, size)?;
                                }
                                writeln!(self.out, "){ARGUMENT_VARIABLE_NAME}{i};",)?;
                            } else {
                                writeln!(
                                    self.out,
                                    "{INDENT}{RETURN_VARIABLE_NAME}.{field_name} = {ARGUMENT_VARIABLE_NAME}{i};",
                                )?;
                            }
                        }
                    }
                }
            }
            crate::TypeInner::Array {
                base,
                size: crate::ArraySize::Constant(size),
                ..
            } => {
                write!(self.out, "{INDENT}")?;
                self.write_type(module, base)?;
                write!(self.out, " {RETURN_VARIABLE_NAME}")?;
                self.write_array_size(module, base, crate::ArraySize::Constant(size))?;
                write!(self.out, " = {{ ")?;
                for i in 0..size.get() {
                    if i != 0 {
                        write!(self.out, ", ")?;
                    }
                    write!(self.out, "{ARGUMENT_VARIABLE_NAME}{i}")?;
                }
                writeln!(self.out, " }};",)?;
            }
            _ => unreachable!(),
        }

        // Write return value
        writeln!(self.out, "{INDENT}return {RETURN_VARIABLE_NAME};")?;

        // End of function body
        writeln!(self.out, "}}")?;
        // Write extra new line
        writeln!(self.out)?;

        Ok(())
    }

    /// Writes the conversion from a single length storage texture load to a vec4 with the loaded
    /// scalar in its `x` component, 1 in its `a` component and 0 everywhere else.
    fn write_loaded_scalar_to_storage_loaded_value(
        &mut self,
        scalar_type: crate::Scalar,
    ) -> BackendResult {
        const ARGUMENT_VARIABLE_NAME: &str = "arg";
        const RETURN_VARIABLE_NAME: &str = "ret";

        let zero;
        let one;
        match scalar_type.kind {
            ScalarKind::Sint => {
                assert_eq!(
                    scalar_type.width, 4,
                    "Scalar {scalar_type:?} is not a result from any storage format"
                );
                zero = "0";
                one = "1";
            }
            ScalarKind::Uint => match scalar_type.width {
                4 => {
                    zero = "0u";
                    one = "1u";
                }
                8 => {
                    zero = "0uL";
                    one = "1uL"
                }
                _ => unreachable!("Scalar {scalar_type:?} is not a result from any storage format"),
            },
            ScalarKind::Float => {
                assert_eq!(
                    scalar_type.width, 4,
                    "Scalar {scalar_type:?} is not a result from any storage format"
                );
                zero = "0.0";
                one = "1.0";
            }
            _ => unreachable!("Scalar {scalar_type:?} is not a result from any storage format"),
        }

        let ty = scalar_type.to_hlsl_str()?;
        writeln!(
            self.out,
            "{ty}4 {IMAGE_STORAGE_LOAD_SCALAR_WRAPPER}{ty}({ty} {ARGUMENT_VARIABLE_NAME}) {{\
    {ty}4 {RETURN_VARIABLE_NAME} = {ty}4({ARGUMENT_VARIABLE_NAME}, {zero}, {zero}, {one});\
    return {RETURN_VARIABLE_NAME};\
}}"
        )?;

        Ok(())
    }

    pub(super) fn write_wrapped_struct_matrix_get_function_name(
        &mut self,
        access: WrappedStructMatrixAccess,
    ) -> BackendResult {
        let name = &self.names[&NameKey::Type(access.ty)];
        let field_name = &self.names[&NameKey::StructMember(access.ty, access.index)];
        write!(self.out, "GetMat{field_name}On{name}")?;
        Ok(())
    }

    /// Writes a function used to get a matCx2 from within a structure.
    pub(super) fn write_wrapped_struct_matrix_get_function(
        &mut self,
        module: &crate::Module,
        access: WrappedStructMatrixAccess,
    ) -> BackendResult {
        use crate::back::INDENT;

        const STRUCT_ARGUMENT_VARIABLE_NAME: &str = "obj";

        // Write function return type and name
        let member = match module.types[access.ty].inner {
            crate::TypeInner::Struct { ref members, .. } => &members[access.index as usize],
            _ => unreachable!(),
        };
        let ret_ty = &module.types[member.ty].inner;
        self.write_value_type(module, ret_ty)?;
        write!(self.out, " ")?;
        self.write_wrapped_struct_matrix_get_function_name(access)?;

        // Write function parameters
        write!(self.out, "(")?;
        let struct_name = &self.names[&NameKey::Type(access.ty)];
        write!(self.out, "{struct_name} {STRUCT_ARGUMENT_VARIABLE_NAME}")?;

        // Write function body
        writeln!(self.out, ") {{")?;

        // Write return value
        write!(self.out, "{INDENT}return ")?;
        self.write_value_type(module, ret_ty)?;
        write!(self.out, "(")?;
        let field_name = &self.names[&NameKey::StructMember(access.ty, access.index)];
        match module.types[member.ty].inner {
            crate::TypeInner::Matrix { columns, .. } => {
                for i in 0..columns as u8 {
                    if i != 0 {
                        write!(self.out, ", ")?;
                    }
                    write!(self.out, "{STRUCT_ARGUMENT_VARIABLE_NAME}.{field_name}_{i}")?;
                }
            }
            _ => unreachable!(),
        }
        writeln!(self.out, ");")?;

        // End of function body
        writeln!(self.out, "}}")?;
        // Write extra new line
        writeln!(self.out)?;

        Ok(())
    }

    pub(super) fn write_wrapped_struct_matrix_set_function_name(
        &mut self,
        access: WrappedStructMatrixAccess,
    ) -> BackendResult {
        let name = &self.names[&NameKey::Type(access.ty)];
        let field_name = &self.names[&NameKey::StructMember(access.ty, access.index)];
        write!(self.out, "SetMat{field_name}On{name}")?;
        Ok(())
    }

    /// Writes a function used to set a matCx2 from within a structure.
    pub(super) fn write_wrapped_struct_matrix_set_function(
        &mut self,
        module: &crate::Module,
        access: WrappedStructMatrixAccess,
    ) -> BackendResult {
        use crate::back::INDENT;

        const STRUCT_ARGUMENT_VARIABLE_NAME: &str = "obj";
        const MATRIX_ARGUMENT_VARIABLE_NAME: &str = "mat";

        // Write function return type and name
        write!(self.out, "void ")?;
        self.write_wrapped_struct_matrix_set_function_name(access)?;

        // Write function parameters
        write!(self.out, "(")?;
        let struct_name = &self.names[&NameKey::Type(access.ty)];
        write!(self.out, "{struct_name} {STRUCT_ARGUMENT_VARIABLE_NAME}, ")?;
        let member = match module.types[access.ty].inner {
            crate::TypeInner::Struct { ref members, .. } => &members[access.index as usize],
            _ => unreachable!(),
        };
        self.write_type(module, member.ty)?;
        write!(self.out, " {MATRIX_ARGUMENT_VARIABLE_NAME}")?;
        // Write function body
        writeln!(self.out, ") {{")?;

        let field_name = &self.names[&NameKey::StructMember(access.ty, access.index)];

        match module.types[member.ty].inner {
            crate::TypeInner::Matrix { columns, .. } => {
                for i in 0..columns as u8 {
                    writeln!(
                        self.out,
                        "{INDENT}{STRUCT_ARGUMENT_VARIABLE_NAME}.{field_name}_{i} = {MATRIX_ARGUMENT_VARIABLE_NAME}[{i}];"
                    )?;
                }
            }
            _ => unreachable!(),
        }

        // End of function body
        writeln!(self.out, "}}")?;
        // Write extra new line
        writeln!(self.out)?;

        Ok(())
    }

    pub(super) fn write_wrapped_struct_matrix_set_vec_function_name(
        &mut self,
        access: WrappedStructMatrixAccess,
    ) -> BackendResult {
        let name = &self.names[&NameKey::Type(access.ty)];
        let field_name = &self.names[&NameKey::StructMember(access.ty, access.index)];
        write!(self.out, "SetMatVec{field_name}On{name}")?;
        Ok(())
    }

    /// Writes a function used to set a vec2 on a matCx2 from within a structure.
    pub(super) fn write_wrapped_struct_matrix_set_vec_function(
        &mut self,
        module: &crate::Module,
        access: WrappedStructMatrixAccess,
    ) -> BackendResult {
        use crate::back::INDENT;

        const STRUCT_ARGUMENT_VARIABLE_NAME: &str = "obj";
        const VECTOR_ARGUMENT_VARIABLE_NAME: &str = "vec";
        const MATRIX_INDEX_ARGUMENT_VARIABLE_NAME: &str = "mat_idx";

        // Write function return type and name
        write!(self.out, "void ")?;
        self.write_wrapped_struct_matrix_set_vec_function_name(access)?;

        // Write function parameters
        write!(self.out, "(")?;
        let struct_name = &self.names[&NameKey::Type(access.ty)];
        write!(self.out, "{struct_name} {STRUCT_ARGUMENT_VARIABLE_NAME}, ")?;
        let member = match module.types[access.ty].inner {
            crate::TypeInner::Struct { ref members, .. } => &members[access.index as usize],
            _ => unreachable!(),
        };
        let vec_ty = match module.types[member.ty].inner {
            crate::TypeInner::Matrix { rows, scalar, .. } => {
                crate::TypeInner::Vector { size: rows, scalar }
            }
            _ => unreachable!(),
        };
        self.write_value_type(module, &vec_ty)?;
        write!(
            self.out,
            " {VECTOR_ARGUMENT_VARIABLE_NAME}, uint {MATRIX_INDEX_ARGUMENT_VARIABLE_NAME}"
        )?;

        // Write function body
        writeln!(self.out, ") {{")?;

        writeln!(
            self.out,
            "{INDENT}switch({MATRIX_INDEX_ARGUMENT_VARIABLE_NAME}) {{"
        )?;

        let field_name = &self.names[&NameKey::StructMember(access.ty, access.index)];

        match module.types[member.ty].inner {
            crate::TypeInner::Matrix { columns, .. } => {
                for i in 0..columns as u8 {
                    writeln!(
                        self.out,
                        "{INDENT}case {i}: {{ {STRUCT_ARGUMENT_VARIABLE_NAME}.{field_name}_{i} = {VECTOR_ARGUMENT_VARIABLE_NAME}; break; }}"
                    )?;
                }
            }
            _ => unreachable!(),
        }

        writeln!(self.out, "{INDENT}}}")?;

        // End of function body
        writeln!(self.out, "}}")?;
        // Write extra new line
        writeln!(self.out)?;

        Ok(())
    }

    pub(super) fn write_wrapped_struct_matrix_set_scalar_function_name(
        &mut self,
        access: WrappedStructMatrixAccess,
    ) -> BackendResult {
        let name = &self.names[&NameKey::Type(access.ty)];
        let field_name = &self.names[&NameKey::StructMember(access.ty, access.index)];
        write!(self.out, "SetMatScalar{field_name}On{name}")?;
        Ok(())
    }

    /// Writes a function used to set a float on a matCx2 from within a structure.
    pub(super) fn write_wrapped_struct_matrix_set_scalar_function(
        &mut self,
        module: &crate::Module,
        access: WrappedStructMatrixAccess,
    ) -> BackendResult {
        use crate::back::INDENT;

        const STRUCT_ARGUMENT_VARIABLE_NAME: &str = "obj";
        const SCALAR_ARGUMENT_VARIABLE_NAME: &str = "scalar";
        const MATRIX_INDEX_ARGUMENT_VARIABLE_NAME: &str = "mat_idx";
        const VECTOR_INDEX_ARGUMENT_VARIABLE_NAME: &str = "vec_idx";

        // Write function return type and name
        write!(self.out, "void ")?;
        self.write_wrapped_struct_matrix_set_scalar_function_name(access)?;

        // Write function parameters
        write!(self.out, "(")?;
        let struct_name = &self.names[&NameKey::Type(access.ty)];
        write!(self.out, "{struct_name} {STRUCT_ARGUMENT_VARIABLE_NAME}, ")?;
        let member = match module.types[access.ty].inner {
            crate::TypeInner::Struct { ref members, .. } => &members[access.index as usize],
            _ => unreachable!(),
        };
        let scalar_ty = match module.types[member.ty].inner {
            crate::TypeInner::Matrix { scalar, .. } => crate::TypeInner::Scalar(scalar),
            _ => unreachable!(),
        };
        self.write_value_type(module, &scalar_ty)?;
        write!(
            self.out,
            " {SCALAR_ARGUMENT_VARIABLE_NAME}, uint {MATRIX_INDEX_ARGUMENT_VARIABLE_NAME}, uint {VECTOR_INDEX_ARGUMENT_VARIABLE_NAME}"
        )?;

        // Write function body
        writeln!(self.out, ") {{")?;

        writeln!(
            self.out,
            "{INDENT}switch({MATRIX_INDEX_ARGUMENT_VARIABLE_NAME}) {{"
        )?;

        let field_name = &self.names[&NameKey::StructMember(access.ty, access.index)];

        match module.types[member.ty].inner {
            crate::TypeInner::Matrix { columns, .. } => {
                for i in 0..columns as u8 {
                    writeln!(
                        self.out,
                        "{INDENT}case {i}: {{ {STRUCT_ARGUMENT_VARIABLE_NAME}.{field_name}_{i}[{VECTOR_INDEX_ARGUMENT_VARIABLE_NAME}] = {SCALAR_ARGUMENT_VARIABLE_NAME}; break; }}"
                    )?;
                }
            }
            _ => unreachable!(),
        }

        writeln!(self.out, "{INDENT}}}")?;

        // End of function body
        writeln!(self.out, "}}")?;
        // Write extra new line
        writeln!(self.out)?;

        Ok(())
    }

    /// Write functions to create special types.
    pub(super) fn write_special_functions(&mut self, module: &crate::Module) -> BackendResult {
        for (type_key, struct_ty) in module.special_types.predeclared_types.iter() {
            match type_key {
                &crate::PredeclaredType::ModfResult { size, scalar }
                | &crate::PredeclaredType::FrexpResult { size, scalar } => {
                    let arg_type_name_owner;
                    let arg_type_name = if let Some(size) = size {
                        arg_type_name_owner = format!(
                            "{}{}",
                            if scalar.width == 8 { "double" } else { "float" },
                            size as u8
                        );
                        &arg_type_name_owner
                    } else if scalar.width == 8 {
                        "double"
                    } else {
                        "float"
                    };

                    let (defined_func_name, called_func_name, second_field_name, sign_multiplier) =
                        if matches!(type_key, &crate::PredeclaredType::ModfResult { .. }) {
                            (super::writer::MODF_FUNCTION, "modf", "whole", "")
                        } else {
                            (
                                super::writer::FREXP_FUNCTION,
                                "frexp",
                                "exp_",
                                "sign(arg) * ",
                            )
                        };

                    let struct_name = &self.names[&NameKey::Type(*struct_ty)];

                    writeln!(
                        self.out,
                        "{struct_name} {defined_func_name}({arg_type_name} arg) {{
    {arg_type_name} other;
    {struct_name} result;
    result.fract = {sign_multiplier}{called_func_name}(arg, other);
    result.{second_field_name} = other;
    return result;
}}"
                    )?;
                    writeln!(self.out)?;
                }
                &crate::PredeclaredType::AtomicCompareExchangeWeakResult { .. } => {}
            }
        }
        if module.special_types.ray_desc.is_some() {
            self.write_ray_desc_from_ray_desc_constructor_function(module)?;
        }

        Ok(())
    }

    /// Helper function that writes wrapped functions for expressions in a function
    pub(super) fn write_wrapped_expression_functions(
        &mut self,
        module: &crate::Module,
        expressions: &crate::Arena<crate::Expression>,
        context: Option<&FunctionCtx>,
    ) -> BackendResult {
        for (handle, _) in expressions.iter() {
            match expressions[handle] {
                crate::Expression::Compose { ty, .. } => {
                    match module.types[ty].inner {
                        crate::TypeInner::Struct { .. } | crate::TypeInner::Array { .. } => {
                            let constructor = WrappedConstructor { ty };
                            if self.wrapped.insert(WrappedType::Constructor(constructor)) {
                                self.write_wrapped_constructor_function(module, constructor)?;
                            }
                        }
                        _ => {}
                    };
                }
                crate::Expression::ImageLoad { image, .. } => {
                    // This can only happen in a function as this is not a valid const expression
                    match *context.as_ref().unwrap().resolve_type(image, &module.types) {
                        crate::TypeInner::Image {
                            class: crate::ImageClass::Storage { format, .. },
                            ..
                        } => {
                            if format.single_component() {
                                let scalar: crate::Scalar = format.into();
                                if self.wrapped.insert(WrappedType::ImageLoadScalar(scalar)) {
                                    self.write_loaded_scalar_to_storage_loaded_value(scalar)?;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                crate::Expression::RayQueryGetIntersection { committed, .. } => {
                    if committed {
                        if !self.written_committed_intersection {
                            self.write_committed_intersection_function(module)?;
                            self.written_committed_intersection = true;
                        }
                    } else if !self.written_candidate_intersection {
                        self.write_candidate_intersection_function(module)?;
                        self.written_candidate_intersection = true;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    // TODO: we could merge this with iteration in write_wrapped_expression_functions...
    //
    /// Helper function that writes zero value wrapped functions
    pub(super) fn write_wrapped_zero_value_functions(
        &mut self,
        module: &crate::Module,
        expressions: &crate::Arena<crate::Expression>,
    ) -> BackendResult {
        for (handle, _) in expressions.iter() {
            if let crate::Expression::ZeroValue(ty) = expressions[handle] {
                let zero_value = WrappedZeroValue { ty };
                if self.wrapped.insert(WrappedType::ZeroValue(zero_value)) {
                    self.write_wrapped_zero_value_function(module, zero_value)?;
                }
            }
        }
        Ok(())
    }

    pub(super) fn write_wrapped_math_functions(
        &mut self,
        module: &crate::Module,
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        for (_, expression) in func_ctx.expressions.iter() {
            if let crate::Expression::Math {
                fun,
                arg,
                arg1: _arg1,
                arg2: _arg2,
                arg3: _arg3,
            } = *expression
            {
                let arg_ty = func_ctx.resolve_type(arg, &module.types);

                match fun {
                    crate::MathFunction::ExtractBits => {
                        // The behavior of our extractBits polyfill is undefined if offset + count > bit_width. We need
                        // to first sanitize the offset and count first. If we don't do this, we will get out-of-spec
                        // values if the extracted range is not within the bit width.
                        //
                        // This encodes the exact formula specified by the wgsl spec:
                        // https://gpuweb.github.io/gpuweb/wgsl/#extractBits-unsigned-builtin
                        //
                        // w = sizeof(x) * 8
                        // o = min(offset, w)
                        // c = min(count, w - o)
                        //
                        // bitfieldExtract(x, o, c)
                        let scalar = arg_ty.scalar().unwrap();
                        let components = arg_ty.components();

                        let wrapped = WrappedMath {
                            fun,
                            scalar,
                            components,
                        };

                        if !self.wrapped.insert(WrappedType::Math(wrapped)) {
                            continue;
                        }

                        // Write return type
                        self.write_value_type(module, arg_ty)?;

                        let scalar_width: u8 = scalar.width * 8;

                        // Write function name and parameters
                        writeln!(self.out, " {EXTRACT_BITS_FUNCTION}(")?;
                        write!(self.out, "    ")?;
                        self.write_value_type(module, arg_ty)?;
                        writeln!(self.out, " e,")?;
                        writeln!(self.out, "    uint offset,")?;
                        writeln!(self.out, "    uint count")?;
                        writeln!(self.out, ") {{")?;

                        // Write function body
                        writeln!(self.out, "    uint w = {scalar_width};")?;
                        writeln!(self.out, "    uint o = min(offset, w);")?;
                        writeln!(self.out, "    uint c = min(count, w - o);")?;
                        writeln!(
                            self.out,
                            "    return (c == 0 ? 0 : (e << (w - c - o)) >> (w - c));"
                        )?;

                        // End of function body
                        writeln!(self.out, "}}")?;
                    }
                    crate::MathFunction::InsertBits => {
                        // The behavior of our insertBits polyfill has the same constraints as the extractBits polyfill.

                        let scalar = arg_ty.scalar().unwrap();
                        let components = arg_ty.components();

                        let wrapped = WrappedMath {
                            fun,
                            scalar,
                            components,
                        };

                        if !self.wrapped.insert(WrappedType::Math(wrapped)) {
                            continue;
                        }

                        // Write return type
                        self.write_value_type(module, arg_ty)?;

                        let scalar_width: u8 = scalar.width * 8;
                        let scalar_max: u64 = match scalar.width {
                            1 => 0xFF,
                            2 => 0xFFFF,
                            4 => 0xFFFFFFFF,
                            8 => 0xFFFFFFFFFFFFFFFF,
                            _ => unreachable!(),
                        };

                        // Write function name and parameters
                        writeln!(self.out, " {INSERT_BITS_FUNCTION}(")?;
                        write!(self.out, "    ")?;
                        self.write_value_type(module, arg_ty)?;
                        writeln!(self.out, " e,")?;
                        write!(self.out, "    ")?;
                        self.write_value_type(module, arg_ty)?;
                        writeln!(self.out, " newbits,")?;
                        writeln!(self.out, "    uint offset,")?;
                        writeln!(self.out, "    uint count")?;
                        writeln!(self.out, ") {{")?;

                        // Write function body
                        writeln!(self.out, "    uint w = {scalar_width}u;")?;
                        writeln!(self.out, "    uint o = min(offset, w);")?;
                        writeln!(self.out, "    uint c = min(count, w - o);")?;

                        // The `u` suffix on the literals is _extremely_ important. Otherwise it will use
                        // i32 shifting instead of the intended u32 shifting.
                        writeln!(
                            self.out,
                            "    uint mask = (({scalar_max}u >> ({scalar_width}u - c)) << o);"
                        )?;
                        writeln!(
                            self.out,
                            "    return (c == 0 ? e : ((e & ~mask) | ((newbits << o) & mask)));"
                        )?;

                        // End of function body
                        writeln!(self.out, "}}")?;
                    }
                    // Taking the absolute value of the minimum value of a two's
                    // complement signed integer type causes overflow, which is
                    // undefined behaviour in HLSL. To avoid this, when the value is
                    // negative we bitcast the value to unsigned and negate it, then
                    // bitcast back to signed.
                    // This adheres to the WGSL spec in that the absolute of the type's
                    // minimum value should equal to the minimum value.
                    //
                    // TODO(#7109): asint()/asuint() only support 32-bit integers, so we
                    // must find another solution for different bit-widths.
                    crate::MathFunction::Abs
                        if matches!(arg_ty.scalar(), Some(crate::Scalar::I32)) =>
                    {
                        let scalar = arg_ty.scalar().unwrap();
                        let components = arg_ty.components();

                        let wrapped = WrappedMath {
                            fun,
                            scalar,
                            components,
                        };

                        if !self.wrapped.insert(WrappedType::Math(wrapped)) {
                            continue;
                        }

                        self.write_value_type(module, arg_ty)?;
                        write!(self.out, " {ABS_FUNCTION}(")?;
                        self.write_value_type(module, arg_ty)?;
                        writeln!(self.out, " val) {{")?;

                        let level = crate::back::Level(1);
                        writeln!(
                            self.out,
                            "{level}return val >= 0 ? val : asint(-asuint(val));"
                        )?;
                        writeln!(self.out, "}}")?;
                        writeln!(self.out)?;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub(super) fn write_wrapped_unary_ops(
        &mut self,
        module: &crate::Module,
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        for (_, expression) in func_ctx.expressions.iter() {
            if let crate::Expression::Unary { op, expr } = *expression {
                let expr_ty = func_ctx.resolve_type(expr, &module.types);
                let Some((vector_size, scalar)) = expr_ty.vector_size_and_scalar() else {
                    continue;
                };
                let wrapped = WrappedUnaryOp {
                    op,
                    ty: (vector_size, scalar),
                };

                // Negating the minimum value of a two's complement signed integer type
                // causes overflow, which is undefined behaviour in HLSL. To avoid this
                // we bitcast the value to unsigned and negate it, then bitcast back to
                // signed. This adheres to the WGSL spec in that the negative of the
                // type's minimum value should equal to the minimum value.
                //
                // TODO(#7109): asint()/asuint() only support 32-bit integers, so we must
                // find another solution for different bit-widths.
                match (op, scalar) {
                    (crate::UnaryOperator::Negate, crate::Scalar::I32) => {
                        if !self.wrapped.insert(WrappedType::UnaryOp(wrapped)) {
                            continue;
                        }

                        self.write_value_type(module, expr_ty)?;
                        write!(self.out, " {NEG_FUNCTION}(")?;
                        self.write_value_type(module, expr_ty)?;
                        writeln!(self.out, " val) {{")?;

                        let level = crate::back::Level(1);
                        writeln!(self.out, "{level}return asint(-asuint(val));",)?;
                        writeln!(self.out, "}}")?;
                        writeln!(self.out)?;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub(super) fn write_wrapped_binary_ops(
        &mut self,
        module: &crate::Module,
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        for (expr_handle, expression) in func_ctx.expressions.iter() {
            if let crate::Expression::Binary { op, left, right } = *expression {
                let expr_ty = func_ctx.resolve_type(expr_handle, &module.types);
                let left_ty = func_ctx.resolve_type(left, &module.types);
                let right_ty = func_ctx.resolve_type(right, &module.types);

                match (op, expr_ty.scalar()) {
                    // Signed integer division of the type's minimum representable value
                    // divided by -1, or signed or unsigned division by zero, is
                    // undefined behaviour in HLSL. We override the divisor to 1 in these
                    // cases.
                    // This adheres to the WGSL spec in that:
                    // * TYPE_MIN / -1 == TYPE_MIN
                    // * x / 0 == x
                    (
                        crate::BinaryOperator::Divide,
                        Some(
                            scalar @ crate::Scalar {
                                kind: ScalarKind::Sint | ScalarKind::Uint,
                                ..
                            },
                        ),
                    ) => {
                        let Some(left_wrapped_ty) = left_ty.vector_size_and_scalar() else {
                            continue;
                        };
                        let Some(right_wrapped_ty) = right_ty.vector_size_and_scalar() else {
                            continue;
                        };
                        let wrapped = WrappedBinaryOp {
                            op,
                            left_ty: left_wrapped_ty,
                            right_ty: right_wrapped_ty,
                        };
                        if !self.wrapped.insert(WrappedType::BinaryOp(wrapped)) {
                            continue;
                        }

                        self.write_value_type(module, expr_ty)?;
                        write!(self.out, " {DIV_FUNCTION}(")?;
                        self.write_value_type(module, left_ty)?;
                        write!(self.out, " lhs, ")?;
                        self.write_value_type(module, right_ty)?;
                        writeln!(self.out, " rhs) {{")?;
                        let level = crate::back::Level(1);
                        match scalar.kind {
                            ScalarKind::Sint => {
                                let min_val = match scalar.width {
                                    4 => crate::Literal::I32(i32::MIN),
                                    8 => crate::Literal::I64(i64::MIN),
                                    _ => {
                                        return Err(super::Error::UnsupportedScalar(scalar));
                                    }
                                };
                                write!(self.out, "{level}return lhs / (((lhs == ")?;
                                self.write_literal(min_val)?;
                                writeln!(self.out, " & rhs == -1) | (rhs == 0)) ? 1 : rhs);")?
                            }
                            ScalarKind::Uint => {
                                writeln!(self.out, "{level}return lhs / (rhs == 0u ? 1u : rhs);")?
                            }
                            _ => unreachable!(),
                        }
                        writeln!(self.out, "}}")?;
                        writeln!(self.out)?;
                    }
                    // The modulus operator is only defined for integers in HLSL when
                    // either both sides are positive or both sides are negative. To
                    // avoid this undefined behaviour we use the following equation:
                    //
                    // dividend - (dividend / divisor) * divisor
                    //
                    // overriding the divisor to 1 if either it is 0, or it is -1
                    // and the dividend is the minimum representable value.
                    //
                    // This adheres to the WGSL spec in that:
                    // * min_value % -1 == 0
                    // * x % 0 == 0
                    (
                        crate::BinaryOperator::Modulo,
                        Some(
                            scalar @ crate::Scalar {
                                kind: ScalarKind::Sint | ScalarKind::Uint | ScalarKind::Float,
                                ..
                            },
                        ),
                    ) => {
                        let Some(left_wrapped_ty) = left_ty.vector_size_and_scalar() else {
                            continue;
                        };
                        let Some(right_wrapped_ty) = right_ty.vector_size_and_scalar() else {
                            continue;
                        };
                        let wrapped = WrappedBinaryOp {
                            op,
                            left_ty: left_wrapped_ty,
                            right_ty: right_wrapped_ty,
                        };
                        if !self.wrapped.insert(WrappedType::BinaryOp(wrapped)) {
                            continue;
                        }

                        self.write_value_type(module, expr_ty)?;
                        write!(self.out, " {MOD_FUNCTION}(")?;
                        self.write_value_type(module, left_ty)?;
                        write!(self.out, " lhs, ")?;
                        self.write_value_type(module, right_ty)?;
                        writeln!(self.out, " rhs) {{")?;
                        let level = crate::back::Level(1);
                        match scalar.kind {
                            ScalarKind::Sint => {
                                let min_val = match scalar.width {
                                    4 => crate::Literal::I32(i32::MIN),
                                    8 => crate::Literal::I64(i64::MIN),
                                    _ => {
                                        return Err(super::Error::UnsupportedScalar(scalar));
                                    }
                                };
                                write!(self.out, "{level}")?;
                                self.write_value_type(module, right_ty)?;
                                write!(self.out, " divisor = ((lhs == ")?;
                                self.write_literal(min_val)?;
                                writeln!(self.out, " & rhs == -1) | (rhs == 0)) ? 1 : rhs;")?;
                                writeln!(
                                    self.out,
                                    "{level}return lhs - (lhs / divisor) * divisor;"
                                )?
                            }
                            ScalarKind::Uint => {
                                writeln!(self.out, "{level}return lhs % (rhs == 0u ? 1u : rhs);")?
                            }
                            // HLSL's fmod has the same definition as WGSL's % operator but due
                            // to its implementation in DXC it is not as accurate as the WGSL spec
                            // requires it to be. See:
                            // - https://shader-playground.timjones.io/0c8572816dbb6fc4435cc5d016a978a7
                            // - https://github.com/llvm/llvm-project/blob/50f9b8acafdca48e87e6b8e393c1f116a2d193ee/clang/lib/Headers/hlsl/hlsl_intrinsic_helpers.h#L78-L81
                            ScalarKind::Float => {
                                writeln!(self.out, "{level}return lhs - rhs * trunc(lhs / rhs);")?
                            }
                            _ => unreachable!(),
                        }
                        writeln!(self.out, "}}")?;
                        writeln!(self.out)?;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn write_wrapped_cast_functions(
        &mut self,
        module: &crate::Module,
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        for (_, expression) in func_ctx.expressions.iter() {
            if let crate::Expression::As {
                expr,
                kind,
                convert: Some(width),
            } = *expression
            {
                // Avoid undefined behaviour when casting from a float to integer
                // when the value is out of range for the target type. Additionally
                // ensure we clamp to the correct value as per the WGSL spec.
                //
                // https://www.w3.org/TR/WGSL/#floating-point-conversion:
                // * If X is exactly representable in the target type T, then the
                //   result is that value.
                // * Otherwise, the result is the value in T closest to
                //   truncate(X) and also exactly representable in the original
                //   floating point type.
                let src_ty = func_ctx.resolve_type(expr, &module.types);
                let Some((vector_size, src_scalar)) = src_ty.vector_size_and_scalar() else {
                    continue;
                };
                let dst_scalar = crate::Scalar { kind, width };
                if src_scalar.kind != ScalarKind::Float
                    || (dst_scalar.kind != ScalarKind::Sint && dst_scalar.kind != ScalarKind::Uint)
                {
                    continue;
                }

                let wrapped = WrappedCast {
                    src_scalar,
                    vector_size,
                    dst_scalar,
                };
                if !self.wrapped.insert(WrappedType::Cast(wrapped)) {
                    continue;
                }

                let (src_ty, dst_ty) = match vector_size {
                    None => (
                        crate::TypeInner::Scalar(src_scalar),
                        crate::TypeInner::Scalar(dst_scalar),
                    ),
                    Some(vector_size) => (
                        crate::TypeInner::Vector {
                            scalar: src_scalar,
                            size: vector_size,
                        },
                        crate::TypeInner::Vector {
                            scalar: dst_scalar,
                            size: vector_size,
                        },
                    ),
                };
                let (min, max) =
                    crate::proc::min_max_float_representable_by(src_scalar, dst_scalar);
                let cast_str = format!(
                    "{}{}",
                    dst_scalar.to_hlsl_str()?,
                    vector_size
                        .map(crate::common::vector_size_str)
                        .unwrap_or(""),
                );
                let fun_name = match dst_scalar {
                    crate::Scalar::I32 => F2I32_FUNCTION,
                    crate::Scalar::U32 => F2U32_FUNCTION,
                    crate::Scalar::I64 => F2I64_FUNCTION,
                    crate::Scalar::U64 => F2U64_FUNCTION,
                    _ => unreachable!(),
                };
                self.write_value_type(module, &dst_ty)?;
                write!(self.out, " {fun_name}(")?;
                self.write_value_type(module, &src_ty)?;
                writeln!(self.out, " value) {{")?;
                let level = crate::back::Level(1);
                write!(self.out, "{level}return {cast_str}(clamp(value, ")?;
                self.write_literal(min)?;
                write!(self.out, ", ")?;
                self.write_literal(max)?;
                writeln!(self.out, "));",)?;
                writeln!(self.out, "}}")?;
                writeln!(self.out)?;
            }
        }
        Ok(())
    }

    /// Helper function that writes various wrapped functions
    pub(super) fn write_wrapped_functions(
        &mut self,
        module: &crate::Module,
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        self.write_wrapped_math_functions(module, func_ctx)?;
        self.write_wrapped_unary_ops(module, func_ctx)?;
        self.write_wrapped_binary_ops(module, func_ctx)?;
        self.write_wrapped_expression_functions(module, func_ctx.expressions, Some(func_ctx))?;
        self.write_wrapped_zero_value_functions(module, func_ctx.expressions)?;
        self.write_wrapped_cast_functions(module, func_ctx)?;

        for (handle, _) in func_ctx.expressions.iter() {
            match func_ctx.expressions[handle] {
                crate::Expression::ArrayLength(expr) => {
                    let global_expr = match func_ctx.expressions[expr] {
                        crate::Expression::GlobalVariable(_) => expr,
                        crate::Expression::AccessIndex { base, index: _ } => base,
                        ref other => unreachable!("Array length of {:?}", other),
                    };
                    let global_var = match func_ctx.expressions[global_expr] {
                        crate::Expression::GlobalVariable(var_handle) => {
                            &module.global_variables[var_handle]
                        }
                        ref other => {
                            return Err(super::Error::Unimplemented(format!(
                                "Array length of base {other:?}"
                            )))
                        }
                    };
                    let storage_access = match global_var.space {
                        crate::AddressSpace::Storage { access } => access,
                        _ => crate::StorageAccess::default(),
                    };
                    let wal = WrappedArrayLength {
                        writable: storage_access.contains(crate::StorageAccess::STORE),
                    };

                    if self.wrapped.insert(WrappedType::ArrayLength(wal)) {
                        self.write_wrapped_array_length_function(wal)?;
                    }
                }
                crate::Expression::ImageLoad { image, .. } => {
                    let class = match *func_ctx.resolve_type(image, &module.types) {
                        crate::TypeInner::Image { class, .. } => class,
                        _ => unreachable!(),
                    };
                    let wrapped = WrappedImageLoad { class };
                    if self.wrapped.insert(WrappedType::ImageLoad(wrapped)) {
                        self.write_wrapped_image_load_function(module, wrapped)?;
                    }
                }
                crate::Expression::ImageSample {
                    image,
                    clamp_to_edge,
                    ..
                } => {
                    let class = match *func_ctx.resolve_type(image, &module.types) {
                        crate::TypeInner::Image { class, .. } => class,
                        _ => unreachable!(),
                    };
                    let wrapped = WrappedImageSample {
                        class,
                        clamp_to_edge,
                    };
                    if self.wrapped.insert(WrappedType::ImageSample(wrapped)) {
                        self.write_wrapped_image_sample_function(module, wrapped)?;
                    }
                }
                crate::Expression::ImageQuery { image, query } => {
                    let wiq = match *func_ctx.resolve_type(image, &module.types) {
                        crate::TypeInner::Image {
                            dim,
                            arrayed,
                            class,
                        } => WrappedImageQuery {
                            dim,
                            arrayed,
                            class,
                            query: query.into(),
                        },
                        _ => unreachable!("we only query images"),
                    };

                    if self.wrapped.insert(WrappedType::ImageQuery(wiq)) {
                        self.write_wrapped_image_query_function(module, wiq, handle, func_ctx)?;
                    }
                }
                // Write `WrappedConstructor` for structs that are loaded from `AddressSpace::Storage`
                // since they will later be used by the fn `write_storage_load`
                crate::Expression::Load { pointer } => {
                    let pointer_space = func_ctx
                        .resolve_type(pointer, &module.types)
                        .pointer_space();

                    if let Some(crate::AddressSpace::Storage { .. }) = pointer_space {
                        if let Some(ty) = func_ctx.info[handle].ty.handle() {
                            write_wrapped_constructor(self, ty, module)?;
                        }
                    }

                    fn write_wrapped_constructor<W: Write>(
                        writer: &mut super::Writer<'_, W>,
                        ty: Handle<crate::Type>,
                        module: &crate::Module,
                    ) -> BackendResult {
                        match module.types[ty].inner {
                            crate::TypeInner::Struct { ref members, .. } => {
                                for member in members {
                                    write_wrapped_constructor(writer, member.ty, module)?;
                                }

                                let constructor = WrappedConstructor { ty };
                                if writer.wrapped.insert(WrappedType::Constructor(constructor)) {
                                    writer
                                        .write_wrapped_constructor_function(module, constructor)?;
                                }
                            }
                            crate::TypeInner::Array { base, .. } => {
                                write_wrapped_constructor(writer, base, module)?;

                                let constructor = WrappedConstructor { ty };
                                if writer.wrapped.insert(WrappedType::Constructor(constructor)) {
                                    writer
                                        .write_wrapped_constructor_function(module, constructor)?;
                                }
                            }
                            _ => {}
                        };

                        Ok(())
                    }
                }
                // We treat matrices of the form `matCx2` as a sequence of C `vec2`s
                // (see top level module docs for details).
                //
                // The functions injected here are required to get the matrix accesses working.
                crate::Expression::AccessIndex { base, index } => {
                    let base_ty_res = &func_ctx.info[base].ty;
                    let mut resolved = base_ty_res.inner_with(&module.types);
                    let base_ty_handle = match *resolved {
                        crate::TypeInner::Pointer { base, .. } => {
                            resolved = &module.types[base].inner;
                            Some(base)
                        }
                        _ => base_ty_res.handle(),
                    };
                    if let crate::TypeInner::Struct { ref members, .. } = *resolved {
                        let member = &members[index as usize];

                        match module.types[member.ty].inner {
                            crate::TypeInner::Matrix {
                                rows: crate::VectorSize::Bi,
                                ..
                            } if member.binding.is_none() => {
                                let ty = base_ty_handle.unwrap();
                                let access = WrappedStructMatrixAccess { ty, index };

                                if self.wrapped.insert(WrappedType::StructMatrixAccess(access)) {
                                    self.write_wrapped_struct_matrix_get_function(module, access)?;
                                    self.write_wrapped_struct_matrix_set_function(module, access)?;
                                    self.write_wrapped_struct_matrix_set_vec_function(
                                        module, access,
                                    )?;
                                    self.write_wrapped_struct_matrix_set_scalar_function(
                                        module, access,
                                    )?;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
        }

        Ok(())
    }

    /// Writes out the sampler heap declarations if they haven't been written yet.
    pub(super) fn write_sampler_heaps(&mut self) -> BackendResult {
        if self.wrapped.sampler_heaps {
            return Ok(());
        }

        writeln!(
            self.out,
            "SamplerState {}[2048]: register(s{}, space{});",
            super::writer::SAMPLER_HEAP_VAR,
            self.options.sampler_heap_target.standard_samplers.register,
            self.options.sampler_heap_target.standard_samplers.space
        )?;
        writeln!(
            self.out,
            "SamplerComparisonState {}[2048]: register(s{}, space{});",
            super::writer::COMPARISON_SAMPLER_HEAP_VAR,
            self.options
                .sampler_heap_target
                .comparison_samplers
                .register,
            self.options.sampler_heap_target.comparison_samplers.space
        )?;

        self.wrapped.sampler_heaps = true;

        Ok(())
    }

    /// Writes out the sampler index buffer declaration if it hasn't been written yet.
    pub(super) fn write_wrapped_sampler_buffer(
        &mut self,
        key: super::SamplerIndexBufferKey,
    ) -> BackendResult {
        // The astute will notice that we do a double hash lookup, but we do this to avoid
        // holding a mutable reference to `self` while trying to call `write_sampler_heaps`.
        //
        // We only pay this double lookup cost when we actually need to write out the sampler
        // buffer, which should be not be common.

        if self.wrapped.sampler_index_buffers.contains_key(&key) {
            return Ok(());
        };

        self.write_sampler_heaps()?;

        // Because the group number can be arbitrary, we use the namer to generate a unique name
        // instead of adding it to the reserved name list.
        let sampler_array_name = self
            .namer
            .call(&format!("nagaGroup{}SamplerIndexArray", key.group));

        let bind_target = match self.options.sampler_buffer_binding_map.get(&key) {
            Some(&bind_target) => bind_target,
            None if self.options.fake_missing_bindings => super::BindTarget {
                space: u8::MAX,
                register: key.group,
                binding_array_size: None,
                dynamic_storage_buffer_offsets_index: None,
                restrict_indexing: false,
            },
            None => {
                unreachable!("Sampler buffer of group {key:?} not bound to a register");
            }
        };

        writeln!(
            self.out,
            "StructuredBuffer<uint> {sampler_array_name} : register(t{}, space{});",
            bind_target.register, bind_target.space
        )?;

        self.wrapped
            .sampler_index_buffers
            .insert(key, sampler_array_name);

        Ok(())
    }

    pub(super) fn write_texture_coordinates(
        &mut self,
        kind: &str,
        coordinate: Handle<crate::Expression>,
        array_index: Option<Handle<crate::Expression>>,
        mip_level: Option<Handle<crate::Expression>>,
        module: &crate::Module,
        func_ctx: &FunctionCtx,
    ) -> BackendResult {
        // HLSL expects the array index to be merged with the coordinate
        let extra = array_index.is_some() as usize + (mip_level.is_some()) as usize;
        if extra == 0 {
            self.write_expr(module, coordinate, func_ctx)?;
        } else {
            let num_coords = match *func_ctx.resolve_type(coordinate, &module.types) {
                crate::TypeInner::Scalar { .. } => 1,
                crate::TypeInner::Vector { size, .. } => size as usize,
                _ => unreachable!(),
            };
            write!(self.out, "{}{}(", kind, num_coords + extra)?;
            self.write_expr(module, coordinate, func_ctx)?;
            if let Some(expr) = array_index {
                write!(self.out, ", ")?;
                self.write_expr(module, expr, func_ctx)?;
            }
            if let Some(expr) = mip_level {
                // Explicit cast if needed
                let cast_to_int = matches!(
                    *func_ctx.resolve_type(expr, &module.types),
                    crate::TypeInner::Scalar(crate::Scalar {
                        kind: ScalarKind::Uint,
                        ..
                    })
                );

                write!(self.out, ", ")?;

                if cast_to_int {
                    write!(self.out, "int(")?;
                }

                self.write_expr(module, expr, func_ctx)?;

                if cast_to_int {
                    write!(self.out, ")")?;
                }
            }
            write!(self.out, ")")?;
        }
        Ok(())
    }

    pub(super) fn write_mat_cx2_typedef_and_functions(
        &mut self,
        WrappedMatCx2 { columns }: WrappedMatCx2,
    ) -> BackendResult {
        use crate::back::INDENT;

        // typedef
        write!(self.out, "typedef struct {{ ")?;
        for i in 0..columns as u8 {
            write!(self.out, "float2 _{i}; ")?;
        }
        writeln!(self.out, "}} __mat{}x2;", columns as u8)?;

        // __get_col_of_mat
        writeln!(
            self.out,
            "float2 __get_col_of_mat{}x2(__mat{}x2 mat, uint idx) {{",
            columns as u8, columns as u8
        )?;
        writeln!(self.out, "{INDENT}switch(idx) {{")?;
        for i in 0..columns as u8 {
            writeln!(self.out, "{INDENT}case {i}: {{ return mat._{i}; }}")?;
        }
        writeln!(self.out, "{INDENT}default: {{ return (float2)0; }}")?;
        writeln!(self.out, "{INDENT}}}")?;
        writeln!(self.out, "}}")?;

        // __set_col_of_mat
        writeln!(
            self.out,
            "void __set_col_of_mat{}x2(__mat{}x2 mat, uint idx, float2 value) {{",
            columns as u8, columns as u8
        )?;
        writeln!(self.out, "{INDENT}switch(idx) {{")?;
        for i in 0..columns as u8 {
            writeln!(self.out, "{INDENT}case {i}: {{ mat._{i} = value; break; }}")?;
        }
        writeln!(self.out, "{INDENT}}}")?;
        writeln!(self.out, "}}")?;

        // __set_el_of_mat
        writeln!(
            self.out,
            "void __set_el_of_mat{}x2(__mat{}x2 mat, uint idx, uint vec_idx, float value) {{",
            columns as u8, columns as u8
        )?;
        writeln!(self.out, "{INDENT}switch(idx) {{")?;
        for i in 0..columns as u8 {
            writeln!(
                self.out,
                "{INDENT}case {i}: {{ mat._{i}[vec_idx] = value; break; }}"
            )?;
        }
        writeln!(self.out, "{INDENT}}}")?;
        writeln!(self.out, "}}")?;

        writeln!(self.out)?;

        Ok(())
    }

    pub(super) fn write_all_mat_cx2_typedefs_and_functions(
        &mut self,
        module: &crate::Module,
    ) -> BackendResult {
        for (handle, _) in module.global_variables.iter() {
            let global = &module.global_variables[handle];

            if global.space == crate::AddressSpace::Uniform {
                if let Some(super::writer::MatrixType {
                    columns,
                    rows: crate::VectorSize::Bi,
                    width: 4,
                }) = super::writer::get_inner_matrix_data(module, global.ty)
                {
                    let entry = WrappedMatCx2 { columns };
                    if self.wrapped.insert(WrappedType::MatCx2(entry)) {
                        self.write_mat_cx2_typedef_and_functions(entry)?;
                    }
                }
            }
        }

        for (_, ty) in module.types.iter() {
            if let crate::TypeInner::Struct { ref members, .. } = ty.inner {
                for member in members.iter() {
                    if let crate::TypeInner::Array { .. } = module.types[member.ty].inner {
                        if let Some(super::writer::MatrixType {
                            columns,
                            rows: crate::VectorSize::Bi,
                            width: 4,
                        }) = super::writer::get_inner_matrix_data(module, member.ty)
                        {
                            let entry = WrappedMatCx2 { columns };
                            if self.wrapped.insert(WrappedType::MatCx2(entry)) {
                                self.write_mat_cx2_typedef_and_functions(entry)?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub(super) fn write_wrapped_zero_value_function_name(
        &mut self,
        module: &crate::Module,
        zero_value: WrappedZeroValue,
    ) -> BackendResult {
        let name = crate::TypeInner::hlsl_type_id(zero_value.ty, module.to_ctx(), &self.names)?;
        write!(self.out, "ZeroValue{name}")?;
        Ok(())
    }

    /// Helper function that write wrapped function for `Expression::ZeroValue`
    ///
    /// This is necessary since we might have a member access after the zero value expression, e.g.
    /// `.y` (in practice this can come up when consuming SPIRV that's been produced by glslc).
    ///
    /// So we can't just write `(float4)0` since `(float4)0.y` won't parse correctly.
    ///
    /// Parenthesizing the expression like `((float4)0).y` would work... except DXC can't handle
    /// cases like:
    ///
    /// ```text
    /// tests\out\hlsl\access.hlsl:183:41: error: cannot compile this l-value expression yet
    ///     t_1.am = (__mat4x2[2])((float4x2[2])0);
    ///                                         ^
    /// ```
    fn write_wrapped_zero_value_function(
        &mut self,
        module: &crate::Module,
        zero_value: WrappedZeroValue,
    ) -> BackendResult {
        use crate::back::INDENT;

        // Write function return type and name
        if let crate::TypeInner::Array { base, size, .. } = module.types[zero_value.ty].inner {
            write!(self.out, "typedef ")?;
            self.write_type(module, zero_value.ty)?;
            write!(self.out, " ret_")?;
            self.write_wrapped_zero_value_function_name(module, zero_value)?;
            self.write_array_size(module, base, size)?;
            writeln!(self.out, ";")?;

            write!(self.out, "ret_")?;
            self.write_wrapped_zero_value_function_name(module, zero_value)?;
        } else {
            self.write_type(module, zero_value.ty)?;
        }
        write!(self.out, " ")?;
        self.write_wrapped_zero_value_function_name(module, zero_value)?;

        // Write function parameters (none) and start function body
        writeln!(self.out, "() {{")?;

        // Write `ZeroValue` function.
        write!(self.out, "{INDENT}return ")?;
        self.write_default_init(module, zero_value.ty)?;
        writeln!(self.out, ";")?;

        // End of function body
        writeln!(self.out, "}}")?;
        // Write extra new line
        writeln!(self.out)?;

        Ok(())
    }
}

impl crate::StorageFormat {
    /// Returns `true` if there is just one component, otherwise `false`
    pub(super) const fn single_component(&self) -> bool {
        match *self {
            crate::StorageFormat::R16Float
            | crate::StorageFormat::R32Float
            | crate::StorageFormat::R8Unorm
            | crate::StorageFormat::R16Unorm
            | crate::StorageFormat::R8Snorm
            | crate::StorageFormat::R16Snorm
            | crate::StorageFormat::R8Uint
            | crate::StorageFormat::R16Uint
            | crate::StorageFormat::R32Uint
            | crate::StorageFormat::R8Sint
            | crate::StorageFormat::R16Sint
            | crate::StorageFormat::R32Sint
            | crate::StorageFormat::R64Uint => true,
            _ => false,
        }
    }
}
