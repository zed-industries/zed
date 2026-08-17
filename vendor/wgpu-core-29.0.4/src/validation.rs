use alloc::{
    boxed::Box,
    string::{String, ToString as _},
    sync::Arc,
    vec::Vec,
};
use core::fmt;

use arrayvec::ArrayVec;
use hashbrown::hash_map::Entry;
use shader_io_deductions::{display_deductions_as_optional_list, MaxVertexShaderOutputDeduction};
use thiserror::Error;
use wgt::{
    error::{ErrorType, WebGpuError},
    BindGroupLayoutEntry, BindingType,
};

use crate::{
    device::bgl, resource::InvalidResourceError,
    validation::shader_io_deductions::MaxFragmentShaderInputDeduction, FastHashMap, FastHashSet,
};

pub mod shader_io_deductions;

#[derive(Debug)]
enum ResourceType {
    Buffer {
        size: wgt::BufferSize,
    },
    Texture {
        dim: naga::ImageDimension,
        arrayed: bool,
        class: naga::ImageClass,
    },
    Sampler {
        comparison: bool,
    },
    AccelerationStructure {
        vertex_return: bool,
    },
}

#[derive(Clone, Debug)]
pub enum BindingTypeName {
    Buffer,
    Texture,
    Sampler,
    AccelerationStructure,
    ExternalTexture,
}

impl From<&ResourceType> for BindingTypeName {
    fn from(ty: &ResourceType) -> BindingTypeName {
        match ty {
            ResourceType::Buffer { .. } => BindingTypeName::Buffer,
            ResourceType::Texture {
                class: naga::ImageClass::External,
                ..
            } => BindingTypeName::ExternalTexture,
            ResourceType::Texture { .. } => BindingTypeName::Texture,
            ResourceType::Sampler { .. } => BindingTypeName::Sampler,
            ResourceType::AccelerationStructure { .. } => BindingTypeName::AccelerationStructure,
        }
    }
}

impl From<&BindingType> for BindingTypeName {
    fn from(ty: &BindingType) -> BindingTypeName {
        match ty {
            BindingType::Buffer { .. } => BindingTypeName::Buffer,
            BindingType::Texture { .. } => BindingTypeName::Texture,
            BindingType::StorageTexture { .. } => BindingTypeName::Texture,
            BindingType::Sampler { .. } => BindingTypeName::Sampler,
            BindingType::AccelerationStructure { .. } => BindingTypeName::AccelerationStructure,
            BindingType::ExternalTexture => BindingTypeName::ExternalTexture,
        }
    }
}

#[derive(Debug)]
struct Resource {
    #[allow(unused)]
    name: Option<String>,
    bind: naga::ResourceBinding,
    ty: ResourceType,
    class: naga::AddressSpace,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NumericDimension {
    Scalar,
    Vector(naga::VectorSize),
    Matrix(naga::VectorSize, naga::VectorSize),
}

impl fmt::Display for NumericDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Scalar => write!(f, ""),
            Self::Vector(size) => write!(f, "x{}", size as u8),
            Self::Matrix(columns, rows) => write!(f, "x{}{}", columns as u8, rows as u8),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NumericType {
    dim: NumericDimension,
    scalar: naga::Scalar,
}

impl fmt::Display for NumericType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}{}{}",
            self.scalar.kind,
            self.scalar.width * 8,
            self.dim
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterfaceVar {
    pub ty: NumericType,
    interpolation: Option<naga::Interpolation>,
    sampling: Option<naga::Sampling>,
    per_primitive: bool,
}

impl InterfaceVar {
    pub fn vertex_attribute(format: wgt::VertexFormat) -> Self {
        InterfaceVar {
            ty: NumericType::from_vertex_format(format),
            interpolation: None,
            sampling: None,
            per_primitive: false,
        }
    }
}

impl fmt::Display for InterfaceVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} interpolated as {:?} with sampling {:?}",
            self.ty, self.interpolation, self.sampling
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Varying {
    Local { location: u32, iv: InterfaceVar },
    BuiltIn(naga::BuiltIn),
}

#[allow(unused)]
#[derive(Debug)]
struct SpecializationConstant {
    id: u32,
    ty: NumericType,
}

#[derive(Debug)]
struct EntryPointMeshInfo {
    max_vertices: u32,
    max_primitives: u32,
}

#[derive(Debug, Default)]
struct EntryPoint {
    inputs: Vec<Varying>,
    outputs: Vec<Varying>,
    resources: Vec<naga::Handle<Resource>>,
    #[allow(unused)]
    spec_constants: Vec<SpecializationConstant>,
    sampling_pairs: FastHashSet<(naga::Handle<Resource>, naga::Handle<Resource>)>,
    workgroup_size: [u32; 3],
    dual_source_blending: bool,
    task_payload_size: Option<u32>,
    mesh_info: Option<EntryPointMeshInfo>,
}

#[derive(Debug)]
pub struct Interface {
    limits: wgt::Limits,
    resources: naga::Arena<Resource>,
    entry_points: FastHashMap<(naga::ShaderStage, String), EntryPoint>,
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum BindingError {
    #[error("Binding is missing from the pipeline layout")]
    Missing,
    #[error("Visibility flags don't include the shader stage")]
    Invisible,
    #[error(
        "Type on the shader side ({shader:?}) does not match the pipeline binding ({binding:?})"
    )]
    WrongType {
        binding: BindingTypeName,
        shader: BindingTypeName,
    },
    #[error("Storage class {binding:?} doesn't match the shader {shader:?}")]
    WrongAddressSpace {
        binding: naga::AddressSpace,
        shader: naga::AddressSpace,
    },
    #[error("Address space {space:?} is not a valid Buffer address space")]
    WrongBufferAddressSpace { space: naga::AddressSpace },
    #[error("Buffer structure size {buffer_size}, added to one element of an unbound array, if it's the last field, ended up greater than the given `min_binding_size`, which is {min_binding_size}")]
    WrongBufferSize {
        buffer_size: wgt::BufferSize,
        min_binding_size: wgt::BufferSize,
    },
    #[error("View dimension {dim:?} (is array: {is_array}) doesn't match the binding {binding:?}")]
    WrongTextureViewDimension {
        dim: naga::ImageDimension,
        is_array: bool,
        binding: BindingType,
    },
    #[error("Texture class {binding:?} doesn't match the shader {shader:?}")]
    WrongTextureClass {
        binding: naga::ImageClass,
        shader: naga::ImageClass,
    },
    #[error("Comparison flag doesn't match the shader")]
    WrongSamplerComparison,
    #[error("Derived bind group layout type is not consistent between stages")]
    InconsistentlyDerivedType,
    #[error("Texture format {0:?} is not supported for storage use")]
    BadStorageFormat(wgt::TextureFormat),
}

impl WebGpuError for BindingError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum FilteringError {
    #[error("Integer textures can't be sampled with a filtering sampler")]
    Integer,
    #[error("Non-filterable float textures can't be sampled with a filtering sampler")]
    Float,
}

impl WebGpuError for FilteringError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum InputError {
    #[error("Input is not provided by the earlier stage in the pipeline")]
    Missing,
    #[error("Input type is not compatible with the provided {0}")]
    WrongType(NumericType),
    #[error("Input interpolation doesn't match provided {0:?}")]
    InterpolationMismatch(Option<naga::Interpolation>),
    #[error("Input sampling doesn't match provided {0:?}")]
    SamplingMismatch(Option<naga::Sampling>),
    #[error("Pipeline input has per_primitive={pipeline_input}, but shader expects per_primitive={shader}")]
    WrongPerPrimitive { pipeline_input: bool, shader: bool },
}

impl WebGpuError for InputError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

/// Errors produced when validating a programmable stage of a pipeline.
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum StageError {
    #[error(
        "Shader entry point's workgroup size {dimensions:?} ({total} total invocations) must be \
        less or equal to the per-dimension limit `Limits::{per_dimension_limits_desc}` of \
        {per_dimension_limits:?} and the total invocation limit `Limits::{total_limit_desc}` of \
        {total_limit}"
    )]
    InvalidWorkgroupSize {
        dimensions: [u32; 3],
        per_dimension_limits: [u32; 3],
        per_dimension_limits_desc: &'static str,
        total: u32,
        total_limit: u32,
        total_limit_desc: &'static str,
    },
    #[error("Unable to find entry point '{0}'")]
    MissingEntryPoint(String),
    #[error("Shader global {0:?} is not available in the pipeline layout")]
    Binding(naga::ResourceBinding, #[source] BindingError),
    #[error("Unable to filter the texture ({texture:?}) by the sampler ({sampler:?})")]
    Filtering {
        texture: naga::ResourceBinding,
        sampler: naga::ResourceBinding,
        #[source]
        error: FilteringError,
    },
    #[error("Location[{location}] {var} is not provided by the previous stage outputs")]
    Input {
        location: wgt::ShaderLocation,
        var: InterfaceVar,
        #[source]
        error: InputError,
    },
    #[error(
        "Unable to select an entry point: no entry point was found in the provided shader module"
    )]
    NoEntryPointFound,
    #[error(
        "Unable to select an entry point: \
        multiple entry points were found in the provided shader module, \
        but no entry point was specified"
    )]
    MultipleEntryPointsFound,
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
    #[error(
        "vertex shader output location Location[{location}] ({var}) exceeds the \
        `max_inter_stage_shader_variables` limit ({}, 0-based){}",
        // NOTE: Remember: the limit is 0-based for indices.
        limit - 1,
        display_deductions_as_optional_list(deductions, |d| d.for_location())
    )]
    VertexOutputLocationTooLarge {
        location: u32,
        var: InterfaceVar,
        limit: u32,
        deductions: Vec<MaxVertexShaderOutputDeduction>,
    },
    #[error(
        "found {num_found} user-defined vertex shader output variables, which exceeds the \
        `max_inter_stage_shader_variables` limit ({limit}){}",
        display_deductions_as_optional_list(deductions, |d| d.for_variables())
    )]
    TooManyUserDefinedVertexOutputs {
        num_found: u32,
        limit: u32,
        deductions: Vec<MaxVertexShaderOutputDeduction>,
    },
    #[error(
        "fragment shader input location Location[{location}] ({var}) exceeds the \
        `max_inter_stage_shader_variables` limit ({}, 0-based){}",
        // NOTE: Remember: the limit is 0-based for indices.
        limit - 1,
        // NOTE: WebGPU spec. validation for fragment inputs is expressed in terms of variables
        // (unlike vertex outputs), so we use `MaxFragmentShaderInputDeduction::for_variables` here
        // (and not a non-existent `for_locations`).
        display_deductions_as_optional_list(deductions, |d| d.for_variables())
    )]
    FragmentInputLocationTooLarge {
        location: u32,
        var: InterfaceVar,
        limit: u32,
        deductions: Vec<MaxFragmentShaderInputDeduction>,
    },
    #[error(
        "found {num_found} user-defined fragment shader input variables, which exceeds the \
        `max_inter_stage_shader_variables` limit ({limit}){}",
        display_deductions_as_optional_list(deductions, |d| d.for_variables())
    )]
    TooManyUserDefinedFragmentInputs {
        num_found: u32,
        limit: u32,
        deductions: Vec<MaxFragmentShaderInputDeduction>,
    },
    #[error(
        "Location[{location}] {var}'s index exceeds the `max_color_attachments` limit ({limit})"
    )]
    ColorAttachmentLocationTooLarge {
        location: u32,
        var: InterfaceVar,
        limit: u32,
    },
    #[error("Mesh shaders are limited to {limit} output vertices by `Limits::max_mesh_output_vertices`, but the shader has a maximum number of {value}")]
    TooManyMeshVertices { limit: u32, value: u32 },
    #[error("Mesh shaders are limited to {limit} output primitives by `Limits::max_mesh_output_primitives`, but the shader has a maximum number of {value}")]
    TooManyMeshPrimitives { limit: u32, value: u32 },
    #[error("Mesh or task shaders are limited to {limit} bytes of task payload by `Limits::max_task_payload_size`, but the shader has a task payload of size {value}")]
    TaskPayloadTooLarge { limit: u32, value: u32 },
    #[error("Mesh shader's task payload has size ({shader:?}), which doesn't match the payload declared in the task stage ({input:?})")]
    TaskPayloadMustMatch {
        input: Option<u32>,
        shader: Option<u32>,
    },
    #[error("Primitive index can only be used in a fragment shader if the preceding shader was a vertex shader or a mesh shader that writes to primitive index.")]
    InvalidPrimitiveIndex,
    #[error("If a mesh shader writes to primitive index, it must be read by the fragment shader.")]
    MissingPrimitiveIndex,
    #[error("DrawId cannot be used in the same pipeline as a task shader")]
    DrawIdError,
    #[error("Pipeline uses dual-source blending, but the shader does not support it")]
    InvalidDualSourceBlending,
    #[error("Fragment shader writes depth, but pipeline does not have a depth attachment")]
    MissingFragDepthAttachment,
}

impl WebGpuError for StageError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Binding(_, e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::Filtering {
                texture: _,
                sampler: _,
                error,
            } => error.webgpu_error_type(),
            Self::Input {
                location: _,
                var: _,
                error,
            } => error.webgpu_error_type(),
            Self::InvalidWorkgroupSize { .. }
            | Self::MissingEntryPoint(..)
            | Self::NoEntryPointFound
            | Self::MultipleEntryPointsFound
            | Self::VertexOutputLocationTooLarge { .. }
            | Self::TooManyUserDefinedVertexOutputs { .. }
            | Self::FragmentInputLocationTooLarge { .. }
            | Self::TooManyUserDefinedFragmentInputs { .. }
            | Self::ColorAttachmentLocationTooLarge { .. }
            | Self::TooManyMeshVertices { .. }
            | Self::TooManyMeshPrimitives { .. }
            | Self::TaskPayloadTooLarge { .. }
            | Self::TaskPayloadMustMatch { .. }
            | Self::InvalidPrimitiveIndex
            | Self::MissingPrimitiveIndex
            | Self::DrawIdError
            | Self::InvalidDualSourceBlending
            | Self::MissingFragDepthAttachment => ErrorType::Validation,
        }
    }
}

pub use wgpu_naga_bridge::map_storage_format_from_naga;
pub use wgpu_naga_bridge::map_storage_format_to_naga;

impl Resource {
    fn check_binding_use(&self, entry: &BindGroupLayoutEntry) -> Result<(), BindingError> {
        match self.ty {
            ResourceType::Buffer { size } => {
                let min_size = match entry.ty {
                    BindingType::Buffer {
                        ty,
                        has_dynamic_offset: _,
                        min_binding_size,
                    } => {
                        let class = match ty {
                            wgt::BufferBindingType::Uniform => naga::AddressSpace::Uniform,
                            wgt::BufferBindingType::Storage { read_only } => {
                                let mut naga_access = naga::StorageAccess::LOAD;
                                naga_access.set(naga::StorageAccess::STORE, !read_only);
                                naga::AddressSpace::Storage {
                                    access: naga_access,
                                }
                            }
                        };
                        if self.class != class {
                            return Err(BindingError::WrongAddressSpace {
                                binding: class,
                                shader: self.class,
                            });
                        }
                        min_binding_size
                    }
                    _ => {
                        return Err(BindingError::WrongType {
                            binding: (&entry.ty).into(),
                            shader: (&self.ty).into(),
                        })
                    }
                };
                match min_size {
                    Some(non_zero) if non_zero < size => {
                        return Err(BindingError::WrongBufferSize {
                            buffer_size: size,
                            min_binding_size: non_zero,
                        })
                    }
                    _ => (),
                }
            }
            ResourceType::Sampler { comparison } => match entry.ty {
                BindingType::Sampler(ty) => {
                    if (ty == wgt::SamplerBindingType::Comparison) != comparison {
                        return Err(BindingError::WrongSamplerComparison);
                    }
                }
                _ => {
                    return Err(BindingError::WrongType {
                        binding: (&entry.ty).into(),
                        shader: (&self.ty).into(),
                    })
                }
            },
            ResourceType::Texture {
                dim,
                arrayed,
                class: shader_class,
            } => {
                let view_dimension = match entry.ty {
                    BindingType::Texture { view_dimension, .. }
                    | BindingType::StorageTexture { view_dimension, .. } => view_dimension,
                    BindingType::ExternalTexture => wgt::TextureViewDimension::D2,
                    _ => {
                        return Err(BindingError::WrongTextureViewDimension {
                            dim,
                            is_array: false,
                            binding: entry.ty,
                        })
                    }
                };
                if arrayed {
                    match (dim, view_dimension) {
                        (naga::ImageDimension::D2, wgt::TextureViewDimension::D2Array) => (),
                        (naga::ImageDimension::Cube, wgt::TextureViewDimension::CubeArray) => (),
                        _ => {
                            return Err(BindingError::WrongTextureViewDimension {
                                dim,
                                is_array: true,
                                binding: entry.ty,
                            })
                        }
                    }
                } else {
                    match (dim, view_dimension) {
                        (naga::ImageDimension::D1, wgt::TextureViewDimension::D1) => (),
                        (naga::ImageDimension::D2, wgt::TextureViewDimension::D2) => (),
                        (naga::ImageDimension::D3, wgt::TextureViewDimension::D3) => (),
                        (naga::ImageDimension::Cube, wgt::TextureViewDimension::Cube) => (),
                        _ => {
                            return Err(BindingError::WrongTextureViewDimension {
                                dim,
                                is_array: false,
                                binding: entry.ty,
                            })
                        }
                    }
                }
                match entry.ty {
                    BindingType::Texture {
                        sample_type,
                        view_dimension: _,
                        multisampled: multi,
                    } => {
                        let binding_class = match sample_type {
                            wgt::TextureSampleType::Float { .. } => naga::ImageClass::Sampled {
                                kind: naga::ScalarKind::Float,
                                multi,
                            },
                            wgt::TextureSampleType::Sint => naga::ImageClass::Sampled {
                                kind: naga::ScalarKind::Sint,
                                multi,
                            },
                            wgt::TextureSampleType::Uint => naga::ImageClass::Sampled {
                                kind: naga::ScalarKind::Uint,
                                multi,
                            },
                            wgt::TextureSampleType::Depth => naga::ImageClass::Depth { multi },
                        };
                        if shader_class == binding_class {
                            Ok(())
                        } else {
                            Err(binding_class)
                        }
                    }
                    BindingType::StorageTexture {
                        access: wgt_binding_access,
                        format: wgt_binding_format,
                        view_dimension: _,
                    } => {
                        const LOAD_STORE: naga::StorageAccess =
                            naga::StorageAccess::LOAD.union(naga::StorageAccess::STORE);
                        let binding_format = map_storage_format_to_naga(wgt_binding_format)
                            .ok_or(BindingError::BadStorageFormat(wgt_binding_format))?;
                        let binding_access = match wgt_binding_access {
                            wgt::StorageTextureAccess::ReadOnly => naga::StorageAccess::LOAD,
                            wgt::StorageTextureAccess::WriteOnly => naga::StorageAccess::STORE,
                            wgt::StorageTextureAccess::ReadWrite => LOAD_STORE,
                            wgt::StorageTextureAccess::Atomic => {
                                naga::StorageAccess::ATOMIC | LOAD_STORE
                            }
                        };
                        match shader_class {
                            // Formats must match exactly. A write-only shader (but not a
                            // read-only shader) is compatible with a read-write binding.
                            naga::ImageClass::Storage {
                                format: shader_format,
                                access: shader_access,
                            } if shader_format == binding_format
                                && (shader_access == binding_access
                                    || shader_access == naga::StorageAccess::STORE
                                        && binding_access == LOAD_STORE) =>
                            {
                                Ok(())
                            }
                            _ => Err(naga::ImageClass::Storage {
                                format: binding_format,
                                access: binding_access,
                            }),
                        }
                    }
                    BindingType::ExternalTexture => {
                        let binding_class = naga::ImageClass::External;
                        if shader_class == binding_class {
                            Ok(())
                        } else {
                            Err(binding_class)
                        }
                    }
                    _ => {
                        return Err(BindingError::WrongType {
                            binding: (&entry.ty).into(),
                            shader: (&self.ty).into(),
                        })
                    }
                }
                .map_err(|binding_class| BindingError::WrongTextureClass {
                    binding: binding_class,
                    shader: shader_class,
                })?;
            }
            ResourceType::AccelerationStructure { vertex_return } => match entry.ty {
                BindingType::AccelerationStructure {
                    vertex_return: entry_vertex_return,
                } if vertex_return == entry_vertex_return => (),
                _ => {
                    return Err(BindingError::WrongType {
                        binding: (&entry.ty).into(),
                        shader: (&self.ty).into(),
                    })
                }
            },
        };

        Ok(())
    }

    fn derive_binding_type(
        &self,
        is_reffed_by_sampler_in_entrypoint: bool,
    ) -> Result<BindingType, BindingError> {
        Ok(match self.ty {
            ResourceType::Buffer { size } => BindingType::Buffer {
                ty: match self.class {
                    naga::AddressSpace::Uniform => wgt::BufferBindingType::Uniform,
                    naga::AddressSpace::Storage { access } => wgt::BufferBindingType::Storage {
                        read_only: access == naga::StorageAccess::LOAD,
                    },
                    _ => return Err(BindingError::WrongBufferAddressSpace { space: self.class }),
                },
                has_dynamic_offset: false,
                min_binding_size: Some(size),
            },
            ResourceType::Sampler { comparison } => BindingType::Sampler(if comparison {
                wgt::SamplerBindingType::Comparison
            } else {
                wgt::SamplerBindingType::Filtering
            }),
            ResourceType::Texture {
                dim,
                arrayed,
                class,
            } => {
                let view_dimension = match dim {
                    naga::ImageDimension::D1 => wgt::TextureViewDimension::D1,
                    naga::ImageDimension::D2 if arrayed => wgt::TextureViewDimension::D2Array,
                    naga::ImageDimension::D2 => wgt::TextureViewDimension::D2,
                    naga::ImageDimension::D3 => wgt::TextureViewDimension::D3,
                    naga::ImageDimension::Cube if arrayed => wgt::TextureViewDimension::CubeArray,
                    naga::ImageDimension::Cube => wgt::TextureViewDimension::Cube,
                };
                match class {
                    naga::ImageClass::Sampled { multi, kind } => BindingType::Texture {
                        sample_type: match kind {
                            naga::ScalarKind::Float => wgt::TextureSampleType::Float {
                                filterable: is_reffed_by_sampler_in_entrypoint,
                            },
                            naga::ScalarKind::Sint => wgt::TextureSampleType::Sint,
                            naga::ScalarKind::Uint => wgt::TextureSampleType::Uint,
                            naga::ScalarKind::AbstractInt
                            | naga::ScalarKind::AbstractFloat
                            | naga::ScalarKind::Bool => unreachable!(),
                        },
                        view_dimension,
                        multisampled: multi,
                    },
                    naga::ImageClass::Depth { multi } => BindingType::Texture {
                        sample_type: wgt::TextureSampleType::Depth,
                        view_dimension,
                        multisampled: multi,
                    },
                    naga::ImageClass::Storage { format, access } => BindingType::StorageTexture {
                        access: {
                            const LOAD_STORE: naga::StorageAccess =
                                naga::StorageAccess::LOAD.union(naga::StorageAccess::STORE);
                            match access {
                                naga::StorageAccess::LOAD => wgt::StorageTextureAccess::ReadOnly,
                                naga::StorageAccess::STORE => wgt::StorageTextureAccess::WriteOnly,
                                LOAD_STORE => wgt::StorageTextureAccess::ReadWrite,
                                _ if access.contains(naga::StorageAccess::ATOMIC) => {
                                    wgt::StorageTextureAccess::Atomic
                                }
                                _ => unreachable!(),
                            }
                        },
                        view_dimension,
                        format: {
                            let f = map_storage_format_from_naga(format);
                            let original = map_storage_format_to_naga(f)
                                .ok_or(BindingError::BadStorageFormat(f))?;
                            debug_assert_eq!(format, original);
                            f
                        },
                    },
                    naga::ImageClass::External => BindingType::ExternalTexture,
                }
            }
            ResourceType::AccelerationStructure { vertex_return } => {
                BindingType::AccelerationStructure { vertex_return }
            }
        })
    }
}

impl NumericType {
    fn from_vertex_format(format: wgt::VertexFormat) -> Self {
        use naga::{Scalar, VectorSize as Vs};
        use wgt::VertexFormat as Vf;

        let (dim, scalar) = match format {
            Vf::Uint8 | Vf::Uint16 | Vf::Uint32 => (NumericDimension::Scalar, Scalar::U32),
            Vf::Uint8x2 | Vf::Uint16x2 | Vf::Uint32x2 => {
                (NumericDimension::Vector(Vs::Bi), Scalar::U32)
            }
            Vf::Uint32x3 => (NumericDimension::Vector(Vs::Tri), Scalar::U32),
            Vf::Uint8x4 | Vf::Uint16x4 | Vf::Uint32x4 => {
                (NumericDimension::Vector(Vs::Quad), Scalar::U32)
            }
            Vf::Sint8 | Vf::Sint16 | Vf::Sint32 => (NumericDimension::Scalar, Scalar::I32),
            Vf::Sint8x2 | Vf::Sint16x2 | Vf::Sint32x2 => {
                (NumericDimension::Vector(Vs::Bi), Scalar::I32)
            }
            Vf::Sint32x3 => (NumericDimension::Vector(Vs::Tri), Scalar::I32),
            Vf::Sint8x4 | Vf::Sint16x4 | Vf::Sint32x4 => {
                (NumericDimension::Vector(Vs::Quad), Scalar::I32)
            }
            Vf::Unorm8 | Vf::Unorm16 | Vf::Snorm8 | Vf::Snorm16 | Vf::Float16 | Vf::Float32 => {
                (NumericDimension::Scalar, Scalar::F32)
            }
            Vf::Unorm8x2
            | Vf::Snorm8x2
            | Vf::Unorm16x2
            | Vf::Snorm16x2
            | Vf::Float16x2
            | Vf::Float32x2 => (NumericDimension::Vector(Vs::Bi), Scalar::F32),
            Vf::Float32x3 => (NumericDimension::Vector(Vs::Tri), Scalar::F32),
            Vf::Unorm8x4
            | Vf::Snorm8x4
            | Vf::Unorm16x4
            | Vf::Snorm16x4
            | Vf::Float16x4
            | Vf::Float32x4
            | Vf::Unorm10_10_10_2
            | Vf::Unorm8x4Bgra => (NumericDimension::Vector(Vs::Quad), Scalar::F32),
            Vf::Float64 => (NumericDimension::Scalar, Scalar::F64),
            Vf::Float64x2 => (NumericDimension::Vector(Vs::Bi), Scalar::F64),
            Vf::Float64x3 => (NumericDimension::Vector(Vs::Tri), Scalar::F64),
            Vf::Float64x4 => (NumericDimension::Vector(Vs::Quad), Scalar::F64),
        };

        NumericType {
            dim,
            //Note: Shader always sees data as int, uint, or float.
            // It doesn't know if the original is normalized in a tighter form.
            scalar,
        }
    }

    fn from_texture_format(format: wgt::TextureFormat) -> Self {
        use naga::{Scalar, VectorSize as Vs};
        use wgt::TextureFormat as Tf;

        let (dim, scalar) = match format {
            Tf::R8Unorm | Tf::R8Snorm | Tf::R16Float | Tf::R32Float => {
                (NumericDimension::Scalar, Scalar::F32)
            }
            Tf::R8Uint | Tf::R16Uint | Tf::R32Uint => (NumericDimension::Scalar, Scalar::U32),
            Tf::R8Sint | Tf::R16Sint | Tf::R32Sint => (NumericDimension::Scalar, Scalar::I32),
            Tf::Rg8Unorm | Tf::Rg8Snorm | Tf::Rg16Float | Tf::Rg32Float => {
                (NumericDimension::Vector(Vs::Bi), Scalar::F32)
            }
            Tf::R64Uint => (NumericDimension::Scalar, Scalar::U64),
            Tf::Rg8Uint | Tf::Rg16Uint | Tf::Rg32Uint => {
                (NumericDimension::Vector(Vs::Bi), Scalar::U32)
            }
            Tf::Rg8Sint | Tf::Rg16Sint | Tf::Rg32Sint => {
                (NumericDimension::Vector(Vs::Bi), Scalar::I32)
            }
            Tf::R16Snorm | Tf::R16Unorm => (NumericDimension::Scalar, Scalar::F32),
            Tf::Rg16Snorm | Tf::Rg16Unorm => (NumericDimension::Vector(Vs::Bi), Scalar::F32),
            Tf::Rgba16Snorm | Tf::Rgba16Unorm => (NumericDimension::Vector(Vs::Quad), Scalar::F32),
            Tf::Rgba8Unorm
            | Tf::Rgba8UnormSrgb
            | Tf::Rgba8Snorm
            | Tf::Bgra8Unorm
            | Tf::Bgra8UnormSrgb
            | Tf::Rgb10a2Unorm
            | Tf::Rgba16Float
            | Tf::Rgba32Float => (NumericDimension::Vector(Vs::Quad), Scalar::F32),
            Tf::Rgba8Uint | Tf::Rgba16Uint | Tf::Rgba32Uint | Tf::Rgb10a2Uint => {
                (NumericDimension::Vector(Vs::Quad), Scalar::U32)
            }
            Tf::Rgba8Sint | Tf::Rgba16Sint | Tf::Rgba32Sint => {
                (NumericDimension::Vector(Vs::Quad), Scalar::I32)
            }
            Tf::Rg11b10Ufloat => (NumericDimension::Vector(Vs::Tri), Scalar::F32),
            Tf::Stencil8
            | Tf::Depth16Unorm
            | Tf::Depth32Float
            | Tf::Depth32FloatStencil8
            | Tf::Depth24Plus
            | Tf::Depth24PlusStencil8 => {
                panic!("Unexpected depth format")
            }
            Tf::NV12 => panic!("Unexpected nv12 format"),
            Tf::P010 => panic!("Unexpected p010 format"),
            Tf::Rgb9e5Ufloat => (NumericDimension::Vector(Vs::Tri), Scalar::F32),
            Tf::Bc1RgbaUnorm
            | Tf::Bc1RgbaUnormSrgb
            | Tf::Bc2RgbaUnorm
            | Tf::Bc2RgbaUnormSrgb
            | Tf::Bc3RgbaUnorm
            | Tf::Bc3RgbaUnormSrgb
            | Tf::Bc7RgbaUnorm
            | Tf::Bc7RgbaUnormSrgb
            | Tf::Etc2Rgb8A1Unorm
            | Tf::Etc2Rgb8A1UnormSrgb
            | Tf::Etc2Rgba8Unorm
            | Tf::Etc2Rgba8UnormSrgb => (NumericDimension::Vector(Vs::Quad), Scalar::F32),
            Tf::Bc4RUnorm | Tf::Bc4RSnorm | Tf::EacR11Unorm | Tf::EacR11Snorm => {
                (NumericDimension::Scalar, Scalar::F32)
            }
            Tf::Bc5RgUnorm | Tf::Bc5RgSnorm | Tf::EacRg11Unorm | Tf::EacRg11Snorm => {
                (NumericDimension::Vector(Vs::Bi), Scalar::F32)
            }
            Tf::Bc6hRgbUfloat | Tf::Bc6hRgbFloat | Tf::Etc2Rgb8Unorm | Tf::Etc2Rgb8UnormSrgb => {
                (NumericDimension::Vector(Vs::Tri), Scalar::F32)
            }
            Tf::Astc {
                block: _,
                channel: _,
            } => (NumericDimension::Vector(Vs::Quad), Scalar::F32),
        };

        NumericType {
            dim,
            //Note: Shader always sees data as int, uint, or float.
            // It doesn't know if the original is normalized in a tighter form.
            scalar,
        }
    }

    fn is_subtype_of(&self, other: &NumericType) -> bool {
        if self.scalar.width > other.scalar.width {
            return false;
        }
        if self.scalar.kind != other.scalar.kind {
            return false;
        }
        match (self.dim, other.dim) {
            (NumericDimension::Scalar, NumericDimension::Scalar) => true,
            (NumericDimension::Scalar, NumericDimension::Vector(_)) => true,
            (NumericDimension::Vector(s0), NumericDimension::Vector(s1)) => s0 <= s1,
            (NumericDimension::Matrix(c0, r0), NumericDimension::Matrix(c1, r1)) => {
                c0 == c1 && r0 == r1
            }
            _ => false,
        }
    }
}

/// Return true if the fragment `format` is covered by the provided `output`.
pub fn check_texture_format(
    format: wgt::TextureFormat,
    output: &NumericType,
) -> Result<(), NumericType> {
    let nt = NumericType::from_texture_format(format);
    if nt.is_subtype_of(output) {
        Ok(())
    } else {
        Err(nt)
    }
}

pub enum BindingLayoutSource {
    /// The binding layout is derived from the pipeline layout.
    ///
    /// This will be filled in by the shader binding validation, as it iterates the shader's interfaces.
    Derived(Box<ArrayVec<bgl::EntryMap, { hal::MAX_BIND_GROUPS }>>),
    /// The binding layout is provided by the user in BGLs.
    ///
    /// This will be validated against the shader's interfaces.
    Provided(Arc<crate::binding_model::PipelineLayout>),
}

impl BindingLayoutSource {
    pub fn new_derived(limits: &wgt::Limits) -> Self {
        let mut array = ArrayVec::new();
        for _ in 0..limits.max_bind_groups {
            array.push(Default::default());
        }
        BindingLayoutSource::Derived(Box::new(array))
    }
}

#[derive(Debug, Clone, Default)]
pub struct StageIo {
    pub varyings: FastHashMap<wgt::ShaderLocation, InterfaceVar>,
    /// This must match between mesh & task shaders
    pub task_payload_size: Option<u32>,
    /// Fragment shaders cannot input primitive index on mesh shaders that don't output it on DX12.
    /// Therefore, we track between shader stages if primitive index is written (or if vertex shader
    /// is used).
    ///
    /// This is Some if it was a mesh shader.
    pub primitive_index: Option<bool>,
}

impl Interface {
    fn populate(
        list: &mut Vec<Varying>,
        binding: Option<&naga::Binding>,
        ty: naga::Handle<naga::Type>,
        arena: &naga::UniqueArena<naga::Type>,
    ) {
        let numeric_ty = match arena[ty].inner {
            naga::TypeInner::Scalar(scalar) => NumericType {
                dim: NumericDimension::Scalar,
                scalar,
            },
            naga::TypeInner::Vector { size, scalar } => NumericType {
                dim: NumericDimension::Vector(size),
                scalar,
            },
            naga::TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } => NumericType {
                dim: NumericDimension::Matrix(columns, rows),
                scalar,
            },
            naga::TypeInner::Struct { ref members, .. } => {
                for member in members {
                    Self::populate(list, member.binding.as_ref(), member.ty, arena);
                }
                return;
            }
            ref other => {
                //Note: technically this should be at least `log::error`, but
                // the reality is - every shader coming from `glslc` outputs an array
                // of clip distances and hits this path :(
                // So we lower it to `log::debug` to be less annoying as
                // there's nothing the user can do about it.
                log::debug!("Unexpected varying type: {other:?}");
                return;
            }
        };

        let varying = match binding {
            Some(&naga::Binding::Location {
                location,
                interpolation,
                sampling,
                per_primitive,
                blend_src: _,
            }) => Varying::Local {
                location,
                iv: InterfaceVar {
                    ty: numeric_ty,
                    interpolation,
                    sampling,
                    per_primitive,
                },
            },
            Some(&naga::Binding::BuiltIn(built_in)) => Varying::BuiltIn(built_in),
            None => {
                log::error!("Missing binding for a varying");
                return;
            }
        };
        list.push(varying);
    }

    pub fn new(module: &naga::Module, info: &naga::valid::ModuleInfo, limits: wgt::Limits) -> Self {
        let mut resources = naga::Arena::new();
        let mut resource_mapping = FastHashMap::default();
        for (var_handle, var) in module.global_variables.iter() {
            let bind = match var.binding {
                Some(br) => br,
                _ => continue,
            };
            let naga_ty = &module.types[var.ty].inner;

            let inner_ty = match *naga_ty {
                naga::TypeInner::BindingArray { base, .. } => &module.types[base].inner,
                ref ty => ty,
            };

            let ty = match *inner_ty {
                naga::TypeInner::Image {
                    dim,
                    arrayed,
                    class,
                } => ResourceType::Texture {
                    dim,
                    arrayed,
                    class,
                },
                naga::TypeInner::Sampler { comparison } => ResourceType::Sampler { comparison },
                naga::TypeInner::AccelerationStructure { vertex_return } => {
                    ResourceType::AccelerationStructure { vertex_return }
                }
                ref other => ResourceType::Buffer {
                    size: wgt::BufferSize::new(other.size(module.to_ctx()) as u64).unwrap(),
                },
            };
            let handle = resources.append(
                Resource {
                    name: var.name.clone(),
                    bind,
                    ty,
                    class: var.space,
                },
                Default::default(),
            );
            resource_mapping.insert(var_handle, handle);
        }

        let mut entry_points = FastHashMap::default();
        entry_points.reserve(module.entry_points.len());
        for (index, entry_point) in module.entry_points.iter().enumerate() {
            let info = info.get_entry_point(index);
            let mut ep = EntryPoint::default();
            for arg in entry_point.function.arguments.iter() {
                Self::populate(&mut ep.inputs, arg.binding.as_ref(), arg.ty, &module.types);
            }
            if let Some(ref result) = entry_point.function.result {
                Self::populate(
                    &mut ep.outputs,
                    result.binding.as_ref(),
                    result.ty,
                    &module.types,
                );
            }

            for (var_handle, var) in module.global_variables.iter() {
                let usage = info[var_handle];
                if !usage.is_empty() && var.binding.is_some() {
                    ep.resources.push(resource_mapping[&var_handle]);
                }
            }

            for key in info.sampling_set.iter() {
                ep.sampling_pairs
                    .insert((resource_mapping[&key.image], resource_mapping[&key.sampler]));
            }
            ep.dual_source_blending = info.dual_source_blending;
            ep.workgroup_size = entry_point.workgroup_size;

            if let Some(task_payload) = entry_point.task_payload {
                ep.task_payload_size = Some(
                    module.types[module.global_variables[task_payload].ty]
                        .inner
                        .size(module.to_ctx()),
                );
            }
            if let Some(ref mesh_info) = entry_point.mesh_info {
                ep.mesh_info = Some(EntryPointMeshInfo {
                    max_vertices: mesh_info.max_vertices,
                    max_primitives: mesh_info.max_primitives,
                });
                Self::populate(
                    &mut ep.outputs,
                    None,
                    mesh_info.vertex_output_type,
                    &module.types,
                );
                Self::populate(
                    &mut ep.outputs,
                    None,
                    mesh_info.primitive_output_type,
                    &module.types,
                );
            }

            entry_points.insert((entry_point.stage, entry_point.name.clone()), ep);
        }

        Self {
            limits,
            resources,
            entry_points,
        }
    }

    pub fn finalize_entry_point_name(
        &self,
        stage: naga::ShaderStage,
        entry_point_name: Option<&str>,
    ) -> Result<String, StageError> {
        entry_point_name
            .map(|ep| ep.to_string())
            .map(Ok)
            .unwrap_or_else(|| {
                let mut entry_points = self
                    .entry_points
                    .keys()
                    .filter_map(|(ep_stage, name)| (ep_stage == &stage).then_some(name));
                let first = entry_points.next().ok_or(StageError::NoEntryPointFound)?;
                if entry_points.next().is_some() {
                    return Err(StageError::MultipleEntryPointsFound);
                }
                Ok(first.clone())
            })
    }

    /// Among other things, this implements some validation logic defined by the WebGPU spec. at
    /// <https://www.w3.org/TR/webgpu/#abstract-opdef-validating-inter-stage-interfaces>.
    pub fn check_stage(
        &self,
        layouts: &mut BindingLayoutSource,
        shader_binding_sizes: &mut FastHashMap<naga::ResourceBinding, wgt::BufferSize>,
        entry_point_name: &str,
        shader_stage: ShaderStageForValidation,
        inputs: StageIo,
    ) -> Result<StageIo, StageError> {
        // Since a shader module can have multiple entry points with the same name,
        // we need to look for one with the right execution model.
        let pair = (shader_stage.to_naga(), entry_point_name.to_string());
        let entry_point = match self.entry_points.get(&pair) {
            Some(some) => some,
            None => return Err(StageError::MissingEntryPoint(pair.1)),
        };
        let (_, entry_point_name) = pair;

        let stage_bit = shader_stage.to_wgt_bit();

        // check resources visibility
        for &handle in entry_point.resources.iter() {
            let res = &self.resources[handle];
            let result = 'err: {
                match layouts {
                    BindingLayoutSource::Provided(pipeline_layout) => {
                        // update the required binding size for this buffer
                        if let ResourceType::Buffer { size } = res.ty {
                            match shader_binding_sizes.entry(res.bind) {
                                Entry::Occupied(e) => {
                                    *e.into_mut() = size.max(*e.get());
                                }
                                Entry::Vacant(e) => {
                                    e.insert(size);
                                }
                            }
                        }

                        let Some(entry) =
                            pipeline_layout.get_bgl_entry(res.bind.group, res.bind.binding)
                        else {
                            break 'err Err(BindingError::Missing);
                        };

                        if !entry.visibility.contains(stage_bit) {
                            break 'err Err(BindingError::Invisible);
                        }

                        res.check_binding_use(entry)
                    }
                    BindingLayoutSource::Derived(layouts) => {
                        let Some(map) = layouts.get_mut(res.bind.group as usize) else {
                            break 'err Err(BindingError::Missing);
                        };

                        let ty = match res.derive_binding_type(
                            entry_point
                                .sampling_pairs
                                .iter()
                                .any(|&(im, _samp)| im == handle),
                        ) {
                            Ok(ty) => ty,
                            Err(error) => break 'err Err(error),
                        };

                        match map.entry(res.bind.binding) {
                            indexmap::map::Entry::Occupied(e) if e.get().ty != ty => {
                                break 'err Err(BindingError::InconsistentlyDerivedType)
                            }
                            indexmap::map::Entry::Occupied(e) => {
                                e.into_mut().visibility |= stage_bit;
                            }
                            indexmap::map::Entry::Vacant(e) => {
                                e.insert(BindGroupLayoutEntry {
                                    binding: res.bind.binding,
                                    ty,
                                    visibility: stage_bit,
                                    count: None,
                                });
                            }
                        }
                        Ok(())
                    }
                }
            };
            if let Err(error) = result {
                return Err(StageError::Binding(res.bind, error));
            }
        }

        // Check the compatibility between textures and samplers
        //
        // We only need to do this if the binding layout is provided by the user, as derived
        // layouts will inherently be correctly tagged.
        if let BindingLayoutSource::Provided(pipeline_layout) = layouts {
            for &(texture_handle, sampler_handle) in entry_point.sampling_pairs.iter() {
                let texture_bind = &self.resources[texture_handle].bind;
                let sampler_bind = &self.resources[sampler_handle].bind;
                let texture_layout = pipeline_layout
                    .get_bgl_entry(texture_bind.group, texture_bind.binding)
                    .unwrap();
                let sampler_layout = pipeline_layout
                    .get_bgl_entry(sampler_bind.group, sampler_bind.binding)
                    .unwrap();
                assert!(texture_layout.visibility.contains(stage_bit));
                assert!(sampler_layout.visibility.contains(stage_bit));

                let sampler_filtering = matches!(
                    sampler_layout.ty,
                    BindingType::Sampler(wgt::SamplerBindingType::Filtering)
                );
                let texture_sample_type = match texture_layout.ty {
                    BindingType::Texture { sample_type, .. } => sample_type,
                    BindingType::ExternalTexture => {
                        wgt::TextureSampleType::Float { filterable: true }
                    }
                    _ => unreachable!(),
                };

                let error = match (sampler_filtering, texture_sample_type) {
                    (true, wgt::TextureSampleType::Float { filterable: false }) => {
                        Some(FilteringError::Float)
                    }
                    (true, wgt::TextureSampleType::Sint) => Some(FilteringError::Integer),
                    (true, wgt::TextureSampleType::Uint) => Some(FilteringError::Integer),
                    _ => None,
                };

                if let Some(error) = error {
                    return Err(StageError::Filtering {
                        texture: *texture_bind,
                        sampler: *sampler_bind,
                        error,
                    });
                }
            }
        }

        // check workgroup size limits
        if shader_stage.to_naga().compute_like() {
            let (
                max_workgroup_size_limits,
                max_workgroup_size_total,
                per_dimension_limit,
                total_limit,
            ) = match shader_stage.to_naga() {
                naga::ShaderStage::Compute => (
                    [
                        self.limits.max_compute_workgroup_size_x,
                        self.limits.max_compute_workgroup_size_y,
                        self.limits.max_compute_workgroup_size_z,
                    ],
                    self.limits.max_compute_invocations_per_workgroup,
                    "max_compute_workgroup_size_*",
                    "max_compute_invocations_per_workgroup",
                ),
                naga::ShaderStage::Task => (
                    [
                        self.limits.max_task_invocations_per_dimension,
                        self.limits.max_task_invocations_per_dimension,
                        self.limits.max_task_invocations_per_dimension,
                    ],
                    self.limits.max_task_invocations_per_workgroup,
                    "max_task_invocations_per_dimension",
                    "max_task_invocations_per_workgroup",
                ),
                naga::ShaderStage::Mesh => (
                    [
                        self.limits.max_mesh_invocations_per_dimension,
                        self.limits.max_mesh_invocations_per_dimension,
                        self.limits.max_mesh_invocations_per_dimension,
                    ],
                    self.limits.max_mesh_invocations_per_workgroup,
                    "max_mesh_invocations_per_dimension",
                    "max_mesh_invocations_per_workgroup",
                ),
                _ => unreachable!(),
            };

            let total_invocations = entry_point
                .workgroup_size
                .iter()
                .fold(1u32, |total, &dim| total.saturating_mul(dim));
            let invalid_total_invocations =
                total_invocations > max_workgroup_size_total || total_invocations == 0;

            assert_eq!(
                entry_point.workgroup_size.len(),
                max_workgroup_size_limits.len()
            );
            let dimension_too_large = entry_point
                .workgroup_size
                .iter()
                .zip(max_workgroup_size_limits.iter())
                .any(|(dim, limit)| dim > limit);

            if invalid_total_invocations || dimension_too_large {
                return Err(StageError::InvalidWorkgroupSize {
                    dimensions: entry_point.workgroup_size,
                    total: total_invocations,
                    per_dimension_limits: max_workgroup_size_limits,
                    total_limit: max_workgroup_size_total,
                    per_dimension_limits_desc: per_dimension_limit,
                    total_limit_desc: total_limit,
                });
            }
        }

        let mut this_stage_primitive_index = false;
        let mut has_draw_id = false;

        // check inputs compatibility
        for input in entry_point.inputs.iter() {
            match *input {
                Varying::Local { location, ref iv } => {
                    let result = inputs
                        .varyings
                        .get(&location)
                        .ok_or(InputError::Missing)
                        .and_then(|provided| {
                            let (compatible, per_primitive_correct) = match shader_stage.to_naga() {
                                // For vertex attributes, there are defaults filled out
                                // by the driver if data is not provided.
                                naga::ShaderStage::Vertex => {
                                    let is_compatible =
                                        iv.ty.scalar.kind == provided.ty.scalar.kind;
                                    // vertex inputs don't count towards inter-stage
                                    (is_compatible, !iv.per_primitive)
                                }
                                naga::ShaderStage::Fragment => {
                                    if iv.interpolation != provided.interpolation {
                                        return Err(InputError::InterpolationMismatch(
                                            provided.interpolation,
                                        ));
                                    }
                                    if iv.sampling != provided.sampling {
                                        return Err(InputError::SamplingMismatch(
                                            provided.sampling,
                                        ));
                                    }
                                    (
                                        iv.ty.is_subtype_of(&provided.ty),
                                        iv.per_primitive == provided.per_primitive,
                                    )
                                }
                                // These can't have varying inputs
                                naga::ShaderStage::Compute
                                | naga::ShaderStage::Task
                                | naga::ShaderStage::Mesh => (false, false),
                                naga::ShaderStage::RayGeneration
                                | naga::ShaderStage::AnyHit
                                | naga::ShaderStage::ClosestHit
                                | naga::ShaderStage::Miss => {
                                    unreachable!()
                                }
                            };
                            if !compatible {
                                return Err(InputError::WrongType(provided.ty));
                            } else if !per_primitive_correct {
                                return Err(InputError::WrongPerPrimitive {
                                    pipeline_input: provided.per_primitive,
                                    shader: iv.per_primitive,
                                });
                            }
                            Ok(())
                        });

                    if let Err(error) = result {
                        return Err(StageError::Input {
                            location,
                            var: iv.clone(),
                            error,
                        });
                    }
                }
                Varying::BuiltIn(naga::BuiltIn::PrimitiveIndex) => {
                    this_stage_primitive_index = true;
                }
                Varying::BuiltIn(naga::BuiltIn::DrawIndex) => {
                    has_draw_id = true;
                }
                Varying::BuiltIn(_) => {}
            }
        }

        match shader_stage {
            ShaderStageForValidation::Vertex {
                topology,
                compare_function,
            } => {
                let mut max_vertex_shader_output_variables =
                    self.limits.max_inter_stage_shader_variables;
                let mut max_vertex_shader_output_location = max_vertex_shader_output_variables - 1;

                let point_list_deduction = if topology == wgt::PrimitiveTopology::PointList {
                    Some(MaxVertexShaderOutputDeduction::PointListPrimitiveTopology)
                } else {
                    None
                };

                let deductions = point_list_deduction.into_iter();

                for deduction in deductions.clone() {
                    // NOTE: Deductions, in the current version of the spec. we implement, do not
                    // ever exceed the minimum variables available.
                    max_vertex_shader_output_variables = max_vertex_shader_output_variables
                        .checked_sub(deduction.for_variables())
                        .unwrap();
                    max_vertex_shader_output_location = max_vertex_shader_output_location
                        .checked_sub(deduction.for_location())
                        .unwrap();
                }

                let mut num_user_defined_outputs = 0;

                for output in entry_point.outputs.iter() {
                    match *output {
                        Varying::Local { ref iv, location } => {
                            if location > max_vertex_shader_output_location {
                                return Err(StageError::VertexOutputLocationTooLarge {
                                    location,
                                    var: iv.clone(),
                                    limit: self.limits.max_inter_stage_shader_variables,
                                    deductions: deductions.collect(),
                                });
                            }
                            num_user_defined_outputs += 1;
                        }
                        Varying::BuiltIn(_) => {}
                    };

                    if let Some(
                        cmp @ wgt::CompareFunction::Equal | cmp @ wgt::CompareFunction::NotEqual,
                    ) = compare_function
                    {
                        if let Varying::BuiltIn(naga::BuiltIn::Position { invariant: false }) =
                            *output
                        {
                            log::warn!(
                                concat!(
                                    "Vertex shader with entry point {} outputs a ",
                                    "@builtin(position) without the @invariant attribute and ",
                                    "is used in a pipeline with {cmp:?}. On some machines, ",
                                    "this can cause bad artifacting as {cmp:?} assumes the ",
                                    "values output from the vertex shader exactly match the ",
                                    "value in the depth buffer. The @invariant attribute on the ",
                                    "@builtin(position) vertex output ensures that the exact ",
                                    "same pixel depths are used every render."
                                ),
                                entry_point_name,
                                cmp = cmp
                            );
                        }
                    }
                }

                if num_user_defined_outputs > max_vertex_shader_output_variables {
                    return Err(StageError::TooManyUserDefinedVertexOutputs {
                        num_found: num_user_defined_outputs,
                        limit: self.limits.max_inter_stage_shader_variables,
                        deductions: deductions.collect(),
                    });
                }
            }
            ShaderStageForValidation::Fragment {
                dual_source_blending,
                has_depth_attachment,
            } => {
                let mut max_fragment_shader_input_variables =
                    self.limits.max_inter_stage_shader_variables;

                let deductions = entry_point.inputs.iter().filter_map(|output| match output {
                    Varying::Local { .. } => None,
                    Varying::BuiltIn(builtin) => {
                        MaxFragmentShaderInputDeduction::from_inter_stage_builtin(*builtin).or_else(
                            || {
                                unreachable!(
                                    concat!(
                                        "unexpected built-in provided; ",
                                        "{:?} is not used for fragment stage input",
                                    ),
                                    builtin
                                )
                            },
                        )
                    }
                });

                for deduction in deductions.clone() {
                    // NOTE: Deductions, in the current version of the spec. we implement, do not
                    // ever exceed the minimum variables available.
                    max_fragment_shader_input_variables = max_fragment_shader_input_variables
                        .checked_sub(deduction.for_variables())
                        .unwrap();
                }

                let mut num_user_defined_inputs = 0;

                for output in entry_point.inputs.iter() {
                    match *output {
                        Varying::Local { ref iv, location } => {
                            if location >= self.limits.max_inter_stage_shader_variables {
                                return Err(StageError::FragmentInputLocationTooLarge {
                                    location,
                                    var: iv.clone(),
                                    limit: self.limits.max_inter_stage_shader_variables,
                                    deductions: deductions.collect(),
                                });
                            }
                            num_user_defined_inputs += 1;
                        }
                        Varying::BuiltIn(_) => {}
                    };
                }

                if num_user_defined_inputs > max_fragment_shader_input_variables {
                    return Err(StageError::TooManyUserDefinedFragmentInputs {
                        num_found: num_user_defined_inputs,
                        limit: self.limits.max_inter_stage_shader_variables,
                        deductions: deductions.collect(),
                    });
                }

                for output in &entry_point.outputs {
                    let &Varying::Local { location, ref iv } = output else {
                        continue;
                    };
                    if location >= self.limits.max_color_attachments {
                        return Err(StageError::ColorAttachmentLocationTooLarge {
                            location,
                            var: iv.clone(),
                            limit: self.limits.max_color_attachments,
                        });
                    }
                }

                // If the pipeline uses dual-source blending, then the shader
                // must configure appropriate I/O, but it is not an error to
                // use a shader that defines the I/O in a pipeline that only
                // uses one blend source.
                if dual_source_blending && !entry_point.dual_source_blending {
                    return Err(StageError::InvalidDualSourceBlending);
                }

                if entry_point
                    .outputs
                    .contains(&Varying::BuiltIn(naga::BuiltIn::FragDepth))
                    && !has_depth_attachment
                {
                    return Err(StageError::MissingFragDepthAttachment);
                }
            }
            ShaderStageForValidation::Mesh => {
                for output in &entry_point.outputs {
                    if matches!(output, Varying::BuiltIn(naga::BuiltIn::PrimitiveIndex)) {
                        this_stage_primitive_index = true;
                    }
                }
            }
            _ => (),
        }

        if let Some(ref mesh_info) = entry_point.mesh_info {
            if mesh_info.max_vertices > self.limits.max_mesh_output_vertices {
                return Err(StageError::TooManyMeshVertices {
                    limit: self.limits.max_mesh_output_vertices,
                    value: mesh_info.max_vertices,
                });
            }
            if mesh_info.max_primitives > self.limits.max_mesh_output_primitives {
                return Err(StageError::TooManyMeshPrimitives {
                    limit: self.limits.max_mesh_output_primitives,
                    value: mesh_info.max_primitives,
                });
            }
        }
        if let Some(task_payload_size) = entry_point.task_payload_size {
            if task_payload_size > self.limits.max_task_payload_size {
                return Err(StageError::TaskPayloadTooLarge {
                    limit: self.limits.max_task_payload_size,
                    value: task_payload_size,
                });
            }
        }
        if shader_stage.to_naga() == naga::ShaderStage::Mesh
            && entry_point.task_payload_size != inputs.task_payload_size
        {
            return Err(StageError::TaskPayloadMustMatch {
                input: inputs.task_payload_size,
                shader: entry_point.task_payload_size,
            });
        }

        // Fragment shader primitive index is treated like a varying
        if shader_stage.to_naga() == naga::ShaderStage::Fragment
            && this_stage_primitive_index
            && inputs.primitive_index == Some(false)
        {
            return Err(StageError::InvalidPrimitiveIndex);
        } else if shader_stage.to_naga() == naga::ShaderStage::Fragment
            && !this_stage_primitive_index
            && inputs.primitive_index == Some(true)
        {
            return Err(StageError::MissingPrimitiveIndex);
        }
        if shader_stage.to_naga() == naga::ShaderStage::Mesh
            && inputs.task_payload_size.is_some()
            && has_draw_id
        {
            return Err(StageError::DrawIdError);
        }

        let outputs = entry_point
            .outputs
            .iter()
            .filter_map(|output| match *output {
                Varying::Local { location, ref iv } => Some((location, iv.clone())),
                Varying::BuiltIn(_) => None,
            })
            .collect();

        Ok(StageIo {
            task_payload_size: entry_point.task_payload_size,
            varyings: outputs,
            primitive_index: if shader_stage.to_naga() == naga::ShaderStage::Mesh {
                Some(this_stage_primitive_index)
            } else {
                None
            },
        })
    }

    pub fn fragment_uses_dual_source_blending(
        &self,
        entry_point_name: &str,
    ) -> Result<bool, StageError> {
        let pair = (naga::ShaderStage::Fragment, entry_point_name.to_string());
        self.entry_points
            .get(&pair)
            .ok_or(StageError::MissingEntryPoint(pair.1))
            .map(|ep| ep.dual_source_blending)
    }
}

/// Validate a list of color attachment formats against `maxColorAttachmentBytesPerSample`.
///
/// The color attachments can be from a render pass descriptor or a pipeline descriptor.
///
/// Implements <https://gpuweb.github.io/gpuweb/#abstract-opdef-calculating-color-attachment-bytes-per-sample>.
pub fn validate_color_attachment_bytes_per_sample(
    attachment_formats: impl IntoIterator<Item = wgt::TextureFormat>,
    limit: u32,
) -> Result<(), crate::command::ColorAttachmentError> {
    let mut total_bytes_per_sample: u32 = 0;
    for format in attachment_formats {
        let byte_cost = format.target_pixel_byte_cost().unwrap();
        let alignment = format.target_component_alignment().unwrap();

        total_bytes_per_sample = total_bytes_per_sample.next_multiple_of(alignment);
        total_bytes_per_sample += byte_cost;
    }

    if total_bytes_per_sample > limit {
        return Err(
            crate::command::ColorAttachmentError::TooManyBytesPerSample {
                total: total_bytes_per_sample,
                limit,
            },
        );
    }

    Ok(())
}

pub enum ShaderStageForValidation {
    Vertex {
        topology: wgt::PrimitiveTopology,
        compare_function: Option<wgt::CompareFunction>,
    },
    Mesh,
    Fragment {
        dual_source_blending: bool,
        has_depth_attachment: bool,
    },
    Compute,
    Task,
}

impl ShaderStageForValidation {
    pub fn to_naga(&self) -> naga::ShaderStage {
        match self {
            Self::Vertex { .. } => naga::ShaderStage::Vertex,
            Self::Mesh => naga::ShaderStage::Mesh,
            Self::Fragment { .. } => naga::ShaderStage::Fragment,
            Self::Compute => naga::ShaderStage::Compute,
            Self::Task => naga::ShaderStage::Task,
        }
    }

    pub fn to_wgt_bit(&self) -> wgt::ShaderStages {
        match self {
            Self::Vertex { .. } => wgt::ShaderStages::VERTEX,
            Self::Mesh => wgt::ShaderStages::MESH,
            Self::Fragment { .. } => wgt::ShaderStages::FRAGMENT,
            Self::Compute => wgt::ShaderStages::COMPUTE,
            Self::Task => wgt::ShaderStages::TASK,
        }
    }
}
