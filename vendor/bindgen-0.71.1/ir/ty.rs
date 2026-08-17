//! Everything related to types in our intermediate representation.

use super::comp::CompInfo;
use super::context::{BindgenContext, ItemId, TypeId};
use super::dot::DotAttributes;
use super::enum_ty::Enum;
use super::function::FunctionSig;
use super::item::{IsOpaque, Item};
use super::layout::{Layout, Opaque};
use super::objc::ObjCInterface;
use super::template::{
    AsTemplateParam, TemplateInstantiation, TemplateParameters,
};
use super::traversal::{EdgeKind, Trace, Tracer};
use crate::clang::{self, Cursor};
use crate::parse::{ParseError, ParseResult};
use std::borrow::Cow;
use std::io;

pub use super::int::IntKind;

/// The base representation of a type in bindgen.
///
/// A type has an optional name, which if present cannot be empty, a `layout`
/// (size, alignment and packedness) if known, a `Kind`, which determines which
/// kind of type it is, and whether the type is const.
#[derive(Debug)]
pub(crate) struct Type {
    /// The name of the type, or None if it was an unnamed struct or union.
    name: Option<String>,
    /// The layout of the type, if known.
    layout: Option<Layout>,
    /// The inner kind of the type
    kind: TypeKind,
    /// Whether this type is const-qualified.
    is_const: bool,
}

/// The maximum number of items in an array for which Rust implements common
/// traits, and so if we have a type containing an array with more than this
/// many items, we won't be able to derive common traits on that type.
///
pub(crate) const RUST_DERIVE_IN_ARRAY_LIMIT: usize = 32;

impl Type {
    /// Get the underlying `CompInfo` for this type as a mutable reference, or
    /// `None` if this is some other kind of type.
    pub(crate) fn as_comp_mut(&mut self) -> Option<&mut CompInfo> {
        match self.kind {
            TypeKind::Comp(ref mut ci) => Some(ci),
            _ => None,
        }
    }

    /// Construct a new `Type`.
    pub(crate) fn new(
        name: Option<String>,
        layout: Option<Layout>,
        kind: TypeKind,
        is_const: bool,
    ) -> Self {
        Type {
            name,
            layout,
            kind,
            is_const,
        }
    }

    /// Which kind of type is this?
    pub(crate) fn kind(&self) -> &TypeKind {
        &self.kind
    }

    /// Get a mutable reference to this type's kind.
    pub(crate) fn kind_mut(&mut self) -> &mut TypeKind {
        &mut self.kind
    }

    /// Get this type's name.
    pub(crate) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Whether this is a block pointer type.
    pub(crate) fn is_block_pointer(&self) -> bool {
        matches!(self.kind, TypeKind::BlockPointer(..))
    }

    /// Is this an integer type, including `bool` or `char`?
    pub(crate) fn is_int(&self) -> bool {
        matches!(self.kind, TypeKind::Int(_))
    }

    /// Is this a compound type?
    pub(crate) fn is_comp(&self) -> bool {
        matches!(self.kind, TypeKind::Comp(..))
    }

    /// Is this a union?
    pub(crate) fn is_union(&self) -> bool {
        match self.kind {
            TypeKind::Comp(ref comp) => comp.is_union(),
            _ => false,
        }
    }

    /// Is this type of kind `TypeKind::TypeParam`?
    pub(crate) fn is_type_param(&self) -> bool {
        matches!(self.kind, TypeKind::TypeParam)
    }

    /// Is this a template instantiation type?
    pub(crate) fn is_template_instantiation(&self) -> bool {
        matches!(self.kind, TypeKind::TemplateInstantiation(..))
    }

    /// Is this a function type?
    pub(crate) fn is_function(&self) -> bool {
        matches!(self.kind, TypeKind::Function(..))
    }

    /// Is this an enum type?
    pub(crate) fn is_enum(&self) -> bool {
        matches!(self.kind, TypeKind::Enum(..))
    }

    /// Is this void?
    pub(crate) fn is_void(&self) -> bool {
        matches!(self.kind, TypeKind::Void)
    }
    /// Is this either a builtin or named type?
    pub(crate) fn is_builtin_or_type_param(&self) -> bool {
        matches!(
            self.kind,
            TypeKind::Void |
                TypeKind::NullPtr |
                TypeKind::Function(..) |
                TypeKind::Array(..) |
                TypeKind::Reference(..) |
                TypeKind::Pointer(..) |
                TypeKind::Int(..) |
                TypeKind::Float(..) |
                TypeKind::TypeParam
        )
    }

    /// Creates a new named type, with name `name`.
    pub(crate) fn named(name: String) -> Self {
        let name = if name.is_empty() { None } else { Some(name) };
        Self::new(name, None, TypeKind::TypeParam, false)
    }

    /// Is this a floating point type?
    pub(crate) fn is_float(&self) -> bool {
        matches!(self.kind, TypeKind::Float(..))
    }

    /// Is this a boolean type?
    pub(crate) fn is_bool(&self) -> bool {
        matches!(self.kind, TypeKind::Int(IntKind::Bool))
    }

    /// Is this an integer type?
    pub(crate) fn is_integer(&self) -> bool {
        matches!(self.kind, TypeKind::Int(..))
    }

    /// Cast this type to an integer kind, or `None` if it is not an integer
    /// type.
    pub(crate) fn as_integer(&self) -> Option<IntKind> {
        match self.kind {
            TypeKind::Int(int_kind) => Some(int_kind),
            _ => None,
        }
    }

    /// Is this a `const` qualified type?
    pub(crate) fn is_const(&self) -> bool {
        self.is_const
    }

    /// Is this an unresolved reference?
    pub(crate) fn is_unresolved_ref(&self) -> bool {
        matches!(self.kind, TypeKind::UnresolvedTypeRef(_, _, _))
    }

    /// Is this a incomplete array type?
    pub(crate) fn is_incomplete_array(
        &self,
        ctx: &BindgenContext,
    ) -> Option<ItemId> {
        match self.kind {
            TypeKind::Array(item, len) => {
                if len == 0 {
                    Some(item.into())
                } else {
                    None
                }
            }
            TypeKind::ResolvedTypeRef(inner) => {
                ctx.resolve_type(inner).is_incomplete_array(ctx)
            }
            _ => None,
        }
    }

    /// What is the layout of this type?
    pub(crate) fn layout(&self, ctx: &BindgenContext) -> Option<Layout> {
        self.layout.or_else(|| {
            match self.kind {
                TypeKind::Comp(ref ci) => ci.layout(ctx),
                TypeKind::Array(inner, 0) => Some(Layout::new(
                    0,
                    ctx.resolve_type(inner).layout(ctx)?.align,
                )),
                // FIXME(emilio): This is a hack for anonymous union templates.
                // Use the actual pointer size!
                TypeKind::Pointer(..) => Some(Layout::new(
                    ctx.target_pointer_size(),
                    ctx.target_pointer_size(),
                )),
                TypeKind::ResolvedTypeRef(inner) => {
                    ctx.resolve_type(inner).layout(ctx)
                }
                _ => None,
            }
        })
    }

    /// Whether this named type is an invalid C++ identifier. This is done to
    /// avoid generating invalid code with some cases we can't handle, see:
    ///
    /// tests/headers/381-decltype-alias.hpp
    pub(crate) fn is_invalid_type_param(&self) -> bool {
        match self.kind {
            TypeKind::TypeParam => {
                let name = self.name().expect("Unnamed named type?");
                !clang::is_valid_identifier(name)
            }
            _ => false,
        }
    }

    /// Takes `name`, and returns a suitable identifier representation for it.
    fn sanitize_name(name: &str) -> Cow<str> {
        if clang::is_valid_identifier(name) {
            return Cow::Borrowed(name);
        }

        let name = name.replace([' ', ':', '.'], "_");
        Cow::Owned(name)
    }

    /// Get this type's sanitized name.
    pub(crate) fn sanitized_name<'a>(
        &'a self,
        ctx: &BindgenContext,
    ) -> Option<Cow<'a, str>> {
        let name_info = match *self.kind() {
            TypeKind::Pointer(inner) => Some((inner, Cow::Borrowed("ptr"))),
            TypeKind::Reference(inner) => Some((inner, Cow::Borrowed("ref"))),
            TypeKind::Array(inner, length) => {
                Some((inner, format!("array{length}").into()))
            }
            _ => None,
        };
        if let Some((inner, prefix)) = name_info {
            ctx.resolve_item(inner)
                .expect_type()
                .sanitized_name(ctx)
                .map(|name| format!("{prefix}_{name}").into())
        } else {
            self.name().map(Self::sanitize_name)
        }
    }

    /// See [`Self::safe_canonical_type`].
    pub(crate) fn canonical_type<'tr>(
        &'tr self,
        ctx: &'tr BindgenContext,
    ) -> &'tr Type {
        self.safe_canonical_type(ctx)
            .expect("Should have been resolved after parsing!")
    }

    /// Returns the canonical type of this type, that is, the "inner type".
    ///
    /// For example, for a `typedef`, the canonical type would be the
    /// `typedef`ed type, for a template instantiation, would be the template
    /// its specializing, and so on. Return None if the type is unresolved.
    pub(crate) fn safe_canonical_type<'tr>(
        &'tr self,
        ctx: &'tr BindgenContext,
    ) -> Option<&'tr Type> {
        match self.kind {
            TypeKind::TypeParam |
            TypeKind::Array(..) |
            TypeKind::Vector(..) |
            TypeKind::Comp(..) |
            TypeKind::Opaque |
            TypeKind::Int(..) |
            TypeKind::Float(..) |
            TypeKind::Complex(..) |
            TypeKind::Function(..) |
            TypeKind::Enum(..) |
            TypeKind::Reference(..) |
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::Pointer(..) |
            TypeKind::BlockPointer(..) |
            TypeKind::ObjCId |
            TypeKind::ObjCSel |
            TypeKind::ObjCInterface(..) => Some(self),

            TypeKind::ResolvedTypeRef(inner) |
            TypeKind::Alias(inner) |
            TypeKind::TemplateAlias(inner, _) => {
                ctx.resolve_type(inner).safe_canonical_type(ctx)
            }
            TypeKind::TemplateInstantiation(ref inst) => ctx
                .resolve_type(inst.template_definition())
                .safe_canonical_type(ctx),

            TypeKind::UnresolvedTypeRef(..) => None,
        }
    }

    /// There are some types we don't want to stop at when finding an opaque
    /// item, so we can arrive to the proper item that needs to be generated.
    pub(crate) fn should_be_traced_unconditionally(&self) -> bool {
        matches!(
            self.kind,
            TypeKind::Comp(..) |
                TypeKind::Function(..) |
                TypeKind::Pointer(..) |
                TypeKind::Array(..) |
                TypeKind::Reference(..) |
                TypeKind::TemplateInstantiation(..) |
                TypeKind::ResolvedTypeRef(..)
        )
    }
}

impl IsOpaque for Type {
    type Extra = Item;

    fn is_opaque(&self, ctx: &BindgenContext, item: &Item) -> bool {
        match self.kind {
            TypeKind::Opaque => true,
            TypeKind::TemplateInstantiation(ref inst) => {
                inst.is_opaque(ctx, item)
            }
            TypeKind::Comp(ref comp) => comp.is_opaque(ctx, &self.layout),
            TypeKind::ResolvedTypeRef(to) => to.is_opaque(ctx, &()),
            _ => false,
        }
    }
}

impl AsTemplateParam for Type {
    type Extra = Item;

    fn as_template_param(
        &self,
        ctx: &BindgenContext,
        item: &Item,
    ) -> Option<TypeId> {
        self.kind.as_template_param(ctx, item)
    }
}

impl AsTemplateParam for TypeKind {
    type Extra = Item;

    fn as_template_param(
        &self,
        ctx: &BindgenContext,
        item: &Item,
    ) -> Option<TypeId> {
        match *self {
            TypeKind::TypeParam => Some(item.id().expect_type_id(ctx)),
            TypeKind::ResolvedTypeRef(id) => id.as_template_param(ctx, &()),
            _ => None,
        }
    }
}

impl DotAttributes for Type {
    fn dot_attributes<W>(
        &self,
        ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        if let Some(ref layout) = self.layout {
            writeln!(
                out,
                "<tr><td>size</td><td>{}</td></tr>
                           <tr><td>align</td><td>{}</td></tr>",
                layout.size, layout.align
            )?;
            if layout.packed {
                writeln!(out, "<tr><td>packed</td><td>true</td></tr>")?;
            }
        }

        if self.is_const {
            writeln!(out, "<tr><td>const</td><td>true</td></tr>")?;
        }

        self.kind.dot_attributes(ctx, out)
    }
}

impl DotAttributes for TypeKind {
    fn dot_attributes<W>(
        &self,
        ctx: &BindgenContext,
        out: &mut W,
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        writeln!(
            out,
            "<tr><td>type kind</td><td>{}</td></tr>",
            self.kind_name()
        )?;

        if let TypeKind::Comp(ref comp) = *self {
            comp.dot_attributes(ctx, out)?;
        }

        Ok(())
    }
}

impl TypeKind {
    fn kind_name(&self) -> &'static str {
        match *self {
            TypeKind::Void => "Void",
            TypeKind::NullPtr => "NullPtr",
            TypeKind::Comp(..) => "Comp",
            TypeKind::Opaque => "Opaque",
            TypeKind::Int(..) => "Int",
            TypeKind::Float(..) => "Float",
            TypeKind::Complex(..) => "Complex",
            TypeKind::Alias(..) => "Alias",
            TypeKind::TemplateAlias(..) => "TemplateAlias",
            TypeKind::Array(..) => "Array",
            TypeKind::Vector(..) => "Vector",
            TypeKind::Function(..) => "Function",
            TypeKind::Enum(..) => "Enum",
            TypeKind::Pointer(..) => "Pointer",
            TypeKind::BlockPointer(..) => "BlockPointer",
            TypeKind::Reference(..) => "Reference",
            TypeKind::TemplateInstantiation(..) => "TemplateInstantiation",
            TypeKind::UnresolvedTypeRef(..) => "UnresolvedTypeRef",
            TypeKind::ResolvedTypeRef(..) => "ResolvedTypeRef",
            TypeKind::TypeParam => "TypeParam",
            TypeKind::ObjCInterface(..) => "ObjCInterface",
            TypeKind::ObjCId => "ObjCId",
            TypeKind::ObjCSel => "ObjCSel",
        }
    }
}

#[test]
fn is_invalid_type_param_valid() {
    let ty = Type::new(Some("foo".into()), None, TypeKind::TypeParam, false);
    assert!(!ty.is_invalid_type_param());
}

#[test]
fn is_invalid_type_param_valid_underscore_and_numbers() {
    let ty = Type::new(
        Some("_foo123456789_".into()),
        None,
        TypeKind::TypeParam,
        false,
    );
    assert!(!ty.is_invalid_type_param());
}

#[test]
fn is_invalid_type_param_valid_unnamed_kind() {
    let ty = Type::new(Some("foo".into()), None, TypeKind::Void, false);
    assert!(!ty.is_invalid_type_param());
}

#[test]
fn is_invalid_type_param_invalid_start() {
    let ty = Type::new(Some("1foo".into()), None, TypeKind::TypeParam, false);
    assert!(ty.is_invalid_type_param());
}

#[test]
fn is_invalid_type_param_invalid_remaining() {
    let ty = Type::new(Some("foo-".into()), None, TypeKind::TypeParam, false);
    assert!(ty.is_invalid_type_param());
}

#[test]
#[should_panic]
fn is_invalid_type_param_unnamed() {
    let ty = Type::new(None, None, TypeKind::TypeParam, false);
    assert!(ty.is_invalid_type_param());
}

#[test]
fn is_invalid_type_param_empty_name() {
    let ty = Type::new(Some("".into()), None, TypeKind::TypeParam, false);
    assert!(ty.is_invalid_type_param());
}

impl TemplateParameters for Type {
    fn self_template_params(&self, ctx: &BindgenContext) -> Vec<TypeId> {
        self.kind.self_template_params(ctx)
    }
}

impl TemplateParameters for TypeKind {
    fn self_template_params(&self, ctx: &BindgenContext) -> Vec<TypeId> {
        match *self {
            TypeKind::ResolvedTypeRef(id) => {
                ctx.resolve_type(id).self_template_params(ctx)
            }
            TypeKind::Comp(ref comp) => comp.self_template_params(ctx),
            TypeKind::TemplateAlias(_, ref args) => args.clone(),

            TypeKind::Opaque |
            TypeKind::TemplateInstantiation(..) |
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::Int(_) |
            TypeKind::Float(_) |
            TypeKind::Complex(_) |
            TypeKind::Array(..) |
            TypeKind::Vector(..) |
            TypeKind::Function(_) |
            TypeKind::Enum(_) |
            TypeKind::Pointer(_) |
            TypeKind::BlockPointer(_) |
            TypeKind::Reference(_) |
            TypeKind::UnresolvedTypeRef(..) |
            TypeKind::TypeParam |
            TypeKind::Alias(_) |
            TypeKind::ObjCId |
            TypeKind::ObjCSel |
            TypeKind::ObjCInterface(_) => vec![],
        }
    }
}

/// The kind of float this type represents.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum FloatKind {
    /// A half (`_Float16` or `__fp16`)
    Float16,
    /// A `float`.
    Float,
    /// A `double`.
    Double,
    /// A `long double`.
    LongDouble,
    /// A `__float128`.
    Float128,
}

/// The different kinds of types that we can parse.
#[derive(Debug)]
pub(crate) enum TypeKind {
    /// The void type.
    Void,

    /// The `nullptr_t` type.
    NullPtr,

    /// A compound type, that is, a class, struct, or union.
    Comp(CompInfo),

    /// An opaque type that we just don't understand. All usage of this should
    /// result in an opaque blob of bytes generated from the containing type's
    /// layout.
    Opaque,

    /// An integer type, of a given kind. `bool` and `char` are also considered
    /// integers.
    Int(IntKind),

    /// A floating point type.
    Float(FloatKind),

    /// A complex floating point type.
    Complex(FloatKind),

    /// A type alias, with a name, that points to another type.
    Alias(TypeId),

    /// A templated alias, pointing to an inner type, just as `Alias`, but with
    /// template parameters.
    TemplateAlias(TypeId, Vec<TypeId>),

    /// A packed vector type: element type, number of elements
    Vector(TypeId, usize),

    /// An array of a type and a length.
    Array(TypeId, usize),

    /// A function type, with a given signature.
    Function(FunctionSig),

    /// An `enum` type.
    Enum(Enum),

    /// A pointer to a type. The bool field represents whether it's const or
    /// not.
    Pointer(TypeId),

    /// A pointer to an Apple block.
    BlockPointer(TypeId),

    /// A reference to a type, as in: int& `foo()`.
    Reference(TypeId),

    /// An instantiation of an abstract template definition with a set of
    /// concrete template arguments.
    TemplateInstantiation(TemplateInstantiation),

    /// A reference to a yet-to-resolve type. This stores the clang cursor
    /// itself, and postpones its resolution.
    ///
    /// These are gone in a phase after parsing where these are mapped to
    /// already known types, and are converted to `ResolvedTypeRef`.
    ///
    /// see tests/headers/typeref.hpp to see somewhere where this is a problem.
    UnresolvedTypeRef(clang::Type, Cursor, /* parent_id */ Option<ItemId>),

    /// An indirection to another type.
    ///
    /// These are generated after we resolve a forward declaration, or when we
    /// replace one type with another.
    ResolvedTypeRef(TypeId),

    /// A named type, that is, a template parameter.
    TypeParam,

    /// Objective C interface. Always referenced through a pointer
    ObjCInterface(ObjCInterface),

    /// Objective C 'id' type, points to any object
    ObjCId,

    /// Objective C selector type
    ObjCSel,
}

impl Type {
    /// This is another of the nasty methods. This one is the one that takes
    /// care of the core logic of converting a clang type to a `Type`.
    ///
    /// It's sort of nasty and full of special-casing, but hopefully the
    /// comments in every special case justify why they're there.
    pub(crate) fn from_clang_ty(
        potential_id: ItemId,
        ty: &clang::Type,
        location: Cursor,
        parent_id: Option<ItemId>,
        ctx: &mut BindgenContext,
    ) -> Result<ParseResult<Self>, ParseError> {
        use clang_sys::*;
        {
            let already_resolved = ctx.builtin_or_resolved_ty(
                potential_id,
                parent_id,
                ty,
                Some(location),
            );
            if let Some(ty) = already_resolved {
                debug!("{ty:?} already resolved: {location:?}");
                return Ok(ParseResult::AlreadyResolved(ty.into()));
            }
        }

        let layout = ty.fallible_layout(ctx).ok();
        let cursor = ty.declaration();
        let is_anonymous = cursor.is_anonymous();
        let mut name = if is_anonymous {
            None
        } else {
            Some(cursor.spelling()).filter(|n| !n.is_empty())
        };

        debug!(
            "from_clang_ty: {potential_id:?}, ty: {ty:?}, loc: {location:?}"
        );
        debug!("currently_parsed_types: {:?}", ctx.currently_parsed_types());

        let canonical_ty = ty.canonical_type();

        // Parse objc protocols as if they were interfaces
        let mut ty_kind = ty.kind();
        match location.kind() {
            CXCursor_ObjCProtocolDecl | CXCursor_ObjCCategoryDecl => {
                ty_kind = CXType_ObjCInterface;
            }
            _ => {}
        }

        // Objective C template type parameter
        // FIXME: This is probably wrong, we are attempting to find the
        //        objc template params, which seem to manifest as a typedef.
        //        We are rewriting them as ID to suppress multiple conflicting
        //        typedefs at root level
        if ty_kind == CXType_Typedef {
            let is_template_type_param =
                ty.declaration().kind() == CXCursor_TemplateTypeParameter;
            let is_canonical_objcpointer =
                canonical_ty.kind() == CXType_ObjCObjectPointer;

            // We have found a template type for objc interface
            if is_canonical_objcpointer && is_template_type_param {
                // Objective-C generics are just ids with fancy name.
                // To keep it simple, just name them ids
                name = Some("id".to_owned());
            }
        }

        if location.kind() == CXCursor_ClassTemplatePartialSpecialization {
            // Sorry! (Not sorry)
            warn!(
                "Found a partial template specialization; bindgen does not \
                 support partial template specialization! Constructing \
                 opaque type instead."
            );
            return Ok(ParseResult::New(
                Opaque::from_clang_ty(&canonical_ty, ctx),
                None,
            ));
        }

        let kind = if location.kind() == CXCursor_TemplateRef ||
            (ty.template_args().is_some() && ty_kind != CXType_Typedef)
        {
            // This is a template instantiation.
            match TemplateInstantiation::from_ty(ty, ctx) {
                Some(inst) => TypeKind::TemplateInstantiation(inst),
                None => TypeKind::Opaque,
            }
        } else {
            match ty_kind {
                CXType_Unexposed
                    if *ty != canonical_ty &&
                                    canonical_ty.kind() != CXType_Invalid &&
                                    ty.ret_type().is_none() &&
                                    // Sometime clang desugars some types more than
                                    // what we need, specially with function
                                    // pointers.
                                    //
                                    // We should also try the solution of inverting
                                    // those checks instead of doing this, that is,
                                    // something like:
                                    //
                                    // CXType_Unexposed if ty.ret_type().is_some()
                                    //   => { ... }
                                    //
                                    // etc.
                                    !canonical_ty.spelling().contains("type-parameter") =>
                {
                    debug!("Looking for canonical type: {canonical_ty:?}");
                    return Self::from_clang_ty(
                        potential_id,
                        &canonical_ty,
                        location,
                        parent_id,
                        ctx,
                    );
                }
                CXType_Unexposed | CXType_Invalid => {
                    // For some reason Clang doesn't give us any hint in some
                    // situations where we should generate a function pointer (see
                    // tests/headers/func_ptr_in_struct.h), so we do a guess here
                    // trying to see if it has a valid return type.
                    if ty.ret_type().is_some() {
                        let signature =
                            FunctionSig::from_ty(ty, &location, ctx)?;
                        TypeKind::Function(signature)
                    // Same here, with template specialisations we can safely
                    // assume this is a Comp(..)
                    } else if ty.is_fully_instantiated_template() {
                        debug!("Template specialization: {ty:?}, {location:?} {canonical_ty:?}");
                        let complex = CompInfo::from_ty(
                            potential_id,
                            ty,
                            Some(location),
                            ctx,
                        )
                        .expect("C'mon");
                        TypeKind::Comp(complex)
                    } else {
                        match location.kind() {
                            CXCursor_CXXBaseSpecifier |
                            CXCursor_ClassTemplate => {
                                if location.kind() == CXCursor_CXXBaseSpecifier
                                {
                                    // In the case we're parsing a base specifier
                                    // inside an unexposed or invalid type, it means
                                    // that we're parsing one of two things:
                                    //
                                    //  * A template parameter.
                                    //  * A complex class that isn't exposed.
                                    //
                                    // This means, unfortunately, that there's no
                                    // good way to differentiate between them.
                                    //
                                    // Probably we could try to look at the
                                    // declaration and complicate more this logic,
                                    // but we'll keep it simple... if it's a valid
                                    // C++ identifier, we'll consider it as a
                                    // template parameter.
                                    //
                                    // This is because:
                                    //
                                    //  * We expect every other base that is a
                                    //    proper identifier (that is, a simple
                                    //    struct/union declaration), to be exposed,
                                    //    so this path can't be reached in that
                                    //    case.
                                    //
                                    //  * Quite conveniently, complex base
                                    //    specifiers preserve their full names (that
                                    //    is: Foo<T> instead of Foo). We can take
                                    //    advantage of this.
                                    //
                                    // If we find some edge case where this doesn't
                                    // work (which I guess is unlikely, see the
                                    // different test cases[1][2][3][4]), we'd need
                                    // to find more creative ways of differentiating
                                    // these two cases.
                                    //
                                    // [1]: inherit_named.hpp
                                    // [2]: forward-inherit-struct-with-fields.hpp
                                    // [3]: forward-inherit-struct.hpp
                                    // [4]: inherit-namespaced.hpp
                                    if location.spelling().chars().all(|c| {
                                        c.is_alphanumeric() || c == '_'
                                    }) {
                                        return Err(ParseError::Recurse);
                                    }
                                } else {
                                    name = Some(location.spelling());
                                }

                                let complex = CompInfo::from_ty(
                                    potential_id,
                                    ty,
                                    Some(location),
                                    ctx,
                                );
                                if let Ok(complex) = complex {
                                    TypeKind::Comp(complex)
                                } else {
                                    warn!(
                                        "Could not create complex type \
                                         from class template or base \
                                         specifier, using opaque blob"
                                    );
                                    let opaque = Opaque::from_clang_ty(ty, ctx);
                                    return Ok(ParseResult::New(opaque, None));
                                }
                            }
                            CXCursor_TypeAliasTemplateDecl => {
                                debug!("TypeAliasTemplateDecl");

                                // We need to manually unwind this one.
                                let mut inner = Err(ParseError::Continue);
                                let mut args = vec![];

                                location.visit(|cur| {
                                    match cur.kind() {
                                        CXCursor_TypeAliasDecl => {
                                            let current = cur.cur_type();

                                            debug_assert_eq!(
                                                current.kind(),
                                                CXType_Typedef
                                            );

                                            name = Some(location.spelling());

                                            let inner_ty = cur
                                                .typedef_type()
                                                .expect("Not valid Type?");
                                            inner = Ok(Item::from_ty_or_ref(
                                                inner_ty,
                                                cur,
                                                Some(potential_id),
                                                ctx,
                                            ));
                                        }
                                        CXCursor_TemplateTypeParameter => {
                                            let param = Item::type_param(
                                                None, cur, ctx,
                                            )
                                            .expect(
                                                "Item::type_param shouldn't \
                                                 ever fail if we are looking \
                                                 at a TemplateTypeParameter",
                                            );
                                            args.push(param);
                                        }
                                        _ => {}
                                    }
                                    CXChildVisit_Continue
                                });

                                let Ok(inner_type) = inner else {
                                    warn!(
                                        "Failed to parse template alias \
                                             {:?}",
                                        location
                                    );
                                    return Err(ParseError::Continue);
                                };

                                TypeKind::TemplateAlias(inner_type, args)
                            }
                            CXCursor_TemplateRef => {
                                let referenced = location.referenced().unwrap();
                                let referenced_ty = referenced.cur_type();

                                debug!("TemplateRef: location = {location:?}; referenced = {referenced:?}; referenced_ty = {referenced_ty:?}");

                                return Self::from_clang_ty(
                                    potential_id,
                                    &referenced_ty,
                                    referenced,
                                    parent_id,
                                    ctx,
                                );
                            }
                            CXCursor_TypeRef => {
                                let referenced = location.referenced().unwrap();
                                let referenced_ty = referenced.cur_type();
                                let declaration = referenced_ty.declaration();

                                debug!("TypeRef: location = {location:?}; referenced = {referenced:?}; referenced_ty = {referenced_ty:?}");

                                let id = Item::from_ty_or_ref_with_id(
                                    potential_id,
                                    referenced_ty,
                                    declaration,
                                    parent_id,
                                    ctx,
                                );
                                return Ok(ParseResult::AlreadyResolved(
                                    id.into(),
                                ));
                            }
                            CXCursor_NamespaceRef => {
                                return Err(ParseError::Continue);
                            }
                            _ => {
                                if ty.kind() == CXType_Unexposed {
                                    warn!("Unexposed type {ty:?}, recursing inside, loc: {location:?}");
                                    return Err(ParseError::Recurse);
                                }

                                warn!("invalid type {ty:?}");
                                return Err(ParseError::Continue);
                            }
                        }
                    }
                }
                CXType_Auto => {
                    if canonical_ty == *ty {
                        debug!("Couldn't find deduced type: {ty:?}");
                        return Err(ParseError::Continue);
                    }

                    return Self::from_clang_ty(
                        potential_id,
                        &canonical_ty,
                        location,
                        parent_id,
                        ctx,
                    );
                }
                // NOTE: We don't resolve pointers eagerly because the pointee type
                // might not have been parsed, and if it contains templates or
                // something else we might get confused, see the comment inside
                // TypeRef.
                //
                // We might need to, though, if the context is already in the
                // process of resolving them.
                CXType_ObjCObjectPointer |
                CXType_MemberPointer |
                CXType_Pointer => {
                    let mut pointee = ty.pointee_type().unwrap();
                    if *ty != canonical_ty {
                        let canonical_pointee =
                            canonical_ty.pointee_type().unwrap();
                        // clang sometimes loses pointee constness here, see
                        // #2244.
                        if canonical_pointee.is_const() != pointee.is_const() {
                            pointee = canonical_pointee;
                        }
                    }
                    let inner =
                        Item::from_ty_or_ref(pointee, location, None, ctx);
                    TypeKind::Pointer(inner)
                }
                CXType_BlockPointer => {
                    let pointee = ty.pointee_type().expect("Not valid Type?");
                    let inner =
                        Item::from_ty_or_ref(pointee, location, None, ctx);
                    TypeKind::BlockPointer(inner)
                }
                // XXX: RValueReference is most likely wrong, but I don't think we
                // can even add bindings for that, so huh.
                CXType_RValueReference | CXType_LValueReference => {
                    let inner = Item::from_ty_or_ref(
                        ty.pointee_type().unwrap(),
                        location,
                        None,
                        ctx,
                    );
                    TypeKind::Reference(inner)
                }
                // XXX DependentSizedArray is wrong
                CXType_VariableArray | CXType_DependentSizedArray => {
                    let inner = Item::from_ty(
                        ty.elem_type().as_ref().unwrap(),
                        location,
                        None,
                        ctx,
                    )
                    .expect("Not able to resolve array element?");
                    TypeKind::Pointer(inner)
                }
                CXType_IncompleteArray => {
                    let inner = Item::from_ty(
                        ty.elem_type().as_ref().unwrap(),
                        location,
                        None,
                        ctx,
                    )
                    .expect("Not able to resolve array element?");
                    TypeKind::Array(inner, 0)
                }
                CXType_FunctionNoProto | CXType_FunctionProto => {
                    let signature = FunctionSig::from_ty(ty, &location, ctx)?;
                    TypeKind::Function(signature)
                }
                CXType_Typedef => {
                    let inner = cursor.typedef_type().expect("Not valid Type?");
                    let inner_id =
                        Item::from_ty_or_ref(inner, location, None, ctx);
                    if inner_id == potential_id {
                        warn!(
                            "Generating opaque type instead of self-referential \
                            typedef");
                        // This can happen if we bail out of recursive situations
                        // within the clang parsing.
                        TypeKind::Opaque
                    } else {
                        // Check if this type definition is an alias to a pointer of a `struct` /
                        // `union` / `enum` with the same name and add the `_ptr` suffix to it to
                        // avoid name collisions.
                        if let Some(ref mut name) = name {
                            if inner.kind() == CXType_Pointer &&
                                !ctx.options().c_naming
                            {
                                let pointee = inner.pointee_type().unwrap();
                                if pointee.kind() == CXType_Elaborated &&
                                    pointee.declaration().spelling() == *name
                                {
                                    *name += "_ptr";
                                }
                            }
                        }
                        TypeKind::Alias(inner_id)
                    }
                }
                CXType_Enum => {
                    let enum_ = Enum::from_ty(ty, ctx).expect("Not an enum?");

                    if !is_anonymous {
                        let pretty_name = ty.spelling();
                        if clang::is_valid_identifier(&pretty_name) {
                            name = Some(pretty_name);
                        }
                    }

                    TypeKind::Enum(enum_)
                }
                CXType_Record => {
                    let complex = CompInfo::from_ty(
                        potential_id,
                        ty,
                        Some(location),
                        ctx,
                    )
                    .expect("Not a complex type?");

                    if !is_anonymous {
                        // The pretty-printed name may contain typedefed name,
                        // but may also be "struct (anonymous at .h:1)"
                        let pretty_name = ty.spelling();
                        if clang::is_valid_identifier(&pretty_name) {
                            name = Some(pretty_name);
                        }
                    }

                    TypeKind::Comp(complex)
                }
                CXType_Vector => {
                    let inner = Item::from_ty(
                        ty.elem_type().as_ref().unwrap(),
                        location,
                        None,
                        ctx,
                    )?;
                    TypeKind::Vector(inner, ty.num_elements().unwrap())
                }
                CXType_ConstantArray => {
                    let inner = Item::from_ty(
                        ty.elem_type().as_ref().unwrap(),
                        location,
                        None,
                        ctx,
                    )
                    .expect("Not able to resolve array element?");
                    TypeKind::Array(inner, ty.num_elements().unwrap())
                }
                CXType_Atomic => {
                    // TODO(emilio): Maybe we can preserve the "is atomic" bit somehow and generate
                    // something more useful... But for now this is better than panicking or
                    // generating nothing.
                    return Self::from_clang_ty(
                        potential_id,
                        &ty.atomic_value_type(),
                        location,
                        parent_id,
                        ctx,
                    );
                }
                CXType_Elaborated => {
                    return Self::from_clang_ty(
                        potential_id,
                        &ty.named(),
                        location,
                        parent_id,
                        ctx,
                    );
                }
                CXType_ObjCId => TypeKind::ObjCId,
                CXType_ObjCSel => TypeKind::ObjCSel,
                CXType_ObjCClass | CXType_ObjCInterface => {
                    let interface = ObjCInterface::from_ty(&location, ctx)
                        .expect("Not a valid objc interface?");
                    if !is_anonymous {
                        name = Some(interface.rust_name());
                    }
                    TypeKind::ObjCInterface(interface)
                }
                CXType_Dependent => {
                    return Err(ParseError::Continue);
                }
                _ => {
                    warn!(
                        "unsupported type: kind = {:?}; ty = {ty:?}; at {location:?}",
                        ty.kind(),
                    );
                    return Err(ParseError::Continue);
                }
            }
        };

        name = name.filter(|n| !n.is_empty());

        let is_const = ty.is_const() ||
            (ty.kind() == CXType_ConstantArray &&
                ty.elem_type().is_some_and(|element| element.is_const()));

        let ty = Type::new(name, layout, kind, is_const);
        // TODO: maybe declaration.canonical()?
        Ok(ParseResult::New(ty, Some(cursor.canonical())))
    }
}

impl Trace for Type {
    type Extra = Item;

    fn trace<T>(&self, context: &BindgenContext, tracer: &mut T, item: &Item)
    where
        T: Tracer,
    {
        if self.name().is_some_and(|name| context.is_stdint_type(name)) {
            // These types are special-cased in codegen and don't need to be traversed.
            return;
        }
        match *self.kind() {
            TypeKind::Pointer(inner) |
            TypeKind::Reference(inner) |
            TypeKind::Array(inner, _) |
            TypeKind::Vector(inner, _) |
            TypeKind::BlockPointer(inner) |
            TypeKind::Alias(inner) |
            TypeKind::ResolvedTypeRef(inner) => {
                tracer.visit_kind(inner.into(), EdgeKind::TypeReference);
            }
            TypeKind::TemplateAlias(inner, ref template_params) => {
                tracer.visit_kind(inner.into(), EdgeKind::TypeReference);
                for param in template_params {
                    tracer.visit_kind(
                        param.into(),
                        EdgeKind::TemplateParameterDefinition,
                    );
                }
            }
            TypeKind::TemplateInstantiation(ref inst) => {
                inst.trace(context, tracer, &());
            }
            TypeKind::Comp(ref ci) => ci.trace(context, tracer, item),
            TypeKind::Function(ref sig) => sig.trace(context, tracer, &()),
            TypeKind::Enum(ref en) => {
                if let Some(repr) = en.repr() {
                    tracer.visit(repr.into());
                }
            }
            TypeKind::UnresolvedTypeRef(_, _, Some(id)) => {
                tracer.visit(id);
            }

            TypeKind::ObjCInterface(ref interface) => {
                interface.trace(context, tracer, &());
            }

            // None of these variants have edges to other items and types.
            TypeKind::Opaque |
            TypeKind::UnresolvedTypeRef(_, _, None) |
            TypeKind::TypeParam |
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::Int(_) |
            TypeKind::Float(_) |
            TypeKind::Complex(_) |
            TypeKind::ObjCId |
            TypeKind::ObjCSel => {}
        }
    }
}
