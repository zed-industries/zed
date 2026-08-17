//! Determining the sizedness of types (as base classes and otherwise).

use super::{
    generate_dependencies, ConstrainResult, HasVtable, MonotoneFramework,
};
use crate::ir::context::{BindgenContext, TypeId};
use crate::ir::item::IsOpaque;
use crate::ir::traversal::EdgeKind;
use crate::ir::ty::TypeKind;
use crate::{Entry, HashMap};
use std::{cmp, ops};

/// The result of the `Sizedness` analysis for an individual item.
///
/// This is a chain lattice of the form:
///
/// ```ignore
///                   NonZeroSized
///                        |
///                DependsOnTypeParam
///                        |
///                     ZeroSized
/// ```
///
/// We initially assume that all types are `ZeroSized` and then update our
/// understanding as we learn more about each type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum SizednessResult {
    /// The type is zero-sized.
    ///
    /// This means that if it is a C++ type, and is not being used as a base
    /// member, then we must add an `_address` byte to enforce the
    /// unique-address-per-distinct-object-instance rule.
    #[default]
    ZeroSized,

    /// Whether this type is zero-sized or not depends on whether a type
    /// parameter is zero-sized or not.
    ///
    /// For example, given these definitions:
    ///
    /// ```c++
    /// template<class T>
    /// class Flongo : public T {};
    ///
    /// class Empty {};
    ///
    /// class NonEmpty { int x; };
    /// ```
    ///
    /// Then `Flongo<Empty>` is zero-sized, and needs an `_address` byte
    /// inserted, while `Flongo<NonEmpty>` is *not* zero-sized, and should *not*
    /// have an `_address` byte inserted.
    ///
    /// We don't properly handle this situation correctly right now:
    /// <https://github.com/rust-lang/rust-bindgen/issues/586>
    DependsOnTypeParam,

    /// Has some size that is known to be greater than zero. That doesn't mean
    /// it has a static size, but it is not zero sized for sure. In other words,
    /// it might contain an incomplete array or some other dynamically sized
    /// type.
    NonZeroSized,
}

impl SizednessResult {
    /// Take the least upper bound of `self` and `rhs`.
    pub(crate) fn join(self, rhs: Self) -> Self {
        cmp::max(self, rhs)
    }
}

impl ops::BitOr for SizednessResult {
    type Output = Self;

    fn bitor(self, rhs: SizednessResult) -> Self::Output {
        self.join(rhs)
    }
}

impl ops::BitOrAssign for SizednessResult {
    fn bitor_assign(&mut self, rhs: SizednessResult) {
        *self = self.join(rhs);
    }
}

/// An analysis that computes the sizedness of all types.
///
/// * For types with known sizes -- for example pointers, scalars, etc... --
///   they are assigned `NonZeroSized`.
///
/// * For compound structure types with one or more fields, they are assigned
///   `NonZeroSized`.
///
/// * For compound structure types without any fields, the results of the bases
///   are `join`ed.
///
/// * For type parameters, `DependsOnTypeParam` is assigned.
#[derive(Debug)]
pub(crate) struct SizednessAnalysis<'ctx> {
    ctx: &'ctx BindgenContext,
    dependencies: HashMap<TypeId, Vec<TypeId>>,
    // Incremental results of the analysis. Missing entries are implicitly
    // considered `ZeroSized`.
    sized: HashMap<TypeId, SizednessResult>,
}

impl SizednessAnalysis<'_> {
    fn consider_edge(kind: EdgeKind) -> bool {
        // These are the only edges that can affect whether a type is
        // zero-sized or not.
        matches!(
            kind,
            EdgeKind::TemplateArgument |
                EdgeKind::TemplateParameterDefinition |
                EdgeKind::TemplateDeclaration |
                EdgeKind::TypeReference |
                EdgeKind::BaseMember |
                EdgeKind::Field
        )
    }

    /// Insert an incremental result, and return whether this updated our
    /// knowledge of types and we should continue the analysis.
    fn insert(
        &mut self,
        id: TypeId,
        result: SizednessResult,
    ) -> ConstrainResult {
        trace!("inserting {result:?} for {id:?}");

        if let SizednessResult::ZeroSized = result {
            return ConstrainResult::Same;
        }

        match self.sized.entry(id) {
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

    fn forward(&mut self, from: TypeId, to: TypeId) -> ConstrainResult {
        match self.sized.get(&from) {
            None => ConstrainResult::Same,
            Some(r) => self.insert(to, *r),
        }
    }
}

impl<'ctx> MonotoneFramework for SizednessAnalysis<'ctx> {
    type Node = TypeId;
    type Extra = &'ctx BindgenContext;
    type Output = HashMap<TypeId, SizednessResult>;

    fn new(ctx: &'ctx BindgenContext) -> SizednessAnalysis<'ctx> {
        let dependencies = generate_dependencies(ctx, Self::consider_edge)
            .into_iter()
            .filter_map(|(id, sub_ids)| {
                id.as_type_id(ctx).map(|id| {
                    (
                        id,
                        sub_ids
                            .into_iter()
                            .filter_map(|s| s.as_type_id(ctx))
                            .collect::<Vec<_>>(),
                    )
                })
            })
            .collect();

        let sized = HashMap::default();

        SizednessAnalysis {
            ctx,
            dependencies,
            sized,
        }
    }

    fn initial_worklist(&self) -> Vec<TypeId> {
        self.ctx
            .allowlisted_items()
            .iter()
            .filter_map(|id| id.as_type_id(self.ctx))
            .collect()
    }

    fn constrain(&mut self, id: TypeId) -> ConstrainResult {
        trace!("constrain {id:?}");

        if let Some(SizednessResult::NonZeroSized) = self.sized.get(&id) {
            trace!("    already know it is not zero-sized");
            return ConstrainResult::Same;
        }

        if id.has_vtable_ptr(self.ctx) {
            trace!("    has an explicit vtable pointer, therefore is not zero-sized");
            return self.insert(id, SizednessResult::NonZeroSized);
        }

        let ty = self.ctx.resolve_type(id);

        if id.is_opaque(self.ctx, &()) {
            trace!("    type is opaque; checking layout...");
            let result =
                ty.layout(self.ctx).map_or(SizednessResult::ZeroSized, |l| {
                    if l.size == 0 {
                        trace!("    ...layout has size == 0");
                        SizednessResult::ZeroSized
                    } else {
                        trace!("    ...layout has size > 0");
                        SizednessResult::NonZeroSized
                    }
                });
            return self.insert(id, result);
        }

        match *ty.kind() {
            TypeKind::Void => {
                trace!("    void is zero-sized");
                self.insert(id, SizednessResult::ZeroSized)
            }

            TypeKind::TypeParam => {
                trace!(
                    "    type params sizedness depends on what they're \
                     instantiated as"
                );
                self.insert(id, SizednessResult::DependsOnTypeParam)
            }

            TypeKind::Int(..) |
            TypeKind::Float(..) |
            TypeKind::Complex(..) |
            TypeKind::Function(..) |
            TypeKind::Enum(..) |
            TypeKind::Reference(..) |
            TypeKind::NullPtr |
            TypeKind::ObjCId |
            TypeKind::ObjCSel |
            TypeKind::Pointer(..) => {
                trace!("    {:?} is known not to be zero-sized", ty.kind());
                self.insert(id, SizednessResult::NonZeroSized)
            }

            TypeKind::ObjCInterface(..) => {
                trace!("    obj-c interfaces always have at least the `isa` pointer");
                self.insert(id, SizednessResult::NonZeroSized)
            }

            TypeKind::TemplateAlias(t, _) |
            TypeKind::Alias(t) |
            TypeKind::BlockPointer(t) |
            TypeKind::ResolvedTypeRef(t) => {
                trace!("    aliases and type refs forward to their inner type");
                self.forward(t, id)
            }

            TypeKind::TemplateInstantiation(ref inst) => {
                trace!(
                    "    template instantiations are zero-sized if their \
                     definition is zero-sized"
                );
                self.forward(inst.template_definition(), id)
            }

            TypeKind::Array(_, 0) => {
                trace!("    arrays of zero elements are zero-sized");
                self.insert(id, SizednessResult::ZeroSized)
            }
            TypeKind::Array(..) => {
                trace!("    arrays of > 0 elements are not zero-sized");
                self.insert(id, SizednessResult::NonZeroSized)
            }
            TypeKind::Vector(..) => {
                trace!("    vectors are not zero-sized");
                self.insert(id, SizednessResult::NonZeroSized)
            }

            TypeKind::Comp(ref info) => {
                trace!("    comp considers its own fields and bases");

                if !info.fields().is_empty() {
                    return self.insert(id, SizednessResult::NonZeroSized);
                }

                let result = info
                    .base_members()
                    .iter()
                    .filter_map(|base| self.sized.get(&base.ty))
                    .fold(SizednessResult::ZeroSized, |a, b| a.join(*b));

                self.insert(id, result)
            }

            TypeKind::Opaque => {
                unreachable!("covered by the .is_opaque() check above")
            }

            TypeKind::UnresolvedTypeRef(..) => {
                unreachable!("Should have been resolved after parsing!");
            }
        }
    }

    fn each_depending_on<F>(&self, id: TypeId, mut f: F)
    where
        F: FnMut(TypeId),
    {
        if let Some(edges) = self.dependencies.get(&id) {
            for ty in edges {
                trace!("enqueue {ty:?} into worklist");
                f(*ty);
            }
        }
    }
}

impl<'ctx> From<SizednessAnalysis<'ctx>> for HashMap<TypeId, SizednessResult> {
    fn from(analysis: SizednessAnalysis<'ctx>) -> Self {
        // We let the lack of an entry mean "ZeroSized" to save space.
        extra_assert!(analysis
            .sized
            .values()
            .all(|v| { *v != SizednessResult::ZeroSized }));

        analysis.sized
    }
}

/// A convenience trait for querying whether some type or ID is sized.
///
/// This is not for _computing_ whether the thing is sized, it is for looking up
/// the results of the `Sizedness` analysis's computations for a specific thing.
pub(crate) trait Sizedness {
    /// Get the sizedness of this type.
    fn sizedness(&self, ctx: &BindgenContext) -> SizednessResult;

    /// Is the sizedness for this type `SizednessResult::ZeroSized`?
    fn is_zero_sized(&self, ctx: &BindgenContext) -> bool {
        self.sizedness(ctx) == SizednessResult::ZeroSized
    }
}
