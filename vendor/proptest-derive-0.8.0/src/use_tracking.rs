// Copyright 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides `UseTracker` as well as `UseMarkable` which is used to
//! track uses of type variables that need `Arbitrary` bounds in our
//! impls.

use std::borrow::Borrow;
use std::collections::HashSet;

use syn;

use crate::attr;
use crate::error::{Ctx, DeriveResult};
use crate::util;

//==============================================================================
// API: Type variable use tracking
//==============================================================================

/// `UseTracker` tracks what type variables that have used in `any_with::<Type>`
/// or similar and thus needs an `Arbitrary` bound added to them.
pub struct UseTracker {
    /// Tracks 'usage' of a type variable name.
    /// Allocation of this "map" will happen at once and no further
    /// allocation will happen after that. Only potential updates
    /// will happen after initial allocation.
    /// We need to preserve insertion order, thus using Vec instead of BTreeMap
    /// or HashMap. A potential alternative would be indexmap crate, but our
    /// maps are so small that it would not bring any significant benefit.
    used_map: Vec<(syn::Ident, bool)>,
    /// Extra types to bound by `Arbitrary` in the `where` clause.
    where_types: HashSet<syn::Type>,
    /// The generics that we are doing this for.
    /// This what we will modify later once we're done.
    generics: syn::Generics,
    /// If set to `true`, then `mark_used` has no effect.
    track: bool,
}

/// Models a thing that may have type variables in it that
/// can be marked as 'used' as defined by `UseTracker`.
pub trait UseMarkable {
    fn mark_uses(&self, tracker: &mut UseTracker);
}

impl UseTracker {
    /// Constructs the tracker for the given `generics`.
    pub fn new(generics: syn::Generics) -> Self {
        // Construct the map by setting all type variables as being unused
        // initially. This is the only time we will allocate for the map.
        let used_map = generics
            .type_params()
            .map(|v| (v.ident.clone(), false))
            .collect();
        Self {
            generics,
            used_map,
            where_types: HashSet::default(),
            track: true,
        }
    }

    /// Stop tracking. `.mark_used` will have no effect.
    pub fn no_track(&mut self) {
        self.track = false;
    }

    /// Mark the _potential_ type variable `tyvar` as used.
    /// If the tracker does not know about the name, it is not
    /// a type variable and this call has no effect.
    fn use_tyvar(&mut self, tyvar: impl Borrow<syn::Ident>) {
        if self.track {
            if let Some(used) = self
                .used_map
                .iter_mut()
                .find_map(|(ty, used)| (ty == tyvar.borrow()).then(|| used))
            {
                *used = true;
            }
        }
    }

    /// Returns true iff the type variable given exists.
    fn has_tyvar(&self, ty_var: impl Borrow<syn::Ident>) -> bool {
        self.used_map.iter().any(|(ty, _)| ty == ty_var.borrow())
    }

    /// Mark the type as used.
    fn use_type(&mut self, ty: syn::Type) {
        self.where_types.insert(ty);
    }

    /// Adds the bound in `for_used` on used type variables and
    /// the bound in `for_not` (`if .is_some()`) on unused type variables.
    pub fn add_bounds(
        &mut self,
        ctx: Ctx,
        for_used: &syn::TypeParamBound,
        for_not: Option<syn::TypeParamBound>,
    ) -> DeriveResult<()> {
        {
            let mut iter = self
                .used_map
                .iter()
                .map(|(_, used)| used)
                .zip(self.generics.type_params_mut());
            if let Some(for_not) = for_not {
                iter.try_for_each(|(&used, tv)| {
                    // Steal the attributes:
                    let no_bound = attr::has_no_bound(ctx, &tv.attrs)?;
                    let bound = if used && !no_bound {
                        for_used
                    } else {
                        &for_not
                    };
                    tv.bounds.push(bound.clone());
                    Ok(())
                })?;
            } else {
                iter.for_each(|(&used, tv)| {
                    if used {
                        tv.bounds.push(for_used.clone())
                    }
                })
            }
        }

        self.generics.make_where_clause().predicates.extend(
            self.where_types.iter().cloned().map(|ty| {
                syn::WherePredicate::Type(syn::PredicateType {
                    lifetimes: None,
                    bounded_ty: ty,
                    colon_token: <Token![:]>::default(),
                    bounds: ::std::iter::once(for_used.clone()).collect(),
                })
            }),
        );

        Ok(())
    }

    /// Consumes the (potentially) modified generics that the
    /// tracker was originally constructed with and returns it.
    pub fn consume(self) -> syn::Generics {
        self.generics
    }
}

//==============================================================================
// Impls
//==============================================================================

impl UseMarkable for syn::Type {
    fn mark_uses(&self, ut: &mut UseTracker) {
        use syn::visit;

        visit::visit_type(&mut PathVisitor(ut), self);

        struct PathVisitor<'ut>(&'ut mut UseTracker);

        impl<'ut, 'ast> visit::Visit<'ast> for PathVisitor<'ut> {
            fn visit_macro(&mut self, _: &syn::Macro) {}

            fn visit_type_path(&mut self, tpath: &syn::TypePath) {
                if matches_prj_tyvar(self.0, tpath) {
                    self.0.use_type(adjust_simple_prj(tpath).into());
                    return;
                }
                visit::visit_type_path(self, tpath);
            }

            fn visit_path(&mut self, path: &syn::Path) {
                // If path is PhantomData<T> do not mark innards.
                if util::is_phantom_data(path) {
                    return;
                }

                if let Some(ident) = util::extract_simple_path(path) {
                    self.0.use_tyvar(ident);
                }

                visit::visit_path(self, path);
            }
        }
    }
}

fn matches_prj_tyvar(ut: &mut UseTracker, tpath: &syn::TypePath) -> bool {
    let path = &tpath.path;
    let segs = &path.segments;

    if let Some(qself) = &tpath.qself {
        // < $qself > :: $path
        if let Some(sub_tp) = extract_path(&qself.ty) {
            return sub_tp.qself.is_none()
                && util::match_singleton(segs.iter().skip(qself.position))
                    .filter(|ps| ps.arguments.is_empty())
                    .and_then(|_| util::extract_simple_path(&sub_tp.path))
                    .filter(|&ident| ut.has_tyvar(ident))
                    .is_some() // < $tyvar as? $path? > :: $path
                || matches_prj_tyvar(ut, sub_tp);
        }

        false
    } else {
        // true => $tyvar :: $projection
        return !util::path_is_global(path)
            && segs.len() == 2
            && ut.has_tyvar(&segs[0].ident)
            && segs[0].arguments.is_empty()
            && segs[1].arguments.is_empty();
    }
}

fn adjust_simple_prj(tpath: &syn::TypePath) -> syn::TypePath {
    let segments = tpath
        .qself
        .as_ref()
        .filter(|qp| qp.as_token.is_none())
        .and_then(|qp| extract_path(&*qp.ty))
        .filter(|tp| tp.qself.is_none())
        .map(|tp| &tp.path.segments);

    if let Some(segments) = segments {
        let tpath = tpath.clone();
        let mut segments = segments.clone();
        segments.push_punct(<Token![::]>::default());
        segments.extend(tpath.path.segments.into_pairs());
        syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments,
            },
        }
    } else {
        tpath.clone()
    }
}

fn extract_path(ty: &syn::Type) -> Option<&syn::TypePath> {
    if let syn::Type::Path(tpath) = ty {
        Some(tpath)
    } else {
        None
    }
}
