//! Common context that is passed around during parsing and codegen.

use super::super::time::Timer;
use super::analysis::{
    analyze, as_cannot_derive_set, CannotDerive, DeriveTrait,
    HasDestructorAnalysis, HasFloat, HasTypeParameterInArray,
    HasVtableAnalysis, HasVtableResult, SizednessAnalysis, SizednessResult,
    UsedTemplateParameters,
};
use super::derive::{
    CanDerive, CanDeriveCopy, CanDeriveDebug, CanDeriveDefault, CanDeriveEq,
    CanDeriveHash, CanDeriveOrd, CanDerivePartialEq, CanDerivePartialOrd,
};
use super::function::Function;
use super::int::IntKind;
use super::item::{IsOpaque, Item, ItemAncestors, ItemSet};
use super::item_kind::ItemKind;
use super::module::{Module, ModuleKind};
use super::template::{TemplateInstantiation, TemplateParameters};
use super::traversal::{self, Edge, ItemTraversal};
use super::ty::{FloatKind, Type, TypeKind};
use crate::clang::{self, ABIKind, Cursor};
use crate::codegen::CodegenError;
use crate::BindgenOptions;
use crate::{Entry, HashMap, HashSet};

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeSet, HashMap as StdHashMap};
use std::fs::OpenOptions;
use std::io::Write;
use std::mem;
use std::path::Path;

/// An identifier for some kind of IR item.
#[derive(Debug, Copy, Clone, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ItemId(usize);

/// Declare a newtype around `ItemId` with conversion methods.
macro_rules! item_id_newtype {
    (
        $( #[$attr:meta] )*
        pub(crate) struct $name:ident(ItemId)
        where
            $( #[$checked_attr:meta] )*
            checked = $checked:ident with $check_method:ident,
            $( #[$expected_attr:meta] )*
            expected = $expected:ident,
            $( #[$unchecked_attr:meta] )*
            unchecked = $unchecked:ident;
    ) => {
        $( #[$attr] )*
        #[derive(Debug, Copy, Clone, Eq, PartialOrd, Ord, Hash)]
        pub(crate) struct $name(ItemId);

        impl $name {
            /// Create an `ItemResolver` from this ID.
            #[allow(dead_code)]
            pub(crate) fn into_resolver(self) -> ItemResolver {
                let id: ItemId = self.into();
                id.into()
            }
        }

        impl<T> ::std::cmp::PartialEq<T> for $name
        where
            T: Copy + Into<ItemId>
        {
            fn eq(&self, rhs: &T) -> bool {
                let rhs: ItemId = (*rhs).into();
                self.0 == rhs
            }
        }

        impl From<$name> for ItemId {
            fn from(id: $name) -> ItemId {
                id.0
            }
        }

        impl<'a> From<&'a $name> for ItemId {
            fn from(id: &'a $name) -> ItemId {
                id.0
            }
        }

        #[allow(dead_code)]
        impl ItemId {
            $( #[$checked_attr] )*
            pub(crate) fn $checked(&self, ctx: &BindgenContext) -> Option<$name> {
                if ctx.resolve_item(*self).kind().$check_method() {
                    Some($name(*self))
                } else {
                    None
                }
            }

            $( #[$expected_attr] )*
            pub(crate) fn $expected(&self, ctx: &BindgenContext) -> $name {
                self.$checked(ctx)
                    .expect(concat!(
                        stringify!($expected),
                        " called with ItemId that points to the wrong ItemKind"
                    ))
            }

            $( #[$unchecked_attr] )*
            pub(crate) fn $unchecked(&self) -> $name {
                $name(*self)
            }
        }
    }
}

item_id_newtype! {
    /// An identifier for an `Item` whose `ItemKind` is known to be
    /// `ItemKind::Type`.
    pub(crate) struct TypeId(ItemId)
    where
        /// Convert this `ItemId` into a `TypeId` if its associated item is a type,
        /// otherwise return `None`.
        checked = as_type_id with is_type,

        /// Convert this `ItemId` into a `TypeId`.
        ///
        /// If this `ItemId` does not point to a type, then panic.
        expected = expect_type_id,

        /// Convert this `ItemId` into a `TypeId` without actually checking whether
        /// this ID actually points to a `Type`.
        unchecked = as_type_id_unchecked;
}

item_id_newtype! {
    /// An identifier for an `Item` whose `ItemKind` is known to be
    /// `ItemKind::Module`.
    pub(crate) struct ModuleId(ItemId)
    where
        /// Convert this `ItemId` into a `ModuleId` if its associated item is a
        /// module, otherwise return `None`.
        checked = as_module_id with is_module,

        /// Convert this `ItemId` into a `ModuleId`.
        ///
        /// If this `ItemId` does not point to a module, then panic.
        expected = expect_module_id,

        /// Convert this `ItemId` into a `ModuleId` without actually checking
        /// whether this ID actually points to a `Module`.
        unchecked = as_module_id_unchecked;
}

item_id_newtype! {
    /// An identifier for an `Item` whose `ItemKind` is known to be
    /// `ItemKind::Var`.
    pub(crate) struct VarId(ItemId)
    where
        /// Convert this `ItemId` into a `VarId` if its associated item is a var,
        /// otherwise return `None`.
        checked = as_var_id with is_var,

        /// Convert this `ItemId` into a `VarId`.
        ///
        /// If this `ItemId` does not point to a var, then panic.
        expected = expect_var_id,

        /// Convert this `ItemId` into a `VarId` without actually checking whether
        /// this ID actually points to a `Var`.
        unchecked = as_var_id_unchecked;
}

item_id_newtype! {
    /// An identifier for an `Item` whose `ItemKind` is known to be
    /// `ItemKind::Function`.
    pub(crate) struct FunctionId(ItemId)
    where
        /// Convert this `ItemId` into a `FunctionId` if its associated item is a function,
        /// otherwise return `None`.
        checked = as_function_id with is_function,

        /// Convert this `ItemId` into a `FunctionId`.
        ///
        /// If this `ItemId` does not point to a function, then panic.
        expected = expect_function_id,

        /// Convert this `ItemId` into a `FunctionId` without actually checking whether
        /// this ID actually points to a `Function`.
        unchecked = as_function_id_unchecked;
}

impl From<ItemId> for usize {
    fn from(id: ItemId) -> usize {
        id.0
    }
}

impl ItemId {
    /// Get a numeric representation of this ID.
    pub(crate) fn as_usize(&self) -> usize {
        (*self).into()
    }
}

impl<T> ::std::cmp::PartialEq<T> for ItemId
where
    T: Copy + Into<ItemId>,
{
    fn eq(&self, rhs: &T) -> bool {
        let rhs: ItemId = (*rhs).into();
        self.0 == rhs.0
    }
}

impl<T> CanDeriveDebug for T
where
    T: Copy + Into<ItemId>,
{
    fn can_derive_debug(&self, ctx: &BindgenContext) -> bool {
        ctx.options().derive_debug && ctx.lookup_can_derive_debug(*self)
    }
}

impl<T> CanDeriveDefault for T
where
    T: Copy + Into<ItemId>,
{
    fn can_derive_default(&self, ctx: &BindgenContext) -> bool {
        ctx.options().derive_default && ctx.lookup_can_derive_default(*self)
    }
}

impl<T> CanDeriveCopy for T
where
    T: Copy + Into<ItemId>,
{
    fn can_derive_copy(&self, ctx: &BindgenContext) -> bool {
        ctx.options().derive_copy && ctx.lookup_can_derive_copy(*self)
    }
}

impl<T> CanDeriveHash for T
where
    T: Copy + Into<ItemId>,
{
    fn can_derive_hash(&self, ctx: &BindgenContext) -> bool {
        ctx.options().derive_hash && ctx.lookup_can_derive_hash(*self)
    }
}

impl<T> CanDerivePartialOrd for T
where
    T: Copy + Into<ItemId>,
{
    fn can_derive_partialord(&self, ctx: &BindgenContext) -> bool {
        ctx.options().derive_partialord &&
            ctx.lookup_can_derive_partialeq_or_partialord(*self) ==
                CanDerive::Yes
    }
}

impl<T> CanDerivePartialEq for T
where
    T: Copy + Into<ItemId>,
{
    fn can_derive_partialeq(&self, ctx: &BindgenContext) -> bool {
        ctx.options().derive_partialeq &&
            ctx.lookup_can_derive_partialeq_or_partialord(*self) ==
                CanDerive::Yes
    }
}

impl<T> CanDeriveEq for T
where
    T: Copy + Into<ItemId>,
{
    fn can_derive_eq(&self, ctx: &BindgenContext) -> bool {
        ctx.options().derive_eq &&
            ctx.lookup_can_derive_partialeq_or_partialord(*self) ==
                CanDerive::Yes &&
            !ctx.lookup_has_float(*self)
    }
}

impl<T> CanDeriveOrd for T
where
    T: Copy + Into<ItemId>,
{
    fn can_derive_ord(&self, ctx: &BindgenContext) -> bool {
        ctx.options().derive_ord &&
            ctx.lookup_can_derive_partialeq_or_partialord(*self) ==
                CanDerive::Yes &&
            !ctx.lookup_has_float(*self)
    }
}

/// A key used to index a resolved type, so we only process it once.
///
/// This is almost always a USR string (an unique identifier generated by
/// clang), but it can also be the canonical declaration if the type is unnamed,
/// in which case clang may generate the same USR for multiple nested unnamed
/// types.
#[derive(Eq, PartialEq, Hash, Debug)]
enum TypeKey {
    Usr(String),
    Declaration(Cursor),
}

/// A context used during parsing and generation of structs.
#[derive(Debug)]
pub(crate) struct BindgenContext {
    /// The map of all the items parsed so far, keyed off `ItemId`.
    items: Vec<Option<Item>>,

    /// Clang USR to type map. This is needed to be able to associate types with
    /// item ids during parsing.
    types: HashMap<TypeKey, TypeId>,

    /// Maps from a cursor to the item ID of the named template type parameter
    /// for that cursor.
    type_params: HashMap<Cursor, TypeId>,

    /// A cursor to module map. Similar reason than above.
    modules: HashMap<Cursor, ModuleId>,

    /// The root module, this is guaranteed to be an item of kind Module.
    root_module: ModuleId,

    /// Current module being traversed.
    current_module: ModuleId,

    /// A `HashMap` keyed on a type definition, and whose value is the parent ID
    /// of the declaration.
    ///
    /// This is used to handle the cases where the semantic and the lexical
    /// parents of the cursor differ, like when a nested class is defined
    /// outside of the parent class.
    semantic_parents: HashMap<Cursor, ItemId>,

    /// A stack with the current type declarations and types we're parsing. This
    /// is needed to avoid infinite recursion when parsing a type like:
    ///
    /// struct c { struct c* next; };
    ///
    /// This means effectively, that a type has a potential ID before knowing if
    /// it's a correct type. But that's not important in practice.
    ///
    /// We could also use the `types` `HashMap`, but my intention with it is that
    /// only valid types and declarations end up there, and this could
    /// potentially break that assumption.
    currently_parsed_types: Vec<PartialType>,

    /// A map with all the already parsed macro names. This is done to avoid
    /// hard errors while parsing duplicated macros, as well to allow macro
    /// expression parsing.
    ///
    /// This needs to be an `std::HashMap` because the `cexpr` API requires it.
    parsed_macros: StdHashMap<Vec<u8>, cexpr::expr::EvalResult>,

    /// A map with all include locations.
    ///
    /// This is needed so that items are created in the order they are defined in.
    ///
    /// The key is the included file, the value is a pair of the source file and
    /// the position of the `#include` directive in the source file.
    includes: StdHashMap<String, (String, usize)>,

    /// A set of all the included filenames.
    deps: BTreeSet<Box<str>>,

    /// The active replacements collected from replaces="xxx" annotations.
    replacements: HashMap<Vec<String>, ItemId>,

    collected_typerefs: bool,

    in_codegen: bool,

    /// The translation unit for parsing.
    translation_unit: clang::TranslationUnit,

    /// The translation unit for macro fallback parsing.
    fallback_tu: Option<clang::FallbackTranslationUnit>,

    /// Target information that can be useful for some stuff.
    target_info: clang::TargetInfo,

    /// The options given by the user via cli or other medium.
    options: BindgenOptions,

    /// Whether an opaque array was generated
    generated_opaque_array: Cell<bool>,

    /// Whether a bindgen complex was generated
    generated_bindgen_complex: Cell<bool>,

    /// Whether a bindgen float16 was generated
    generated_bindgen_float16: Cell<bool>,

    /// The set of `ItemId`s that are allowlisted. This the very first thing
    /// computed after parsing our IR, and before running any of our analyses.
    allowlisted: Option<ItemSet>,

    /// Cache for calls to `ParseCallbacks::blocklisted_type_implements_trait`
    blocklisted_types_implement_traits:
        RefCell<HashMap<DeriveTrait, HashMap<ItemId, CanDerive>>>,

    /// The set of `ItemId`s that are allowlisted for code generation _and_ that
    /// we should generate accounting for the codegen options.
    ///
    /// It's computed right after computing the allowlisted items.
    codegen_items: Option<ItemSet>,

    /// Map from an item's ID to the set of template parameter items that it
    /// uses. See `ir::named` for more details. Always `Some` during the codegen
    /// phase.
    used_template_parameters: Option<HashMap<ItemId, ItemSet>>,

    /// The set of `TypeKind::Comp` items found during parsing that need their
    /// bitfield allocation units computed. Drained in `compute_bitfield_units`.
    need_bitfield_allocation: Vec<ItemId>,

    /// The set of enums that are defined by a pair of `enum` and `typedef`,
    /// which is legal in C (but not C++).
    ///
    /// ```c++
    /// // in either order
    /// enum Enum { Variants... };
    /// typedef int16_t Enum;
    /// ```
    ///
    /// The stored `ItemId` is that of the `TypeKind::Enum`, not of the
    /// `TypeKind::Alias`.
    ///
    /// This is populated when we enter codegen by `compute_enum_typedef_combos`
    /// and is always `None` before that and `Some` after.
    enum_typedef_combos: Option<HashSet<ItemId>>,

    /// The set of (`ItemId`s of) types that can't derive debug.
    ///
    /// This is populated when we enter codegen by `compute_cannot_derive_debug`
    /// and is always `None` before that and `Some` after.
    cannot_derive_debug: Option<HashSet<ItemId>>,

    /// The set of (`ItemId`s of) types that can't derive default.
    ///
    /// This is populated when we enter codegen by `compute_cannot_derive_default`
    /// and is always `None` before that and `Some` after.
    cannot_derive_default: Option<HashSet<ItemId>>,

    /// The set of (`ItemId`s of) types that can't derive copy.
    ///
    /// This is populated when we enter codegen by `compute_cannot_derive_copy`
    /// and is always `None` before that and `Some` after.
    cannot_derive_copy: Option<HashSet<ItemId>>,

    /// The set of (`ItemId`s of) types that can't derive hash.
    ///
    /// This is populated when we enter codegen by `compute_can_derive_hash`
    /// and is always `None` before that and `Some` after.
    cannot_derive_hash: Option<HashSet<ItemId>>,

    /// The map why specified `ItemId`s of) types that can't derive hash.
    ///
    /// This is populated when we enter codegen by
    /// `compute_cannot_derive_partialord_partialeq_or_eq` and is always `None`
    /// before that and `Some` after.
    cannot_derive_partialeq_or_partialord: Option<HashMap<ItemId, CanDerive>>,

    /// The sizedness of types.
    ///
    /// This is populated by `compute_sizedness` and is always `None` before
    /// that function is invoked and `Some` afterwards.
    sizedness: Option<HashMap<TypeId, SizednessResult>>,

    /// The set of (`ItemId's of`) types that has vtable.
    ///
    /// Populated when we enter codegen by `compute_has_vtable`; always `None`
    /// before that and `Some` after.
    have_vtable: Option<HashMap<ItemId, HasVtableResult>>,

    /// The set of (`ItemId's of`) types that has destructor.
    ///
    /// Populated when we enter codegen by `compute_has_destructor`; always `None`
    /// before that and `Some` after.
    have_destructor: Option<HashSet<ItemId>>,

    /// The set of (`ItemId's of`) types that has array.
    ///
    /// Populated when we enter codegen by `compute_has_type_param_in_array`; always `None`
    /// before that and `Some` after.
    has_type_param_in_array: Option<HashSet<ItemId>>,

    /// The set of (`ItemId's of`) types that has float.
    ///
    /// Populated when we enter codegen by `compute_has_float`; always `None`
    /// before that and `Some` after.
    has_float: Option<HashSet<ItemId>>,
}

/// A traversal of allowlisted items.
struct AllowlistedItemsTraversal<'ctx> {
    ctx: &'ctx BindgenContext,
    traversal: ItemTraversal<'ctx, ItemSet, Vec<ItemId>>,
}

impl Iterator for AllowlistedItemsTraversal<'_> {
    type Item = ItemId;

    fn next(&mut self) -> Option<ItemId> {
        loop {
            let id = self.traversal.next()?;

            if self.ctx.resolve_item(id).is_blocklisted(self.ctx) {
                continue;
            }

            return Some(id);
        }
    }
}

impl<'ctx> AllowlistedItemsTraversal<'ctx> {
    /// Construct a new allowlisted items traversal.
    pub(crate) fn new<R>(
        ctx: &'ctx BindgenContext,
        roots: R,
        predicate: for<'a> fn(&'a BindgenContext, Edge) -> bool,
    ) -> Self
    where
        R: IntoIterator<Item = ItemId>,
    {
        AllowlistedItemsTraversal {
            ctx,
            traversal: ItemTraversal::new(ctx, roots, predicate),
        }
    }
}

impl BindgenContext {
    /// Construct the context for the given `options`.
    pub(crate) fn new(
        options: BindgenOptions,
        input_unsaved_files: &[clang::UnsavedFile],
    ) -> Self {
        // TODO(emilio): Use the CXTargetInfo here when available.
        //
        // see: https://reviews.llvm.org/D32389
        let index = clang::Index::new(false, true);

        let parse_options =
            clang_sys::CXTranslationUnit_DetailedPreprocessingRecord;

        let translation_unit = {
            let _t =
                Timer::new("translation_unit").with_output(options.time_phases);

            clang::TranslationUnit::parse(
                &index,
                "",
                &options.clang_args,
                input_unsaved_files,
                parse_options,
            ).expect("libclang error; possible causes include:
- Invalid flag syntax
- Unrecognized flags
- Invalid flag arguments
- File I/O errors
- Host vs. target architecture mismatch
If you encounter an error missing from this list, please file an issue or a PR!")
        };

        let target_info = clang::TargetInfo::new(&translation_unit);
        let root_module = Self::build_root_module(ItemId(0));
        let root_module_id = root_module.id().as_module_id_unchecked();

        // depfiles need to include the explicitly listed headers too
        let deps = options.input_headers.iter().cloned().collect();

        BindgenContext {
            items: vec![Some(root_module)],
            includes: Default::default(),
            deps,
            types: Default::default(),
            type_params: Default::default(),
            modules: Default::default(),
            root_module: root_module_id,
            current_module: root_module_id,
            semantic_parents: Default::default(),
            currently_parsed_types: vec![],
            parsed_macros: Default::default(),
            replacements: Default::default(),
            collected_typerefs: false,
            in_codegen: false,
            translation_unit,
            fallback_tu: None,
            target_info,
            options,
            generated_bindgen_complex: Cell::new(false),
            generated_bindgen_float16: Cell::new(false),
            generated_opaque_array: Cell::new(false),
            allowlisted: None,
            blocklisted_types_implement_traits: Default::default(),
            codegen_items: None,
            used_template_parameters: None,
            need_bitfield_allocation: Default::default(),
            enum_typedef_combos: None,
            cannot_derive_debug: None,
            cannot_derive_default: None,
            cannot_derive_copy: None,
            cannot_derive_hash: None,
            cannot_derive_partialeq_or_partialord: None,
            sizedness: None,
            have_vtable: None,
            have_destructor: None,
            has_type_param_in_array: None,
            has_float: None,
        }
    }

    /// Returns `true` if the target architecture is wasm32
    pub(crate) fn is_target_wasm32(&self) -> bool {
        self.target_info.triple.starts_with("wasm32-")
    }

    /// Creates a timer for the current bindgen phase. If `time_phases` is `true`,
    /// the timer will print to stderr when it is dropped, otherwise it will do
    /// nothing.
    pub(crate) fn timer<'a>(&self, name: &'a str) -> Timer<'a> {
        Timer::new(name).with_output(self.options.time_phases)
    }

    /// Returns the pointer width to use for the target for the current
    /// translation.
    pub(crate) fn target_pointer_size(&self) -> usize {
        self.target_info.pointer_width / 8
    }

    /// Returns the ABI, which is mostly useful for determining the mangling kind.
    pub(crate) fn abi_kind(&self) -> ABIKind {
        self.target_info.abi
    }

    /// Get the stack of partially parsed types that we are in the middle of
    /// parsing.
    pub(crate) fn currently_parsed_types(&self) -> &[PartialType] {
        &self.currently_parsed_types[..]
    }

    /// Begin parsing the given partial type, and push it onto the
    /// `currently_parsed_types` stack so that we won't infinite recurse if we
    /// run into a reference to it while parsing it.
    pub(crate) fn begin_parsing(&mut self, partial_ty: PartialType) {
        self.currently_parsed_types.push(partial_ty);
    }

    /// Finish parsing the current partial type, pop it off the
    /// `currently_parsed_types` stack, and return it.
    pub(crate) fn finish_parsing(&mut self) -> PartialType {
        self.currently_parsed_types.pop().expect(
            "should have been parsing a type, if we finished parsing a type",
        )
    }

    /// Add the location of the `#include` directive for the `included_file`.
    pub(crate) fn add_include(
        &mut self,
        source_file: String,
        included_file: String,
        offset: usize,
    ) {
        self.includes
            .entry(included_file)
            .or_insert((source_file, offset));
    }

    /// Get the location of the first `#include` directive for the `included_file`.
    pub(crate) fn included_file_location(
        &self,
        included_file: &str,
    ) -> Option<(String, usize)> {
        self.includes.get(included_file).cloned()
    }

    /// Add an included file.
    pub(crate) fn add_dep(&mut self, dep: Box<str>) {
        self.deps.insert(dep);
    }

    /// Get any included files.
    pub(crate) fn deps(&self) -> &BTreeSet<Box<str>> {
        &self.deps
    }

    /// Define a new item.
    ///
    /// This inserts it into the internal items set, and its type into the
    /// internal types set.
    pub(crate) fn add_item(
        &mut self,
        item: Item,
        declaration: Option<Cursor>,
        location: Option<Cursor>,
    ) {
        debug!("BindgenContext::add_item({item:?}, declaration: {declaration:?}, loc: {location:?}");
        debug_assert!(
            declaration.is_some() ||
                !item.kind().is_type() ||
                item.kind().expect_type().is_builtin_or_type_param() ||
                item.kind().expect_type().is_opaque(self, &item) ||
                item.kind().expect_type().is_unresolved_ref(),
            "Adding a type without declaration?"
        );

        let id = item.id();
        let is_type = item.kind().is_type();
        let is_unnamed = is_type && item.expect_type().name().is_none();
        let is_template_instantiation =
            is_type && item.expect_type().is_template_instantiation();

        if item.id() != self.root_module {
            self.add_item_to_module(&item);
        }

        if is_type && item.expect_type().is_comp() {
            self.need_bitfield_allocation.push(id);
        }

        let old_item = mem::replace(&mut self.items[id.0], Some(item));
        assert!(
            old_item.is_none(),
            "should not have already associated an item with the given id"
        );

        // Unnamed items can have an USR, but they can't be referenced from
        // other sites explicitly and the USR can match if the unnamed items are
        // nested, so don't bother tracking them.
        if !is_type || is_template_instantiation {
            return;
        }
        if let Some(mut declaration) = declaration {
            if !declaration.is_valid() {
                if let Some(location) = location {
                    if location.is_template_like() {
                        declaration = location;
                    }
                }
            }
            declaration = declaration.canonical();
            if !declaration.is_valid() {
                // This could happen, for example, with types like `int*` or
                // similar.
                //
                // Fortunately, we don't care about those types being
                // duplicated, so we can just ignore them.
                debug!(
                    "Invalid declaration {declaration:?} found for type {:?}",
                    self.resolve_item_fallible(id)
                        .unwrap()
                        .kind()
                        .expect_type()
                );
                return;
            }

            let key = if is_unnamed {
                TypeKey::Declaration(declaration)
            } else if let Some(usr) = declaration.usr() {
                TypeKey::Usr(usr)
            } else {
                warn!("Valid declaration with no USR: {declaration:?}, {location:?}");
                TypeKey::Declaration(declaration)
            };

            let old = self.types.insert(key, id.as_type_id_unchecked());
            debug_assert_eq!(old, None);
        }
    }

    /// Ensure that every item (other than the root module) is in a module's
    /// children list. This is to make sure that every allowlisted item get's
    /// codegen'd, even if its parent is not allowlisted. See issue #769 for
    /// details.
    fn add_item_to_module(&mut self, item: &Item) {
        assert!(item.id() != self.root_module);
        assert!(self.resolve_item_fallible(item.id()).is_none());

        if let Some(ref mut parent) = self.items[item.parent_id().0] {
            if let Some(module) = parent.as_module_mut() {
                debug!(
                    "add_item_to_module: adding {:?} as child of parent module {:?}",
                    item.id(),
                    item.parent_id()
                );

                module.children_mut().insert(item.id());
                return;
            }
        }

        debug!(
            "add_item_to_module: adding {:?} as child of current module {:?}",
            item.id(),
            self.current_module
        );

        self.items[(self.current_module.0).0]
            .as_mut()
            .expect("Should always have an item for self.current_module")
            .as_module_mut()
            .expect("self.current_module should always be a module")
            .children_mut()
            .insert(item.id());
    }

    /// Add a new named template type parameter to this context's item set.
    pub(crate) fn add_type_param(&mut self, item: Item, definition: Cursor) {
        debug!("BindgenContext::add_type_param: item = {item:?}; definition = {definition:?}");

        assert!(
            item.expect_type().is_type_param(),
            "Should directly be a named type, not a resolved reference or anything"
        );
        assert_eq!(
            definition.kind(),
            clang_sys::CXCursor_TemplateTypeParameter
        );

        self.add_item_to_module(&item);

        let id = item.id();
        let old_item = mem::replace(&mut self.items[id.0], Some(item));
        assert!(
            old_item.is_none(),
            "should not have already associated an item with the given id"
        );

        let old_named_ty = self
            .type_params
            .insert(definition, id.as_type_id_unchecked());
        assert!(
            old_named_ty.is_none(),
            "should not have already associated a named type with this id"
        );
    }

    /// Get the named type defined at the given cursor location, if we've
    /// already added one.
    pub(crate) fn get_type_param(&self, definition: &Cursor) -> Option<TypeId> {
        assert_eq!(
            definition.kind(),
            clang_sys::CXCursor_TemplateTypeParameter
        );
        self.type_params.get(definition).copied()
    }

    // TODO: Move all this syntax crap to other part of the code.

    /// Mangles a name so it doesn't conflict with any keyword.
    #[rustfmt::skip]
    pub(crate) fn rust_mangle<'a>(&self, name: &'a str) -> Cow<'a, str> {
        if name.contains('@') ||
            name.contains('?') ||
            name.contains('$') ||
            matches!(
                name,
                "abstract" | "alignof" | "as" | "async" | "await" | "become" |
                    "box" | "break" | "const" | "continue" | "crate" | "do" |
                    "dyn" | "else" | "enum" | "extern" | "false" | "final" |
                    "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" |
                    "macro" | "match" | "mod" | "move" | "mut" | "offsetof" |
                    "override" | "priv" | "proc" | "pub" | "pure" | "ref" |
                    "return" | "Self" | "self" | "sizeof" | "static" |
                    "struct" | "super" | "trait" | "true" | "try" | "type" | "typeof" |
                    "unsafe" | "unsized" | "use" | "virtual" | "where" |
                    "while" | "yield" | "str" | "bool" | "f32" | "f64" |
                    "usize" | "isize" | "u128" | "i128" | "u64" | "i64" |
                    "u32" | "i32" | "u16" | "i16" | "u8" | "i8" | "_"
            )
        {
            let mut s = name.to_owned();
            s = s.replace('@', "_");
            s = s.replace('?', "_");
            s = s.replace('$', "_");
            s.push('_');
            return Cow::Owned(s);
        }
        Cow::Borrowed(name)
    }

    /// Returns a mangled name as a rust identifier.
    pub(crate) fn rust_ident<S>(&self, name: S) -> Ident
    where
        S: AsRef<str>,
    {
        self.rust_ident_raw(self.rust_mangle(name.as_ref()))
    }

    /// Returns a mangled name as a rust identifier.
    pub(crate) fn rust_ident_raw<T>(&self, name: T) -> Ident
    where
        T: AsRef<str>,
    {
        Ident::new(name.as_ref(), Span::call_site())
    }

    /// Iterate over all items that have been defined.
    pub(crate) fn items(&self) -> impl Iterator<Item = (ItemId, &Item)> {
        self.items.iter().enumerate().filter_map(|(index, item)| {
            let item = item.as_ref()?;
            Some((ItemId(index), item))
        })
    }

    /// Have we collected all unresolved type references yet?
    pub(crate) fn collected_typerefs(&self) -> bool {
        self.collected_typerefs
    }

    /// Gather all the unresolved type references.
    fn collect_typerefs(
        &mut self,
    ) -> Vec<(ItemId, clang::Type, Cursor, Option<ItemId>)> {
        debug_assert!(!self.collected_typerefs);
        self.collected_typerefs = true;
        let mut typerefs = vec![];

        for (id, item) in self.items() {
            let kind = item.kind();
            let Some(ty) = kind.as_type() else { continue };

            if let TypeKind::UnresolvedTypeRef(ref ty, loc, parent_id) =
                *ty.kind()
            {
                typerefs.push((id, *ty, loc, parent_id));
            };
        }
        typerefs
    }

    /// Collect all of our unresolved type references and resolve them.
    fn resolve_typerefs(&mut self) {
        let _t = self.timer("resolve_typerefs");

        let typerefs = self.collect_typerefs();

        for (id, ty, loc, parent_id) in typerefs {
            let _resolved =
                {
                    let resolved = Item::from_ty(&ty, loc, parent_id, self)
                    .unwrap_or_else(|_| {
                        warn!("Could not resolve type reference, falling back \
                               to opaque blob");
                        Item::new_opaque_type(self.next_item_id(), &ty, self)
                    });

                    let item = self.items[id.0].as_mut().unwrap();
                    *item.kind_mut().as_type_mut().unwrap().kind_mut() =
                        TypeKind::ResolvedTypeRef(resolved);
                    resolved
                };

            // Something in the STL is trolling me. I don't need this assertion
            // right now, but worth investigating properly once this lands.
            //
            // debug_assert!(self.items.get(&resolved).is_some(), "How?");
            //
            // if let Some(parent_id) = parent_id {
            //     assert_eq!(self.items[&resolved].parent_id(), parent_id);
            // }
        }
    }

    /// Temporarily loan `Item` with the given `ItemId`. This provides means to
    /// mutably borrow `Item` while having a reference to `BindgenContext`.
    ///
    /// `Item` with the given `ItemId` is removed from the context, given
    /// closure is executed and then `Item` is placed back.
    ///
    /// # Panics
    ///
    /// Panics if attempt to resolve given `ItemId` inside the given
    /// closure is made.
    fn with_loaned_item<F, T>(&mut self, id: ItemId, f: F) -> T
    where
        F: (FnOnce(&BindgenContext, &mut Item) -> T),
    {
        let mut item = self.items[id.0].take().unwrap();

        let result = f(self, &mut item);

        let existing = mem::replace(&mut self.items[id.0], Some(item));
        assert!(existing.is_none());

        result
    }

    /// Compute the bitfield allocation units for all `TypeKind::Comp` items we
    /// parsed.
    fn compute_bitfield_units(&mut self) {
        let _t = self.timer("compute_bitfield_units");

        assert!(self.collected_typerefs());

        let need_bitfield_allocation =
            mem::take(&mut self.need_bitfield_allocation);
        for id in need_bitfield_allocation {
            self.with_loaned_item(id, |ctx, item| {
                let ty = item.kind_mut().as_type_mut().unwrap();
                let layout = ty.layout(ctx);
                ty.as_comp_mut()
                    .unwrap()
                    .compute_bitfield_units(ctx, layout.as_ref());
            });
        }
    }

    /// Assign a new generated name for each anonymous field.
    fn deanonymize_fields(&mut self) {
        let _t = self.timer("deanonymize_fields");

        let comp_item_ids: Vec<ItemId> = self
            .items()
            .filter_map(|(id, item)| {
                if item.kind().as_type()?.is_comp() {
                    return Some(id);
                }
                None
            })
            .collect();

        for id in comp_item_ids {
            self.with_loaned_item(id, |ctx, item| {
                item.kind_mut()
                    .as_type_mut()
                    .unwrap()
                    .as_comp_mut()
                    .unwrap()
                    .deanonymize_fields(ctx);
            });
        }
    }

    /// Iterate over all items and replace any item that has been named in a
    /// `replaces="SomeType"` annotation with the replacement type.
    fn process_replacements(&mut self) {
        let _t = self.timer("process_replacements");
        if self.replacements.is_empty() {
            debug!("No replacements to process");
            return;
        }

        // FIXME: This is linear, but the replaces="xxx" annotation was already
        // there, and for better or worse it's useful, sigh...
        //
        // We leverage the ResolvedTypeRef thing, though, which is cool :P.

        let mut replacements = vec![];

        for (id, item) in self.items() {
            if item.annotations().use_instead_of().is_some() {
                continue;
            }

            // Calls to `canonical_name` are expensive, so eagerly filter out
            // items that cannot be replaced.
            let Some(ty) = item.kind().as_type() else {
                continue;
            };

            match *ty.kind() {
                TypeKind::Comp(..) |
                TypeKind::TemplateAlias(..) |
                TypeKind::Enum(..) |
                TypeKind::Alias(..) => {}
                _ => continue,
            }

            let path = item.path_for_allowlisting(self);
            let replacement = self.replacements.get(&path[1..]);

            if let Some(replacement) = replacement {
                if *replacement != id {
                    // We set this just after parsing the annotation. It's
                    // very unlikely, but this can happen.
                    if self.resolve_item_fallible(*replacement).is_some() {
                        replacements.push((
                            id.expect_type_id(self),
                            replacement.expect_type_id(self),
                        ));
                    }
                }
            }
        }

        for (id, replacement_id) in replacements {
            debug!("Replacing {id:?} with {replacement_id:?}");
            let new_parent = {
                let item_id: ItemId = id.into();
                let item = self.items[item_id.0].as_mut().unwrap();
                *item.kind_mut().as_type_mut().unwrap().kind_mut() =
                    TypeKind::ResolvedTypeRef(replacement_id);
                item.parent_id()
            };

            // Relocate the replacement item from where it was declared, to
            // where the thing it is replacing was declared.
            //
            // First, we'll make sure that its parent ID is correct.

            let old_parent = self.resolve_item(replacement_id).parent_id();
            if new_parent == old_parent {
                // Same parent and therefore also same containing
                // module. Nothing to do here.
                continue;
            }

            let replacement_item_id: ItemId = replacement_id.into();
            self.items[replacement_item_id.0]
                .as_mut()
                .unwrap()
                .set_parent_for_replacement(new_parent);

            // Second, make sure that it is in the correct module's children
            // set.

            let old_module = {
                let immut_self = &*self;
                old_parent
                    .ancestors(immut_self)
                    .chain(Some(immut_self.root_module.into()))
                    .find(|id| {
                        let item = immut_self.resolve_item(*id);
                        item.as_module().is_some_and(|m| {
                            m.children().contains(&replacement_id.into())
                        })
                    })
            };
            let old_module = old_module
                .expect("Every replacement item should be in a module");

            let new_module = {
                let immut_self = &*self;
                new_parent
                    .ancestors(immut_self)
                    .find(|id| immut_self.resolve_item(*id).is_module())
            };
            let new_module =
                new_module.unwrap_or_else(|| self.root_module.into());

            if new_module == old_module {
                // Already in the correct module.
                continue;
            }

            self.items[old_module.0]
                .as_mut()
                .unwrap()
                .as_module_mut()
                .unwrap()
                .children_mut()
                .remove(&replacement_id.into());

            self.items[new_module.0]
                .as_mut()
                .unwrap()
                .as_module_mut()
                .unwrap()
                .children_mut()
                .insert(replacement_id.into());
        }
    }

    /// Enter the code generation phase, invoke the given callback `cb`, and
    /// leave the code generation phase.
    pub(crate) fn gen<F, Out>(
        mut self,
        cb: F,
    ) -> Result<(Out, BindgenOptions), CodegenError>
    where
        F: FnOnce(&Self) -> Result<Out, CodegenError>,
    {
        self.in_codegen = true;

        self.resolve_typerefs();
        self.compute_bitfield_units();
        self.process_replacements();

        self.deanonymize_fields();

        self.assert_no_dangling_references();

        // Compute the allowlisted set after processing replacements and
        // resolving type refs, as those are the final mutations of the IR
        // graph, and their completion means that the IR graph is now frozen.
        self.compute_allowlisted_and_codegen_items();

        // Make sure to do this after processing replacements, since that messes
        // with the parentage and module children, and we want to assert that it
        // messes with them correctly.
        self.assert_every_item_in_a_module();

        self.compute_has_vtable();
        self.compute_sizedness();
        self.compute_has_destructor();
        self.find_used_template_parameters();
        self.compute_enum_typedef_combos();
        self.compute_cannot_derive_debug();
        self.compute_cannot_derive_default();
        self.compute_cannot_derive_copy();
        self.compute_has_type_param_in_array();
        self.compute_has_float();
        self.compute_cannot_derive_hash();
        self.compute_cannot_derive_partialord_partialeq_or_eq();

        let ret = cb(&self)?;
        Ok((ret, self.options))
    }

    /// When the `__testing_only_extra_assertions` feature is enabled, this
    /// function walks the IR graph and asserts that we do not have any edges
    /// referencing an `ItemId` for which we do not have an associated IR item.
    fn assert_no_dangling_references(&self) {
        if cfg!(feature = "__testing_only_extra_assertions") {
            for _ in self.assert_no_dangling_item_traversal() {
                // The iterator's next method does the asserting for us.
            }
        }
    }

    fn assert_no_dangling_item_traversal(
        &self,
    ) -> traversal::AssertNoDanglingItemsTraversal {
        assert!(self.in_codegen_phase());
        assert!(self.current_module == self.root_module);

        let roots = self.items().map(|(id, _)| id);
        traversal::AssertNoDanglingItemsTraversal::new(
            self,
            roots,
            traversal::all_edges,
        )
    }

    /// When the `__testing_only_extra_assertions` feature is enabled, walk over
    /// every item and ensure that it is in the children set of one of its
    /// module ancestors.
    fn assert_every_item_in_a_module(&self) {
        if cfg!(feature = "__testing_only_extra_assertions") {
            assert!(self.in_codegen_phase());
            assert!(self.current_module == self.root_module);

            for (id, _item) in self.items() {
                if id == self.root_module {
                    continue;
                }

                assert!(
                    {
                        let id = id
                            .into_resolver()
                            .through_type_refs()
                            .through_type_aliases()
                            .resolve(self)
                            .id();
                        id.ancestors(self)
                            .chain(Some(self.root_module.into()))
                            .any(|ancestor| {
                                debug!("Checking if {id:?} is a child of {ancestor:?}");
                                self.resolve_item(ancestor)
                                    .as_module()
                                    .is_some_and(|m| m.children().contains(&id))
                            })
                    },
                    "{id:?} should be in some ancestor module's children set"
                );
            }
        }
    }

    /// Compute for every type whether it is sized or not, and whether it is
    /// sized or not as a base class.
    fn compute_sizedness(&mut self) {
        let _t = self.timer("compute_sizedness");
        assert!(self.sizedness.is_none());
        self.sizedness = Some(analyze::<SizednessAnalysis>(self));
    }

    /// Look up whether the type with the given ID is sized or not.
    pub(crate) fn lookup_sizedness(&self, id: TypeId) -> SizednessResult {
        assert!(
            self.in_codegen_phase(),
            "We only compute sizedness after we've entered codegen"
        );

        self.sizedness
            .as_ref()
            .unwrap()
            .get(&id)
            .copied()
            .unwrap_or(SizednessResult::ZeroSized)
    }

    /// Compute whether the type has vtable.
    fn compute_has_vtable(&mut self) {
        let _t = self.timer("compute_has_vtable");
        assert!(self.have_vtable.is_none());
        self.have_vtable = Some(analyze::<HasVtableAnalysis>(self));
    }

    /// Look up whether the item with `id` has vtable or not.
    pub(crate) fn lookup_has_vtable(&self, id: TypeId) -> HasVtableResult {
        assert!(
            self.in_codegen_phase(),
            "We only compute vtables when we enter codegen"
        );

        // Look up the computed value for whether the item with `id` has a
        // vtable or not.
        self.have_vtable
            .as_ref()
            .unwrap()
            .get(&id.into())
            .copied()
            .unwrap_or(HasVtableResult::No)
    }

    /// Compute whether the type has a destructor.
    fn compute_has_destructor(&mut self) {
        let _t = self.timer("compute_has_destructor");
        assert!(self.have_destructor.is_none());
        self.have_destructor = Some(analyze::<HasDestructorAnalysis>(self));
    }

    /// Look up whether the item with `id` has a destructor.
    pub(crate) fn lookup_has_destructor(&self, id: TypeId) -> bool {
        assert!(
            self.in_codegen_phase(),
            "We only compute destructors when we enter codegen"
        );

        self.have_destructor.as_ref().unwrap().contains(&id.into())
    }

    fn find_used_template_parameters(&mut self) {
        let _t = self.timer("find_used_template_parameters");
        if self.options.allowlist_recursively {
            let used_params = analyze::<UsedTemplateParameters>(self);
            self.used_template_parameters = Some(used_params);
        } else {
            // If you aren't recursively allowlisting, then we can't really make
            // any sense of template parameter usage, and you're on your own.
            let mut used_params = HashMap::default();
            for &id in self.allowlisted_items() {
                used_params.entry(id).or_insert_with(|| {
                    id.self_template_params(self)
                        .into_iter()
                        .map(|p| p.into())
                        .collect()
                });
            }
            self.used_template_parameters = Some(used_params);
        }
    }

    /// Return `true` if `item` uses the given `template_param`, `false`
    /// otherwise.
    ///
    /// This method may only be called during the codegen phase, because the
    /// template usage information is only computed as we enter the codegen
    /// phase.
    ///
    /// If the item is blocklisted, then we say that it always uses the template
    /// parameter. This is a little subtle. The template parameter usage
    /// analysis only considers allowlisted items, and if any blocklisted item
    /// shows up in the generated bindings, it is the user's responsibility to
    /// manually provide a definition for them. To give them the most
    /// flexibility when doing that, we assume that they use every template
    /// parameter and always pass template arguments through in instantiations.
    pub(crate) fn uses_template_parameter(
        &self,
        item: ItemId,
        template_param: TypeId,
    ) -> bool {
        assert!(
            self.in_codegen_phase(),
            "We only compute template parameter usage as we enter codegen"
        );

        if self.resolve_item(item).is_blocklisted(self) {
            return true;
        }

        let template_param = template_param
            .into_resolver()
            .through_type_refs()
            .through_type_aliases()
            .resolve(self)
            .id();

        self.used_template_parameters
            .as_ref()
            .expect("should have found template parameter usage if we're in codegen")
            .get(&item).is_some_and(|items_used_params| items_used_params.contains(&template_param))
    }

    /// Return `true` if `item` uses any unbound, generic template parameters,
    /// `false` otherwise.
    ///
    /// Has the same restrictions that `uses_template_parameter` has.
    pub(crate) fn uses_any_template_parameters(&self, item: ItemId) -> bool {
        assert!(
            self.in_codegen_phase(),
            "We only compute template parameter usage as we enter codegen"
        );

        self.used_template_parameters
            .as_ref()
            .expect(
                "should have template parameter usage info in codegen phase",
            )
            .get(&item)
            .is_some_and(|used| !used.is_empty())
    }

    // This deserves a comment. Builtin types don't get a valid declaration, so
    // we can't add it to the cursor->type map.
    //
    // That being said, they're not generated anyway, and are few, so the
    // duplication and special-casing is fine.
    //
    // If at some point we care about the memory here, probably a map TypeKind
    // -> builtin type ItemId would be the best to improve that.
    fn add_builtin_item(&mut self, item: Item) {
        debug!("add_builtin_item: item = {item:?}");
        debug_assert!(item.kind().is_type());
        self.add_item_to_module(&item);
        let id = item.id();
        let old_item = mem::replace(&mut self.items[id.0], Some(item));
        assert!(old_item.is_none(), "Inserted type twice?");
    }

    fn build_root_module(id: ItemId) -> Item {
        let module = Module::new(Some("root".into()), ModuleKind::Normal);
        Item::new(id, None, None, id, ItemKind::Module(module), None)
    }

    /// Get the root module.
    pub(crate) fn root_module(&self) -> ModuleId {
        self.root_module
    }

    /// Resolve a type with the given ID.
    ///
    /// Panics if there is no item for the given `TypeId` or if the resolved
    /// item is not a `Type`.
    pub(crate) fn resolve_type(&self, type_id: TypeId) -> &Type {
        self.resolve_item(type_id).kind().expect_type()
    }

    /// Resolve a function with the given ID.
    ///
    /// Panics if there is no item for the given `FunctionId` or if the resolved
    /// item is not a `Function`.
    pub(crate) fn resolve_func(&self, func_id: FunctionId) -> &Function {
        self.resolve_item(func_id).kind().expect_function()
    }

    /// Resolve the given `ItemId` as a type, or `None` if there is no item with
    /// the given ID.
    ///
    /// Panics if the ID resolves to an item that is not a type.
    pub(crate) fn safe_resolve_type(&self, type_id: TypeId) -> Option<&Type> {
        self.resolve_item_fallible(type_id)
            .map(|t| t.kind().expect_type())
    }

    /// Resolve the given `ItemId` into an `Item`, or `None` if no such item
    /// exists.
    pub(crate) fn resolve_item_fallible<Id: Into<ItemId>>(
        &self,
        id: Id,
    ) -> Option<&Item> {
        self.items.get(id.into().0)?.as_ref()
    }

    /// Resolve the given `ItemId` into an `Item`.
    ///
    /// Panics if the given ID does not resolve to any item.
    pub(crate) fn resolve_item<Id: Into<ItemId>>(&self, item_id: Id) -> &Item {
        let item_id = item_id.into();
        match self.resolve_item_fallible(item_id) {
            Some(item) => item,
            None => panic!("Not an item: {item_id:?}"),
        }
    }

    /// Get the current module.
    pub(crate) fn current_module(&self) -> ModuleId {
        self.current_module
    }

    /// Add a semantic parent for a given type definition.
    ///
    /// We do this from the type declaration, in order to be able to find the
    /// correct type definition afterwards.
    ///
    /// TODO(emilio): We could consider doing this only when
    /// `declaration.lexical_parent() != definition.lexical_parent()`, but it's
    /// not sure it's worth it.
    pub(crate) fn add_semantic_parent(
        &mut self,
        definition: Cursor,
        parent_id: ItemId,
    ) {
        self.semantic_parents.insert(definition, parent_id);
    }

    /// Returns a known semantic parent for a given definition.
    pub(crate) fn known_semantic_parent(
        &self,
        definition: Cursor,
    ) -> Option<ItemId> {
        self.semantic_parents.get(&definition).copied()
    }

    /// Given a cursor pointing to the location of a template instantiation,
    /// return a tuple of the form `(declaration_cursor, declaration_id,
    /// num_expected_template_args)`.
    ///
    /// Note that `declaration_id` is not guaranteed to be in the context's item
    /// set! It is possible that it is a partial type that we are still in the
    /// middle of parsing.
    fn get_declaration_info_for_template_instantiation(
        &self,
        instantiation: &Cursor,
    ) -> Option<(Cursor, ItemId, usize)> {
        instantiation
            .cur_type()
            .canonical_declaration(Some(instantiation))
            .and_then(|canon_decl| {
                self.get_resolved_type(&canon_decl).and_then(
                    |template_decl_id| {
                        let num_params =
                            template_decl_id.num_self_template_params(self);
                        if num_params == 0 {
                            None
                        } else {
                            Some((
                                *canon_decl.cursor(),
                                template_decl_id.into(),
                                num_params,
                            ))
                        }
                    },
                )
            })
            .or_else(|| {
                // If we haven't already parsed the declaration of
                // the template being instantiated, then it *must*
                // be on the stack of types we are currently
                // parsing. If it wasn't then clang would have
                // already errored out before we started
                // constructing our IR because you can't instantiate
                // a template until it is fully defined.
                instantiation
                    .referenced()
                    .and_then(|referenced| {
                        self.currently_parsed_types()
                            .iter()
                            .find(|partial_ty| *partial_ty.decl() == referenced)
                    })
                    .and_then(|template_decl| {
                        let num_template_params =
                            template_decl.num_self_template_params(self);
                        if num_template_params == 0 {
                            None
                        } else {
                            Some((
                                *template_decl.decl(),
                                template_decl.id(),
                                num_template_params,
                            ))
                        }
                    })
            })
    }

    /// Parse a template instantiation, eg `Foo<int>`.
    ///
    /// This is surprisingly difficult to do with libclang, due to the fact that
    /// it doesn't provide explicit template argument information, except for
    /// function template declarations(!?!??!).
    ///
    /// The only way to do this is manually inspecting the AST and looking for
    /// `TypeRefs` and `TemplateRefs` inside. This, unfortunately, doesn't work for
    /// more complex cases, see the comment on the assertion below.
    ///
    /// To add insult to injury, the AST itself has structure that doesn't make
    /// sense. Sometimes `Foo<Bar<int>>` has an AST with nesting like you might
    /// expect: `(Foo (Bar (int)))`. Other times, the AST we get is completely
    /// flat: `(Foo Bar int)`.
    ///
    /// To see an example of what this method handles:
    ///
    /// ```c++
    /// template<typename T>
    /// class Incomplete {
    ///   T p;
    /// };
    ///
    /// template<typename U>
    /// class Foo {
    ///   Incomplete<U> bar;
    /// };
    /// ```
    ///
    /// Finally, template instantiations are always children of the current
    /// module. They use their template's definition for their name, so the
    /// parent is only useful for ensuring that their layout tests get
    /// codegen'd.
    fn instantiate_template(
        &mut self,
        with_id: ItemId,
        template: TypeId,
        ty: &clang::Type,
        location: Cursor,
    ) -> Option<TypeId> {
        let num_expected_args =
            self.resolve_type(template).num_self_template_params(self);
        if num_expected_args == 0 {
            warn!(
                "Tried to instantiate a template for which we could not \
                 determine any template parameters"
            );
            return None;
        }

        let mut args = vec![];
        let mut found_const_arg = false;
        let mut children = location.collect_children();

        if children.iter().all(|c| !c.has_children()) {
            // This is insanity... If clang isn't giving us a properly nested
            // AST for which template arguments belong to which template we are
            // instantiating, we'll need to construct it ourselves. However,
            // there is an extra `NamespaceRef, NamespaceRef, ..., TemplateRef`
            // representing a reference to the outermost template declaration
            // that we need to filter out of the children. We need to do this
            // filtering because we already know which template declaration is
            // being specialized via the `location`'s type, and if we do not
            // filter it out, we'll add an extra layer of template instantiation
            // on accident.
            let idx = children
                .iter()
                .position(|c| c.kind() == clang_sys::CXCursor_TemplateRef);
            if let Some(idx) = idx {
                if children
                    .iter()
                    .take(idx)
                    .all(|c| c.kind() == clang_sys::CXCursor_NamespaceRef)
                {
                    children = children.into_iter().skip(idx + 1).collect();
                }
            }
        }

        for child in children.iter().rev() {
            match child.kind() {
                clang_sys::CXCursor_TypeRef |
                clang_sys::CXCursor_TypedefDecl |
                clang_sys::CXCursor_TypeAliasDecl => {
                    // The `with_id` ID will potentially end up unused if we give up
                    // on this type (for example, because it has const value
                    // template args), so if we pass `with_id` as the parent, it is
                    // potentially a dangling reference. Instead, use the canonical
                    // template declaration as the parent. It is already parsed and
                    // has a known-resolvable `ItemId`.
                    let ty = Item::from_ty_or_ref(
                        child.cur_type(),
                        *child,
                        Some(template.into()),
                        self,
                    );
                    args.push(ty);
                }
                clang_sys::CXCursor_TemplateRef => {
                    let (
                        template_decl_cursor,
                        template_decl_id,
                        num_expected_template_args,
                    ) = self.get_declaration_info_for_template_instantiation(
                        child,
                    )?;

                    if num_expected_template_args == 0 ||
                        child.has_at_least_num_children(
                            num_expected_template_args,
                        )
                    {
                        // Do a happy little parse. See comment in the TypeRef
                        // match arm about parent IDs.
                        let ty = Item::from_ty_or_ref(
                            child.cur_type(),
                            *child,
                            Some(template.into()),
                            self,
                        );
                        args.push(ty);
                    } else {
                        // This is the case mentioned in the doc comment where
                        // clang gives us a flattened AST and we have to
                        // reconstruct which template arguments go to which
                        // instantiation :(
                        let args_len = args.len();
                        if args_len < num_expected_template_args {
                            warn!(
                                "Found a template instantiation without \
                                 enough template arguments"
                            );
                            return None;
                        }

                        let mut sub_args: Vec<_> = args
                            .drain(args_len - num_expected_template_args..)
                            .collect();
                        sub_args.reverse();

                        let sub_name = Some(template_decl_cursor.spelling());
                        let sub_inst = TemplateInstantiation::new(
                            // This isn't guaranteed to be a type that we've
                            // already finished parsing yet.
                            template_decl_id.as_type_id_unchecked(),
                            sub_args,
                        );
                        let sub_kind =
                            TypeKind::TemplateInstantiation(sub_inst);
                        let sub_ty = Type::new(
                            sub_name,
                            template_decl_cursor
                                .cur_type()
                                .fallible_layout(self)
                                .ok(),
                            sub_kind,
                            false,
                        );
                        let sub_id = self.next_item_id();
                        let sub_item = Item::new(
                            sub_id,
                            None,
                            None,
                            self.current_module.into(),
                            ItemKind::Type(sub_ty),
                            Some(child.location()),
                        );

                        // Bypass all the validations in add_item explicitly.
                        debug!(
                            "instantiate_template: inserting nested \
                             instantiation item: {:?}",
                            sub_item
                        );
                        self.add_item_to_module(&sub_item);
                        debug_assert_eq!(sub_id, sub_item.id());
                        self.items[sub_id.0] = Some(sub_item);
                        args.push(sub_id.as_type_id_unchecked());
                    }
                }
                _ => {
                    warn!(
                        "Found template arg cursor we can't handle: {child:?}"
                    );
                    found_const_arg = true;
                }
            }
        }

        if found_const_arg {
            // This is a dependently typed template instantiation. That is, an
            // instantiation of a template with one or more const values as
            // template arguments, rather than only types as template
            // arguments. For example, `Foo<true, 5>` versus `Bar<bool, int>`.
            // We can't handle these instantiations, so just punt in this
            // situation...
            warn!(
                "Found template instantiated with a const value; \
                 bindgen can't handle this kind of template instantiation!"
            );
            return None;
        }

        if args.len() != num_expected_args {
            warn!(
                "Found a template with an unexpected number of template \
                 arguments"
            );
            return None;
        }

        args.reverse();
        let type_kind = TypeKind::TemplateInstantiation(
            TemplateInstantiation::new(template, args),
        );
        let name = ty.spelling();
        let name = if name.is_empty() { None } else { Some(name) };
        let ty = Type::new(
            name,
            ty.fallible_layout(self).ok(),
            type_kind,
            ty.is_const(),
        );
        let item = Item::new(
            with_id,
            None,
            None,
            self.current_module.into(),
            ItemKind::Type(ty),
            Some(location.location()),
        );

        // Bypass all the validations in add_item explicitly.
        debug!("instantiate_template: inserting item: {item:?}");
        self.add_item_to_module(&item);
        debug_assert_eq!(with_id, item.id());
        self.items[with_id.0] = Some(item);
        Some(with_id.as_type_id_unchecked())
    }

    /// If we have already resolved the type for the given type declaration,
    /// return its `ItemId`. Otherwise, return `None`.
    pub(crate) fn get_resolved_type(
        &self,
        decl: &clang::CanonicalTypeDeclaration,
    ) -> Option<TypeId> {
        self.types
            .get(&TypeKey::Declaration(*decl.cursor()))
            .or_else(|| {
                decl.cursor()
                    .usr()
                    .and_then(|usr| self.types.get(&TypeKey::Usr(usr)))
            })
            .copied()
    }

    /// Looks up for an already resolved type, either because it's builtin, or
    /// because we already have it in the map.
    pub(crate) fn builtin_or_resolved_ty(
        &mut self,
        with_id: ItemId,
        parent_id: Option<ItemId>,
        ty: &clang::Type,
        location: Option<Cursor>,
    ) -> Option<TypeId> {
        use clang_sys::{CXCursor_TypeAliasTemplateDecl, CXCursor_TypeRef};
        debug!("builtin_or_resolved_ty: {ty:?}, {location:?}, {with_id:?}, {parent_id:?}");

        if let Some(decl) = ty.canonical_declaration(location.as_ref()) {
            if let Some(id) = self.get_resolved_type(&decl) {
                debug!(
                    "Already resolved ty {id:?}, {decl:?}, {ty:?} {location:?}"
                );
                // If the declaration already exists, then either:
                //
                //   * the declaration is a template declaration of some sort,
                //     and we are looking at an instantiation or specialization
                //     of it, or
                //   * we have already parsed and resolved this type, and
                //     there's nothing left to do.
                if let Some(location) = location {
                    if decl.cursor().is_template_like() &&
                        *ty != decl.cursor().cur_type()
                    {
                        // For specialized type aliases, there's no way to get the
                        // template parameters as of this writing (for a struct
                        // specialization we wouldn't be in this branch anyway).
                        //
                        // Explicitly return `None` if there aren't any
                        // unspecialized parameters (contains any `TypeRef`) so we
                        // resolve the canonical type if there is one and it's
                        // exposed.
                        //
                        // This is _tricky_, I know :(
                        if decl.cursor().kind() ==
                            CXCursor_TypeAliasTemplateDecl &&
                            !location.contains_cursor(CXCursor_TypeRef) &&
                            ty.canonical_type().is_valid_and_exposed()
                        {
                            return None;
                        }

                        return self
                            .instantiate_template(with_id, id, ty, location)
                            .or(Some(id));
                    }
                }

                return Some(self.build_ty_wrapper(with_id, id, parent_id, ty));
            }
        }

        debug!("Not resolved, maybe builtin?");
        self.build_builtin_ty(ty)
    }

    /// Make a new item that is a resolved type reference to the `wrapped_id`.
    ///
    /// This is unfortunately a lot of bloat, but is needed to properly track
    /// constness et al.
    ///
    /// We should probably make the constness tracking separate, so it doesn't
    /// bloat that much, but hey, we already bloat the heck out of builtin
    /// types.
    pub(crate) fn build_ty_wrapper(
        &mut self,
        with_id: ItemId,
        wrapped_id: TypeId,
        parent_id: Option<ItemId>,
        ty: &clang::Type,
    ) -> TypeId {
        self.build_wrapper(with_id, wrapped_id, parent_id, ty, ty.is_const())
    }

    /// A wrapper over a type that adds a const qualifier explicitly.
    ///
    /// Needed to handle const methods in C++, wrapping the type .
    pub(crate) fn build_const_wrapper(
        &mut self,
        with_id: ItemId,
        wrapped_id: TypeId,
        parent_id: Option<ItemId>,
        ty: &clang::Type,
    ) -> TypeId {
        self.build_wrapper(
            with_id, wrapped_id, parent_id, ty, /* is_const = */ true,
        )
    }

    fn build_wrapper(
        &mut self,
        with_id: ItemId,
        wrapped_id: TypeId,
        parent_id: Option<ItemId>,
        ty: &clang::Type,
        is_const: bool,
    ) -> TypeId {
        let spelling = ty.spelling();
        let layout = ty.fallible_layout(self).ok();
        let location = ty.declaration().location();
        let type_kind = TypeKind::ResolvedTypeRef(wrapped_id);
        let ty = Type::new(Some(spelling), layout, type_kind, is_const);
        let item = Item::new(
            with_id,
            None,
            None,
            parent_id.unwrap_or_else(|| self.current_module.into()),
            ItemKind::Type(ty),
            Some(location),
        );
        self.add_builtin_item(item);
        with_id.as_type_id_unchecked()
    }

    /// Returns the next item ID to be used for an item.
    pub(crate) fn next_item_id(&mut self) -> ItemId {
        let ret = ItemId(self.items.len());
        self.items.push(None);
        ret
    }

    fn build_builtin_ty(&mut self, ty: &clang::Type) -> Option<TypeId> {
        use clang_sys::*;
        let type_kind = match ty.kind() {
            CXType_NullPtr => TypeKind::NullPtr,
            CXType_Void => TypeKind::Void,
            CXType_Bool => TypeKind::Int(IntKind::Bool),
            CXType_Int => TypeKind::Int(IntKind::Int),
            CXType_UInt => TypeKind::Int(IntKind::UInt),
            CXType_Char_S => TypeKind::Int(IntKind::Char { is_signed: true }),
            CXType_Char_U => TypeKind::Int(IntKind::Char { is_signed: false }),
            CXType_SChar => TypeKind::Int(IntKind::SChar),
            CXType_UChar => TypeKind::Int(IntKind::UChar),
            CXType_Short => TypeKind::Int(IntKind::Short),
            CXType_UShort => TypeKind::Int(IntKind::UShort),
            CXType_WChar => TypeKind::Int(IntKind::WChar),
            CXType_Char16 => TypeKind::Int(IntKind::U16),
            CXType_Char32 => TypeKind::Int(IntKind::U32),
            CXType_Long => TypeKind::Int(IntKind::Long),
            CXType_ULong => TypeKind::Int(IntKind::ULong),
            CXType_LongLong => TypeKind::Int(IntKind::LongLong),
            CXType_ULongLong => TypeKind::Int(IntKind::ULongLong),
            CXType_Int128 => TypeKind::Int(IntKind::I128),
            CXType_UInt128 => TypeKind::Int(IntKind::U128),
            CXType_Float16 | CXType_Half => TypeKind::Float(FloatKind::Float16),
            CXType_Float => TypeKind::Float(FloatKind::Float),
            CXType_Double => TypeKind::Float(FloatKind::Double),
            CXType_LongDouble => TypeKind::Float(FloatKind::LongDouble),
            CXType_Float128 => TypeKind::Float(FloatKind::Float128),
            CXType_Complex => {
                let float_type =
                    ty.elem_type().expect("Not able to resolve complex type?");
                let float_kind = match float_type.kind() {
                    CXType_Float16 | CXType_Half => FloatKind::Float16,
                    CXType_Float => FloatKind::Float,
                    CXType_Double => FloatKind::Double,
                    CXType_LongDouble => FloatKind::LongDouble,
                    CXType_Float128 => FloatKind::Float128,
                    _ => panic!(
                        "Non floating-type complex? {ty:?}, {float_type:?}",
                    ),
                };
                TypeKind::Complex(float_kind)
            }
            _ => return None,
        };

        let spelling = ty.spelling();
        let is_const = ty.is_const();
        let layout = ty.fallible_layout(self).ok();
        let location = ty.declaration().location();
        let ty = Type::new(Some(spelling), layout, type_kind, is_const);
        let id = self.next_item_id();
        let item = Item::new(
            id,
            None,
            None,
            self.root_module.into(),
            ItemKind::Type(ty),
            Some(location),
        );
        self.add_builtin_item(item);
        Some(id.as_type_id_unchecked())
    }

    /// Get the current Clang translation unit that is being processed.
    pub(crate) fn translation_unit(&self) -> &clang::TranslationUnit {
        &self.translation_unit
    }

    /// Initialize fallback translation unit if it does not exist and
    /// then return a mutable reference to the fallback translation unit.
    pub(crate) fn try_ensure_fallback_translation_unit(
        &mut self,
    ) -> Option<&mut clang::FallbackTranslationUnit> {
        if self.fallback_tu.is_none() {
            let file = format!(
                "{}/.macro_eval.c",
                match self.options().clang_macro_fallback_build_dir {
                    Some(ref path) => path.as_os_str().to_str()?,
                    None => ".",
                }
            );

            let index = clang::Index::new(false, false);

            let mut header_names_to_compile = Vec::new();
            let mut header_paths = Vec::new();
            let mut header_contents = String::new();
            for input_header in &self.options.input_headers {
                let path = Path::new(input_header.as_ref());
                if let Some(header_path) = path.parent() {
                    if header_path == Path::new("") {
                        header_paths.push(".");
                    } else {
                        header_paths.push(header_path.as_os_str().to_str()?);
                    }
                } else {
                    header_paths.push(".");
                }
                let header_name = path.file_name()?.to_str()?;
                header_names_to_compile
                    .push(header_name.split(".h").next()?.to_string());
                header_contents +=
                    format!("\n#include <{header_name}>").as_str();
            }
            let header_to_precompile = format!(
                "{}/{}",
                match self.options().clang_macro_fallback_build_dir {
                    Some(ref path) => path.as_os_str().to_str()?,
                    None => ".",
                },
                header_names_to_compile.join("-") + "-precompile.h"
            );
            let pch = header_to_precompile.clone() + ".pch";

            let mut header_to_precompile_file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(&header_to_precompile)
                .ok()?;
            header_to_precompile_file
                .write_all(header_contents.as_bytes())
                .ok()?;

            let mut c_args = Vec::new();
            c_args.push("-x".to_string().into_boxed_str());
            c_args.push("c-header".to_string().into_boxed_str());
            for header_path in header_paths {
                c_args.push(format!("-I{header_path}").into_boxed_str());
            }
            c_args.extend(
                self.options
                    .clang_args
                    .iter()
                    .filter(|next| {
                        !self.options.input_headers.contains(next) &&
                            next.as_ref() != "-include"
                    })
                    .cloned(),
            );
            let mut tu = clang::TranslationUnit::parse(
                &index,
                &header_to_precompile,
                &c_args,
                &[],
                clang_sys::CXTranslationUnit_ForSerialization,
            )?;
            tu.save(&pch).ok()?;

            let mut c_args = vec![
                "-include-pch".to_string().into_boxed_str(),
                pch.clone().into_boxed_str(),
            ];
            c_args.extend(
                self.options
                    .clang_args
                    .clone()
                    .iter()
                    .filter(|next| {
                        !self.options.input_headers.contains(next) &&
                            next.as_ref() != "-include"
                    })
                    .cloned(),
            );
            self.fallback_tu = Some(clang::FallbackTranslationUnit::new(
                file,
                header_to_precompile,
                pch,
                &c_args,
            )?);
        }

        self.fallback_tu.as_mut()
    }

    /// Have we parsed the macro named `macro_name` already?
    pub(crate) fn parsed_macro(&self, macro_name: &[u8]) -> bool {
        self.parsed_macros.contains_key(macro_name)
    }

    /// Get the currently parsed macros.
    pub(crate) fn parsed_macros(
        &self,
    ) -> &StdHashMap<Vec<u8>, cexpr::expr::EvalResult> {
        debug_assert!(!self.in_codegen_phase());
        &self.parsed_macros
    }

    /// Mark the macro named `macro_name` as parsed.
    pub(crate) fn note_parsed_macro(
        &mut self,
        id: Vec<u8>,
        value: cexpr::expr::EvalResult,
    ) {
        self.parsed_macros.insert(id, value);
    }

    /// Are we in the codegen phase?
    pub(crate) fn in_codegen_phase(&self) -> bool {
        self.in_codegen
    }

    /// Mark the type with the given `name` as replaced by the type with ID
    /// `potential_ty`.
    ///
    /// Replacement types are declared using the `replaces="xxx"` annotation,
    /// and implies that the original type is hidden.
    pub(crate) fn replace(&mut self, name: &[String], potential_ty: ItemId) {
        match self.replacements.entry(name.into()) {
            Entry::Vacant(entry) => {
                debug!("Defining replacement for {name:?} as {potential_ty:?}");
                entry.insert(potential_ty);
            }
            Entry::Occupied(occupied) => {
                warn!(
                    "Replacement for {name:?} already defined as {:?}; \
                     ignoring duplicate replacement definition as {potential_ty:?}",
                    occupied.get(),
                );
            }
        }
    }

    /// Has the item with the given `name` and `id` been replaced by another
    /// type?
    pub(crate) fn is_replaced_type<Id: Into<ItemId>>(
        &self,
        path: &[String],
        id: Id,
    ) -> bool {
        let id = id.into();
        matches!(self.replacements.get(path), Some(replaced_by) if *replaced_by != id)
    }

    /// Is the type with the given `name` marked as opaque?
    pub(crate) fn opaque_by_name(&self, path: &[String]) -> bool {
        debug_assert!(
            self.in_codegen_phase(),
            "You're not supposed to call this yet"
        );
        self.options.opaque_types.matches(path[1..].join("::"))
    }

    /// Get the options used to configure this bindgen context.
    pub(crate) fn options(&self) -> &BindgenOptions {
        &self.options
    }

    /// Tokenizes a namespace cursor in order to get the name and kind of the
    /// namespace.
    fn tokenize_namespace(
        &self,
        cursor: &Cursor,
    ) -> (Option<String>, ModuleKind) {
        assert_eq!(
            cursor.kind(),
            ::clang_sys::CXCursor_Namespace,
            "Be a nice person"
        );

        let mut module_name = None;
        let spelling = cursor.spelling();
        if !spelling.is_empty() {
            module_name = Some(spelling);
        }

        let mut kind = ModuleKind::Normal;
        let mut looking_for_name = false;
        for token in cursor.tokens().iter() {
            match token.spelling() {
                b"inline" => {
                    debug_assert!(
                        kind != ModuleKind::Inline,
                        "Multiple inline keywords?"
                    );
                    kind = ModuleKind::Inline;
                    // When hitting a nested inline namespace we get a spelling
                    // that looks like ["inline", "foo"]. Deal with it properly.
                    looking_for_name = true;
                }
                // The double colon allows us to handle nested namespaces like
                // namespace foo::bar { }
                //
                // libclang still gives us two namespace cursors, which is cool,
                // but the tokenization of the second begins with the double
                // colon. That's ok, so we only need to handle the weird
                // tokenization here.
                b"namespace" | b"::" => {
                    looking_for_name = true;
                }
                b"{" => {
                    // This should be an anonymous namespace.
                    assert!(looking_for_name);
                    break;
                }
                name => {
                    if looking_for_name {
                        if module_name.is_none() {
                            module_name = Some(
                                String::from_utf8_lossy(name).into_owned(),
                            );
                        }
                        break;
                    } else {
                        // This is _likely_, but not certainly, a macro that's
                        // been placed just before the namespace keyword.
                        // Unfortunately, clang tokens don't let us easily see
                        // through the ifdef tokens, so we don't know what this
                        // token should really be. Instead of panicking though,
                        // we warn the user that we assumed the token was blank,
                        // and then move on.
                        //
                        // See also https://github.com/rust-lang/rust-bindgen/issues/1676.
                        warn!(
                            "Ignored unknown namespace prefix '{}' at {token:?} in {cursor:?}",
                            String::from_utf8_lossy(name),
                        );
                    }
                }
            }
        }

        if cursor.is_inline_namespace() {
            kind = ModuleKind::Inline;
        }

        (module_name, kind)
    }

    /// Given a `CXCursor_Namespace` cursor, return the item ID of the
    /// corresponding module, or create one on the fly.
    pub(crate) fn module(&mut self, cursor: Cursor) -> ModuleId {
        use clang_sys::*;
        assert_eq!(cursor.kind(), CXCursor_Namespace, "Be a nice person");
        let cursor = cursor.canonical();
        if let Some(id) = self.modules.get(&cursor) {
            return *id;
        }

        let (module_name, kind) = self.tokenize_namespace(&cursor);

        let module_id = self.next_item_id();
        let module = Module::new(module_name, kind);
        let module = Item::new(
            module_id,
            None,
            None,
            self.current_module.into(),
            ItemKind::Module(module),
            Some(cursor.location()),
        );

        let module_id = module.id().as_module_id_unchecked();
        self.modules.insert(cursor, module_id);

        self.add_item(module, None, None);

        module_id
    }

    /// Start traversing the module with the given `module_id`, invoke the
    /// callback `cb`, and then return to traversing the original module.
    pub(crate) fn with_module<F>(&mut self, module_id: ModuleId, cb: F)
    where
        F: FnOnce(&mut Self),
    {
        debug_assert!(self.resolve_item(module_id).kind().is_module(), "Wat");

        let previous_id = self.current_module;
        self.current_module = module_id;

        cb(self);

        self.current_module = previous_id;
    }

    /// Iterate over all (explicitly or transitively) allowlisted items.
    ///
    /// If no items are explicitly allowlisted, then all items are considered
    /// allowlisted.
    pub(crate) fn allowlisted_items(&self) -> &ItemSet {
        assert!(self.in_codegen_phase());
        assert!(self.current_module == self.root_module);

        self.allowlisted.as_ref().unwrap()
    }

    /// Check whether a particular blocklisted type implements a trait or not.
    /// Results may be cached.
    pub(crate) fn blocklisted_type_implements_trait(
        &self,
        item: &Item,
        derive_trait: DeriveTrait,
    ) -> CanDerive {
        assert!(self.in_codegen_phase());
        assert!(self.current_module == self.root_module);

        *self
            .blocklisted_types_implement_traits
            .borrow_mut()
            .entry(derive_trait)
            .or_default()
            .entry(item.id())
            .or_insert_with(|| {
                item.expect_type()
                    .name()
                    .and_then(|name| {
                        if self.options.parse_callbacks.is_empty() {
                            // Sized integer types from <stdint.h> get mapped to Rust primitive
                            // types regardless of whether they are blocklisted, so ensure that
                            // standard traits are considered derivable for them too.
                            if self.is_stdint_type(name) {
                                Some(CanDerive::Yes)
                            } else {
                                Some(CanDerive::No)
                            }
                        } else {
                            self.options.last_callback(|cb| {
                                cb.blocklisted_type_implements_trait(
                                    name,
                                    derive_trait,
                                )
                            })
                        }
                    })
                    .unwrap_or(CanDerive::No)
            })
    }

    /// Is the given type a type from <stdint.h> that corresponds to a Rust primitive type?
    pub(crate) fn is_stdint_type(&self, name: &str) -> bool {
        match name {
            "int8_t" | "uint8_t" | "int16_t" | "uint16_t" | "int32_t" |
            "uint32_t" | "int64_t" | "uint64_t" | "uintptr_t" |
            "intptr_t" | "ptrdiff_t" => true,
            "size_t" | "ssize_t" => self.options.size_t_is_usize,
            _ => false,
        }
    }

    /// Get a reference to the set of items we should generate.
    pub(crate) fn codegen_items(&self) -> &ItemSet {
        assert!(self.in_codegen_phase());
        assert!(self.current_module == self.root_module);
        self.codegen_items.as_ref().unwrap()
    }

    /// Compute the allowlisted items set and populate `self.allowlisted`.
    fn compute_allowlisted_and_codegen_items(&mut self) {
        assert!(self.in_codegen_phase());
        assert!(self.current_module == self.root_module);
        assert!(self.allowlisted.is_none());
        let _t = self.timer("compute_allowlisted_and_codegen_items");

        let roots = {
            let mut roots = self
                .items()
                // Only consider roots that are enabled for codegen.
                .filter(|&(_, item)| item.is_enabled_for_codegen(self))
                .filter(|&(_, item)| {
                    // If nothing is explicitly allowlisted, then everything is fair
                    // game.
                    if self.options().allowlisted_types.is_empty() &&
                        self.options().allowlisted_functions.is_empty() &&
                        self.options().allowlisted_vars.is_empty() &&
                        self.options().allowlisted_files.is_empty() &&
                        self.options().allowlisted_items.is_empty()
                    {
                        return true;
                    }

                    // If this is a type that explicitly replaces another, we assume
                    // you know what you're doing.
                    if item.annotations().use_instead_of().is_some() {
                        return true;
                    }

                    // Items with a source location in an explicitly allowlisted file
                    // are always included.
                    if !self.options().allowlisted_files.is_empty() {
                        if let Some(location) = item.location() {
                            let (file, _, _, _) = location.location();
                            if let Some(filename) = file.name() {
                                if self
                                    .options()
                                    .allowlisted_files
                                    .matches(filename)
                                {
                                    return true;
                                }
                            }
                        }
                    }

                    let name = item.path_for_allowlisting(self)[1..].join("::");
                    debug!("allowlisted_items: testing {name:?}");

                    if self.options().allowlisted_items.matches(&name) {
                        return true;
                    }

                    match *item.kind() {
                        ItemKind::Module(..) => true,
                        ItemKind::Function(_) => {
                            self.options().allowlisted_functions.matches(&name)
                        }
                        ItemKind::Var(_) => {
                            self.options().allowlisted_vars.matches(&name)
                        }
                        ItemKind::Type(ref ty) => {
                            if self.options().allowlisted_types.matches(&name) {
                                return true;
                            }

                            // Auto-allowlist types that don't need code
                            // generation if not allowlisting recursively, to
                            // make the #[derive] analysis not be lame.
                            if !self.options().allowlist_recursively {
                                match *ty.kind() {
                                    TypeKind::Void |
                                    TypeKind::NullPtr |
                                    TypeKind::Int(..) |
                                    TypeKind::Float(..) |
                                    TypeKind::Complex(..) |
                                    TypeKind::Array(..) |
                                    TypeKind::Vector(..) |
                                    TypeKind::Pointer(..) |
                                    TypeKind::Reference(..) |
                                    TypeKind::Function(..) |
                                    TypeKind::ResolvedTypeRef(..) |
                                    TypeKind::Opaque |
                                    TypeKind::TypeParam => return true,
                                    _ => {}
                                }
                                if self.is_stdint_type(&name) {
                                    return true;
                                }
                            }

                            // Unnamed top-level enums are special and we
                            // allowlist them via the `allowlisted_vars` filter,
                            // since they're effectively top-level constants,
                            // and there's no way for them to be referenced
                            // consistently.
                            let parent = self.resolve_item(item.parent_id());
                            if !parent.is_module() {
                                return false;
                            }

                            let TypeKind::Enum(ref enum_) = *ty.kind() else {
                                return false;
                            };

                            if ty.name().is_some() {
                                return false;
                            }

                            let mut prefix_path =
                                parent.path_for_allowlisting(self).clone();
                            enum_.variants().iter().any(|variant| {
                                prefix_path.push(
                                    variant.name_for_allowlisting().into(),
                                );
                                let name = prefix_path[1..].join("::");
                                prefix_path.pop().unwrap();
                                self.options().allowlisted_vars.matches(&name)
                                    || self
                                        .options()
                                        .allowlisted_items
                                        .matches(name)
                            })
                        }
                    }
                })
                .map(|(id, _)| id)
                .collect::<Vec<_>>();

            // The reversal preserves the expected ordering of traversal,
            // resulting in more stable-ish bindgen-generated names for
            // anonymous types (like unions).
            roots.reverse();
            roots
        };

        let allowlisted_items_predicate =
            if self.options().allowlist_recursively {
                traversal::all_edges
            } else {
                // Only follow InnerType edges from the allowlisted roots.
                // Such inner types (e.g. anonymous structs/unions) are
                // always emitted by codegen, and they need to be allowlisted
                // to make sure they are processed by e.g. the derive analysis.
                traversal::only_inner_type_edges
            };

        let allowlisted = AllowlistedItemsTraversal::new(
            self,
            roots.clone(),
            allowlisted_items_predicate,
        )
        .collect::<ItemSet>();

        let codegen_items = if self.options().allowlist_recursively {
            AllowlistedItemsTraversal::new(
                self,
                roots,
                traversal::codegen_edges,
            )
            .collect::<ItemSet>()
        } else {
            allowlisted.clone()
        };

        self.allowlisted = Some(allowlisted);
        self.codegen_items = Some(codegen_items);

        for item in self.options().allowlisted_functions.unmatched_items() {
            unused_regex_diagnostic(item, "--allowlist-function", self);
        }

        for item in self.options().allowlisted_vars.unmatched_items() {
            unused_regex_diagnostic(item, "--allowlist-var", self);
        }

        for item in self.options().allowlisted_types.unmatched_items() {
            unused_regex_diagnostic(item, "--allowlist-type", self);
        }

        for item in self.options().allowlisted_items.unmatched_items() {
            unused_regex_diagnostic(item, "--allowlist-items", self);
        }
    }

    /// Convenient method for getting the prefix to use for most traits in
    /// codegen depending on the `use_core` option.
    pub(crate) fn trait_prefix(&self) -> Ident {
        if self.options().use_core {
            self.rust_ident_raw("core")
        } else {
            self.rust_ident_raw("std")
        }
    }

    /// Call if an opaque array is generated
    pub(crate) fn generated_opaque_array(&self) {
        self.generated_opaque_array.set(true)
    }

    /// Whether we need to generate the opaque array type
    pub(crate) fn need_opaque_array_type(&self) -> bool {
        self.generated_opaque_array.get()
    }

    /// Call if a bindgen complex is generated
    pub(crate) fn generated_bindgen_complex(&self) {
        self.generated_bindgen_complex.set(true);
    }

    /// Whether we need to generate the bindgen complex type
    pub(crate) fn need_bindgen_complex_type(&self) -> bool {
        self.generated_bindgen_complex.get()
    }

    /// Call if a bindgen float16 is generated
    pub(crate) fn generated_bindgen_float16(&self) {
        self.generated_bindgen_float16.set(true);
    }

    /// Whether we need to generate the bindgen float16 type
    pub(crate) fn need_bindgen_float16_type(&self) -> bool {
        self.generated_bindgen_float16.get()
    }

    /// Compute which `enum`s have an associated `typedef` definition.
    fn compute_enum_typedef_combos(&mut self) {
        let _t = self.timer("compute_enum_typedef_combos");
        assert!(self.enum_typedef_combos.is_none());

        let mut enum_typedef_combos = HashSet::default();
        for item in &self.items {
            if let Some(ItemKind::Module(module)) =
                item.as_ref().map(Item::kind)
            {
                // Find typedefs in this module, and build set of their names.
                let mut names_of_typedefs = HashSet::default();
                for child_id in module.children() {
                    if let Some(ItemKind::Type(ty)) =
                        self.items[child_id.0].as_ref().map(Item::kind)
                    {
                        if let (Some(name), TypeKind::Alias(type_id)) =
                            (ty.name(), ty.kind())
                        {
                            // We disregard aliases that refer to the enum
                            // itself, such as in `typedef enum { ... } Enum;`.
                            if type_id
                                .into_resolver()
                                .through_type_refs()
                                .through_type_aliases()
                                .resolve(self)
                                .expect_type()
                                .is_int()
                            {
                                names_of_typedefs.insert(name);
                            }
                        }
                    }
                }

                // Find enums in this module, and record the ID of each one that
                // has a typedef.
                for child_id in module.children() {
                    if let Some(ItemKind::Type(ty)) =
                        self.items[child_id.0].as_ref().map(Item::kind)
                    {
                        if let (Some(name), true) = (ty.name(), ty.is_enum()) {
                            if names_of_typedefs.contains(name) {
                                enum_typedef_combos.insert(*child_id);
                            }
                        }
                    }
                }
            }
        }

        self.enum_typedef_combos = Some(enum_typedef_combos);
    }

    /// Look up whether `id` refers to an `enum` whose underlying type is
    /// defined by a `typedef`.
    pub(crate) fn is_enum_typedef_combo(&self, id: ItemId) -> bool {
        assert!(
            self.in_codegen_phase(),
            "We only compute enum_typedef_combos when we enter codegen",
        );
        self.enum_typedef_combos.as_ref().unwrap().contains(&id)
    }

    /// Compute whether we can derive debug.
    fn compute_cannot_derive_debug(&mut self) {
        let _t = self.timer("compute_cannot_derive_debug");
        assert!(self.cannot_derive_debug.is_none());
        if self.options.derive_debug {
            self.cannot_derive_debug =
                Some(as_cannot_derive_set(analyze::<CannotDerive>((
                    self,
                    DeriveTrait::Debug,
                ))));
        }
    }

    /// Look up whether the item with `id` can
    /// derive debug or not.
    pub(crate) fn lookup_can_derive_debug<Id: Into<ItemId>>(
        &self,
        id: Id,
    ) -> bool {
        let id = id.into();
        assert!(
            self.in_codegen_phase(),
            "We only compute can_derive_debug when we enter codegen"
        );

        // Look up the computed value for whether the item with `id` can
        // derive debug or not.
        !self.cannot_derive_debug.as_ref().unwrap().contains(&id)
    }

    /// Compute whether we can derive default.
    fn compute_cannot_derive_default(&mut self) {
        let _t = self.timer("compute_cannot_derive_default");
        assert!(self.cannot_derive_default.is_none());
        if self.options.derive_default {
            self.cannot_derive_default =
                Some(as_cannot_derive_set(analyze::<CannotDerive>((
                    self,
                    DeriveTrait::Default,
                ))));
        }
    }

    /// Look up whether the item with `id` can
    /// derive default or not.
    pub(crate) fn lookup_can_derive_default<Id: Into<ItemId>>(
        &self,
        id: Id,
    ) -> bool {
        let id = id.into();
        assert!(
            self.in_codegen_phase(),
            "We only compute can_derive_default when we enter codegen"
        );

        // Look up the computed value for whether the item with `id` can
        // derive default or not.
        !self.cannot_derive_default.as_ref().unwrap().contains(&id)
    }

    /// Compute whether we can derive copy.
    fn compute_cannot_derive_copy(&mut self) {
        let _t = self.timer("compute_cannot_derive_copy");
        assert!(self.cannot_derive_copy.is_none());
        self.cannot_derive_copy =
            Some(as_cannot_derive_set(analyze::<CannotDerive>((
                self,
                DeriveTrait::Copy,
            ))));
    }

    /// Compute whether we can derive hash.
    fn compute_cannot_derive_hash(&mut self) {
        let _t = self.timer("compute_cannot_derive_hash");
        assert!(self.cannot_derive_hash.is_none());
        if self.options.derive_hash {
            self.cannot_derive_hash =
                Some(as_cannot_derive_set(analyze::<CannotDerive>((
                    self,
                    DeriveTrait::Hash,
                ))));
        }
    }

    /// Look up whether the item with `id` can
    /// derive hash or not.
    pub(crate) fn lookup_can_derive_hash<Id: Into<ItemId>>(
        &self,
        id: Id,
    ) -> bool {
        let id = id.into();
        assert!(
            self.in_codegen_phase(),
            "We only compute can_derive_debug when we enter codegen"
        );

        // Look up the computed value for whether the item with `id` can
        // derive hash or not.
        !self.cannot_derive_hash.as_ref().unwrap().contains(&id)
    }

    /// Compute whether we can derive `PartialOrd`, `PartialEq` or `Eq`.
    fn compute_cannot_derive_partialord_partialeq_or_eq(&mut self) {
        let _t = self.timer("compute_cannot_derive_partialord_partialeq_or_eq");
        assert!(self.cannot_derive_partialeq_or_partialord.is_none());
        if self.options.derive_partialord ||
            self.options.derive_partialeq ||
            self.options.derive_eq
        {
            self.cannot_derive_partialeq_or_partialord =
                Some(analyze::<CannotDerive>((
                    self,
                    DeriveTrait::PartialEqOrPartialOrd,
                )));
        }
    }

    /// Look up whether the item with `id` can derive `Partial{Eq,Ord}`.
    pub(crate) fn lookup_can_derive_partialeq_or_partialord<
        Id: Into<ItemId>,
    >(
        &self,
        id: Id,
    ) -> CanDerive {
        let id = id.into();
        assert!(
            self.in_codegen_phase(),
            "We only compute can_derive_partialeq_or_partialord when we enter codegen"
        );

        // Look up the computed value for whether the item with `id` can
        // derive partialeq or not.
        self.cannot_derive_partialeq_or_partialord
            .as_ref()
            .unwrap()
            .get(&id)
            .copied()
            .unwrap_or(CanDerive::Yes)
    }

    /// Look up whether the item with `id` can derive `Copy` or not.
    pub(crate) fn lookup_can_derive_copy<Id: Into<ItemId>>(
        &self,
        id: Id,
    ) -> bool {
        assert!(
            self.in_codegen_phase(),
            "We only compute can_derive_debug when we enter codegen"
        );

        // Look up the computed value for whether the item with `id` can
        // derive `Copy` or not.
        let id = id.into();

        !self.lookup_has_type_param_in_array(id) &&
            !self.cannot_derive_copy.as_ref().unwrap().contains(&id)
    }

    /// Compute whether the type has type parameter in array.
    fn compute_has_type_param_in_array(&mut self) {
        let _t = self.timer("compute_has_type_param_in_array");
        assert!(self.has_type_param_in_array.is_none());
        self.has_type_param_in_array =
            Some(analyze::<HasTypeParameterInArray>(self));
    }

    /// Look up whether the item with `id` has type parameter in array or not.
    pub(crate) fn lookup_has_type_param_in_array<Id: Into<ItemId>>(
        &self,
        id: Id,
    ) -> bool {
        assert!(
            self.in_codegen_phase(),
            "We only compute has array when we enter codegen"
        );

        // Look up the computed value for whether the item with `id` has
        // type parameter in array or not.
        self.has_type_param_in_array
            .as_ref()
            .unwrap()
            .contains(&id.into())
    }

    /// Compute whether the type has float.
    fn compute_has_float(&mut self) {
        let _t = self.timer("compute_has_float");
        assert!(self.has_float.is_none());
        if self.options.derive_eq || self.options.derive_ord {
            self.has_float = Some(analyze::<HasFloat>(self));
        }
    }

    /// Look up whether the item with `id` has array or not.
    pub(crate) fn lookup_has_float<Id: Into<ItemId>>(&self, id: Id) -> bool {
        assert!(
            self.in_codegen_phase(),
            "We only compute has float when we enter codegen"
        );

        // Look up the computed value for whether the item with `id` has
        // float or not.
        self.has_float.as_ref().unwrap().contains(&id.into())
    }

    /// Check if `--no-partialeq` flag is enabled for this item.
    pub(crate) fn no_partialeq_by_name(&self, item: &Item) -> bool {
        let name = item.path_for_allowlisting(self)[1..].join("::");
        self.options().no_partialeq_types.matches(name)
    }

    /// Check if `--no-copy` flag is enabled for this item.
    pub(crate) fn no_copy_by_name(&self, item: &Item) -> bool {
        let name = item.path_for_allowlisting(self)[1..].join("::");
        self.options().no_copy_types.matches(name)
    }

    /// Check if `--no-debug` flag is enabled for this item.
    pub(crate) fn no_debug_by_name(&self, item: &Item) -> bool {
        let name = item.path_for_allowlisting(self)[1..].join("::");
        self.options().no_debug_types.matches(name)
    }

    /// Check if `--no-default` flag is enabled for this item.
    pub(crate) fn no_default_by_name(&self, item: &Item) -> bool {
        let name = item.path_for_allowlisting(self)[1..].join("::");
        self.options().no_default_types.matches(name)
    }

    /// Check if `--no-hash` flag is enabled for this item.
    pub(crate) fn no_hash_by_name(&self, item: &Item) -> bool {
        let name = item.path_for_allowlisting(self)[1..].join("::");
        self.options().no_hash_types.matches(name)
    }

    /// Check if `--must-use-type` flag is enabled for this item.
    pub(crate) fn must_use_type_by_name(&self, item: &Item) -> bool {
        let name = item.path_for_allowlisting(self)[1..].join("::");
        self.options().must_use_types.matches(name)
    }

    /// Wrap some tokens in an `unsafe` block if the `--wrap-unsafe-ops` option is enabled.
    pub(crate) fn wrap_unsafe_ops(&self, tokens: impl ToTokens) -> TokenStream {
        if self.options.wrap_unsafe_ops {
            quote!(unsafe { #tokens })
        } else {
            tokens.into_token_stream()
        }
    }

    /// Get the suffix to be added to `static` functions if the `--wrap-static-fns` option is
    /// enabled.
    pub(crate) fn wrap_static_fns_suffix(&self) -> &str {
        self.options()
            .wrap_static_fns_suffix
            .as_deref()
            .unwrap_or(crate::DEFAULT_NON_EXTERN_FNS_SUFFIX)
    }
}

/// A builder struct for configuring item resolution options.
#[derive(Debug, Copy, Clone)]
pub(crate) struct ItemResolver {
    id: ItemId,
    through_type_refs: bool,
    through_type_aliases: bool,
}

impl ItemId {
    /// Create an `ItemResolver` from this item ID.
    pub(crate) fn into_resolver(self) -> ItemResolver {
        self.into()
    }
}

impl<T> From<T> for ItemResolver
where
    T: Into<ItemId>,
{
    fn from(id: T) -> ItemResolver {
        ItemResolver::new(id)
    }
}

impl ItemResolver {
    /// Construct a new `ItemResolver` from the given ID.
    pub(crate) fn new<Id: Into<ItemId>>(id: Id) -> ItemResolver {
        let id = id.into();
        ItemResolver {
            id,
            through_type_refs: false,
            through_type_aliases: false,
        }
    }

    /// Keep resolving through `Type::TypeRef` items.
    pub(crate) fn through_type_refs(mut self) -> ItemResolver {
        self.through_type_refs = true;
        self
    }

    /// Keep resolving through `Type::Alias` items.
    pub(crate) fn through_type_aliases(mut self) -> ItemResolver {
        self.through_type_aliases = true;
        self
    }

    /// Finish configuring and perform the actual item resolution.
    pub(crate) fn resolve(self, ctx: &BindgenContext) -> &Item {
        assert!(ctx.collected_typerefs());

        let mut id = self.id;
        let mut seen_ids = HashSet::default();
        loop {
            let item = ctx.resolve_item(id);

            // Detect cycles and bail out. These can happen in certain cases
            // involving incomplete qualified dependent types (#2085).
            if !seen_ids.insert(id) {
                return item;
            }

            let ty_kind = item.as_type().map(|t| t.kind());
            match ty_kind {
                Some(&TypeKind::ResolvedTypeRef(next_id))
                    if self.through_type_refs =>
                {
                    id = next_id.into();
                }
                // We intentionally ignore template aliases here, as they are
                // more complicated, and don't represent a simple renaming of
                // some type.
                Some(&TypeKind::Alias(next_id))
                    if self.through_type_aliases =>
                {
                    id = next_id.into();
                }
                _ => return item,
            }
        }
    }
}

/// A type that we are in the middle of parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PartialType {
    decl: Cursor,
    // Just an ItemId, and not a TypeId, because we haven't finished this type
    // yet, so there's still time for things to go wrong.
    id: ItemId,
}

impl PartialType {
    /// Construct a new `PartialType`.
    pub(crate) fn new(decl: Cursor, id: ItemId) -> PartialType {
        // assert!(decl == decl.canonical());
        PartialType { decl, id }
    }

    /// The cursor pointing to this partial type's declaration location.
    pub(crate) fn decl(&self) -> &Cursor {
        &self.decl
    }

    /// The item ID allocated for this type. This is *NOT* a key for an entry in
    /// the context's item set yet!
    pub(crate) fn id(&self) -> ItemId {
        self.id
    }
}

impl TemplateParameters for PartialType {
    fn self_template_params(&self, _ctx: &BindgenContext) -> Vec<TypeId> {
        // Maybe at some point we will eagerly parse named types, but for now we
        // don't and this information is unavailable.
        vec![]
    }

    fn num_self_template_params(&self, _ctx: &BindgenContext) -> usize {
        // Wouldn't it be nice if libclang would reliably give us this
        // information‽
        match self.decl().kind() {
            clang_sys::CXCursor_ClassTemplate |
            clang_sys::CXCursor_FunctionTemplate |
            clang_sys::CXCursor_TypeAliasTemplateDecl => {
                let mut num_params = 0;
                self.decl().visit(|c| {
                    match c.kind() {
                        clang_sys::CXCursor_TemplateTypeParameter |
                        clang_sys::CXCursor_TemplateTemplateParameter |
                        clang_sys::CXCursor_NonTypeTemplateParameter => {
                            num_params += 1;
                        }
                        _ => {}
                    };
                    clang_sys::CXChildVisit_Continue
                });
                num_params
            }
            _ => 0,
        }
    }
}

fn unused_regex_diagnostic(item: &str, name: &str, _ctx: &BindgenContext) {
    warn!("unused option: {name} {item}");

    #[cfg(feature = "experimental")]
    if _ctx.options().emit_diagnostics {
        use crate::diagnostics::{Diagnostic, Level};

        Diagnostic::default()
            .with_title(
                format!("Unused regular expression: `{item}`."),
                Level::Warning,
            )
            .add_annotation(
                format!("This regular expression was passed to `{name}`."),
                Level::Note,
            )
            .display();
    }
}
