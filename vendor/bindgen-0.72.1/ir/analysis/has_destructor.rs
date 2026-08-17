//! Determining which types have destructors

use super::{generate_dependencies, ConstrainResult, MonotoneFramework};
use crate::ir::comp::{CompKind, Field, FieldMethods};
use crate::ir::context::{BindgenContext, ItemId};
use crate::ir::traversal::EdgeKind;
use crate::ir::ty::TypeKind;
use crate::{HashMap, HashSet};

/// An analysis that finds for each IR item whether it has a destructor or not
///
/// We use the monotone function `has destructor`, defined as follows:
///
/// * If T is a type alias, a templated alias, or an indirection to another type,
///   T has a destructor if the type T refers to has a destructor.
/// * If T is a compound type, T has a destructor if we saw a destructor when parsing it,
///   or if it's a struct, T has a destructor if any of its base members has a destructor,
///   or if any of its fields have a destructor.
/// * If T is an instantiation of an abstract template definition, T has
///   a destructor if its template definition has a destructor,
///   or if any of the template arguments has a destructor.
/// * If T is the type of a field, that field has a destructor if it's not a bitfield,
///   and if T has a destructor.
#[derive(Debug, Clone)]
pub(crate) struct HasDestructorAnalysis<'ctx> {
    ctx: &'ctx BindgenContext,

    // The incremental result of this analysis's computation. Everything in this
    // set definitely has a destructor.
    have_destructor: HashSet<ItemId>,

    // Dependencies saying that if a key ItemId has been inserted into the
    // `have_destructor` set, then each of the ids in Vec<ItemId> need to be
    // considered again.
    //
    // This is a subset of the natural IR graph with reversed edges, where we
    // only include the edges from the IR graph that can affect whether a type
    // has a destructor or not.
    dependencies: HashMap<ItemId, Vec<ItemId>>,
}

impl HasDestructorAnalysis<'_> {
    fn consider_edge(kind: EdgeKind) -> bool {
        // These are the only edges that can affect whether a type has a
        // destructor or not.
        matches!(
            kind,
            EdgeKind::TypeReference |
                EdgeKind::BaseMember |
                EdgeKind::Field |
                EdgeKind::TemplateArgument |
                EdgeKind::TemplateDeclaration
        )
    }

    fn insert<Id: Into<ItemId>>(&mut self, id: Id) -> ConstrainResult {
        let id = id.into();
        let was_not_already_in_set = self.have_destructor.insert(id);
        assert!(
            was_not_already_in_set,
            "We shouldn't try and insert {id:?} twice because if it was \
             already in the set, `constrain` should have exited early."
        );
        ConstrainResult::Changed
    }
}

impl<'ctx> MonotoneFramework for HasDestructorAnalysis<'ctx> {
    type Node = ItemId;
    type Extra = &'ctx BindgenContext;
    type Output = HashSet<ItemId>;

    fn new(ctx: &'ctx BindgenContext) -> Self {
        let have_destructor = HashSet::default();
        let dependencies = generate_dependencies(ctx, Self::consider_edge);

        HasDestructorAnalysis {
            ctx,
            have_destructor,
            dependencies,
        }
    }

    fn initial_worklist(&self) -> Vec<ItemId> {
        self.ctx.allowlisted_items().iter().copied().collect()
    }

    fn constrain(&mut self, id: ItemId) -> ConstrainResult {
        if self.have_destructor.contains(&id) {
            // We've already computed that this type has a destructor and that can't
            // change.
            return ConstrainResult::Same;
        }

        let item = self.ctx.resolve_item(id);
        let ty = match item.as_type() {
            None => return ConstrainResult::Same,
            Some(ty) => ty,
        };

        match *ty.kind() {
            TypeKind::TemplateAlias(t, _) |
            TypeKind::Alias(t) |
            TypeKind::ResolvedTypeRef(t) => {
                if self.have_destructor.contains(&t.into()) {
                    self.insert(id)
                } else {
                    ConstrainResult::Same
                }
            }

            TypeKind::Comp(ref info) => {
                if info.has_own_destructor() {
                    return self.insert(id);
                }

                match info.kind() {
                    CompKind::Union => ConstrainResult::Same,
                    CompKind::Struct => {
                        let base_or_field_destructor =
                            info.base_members().iter().any(|base| {
                                self.have_destructor.contains(&base.ty.into())
                            }) || info.fields().iter().any(
                                |field| match *field {
                                    Field::DataMember(ref data) => self
                                        .have_destructor
                                        .contains(&data.ty().into()),
                                    Field::Bitfields(_) => false,
                                },
                            );
                        if base_or_field_destructor {
                            self.insert(id)
                        } else {
                            ConstrainResult::Same
                        }
                    }
                }
            }

            TypeKind::TemplateInstantiation(ref inst) => {
                let definition_or_arg_destructor = self
                    .have_destructor
                    .contains(&inst.template_definition().into()) ||
                    inst.template_arguments().iter().any(|arg| {
                        self.have_destructor.contains(&arg.into())
                    });
                if definition_or_arg_destructor {
                    self.insert(id)
                } else {
                    ConstrainResult::Same
                }
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

impl<'ctx> From<HasDestructorAnalysis<'ctx>> for HashSet<ItemId> {
    fn from(analysis: HasDestructorAnalysis<'ctx>) -> Self {
        analysis.have_destructor
    }
}
