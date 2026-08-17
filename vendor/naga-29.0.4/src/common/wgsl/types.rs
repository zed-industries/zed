//! Code for formatting Naga IR types as WGSL source code.

use super::{address_space_str, ToWgsl, TryToWgsl};
use crate::common;
use crate::proc::TypeResolution;
use crate::{Handle, Scalar, TypeInner};

use alloc::string::String;
use core::fmt::Write;

/// A context for printing Naga IR types as WGSL.
///
/// This trait's default methods [`write_type`] and
/// [`write_type_inner`] do the work of formatting types as WGSL.
/// Implementors must provide the remaining methods, to customize
/// behavior for the context at hand.
///
/// For example, the WGSL backend would provide an implementation of
/// [`type_name`] that handles hygienic renaming, whereas the WGSL
/// front end would simply show the name that was given in the source.
///
/// [`write_type`]: TypeContext::write_type
/// [`write_type_inner`]: TypeContext::write_type_inner
/// [`type_name`]: TypeContext::type_name
pub trait TypeContext {
    /// Return the [`Type`] referred to by `handle`.
    ///
    /// [`Type`]: crate::Type
    fn lookup_type(&self, handle: Handle<crate::Type>) -> &crate::Type;

    /// Return the name to be used for the type referred to by
    /// `handle`.
    fn type_name(&self, handle: Handle<crate::Type>) -> &str;

    /// Write the WGSL form of `override` to `out`.
    fn write_override<W: Write>(
        &self,
        r#override: Handle<crate::Override>,
        out: &mut W,
    ) -> core::fmt::Result;

    /// Write a [`TypeInner::Struct`] for which we are unable to find a name.
    ///
    /// The names of struct types are only available if we have `Handle<Type>`,
    /// not from [`TypeInner`]. For logging and debugging, it's fine to just
    /// write something helpful to the developer, but for generating WGSL,
    /// this should be unreachable.
    fn write_unnamed_struct<W: Write>(&self, inner: &TypeInner, out: &mut W) -> core::fmt::Result;

    /// Write a [`TypeInner`] that has no representation as WGSL source,
    /// even including Naga extensions.
    ///
    /// A backend might implement this with a call to the [`unreachable!`]
    /// macro, since backends are allowed to assume that the module has passed
    /// validation.
    ///
    /// The default implementation is appropriate for generating type names to
    /// appear in error messages. It punts to `TypeInner`'s [`core::fmt::Debug`]
    /// implementation, since it's probably best to show the user something they
    /// can act on.
    fn write_non_wgsl_inner<W: Write>(&self, inner: &TypeInner, out: &mut W) -> core::fmt::Result {
        write!(out, "{{non-WGSL Naga type {inner:?}}}")
    }

    /// Write a [`Scalar`] that has no representation as WGSL source,
    /// even including Naga extensions.
    ///
    /// A backend might implement this with a call to the [`unreachable!`]
    /// macro, since backends are allowed to assume that the module has passed
    /// validation.
    ///
    /// The default implementation is appropriate for generating type names to
    /// appear in error messages. It punts to `Scalar`'s [`core::fmt::Debug`]
    /// implementation, since it's probably best to show the user something they
    /// can act on.
    fn write_non_wgsl_scalar<W: Write>(&self, scalar: Scalar, out: &mut W) -> core::fmt::Result {
        match scalar.kind {
            crate::ScalarKind::Sint
            | crate::ScalarKind::Uint
            | crate::ScalarKind::Float
            | crate::ScalarKind::Bool => write!(out, "{{non-WGSL Naga scalar {scalar:?}}}"),

            // The abstract types are kind of an odd quasi-WGSL category:
            // they are definitely part of the spec, but they are not expressible
            // in WGSL itself. So we want to call them out by name in error messages,
            // but the WGSL backend should never generate these.
            crate::ScalarKind::AbstractInt => out.write_str("{AbstractInt}"),
            crate::ScalarKind::AbstractFloat => out.write_str("{AbstractFloat}"),
        }
    }

    /// Write the type `ty` as it would appear in a value's declaration.
    ///
    /// Write the type referred to by `ty` in `module` as it would appear in
    /// a `var`, `let`, etc. declaration, or in a function's argument list.
    fn write_type<W: Write>(&self, handle: Handle<crate::Type>, out: &mut W) -> core::fmt::Result {
        let ty = self.lookup_type(handle);
        match ty.inner {
            TypeInner::Struct { .. } => out.write_str(self.type_name(handle))?,
            ref other => self.write_type_inner(other, out)?,
        }

        Ok(())
    }

    /// Write the [`TypeInner`] `inner` as it would appear in a value's declaration.
    ///
    /// Write `inner` as it would appear in a `var`, `let`, etc.
    /// declaration, or in a function's argument list.
    ///
    /// Note that this cannot handle writing [`Struct`] types: those
    /// must be referred to by name, but the name isn't available in
    /// [`TypeInner`].
    ///
    /// [`Struct`]: TypeInner::Struct
    fn write_type_inner<W: Write>(&self, inner: &TypeInner, out: &mut W) -> core::fmt::Result {
        match try_write_type_inner(self, inner, out) {
            Ok(()) => Ok(()),
            Err(WriteTypeError::Format(err)) => Err(err),
            Err(WriteTypeError::NonWgsl) => self.write_non_wgsl_inner(inner, out),
        }
    }

    /// Write the [`Scalar`] `scalar` as a WGSL type.
    fn write_scalar<W: Write>(&self, scalar: Scalar, out: &mut W) -> core::fmt::Result {
        match scalar.try_to_wgsl() {
            Some(string) => out.write_str(string),
            None => self.write_non_wgsl_scalar(scalar, out),
        }
    }

    /// Write the [`TypeResolution`] `resolution` as a WGSL type.
    fn write_type_resolution<W: Write>(
        &self,
        resolution: &TypeResolution,
        out: &mut W,
    ) -> core::fmt::Result {
        match *resolution {
            TypeResolution::Handle(handle) => self.write_type(handle, out),
            TypeResolution::Value(ref inner) => self.write_type_inner(inner, out),
        }
    }

    fn write_type_conclusion<W: Write>(
        &self,
        conclusion: &crate::proc::Conclusion,
        out: &mut W,
    ) -> core::fmt::Result {
        use crate::proc::Conclusion as Co;

        match *conclusion {
            Co::Value(ref inner) => self.write_type_inner(inner, out),
            Co::Predeclared(ref predeclared) => out.write_str(&predeclared.struct_name()),
        }
    }

    fn write_type_rule<W: Write>(
        &self,
        name: &str,
        rule: &crate::proc::Rule,
        out: &mut W,
    ) -> core::fmt::Result {
        write!(out, "fn {name}(")?;
        for (i, arg) in rule.arguments.iter().enumerate() {
            if i > 0 {
                out.write_str(", ")?;
            }
            self.write_type_resolution(arg, out)?
        }
        out.write_str(") -> ")?;
        self.write_type_conclusion(&rule.conclusion, out)?;
        Ok(())
    }

    fn type_to_string(&self, handle: Handle<crate::Type>) -> String {
        let mut buf = String::new();
        self.write_type(handle, &mut buf).unwrap();
        buf
    }

    fn type_resolution_to_string(&self, resolution: &TypeResolution) -> String {
        let mut buf = String::new();
        self.write_type_resolution(resolution, &mut buf).unwrap();
        buf
    }

    fn type_rule_to_string(&self, name: &str, rule: &crate::proc::Rule) -> String {
        let mut buf = String::new();
        self.write_type_rule(name, rule, &mut buf).unwrap();
        buf
    }
}

fn try_write_type_inner<C, W>(ctx: &C, inner: &TypeInner, out: &mut W) -> Result<(), WriteTypeError>
where
    C: TypeContext + ?Sized,
    W: Write,
{
    match *inner {
        TypeInner::Vector { size, scalar } => {
            write!(out, "vec{}<", common::vector_size_str(size))?;
            ctx.write_scalar(scalar, out)?;
            out.write_str(">")?;
        }
        TypeInner::Sampler { comparison: false } => {
            write!(out, "sampler")?;
        }
        TypeInner::Sampler { comparison: true } => {
            write!(out, "sampler_comparison")?;
        }
        TypeInner::Image {
            dim,
            arrayed,
            class,
        } => {
            // More about texture types: https://gpuweb.github.io/gpuweb/wgsl/#sampled-texture-type
            use crate::ImageClass as Ic;

            let dim_str = dim.to_wgsl();
            let arrayed_str = if arrayed { "_array" } else { "" };
            match class {
                Ic::Sampled { kind, multi } => {
                    let multisampled_str = if multi { "multisampled_" } else { "" };
                    write!(out, "texture_{multisampled_str}{dim_str}{arrayed_str}<")?;
                    ctx.write_scalar(Scalar { kind, width: 4 }, out)?;
                    out.write_str(">")?;
                }
                Ic::Depth { multi } => {
                    let multisampled_str = if multi { "multisampled_" } else { "" };
                    write!(
                        out,
                        "texture_depth_{multisampled_str}{dim_str}{arrayed_str}"
                    )?;
                }
                Ic::Storage { format, access } => {
                    let format_str = format.to_wgsl();
                    let access_str = if access.contains(crate::StorageAccess::ATOMIC) {
                        ",atomic"
                    } else if access
                        .contains(crate::StorageAccess::LOAD | crate::StorageAccess::STORE)
                    {
                        ",read_write"
                    } else if access.contains(crate::StorageAccess::LOAD) {
                        ",read"
                    } else {
                        ",write"
                    };
                    write!(
                        out,
                        "texture_storage_{dim_str}{arrayed_str}<{format_str}{access_str}>"
                    )?;
                }
                Ic::External => {
                    write!(out, "texture_external")?;
                }
            }
        }
        TypeInner::Scalar(scalar) => {
            ctx.write_scalar(scalar, out)?;
        }
        TypeInner::Atomic(scalar) => {
            out.write_str("atomic<")?;
            ctx.write_scalar(scalar, out)?;
            out.write_str(">")?;
        }
        TypeInner::Array {
            base,
            size,
            stride: _,
        } => {
            // More info https://gpuweb.github.io/gpuweb/wgsl/#array-types
            // array<A, 3> -- Constant array
            // array<A> -- Dynamic array
            write!(out, "array<")?;
            match size {
                crate::ArraySize::Constant(len) => {
                    ctx.write_type(base, out)?;
                    write!(out, ", {len}")?;
                }
                crate::ArraySize::Pending(r#override) => {
                    ctx.write_override(r#override, out)?;
                }
                crate::ArraySize::Dynamic => {
                    ctx.write_type(base, out)?;
                }
            }
            write!(out, ">")?;
        }
        TypeInner::BindingArray { base, size } => {
            // More info https://github.com/gpuweb/gpuweb/issues/2105
            write!(out, "binding_array<")?;
            match size {
                crate::ArraySize::Constant(len) => {
                    ctx.write_type(base, out)?;
                    write!(out, ", {len}")?;
                }
                crate::ArraySize::Pending(r#override) => {
                    ctx.write_override(r#override, out)?;
                }
                crate::ArraySize::Dynamic => {
                    ctx.write_type(base, out)?;
                }
            }
            write!(out, ">")?;
        }
        TypeInner::Matrix {
            columns,
            rows,
            scalar,
        } => {
            write!(
                out,
                "mat{}x{}<",
                common::vector_size_str(columns),
                common::vector_size_str(rows),
            )?;
            ctx.write_scalar(scalar, out)?;
            out.write_str(">")?;
        }
        TypeInner::CooperativeMatrix {
            columns,
            rows,
            scalar,
            role,
        } => {
            write!(
                out,
                "coop_mat{}x{}<{},{}>",
                columns as u32,
                rows as u32,
                scalar.try_to_wgsl().unwrap_or_default(),
                role.to_wgsl(),
            )?;
        }
        TypeInner::Pointer { base, space } => {
            let (address, maybe_access) = address_space_str(space);
            // Everything but `AddressSpace::Handle` gives us a `address` name, but
            // Naga IR never produces pointers to handles, so it doesn't matter much
            // how we write such a type. Just write it as the base type alone.
            if let Some(space) = address {
                write!(out, "ptr<{space}, ")?;
            }
            ctx.write_type(base, out)?;
            if address.is_some() {
                if let Some(access) = maybe_access {
                    write!(out, ", {access}")?;
                }
                write!(out, ">")?;
            }
        }
        TypeInner::ValuePointer {
            size: None,
            scalar,
            space,
        } => {
            let (address, maybe_access) = address_space_str(space);
            if let Some(space) = address {
                write!(out, "ptr<{space}, ")?;
                ctx.write_scalar(scalar, out)?;
                if let Some(access) = maybe_access {
                    write!(out, ", {access}")?;
                }
                write!(out, ">")?;
            } else {
                return Err(WriteTypeError::NonWgsl);
            }
        }
        TypeInner::ValuePointer {
            size: Some(size),
            scalar,
            space,
        } => {
            let (address, maybe_access) = address_space_str(space);
            if let Some(space) = address {
                write!(out, "ptr<{}, vec{}<", space, common::vector_size_str(size),)?;
                ctx.write_scalar(scalar, out)?;
                out.write_str(">")?;
                if let Some(access) = maybe_access {
                    write!(out, ", {access}")?;
                }
                write!(out, ">")?;
            } else {
                return Err(WriteTypeError::NonWgsl);
            }
            write!(out, ">")?;
        }
        TypeInner::AccelerationStructure { vertex_return } => {
            let caps = if vertex_return { "<vertex_return>" } else { "" };
            write!(out, "acceleration_structure{caps}")?
        }
        TypeInner::Struct { .. } => {
            ctx.write_unnamed_struct(inner, out)?;
        }
        TypeInner::RayQuery { vertex_return } => {
            let caps = if vertex_return { "<vertex_return>" } else { "" };
            write!(out, "ray_query{caps}")?
        }
    }

    Ok(())
}

/// Error type returned by `try_write_type_inner`.
///
/// This type is private to the module.
enum WriteTypeError {
    Format(core::fmt::Error),
    NonWgsl,
}

impl From<core::fmt::Error> for WriteTypeError {
    fn from(err: core::fmt::Error) -> Self {
        Self::Format(err)
    }
}

/// Format types as WGSL based on a [`GlobalCtx`].
///
/// This is probably good enough for diagnostic output, but it has some
/// limitations:
///
/// - It does not apply [`Namer`] renamings, to avoid collisions.
///
/// - It generates invalid WGSL for anonymous struct types.
///
/// - It doesn't write the lengths of override-expression-sized arrays
///   correctly, unless the expression is just the override identifier.
///
/// [`GlobalCtx`]: crate::proc::GlobalCtx
/// [`Namer`]: crate::proc::Namer
impl TypeContext for crate::proc::GlobalCtx<'_> {
    fn lookup_type(&self, handle: Handle<crate::Type>) -> &crate::Type {
        &self.types[handle]
    }

    fn type_name(&self, handle: Handle<crate::Type>) -> &str {
        self.types[handle]
            .name
            .as_deref()
            .unwrap_or("{anonymous type}")
    }

    fn write_unnamed_struct<W: Write>(&self, _: &TypeInner, out: &mut W) -> core::fmt::Result {
        write!(out, "{{unnamed struct}}")
    }

    fn write_override<W: Write>(
        &self,
        handle: Handle<crate::Override>,
        out: &mut W,
    ) -> core::fmt::Result {
        match self.overrides[handle].name {
            Some(ref name) => out.write_str(name),
            None => write!(out, "{{anonymous override {handle:?}}}"),
        }
    }
}

/// Format types as WGSL based on a `UniqueArena<Type>`.
///
/// This is probably only good enough for logging:
///
/// - It does not apply any kind of [`Namer`] renamings.
///
/// - It generates invalid WGSL for anonymous struct types.
///
/// - It doesn't write override-sized arrays properly.
///
/// [`Namer`]: crate::proc::Namer
impl TypeContext for crate::UniqueArena<crate::Type> {
    fn lookup_type(&self, handle: Handle<crate::Type>) -> &crate::Type {
        &self[handle]
    }

    fn type_name(&self, handle: Handle<crate::Type>) -> &str {
        self[handle].name.as_deref().unwrap_or("{anonymous type}")
    }

    fn write_unnamed_struct<W: Write>(&self, inner: &TypeInner, out: &mut W) -> core::fmt::Result {
        write!(out, "{{unnamed struct {inner:?}}}")
    }

    fn write_override<W: Write>(
        &self,
        handle: Handle<crate::Override>,
        out: &mut W,
    ) -> core::fmt::Result {
        write!(out, "{{override {handle:?}}}")
    }
}
