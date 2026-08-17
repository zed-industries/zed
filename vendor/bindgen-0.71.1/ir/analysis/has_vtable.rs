//! Determining which types has vtable

use super::{generate_dependencies, ConstrainResult, MonotoneFramework};
use crate::ir::context::{BindgenContext, ItemId};
use crate::ir::traversal::EdgeKind;
use crate::ir::ty::TypeKind;
use crate::{Entry, HashMap};
use std::cmp;
use std::ops;

/// The result of the `HasVtableAnalysis` for an individual item.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum HasVtableResult {
    /// The item does not have a vtable pointer.
    #[default]
    No,

    /// The item has a vtable and the actual vtable pointer is within this item.
    SelfHasVtable,

    /// The item has a vtable, but the actual vtable pointer is in a base
    /// member.
    BaseHasVtable,
}

impl HasVtableResult {
    /// Take the least upper bound of `self` and `rhs`.
    pub(crate) fn join(self, rhs: Self) -> Self {
        cmp::max(self, rhs)
    }
}

impl ops::BitOr for HasVtableResult {
    type Output = Self;

    fn bitor(self, rhs: HasVtableResult) -> Self::Output {
        self.join(rhs)
    }
}

impl ops::BitOrAssign for HasVtableResult {
    fn bitor_assign(&mut self, rhs: HasVtableResult) {
        *self = self.join(rhs);
    }
}

/// An analysis that finds for each IR item whether it has vtable or not
///
/// We use the monotone function `has vtable`, defined as follows:
///
/// * If T is a type alias, a templated alias, an indirection to another type,
///   or a reference of a type, T has vtable if the type T refers to has vtable.
/// * If T is a compound type, T has vtable if we saw a virtual function when
///   parsing it or any of its base member has vtable.
/// * If T is an instantiation of an abstract template definition, T has
///   vtable if template definition has vtable
#[derive(Debug, Clone)]
pub(crate) struct HasVtableAnalysis<'ctx> {
    ctx: &'ctx BindgenContext,

    // The incremental result of this analysis's computation. Everything in this
    // set definitely has a vtable.
    have_vtable: HashMap<ItemId, HasVtableResult>,

    // Dependencies saying that if a key ItemId has been inserted into the
    // `have_vtable` set, then each of the ids in Vec<ItemId> need to be
    // considered again.
    //
    // This is a subset of the natural IR graph with reversed edges, where we
    // only include the edges from the IR graph that can affect whether a type
    // has a vtable or not.
    dependencies: HashMap<ItemId, Vec<ItemId>>,
}

impl HasVtableAnalysis<'_> {
    fn consider_edge(kind: EdgeKind) -> bool {
        // These are the only edges that can affect whether a type has a
        // vtable or not.
        matches!(
            kind,
            EdgeKind::TypeReference |
                EdgeKind::BaseMember |
                EdgeKind::TemplateDeclaration
        )
    }

    fn insert<Id: Into<ItemId>>(
        &mut self,
        id: Id,
        result: HasVtableResult,
    ) -> ConstrainResult {
        if let HasVtableResult::No = result {
            return ConstrainResult::Same;
        }

        let id = id.into();
        match self.have_vtable.entry(id) {
            Entry::Occupied(mut entry) => {
                if *entry.get() < result {
                    entry.insert(result);
                    ConstrainResult::Changed
                } else {
                    ConstrainResult::Same
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(result);
                ConstrainResult::Changed
            }
        }
    }

    fn forward<Id1, Id2>(&mut self, from: Id1, to: Id2) -> ConstrainResult
    where
        Id1: Into<ItemId>,
        Id2: Into<ItemId>,
    {
        let from = from.into();
        let to = to.into();

        match self.have_vtable.get(&from) {
            None => ConstrainResult::Same,
            Some(r) => self.insert(to, *r),
        }
    }
}

impl<'ctx> MonotoneFramework for HasVtableAnalysis<'ctx> {
    type Node = ItemId;
    type Extra = &'ctx BindgenContext;
    type Output = HashMap<ItemId, HasVtableResult>;

    fn new(ctx: &'ctx BindgenContext) -> HasVtableAnalysis<'ctx> {
        let have_vtable = HashMap::default();
        let dependencies = generate_dependencies(ctx, Self::consider_edge);

        HasVtableAnalysis {
            ctx,
            have_vtable,
            dependencies,
        }
    }

    fn initial_worklist(&self) -> Vec<ItemId> {
        self.ctx.allowlisted_items().iter().copied().collect()
    }

    fn constrain(&mut self, id: ItemId) -> ConstrainResult {
        trace!("constrain {id:?}");

        let item = self.ctx.resolve_item(id);
        let ty = match item.as_type() {
            None => return ConstrainResult::Same,
            Some(ty) => ty,
        };

        // TODO #851: figure out a way to handle deriving from template type parameters.
        match *ty.kind() {
            TypeKind::TemplateAlias(t, _) |
            TypeKind::Alias(t) |
            TypeKind::ResolvedTypeRef(t) |
            TypeKind::Reference(t) => {
                trace!(
                    "    aliases and references forward to their inner type"
                );
                self.forward(t, id)
            }

            TypeKind::Comp(ref info) => {
                trace!("    comp considers its own methods and bases");
                let mut result = HasVtableResult::No;

                if info.has_own_virtual_method() {
                    trace!("    comp has its own virtual method");
                    result |= HasVtableResult::SelfHasVtable;
                }

                let bases_has_vtable = info.base_members().iter().any(|base| {
                    trace!("    comp has a base with a vtable: {base:?}");
                    self.have_vtable.contains_key(&base.ty.into())
                });
                if bases_has_vtable {
                    result |= HasVtableResult::BaseHasVtable;
                }

                self.insert(id, result)
            }

            TypeKind::TemplateInstantiation(ref inst) => {
                self.forward(inst.template_definition(), id)
            }

            _ => ConstrainResult::Same,
        }
    }

    fn each_depending_on<F>(&self, id: ItemId, mut f: F)
    where
        F: FnMut(ItemId),
    {
        if let Some(edges) = self.dependencies.get(&id) {
            for item in edges {
                trace!("enqueue {item:?} into worklist");
                f(*item);
            }
        }
    }
}

impl<'ctx> From<HasVtableAnalysis<'ctx>> for HashMap<ItemId, HasVtableResult> {
    fn from(analysis: HasVtableAnalysis<'ctx>) -> Self {
        // We let the lack of an entry mean "No" to save space.
        extra_assert!(analysis
            .have_vtable
            .values()
            .all(|v| { *v != HasVtableResult::No }));

        analysis.have_vtable
    }
}

/// A convenience trait for the things for which we might wonder if they have a
/// vtable during codegen.
///
/// This is not for _computing_ whether the thing has a vtable, it is for
/// looking up the results of the `HasVtableAnalysis`'s computations for a
/// specific thing.
pub(crate) trait HasVtable {
    /// Return `true` if this thing has vtable, `false` otherwise.
    fn has_vtable(&self, ctx: &BindgenContext) -> bool;

    /// Return `true` if this thing has an actual vtable pointer in itself, as
    /// opposed to transitively in a base member.
    fn has_vtable_ptr(&self, ctx: &BindgenContext) -> bool;
}
