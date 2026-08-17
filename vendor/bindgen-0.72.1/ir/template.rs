//! Template declaration and instantiation related things.
//!
//! The nomenclature surrounding templates is often confusing, so here are a few
//! brief definitions:
//!
//! * "Template definition": a class/struct/alias/function definition that takes
//!   generic template parameters. For example:
//!
//! ```c++
//! template<typename T>
//! class List<T> {
//!     // ...
//! };
//! ```
//!
//! * "Template instantiation": an instantiation is a use of a template with
//!   concrete template arguments. For example, `List<int>`.
//!
//! * "Template specialization": an alternative template definition providing a
//!   custom definition for instantiations with the matching template
//!   arguments. This C++ feature is unsupported by bindgen. For example:
//!
//! ```c++
//! template<>
//! class List<int> {
//!     // Special layout for int lists...
//! };
//! ```

use super::context::{BindgenContext, ItemId, TypeId};
use super::item::{IsOpaque, Item, ItemAncestors};
use super::traversal::{EdgeKind, Trace, Tracer};
use crate::clang;

/// Template declaration (and such declaration's template parameters) related
/// methods.
///
/// This trait's methods distinguish between `None` and `Some([])` for
/// declarations that are not templates and template declarations with zero
/// parameters, in general.
///
/// Consider this example:
///
/// ```c++
/// template <typename T, typename U>
/// class Foo {
///     T use_of_t;
///     U use_of_u;
///
///     template <typename V>
///     using Bar = V*;
///
///     class Inner {
///         T        x;
///         U        y;
///         Bar<int> z;
///     };
///
///     template <typename W>
///     class Lol {
///         // No use of W, but here's a use of T.
///         T t;
///     };
///
///     template <typename X>
///     class Wtf {
///         // X is not used because W is not used.
///         Lol<X> lololol;
///     };
/// };
///
/// class Qux {
///     int y;
/// };
/// ```
///
/// The following table depicts the results of each trait method when invoked on
/// each of the declarations above:
///
/// |Decl. | self_template_params | num_self_template_params | all_template_parameters |
/// |------|----------------------|--------------------------|-------------------------|
/// |Foo   | T, U                 | 2                        | T, U                    |
/// |Bar   | V                    | 1                        | T, U, V                 |
/// |Inner |                      | 0                        | T, U                    |
/// |Lol   | W                    | 1                        | T, U, W                 |
/// |Wtf   | X                    | 1                        | T, U, X                 |
/// |Qux   |                      | 0                        |                         |
///
/// | Decl. | used_template_params |
/// |-------|----------------------|
/// | Foo   | T, U                 |
/// | Bar   | V                    |
/// | Inner |                      |
/// | Lol   | T                    |
/// | Wtf   | T                    |
/// | Qux   |                      |
pub(crate) trait TemplateParameters: Sized {
    /// Get the set of `ItemId`s that make up this template declaration's free
    /// template parameters.
    ///
    /// Note that these might *not* all be named types: C++ allows
    /// constant-value template parameters as well as template-template
    /// parameters. Of course, Rust does not allow generic parameters to be
    /// anything but types, so we must treat them as opaque, and avoid
    /// instantiating them.
    fn self_template_params(&self, ctx: &BindgenContext) -> Vec<TypeId>;

    /// Get the number of free template parameters this template declaration
    /// has.
    fn num_self_template_params(&self, ctx: &BindgenContext) -> usize {
        self.self_template_params(ctx).len()
    }

    /// Get the complete set of template parameters that can affect this
    /// declaration.
    ///
    /// Note that this item doesn't need to be a template declaration itself for
    /// `Some` to be returned here (in contrast to `self_template_params`). If
    /// this item is a member of a template declaration, then the parent's
    /// template parameters are included here.
    ///
    /// In the example above, `Inner` depends on both of the `T` and `U` type
    /// parameters, even though it is not itself a template declaration and
    /// therefore has no type parameters itself. Perhaps it helps to think about
    /// how we would fully reference such a member type in C++:
    /// `Foo<int,char>::Inner`. `Foo` *must* be instantiated with template
    /// arguments before we can gain access to the `Inner` member type.
    fn all_template_params(&self, ctx: &BindgenContext) -> Vec<TypeId>
    where
        Self: ItemAncestors,
    {
        let mut ancestors: Vec<_> = self.ancestors(ctx).collect();
        ancestors.reverse();
        ancestors
            .into_iter()
            .flat_map(|id| id.self_template_params(ctx).into_iter())
            .collect()
    }

    /// Get only the set of template parameters that this item uses. This is a
    /// subset of `all_template_params` and does not necessarily contain any of
    /// `self_template_params`.
    fn used_template_params(&self, ctx: &BindgenContext) -> Vec<TypeId>
    where
        Self: AsRef<ItemId>,
    {
        assert!(
            ctx.in_codegen_phase(),
            "template parameter usage is not computed until codegen"
        );

        let id = *self.as_ref();
        ctx.resolve_item(id)
            .all_template_params(ctx)
            .into_iter()
            .filter(|p| ctx.uses_template_parameter(id, *p))
            .collect()
    }
}

/// A trait for things which may or may not be a named template type parameter.
pub(crate) trait AsTemplateParam {
    /// Any extra information the implementor might need to make this decision.
    type Extra;

    /// Convert this thing to the item ID of a named template type parameter.
    fn as_template_param(
        &self,
        ctx: &BindgenContext,
        extra: &Self::Extra,
    ) -> Option<TypeId>;

    /// Is this a named template type parameter?
    fn is_template_param(
        &self,
        ctx: &BindgenContext,
        extra: &Self::Extra,
    ) -> bool {
        self.as_template_param(ctx, extra).is_some()
    }
}

/// A concrete instantiation of a generic template.
#[derive(Clone, Debug)]
pub(crate) struct TemplateInstantiation {
    /// The template definition which this is instantiating.
    definition: TypeId,
    /// The concrete template arguments, which will be substituted in the
    /// definition for the generic template parameters.
    args: Vec<TypeId>,
}

impl TemplateInstantiation {
    /// Construct a new template instantiation from the given parts.
    pub(crate) fn new<I>(definition: TypeId, args: I) -> TemplateInstantiation
    where
        I: IntoIterator<Item = TypeId>,
    {
        TemplateInstantiation {
            definition,
            args: args.into_iter().collect(),
        }
    }

    /// Get the template definition for this instantiation.
    pub(crate) fn template_definition(&self) -> TypeId {
        self.definition
    }

    /// Get the concrete template arguments used in this instantiation.
    pub(crate) fn template_arguments(&self) -> &[TypeId] {
        &self.args[..]
    }

    /// Parse a `TemplateInstantiation` from a clang `Type`.
    pub(crate) fn from_ty(
        ty: &clang::Type,
        ctx: &mut BindgenContext,
    ) -> Option<TemplateInstantiation> {
        use clang_sys::*;

        let template_args = ty.template_args().map_or(vec![], |args| match ty
            .canonical_type()
            .template_args()
        {
            Some(canonical_args) => {
                let arg_count = args.len();
                args.chain(canonical_args.skip(arg_count))
                    .filter(|t| t.kind() != CXType_Invalid)
                    .map(|t| {
                        Item::from_ty_or_ref(t, t.declaration(), None, ctx)
                    })
                    .collect()
            }
            None => args
                .filter(|t| t.kind() != CXType_Invalid)
                .map(|t| Item::from_ty_or_ref(t, t.declaration(), None, ctx))
                .collect(),
        });

        let declaration = ty.declaration();
        let definition = if declaration.kind() == CXCursor_TypeAliasTemplateDecl
        {
            Some(declaration)
        } else {
            declaration.specialized().or_else(|| {
                let mut template_ref = None;
                ty.declaration().visit(|child| {
                    if child.kind() == CXCursor_TemplateRef {
                        template_ref = Some(child);
                        return CXVisit_Break;
                    }

                    // Instantiations of template aliases might have the
                    // TemplateRef to the template alias definition arbitrarily
                    // deep, so we need to recurse here and not only visit
                    // direct children.
                    CXChildVisit_Recurse
                });

                template_ref.and_then(|cur| cur.referenced())
            })
        };

        let Some(definition) = definition else {
            if !ty.declaration().is_builtin() {
                warn!(
                    "Could not find template definition for template \
                         instantiation"
                );
            }
            return None;
        };

        let template_definition =
            Item::from_ty_or_ref(definition.cur_type(), definition, None, ctx);

        Some(TemplateInstantiation::new(
            template_definition,
            template_args,
        ))
    }
}

impl IsOpaque for TemplateInstantiation {
    type Extra = Item;

    /// Is this an opaque template instantiation?
    fn is_opaque(&self, ctx: &BindgenContext, item: &Item) -> bool {
        if self.template_definition().is_opaque(ctx, &()) {
            return true;
        }

        // TODO(#774): This doesn't properly handle opaque instantiations where
        // an argument is itself an instantiation because `canonical_name` does
        // not insert the template arguments into the name, ie it for nested
        // template arguments it creates "Foo" instead of "Foo<int>". The fully
        // correct fix is to make `canonical_{name,path}` include template
        // arguments properly.

        let mut path = item.path_for_allowlisting(ctx).clone();
        let args: Vec<_> = self
            .template_arguments()
            .iter()
            .map(|arg| {
                let arg_path =
                    ctx.resolve_item(*arg).path_for_allowlisting(ctx);
                arg_path[1..].join("::")
            })
            .collect();
        {
            let last = path.last_mut().unwrap();
            last.push('<');
            last.push_str(&args.join(", "));
            last.push('>');
        }

        ctx.opaque_by_name(&path)
    }
}

impl Trace for TemplateInstantiation {
    type Extra = ();

    fn trace<T>(&self, _ctx: &BindgenContext, tracer: &mut T, _: &())
    where
        T: Tracer,
    {
        tracer
            .visit_kind(self.definition.into(), EdgeKind::TemplateDeclaration);
        for arg in self.template_arguments() {
            tracer.visit_kind(arg.into(), EdgeKind::TemplateArgument);
        }
    }
}
