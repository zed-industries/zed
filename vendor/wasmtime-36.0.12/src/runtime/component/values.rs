use crate::ValRaw;
use crate::component::ResourceAny;
use crate::component::concurrent::{self, ErrorContext, FutureReader, StreamReader};
use crate::component::func::{Lift, LiftContext, Lower, LowerContext, desc};
use crate::prelude::*;
use core::mem::MaybeUninit;
use core::slice::{Iter, IterMut};
use wasmtime_component_util::{DiscriminantSize, FlagsSize};
use wasmtime_environ::component::{
    CanonicalAbiInfo, InterfaceType, TypeEnum, TypeFlags, TypeListIndex, TypeOption, TypeResult,
    TypeVariant, VariantInfo,
};

/// Represents possible runtime values which a component function can either
/// consume or produce
///
/// This is a dynamic representation of possible values in the component model.
/// Note that this is not an efficient representation but is instead intended to
/// be a flexible and somewhat convenient representation. The most efficient
/// representation of component model types is to use the `bindgen!` macro to
/// generate native Rust types with specialized liftings and lowerings.
///
/// This type is used in conjunction with [`Func::call`] for example if the
/// signature of a component is not statically known ahead of time.
///
/// # Equality and `Val`
///
/// This type implements both the Rust `PartialEq` and `Eq` traits. This type
/// additionally contains values which are not necessarily easily equated,
/// however, such as floats (`Float32` and `Float64`) and resources. Equality
/// does require that two values have the same type, and then these cases are
/// handled as:
///
/// * Floats are tested if they are "semantically the same" meaning all NaN
///   values are equal to all other NaN values. Additionally zero values must be
///   exactly the same, so positive zero is not equal to negative zero. The
///   primary use case at this time is fuzzing-related equality which this is
///   sufficient for.
///
/// * Resources are tested if their types and indices into the host table are
///   equal. This does not compare the underlying representation so borrows of
///   the same guest resource are not considered equal. This additionally
///   doesn't go further and test for equality in the guest itself (for example
///   two different heap allocations of `Box<u32>` can be equal in normal Rust
///   if they contain the same value, but will never be considered equal when
///   compared as `Val::Resource`s).
///
/// In general if a strict guarantee about equality is required here it's
/// recommended to "build your own" as this equality intended for fuzzing
/// Wasmtime may not be suitable for you.
///
/// # Component model types and `Val`
///
/// The `Val` type here does not contain enough information to say what the
/// component model type of a `Val` is. This is instead more of an AST of sorts.
/// For example the `Val::Enum` only carries information about a single
/// discriminant, not the entire enumeration or what it's a discriminant of.
///
/// This means that when a `Val` is passed to Wasmtime, for example as a
/// function parameter when calling a function or as a return value from an
/// host-defined imported function, then it must pass a type-check. Instances of
/// `Val` are type-checked against what's required by the component itself.
///
/// [`Func::call`]: crate::component::Func::call
#[derive(Debug, Clone)]
#[expect(missing_docs, reason = "self-describing variants")]
pub enum Val {
    Bool(bool),
    S8(i8),
    U8(u8),
    S16(i16),
    U16(u16),
    S32(i32),
    U32(u32),
    S64(i64),
    U64(u64),
    Float32(f32),
    Float64(f64),
    Char(char),
    String(String),
    List(Vec<Val>),
    Record(Vec<(String, Val)>),
    Tuple(Vec<Val>),
    Variant(String, Option<Box<Val>>),
    Enum(String),
    Option(Option<Box<Val>>),
    Result(Result<Option<Box<Val>>, Option<Box<Val>>>),
    Flags(Vec<String>),
    Resource(ResourceAny),
    Future(FutureAny),
    Stream(StreamAny),
    ErrorContext(ErrorContextAny),
}

impl Val {
    /// Deserialize a value of this type from core Wasm stack values.
    pub(crate) fn lift(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &mut Iter<'_, ValRaw>,
    ) -> Result<Val> {
        Ok(match ty {
            InterfaceType::Bool => Val::Bool(bool::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::S8 => Val::S8(i8::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::U8 => Val::U8(u8::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::S16 => Val::S16(i16::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::U16 => Val::U16(u16::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::S32 => Val::S32(i32::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::U32 => Val::U32(u32::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::S64 => Val::S64(i64::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::U64 => Val::U64(u64::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::Float32 => Val::Float32(f32::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::Float64 => Val::Float64(f64::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::Char => Val::Char(char::linear_lift_from_flat(cx, ty, next(src))?),
            InterfaceType::Own(_) | InterfaceType::Borrow(_) => {
                Val::Resource(ResourceAny::linear_lift_from_flat(cx, ty, next(src))?)
            }
            InterfaceType::String => Val::String(<_>::linear_lift_from_flat(
                cx,
                ty,
                &[*next(src), *next(src)],
            )?),
            InterfaceType::List(i) => {
                // FIXME(#4311): needs memory64 treatment
                let ptr = u32::linear_lift_from_flat(cx, InterfaceType::U32, next(src))? as usize;
                let len = u32::linear_lift_from_flat(cx, InterfaceType::U32, next(src))? as usize;
                load_list(cx, i, ptr, len)?
            }
            InterfaceType::Record(i) => Val::Record(
                cx.types[i]
                    .fields
                    .iter()
                    .map(|field| {
                        let val = Self::lift(cx, field.ty, src)?;
                        Ok((field.name.to_string(), val))
                    })
                    .collect::<Result<_>>()?,
            ),
            InterfaceType::Tuple(i) => Val::Tuple(
                cx.types[i]
                    .types
                    .iter()
                    .map(|ty| Self::lift(cx, *ty, src))
                    .collect::<Result<_>>()?,
            ),
            InterfaceType::Variant(i) => {
                let vty = &cx.types[i];
                let (discriminant, value) = lift_variant(
                    cx,
                    cx.types.canonical_abi(&ty).flat_count(usize::MAX).unwrap(),
                    vty.cases.values().copied(),
                    src,
                )?;

                let (k, _) = vty.cases.get_index(discriminant as usize).unwrap();
                Val::Variant(k.clone(), value)
            }
            InterfaceType::Enum(i) => {
                let ety = &cx.types[i];
                let (discriminant, _) = lift_variant(
                    cx,
                    cx.types.canonical_abi(&ty).flat_count(usize::MAX).unwrap(),
                    ety.names.iter().map(|_| None),
                    src,
                )?;

                Val::Enum(ety.names[discriminant as usize].clone())
            }
            InterfaceType::Option(i) => {
                let (_discriminant, value) = lift_variant(
                    cx,
                    cx.types.canonical_abi(&ty).flat_count(usize::MAX).unwrap(),
                    [None, Some(cx.types[i].ty)].into_iter(),
                    src,
                )?;

                Val::Option(value)
            }
            InterfaceType::Result(i) => {
                let result_ty = &cx.types[i];
                let (discriminant, value) = lift_variant(
                    cx,
                    cx.types.canonical_abi(&ty).flat_count(usize::MAX).unwrap(),
                    [result_ty.ok, result_ty.err].into_iter(),
                    src,
                )?;

                Val::Result(if discriminant == 0 {
                    Ok(value)
                } else {
                    Err(value)
                })
            }
            InterfaceType::Flags(i) => {
                let u32_count = cx.types.canonical_abi(&ty).flat_count(usize::MAX).unwrap();
                let ty = &cx.types[i];
                let mut flags = Vec::new();
                for i in 0..u32::try_from(u32_count).unwrap() {
                    push_flags(
                        ty,
                        &mut flags,
                        i * 32,
                        u32::linear_lift_from_flat(cx, InterfaceType::U32, next(src))?,
                    );
                }

                Val::Flags(flags)
            }
            InterfaceType::Future(_) => {
                FutureReader::<()>::linear_lift_from_flat(cx, ty, next(src))?.into_val()
            }
            InterfaceType::Stream(_) => {
                StreamReader::<()>::linear_lift_from_flat(cx, ty, next(src))?.into_val()
            }
            InterfaceType::ErrorContext(_) => {
                ErrorContext::linear_lift_from_flat(cx, ty, next(src))?.into_val()
            }
        })
    }

    /// Deserialize a value of this type from the heap.
    pub(crate) fn load(cx: &mut LiftContext<'_>, ty: InterfaceType, bytes: &[u8]) -> Result<Val> {
        Ok(match ty {
            InterfaceType::Bool => Val::Bool(bool::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::S8 => Val::S8(i8::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::U8 => Val::U8(u8::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::S16 => Val::S16(i16::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::U16 => Val::U16(u16::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::S32 => Val::S32(i32::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::U32 => Val::U32(u32::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::S64 => Val::S64(i64::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::U64 => Val::U64(u64::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::Float32 => Val::Float32(f32::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::Float64 => Val::Float64(f64::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::Char => Val::Char(char::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::String => Val::String(<_>::linear_lift_from_memory(cx, ty, bytes)?),
            InterfaceType::Own(_) | InterfaceType::Borrow(_) => {
                Val::Resource(ResourceAny::linear_lift_from_memory(cx, ty, bytes)?)
            }
            InterfaceType::List(i) => {
                // FIXME(#4311): needs memory64 treatment
                let ptr = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize;
                let len = u32::from_le_bytes(bytes[4..].try_into().unwrap()) as usize;
                load_list(cx, i, ptr, len)?
            }

            InterfaceType::Record(i) => {
                let mut offset = 0;
                let fields = cx.types[i].fields.iter();
                Val::Record(
                    fields
                        .map(|field| -> Result<(String, Val)> {
                            let abi = cx.types.canonical_abi(&field.ty);
                            let offset = abi.next_field32(&mut offset);
                            let offset = usize::try_from(offset).unwrap();
                            let size = usize::try_from(abi.size32).unwrap();
                            Ok((
                                field.name.to_string(),
                                Val::load(cx, field.ty, &bytes[offset..][..size])?,
                            ))
                        })
                        .collect::<Result<_>>()?,
                )
            }
            InterfaceType::Tuple(i) => {
                let types = cx.types[i].types.iter().copied();
                let mut offset = 0;
                Val::Tuple(
                    types
                        .map(|ty| {
                            let abi = cx.types.canonical_abi(&ty);
                            let offset = abi.next_field32(&mut offset);
                            let offset = usize::try_from(offset).unwrap();
                            let size = usize::try_from(abi.size32).unwrap();
                            Val::load(cx, ty, &bytes[offset..][..size])
                        })
                        .collect::<Result<_>>()?,
                )
            }
            InterfaceType::Variant(i) => {
                let ty = &cx.types[i];
                let (discriminant, value) =
                    load_variant(cx, &ty.info, ty.cases.values().copied(), bytes)?;

                let (k, _) = ty.cases.get_index(discriminant as usize).unwrap();
                Val::Variant(k.clone(), value)
            }
            InterfaceType::Enum(i) => {
                let ty = &cx.types[i];
                let (discriminant, _) =
                    load_variant(cx, &ty.info, ty.names.iter().map(|_| None), bytes)?;

                Val::Enum(ty.names[discriminant as usize].clone())
            }
            InterfaceType::Option(i) => {
                let ty = &cx.types[i];
                let (_discriminant, value) =
                    load_variant(cx, &ty.info, [None, Some(ty.ty)].into_iter(), bytes)?;

                Val::Option(value)
            }
            InterfaceType::Result(i) => {
                let ty = &cx.types[i];
                let (discriminant, value) =
                    load_variant(cx, &ty.info, [ty.ok, ty.err].into_iter(), bytes)?;

                Val::Result(if discriminant == 0 {
                    Ok(value)
                } else {
                    Err(value)
                })
            }
            InterfaceType::Flags(i) => {
                let ty = &cx.types[i];
                let mut flags = Vec::new();
                match FlagsSize::from_count(ty.names.len()) {
                    FlagsSize::Size0 => {}
                    FlagsSize::Size1 => {
                        let bits = u8::linear_lift_from_memory(cx, InterfaceType::U8, bytes)?;
                        push_flags(ty, &mut flags, 0, u32::from(bits));
                    }
                    FlagsSize::Size2 => {
                        let bits = u16::linear_lift_from_memory(cx, InterfaceType::U16, bytes)?;
                        push_flags(ty, &mut flags, 0, u32::from(bits));
                    }
                    FlagsSize::Size4Plus(n) => {
                        for i in 0..n {
                            let bits = u32::linear_lift_from_memory(
                                cx,
                                InterfaceType::U32,
                                &bytes[usize::from(i) * 4..][..4],
                            )?;
                            push_flags(ty, &mut flags, u32::from(i) * 32, bits);
                        }
                    }
                }
                Val::Flags(flags)
            }
            InterfaceType::Future(_) => {
                FutureReader::<()>::linear_lift_from_memory(cx, ty, bytes)?.into_val()
            }
            InterfaceType::Stream(_) => {
                StreamReader::<()>::linear_lift_from_memory(cx, ty, bytes)?.into_val()
            }
            InterfaceType::ErrorContext(_) => {
                ErrorContext::linear_lift_from_memory(cx, ty, bytes)?.into_val()
            }
        })
    }

    /// Serialize this value as core Wasm stack values.
    pub(crate) fn lower<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        dst: &mut IterMut<'_, MaybeUninit<ValRaw>>,
    ) -> Result<()> {
        match (ty, self) {
            (InterfaceType::Bool, Val::Bool(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::Bool, _) => unexpected(ty, self),
            (InterfaceType::S8, Val::S8(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::S8, _) => unexpected(ty, self),
            (InterfaceType::U8, Val::U8(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::U8, _) => unexpected(ty, self),
            (InterfaceType::S16, Val::S16(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::S16, _) => unexpected(ty, self),
            (InterfaceType::U16, Val::U16(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::U16, _) => unexpected(ty, self),
            (InterfaceType::S32, Val::S32(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::S32, _) => unexpected(ty, self),
            (InterfaceType::U32, Val::U32(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::U32, _) => unexpected(ty, self),
            (InterfaceType::S64, Val::S64(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::S64, _) => unexpected(ty, self),
            (InterfaceType::U64, Val::U64(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::U64, _) => unexpected(ty, self),
            (InterfaceType::Float32, Val::Float32(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::Float32, _) => unexpected(ty, self),
            (InterfaceType::Float64, Val::Float64(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::Float64, _) => unexpected(ty, self),
            (InterfaceType::Char, Val::Char(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::Char, _) => unexpected(ty, self),
            // NB: `lower` on `ResourceAny` does its own type-checking, so skip
            // looking at it here.
            (InterfaceType::Borrow(_) | InterfaceType::Own(_), Val::Resource(value)) => {
                value.linear_lower_to_flat(cx, ty, next_mut(dst))
            }
            (InterfaceType::Borrow(_) | InterfaceType::Own(_), _) => unexpected(ty, self),
            (InterfaceType::String, Val::String(value)) => {
                let my_dst = &mut MaybeUninit::<[ValRaw; 2]>::uninit();
                value.linear_lower_to_flat(cx, ty, my_dst)?;
                let my_dst = unsafe { my_dst.assume_init() };
                next_mut(dst).write(my_dst[0]);
                next_mut(dst).write(my_dst[1]);
                Ok(())
            }
            (InterfaceType::String, _) => unexpected(ty, self),
            (InterfaceType::List(ty), Val::List(values)) => {
                let ty = &cx.types[ty];
                let (ptr, len) = lower_list(cx, ty.element, values)?;
                next_mut(dst).write(ValRaw::i64(ptr as i64));
                next_mut(dst).write(ValRaw::i64(len as i64));
                Ok(())
            }
            (InterfaceType::List(_), _) => unexpected(ty, self),
            (InterfaceType::Record(ty), Val::Record(values)) => {
                let ty = &cx.types[ty];
                if ty.fields.len() != values.len() {
                    bail!("expected {} fields, got {}", ty.fields.len(), values.len());
                }
                for ((name, value), field) in values.iter().zip(ty.fields.iter()) {
                    if *name != field.name {
                        bail!("expected field `{}`, got `{name}`", field.name);
                    }
                    value.lower(cx, field.ty, dst)?;
                }
                Ok(())
            }
            (InterfaceType::Record(_), _) => unexpected(ty, self),
            (InterfaceType::Tuple(ty), Val::Tuple(values)) => {
                let ty = &cx.types[ty];
                if ty.types.len() != values.len() {
                    bail!("expected {} types, got {}", ty.types.len(), values.len());
                }
                for (value, ty) in values.iter().zip(ty.types.iter()) {
                    value.lower(cx, *ty, dst)?;
                }
                Ok(())
            }
            (InterfaceType::Tuple(_), _) => unexpected(ty, self),
            (InterfaceType::Variant(ty), Val::Variant(n, v)) => {
                GenericVariant::variant(&cx.types[ty], n, v)?.lower(cx, dst)
            }
            (InterfaceType::Variant(_), _) => unexpected(ty, self),
            (InterfaceType::Option(ty), Val::Option(v)) => {
                GenericVariant::option(&cx.types[ty], v).lower(cx, dst)
            }
            (InterfaceType::Option(_), _) => unexpected(ty, self),
            (InterfaceType::Result(ty), Val::Result(v)) => {
                GenericVariant::result(&cx.types[ty], v)?.lower(cx, dst)
            }
            (InterfaceType::Result(_), _) => unexpected(ty, self),
            (InterfaceType::Enum(ty), Val::Enum(discriminant)) => {
                let discriminant = get_enum_discriminant(&cx.types[ty], discriminant)?;
                next_mut(dst).write(ValRaw::u32(discriminant));
                Ok(())
            }
            (InterfaceType::Enum(_), _) => unexpected(ty, self),
            (InterfaceType::Flags(ty), Val::Flags(value)) => {
                let ty = &cx.types[ty];
                let storage = flags_to_storage(ty, value)?;
                for value in storage {
                    next_mut(dst).write(ValRaw::u32(value));
                }
                Ok(())
            }
            (InterfaceType::Flags(_), _) => unexpected(ty, self),
            (InterfaceType::Future(_), Val::Future(FutureAny(rep))) => {
                concurrent::lower_future_to_index(*rep, cx, ty)?.linear_lower_to_flat(
                    cx,
                    InterfaceType::U32,
                    next_mut(dst),
                )
            }
            (InterfaceType::Future(_), _) => unexpected(ty, self),
            (InterfaceType::Stream(_), Val::Stream(StreamAny(rep))) => {
                concurrent::lower_stream_to_index(*rep, cx, ty)?.linear_lower_to_flat(
                    cx,
                    InterfaceType::U32,
                    next_mut(dst),
                )
            }
            (InterfaceType::Stream(_), _) => unexpected(ty, self),
            (InterfaceType::ErrorContext(_), Val::ErrorContext(ErrorContextAny(rep))) => {
                concurrent::lower_error_context_to_index(*rep, cx, ty)?.linear_lower_to_flat(
                    cx,
                    InterfaceType::U32,
                    next_mut(dst),
                )
            }
            (InterfaceType::ErrorContext(_), _) => unexpected(ty, self),
        }
    }

    /// Serialize this value to the heap at the specified memory location.
    pub(crate) fn store<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        debug_assert!(offset % usize::try_from(cx.types.canonical_abi(&ty).align32)? == 0);

        match (ty, self) {
            (InterfaceType::Bool, Val::Bool(value)) => value.linear_lower_to_memory(cx, ty, offset),
            (InterfaceType::Bool, _) => unexpected(ty, self),
            (InterfaceType::U8, Val::U8(value)) => value.linear_lower_to_memory(cx, ty, offset),
            (InterfaceType::U8, _) => unexpected(ty, self),
            (InterfaceType::S8, Val::S8(value)) => value.linear_lower_to_memory(cx, ty, offset),
            (InterfaceType::S8, _) => unexpected(ty, self),
            (InterfaceType::U16, Val::U16(value)) => value.linear_lower_to_memory(cx, ty, offset),
            (InterfaceType::U16, _) => unexpected(ty, self),
            (InterfaceType::S16, Val::S16(value)) => value.linear_lower_to_memory(cx, ty, offset),
            (InterfaceType::S16, _) => unexpected(ty, self),
            (InterfaceType::U32, Val::U32(value)) => value.linear_lower_to_memory(cx, ty, offset),
            (InterfaceType::U32, _) => unexpected(ty, self),
            (InterfaceType::S32, Val::S32(value)) => value.linear_lower_to_memory(cx, ty, offset),
            (InterfaceType::S32, _) => unexpected(ty, self),
            (InterfaceType::U64, Val::U64(value)) => value.linear_lower_to_memory(cx, ty, offset),
            (InterfaceType::U64, _) => unexpected(ty, self),
            (InterfaceType::S64, Val::S64(value)) => value.linear_lower_to_memory(cx, ty, offset),
            (InterfaceType::S64, _) => unexpected(ty, self),
            (InterfaceType::Float32, Val::Float32(value)) => {
                value.linear_lower_to_memory(cx, ty, offset)
            }
            (InterfaceType::Float32, _) => unexpected(ty, self),
            (InterfaceType::Float64, Val::Float64(value)) => {
                value.linear_lower_to_memory(cx, ty, offset)
            }
            (InterfaceType::Float64, _) => unexpected(ty, self),
            (InterfaceType::Char, Val::Char(value)) => value.linear_lower_to_memory(cx, ty, offset),
            (InterfaceType::Char, _) => unexpected(ty, self),
            (InterfaceType::String, Val::String(value)) => {
                value.linear_lower_to_memory(cx, ty, offset)
            }
            (InterfaceType::String, _) => unexpected(ty, self),

            // NB: resources do type-checking when they lower.
            (InterfaceType::Borrow(_) | InterfaceType::Own(_), Val::Resource(value)) => {
                value.linear_lower_to_memory(cx, ty, offset)
            }
            (InterfaceType::Borrow(_) | InterfaceType::Own(_), _) => unexpected(ty, self),
            (InterfaceType::List(ty), Val::List(values)) => {
                let ty = &cx.types[ty];
                let (ptr, len) = lower_list(cx, ty.element, values)?;
                // FIXME(#4311): needs memory64 handling
                *cx.get(offset + 0) = u32::try_from(ptr).unwrap().to_le_bytes();
                *cx.get(offset + 4) = u32::try_from(len).unwrap().to_le_bytes();
                Ok(())
            }
            (InterfaceType::List(_), _) => unexpected(ty, self),
            (InterfaceType::Record(ty), Val::Record(values)) => {
                let ty = &cx.types[ty];
                if ty.fields.len() != values.len() {
                    bail!("expected {} fields, got {}", ty.fields.len(), values.len());
                }
                let mut offset = offset;
                for ((name, value), field) in values.iter().zip(ty.fields.iter()) {
                    if *name != field.name {
                        bail!("expected field `{}`, got `{name}`", field.name);
                    }
                    value.store(
                        cx,
                        field.ty,
                        cx.types
                            .canonical_abi(&field.ty)
                            .next_field32_size(&mut offset),
                    )?;
                }
                Ok(())
            }
            (InterfaceType::Record(_), _) => unexpected(ty, self),
            (InterfaceType::Tuple(ty), Val::Tuple(values)) => {
                let ty = &cx.types[ty];
                if ty.types.len() != values.len() {
                    bail!("expected {} types, got {}", ty.types.len(), values.len());
                }
                let mut offset = offset;
                for (value, ty) in values.iter().zip(ty.types.iter()) {
                    value.store(
                        cx,
                        *ty,
                        cx.types.canonical_abi(ty).next_field32_size(&mut offset),
                    )?;
                }
                Ok(())
            }
            (InterfaceType::Tuple(_), _) => unexpected(ty, self),

            (InterfaceType::Variant(ty), Val::Variant(n, v)) => {
                GenericVariant::variant(&cx.types[ty], n, v)?.store(cx, offset)
            }
            (InterfaceType::Variant(_), _) => unexpected(ty, self),
            (InterfaceType::Enum(ty), Val::Enum(v)) => {
                GenericVariant::enum_(&cx.types[ty], v)?.store(cx, offset)
            }
            (InterfaceType::Enum(_), _) => unexpected(ty, self),
            (InterfaceType::Option(ty), Val::Option(v)) => {
                GenericVariant::option(&cx.types[ty], v).store(cx, offset)
            }
            (InterfaceType::Option(_), _) => unexpected(ty, self),
            (InterfaceType::Result(ty), Val::Result(v)) => {
                GenericVariant::result(&cx.types[ty], v)?.store(cx, offset)
            }
            (InterfaceType::Result(_), _) => unexpected(ty, self),

            (InterfaceType::Flags(ty), Val::Flags(flags)) => {
                let ty = &cx.types[ty];
                let storage = flags_to_storage(ty, flags)?;
                match FlagsSize::from_count(ty.names.len()) {
                    FlagsSize::Size0 => {}
                    FlagsSize::Size1 => u8::try_from(storage[0]).unwrap().linear_lower_to_memory(
                        cx,
                        InterfaceType::U8,
                        offset,
                    )?,
                    FlagsSize::Size2 => u16::try_from(storage[0]).unwrap().linear_lower_to_memory(
                        cx,
                        InterfaceType::U16,
                        offset,
                    )?,
                    FlagsSize::Size4Plus(_) => {
                        let mut offset = offset;
                        for value in storage {
                            value.linear_lower_to_memory(cx, InterfaceType::U32, offset)?;
                            offset += 4;
                        }
                    }
                }
                Ok(())
            }
            (InterfaceType::Flags(_), _) => unexpected(ty, self),
            (InterfaceType::Future(_), Val::Future(FutureAny(rep))) => {
                concurrent::lower_future_to_index::<T>(*rep, cx, ty)?.linear_lower_to_memory(
                    cx,
                    InterfaceType::U32,
                    offset,
                )
            }
            (InterfaceType::Future(_), _) => unexpected(ty, self),
            (InterfaceType::Stream(_), Val::Stream(StreamAny(rep))) => {
                concurrent::lower_stream_to_index::<T>(*rep, cx, ty)?.linear_lower_to_memory(
                    cx,
                    InterfaceType::U32,
                    offset,
                )
            }
            (InterfaceType::Stream(_), _) => unexpected(ty, self),
            (InterfaceType::ErrorContext(_), Val::ErrorContext(ErrorContextAny(rep))) => {
                concurrent::lower_error_context_to_index(*rep, cx, ty)?.linear_lower_to_memory(
                    cx,
                    InterfaceType::U32,
                    offset,
                )
            }
            (InterfaceType::ErrorContext(_), _) => unexpected(ty, self),
        }
    }

    pub(crate) fn desc(&self) -> &'static str {
        match self {
            Val::Bool(_) => "bool",
            Val::U8(_) => "u8",
            Val::S8(_) => "s8",
            Val::U16(_) => "u16",
            Val::S16(_) => "s16",
            Val::U32(_) => "u32",
            Val::S32(_) => "s32",
            Val::U64(_) => "u64",
            Val::S64(_) => "s64",
            Val::Float32(_) => "f32",
            Val::Float64(_) => "f64",
            Val::Char(_) => "char",
            Val::List(_) => "list",
            Val::String(_) => "string",
            Val::Record(_) => "record",
            Val::Enum(_) => "enum",
            Val::Variant(..) => "variant",
            Val::Tuple(_) => "tuple",
            Val::Option(_) => "option",
            Val::Result(_) => "result",
            Val::Resource(_) => "resource",
            Val::Flags(_) => "flags",
            Val::Future(_) => "future",
            Val::Stream(_) => "stream",
            Val::ErrorContext(_) => "error-context",
        }
    }

    /// Deserialize a [`Val`] from its [`crate::component::wasm_wave`] encoding. Deserialization
    /// requires a target [`crate::component::Type`].
    #[cfg(feature = "wave")]
    pub fn from_wave(ty: &crate::component::Type, s: &str) -> Result<Self> {
        Ok(wasm_wave::from_str(ty, s)?)
    }

    /// Serialize a [`Val`] to its [`crate::component::wasm_wave`] encoding.
    #[cfg(feature = "wave")]
    pub fn to_wave(&self) -> Result<String> {
        Ok(wasm_wave::to_string(self)?)
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // IEEE 754 equality considers NaN inequal to NaN and negative zero
            // equal to positive zero, however we do the opposite here, because
            // this logic is used by testing and fuzzing, which want to know
            // whether two values are semantically the same, rather than
            // numerically equal.
            (Self::Float32(l), Self::Float32(r)) => {
                (*l != 0.0 && l == r)
                    || (*l == 0.0 && l.to_bits() == r.to_bits())
                    || (l.is_nan() && r.is_nan())
            }
            (Self::Float32(_), _) => false,
            (Self::Float64(l), Self::Float64(r)) => {
                (*l != 0.0 && l == r)
                    || (*l == 0.0 && l.to_bits() == r.to_bits())
                    || (l.is_nan() && r.is_nan())
            }
            (Self::Float64(_), _) => false,

            (Self::Bool(l), Self::Bool(r)) => l == r,
            (Self::Bool(_), _) => false,
            (Self::S8(l), Self::S8(r)) => l == r,
            (Self::S8(_), _) => false,
            (Self::U8(l), Self::U8(r)) => l == r,
            (Self::U8(_), _) => false,
            (Self::S16(l), Self::S16(r)) => l == r,
            (Self::S16(_), _) => false,
            (Self::U16(l), Self::U16(r)) => l == r,
            (Self::U16(_), _) => false,
            (Self::S32(l), Self::S32(r)) => l == r,
            (Self::S32(_), _) => false,
            (Self::U32(l), Self::U32(r)) => l == r,
            (Self::U32(_), _) => false,
            (Self::S64(l), Self::S64(r)) => l == r,
            (Self::S64(_), _) => false,
            (Self::U64(l), Self::U64(r)) => l == r,
            (Self::U64(_), _) => false,
            (Self::Char(l), Self::Char(r)) => l == r,
            (Self::Char(_), _) => false,
            (Self::String(l), Self::String(r)) => l == r,
            (Self::String(_), _) => false,
            (Self::List(l), Self::List(r)) => l == r,
            (Self::List(_), _) => false,
            (Self::Record(l), Self::Record(r)) => l == r,
            (Self::Record(_), _) => false,
            (Self::Tuple(l), Self::Tuple(r)) => l == r,
            (Self::Tuple(_), _) => false,
            (Self::Variant(ln, lv), Self::Variant(rn, rv)) => ln == rn && lv == rv,
            (Self::Variant(..), _) => false,
            (Self::Enum(l), Self::Enum(r)) => l == r,
            (Self::Enum(_), _) => false,
            (Self::Option(l), Self::Option(r)) => l == r,
            (Self::Option(_), _) => false,
            (Self::Result(l), Self::Result(r)) => l == r,
            (Self::Result(_), _) => false,
            (Self::Flags(l), Self::Flags(r)) => l == r,
            (Self::Flags(_), _) => false,
            (Self::Resource(l), Self::Resource(r)) => l == r,
            (Self::Resource(_), _) => false,
            (Self::Future(l), Self::Future(r)) => l == r,
            (Self::Future(_), _) => false,
            (Self::Stream(l), Self::Stream(r)) => l == r,
            (Self::Stream(_), _) => false,
            (Self::ErrorContext(l), Self::ErrorContext(r)) => l == r,
            (Self::ErrorContext(_), _) => false,
        }
    }
}

impl Eq for Val {}

struct GenericVariant<'a> {
    discriminant: u32,
    payload: Option<(&'a Val, InterfaceType)>,
    abi: &'a CanonicalAbiInfo,
    info: &'a VariantInfo,
}

impl GenericVariant<'_> {
    fn result<'a>(
        ty: &'a TypeResult,
        r: &'a Result<Option<Box<Val>>, Option<Box<Val>>>,
    ) -> Result<GenericVariant<'a>> {
        let (discriminant, payload) = match r {
            Ok(val) => {
                let payload = match (val, ty.ok) {
                    (Some(val), Some(ty)) => Some((&**val, ty)),
                    (None, None) => None,
                    (Some(_), None) => {
                        bail!("payload provided to `ok` but not expected");
                    }
                    (None, Some(_)) => {
                        bail!("payload expected to `ok` but not provided");
                    }
                };
                (0, payload)
            }
            Err(val) => {
                let payload = match (val, ty.err) {
                    (Some(val), Some(ty)) => Some((&**val, ty)),
                    (None, None) => None,
                    (Some(_), None) => {
                        bail!("payload provided to `err` but not expected");
                    }
                    (None, Some(_)) => {
                        bail!("payload expected to `err` but not provided");
                    }
                };
                (1, payload)
            }
        };
        Ok(GenericVariant {
            discriminant,
            payload,
            abi: &ty.abi,
            info: &ty.info,
        })
    }

    fn option<'a>(ty: &'a TypeOption, r: &'a Option<Box<Val>>) -> GenericVariant<'a> {
        let (discriminant, payload) = match r {
            None => (0, None),
            Some(val) => (1, Some((&**val, ty.ty))),
        };
        GenericVariant {
            discriminant,
            payload,
            abi: &ty.abi,
            info: &ty.info,
        }
    }

    fn enum_<'a>(ty: &'a TypeEnum, discriminant: &str) -> Result<GenericVariant<'a>> {
        let discriminant = get_enum_discriminant(ty, discriminant)?;

        Ok(GenericVariant {
            discriminant,
            payload: None,
            abi: &ty.abi,
            info: &ty.info,
        })
    }

    fn variant<'a>(
        ty: &'a TypeVariant,
        discriminant_name: &str,
        payload: &'a Option<Box<Val>>,
    ) -> Result<GenericVariant<'a>> {
        let (discriminant, payload_ty) = get_variant_discriminant(ty, discriminant_name)?;

        let payload = match (payload, payload_ty) {
            (Some(val), Some(ty)) => Some((&**val, *ty)),
            (None, None) => None,
            (Some(_), None) => bail!("did not expect a payload for case `{discriminant_name}`"),
            (None, Some(_)) => bail!("expected a payload for case `{discriminant_name}`"),
        };

        Ok(GenericVariant {
            discriminant,
            payload,
            abi: &ty.abi,
            info: &ty.info,
        })
    }

    fn lower<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        dst: &mut IterMut<'_, MaybeUninit<ValRaw>>,
    ) -> Result<()> {
        next_mut(dst).write(ValRaw::u32(self.discriminant));

        // For the remaining lowered representation of this variant that
        // the payload didn't write we write out zeros here to ensure
        // the entire variant is written.
        let value_flat = match self.payload {
            Some((value, ty)) => {
                value.lower(cx, ty, dst)?;
                cx.types.canonical_abi(&ty).flat_count(usize::MAX).unwrap()
            }
            None => 0,
        };
        let variant_flat = self.abi.flat_count(usize::MAX).unwrap();
        for _ in (1 + value_flat)..variant_flat {
            next_mut(dst).write(ValRaw::u64(0));
        }
        Ok(())
    }

    fn store<T>(&self, cx: &mut LowerContext<'_, T>, offset: usize) -> Result<()> {
        match self.info.size {
            DiscriminantSize::Size1 => u8::try_from(self.discriminant)
                .unwrap()
                .linear_lower_to_memory(cx, InterfaceType::U8, offset)?,
            DiscriminantSize::Size2 => u16::try_from(self.discriminant)
                .unwrap()
                .linear_lower_to_memory(cx, InterfaceType::U16, offset)?,
            DiscriminantSize::Size4 => {
                self.discriminant
                    .linear_lower_to_memory(cx, InterfaceType::U32, offset)?
            }
        }

        if let Some((value, ty)) = self.payload {
            let offset = offset + usize::try_from(self.info.payload_offset32).unwrap();
            value.store(cx, ty, offset)?;
        }

        Ok(())
    }
}

fn load_list(cx: &mut LiftContext<'_>, ty: TypeListIndex, ptr: usize, len: usize) -> Result<Val> {
    let elem = cx.types[ty].element;
    let abi = cx.types.canonical_abi(&elem);
    let element_size = usize::try_from(abi.size32).unwrap();
    let element_alignment = abi.align32;

    match len
        .checked_mul(element_size)
        .and_then(|len| ptr.checked_add(len))
    {
        Some(n) if n <= cx.memory().len() => cx.consume_fuel(n - ptr)?,
        _ => bail!("list pointer/length out of bounds of memory"),
    }
    if ptr % usize::try_from(element_alignment)? != 0 {
        bail!("list pointer is not aligned")
    }

    Ok(Val::List(
        (0..len)
            .map(|index| {
                Val::load(
                    cx,
                    elem,
                    &cx.memory()[ptr + (index * element_size)..][..element_size],
                )
            })
            .collect::<Result<_>>()?,
    ))
}

fn load_variant(
    cx: &mut LiftContext<'_>,
    info: &VariantInfo,
    mut types: impl ExactSizeIterator<Item = Option<InterfaceType>>,
    bytes: &[u8],
) -> Result<(u32, Option<Box<Val>>)> {
    let discriminant = match info.size {
        DiscriminantSize::Size1 => u32::from(u8::linear_lift_from_memory(
            cx,
            InterfaceType::U8,
            &bytes[..1],
        )?),
        DiscriminantSize::Size2 => u32::from(u16::linear_lift_from_memory(
            cx,
            InterfaceType::U16,
            &bytes[..2],
        )?),
        DiscriminantSize::Size4 => {
            u32::linear_lift_from_memory(cx, InterfaceType::U32, &bytes[..4])?
        }
    };
    let case_ty = types.nth(discriminant as usize).ok_or_else(|| {
        anyhow!(
            "discriminant {} out of range [0..{})",
            discriminant,
            types.len()
        )
    })?;
    let value = match case_ty {
        Some(case_ty) => {
            let payload_offset = usize::try_from(info.payload_offset32).unwrap();
            let case_abi = cx.types.canonical_abi(&case_ty);
            let case_size = usize::try_from(case_abi.size32).unwrap();
            Some(Box::new(Val::load(
                cx,
                case_ty,
                &bytes[payload_offset..][..case_size],
            )?))
        }
        None => None,
    };
    Ok((discriminant, value))
}

fn lift_variant(
    cx: &mut LiftContext<'_>,
    flatten_count: usize,
    mut types: impl ExactSizeIterator<Item = Option<InterfaceType>>,
    src: &mut Iter<'_, ValRaw>,
) -> Result<(u32, Option<Box<Val>>)> {
    let len = types.len();
    let discriminant = next(src).get_u32();
    let ty = types
        .nth(discriminant as usize)
        .ok_or_else(|| anyhow!("discriminant {} out of range [0..{})", discriminant, len))?;
    let (value, value_flat) = match ty {
        Some(ty) => (
            Some(Box::new(Val::lift(cx, ty, src)?)),
            cx.types.canonical_abi(&ty).flat_count(usize::MAX).unwrap(),
        ),
        None => (None, 0),
    };
    for _ in (1 + value_flat)..flatten_count {
        next(src);
    }
    Ok((discriminant, value))
}

/// Lower a list with the specified element type and values.
fn lower_list<T>(
    cx: &mut LowerContext<'_, T>,
    element_type: InterfaceType,
    items: &[Val],
) -> Result<(usize, usize)> {
    let abi = cx.types.canonical_abi(&element_type);
    let elt_size = usize::try_from(abi.size32)?;
    let elt_align = abi.align32;
    let size = items
        .len()
        .checked_mul(elt_size)
        .ok_or_else(|| anyhow::anyhow!("size overflow copying a list"))?;
    let ptr = cx.realloc(0, 0, elt_align, size)?;
    let mut element_ptr = ptr;
    for item in items {
        item.store(cx, element_type, element_ptr)?;
        element_ptr += elt_size;
    }
    Ok((ptr, items.len()))
}

fn push_flags(ty: &TypeFlags, flags: &mut Vec<String>, mut offset: u32, mut bits: u32) {
    while bits > 0 && usize::try_from(offset).unwrap() < ty.names.len() {
        if bits & 1 != 0 {
            flags.push(ty.names[offset as usize].clone());
        }
        bits >>= 1;
        offset += 1;
    }
}

fn flags_to_storage(ty: &TypeFlags, flags: &[String]) -> Result<Vec<u32>> {
    let mut storage = match FlagsSize::from_count(ty.names.len()) {
        FlagsSize::Size0 => Vec::new(),
        FlagsSize::Size1 | FlagsSize::Size2 => vec![0],
        FlagsSize::Size4Plus(n) => vec![0; n.into()],
    };

    for flag in flags {
        let bit = ty
            .names
            .get_index_of(flag)
            .ok_or_else(|| anyhow::anyhow!("unknown flag: `{flag}`"))?;
        storage[bit / 32] |= 1 << (bit % 32);
    }
    Ok(storage)
}

fn get_enum_discriminant(ty: &TypeEnum, n: &str) -> Result<u32> {
    ty.names
        .get_index_of(n)
        .ok_or_else(|| anyhow::anyhow!("enum variant name `{n}` is not valid"))
        .map(|i| i.try_into().unwrap())
}

fn get_variant_discriminant<'a>(
    ty: &'a TypeVariant,
    name: &str,
) -> Result<(u32, &'a Option<InterfaceType>)> {
    let (i, _, ty) = ty
        .cases
        .get_full(name)
        .ok_or_else(|| anyhow::anyhow!("unknown variant case: `{name}`"))?;
    Ok((i.try_into().unwrap(), ty))
}

fn next<'a>(src: &mut Iter<'a, ValRaw>) -> &'a ValRaw {
    src.next().unwrap()
}

fn next_mut<'a>(dst: &mut IterMut<'a, MaybeUninit<ValRaw>>) -> &'a mut MaybeUninit<ValRaw> {
    dst.next().unwrap()
}

#[cold]
fn unexpected<T>(ty: InterfaceType, val: &Val) -> Result<T> {
    bail!(
        "type mismatch: expected {}, found {}",
        desc(&ty),
        val.desc()
    )
}

/// Represents a component model `future`.
///
/// Note that this type is not usable at this time as its implementation has not
/// been filled out. There are no operations on this and there's additionally no
/// ability to "drop" or deallocate this index. This means that from the
/// perspective of wasm it'll appear that this handle has "leaked" without ever
/// being dropped or read from.
//
// FIXME(#11161) this needs to be filled out implementation-wise
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureAny(pub(crate) u32);

/// Represents a component model `stream`.
///
/// Note that this type is not usable at this time as its implementation has not
/// been filled out. There are no operations on this and there's additionally no
/// ability to "drop" or deallocate this index. This means that from the
/// perspective of wasm it'll appear that this handle has "leaked" without ever
/// being dropped or read from.
//
// FIXME(#11161) this needs to be filled out implementation-wise
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamAny(pub(crate) u32);

/// Represents a component model `error-context`.
///
/// Note that this type is not usable at this time as its implementation has not
/// been filled out. There are no operations on this and there's additionally no
/// ability to "drop" or deallocate this index.
//
// FIXME(#11161) this needs to be filled out implementation-wise
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorContextAny(pub(crate) u32);
