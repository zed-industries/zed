use alloc::string::String;

use super::Capabilities;
use crate::{arena::Handle, ir, proc::Alignment};

bitflags::bitflags! {
    /// Flags associated with [`Type`]s by [`Validator`].
    ///
    /// [`Type`]: crate::Type
    /// [`Validator`]: crate::valid::Validator
    #[cfg_attr(feature = "serialize", derive(serde::Serialize))]
    #[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct TypeFlags: u8 {
        /// Can be used for data variables.
        ///
        /// This flag is required on types of local variables, function
        /// arguments, array elements, and struct members.
        ///
        /// This includes all types except [`Image`], [`Sampler`],
        /// and some [`Pointer`] types.
        ///
        /// [`Image`]: crate::TypeInner::Image
        /// [`Sampler`]: crate::TypeInner::Sampler
        /// [`Pointer`]: crate::TypeInner::Pointer
        const DATA = 0x1;

        /// The data type has a size known by pipeline creation time.
        ///
        /// Unsized types are quite restricted. The only unsized types permitted
        /// by Naga, other than the non-[`DATA`] types like [`Image`] and
        /// [`Sampler`], are dynamically-sized [`Array`]s, and [`Struct`]s whose
        /// last members are such arrays. See the documentation for those types
        /// for details.
        ///
        /// [`DATA`]: TypeFlags::DATA
        /// [`Image`]: crate::TypeInner::Image
        /// [`Sampler`]: crate::TypeInner::Sampler
        /// [`Array`]: crate::TypeInner::Array
        /// [`Struct`]: crate::TypeInner::Struct
        const SIZED = 0x2;

        /// The data can be copied around.
        const COPY = 0x4;

        /// Can be be used in pipeline stage I/O.
        ///
        /// Applies to the following:
        ///   - Types that may be used in a [`Location`] binding (numeric scalars and vectors)
        ///   - `@blend_src` structs
        ///
        /// See [location-attr] and [input-output].
        ///
        /// [`Location`]: crate::Binding::Location
        /// [location-attr]: https://gpuweb.github.io/gpuweb/wgsl/#location-attr
        /// [input-output]: https://gpuweb.github.io/gpuweb/wgsl/#input-output-locations
        /// https://gpuweb.github.io/gpuweb/wgsl/#location-attr
        const IO_SHAREABLE = 0x8;

        /// Can be used for host-shareable structures.
        const HOST_SHAREABLE = 0x10;

        /// The set of types with a fixed size at shader-creation time (ie. everything
        /// except arrays sized by an override-expression)
        const CREATION_RESOLVED = 0x20;

        /// This type can be passed as a function argument.
        const ARGUMENT = 0x40;

        /// A WGSL [constructible] type.
        ///
        /// The constructible types are scalars, vectors, matrices, fixed-size
        /// arrays of constructible types, and structs whose members are all
        /// constructible.
        ///
        /// [constructible]: https://gpuweb.github.io/gpuweb/wgsl/#constructible
        const CONSTRUCTIBLE = 0x80;
    }
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Disalignment {
    #[error("The array stride {stride} is not a multiple of the required alignment {alignment}")]
    ArrayStride { stride: u32, alignment: Alignment },
    #[error("The struct span {span}, is not a multiple of the required alignment {alignment}")]
    StructSpan { span: u32, alignment: Alignment },
    #[error("The struct member[{index}] offset {offset} is not a multiple of the required alignment {alignment}")]
    MemberOffset {
        index: u32,
        offset: u32,
        alignment: Alignment,
    },
    #[error("The struct member[{index}] offset {offset} must be at least {expected}")]
    MemberOffsetAfterStruct {
        index: u32,
        offset: u32,
        expected: u32,
    },
    #[error("The struct member[{index}] is not statically sized")]
    UnsizedMember { index: u32 },
    #[error("The type is not host-shareable")]
    NonHostShareable,
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TypeError {
    #[error("Capability {0:?} is required")]
    MissingCapability(Capabilities),
    #[error("The {0:?} scalar width {1} is not supported for an atomic")]
    InvalidAtomicWidth(crate::ScalarKind, crate::Bytes),
    #[error("Invalid type for pointer target {0:?}")]
    InvalidPointerBase(Handle<crate::Type>),
    #[error("Unsized types like {base:?} must be in the `Storage` address space, not `{space:?}`")]
    InvalidPointerToUnsized {
        base: Handle<crate::Type>,
        space: crate::AddressSpace,
    },
    #[error("Expected data type, found {0:?}")]
    InvalidData(Handle<crate::Type>),
    #[error("Base type {0:?} for the array is invalid")]
    InvalidArrayBaseType(Handle<crate::Type>),
    #[error("Matrix elements must always be floating-point types")]
    MatrixElementNotFloat,
    #[error("The constant {0:?} is specialized, and cannot be used as an array size")]
    UnsupportedSpecializedArrayLength(Handle<crate::Constant>),
    #[error("{} of dimensionality {dim:?} and class {class:?} are not supported", if *.arrayed {"Arrayed images"} else {"Images"})]
    UnsupportedImageType {
        dim: crate::ImageDimension,
        arrayed: bool,
        class: crate::ImageClass,
    },
    #[error("Array stride {stride} does not match the expected {expected}")]
    InvalidArrayStride { stride: u32, expected: u32 },
    #[error("Field '{0}' can't be dynamically-sized, has type {1:?}")]
    InvalidDynamicArray(String, Handle<crate::Type>),
    #[error("The base handle {0:?} has to be a struct")]
    BindingArrayBaseTypeNotStruct(Handle<crate::Type>),
    #[error("Binding arrays of external textures are not yet supported")]
    BindingArrayBaseExternalTextures,
    #[error("Structure member[{index}] at {offset} overlaps the previous member")]
    MemberOverlap { index: u32, offset: u32 },
    #[error(
        "Structure member[{index}] at {offset} and size {size} crosses the structure boundary of size {span}"
    )]
    MemberOutOfBounds {
        index: u32,
        offset: u32,
        size: u32,
        span: u32,
    },
    #[error("Structure types must have at least one member")]
    EmptyStruct,
    #[error("Invalid `@blend_src` structure: {0}")]
    InvalidBlendSrc(super::VaryingError),
    #[error(transparent)]
    WidthError(#[from] WidthError),
    #[error(
        "The base handle {0:?} has an override-expression that didn't get resolved to a constant"
    )]
    UnresolvedOverride(Handle<crate::Type>),
    #[error("Override-sized array type {0:?} does not have a positive size")]
    InvalidArraySize(Handle<crate::Type>),
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum WidthError {
    #[error("The {0:?} scalar width {1} is not supported")]
    Invalid(crate::ScalarKind, crate::Bytes),
    #[error("Using `{name}` values requires the `naga::valid::Capabilities::{flag}` flag")]
    MissingCapability {
        name: &'static str,
        flag: &'static str,
    },

    #[error("Abstract types may only appear in constant expressions")]
    Abstract,
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ImmediateError {
    #[error("The scalar type {0:?} is not supported in immediates")]
    InvalidScalar(crate::Scalar),
}

// Only makes sense if `flags.contains(HOST_SHAREABLE)`
type LayoutCompatibility = Result<Alignment, (Handle<crate::Type>, Disalignment)>;
type ImmediateCompatibility = Result<(), ImmediateError>;

fn check_member_layout(
    accum: &mut LayoutCompatibility,
    member: &crate::StructMember,
    member_index: u32,
    member_layout: LayoutCompatibility,
    parent_handle: Handle<crate::Type>,
) {
    *accum = match (*accum, member_layout) {
        (Ok(cur_alignment), Ok(alignment)) => {
            if alignment.is_aligned(member.offset) {
                Ok(cur_alignment.max(alignment))
            } else {
                Err((
                    parent_handle,
                    Disalignment::MemberOffset {
                        index: member_index,
                        offset: member.offset,
                        alignment,
                    },
                ))
            }
        }
        (Err(e), _) | (_, Err(e)) => Err(e),
    };
}

/// Determine whether a pointer in `space` can be passed as an argument.
///
/// If a pointer in `space` is permitted to be passed as an argument to a
/// user-defined function, return `TypeFlags::ARGUMENT`. Otherwise, return
/// `TypeFlags::empty()`.
///
/// Pointers passed as arguments to user-defined functions must be in the
/// `Function` or `Private` address space.
const fn ptr_space_argument_flag(space: crate::AddressSpace) -> TypeFlags {
    use crate::AddressSpace as As;
    match space {
        As::Function | As::Private | As::RayPayload | As::IncomingRayPayload => TypeFlags::ARGUMENT,
        As::Uniform
        | As::Storage { .. }
        | As::Handle
        | As::Immediate
        | As::WorkGroup
        | As::TaskPayload => TypeFlags::empty(),
    }
}

#[derive(Clone, Debug)]
pub(super) struct TypeInfo {
    pub flags: TypeFlags,
    pub uniform_layout: LayoutCompatibility,
    pub storage_layout: LayoutCompatibility,
    pub immediates_compatibility: ImmediateCompatibility,
}

impl TypeInfo {
    const fn dummy() -> Self {
        TypeInfo {
            flags: TypeFlags::empty(),
            uniform_layout: Ok(Alignment::ONE),
            storage_layout: Ok(Alignment::ONE),
            immediates_compatibility: Ok(()),
        }
    }

    const fn new(flags: TypeFlags, alignment: Alignment) -> Self {
        TypeInfo {
            flags,
            uniform_layout: Ok(alignment),
            storage_layout: Ok(alignment),
            immediates_compatibility: Ok(()),
        }
    }
}

impl super::Validator {
    const fn require_type_capability(&self, capability: Capabilities) -> Result<(), TypeError> {
        if self.capabilities.contains(capability) {
            Ok(())
        } else {
            Err(TypeError::MissingCapability(capability))
        }
    }

    /// Check whether `scalar` is a permitted scalar width.
    ///
    /// If `scalar` is not a width allowed by the selected [`Capabilities`],
    /// return an error explaining why.
    ///
    /// If `scalar` is allowed, return a [`ImmediateCompatibility`] result
    /// that says whether `scalar` is allowed specifically in immediates.
    ///
    /// [`Capabilities`]: crate::valid::Capabilities
    pub(super) const fn check_width(
        &self,
        scalar: crate::Scalar,
    ) -> Result<ImmediateCompatibility, WidthError> {
        let mut immediates_compatibility = Ok(());
        let good = match scalar.kind {
            crate::ScalarKind::Bool => scalar.width == crate::BOOL_WIDTH,
            crate::ScalarKind::Float => match scalar.width {
                8 => {
                    if !self.capabilities.contains(Capabilities::FLOAT64) {
                        return Err(WidthError::MissingCapability {
                            name: "f64",
                            flag: "FLOAT64",
                        });
                    }
                    true
                }
                2 => {
                    if !self.capabilities.contains(Capabilities::SHADER_FLOAT16) {
                        return Err(WidthError::MissingCapability {
                            name: "f16",
                            flag: "FLOAT16",
                        });
                    }

                    immediates_compatibility = Err(ImmediateError::InvalidScalar(scalar));

                    true
                }
                _ => scalar.width == 4,
            },
            crate::ScalarKind::Sint => {
                if scalar.width == 8 {
                    if !self.capabilities.contains(Capabilities::SHADER_INT64) {
                        return Err(WidthError::MissingCapability {
                            name: "i64",
                            flag: "SHADER_INT64",
                        });
                    }
                    true
                } else {
                    scalar.width == 4
                }
            }
            crate::ScalarKind::Uint => {
                if scalar.width == 8 {
                    if !self.capabilities.contains(Capabilities::SHADER_INT64) {
                        return Err(WidthError::MissingCapability {
                            name: "u64",
                            flag: "SHADER_INT64",
                        });
                    }
                    true
                } else {
                    scalar.width == 4
                }
            }
            crate::ScalarKind::AbstractInt | crate::ScalarKind::AbstractFloat => {
                return Err(WidthError::Abstract);
            }
        };
        if good {
            Ok(immediates_compatibility)
        } else {
            Err(WidthError::Invalid(scalar.kind, scalar.width))
        }
    }

    pub(super) fn reset_types(&mut self, size: usize) {
        self.types.clear();
        self.types.resize(size, TypeInfo::dummy());
        self.layouter.clear();
    }

    pub(super) fn validate_type(
        &self,
        handle: Handle<crate::Type>,
        gctx: crate::proc::GlobalCtx,
    ) -> Result<TypeInfo, TypeError> {
        use crate::TypeInner as Ti;
        Ok(match gctx.types[handle].inner {
            Ti::Scalar(scalar) => {
                let immediates_compatibility = self.check_width(scalar)?;
                let shareable = if scalar.kind.is_numeric() {
                    TypeFlags::IO_SHAREABLE | TypeFlags::HOST_SHAREABLE
                } else {
                    TypeFlags::empty()
                };
                let mut type_info = TypeInfo::new(
                    TypeFlags::DATA
                        | TypeFlags::SIZED
                        | TypeFlags::COPY
                        | TypeFlags::ARGUMENT
                        | TypeFlags::CONSTRUCTIBLE
                        | TypeFlags::CREATION_RESOLVED
                        | shareable,
                    Alignment::from_width(scalar.width),
                );
                type_info.immediates_compatibility = immediates_compatibility;
                type_info
            }
            Ti::Vector { size, scalar } => {
                let immediates_compatibility = self.check_width(scalar)?;
                let shareable = if scalar.kind.is_numeric() {
                    TypeFlags::IO_SHAREABLE | TypeFlags::HOST_SHAREABLE
                } else {
                    TypeFlags::empty()
                };
                let mut type_info = TypeInfo::new(
                    TypeFlags::DATA
                        | TypeFlags::SIZED
                        | TypeFlags::COPY
                        | TypeFlags::ARGUMENT
                        | TypeFlags::CONSTRUCTIBLE
                        | TypeFlags::CREATION_RESOLVED
                        | shareable,
                    Alignment::from(size) * Alignment::from_width(scalar.width),
                );
                type_info.immediates_compatibility = immediates_compatibility;
                type_info
            }
            Ti::Matrix {
                columns: _,
                rows,
                scalar,
            } => {
                if scalar.kind != crate::ScalarKind::Float {
                    return Err(TypeError::MatrixElementNotFloat);
                }
                let immediates_compatibility = self.check_width(scalar)?;
                let mut type_info = TypeInfo::new(
                    TypeFlags::DATA
                        | TypeFlags::SIZED
                        | TypeFlags::COPY
                        | TypeFlags::HOST_SHAREABLE
                        | TypeFlags::ARGUMENT
                        | TypeFlags::CONSTRUCTIBLE
                        | TypeFlags::CREATION_RESOLVED,
                    Alignment::from(rows) * Alignment::from_width(scalar.width),
                );
                type_info.immediates_compatibility = immediates_compatibility;
                type_info
            }
            Ti::CooperativeMatrix {
                columns: _,
                rows: _,
                scalar,
                role: _,
            } => {
                self.require_type_capability(Capabilities::COOPERATIVE_MATRIX)?;
                // Allow f16 (width 2) and f32 (width 4) for cooperative matrices
                if scalar.kind != crate::ScalarKind::Float
                    || (scalar.width != 2 && scalar.width != 4)
                {
                    return Err(TypeError::MatrixElementNotFloat);
                }
                TypeInfo::new(
                    TypeFlags::DATA
                        | TypeFlags::SIZED
                        | TypeFlags::COPY
                        | TypeFlags::HOST_SHAREABLE
                        | TypeFlags::ARGUMENT
                        | TypeFlags::CONSTRUCTIBLE
                        | TypeFlags::CREATION_RESOLVED,
                    Alignment::from_width(scalar.width),
                )
            }
            Ti::Atomic(scalar) => {
                match scalar {
                    crate::Scalar {
                        kind: crate::ScalarKind::Sint | crate::ScalarKind::Uint,
                        width: _,
                    } => {
                        if scalar.width == 8
                            && !self.capabilities.intersects(
                                Capabilities::SHADER_INT64_ATOMIC_ALL_OPS
                                    | Capabilities::SHADER_INT64_ATOMIC_MIN_MAX,
                            )
                        {
                            return Err(TypeError::MissingCapability(
                                Capabilities::SHADER_INT64_ATOMIC_ALL_OPS,
                            ));
                        }
                    }
                    crate::Scalar::F32 => {
                        if !self
                            .capabilities
                            .contains(Capabilities::SHADER_FLOAT32_ATOMIC)
                        {
                            return Err(TypeError::MissingCapability(
                                Capabilities::SHADER_FLOAT32_ATOMIC,
                            ));
                        }
                    }
                    _ => return Err(TypeError::InvalidAtomicWidth(scalar.kind, scalar.width)),
                };
                TypeInfo::new(
                    TypeFlags::DATA
                        | TypeFlags::SIZED
                        | TypeFlags::HOST_SHAREABLE
                        | TypeFlags::CREATION_RESOLVED,
                    Alignment::from_width(scalar.width),
                )
            }
            Ti::Pointer { base, space } => {
                use crate::AddressSpace as As;

                let base_info = &self.types[base.index()];
                if !base_info.flags.contains(TypeFlags::DATA) {
                    return Err(TypeError::InvalidPointerBase(base));
                }

                // Runtime-sized values can only live in the `Storage` address
                // space, so it's useless to have a pointer to such a type in
                // any other space.
                //
                // Detecting this problem here prevents the definition of
                // functions like:
                //
                //     fn f(p: ptr<workgroup, UnsizedType>) -> ... { ... }
                //
                // which would otherwise be permitted, but uncallable. (They
                // may also present difficulties in code generation).
                if !base_info.flags.contains(TypeFlags::SIZED) {
                    match space {
                        As::Storage { .. } => {}
                        _ => {
                            return Err(TypeError::InvalidPointerToUnsized { base, space });
                        }
                    }
                }

                // `Validator::validate_function` actually checks the address
                // space of pointer arguments explicitly before checking the
                // `ARGUMENT` flag, to give better error messages. But it seems
                // best to set `ARGUMENT` accurately anyway.
                let argument_flag = ptr_space_argument_flag(space);

                // Pointers cannot be stored in variables, structure members, or
                // array elements, so we do not mark them as `DATA`.
                TypeInfo::new(
                    argument_flag
                        | TypeFlags::SIZED
                        | TypeFlags::COPY
                        | TypeFlags::CREATION_RESOLVED,
                    Alignment::ONE,
                )
            }
            Ti::ValuePointer {
                size: _,
                scalar,
                space,
            } => {
                // ValuePointer should be treated the same way as the equivalent
                // Pointer / Scalar / Vector combination, so each step in those
                // variants' match arms should have a counterpart here.
                //
                // However, some cases are trivial: All our implicit base types
                // are DATA and SIZED, so we can never return
                // `InvalidPointerBase` or `InvalidPointerToUnsized`.
                let _ = self.check_width(scalar)?;

                // `Validator::validate_function` actually checks the address
                // space of pointer arguments explicitly before checking the
                // `ARGUMENT` flag, to give better error messages. But it seems
                // best to set `ARGUMENT` accurately anyway.
                let argument_flag = ptr_space_argument_flag(space);

                // Pointers cannot be stored in variables, structure members, or
                // array elements, so we do not mark them as `DATA`.
                TypeInfo::new(
                    argument_flag
                        | TypeFlags::SIZED
                        | TypeFlags::COPY
                        | TypeFlags::CREATION_RESOLVED,
                    Alignment::ONE,
                )
            }
            Ti::Array { base, size, stride } => {
                let base_info = &self.types[base.index()];
                if !base_info
                    .flags
                    .contains(TypeFlags::DATA | TypeFlags::SIZED | TypeFlags::CREATION_RESOLVED)
                {
                    return Err(TypeError::InvalidArrayBaseType(base));
                }

                if self.overrides_resolved {
                    // This check only makes sense for override-sized arrays.
                    // `ArraySize::Constant` holds a `NonZeroU32`.
                    if let crate::ArraySize::Pending(_) = size {
                        size.resolve(gctx)
                            .map_err(|_| TypeError::InvalidArraySize(handle))?;
                    }
                }

                let base_layout = self.layouter[base];
                let general_alignment = base_layout.alignment;
                let uniform_layout = match base_info.uniform_layout {
                    Ok(base_alignment) => {
                        let alignment = base_alignment
                            .max(general_alignment)
                            .max(Alignment::MIN_UNIFORM);
                        if alignment.is_aligned(stride) {
                            Ok(alignment)
                        } else {
                            Err((handle, Disalignment::ArrayStride { stride, alignment }))
                        }
                    }
                    Err(e) => Err(e),
                };
                let storage_layout = match base_info.storage_layout {
                    Ok(base_alignment) => {
                        let alignment = base_alignment.max(general_alignment);
                        if alignment.is_aligned(stride) {
                            Ok(alignment)
                        } else {
                            Err((handle, Disalignment::ArrayStride { stride, alignment }))
                        }
                    }
                    Err(e) => Err(e),
                };

                let type_info_mask = match size {
                    crate::ArraySize::Constant(_) => {
                        TypeFlags::DATA
                            | TypeFlags::SIZED
                            | TypeFlags::COPY
                            | TypeFlags::HOST_SHAREABLE
                            | TypeFlags::ARGUMENT
                            | TypeFlags::CONSTRUCTIBLE
                            | TypeFlags::CREATION_RESOLVED
                    }
                    crate::ArraySize::Pending(_) => {
                        TypeFlags::DATA
                            | TypeFlags::SIZED
                            | TypeFlags::COPY
                            | TypeFlags::HOST_SHAREABLE
                            | TypeFlags::ARGUMENT
                    }
                    crate::ArraySize::Dynamic => {
                        // Non-SIZED types may only appear as the last element of a structure.
                        // This is enforced by checks for SIZED-ness for all compound types,
                        // and a special case for structs.
                        TypeFlags::DATA
                            | TypeFlags::COPY
                            | TypeFlags::HOST_SHAREABLE
                            | TypeFlags::CREATION_RESOLVED
                    }
                };

                TypeInfo {
                    flags: base_info.flags & type_info_mask,
                    uniform_layout,
                    storage_layout,
                    immediates_compatibility: base_info.immediates_compatibility.clone(),
                }
            }
            Ti::Struct { ref members, span } => {
                if members.is_empty() {
                    return Err(TypeError::EmptyStruct);
                }

                let mut blend_src_types = [None, None];
                let mut non_blend_src_location = None;

                let mut ti = TypeInfo::new(
                    TypeFlags::DATA
                        | TypeFlags::SIZED
                        | TypeFlags::COPY
                        | TypeFlags::HOST_SHAREABLE
                        | TypeFlags::ARGUMENT
                        | TypeFlags::CONSTRUCTIBLE
                        | TypeFlags::CREATION_RESOLVED,
                    Alignment::ONE,
                );
                ti.uniform_layout = Ok(Alignment::MIN_UNIFORM);

                let mut min_offset = 0;
                let mut prev_struct_data: Option<(u32, u32)> = None;

                for (i, member) in members.iter().enumerate() {
                    let base_info = &self.types[member.ty.index()];
                    if !base_info
                        .flags
                        .contains(TypeFlags::DATA | TypeFlags::CREATION_RESOLVED)
                    {
                        return Err(TypeError::InvalidData(member.ty));
                    }
                    if !base_info.flags.contains(TypeFlags::HOST_SHAREABLE) {
                        if ti.uniform_layout.is_ok() {
                            ti.uniform_layout = Err((member.ty, Disalignment::NonHostShareable));
                        }
                        if ti.storage_layout.is_ok() {
                            ti.storage_layout = Err((member.ty, Disalignment::NonHostShareable));
                        }
                    }
                    ti.flags &= base_info.flags;

                    match member.binding {
                        Some(ir::Binding::Location {
                            location,
                            blend_src: Some(blend_src),
                            ..
                        }) => {
                            // `blend_src` is only valid if dual source blending was explicitly enabled,
                            // see https://www.w3.org/TR/WGSL/#extension-dual_source_blending
                            if !self
                                .capabilities
                                .contains(Capabilities::DUAL_SOURCE_BLENDING)
                            {
                                return Err(TypeError::MissingCapability(
                                    Capabilities::DUAL_SOURCE_BLENDING,
                                ));
                            }
                            if !(location == 0 && (blend_src == 0 || blend_src == 1)) {
                                return Err(TypeError::InvalidBlendSrc(
                                    super::VaryingError::InvalidBlendSrcIndex {
                                        location,
                                        blend_src,
                                    },
                                ));
                            }
                            if blend_src_types[blend_src as usize]
                                .replace(member.ty)
                                .is_some()
                            {
                                // @blend_src(i) appeared multiple times
                                return Err(TypeError::InvalidBlendSrc(
                                    super::VaryingError::BindingCollisionBlendSrc { blend_src },
                                ));
                            }
                        }
                        Some(ir::Binding::Location {
                            location,
                            blend_src: None,
                            ..
                        }) => non_blend_src_location = Some(location),
                        _ => {}
                    }

                    if member.offset < min_offset {
                        // HACK: this could be nicer. We want to allow some structures
                        // to not bother with offsets/alignments if they are never
                        // used for host sharing.
                        if member.offset == 0 {
                            ti.flags.set(TypeFlags::HOST_SHAREABLE, false);
                        } else {
                            return Err(TypeError::MemberOverlap {
                                index: i as u32,
                                offset: member.offset,
                            });
                        }
                    }

                    let base_size = gctx.types[member.ty].inner.size(gctx);
                    min_offset = member.offset + base_size;
                    if min_offset > span {
                        return Err(TypeError::MemberOutOfBounds {
                            index: i as u32,
                            offset: member.offset,
                            size: base_size,
                            span,
                        });
                    }

                    check_member_layout(
                        &mut ti.uniform_layout,
                        member,
                        i as u32,
                        base_info.uniform_layout,
                        handle,
                    );
                    check_member_layout(
                        &mut ti.storage_layout,
                        member,
                        i as u32,
                        base_info.storage_layout,
                        handle,
                    );
                    if base_info.immediates_compatibility.is_err() {
                        ti.immediates_compatibility = base_info.immediates_compatibility.clone();
                    }

                    // Validate rule: If a structure member itself has a structure type S,
                    // then the number of bytes between the start of that member and
                    // the start of any following member must be at least roundUp(16, SizeOf(S)).
                    if let Some((span, offset)) = prev_struct_data {
                        let diff = member.offset - offset;
                        let min = Alignment::MIN_UNIFORM.round_up(span);
                        if diff < min {
                            ti.uniform_layout = Err((
                                handle,
                                Disalignment::MemberOffsetAfterStruct {
                                    index: i as u32,
                                    offset: member.offset,
                                    expected: offset + min,
                                },
                            ));
                        }
                    };

                    prev_struct_data = match gctx.types[member.ty].inner {
                        crate::TypeInner::Struct { span, .. } => Some((span, member.offset)),
                        _ => None,
                    };

                    // The last field may be an unsized array.
                    if !base_info.flags.contains(TypeFlags::SIZED) {
                        let is_array = match gctx.types[member.ty].inner {
                            crate::TypeInner::Array { .. } => true,
                            _ => false,
                        };
                        if !is_array || i + 1 != members.len() {
                            let name = member.name.clone().unwrap_or_default();
                            return Err(TypeError::InvalidDynamicArray(name, member.ty));
                        }
                        if ti.uniform_layout.is_ok() {
                            ti.uniform_layout =
                                Err((handle, Disalignment::UnsizedMember { index: i as u32 }));
                        }
                    }
                }

                match blend_src_types {
                    [None, None] => {}
                    [Some(ty0), Some(ty1)] => {
                        if let Some(location) = non_blend_src_location {
                            // If `@blend_src` members are present, then `@location`
                            // may only be used for those members.
                            return Err(TypeError::InvalidBlendSrc(
                                super::VaryingError::InvalidBlendSrcWithOtherBindings { location },
                            ));
                        }
                        let ty0_inner = &gctx.types[ty0].inner;
                        let ty1_inner = &gctx.types[ty1].inner;
                        // The two blend sources must have the same type...
                        if !ty0_inner.non_struct_equivalent(ty1_inner, gctx.types) {
                            return Err(TypeError::InvalidBlendSrc(
                                super::VaryingError::BlendSrcOutputTypeMismatch {
                                    blend_src_0_type: ty0,
                                    blend_src_1_type: ty1,
                                },
                            ));
                        }
                        // ... and that type must be I/O-shareable.
                        if !self.types[ty0.index()]
                            .flags
                            .contains(TypeFlags::IO_SHAREABLE)
                        {
                            return Err(TypeError::InvalidBlendSrc(
                                super::VaryingError::NotIOShareableType(ty0),
                            ));
                        }

                        // `@blend_src` is the only case where we classify a struct as
                        // I/O-shareable. (In the case of a struct with `@location` bindings, we
                        // process the members individually in interface validation, and do not
                        // classify the struct as I/O-shareable.)
                        ti.flags |= TypeFlags::IO_SHAREABLE;
                    }
                    [None, Some(_)] | [Some(_), None] => {
                        // Only one of the blend sources was specified.
                        return Err(TypeError::InvalidBlendSrc(
                            super::VaryingError::IncompleteBlendSrcUsage {
                                present_blend_src: blend_src_types
                                    .iter()
                                    .position(|src| src.is_some())
                                    .unwrap()
                                    as u32,
                            },
                        ));
                    }
                }

                let alignment = self.layouter[handle].alignment;
                if !alignment.is_aligned(span) {
                    ti.uniform_layout = Err((handle, Disalignment::StructSpan { span, alignment }));
                    ti.storage_layout = Err((handle, Disalignment::StructSpan { span, alignment }));
                }

                ti
            }
            Ti::Image {
                dim,
                arrayed,
                class,
            } => {
                if arrayed && matches!(dim, crate::ImageDimension::D3) {
                    return Err(TypeError::UnsupportedImageType {
                        dim,
                        arrayed,
                        class,
                    });
                }
                if arrayed && matches!(dim, crate::ImageDimension::Cube) {
                    self.require_type_capability(Capabilities::CUBE_ARRAY_TEXTURES)?;
                }
                if matches!(class, crate::ImageClass::External) {
                    if dim != crate::ImageDimension::D2 || arrayed {
                        return Err(TypeError::UnsupportedImageType {
                            dim,
                            arrayed,
                            class,
                        });
                    }
                    self.require_type_capability(Capabilities::TEXTURE_EXTERNAL)?;
                }
                TypeInfo::new(
                    TypeFlags::ARGUMENT | TypeFlags::CREATION_RESOLVED,
                    Alignment::ONE,
                )
            }
            Ti::Sampler { .. } => TypeInfo::new(
                TypeFlags::ARGUMENT | TypeFlags::CREATION_RESOLVED,
                Alignment::ONE,
            ),
            Ti::AccelerationStructure { vertex_return } => {
                self.require_type_capability(Capabilities::RAY_TRACING_PIPELINE)
                    .or_else(|_| self.require_type_capability(Capabilities::RAY_QUERY))?;
                if vertex_return {
                    self.require_type_capability(Capabilities::RAY_HIT_VERTEX_POSITION)?;
                }
                TypeInfo::new(
                    TypeFlags::ARGUMENT | TypeFlags::CREATION_RESOLVED,
                    Alignment::ONE,
                )
            }
            Ti::RayQuery { vertex_return } => {
                self.require_type_capability(Capabilities::RAY_QUERY)?;
                if vertex_return {
                    self.require_type_capability(Capabilities::RAY_HIT_VERTEX_POSITION)?;
                }
                TypeInfo::new(
                    TypeFlags::DATA
                        | TypeFlags::CONSTRUCTIBLE
                        | TypeFlags::SIZED
                        | TypeFlags::CREATION_RESOLVED,
                    Alignment::ONE,
                )
            }
            Ti::BindingArray { base, size } => {
                let type_info_mask = match size {
                    crate::ArraySize::Constant(_) => {
                        TypeFlags::SIZED | TypeFlags::HOST_SHAREABLE | TypeFlags::CREATION_RESOLVED
                    }
                    crate::ArraySize::Pending(_) => TypeFlags::SIZED | TypeFlags::HOST_SHAREABLE,
                    crate::ArraySize::Dynamic => {
                        // Final type is non-sized
                        TypeFlags::HOST_SHAREABLE | TypeFlags::CREATION_RESOLVED
                    }
                };
                let base_info = &self.types[base.index()];

                if base_info.flags.contains(TypeFlags::DATA) {
                    // Currently Naga only supports binding arrays of structs for non-handle types.
                    // `validate_global_var` relies on ray queries (which are `DATA`) being rejected here
                    match gctx.types[base].inner {
                        crate::TypeInner::Struct { .. } => {}
                        _ => return Err(TypeError::BindingArrayBaseTypeNotStruct(base)),
                    };
                }
                if matches!(
                    gctx.types[base].inner,
                    crate::TypeInner::Image {
                        class: crate::ImageClass::External,
                        ..
                    }
                ) {
                    // Binding arrays of external textures are not yet supported.
                    // See <https://github.com/gfx-rs/wgpu/issues/8027>. Note that
                    // `validate_global_var` relies on this error being raised here.
                    return Err(TypeError::BindingArrayBaseExternalTextures);
                }

                if !base_info.flags.contains(TypeFlags::CREATION_RESOLVED) {
                    return Err(TypeError::InvalidData(base));
                }

                TypeInfo::new(base_info.flags & type_info_mask, Alignment::ONE)
            }
        })
    }
}
