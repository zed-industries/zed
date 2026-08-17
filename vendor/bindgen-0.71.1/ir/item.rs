//! Bindgen's core intermediate representation type.

use super::super::codegen::{EnumVariation, CONSTIFIED_ENUM_MODULE_REPR_NAME};
use super::analysis::{HasVtable, HasVtableResult, Sizedness, SizednessResult};
use super::annotations::Annotations;
use super::comp::{CompKind, MethodKind};
use super::context::{BindgenContext, ItemId, PartialType, TypeId};
use super::derive::{
    CanDeriveCopy, CanDeriveDebug, CanDeriveDefault, CanDeriveEq,
    CanDeriveHash, CanDeriveOrd, CanDerivePartialEq, CanDerivePartialOrd,
};
use super::dot::DotAttributes;
use super::function::{Function, FunctionKind};
use super::item_kind::ItemKind;
use super::layout::Opaque;
use super::module::Module;
use super::template::{AsTemplateParam, TemplateParameters};
use super::traversal::{EdgeKind, Trace, Tracer};
use super::ty::{Type, TypeKind};
use crate::clang;
use crate::parse::{ClangSubItemParser, ParseError, ParseResult};

use std::cell::{Cell, OnceCell};
use std::collections::BTreeSet;
use std::fmt::Write;
use std::io;
use std::iter;
use std::sync::OnceLock;

/// A trait to get the canonical name from an item.
///
/// This is the trait that will eventually isolate all the logic related to name
/// mangling and that kind of stuff.
///
/// This assumes no nested paths, at some point I'll have to make it a more
/// complex thing.
///
/// This name is required to be safe for Rust, that is, is not expected to
/// return any rust keyword from here.
pub(crate) trait ItemCanonicalName {
    /// Get the canonical name for this item.
    fn canonical_name(&self, ctx: &BindgenContext) -> String;
}

/// The same, but specifies the path that needs to be followed to reach an item.
///
/// To contrast with `canonical_name`, here's an example:
///
/// ```c++
/// namespace foo {
///     const BAR = 3;
/// }
/// ```
///
/// For bar, the canonical path is `vec!["foo", "BAR"]`, while the canonical
/// name is just `"BAR"`.
pub(crate) trait ItemCanonicalPath {
    /// Get the namespace-aware canonical path for this item. This means that if
    /// namespaces are disabled, you'll get a single item, and otherwise you get
    /// the whole path.
    fn namespace_aware_canonical_path(
        &self,
        ctx: &BindgenContext,
    ) -> Vec<String>;

    /// Get the canonical path for this item.
    fn canonical_path(&self, ctx: &BindgenContext) -> Vec<String>;
}

/// A trait for determining if some IR thing is opaque or not.
pub(crate) trait IsOpaque {
    /// Extra context the IR thing needs to determine if it is opaque or not.
    type Extra;

    /// Returns `true` if the thing is opaque, and `false` otherwise.
    ///
    /// May only be called when `ctx` is in the codegen phase.
    fn is_opaque(&self, ctx: &BindgenContext, extra: &Self::Extra) -> bool;
}

/// A trait for determining if some IR thing has type parameter in array or not.
pub(crate) trait HasTypeParamInArray {
    /// Returns `true` if the thing has Array, and `false` otherwise.
    fn has_type_param_in_array(&self, ctx: &BindgenContext) -> bool;
}

/// A trait for iterating over an item and its parents and up its ancestor chain
/// up to (but not including) the implicit root module.
pub(crate) trait ItemAncestors {
    /// Get an iterable over this item's ancestors.
    fn ancestors<'a>(&self, ctx: &'a BindgenContext) -> ItemAncestorsIter<'a>;
}

#[cfg(feature = "__testing_only_extra_assertions")]
type DebugOnlyItemSet = ItemSet;

#[cfg(not(feature = "__testing_only_extra_assertions"))]
struct DebugOnlyItemSet;

#[cfg(not(feature = "__testing_only_extra_assertions"))]
impl DebugOnlyItemSet {
    fn new() -> Self {
        DebugOnlyItemSet
    }

    fn contains(&self, _id: &ItemId) -> bool {
        false
    }

    fn insert(&mut self, _id: ItemId) {}
}

/// An iterator over an item and its ancestors.
pub(crate) struct ItemAncestorsIter<'a> {
    item: ItemId,
    ctx: &'a BindgenContext,
    seen: DebugOnlyItemSet,
}

impl<'a> ItemAncestorsIter<'a> {
    fn new<Id: Into<ItemId>>(ctx: &'a BindgenContext, id: Id) -> Self {
        ItemAncestorsIter {
            item: id.into(),
            ctx,
            seen: DebugOnlyItemSet::new(),
        }
    }
}

impl Iterator for ItemAncestorsIter<'_> {
    type Item = ItemId;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.ctx.resolve_item(self.item);

        if item.parent_id() == self.item {
            None
        } else {
            self.item = item.parent_id();

            extra_assert!(!self.seen.contains(&item.id()));
            self.seen.insert(item.id());

            Some(item.id())
        }
    }
}

impl<T> AsTemplateParam for T
where
    T: Copy + Into<ItemId>,
{
    type Extra = ();

    fn as_template_param(
        &self,
        ctx: &BindgenContext,
        _: &(),
    ) -> Option<TypeId> {
        ctx.resolve_item((*self).into()).as_template_param(ctx, &())
    }
}

impl AsTemplateParam for Item {
    type Extra = ();

    fn as_template_param(
        &self,
        ctx: &BindgenContext,
        _: &(),
    ) -> Option<TypeId> {
        self.kind.as_template_param(ctx, self)
    }
}

impl AsTemplateParam for ItemKind {
    type Extra = Item;

    fn as_template_param(
        &self,
        ctx: &BindgenContext,
        item: &Item,
    ) -> Option<TypeId> {
        match *self {
            ItemKind::Type(ref ty) => ty.as_template_param(ctx, item),
            ItemKind::Module(..) |
            ItemKind::Function(..) |
            ItemKind::Var(..) => None,
        }
    }
}

impl<T> ItemCanonicalName for T
where
    T: Copy + Into<ItemId>,
{
    fn canonical_name(&self, ctx: &BindgenContext) -> String {
        debug_assert!(
            ctx.in_codegen_phase(),
            "You're not supposed to call this yet"
        );
        ctx.resolve_item(*self).canonical_name(ctx)
    }
}

impl<T> ItemCanonicalPath for T
where
    T: Copy + Into<ItemId>,
{
    fn namespace_aware_canonical_path(
        &self,
        ctx: &BindgenContext,
    ) -> Vec<String> {
        debug_assert!(
            ctx.in_codegen_phase(),
            "You're not supposed to call this yet"
        );
        ctx.resolve_item(*self).namespace_aware_canonical_path(ctx)
    }

    fn canonical_path(&self, ctx: &BindgenContext) -> Vec<String> {
        debug_assert!(
            ctx.in_codegen_phase(),
            "You're not supposed to call this yet"
        );
        ctx.resolve_item(*self).canonical_path(ctx)
    }
}

impl<T> ItemAncestors for T
where
    T: Copy + Into<ItemId>,
{
    fn ancestors<'a>(&self, ctx: &'a BindgenContext) -> ItemAncestorsIter<'a> {
        ItemAncestorsIter::new(ctx, *self)
    }
}

impl ItemAncestors for Item {
    fn ancestors<'a>(&self, ctx: &'a BindgenContext) -> ItemAncestorsIter<'a> {
        self.id().ancestors(ctx)
    }
}

impl<Id> Trace for Id
where
    Id: Copy + Into<ItemId>,
{
    type Extra = ();

    fn trace<T>(&self, ctx: &BindgenContext, tracer: &mut T, extra: &())
    where
        T: Tracer,
    {
        ctx.resolve_item(*self).trace(ctx, tracer, extra);
    }
}

impl Trace for Item {
    type Extra = ();

    fn trace<T>(&self, ctx: &BindgenContext, tracer: &mut T, _extra: &())
    where
        T: Tracer,
    {
        // Even if this item is blocklisted/hidden, we want to trace it. It is
        // traversal iterators' consumers' responsibility to filter items as
        // needed. Generally, this filtering happens in the implementation of
        // `Iterator` for `allowlistedItems`. Fully tracing blocklisted items is
        // necessary for things like the template parameter usage analysis to
        // function correctly.

        match *self.kind() {
            ItemKind::Type(ref ty) => {
                // There are some types, like resolved type references, where we
                // don't want to stop collecting types even though they may be
                // opaque.
                if ty.should_be_traced_unconditionally() ||
                    !self.is_opaque(ctx, &())
                {
                    ty.trace(ctx, tracer, self);
                }
            }
            ItemKind::Function(ref fun) => {
                // Just the same way, it has not real meaning for a function to
                // be opaque, so we trace across it.
                tracer.visit(fun.signature().into());
            }
            ItemKind::Var(ref var) => {
                tracer.visit_kind(var.ty().into(), EdgeKind::VarType);
            }
            ItemKind::Module(_) => {
                // Module -> children edges are "weak", and we do not want to
                // trace them. If we did, then allowlisting wouldn't work as
                // expected: everything in every module would end up
                // allowlisted.
                //
                // TODO: make a new edge kind for module -> children edges and
                // filter them during allowlisting traversals.
            }
        }
    }
}

impl CanDeriveDebug for Item {
    fn can_derive_debug(&self, ctx: &BindgenContext) -> bool {
        self.id().can_derive_debug(ctx)
    }
}

impl CanDeriveDefault for Item {
    fn can_derive_default(&self, ctx: &BindgenContext) -> bool {
        self.id().can_derive_default(ctx)
    }
}

impl CanDeriveCopy for Item {
    fn can_derive_copy(&self, ctx: &BindgenContext) -> bool {
        self.id().can_derive_copy(ctx)
    }
}

impl CanDeriveHash for Item {
    fn can_derive_hash(&self, ctx: &BindgenContext) -> bool {
        self.id().can_derive_hash(ctx)
    }
}

impl CanDerivePartialOrd for Item {
    fn can_derive_partialord(&self, ctx: &BindgenContext) -> bool {
        self.id().can_derive_partialord(ctx)
    }
}

impl CanDerivePartialEq for Item {
    fn can_derive_partialeq(&self, ctx: &BindgenContext) -> bool {
        self.id().can_derive_partialeq(ctx)
    }
}

impl CanDeriveEq for Item {
    fn can_derive_eq(&self, ctx: &BindgenContext) -> bool {
        self.id().can_derive_eq(ctx)
    }
}

impl CanDeriveOrd for Item {
    fn can_derive_ord(&self, ctx: &BindgenContext) -> bool {
        self.id().can_derive_ord(ctx)
    }
}

/// An item is the base of the bindgen representation, it can be either a
/// module, a type, a function, or a variable (see `ItemKind` for more
/// information).
///
/// Items refer to each other by `ItemId`. Every item has its parent's
/// ID. Depending on the kind of item this is, it may also refer to other items,
/// such as a compound type item referring to other types. Collectively, these
/// references form a graph.
///
/// The entry-point to this graph is the "root module": a meta-item used to hold
/// all top-level items.
///
/// An item may have a comment, and annotations (see the `annotations` module).
///
/// Note that even though we parse all the types of annotations in comments, not
/// all of them apply to every item. Those rules are described in the
/// `annotations` module.
#[derive(Debug)]
pub(crate) struct Item {
    /// This item's ID.
    id: ItemId,

    /// The item's local ID, unique only amongst its siblings. Only used for
    /// anonymous items.
    ///
    /// Lazily initialized in `local_id()`.
    ///
    /// Note that only structs, unions, and enums get a local type ID. In any
    /// case this is an implementation detail.
    local_id: OnceCell<usize>,

    /// The next local ID to use for a child or template instantiation.
    next_child_local_id: Cell<usize>,

    /// A cached copy of the canonical name, as returned by `canonical_name`.
    ///
    /// This is a fairly used operation during codegen so this makes bindgen
    /// considerably faster in those cases.
    canonical_name: OnceCell<String>,

    /// The path to use for allowlisting and other name-based checks, as
    /// returned by `path_for_allowlisting`, lazily constructed.
    path_for_allowlisting: OnceCell<Vec<String>>,

    /// A doc comment over the item, if any.
    comment: Option<String>,
    /// Annotations extracted from the doc comment, or the default ones
    /// otherwise.
    annotations: Annotations,
    /// An item's parent ID. This will most likely be a class where this item
    /// was declared, or a module, etc.
    ///
    /// All the items have a parent, except the root module, in which case the
    /// parent ID is its own ID.
    parent_id: ItemId,
    /// The item kind.
    kind: ItemKind,
    /// The source location of the item.
    location: Option<clang::SourceLocation>,
}

impl AsRef<ItemId> for Item {
    fn as_ref(&self) -> &ItemId {
        &self.id
    }
}

impl Item {
    /// Construct a new `Item`.
    pub(crate) fn new(
        id: ItemId,
        comment: Option<String>,
        annotations: Option<Annotations>,
        parent_id: ItemId,
        kind: ItemKind,
        location: Option<clang::SourceLocation>,
    ) -> Self {
        debug_assert!(id != parent_id || kind.is_module());
        Item {
            id,
            local_id: OnceCell::new(),
            next_child_local_id: Cell::new(1),
            canonical_name: OnceCell::new(),
            path_for_allowlisting: OnceCell::new(),
            parent_id,
            comment,
            annotations: annotations.unwrap_or_default(),
            kind,
            location,
        }
    }

    /// Construct a new opaque item type.
    pub(crate) fn new_opaque_type(
        with_id: ItemId,
        ty: &clang::Type,
        ctx: &mut BindgenContext,
    ) -> TypeId {
        let location = ty.declaration().location();
        let ty = Opaque::from_clang_ty(ty, ctx);
        let kind = ItemKind::Type(ty);
        let parent = ctx.root_module().into();
        ctx.add_item(
            Item::new(with_id, None, None, parent, kind, Some(location)),
            None,
            None,
        );
        with_id.as_type_id_unchecked()
    }

    /// Get this `Item`'s identifier.
    pub(crate) fn id(&self) -> ItemId {
        self.id
    }

    /// Get this `Item`'s parent's identifier.
    ///
    /// For the root module, the parent's ID is its own ID.
    pub(crate) fn parent_id(&self) -> ItemId {
        self.parent_id
    }

    /// Set this item's parent ID.
    ///
    /// This is only used so replacements get generated in the proper module.
    pub(crate) fn set_parent_for_replacement<Id: Into<ItemId>>(
        &mut self,
        id: Id,
    ) {
        self.parent_id = id.into();
    }

    /// Returns the depth this item is indented to.
    ///
    /// FIXME(emilio): This may need fixes for the enums within modules stuff.
    pub(crate) fn codegen_depth(&self, ctx: &BindgenContext) -> usize {
        if !ctx.options().enable_cxx_namespaces {
            return 0;
        }

        self.ancestors(ctx)
            .filter(|id| {
                ctx.resolve_item(*id).as_module().is_some_and(|module| {
                    !module.is_inline() ||
                        ctx.options().conservative_inline_namespaces
                })
            })
            .count() +
            1
    }

    /// Get this `Item`'s comment, if it has any, already preprocessed and with
    /// the right indentation.
    pub(crate) fn comment(&self, ctx: &BindgenContext) -> Option<String> {
        if !ctx.options().generate_comments {
            return None;
        }

        self.comment
            .as_ref()
            .map(|comment| ctx.options().process_comment(comment))
    }

    /// What kind of item is this?
    pub(crate) fn kind(&self) -> &ItemKind {
        &self.kind
    }

    /// Get a mutable reference to this item's kind.
    pub(crate) fn kind_mut(&mut self) -> &mut ItemKind {
        &mut self.kind
    }

    /// Where in the source is this item located?
    pub(crate) fn location(&self) -> Option<&clang::SourceLocation> {
        self.location.as_ref()
    }

    /// Get an identifier that differentiates this item from its siblings.
    ///
    /// This should stay relatively stable in the face of code motion outside or
    /// below this item's lexical scope, meaning that this can be useful for
    /// generating relatively stable identifiers within a scope.
    pub(crate) fn local_id(&self, ctx: &BindgenContext) -> usize {
        *self.local_id.get_or_init(|| {
            let parent = ctx.resolve_item(self.parent_id);
            parent.next_child_local_id()
        })
    }

    /// Get an identifier that differentiates a child of this item of other
    /// related items.
    ///
    /// This is currently used for anonymous items, and template instantiation
    /// tests, in both cases in order to reduce noise when system headers are at
    /// place.
    pub(crate) fn next_child_local_id(&self) -> usize {
        let local_id = self.next_child_local_id.get();
        self.next_child_local_id.set(local_id + 1);
        local_id
    }

    /// Returns whether this item is a top-level item, from the point of view of
    /// bindgen.
    ///
    /// This point of view changes depending on whether namespaces are enabled
    /// or not. That way, in the following example:
    ///
    /// ```c++
    /// namespace foo {
    ///     static int var;
    /// }
    /// ```
    ///
    /// `var` would be a toplevel item if namespaces are disabled, but won't if
    /// they aren't.
    ///
    /// This function is used to determine when the codegen phase should call
    /// `codegen` on an item, since any item that is not top-level will be
    /// generated by its parent.
    pub(crate) fn is_toplevel(&self, ctx: &BindgenContext) -> bool {
        // FIXME: Workaround for some types falling behind when parsing weird
        // stl classes, for example.
        if ctx.options().enable_cxx_namespaces &&
            self.kind().is_module() &&
            self.id() != ctx.root_module()
        {
            return false;
        }

        let mut parent = self.parent_id;
        loop {
            let Some(parent_item) = ctx.resolve_item_fallible(parent) else {
                return false;
            };

            if parent_item.id() == ctx.root_module() {
                return true;
            } else if ctx.options().enable_cxx_namespaces ||
                !parent_item.kind().is_module()
            {
                return false;
            }

            parent = parent_item.parent_id();
        }
    }

    /// Get a reference to this item's underlying `Type`. Panic if this is some
    /// other kind of item.
    pub(crate) fn expect_type(&self) -> &Type {
        self.kind().expect_type()
    }

    /// Get a reference to this item's underlying `Type`, or `None` if this is
    /// some other kind of item.
    pub(crate) fn as_type(&self) -> Option<&Type> {
        self.kind().as_type()
    }

    /// Get a reference to this item's underlying `Function`. Panic if this is
    /// some other kind of item.
    pub(crate) fn expect_function(&self) -> &Function {
        self.kind().expect_function()
    }

    /// Is this item a module?
    pub(crate) fn is_module(&self) -> bool {
        matches!(self.kind, ItemKind::Module(..))
    }

    /// Get this item's annotations.
    pub(crate) fn annotations(&self) -> &Annotations {
        &self.annotations
    }

    /// Whether this item should be blocklisted.
    ///
    /// This may be due to either annotations or to other kind of configuration.
    pub(crate) fn is_blocklisted(&self, ctx: &BindgenContext) -> bool {
        debug_assert!(
            ctx.in_codegen_phase(),
            "You're not supposed to call this yet"
        );
        if self.annotations.hide() {
            return true;
        }

        if !ctx.options().blocklisted_files.is_empty() {
            if let Some(location) = &self.location {
                let (file, _, _, _) = location.location();
                if let Some(filename) = file.name() {
                    if ctx.options().blocklisted_files.matches(filename) {
                        return true;
                    }
                }
            }
        }

        let path = self.path_for_allowlisting(ctx);
        let name = path[1..].join("::");
        ctx.options().blocklisted_items.matches(&name) ||
            match self.kind {
                ItemKind::Type(..) => {
                    ctx.options().blocklisted_types.matches(&name) ||
                        ctx.is_replaced_type(path, self.id)
                }
                ItemKind::Function(..) => {
                    ctx.options().blocklisted_functions.matches(&name)
                }
                ItemKind::Var(..) => {
                    ctx.options().blocklisted_vars.matches(&name)
                }
                // TODO: Add namespace blocklisting?
                ItemKind::Module(..) => false,
            }
    }

    /// Take out item `NameOptions`
    pub(crate) fn name<'a>(
        &'a self,
        ctx: &'a BindgenContext,
    ) -> NameOptions<'a> {
        NameOptions::new(self, ctx)
    }

    /// Get the target item ID for name generation.
    fn name_target(&self, ctx: &BindgenContext) -> ItemId {
        let mut targets_seen = DebugOnlyItemSet::new();
        let mut item = self;

        loop {
            extra_assert!(!targets_seen.contains(&item.id()));
            targets_seen.insert(item.id());

            if self.annotations().use_instead_of().is_some() {
                return self.id();
            }

            match *item.kind() {
                ItemKind::Type(ref ty) => match *ty.kind() {
                    TypeKind::ResolvedTypeRef(inner) => {
                        item = ctx.resolve_item(inner);
                    }
                    TypeKind::TemplateInstantiation(ref inst) => {
                        item = ctx.resolve_item(inst.template_definition());
                    }
                    _ => return item.id(),
                },
                _ => return item.id(),
            }
        }
    }

    /// Create a fully disambiguated name for an item, including template
    /// parameters if it is a type
    pub(crate) fn full_disambiguated_name(
        &self,
        ctx: &BindgenContext,
    ) -> String {
        let mut s = String::new();
        let level = 0;
        self.push_disambiguated_name(ctx, &mut s, level);
        s
    }

    /// Helper function for `full_disambiguated_name`
    fn push_disambiguated_name(
        &self,
        ctx: &BindgenContext,
        to: &mut String,
        level: u8,
    ) {
        to.push_str(&self.canonical_name(ctx));
        if let ItemKind::Type(ref ty) = *self.kind() {
            if let TypeKind::TemplateInstantiation(ref inst) = *ty.kind() {
                to.push_str(&format!("_open{level}_"));
                for arg in inst.template_arguments() {
                    arg.into_resolver()
                        .through_type_refs()
                        .resolve(ctx)
                        .push_disambiguated_name(ctx, to, level + 1);
                    to.push('_');
                }
                to.push_str(&format!("close{level}"));
            }
        }
    }

    /// Get this function item's name, or `None` if this item is not a function.
    fn func_name(&self) -> Option<&str> {
        match *self.kind() {
            ItemKind::Function(ref func) => Some(func.name()),
            _ => None,
        }
    }

    /// Get the overload index for this method. If this is not a method, return
    /// `None`.
    fn overload_index(&self, ctx: &BindgenContext) -> Option<usize> {
        self.func_name().and_then(|func_name| {
            let parent = ctx.resolve_item(self.parent_id());
            if let ItemKind::Type(ref ty) = *parent.kind() {
                if let TypeKind::Comp(ref ci) = *ty.kind() {
                    // All the constructors have the same name, so no need to
                    // resolve and check.
                    return ci
                        .constructors()
                        .iter()
                        .position(|c| *c == self.id())
                        .or_else(|| {
                            ci.methods()
                                .iter()
                                .filter(|m| {
                                    let item = ctx.resolve_item(m.signature());
                                    let func = item.expect_function();
                                    func.name() == func_name
                                })
                                .position(|m| m.signature() == self.id())
                        });
                }
            }

            None
        })
    }

    /// Get this item's base name (aka non-namespaced name).
    fn base_name(&self, ctx: &BindgenContext) -> String {
        if let Some(path) = self.annotations().use_instead_of() {
            return path.last().unwrap().clone();
        }

        match *self.kind() {
            ItemKind::Var(ref var) => var.name().to_owned(),
            ItemKind::Module(ref module) => module.name().map_or_else(
                || format!("_bindgen_mod_{}", self.exposed_id(ctx)),
                ToOwned::to_owned,
            ),
            ItemKind::Type(ref ty) => ty.sanitized_name(ctx).map_or_else(
                || format!("_bindgen_ty_{}", self.exposed_id(ctx)),
                Into::into,
            ),
            ItemKind::Function(ref fun) => {
                let mut name = fun.name().to_owned();

                if let Some(idx) = self.overload_index(ctx) {
                    if idx > 0 {
                        write!(&mut name, "{idx}").unwrap();
                    }
                }

                name
            }
        }
    }

    fn is_anon(&self) -> bool {
        match self.kind() {
            ItemKind::Module(module) => module.name().is_none(),
            ItemKind::Type(ty) => ty.name().is_none(),
            ItemKind::Function(_) => false,
            ItemKind::Var(_) => false,
        }
    }

    /// Get the canonical name without taking into account the replaces
    /// annotation.
    ///
    /// This is the base logic used to implement hiding and replacing via
    /// annotations, and also to implement proper name mangling.
    ///
    /// The idea is that each generated type in the same "level" (read: module
    /// or namespace) has a unique canonical name.
    ///
    /// This name should be derived from the immutable state contained in the
    /// type and the parent chain, since it should be consistent.
    ///
    /// If `BindgenOptions::disable_nested_struct_naming` is true then returned
    /// name is the inner most non-anonymous name plus all the anonymous base names
    /// that follows.
    pub(crate) fn real_canonical_name(
        &self,
        ctx: &BindgenContext,
        opt: &NameOptions,
    ) -> String {
        let target = ctx.resolve_item(self.name_target(ctx));

        // Short-circuit if the target has an override, and just use that.
        if let Some(path) = target.annotations.use_instead_of() {
            if ctx.options().enable_cxx_namespaces {
                return path.last().unwrap().clone();
            }
            return path.join("_");
        }

        let base_name = target.base_name(ctx);

        // Named template type arguments are never namespaced, and never
        // mangled.
        if target.is_template_param(ctx, &()) {
            return base_name;
        }

        // Ancestors' ID iter
        let mut ids_iter = target
            .parent_id()
            .ancestors(ctx)
            .filter(|id| *id != ctx.root_module())
            .take_while(|id| {
                // Stop iterating ancestors once we reach a non-inline namespace
                // when opt.within_namespaces is set.
                !opt.within_namespaces || !ctx.resolve_item(*id).is_module()
            })
            .filter(|id| {
                if !ctx.options().conservative_inline_namespaces {
                    if let ItemKind::Module(ref module) =
                        *ctx.resolve_item(*id).kind()
                    {
                        return !module.is_inline();
                    }
                }

                true
            });

        let ids: Vec<_> = if ctx.options().disable_nested_struct_naming {
            let mut ids = Vec::new();

            // If target is anonymous we need find its first named ancestor.
            if target.is_anon() {
                for id in ids_iter.by_ref() {
                    ids.push(id);

                    if !ctx.resolve_item(id).is_anon() {
                        break;
                    }
                }
            }

            ids
        } else {
            ids_iter.collect()
        };

        // Concatenate this item's ancestors' names together.
        let mut names: Vec<_> = ids
            .into_iter()
            .map(|id| {
                let item = ctx.resolve_item(id);
                let target = ctx.resolve_item(item.name_target(ctx));
                target.base_name(ctx)
            })
            .filter(|name| !name.is_empty())
            .collect();

        names.reverse();

        if !base_name.is_empty() {
            names.push(base_name);
        }

        if ctx.options().c_naming {
            if let Some(prefix) = self.c_naming_prefix() {
                names.insert(0, prefix.to_string());
            }
        }

        let name = names.join("_");

        let name = if opt.user_mangled == UserMangled::Yes {
            ctx.options()
                .last_callback(|callbacks| callbacks.item_name(&name))
                .unwrap_or(name)
        } else {
            name
        };

        ctx.rust_mangle(&name).into_owned()
    }

    /// The exposed ID that represents an unique ID among the siblings of a
    /// given item.
    pub(crate) fn exposed_id(&self, ctx: &BindgenContext) -> String {
        // Only use local ids for enums, classes, structs and union types.  All
        // other items use their global ID.
        let ty_kind = self.kind().as_type().map(|t| t.kind());
        if let Some(
            TypeKind::Comp(..) |
            TypeKind::TemplateInstantiation(..) |
            TypeKind::Enum(..),
        ) = ty_kind
        {
            return self.local_id(ctx).to_string();
        }

        // Note that this `id_` prefix prevents (really unlikely) collisions
        // between the global ID and the local ID of an item with the same
        // parent.
        format!("id_{}", self.id().as_usize())
    }

    /// Get a reference to this item's `Module`, or `None` if this is not a
    /// `Module` item.
    pub(crate) fn as_module(&self) -> Option<&Module> {
        match self.kind {
            ItemKind::Module(ref module) => Some(module),
            _ => None,
        }
    }

    /// Get a mutable reference to this item's `Module`, or `None` if this is
    /// not a `Module` item.
    pub(crate) fn as_module_mut(&mut self) -> Option<&mut Module> {
        match self.kind {
            ItemKind::Module(ref mut module) => Some(module),
            _ => None,
        }
    }

    /// Returns whether the item is a constified module enum
    fn is_constified_enum_module(&self, ctx: &BindgenContext) -> bool {
        // Do not jump through aliases, except for aliases that point to a type
        // with the same name, since we dont generate coe for them.
        let item = self.id.into_resolver().through_type_refs().resolve(ctx);
        let ItemKind::Type(ref type_) = *item.kind() else {
            return false;
        };

        match *type_.kind() {
            TypeKind::Enum(ref enum_) => {
                enum_.computed_enum_variation(ctx, self) ==
                    EnumVariation::ModuleConsts
            }
            TypeKind::Alias(inner_id) => {
                // TODO(emilio): Make this "hop through type aliases that aren't
                // really generated" an option in `ItemResolver`?
                let inner_item = ctx.resolve_item(inner_id);
                let name = item.canonical_name(ctx);

                if inner_item.canonical_name(ctx) == name {
                    inner_item.is_constified_enum_module(ctx)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Is this item of a kind that is enabled for code generation?
    pub(crate) fn is_enabled_for_codegen(&self, ctx: &BindgenContext) -> bool {
        let cc = &ctx.options().codegen_config;
        match *self.kind() {
            ItemKind::Module(..) => true,
            ItemKind::Var(_) => cc.vars(),
            ItemKind::Type(_) => cc.types(),
            ItemKind::Function(ref f) => match f.kind() {
                FunctionKind::Function => cc.functions(),
                FunctionKind::Method(MethodKind::Constructor) => {
                    cc.constructors()
                }
                FunctionKind::Method(MethodKind::Destructor) |
                FunctionKind::Method(MethodKind::VirtualDestructor {
                    ..
                }) => cc.destructors(),
                FunctionKind::Method(MethodKind::Static) |
                FunctionKind::Method(MethodKind::Normal) |
                FunctionKind::Method(MethodKind::Virtual { .. }) => {
                    cc.methods()
                }
            },
        }
    }

    /// Returns the path we should use for allowlisting / blocklisting, which
    /// doesn't include user-mangling.
    pub(crate) fn path_for_allowlisting(
        &self,
        ctx: &BindgenContext,
    ) -> &Vec<String> {
        self.path_for_allowlisting
            .get_or_init(|| self.compute_path(ctx, UserMangled::No))
    }

    fn compute_path(
        &self,
        ctx: &BindgenContext,
        mangled: UserMangled,
    ) -> Vec<String> {
        if let Some(path) = self.annotations().use_instead_of() {
            let mut ret =
                vec![ctx.resolve_item(ctx.root_module()).name(ctx).get()];
            ret.extend_from_slice(path);
            return ret;
        }

        let target = ctx.resolve_item(self.name_target(ctx));
        let mut path: Vec<_> = target
            .ancestors(ctx)
            .chain(iter::once(ctx.root_module().into()))
            .map(|id| ctx.resolve_item(id))
            .filter(|item| {
                item.id() == target.id() ||
                    item.as_module().is_some_and(|module| {
                        !module.is_inline() ||
                            ctx.options().conservative_inline_namespaces
                    })
            })
            .map(|item| {
                ctx.resolve_item(item.name_target(ctx))
                    .name(ctx)
                    .within_namespaces()
                    .user_mangled(mangled)
                    .get()
            })
            .collect();
        path.reverse();
        path
    }

    /// Returns a prefix for the canonical name when C naming is enabled.
    fn c_naming_prefix(&self) -> Option<&str> {
        let ItemKind::Type(ref ty) = self.kind else {
            return None;
        };

        Some(match ty.kind() {
            TypeKind::Comp(ref ci) => match ci.kind() {
                CompKind::Struct => "struct",
                CompKind::Union => "union",
            },
            TypeKind::Enum(..) => "enum",
            _ => return None,
        })
    }

    /// Whether this is a `#[must_use]` type.
    pub(crate) fn must_use(&self, ctx: &BindgenContext) -> bool {
        self.annotations().must_use_type() || ctx.must_use_type_by_name(self)
    }
}

impl<T> IsOpaque for T
where
    T: Copy + Into<ItemId>,
{
    type Extra = ();

    fn is_opaque(&self, ctx: &BindgenContext, _: &()) -> bool {
        debug_assert!(
            ctx.in_codegen_phase(),
            "You're not supposed to call this yet"
        );
        ctx.resolve_item((*self).into()).is_opaque(ctx, &())
    }
}

impl IsOpaque for Item {
    type Extra = ();

    fn is_opaque(&self, ctx: &BindgenContext, _: &()) -> bool {
        debug_assert!(
            ctx.in_codegen_phase(),
            "You're not supposed to call this yet"
        );
        self.annotations.opaque() ||
            self.as_type().is_some_and(|ty| ty.is_opaque(ctx, self)) ||
            ctx.opaque_by_name(self.path_for_allowlisting(ctx))
    }
}

impl<T> HasVtable for T
where
    T: Copy + Into<ItemId>,
{
    fn has_vtable(&self, ctx: &BindgenContext) -> bool {
        let id: ItemId = (*self).into();
        id.as_type_id(ctx).is_some_and(|id| {
            !matches!(ctx.lookup_has_vtable(id), HasVtableResult::No)
        })
    }

    fn has_vtable_ptr(&self, ctx: &BindgenContext) -> bool {
        let id: ItemId = (*self).into();
        id.as_type_id(ctx).is_some_and(|id| {
            matches!(ctx.lookup_has_vtable(id), HasVtableResult::SelfHasVtable)
        })
    }
}

impl HasVtable for Item {
    fn has_vtable(&self, ctx: &BindgenContext) -> bool {
        self.id().has_vtable(ctx)
    }

    fn has_vtable_ptr(&self, ctx: &BindgenContext) -> bool {
        self.id().has_vtable_ptr(ctx)
    }
}

impl<T> Sizedness for T
where
    T: Copy + Into<ItemId>,
{
    fn sizedness(&self, ctx: &BindgenContext) -> SizednessResult {
        let id: ItemId = (*self).into();
        id.as_type_id(ctx)
            .map_or(SizednessResult::default(), |id| ctx.lookup_sizedness(id))
    }
}

impl Sizedness for Item {
    fn sizedness(&self, ctx: &BindgenContext) -> SizednessResult {
        self.id().sizedness(ctx)
    }
}

impl<T> HasTypeParamInArray for T
where
    T: Copy + Into<ItemId>,
{
    fn has_type_param_in_array(&self, ctx: &BindgenContext) -> bool {
        debug_assert!(
            ctx.in_codegen_phase(),
            "You're not supposed to call this yet"
        );
        ctx.lookup_has_type_param_in_array(*self)
    }
}

impl HasTypeParamInArray for Item {
    fn has_type_param_in_array(&self, ctx: &BindgenContext) -> bool {
        debug_assert!(
            ctx.in_codegen_phase(),
            "You're not supposed to call this yet"
        );
        ctx.lookup_has_type_param_in_array(self.id())
    }
}

/// A set of items.
pub(crate) type ItemSet = BTreeSet<ItemId>;

impl DotAttributes for Item {
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
            "<tr><td>{:?}</td></tr>
                       <tr><td>name</td><td>{}</td></tr>",
            self.id,
            self.name(ctx).get()
        )?;

        if self.is_opaque(ctx, &()) {
            writeln!(out, "<tr><td>opaque</td><td>true</td></tr>")?;
        }

        self.kind.dot_attributes(ctx, out)
    }
}

impl<T> TemplateParameters for T
where
    T: Copy + Into<ItemId>,
{
    fn self_template_params(&self, ctx: &BindgenContext) -> Vec<TypeId> {
        ctx.resolve_item_fallible(*self)
            .map_or(vec![], |item| item.self_template_params(ctx))
    }
}

impl TemplateParameters for Item {
    fn self_template_params(&self, ctx: &BindgenContext) -> Vec<TypeId> {
        self.kind.self_template_params(ctx)
    }
}

impl TemplateParameters for ItemKind {
    fn self_template_params(&self, ctx: &BindgenContext) -> Vec<TypeId> {
        match *self {
            ItemKind::Type(ref ty) => ty.self_template_params(ctx),
            // If we start emitting bindings to explicitly instantiated
            // functions, then we'll need to check ItemKind::Function for
            // template params.
            ItemKind::Function(_) | ItemKind::Module(_) | ItemKind::Var(_) => {
                vec![]
            }
        }
    }
}

// An utility function to handle recursing inside nested types.
fn visit_child(
    cur: clang::Cursor,
    id: ItemId,
    ty: &clang::Type,
    parent_id: Option<ItemId>,
    ctx: &mut BindgenContext,
    result: &mut Result<TypeId, ParseError>,
) -> clang_sys::CXChildVisitResult {
    use clang_sys::*;
    if result.is_ok() {
        return CXChildVisit_Break;
    }

    *result = Item::from_ty_with_id(id, ty, cur, parent_id, ctx);

    match *result {
        Ok(..) => CXChildVisit_Break,
        Err(ParseError::Recurse) => {
            cur.visit(|c| visit_child(c, id, ty, parent_id, ctx, result));
            CXChildVisit_Continue
        }
        Err(ParseError::Continue) => CXChildVisit_Continue,
    }
}

impl Item {
    /// Create a builtin type.
    pub(crate) fn builtin_type(
        kind: TypeKind,
        is_const: bool,
        ctx: &mut BindgenContext,
    ) -> TypeId {
        // Feel free to add more here, I'm just lazy.
        match kind {
            TypeKind::Void |
            TypeKind::Int(..) |
            TypeKind::Pointer(..) |
            TypeKind::Float(..) => {}
            _ => panic!("Unsupported builtin type"),
        }

        let ty = Type::new(None, None, kind, is_const);
        let id = ctx.next_item_id();
        let module = ctx.root_module().into();
        ctx.add_item(
            Item::new(id, None, None, module, ItemKind::Type(ty), None),
            None,
            None,
        );
        id.as_type_id_unchecked()
    }

    /// Parse this item from the given Clang cursor.
    pub(crate) fn parse(
        cursor: clang::Cursor,
        parent_id: Option<ItemId>,
        ctx: &mut BindgenContext,
    ) -> Result<ItemId, ParseError> {
        use crate::ir::var::Var;
        use clang_sys::*;

        if !cursor.is_valid() {
            return Err(ParseError::Continue);
        }

        let comment = cursor.raw_comment();
        let annotations = Annotations::new(&cursor);

        let current_module = ctx.current_module().into();
        let relevant_parent_id = parent_id.unwrap_or(current_module);

        #[allow(clippy::missing_docs_in_private_items)]
        macro_rules! try_parse {
            ($what:ident) => {
                match $what::parse(cursor, ctx) {
                    Ok(ParseResult::New(item, declaration)) => {
                        let id = ctx.next_item_id();

                        ctx.add_item(
                            Item::new(
                                id,
                                comment,
                                annotations,
                                relevant_parent_id,
                                ItemKind::$what(item),
                                Some(cursor.location()),
                            ),
                            declaration,
                            Some(cursor),
                        );
                        return Ok(id);
                    }
                    Ok(ParseResult::AlreadyResolved(id)) => {
                        return Ok(id);
                    }
                    Err(ParseError::Recurse) => return Err(ParseError::Recurse),
                    Err(ParseError::Continue) => {}
                }
            };
        }

        try_parse!(Module);

        // NOTE: Is extremely important to parse functions and vars **before**
        // types.  Otherwise we can parse a function declaration as a type
        // (which is legal), and lose functions to generate.
        //
        // In general, I'm not totally confident this split between
        // ItemKind::Function and TypeKind::FunctionSig is totally worth it, but
        // I guess we can try.
        try_parse!(Function);
        try_parse!(Var);

        // Types are sort of special, so to avoid parsing template classes
        // twice, handle them separately.
        {
            let definition = cursor.definition();
            let applicable_cursor = definition.unwrap_or(cursor);

            let relevant_parent_id = match definition {
                Some(definition) => {
                    if definition != cursor {
                        ctx.add_semantic_parent(definition, relevant_parent_id);
                        return Ok(Item::from_ty_or_ref(
                            applicable_cursor.cur_type(),
                            cursor,
                            parent_id,
                            ctx,
                        )
                        .into());
                    }
                    ctx.known_semantic_parent(definition)
                        .or(parent_id)
                        .unwrap_or_else(|| ctx.current_module().into())
                }
                None => relevant_parent_id,
            };

            match Item::from_ty(
                &applicable_cursor.cur_type(),
                applicable_cursor,
                Some(relevant_parent_id),
                ctx,
            ) {
                Ok(ty) => return Ok(ty.into()),
                Err(ParseError::Recurse) => return Err(ParseError::Recurse),
                Err(ParseError::Continue) => {}
            }
        }

        match cursor.kind() {
            // On Clang 18+, extern "C" is reported accurately as a LinkageSpec.
            // Older LLVM treat it as UnexposedDecl.
            CXCursor_LinkageSpec | CXCursor_UnexposedDecl => {
                Err(ParseError::Recurse)
            }

            // We allowlist cursors here known to be unhandled, to prevent being
            // too noisy about this.
            CXCursor_MacroDefinition |
            CXCursor_MacroExpansion |
            CXCursor_UsingDeclaration |
            CXCursor_UsingDirective |
            CXCursor_StaticAssert |
            CXCursor_FunctionTemplate => {
                debug!("Unhandled cursor kind {:?}: {cursor:?}", cursor.kind(),);
                Err(ParseError::Continue)
            }

            CXCursor_InclusionDirective => {
                let file = cursor.get_included_file_name();
                match file {
                    None => {
                        warn!("Inclusion of a nameless file in {cursor:?}");
                    }
                    Some(included_file) => {
                        for cb in &ctx.options().parse_callbacks {
                            cb.include_file(&included_file);
                        }

                        ctx.add_dep(included_file.into_boxed_str());
                    }
                }
                Err(ParseError::Continue)
            }

            _ => {
                // ignore toplevel operator overloads
                let spelling = cursor.spelling();
                if !spelling.starts_with("operator") {
                    warn!(
                        "Unhandled cursor kind {:?}: {cursor:?}",
                        cursor.kind(),
                    );
                }
                Err(ParseError::Continue)
            }
        }
    }

    /// Parse this item from the given Clang type, or if we haven't resolved all
    /// the other items this one depends on, an unresolved reference.
    pub(crate) fn from_ty_or_ref(
        ty: clang::Type,
        location: clang::Cursor,
        parent_id: Option<ItemId>,
        ctx: &mut BindgenContext,
    ) -> TypeId {
        let id = ctx.next_item_id();
        Self::from_ty_or_ref_with_id(id, ty, location, parent_id, ctx)
    }

    /// Parse a C++ type. If we find a reference to a type that has not been
    /// defined yet, use `UnresolvedTypeRef` as a placeholder.
    ///
    /// This logic is needed to avoid parsing items with the incorrect parent
    /// and it's sort of complex to explain, so I'll just point to
    /// `tests/headers/typeref.hpp` to see the kind of constructs that forced
    /// this.
    ///
    /// Typerefs are resolved once parsing is completely done, see
    /// `BindgenContext::resolve_typerefs`.
    pub(crate) fn from_ty_or_ref_with_id(
        potential_id: ItemId,
        ty: clang::Type,
        location: clang::Cursor,
        parent_id: Option<ItemId>,
        ctx: &mut BindgenContext,
    ) -> TypeId {
        debug!("from_ty_or_ref_with_id: {potential_id:?} {ty:?}, {location:?}, {parent_id:?}");

        if ctx.collected_typerefs() {
            debug!("refs already collected, resolving directly");
            return Item::from_ty_with_id(
                potential_id,
                &ty,
                location,
                parent_id,
                ctx,
            )
            .unwrap_or_else(|_| Item::new_opaque_type(potential_id, &ty, ctx));
        }

        if let Some(ty) = ctx.builtin_or_resolved_ty(
            potential_id,
            parent_id,
            &ty,
            Some(location),
        ) {
            debug!("{ty:?} already resolved: {location:?}");
            return ty;
        }

        debug!("New unresolved type reference: {ty:?}, {location:?}");

        let is_const = ty.is_const();
        let kind = TypeKind::UnresolvedTypeRef(ty, location, parent_id);
        let current_module = ctx.current_module();

        ctx.add_item(
            Item::new(
                potential_id,
                None,
                None,
                parent_id.unwrap_or_else(|| current_module.into()),
                ItemKind::Type(Type::new(None, None, kind, is_const)),
                Some(location.location()),
            ),
            None,
            None,
        );
        potential_id.as_type_id_unchecked()
    }

    /// Parse this item from the given Clang type. See [`Item::from_ty_with_id`].
    pub(crate) fn from_ty(
        ty: &clang::Type,
        location: clang::Cursor,
        parent_id: Option<ItemId>,
        ctx: &mut BindgenContext,
    ) -> Result<TypeId, ParseError> {
        let id = ctx.next_item_id();
        Item::from_ty_with_id(id, ty, location, parent_id, ctx)
    }

    /// This is one of the trickiest methods you'll find (probably along with
    /// some of the ones that handle templates in `BindgenContext`).
    ///
    /// This method parses a type, given the potential ID of that type (if
    /// parsing it was correct), an optional location we're scanning, which is
    /// critical some times to obtain information, an optional parent item ID,
    /// that will, if it's `None`, become the current module ID, and the
    /// context.
    pub(crate) fn from_ty_with_id(
        id: ItemId,
        ty: &clang::Type,
        location: clang::Cursor,
        parent_id: Option<ItemId>,
        ctx: &mut BindgenContext,
    ) -> Result<TypeId, ParseError> {
        use clang_sys::*;

        debug!(
            "Item::from_ty_with_id: {id:?}\n\
             \tty = {ty:?},\n\
             \tlocation = {location:?}",
        );

        if ty.kind() == CXType_Unexposed ||
            location.cur_type().kind() == CXType_Unexposed
        {
            if ty.is_associated_type() ||
                location.cur_type().is_associated_type()
            {
                return Ok(Item::new_opaque_type(id, ty, ctx));
            }

            if let Some(param_id) = Item::type_param(None, location, ctx) {
                return Ok(ctx.build_ty_wrapper(id, param_id, None, ty));
            }
        }

        // Treat all types that are declared inside functions as opaque. The Rust binding
        // won't be able to do anything with them anyway.
        //
        // (If we don't do this check here, we can have subtle logic bugs because we generally
        // ignore function bodies. See issue #2036.)
        if let Some(ref parent) = ty.declaration().fallible_semantic_parent() {
            if FunctionKind::from_cursor(parent).is_some() {
                debug!("Skipping type declared inside function: {ty:?}");
                return Ok(Item::new_opaque_type(id, ty, ctx));
            }
        }

        let decl = {
            let canonical_def = ty.canonical_type().declaration().definition();
            canonical_def.unwrap_or_else(|| ty.declaration())
        };

        let comment = location
            .raw_comment()
            .or_else(|| decl.raw_comment())
            .or_else(|| location.raw_comment());

        let annotations =
            Annotations::new(&decl).or_else(|| Annotations::new(&location));

        if let Some(ref annotations) = annotations {
            if let Some(replaced) = annotations.use_instead_of() {
                ctx.replace(replaced, id);
            }
        }

        if let Some(ty) =
            ctx.builtin_or_resolved_ty(id, parent_id, ty, Some(location))
        {
            return Ok(ty);
        }

        // First, check we're not recursing.
        let mut valid_decl = decl.kind() != CXCursor_NoDeclFound;
        let declaration_to_look_for = if valid_decl {
            decl.canonical()
        } else if location.kind() == CXCursor_ClassTemplate {
            valid_decl = true;
            location
        } else {
            decl
        };

        if valid_decl {
            if let Some(partial) = ctx
                .currently_parsed_types()
                .iter()
                .find(|ty| *ty.decl() == declaration_to_look_for)
            {
                debug!("Avoiding recursion parsing type: {ty:?}");
                // Unchecked because we haven't finished this type yet.
                return Ok(partial.id().as_type_id_unchecked());
            }
        }

        let current_module = ctx.current_module().into();
        let partial_ty = PartialType::new(declaration_to_look_for, id);
        if valid_decl {
            ctx.begin_parsing(partial_ty);
        }

        let result = Type::from_clang_ty(id, ty, location, parent_id, ctx);
        let relevant_parent_id = parent_id.unwrap_or(current_module);
        let ret = match result {
            Ok(ParseResult::AlreadyResolved(ty)) => {
                Ok(ty.as_type_id_unchecked())
            }
            Ok(ParseResult::New(item, declaration)) => {
                ctx.add_item(
                    Item::new(
                        id,
                        comment,
                        annotations,
                        relevant_parent_id,
                        ItemKind::Type(item),
                        Some(location.location()),
                    ),
                    declaration,
                    Some(location),
                );
                Ok(id.as_type_id_unchecked())
            }
            Err(ParseError::Continue) => Err(ParseError::Continue),
            Err(ParseError::Recurse) => {
                debug!("Item::from_ty recursing in the ast");
                let mut result = Err(ParseError::Recurse);

                // Need to pop here, otherwise we'll get stuck.
                //
                // TODO: Find a nicer interface, really. Also, the
                // declaration_to_look_for suspiciously shares a lot of
                // logic with ir::context, so we should refactor that.
                if valid_decl {
                    let finished = ctx.finish_parsing();
                    assert_eq!(*finished.decl(), declaration_to_look_for);
                }

                location.visit(|cur| {
                    visit_child(cur, id, ty, parent_id, ctx, &mut result)
                });

                if valid_decl {
                    let partial_ty =
                        PartialType::new(declaration_to_look_for, id);
                    ctx.begin_parsing(partial_ty);
                }

                // If we have recursed into the AST all we know, and we still
                // haven't found what we've got, let's just try and make a named
                // type.
                //
                // This is what happens with some template members, for example.
                if let Err(ParseError::Recurse) = result {
                    warn!(
                        "Unknown type, assuming named template type: \
                         id = {:?}; spelling = {}",
                        id,
                        ty.spelling()
                    );
                    Item::type_param(Some(id), location, ctx)
                        .ok_or(ParseError::Recurse)
                } else {
                    result
                }
            }
        };

        if valid_decl {
            let partial_ty = ctx.finish_parsing();
            assert_eq!(*partial_ty.decl(), declaration_to_look_for);
        }

        ret
    }

    /// A named type is a template parameter, e.g., the `T` in `Foo<T>`. They're always local so
    /// it's the only exception when there's no declaration for a type.
    pub(crate) fn type_param(
        with_id: Option<ItemId>,
        location: clang::Cursor,
        ctx: &mut BindgenContext,
    ) -> Option<TypeId> {
        let ty = location.cur_type();

        debug!(
            "Item::type_param:\n\
             \twith_id = {:?},\n\
             \tty = {} {:?},\n\
             \tlocation: {:?}",
            with_id,
            ty.spelling(),
            ty,
            location
        );

        if ty.kind() != clang_sys::CXType_Unexposed {
            // If the given cursor's type's kind is not Unexposed, then we
            // aren't looking at a template parameter. This check may need to be
            // updated in the future if they start properly exposing template
            // type parameters.
            return None;
        }

        let ty_spelling = ty.spelling();

        // Clang does not expose any information about template type parameters
        // via their clang::Type, nor does it give us their canonical cursors
        // the straightforward way. However, there are three situations from
        // which we can find the definition of the template type parameter, if
        // the cursor is indeed looking at some kind of a template type
        // parameter or use of one:
        //
        // 1. The cursor is pointing at the template type parameter's
        // definition. This is the trivial case.
        //
        //     (kind = TemplateTypeParameter, ...)
        //
        // 2. The cursor is pointing at a TypeRef whose referenced() cursor is
        // situation (1).
        //
        //     (kind = TypeRef,
        //      referenced = (kind = TemplateTypeParameter, ...),
        //      ...)
        //
        // 3. The cursor is pointing at some use of a template type parameter
        // (for example, in a FieldDecl), and this cursor has a child cursor
        // whose spelling is the same as the parent's type's spelling, and whose
        // kind is a TypeRef of the situation (2) variety.
        //
        //    (kind = FieldDecl,
        //     type = (kind = Unexposed,
        //             spelling = "T",
        //             ...),
        //     children =
        //        (kind = TypeRef,
        //         spelling = "T",
        //         referenced = (kind = TemplateTypeParameter,
        //                       spelling = "T",
        //                       ...),
        //         ...)
        //     ...)
        //
        // TODO: The alternative to this hacky pattern matching would be to
        // maintain proper scopes of template parameters while parsing and use
        // de Brujin indices to access template parameters, which clang exposes
        // in the cursor's type's canonical type's spelling:
        // "type-parameter-x-y". That is probably a better approach long-term,
        // but maintaining these scopes properly would require more changes to
        // the whole libclang -> IR parsing code.

        fn is_template_with_spelling(
            refd: &clang::Cursor,
            spelling: &str,
        ) -> bool {
            static ANON_TYPE_PARAM_RE: OnceLock<regex::Regex> = OnceLock::new();
            let anon_type_param_re = ANON_TYPE_PARAM_RE.get_or_init(|| {
                regex::Regex::new(r"^type\-parameter\-\d+\-\d+$").unwrap()
            });

            if refd.kind() != clang_sys::CXCursor_TemplateTypeParameter {
                return false;
            }

            let refd_spelling = refd.spelling();
            refd_spelling == spelling ||
                // Allow for anonymous template parameters.
                (refd_spelling.is_empty() && anon_type_param_re.is_match(spelling.as_ref()))
        }

        let definition = if is_template_with_spelling(&location, &ty_spelling) {
            // Situation (1)
            location
        } else if location.kind() == clang_sys::CXCursor_TypeRef {
            // Situation (2)
            match location.referenced() {
                Some(refd)
                    if is_template_with_spelling(&refd, &ty_spelling) =>
                {
                    refd
                }
                _ => return None,
            }
        } else {
            // Situation (3)
            let mut definition = None;

            location.visit(|child| {
                let child_ty = child.cur_type();
                if child_ty.kind() == clang_sys::CXCursor_TypeRef &&
                    child_ty.spelling() == ty_spelling
                {
                    match child.referenced() {
                        Some(refd)
                            if is_template_with_spelling(
                                &refd,
                                &ty_spelling,
                            ) =>
                        {
                            definition = Some(refd);
                            return clang_sys::CXChildVisit_Break;
                        }
                        _ => {}
                    }
                }

                clang_sys::CXChildVisit_Continue
            });

            definition?
        };
        assert!(is_template_with_spelling(&definition, &ty_spelling));

        // Named types are always parented to the root module. They are never
        // referenced with namespace prefixes, and they can't inherit anything
        // from their parent either, so it is simplest to just hang them off
        // something we know will always exist.
        let parent = ctx.root_module().into();

        if let Some(id) = ctx.get_type_param(&definition) {
            if let Some(with_id) = with_id {
                return Some(ctx.build_ty_wrapper(
                    with_id,
                    id,
                    Some(parent),
                    &ty,
                ));
            } else {
                return Some(id);
            }
        }

        // See tests/headers/const_tparam.hpp and
        // tests/headers/variadic_tname.hpp.
        let name = ty_spelling.replace("const ", "").replace('.', "");

        let id = with_id.unwrap_or_else(|| ctx.next_item_id());
        let item = Item::new(
            id,
            None,
            None,
            parent,
            ItemKind::Type(Type::named(name)),
            Some(location.location()),
        );
        ctx.add_type_param(item, definition);
        Some(id.as_type_id_unchecked())
    }
}

impl ItemCanonicalName for Item {
    fn canonical_name(&self, ctx: &BindgenContext) -> String {
        debug_assert!(
            ctx.in_codegen_phase(),
            "You're not supposed to call this yet"
        );
        self.canonical_name
            .get_or_init(|| {
                let in_namespace = ctx.options().enable_cxx_namespaces ||
                    ctx.options().disable_name_namespacing;

                if in_namespace {
                    self.name(ctx).within_namespaces().get()
                } else {
                    self.name(ctx).get()
                }
            })
            .clone()
    }
}

impl ItemCanonicalPath for Item {
    fn namespace_aware_canonical_path(
        &self,
        ctx: &BindgenContext,
    ) -> Vec<String> {
        let mut path = self.canonical_path(ctx);

        // ASSUMPTION: (disable_name_namespacing && cxx_namespaces)
        // is equivalent to
        // disable_name_namespacing
        if ctx.options().disable_name_namespacing {
            // Only keep the last item in path
            let split_idx = path.len() - 1;
            path = path.split_off(split_idx);
        } else if !ctx.options().enable_cxx_namespaces {
            // Ignore first item "root"
            path = vec![path[1..].join("_")];
        }

        if self.is_constified_enum_module(ctx) {
            path.push(CONSTIFIED_ENUM_MODULE_REPR_NAME.into());
        }

        path
    }

    fn canonical_path(&self, ctx: &BindgenContext) -> Vec<String> {
        self.compute_path(ctx, UserMangled::Yes)
    }
}

/// Whether to use the user-mangled name (mangled by the `item_name` callback or
/// not.
///
/// Most of the callers probably want just yes, but the ones dealing with
/// allowlisting and blocklisting don't.
#[derive(Copy, Clone, Debug, PartialEq)]
enum UserMangled {
    No,
    Yes,
}

/// Builder struct for naming variations, which hold inside different
/// flags for naming options.
#[derive(Debug)]
pub(crate) struct NameOptions<'a> {
    item: &'a Item,
    ctx: &'a BindgenContext,
    within_namespaces: bool,
    user_mangled: UserMangled,
}

impl<'a> NameOptions<'a> {
    /// Construct a new `NameOptions`
    pub(crate) fn new(item: &'a Item, ctx: &'a BindgenContext) -> Self {
        NameOptions {
            item,
            ctx,
            within_namespaces: false,
            user_mangled: UserMangled::Yes,
        }
    }

    /// Construct the name without the item's containing C++ namespaces mangled
    /// into it. In other words, the item's name within the item's namespace.
    pub(crate) fn within_namespaces(&mut self) -> &mut Self {
        self.within_namespaces = true;
        self
    }

    fn user_mangled(&mut self, user_mangled: UserMangled) -> &mut Self {
        self.user_mangled = user_mangled;
        self
    }

    /// Construct a name `String`
    pub(crate) fn get(&self) -> String {
        self.item.real_canonical_name(self.ctx, self)
    }
}
