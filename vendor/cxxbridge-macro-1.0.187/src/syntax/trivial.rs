use crate::syntax::cfg::ComputedCfg;
use crate::syntax::instantiate::ImplKey;
use crate::syntax::map::{OrderedMap, UnorderedMap};
use crate::syntax::set::{OrderedSet as Set, UnorderedSet};
use crate::syntax::types::ConditionalImpl;
use crate::syntax::{Api, Enum, ExternFn, NamedType, Pair, SliceRef, Struct, Type, TypeAlias};
use proc_macro2::Ident;
use std::fmt::{self, Display};

#[derive(Copy, Clone)]
pub(crate) enum TrivialReason<'a> {
    StructField(&'a Struct),
    FunctionArgument(&'a ExternFn),
    FunctionReturn(&'a ExternFn),
    BoxTarget {
        // Whether the extern functions used by rust::Box are being produced
        // within this cxx::bridge expansion, as opposed to the boxed type being
        // a type alias from a different module.
        #[cfg_attr(not(proc_macro), expect(dead_code))]
        local: bool,
    },
    VecElement {
        #[cfg_attr(not(proc_macro), expect(dead_code))]
        local: bool,
    },
    SliceElement(&'a SliceRef),
}

pub(crate) fn required_trivial_reasons<'a>(
    apis: &'a [Api],
    all: &OrderedMap<&'a Type, ComputedCfg>,
    structs: &UnorderedMap<&'a Ident, &'a Struct>,
    enums: &UnorderedMap<&'a Ident, &'a Enum>,
    cxx: &UnorderedSet<&'a Ident>,
    aliases: &UnorderedMap<&'a Ident, &'a TypeAlias>,
    impls: &OrderedMap<ImplKey<'a>, ConditionalImpl<'a>>,
) -> UnorderedMap<&'a Ident, Vec<TrivialReason<'a>>> {
    let mut required_trivial = UnorderedMap::new();

    let mut insist_extern_types_are_trivial = |ident: &'a NamedType, reason| {
        if cxx.contains(&ident.rust)
            && !structs.contains_key(&ident.rust)
            && !enums.contains_key(&ident.rust)
        {
            required_trivial
                .entry(&ident.rust)
                .or_insert_with(Vec::new)
                .push(reason);
        }
    };

    for api in apis {
        match api {
            Api::Struct(strct) => {
                for field in &strct.fields {
                    if let Type::Ident(ident) = &field.ty {
                        let reason = TrivialReason::StructField(strct);
                        insist_extern_types_are_trivial(ident, reason);
                    }
                }
            }
            Api::CxxFunction(efn) | Api::RustFunction(efn) => {
                for arg in &efn.args {
                    if let Type::Ident(ident) = &arg.ty {
                        let reason = TrivialReason::FunctionArgument(efn);
                        insist_extern_types_are_trivial(ident, reason);
                    }
                }
                if let Some(Type::Ident(ident)) = &efn.ret {
                    let reason = TrivialReason::FunctionReturn(efn);
                    insist_extern_types_are_trivial(ident, reason);
                }
            }
            _ => {}
        }
    }

    for (ty, _cfg) in all {
        // Ignore cfg. For now if any use of an extern type requires it to be
        // trivial, we enforce that it is trivial in all configurations. This
        // can potentially be relaxed if there is a motivating use case.
        match ty {
            Type::RustBox(ty1) => {
                if let Type::Ident(ident) = &ty1.inner {
                    let local = !aliases.contains_key(&ident.rust)
                        || impls.contains_key(&ty.impl_key().unwrap());
                    let reason = TrivialReason::BoxTarget { local };
                    insist_extern_types_are_trivial(ident, reason);
                }
            }
            Type::RustVec(ty1) => {
                if let Type::Ident(ident) = &ty1.inner {
                    let local = !aliases.contains_key(&ident.rust)
                        || impls.contains_key(&ty.impl_key().unwrap());
                    let reason = TrivialReason::VecElement { local };
                    insist_extern_types_are_trivial(ident, reason);
                }
            }
            Type::SliceRef(ty) => {
                if let Type::Ident(ident) = &ty.inner {
                    let reason = TrivialReason::SliceElement(ty);
                    insist_extern_types_are_trivial(ident, reason);
                }
            }
            _ => {}
        }
    }

    required_trivial
}

// Context:
// "type {type} should be trivially move constructible and trivially destructible in C++ to be used as {what} in Rust"
// "needs a cxx::ExternType impl in order to be used as {what}"
pub(crate) fn as_what<'a>(name: &'a Pair, reasons: &'a [TrivialReason]) -> impl Display + 'a {
    struct Description<'a> {
        name: &'a Pair,
        reasons: &'a [TrivialReason<'a>],
    }

    impl<'a> Display for Description<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut field_of = Set::new();
            let mut argument_of = Set::new();
            let mut return_of = Set::new();
            let mut box_target = false;
            let mut vec_element = false;
            let mut slice_shared_element = false;
            let mut slice_mut_element = false;

            for reason in self.reasons {
                match reason {
                    TrivialReason::StructField(strct) => {
                        field_of.insert(&strct.name.rust);
                    }
                    TrivialReason::FunctionArgument(efn) => {
                        argument_of.insert(&efn.name.rust);
                    }
                    TrivialReason::FunctionReturn(efn) => {
                        return_of.insert(&efn.name.rust);
                    }
                    TrivialReason::BoxTarget { .. } => box_target = true,
                    TrivialReason::VecElement { .. } => vec_element = true,
                    TrivialReason::SliceElement(slice) => {
                        if slice.mutable {
                            slice_mut_element = true;
                        } else {
                            slice_shared_element = true;
                        }
                    }
                }
            }

            let mut clauses = Vec::new();
            if !field_of.is_empty() {
                clauses.push(Clause::Set {
                    article: "a",
                    desc: "field of",
                    set: &field_of,
                });
            }
            if !argument_of.is_empty() {
                clauses.push(Clause::Set {
                    article: "an",
                    desc: "argument of",
                    set: &argument_of,
                });
            }
            if !return_of.is_empty() {
                clauses.push(Clause::Set {
                    article: "a",
                    desc: "return value of",
                    set: &return_of,
                });
            }
            if box_target {
                clauses.push(Clause::Ty1 {
                    article: "type",
                    desc: "Box",
                    param: self.name,
                });
            }
            if vec_element {
                clauses.push(Clause::Ty1 {
                    article: "a",
                    desc: "vector element in Vec",
                    param: self.name,
                });
            }
            if slice_shared_element || slice_mut_element {
                clauses.push(Clause::Slice {
                    article: "a",
                    desc: "slice element in",
                    shared: slice_shared_element,
                    mutable: slice_mut_element,
                    param: self.name,
                });
            }

            for (i, clause) in clauses.iter().enumerate() {
                if i == 0 {
                    write!(f, "{} ", clause.article())?;
                } else if i + 1 < clauses.len() {
                    write!(f, ", ")?;
                } else {
                    write!(f, " or ")?;
                }
                clause.fmt(f)?;
            }

            Ok(())
        }
    }

    enum Clause<'a> {
        Set {
            article: &'a str,
            desc: &'a str,
            set: &'a Set<&'a Ident>,
        },
        Ty1 {
            article: &'a str,
            desc: &'a str,
            param: &'a Pair,
        },
        Slice {
            article: &'a str,
            desc: &'a str,
            shared: bool,
            mutable: bool,
            param: &'a Pair,
        },
    }

    impl<'a> Clause<'a> {
        fn article(&self) -> &'a str {
            match self {
                Clause::Set { article, .. }
                | Clause::Ty1 { article, .. }
                | Clause::Slice { article, .. } => article,
            }
        }

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Clause::Set {
                    article: _,
                    desc,
                    set,
                } => {
                    write!(f, "{} ", desc)?;
                    for (i, ident) in set.iter().take(3).enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "`{}`", ident)?;
                    }
                    Ok(())
                }
                Clause::Ty1 {
                    article: _,
                    desc,
                    param,
                } => write!(f, "{}<{}>", desc, param.rust),
                Clause::Slice {
                    article: _,
                    desc,
                    shared,
                    mutable,
                    param,
                } => {
                    write!(f, "{} ", desc)?;
                    if *shared {
                        write!(f, "&[{}]", param.rust)?;
                    }
                    if *shared && *mutable {
                        write!(f, " and ")?;
                    }
                    if *mutable {
                        write!(f, "&mut [{}]", param.rust)?;
                    }
                    Ok(())
                }
            }
        }
    }

    Description { name, reasons }
}
