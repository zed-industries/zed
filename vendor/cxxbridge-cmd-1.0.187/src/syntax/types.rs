use crate::syntax::attrs::OtherAttrs;
use crate::syntax::cfg::ComputedCfg;
use crate::syntax::improper::ImproperCtype;
use crate::syntax::instantiate::ImplKey;
use crate::syntax::map::{OrderedMap, UnorderedMap};
use crate::syntax::query::TypeQuery;
use crate::syntax::report::Errors;
use crate::syntax::resolve::Resolution;
use crate::syntax::set::UnorderedSet;
use crate::syntax::trivial::{self, TrivialReason};
use crate::syntax::unpin::{self, UnpinReason};
use crate::syntax::visit::{self, Visit};
use crate::syntax::{
    toposort, Api, Atom, Enum, ExternFn, ExternType, Impl, Lifetimes, Pair, Struct, Type, TypeAlias,
};
use indexmap::map::Entry;
use proc_macro2::Ident;
use quote::ToTokens;

pub(crate) struct Types<'a> {
    pub all: OrderedMap<&'a Type, ComputedCfg<'a>>,
    pub structs: UnorderedMap<&'a Ident, &'a Struct>,
    pub enums: UnorderedMap<&'a Ident, &'a Enum>,
    pub cxx: UnorderedSet<&'a Ident>,
    pub rust: UnorderedSet<&'a Ident>,
    pub aliases: UnorderedMap<&'a Ident, &'a TypeAlias>,
    pub untrusted: UnorderedMap<&'a Ident, &'a ExternType>,
    pub required_trivial: UnorderedMap<&'a Ident, Vec<TrivialReason<'a>>>,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub required_unpin: UnorderedMap<&'a Ident, UnpinReason<'a>>,
    pub impls: OrderedMap<ImplKey<'a>, ConditionalImpl<'a>>,
    pub resolutions: UnorderedMap<&'a Ident, Resolution<'a>>,
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub associated_fn: UnorderedMap<&'a Ident, Vec<&'a ExternFn>>,
    pub struct_improper_ctypes: UnorderedSet<&'a Ident>,
    pub toposorted_structs: Vec<&'a Struct>,
}

pub(crate) struct ConditionalImpl<'a> {
    pub cfg: ComputedCfg<'a>,
    // None for implicit impls, which arise from using a generic type
    // instantiation in a struct field or function signature.
    #[cfg_attr(not(proc_macro), expect(dead_code))]
    pub explicit_impl: Option<&'a Impl>,
}

impl<'a> Types<'a> {
    pub(crate) fn collect(cx: &mut Errors, apis: &'a [Api]) -> Self {
        let mut all = OrderedMap::new();
        let mut structs = UnorderedMap::new();
        let mut enums = UnorderedMap::new();
        let mut cxx = UnorderedSet::new();
        let mut rust = UnorderedSet::new();
        let mut aliases = UnorderedMap::new();
        let mut untrusted = UnorderedMap::new();
        let mut impls = OrderedMap::new();
        let mut resolutions = UnorderedMap::new();
        let mut associated_fn = UnorderedMap::new();
        let struct_improper_ctypes = UnorderedSet::new();
        let toposorted_structs = Vec::new();

        fn visit<'a>(
            all: &mut OrderedMap<&'a Type, ComputedCfg<'a>>,
            ty: &'a Type,
            cfg: impl Into<ComputedCfg<'a>>,
        ) {
            struct CollectTypes<'s, 'a> {
                all: &'s mut OrderedMap<&'a Type, ComputedCfg<'a>>,
                cfg: ComputedCfg<'a>,
            }

            impl<'s, 'a> Visit<'a> for CollectTypes<'s, 'a> {
                fn visit_type(&mut self, ty: &'a Type) {
                    match self.all.entry(ty) {
                        Entry::Vacant(entry) => {
                            entry.insert(self.cfg.clone());
                        }
                        Entry::Occupied(mut entry) => entry.get_mut().merge_or(self.cfg.clone()),
                    }
                    visit::visit_type(self, ty);
                }
            }

            let mut visitor = CollectTypes {
                all,
                cfg: cfg.into(),
            };
            visitor.visit_type(ty);
        }

        let mut add_resolution =
            |name: &'a Pair, attrs: &'a OtherAttrs, generics: &'a Lifetimes| {
                resolutions.insert(
                    &name.rust,
                    Resolution {
                        name,
                        attrs,
                        generics,
                    },
                );
            };

        let mut type_names = UnorderedSet::new();
        let mut function_names = UnorderedSet::new();
        for api in apis {
            // The same identifier is permitted to be declared as both a shared
            // enum and extern C++ type, or shared struct and extern C++ type.
            // That indicates to not emit the C++ enum/struct definition because
            // it's defined by the included headers already.
            //
            // All other cases of duplicate identifiers are reported as an error.
            match api {
                Api::Include(_) => {}
                Api::Struct(strct) => {
                    let ident = &strct.name.rust;
                    if !type_names.insert(ident)
                        && (!cxx.contains(ident)
                            || structs.contains_key(ident)
                            || enums.contains_key(ident))
                    {
                        // If already declared as a struct or enum, or if
                        // colliding with something other than an extern C++
                        // type, then error.
                        duplicate_name(cx, strct, ItemName::Type(ident));
                    }
                    structs.insert(&strct.name.rust, strct);
                    for field in &strct.fields {
                        let cfg = ComputedCfg::all(&strct.cfg, &field.cfg);
                        visit(&mut all, &field.ty, cfg);
                    }
                    add_resolution(&strct.name, &strct.attrs, &strct.generics);
                }
                Api::Enum(enm) => {
                    match all.entry(&enm.repr.repr_type) {
                        Entry::Vacant(entry) => {
                            entry.insert(ComputedCfg::Leaf(&enm.cfg));
                        }
                        Entry::Occupied(mut entry) => entry.get_mut().merge_or(&enm.cfg),
                    }
                    let ident = &enm.name.rust;
                    if !type_names.insert(ident)
                        && (!cxx.contains(ident)
                            || structs.contains_key(ident)
                            || enums.contains_key(ident))
                    {
                        // If already declared as a struct or enum, or if
                        // colliding with something other than an extern C++
                        // type, then error.
                        duplicate_name(cx, enm, ItemName::Type(ident));
                    }
                    enums.insert(ident, enm);
                    add_resolution(&enm.name, &enm.attrs, &enm.generics);
                }
                Api::CxxType(ety) => {
                    let ident = &ety.name.rust;
                    if !type_names.insert(ident)
                        && (cxx.contains(ident)
                            || !structs.contains_key(ident) && !enums.contains_key(ident))
                    {
                        // If already declared as an extern C++ type, or if
                        // colliding with something which is neither struct nor
                        // enum, then error.
                        duplicate_name(cx, ety, ItemName::Type(ident));
                    }
                    cxx.insert(ident);
                    if !ety.trusted {
                        untrusted.insert(ident, ety);
                    }
                    add_resolution(&ety.name, &ety.attrs, &ety.generics);
                }
                Api::RustType(ety) => {
                    let ident = &ety.name.rust;
                    if !type_names.insert(ident) {
                        duplicate_name(cx, ety, ItemName::Type(ident));
                    }
                    rust.insert(ident);
                    add_resolution(&ety.name, &ety.attrs, &ety.generics);
                }
                Api::CxxFunction(efn) | Api::RustFunction(efn) => {
                    // Note: duplication of the C++ name is fine because C++ has
                    // function overloading.
                    let self_type = efn.self_type();
                    if let Some(self_type) = self_type {
                        associated_fn
                            .entry(self_type)
                            .or_insert_with(Vec::new)
                            .push(efn);
                    }
                    if !self_type.is_some_and(|self_type| self_type == "Self")
                        && !function_names.insert((self_type, &efn.name.rust))
                    {
                        duplicate_name(cx, efn, ItemName::Function(self_type, &efn.name.rust));
                    }
                    for arg in &efn.args {
                        visit(&mut all, &arg.ty, &efn.cfg);
                    }
                    if let Some(ret) = &efn.ret {
                        visit(&mut all, ret, &efn.cfg);
                    }
                }
                Api::TypeAlias(alias) => {
                    let ident = &alias.name.rust;
                    if !type_names.insert(ident) {
                        duplicate_name(cx, alias, ItemName::Type(ident));
                    }
                    cxx.insert(ident);
                    aliases.insert(ident, alias);
                    add_resolution(&alias.name, &alias.attrs, &alias.generics);
                }
                Api::Impl(imp) => {
                    visit(&mut all, &imp.ty, &imp.cfg);
                    if let Some(key) = imp.ty.impl_key() {
                        impls.insert(key, ConditionalImpl::from(imp));
                    }
                }
            }
        }

        for (ty, cfg) in &all {
            let Some(impl_key) = ty.impl_key() else {
                continue;
            };
            let implicit_impl = match &impl_key {
                ImplKey::RustBox(ident)
                | ImplKey::RustVec(ident)
                | ImplKey::UniquePtr(ident)
                | ImplKey::SharedPtr(ident)
                | ImplKey::WeakPtr(ident)
                | ImplKey::CxxVector(ident) => {
                    Atom::from(ident.rust).is_none() && !aliases.contains_key(ident.rust)
                }
            };
            if implicit_impl {
                match impls.entry(impl_key) {
                    Entry::Vacant(entry) => {
                        entry.insert(ConditionalImpl::from(cfg.clone()));
                    }
                    Entry::Occupied(mut entry) => entry.get_mut().cfg.merge_or(cfg.clone()),
                }
            }
        }

        // All these APIs may contain types passed by value. We need to ensure
        // we check that this is permissible. We do this _after_ scanning all
        // the APIs above, in case some function or struct references a type
        // which is declared subsequently.
        let required_trivial =
            trivial::required_trivial_reasons(apis, &all, &structs, &enums, &cxx, &aliases, &impls);

        let required_unpin =
            unpin::required_unpin_reasons(apis, &all, &structs, &enums, &cxx, &aliases);

        let mut types = Types {
            all,
            structs,
            enums,
            cxx,
            rust,
            aliases,
            untrusted,
            required_trivial,
            required_unpin,
            impls,
            resolutions,
            associated_fn,
            struct_improper_ctypes,
            toposorted_structs,
        };

        types.toposorted_structs = toposort::sort(cx, apis, &types);

        let mut unresolved_structs = types.structs.keys();
        let mut new_information = true;
        while new_information {
            new_information = false;
            unresolved_structs.retain(|ident| {
                let mut retain = false;
                for var in &types.structs[ident].fields {
                    if match types.determine_improper_ctype(&var.ty) {
                        ImproperCtype::Depends(inner) => {
                            retain = true;
                            types.struct_improper_ctypes.contains(inner)
                        }
                        ImproperCtype::Definite(improper) => improper,
                    } {
                        types.struct_improper_ctypes.insert(ident);
                        new_information = true;
                        return false;
                    }
                }
                // If all fields definite false, remove from unresolved_structs.
                retain
            });
        }

        types
    }

    pub(crate) fn needs_indirect_abi(&self, ty: impl Into<TypeQuery<'a>>) -> bool {
        let ty = ty.into();
        match ty {
            TypeQuery::RustBox
            | TypeQuery::UniquePtr
            | TypeQuery::Ref(_)
            | TypeQuery::Ptr(_)
            | TypeQuery::Str
            | TypeQuery::Fn
            | TypeQuery::SliceRef => false,
            TypeQuery::Array(_) => true,
            _ => !self.is_guaranteed_pod(ty) || self.is_considered_improper_ctype(ty),
        }
    }

    // Types that trigger rustc's default #[warn(improper_ctypes)] lint, even if
    // they may be otherwise unproblematic to mention in an extern signature.
    // For example in a signature like `extern "C" fn(*const String)`, rustc
    // refuses to believe that C could know how to supply us with a pointer to a
    // Rust String, even though C could easily have obtained that pointer
    // legitimately from a Rust call.
    pub(crate) fn is_considered_improper_ctype(&self, ty: impl Into<TypeQuery<'a>>) -> bool {
        match self.determine_improper_ctype(ty) {
            ImproperCtype::Definite(improper) => improper,
            ImproperCtype::Depends(ident) => self.struct_improper_ctypes.contains(ident),
        }
    }

    // Types which we need to assume could possibly exist by value on the Rust
    // side.
    pub(crate) fn is_maybe_trivial(&self, ty: &Ident) -> bool {
        self.structs.contains_key(ty)
            || self.enums.contains_key(ty)
            || self.aliases.contains_key(ty)
    }

    pub(crate) fn contains_elided_lifetime(&self, ty: &Type) -> bool {
        match ty {
            Type::Ident(ty) => {
                Atom::from(&ty.rust).is_none()
                    && ty.generics.lifetimes.len()
                        != self.resolve(&ty.rust).generics.lifetimes.len()
            }
            Type::RustBox(ty)
            | Type::RustVec(ty)
            | Type::UniquePtr(ty)
            | Type::SharedPtr(ty)
            | Type::WeakPtr(ty)
            | Type::CxxVector(ty) => self.contains_elided_lifetime(&ty.inner),
            Type::Ref(ty) => ty.lifetime.is_none() || self.contains_elided_lifetime(&ty.inner),
            Type::Ptr(ty) => self.contains_elided_lifetime(&ty.inner),
            Type::Str(ty) => ty.lifetime.is_none(),
            Type::SliceRef(ty) => ty.lifetime.is_none() || self.contains_elided_lifetime(&ty.inner),
            Type::Array(ty) => self.contains_elided_lifetime(&ty.inner),
            Type::Fn(_) | Type::Void(_) => false,
        }
    }
}

impl<'t, 'a> IntoIterator for &'t Types<'a> {
    type Item = &'a Type;
    type IntoIter = std::iter::Copied<indexmap::map::Keys<'t, &'a Type, ComputedCfg<'a>>>;
    fn into_iter(self) -> Self::IntoIter {
        self.all.keys().copied()
    }
}

impl<'a> From<ComputedCfg<'a>> for ConditionalImpl<'a> {
    fn from(cfg: ComputedCfg<'a>) -> Self {
        ConditionalImpl {
            cfg,
            explicit_impl: None,
        }
    }
}

impl<'a> From<&'a Impl> for ConditionalImpl<'a> {
    fn from(imp: &'a Impl) -> Self {
        ConditionalImpl {
            cfg: ComputedCfg::Leaf(&imp.cfg),
            explicit_impl: Some(imp),
        }
    }
}

enum ItemName<'a> {
    Type(&'a Ident),
    Function(Option<&'a Ident>, &'a Ident),
}

fn duplicate_name(cx: &mut Errors, sp: impl ToTokens, name: ItemName) {
    let description = match name {
        ItemName::Type(name) => format!("type `{}`", name),
        ItemName::Function(Some(self_type), name) => {
            format!("associated function `{}::{}`", self_type, name)
        }
        ItemName::Function(None, name) => format!("function `{}`", name),
    };
    let msg = format!("the {} is defined multiple times", description);
    cx.error(sp, msg);
}
