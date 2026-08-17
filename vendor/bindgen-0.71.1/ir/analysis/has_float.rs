//! Determining which types has float.

use super::{generate_dependencies, ConstrainResult, MonotoneFramework};
use crate::ir::comp::Field;
use crate::ir::comp::FieldMethods;
use crate::ir::context::{BindgenContext, ItemId};
use crate::ir::traversal::EdgeKind;
use crate::ir::ty::TypeKind;
use crate::{HashMap, HashSet};

/// An analysis that finds for each IR item whether it has float or not.
///
/// We use the monotone constraint function `has_float`,
/// defined as follows:
///
/// * If T is float or complex float, T trivially has.
/// * If T is a type alias, a templated alias or an indirection to another type,
///   it has float if the type T refers to has.
/// * If T is a compound type, it has float if any of base memter or field
///   has.
/// * If T is an instantiation of an abstract template definition, T has
///   float if any of the template arguments or template definition
///   has.
#[derive(Debug, Clone)]
pub(crate) struct HasFloat<'ctx> {
    ctx: &'ctx BindgenContext,

    // The incremental result of this analysis's computation. Everything in this
    // set has float.
    has_float: HashSet<ItemId>,

    // Dependencies saying that if a key ItemId has been inserted into the
    // `has_float` set, then each of the ids in Vec<ItemId> need to be
    // considered again.
    //
    // This is a subset of the natural IR graph with reversed edges, where we
    // only include the edges from the IR graph that can affect whether a type
    // has float or not.
    dependencies: HashMap<ItemId, Vec<ItemId>>,
}

impl HasFloat<'_> {
    fn consider_edge(kind: EdgeKind) -> bool {
        match kind {
            EdgeKind::BaseMember |
            EdgeKind::Field |
            EdgeKind::TypeReference |
            EdgeKind::VarType |
            EdgeKind::TemplateArgument |
            EdgeKind::TemplateDeclaration |
            EdgeKind::TemplateParameterDefinition => true,

            EdgeKind::Constructor |
            EdgeKind::Destructor |
            EdgeKind::FunctionReturn |
            EdgeKind::FunctionParameter |
            EdgeKind::InnerType |
            EdgeKind::InnerVar |
            EdgeKind::Method => false,
            EdgeKind::Generic => false,
        }
    }

    fn insert<Id: Into<ItemId>>(&mut self, id: Id) -> ConstrainResult {
        let id = id.into();
        trace!("inserting {id:?} into the has_float set");

        let was_not_already_in_set = self.has_float.insert(id);
        assert!(
            was_not_already_in_set,
            "We shouldn't try and insert {id:?} twice because if it was \
             already in the set, `constrain` should have exited early."
        );

        ConstrainResult::Changed
    }
}

impl<'ctx> MonotoneFramework for HasFloat<'ctx> {
    type Node = ItemId;
    type Extra = &'ctx BindgenContext;
    type Output = HashSet<ItemId>;

    fn new(ctx: &'ctx BindgenContext) -> HasFloat<'ctx> {
        let has_float = HashSet::default();
        let dependencies = generate_dependencies(ctx, Self::consider_edge);

        HasFloat {
            ctx,
            has_float,
            dependencies,
        }
    }

    fn initial_worklist(&self) -> Vec<ItemId> {
        self.ctx.allowlisted_items().iter().copied().collect()
    }

    fn constrain(&mut self, id: ItemId) -> ConstrainResult {
        trace!("constrain: {id:?}");

        if self.has_float.contains(&id) {
            trace!("    already know it do not have float");
            return ConstrainResult::Same;
        }

        let item = self.ctx.resolve_item(id);
        let Some(ty) = item.as_type() else {
            trace!("    not a type; ignoring");
            return ConstrainResult::Same;
        };

        match *ty.kind() {
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::Int(..) |
            TypeKind::Function(..) |
            TypeKind::Enum(..) |
            TypeKind::Reference(..) |
            TypeKind::TypeParam |
            TypeKind::Opaque |
            TypeKind::Pointer(..) |
            TypeKind::UnresolvedTypeRef(..) |
            TypeKind::ObjCInterface(..) |
            TypeKind::ObjCId |
            TypeKind::ObjCSel => {
                trace!("    simple type that do not have float");
                ConstrainResult::Same
            }

            TypeKind::Float(..) | TypeKind::Complex(..) => {
                trace!("    float type has float");
                self.insert(id)
            }

            TypeKind::Array(t, _) => {
                if self.has_float.contains(&t.into()) {
                    trace!(
                        "    Array with type T that has float also has float"
                    );
                    return self.insert(id);
                }
                trace!("    Array with type T that do not have float also do not have float");
                ConstrainResult::Same
            }
            TypeKind::Vector(t, _) => {
                if self.has_float.contains(&t.into()) {
                    trace!(
                        "    Vector with type T that has float also has float"
                    );
                    return self.insert(id);
                }
                trace!("    Vector with type T that do not have float also do not have float");
                ConstrainResult::Same
            }

            TypeKind::ResolvedTypeRef(t) |
            TypeKind::TemplateAlias(t, _) |
            TypeKind::Alias(t) |
            TypeKind::BlockPointer(t) => {
                if self.has_float.contains(&t.into()) {
                    trace!(
                        "    aliases and type refs to T which have float \
                         also have float"
                    );
                    self.insert(id)
                } else {
                    trace!("    aliases and type refs to T which do not have float \
                            also do not have floaarrayt");
                    ConstrainResult::Same
                }
            }

            TypeKind::Comp(ref info) => {
                let bases_have = info
                    .base_members()
                    .iter()
                    .any(|base| self.has_float.contains(&base.ty.into()));
                if bases_have {
                    trace!("    bases have float, so we also have");
                    return self.insert(id);
                }
                let fields_have = info.fields().iter().any(|f| match *f {
                    Field::DataMember(ref data) => {
                        self.has_float.contains(&data.ty().into())
                    }
                    Field::Bitfields(ref bfu) => bfu
                        .bitfields()
                        .iter()
                        .any(|b| self.has_float.contains(&b.ty().into())),
                });
                if fields_have {
                    trace!("    fields have float, so we also have");
                    return self.insert(id);
                }

                trace!("    comp doesn't have float");
                ConstrainResult::Same
            }

            TypeKind::TemplateInstantiation(ref template) => {
                let args_have = template
                    .template_arguments()
                    .iter()
                    .any(|arg| self.has_float.contains(&arg.into()));
                if args_have {
                    trace!(
                        "    template args have float, so \
                         instantiation also has float"
                    );
                    return self.insert(id);
                }

                let def_has = self
                    .has_float
                    .contains(&template.template_definition().into());
                if def_has {
                    trace!(
                        "    template definition has float, so \
                         instantiation also has"
                    );
                    return self.insert(id);
                }

                trace!("    template instantiation do not have float");
                ConstrainResult::Same
            }
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

impl<'ctx> From<HasFloat<'ctx>> for HashSet<ItemId> {
    fn from(analysis: HasFloat<'ctx>) -> Self {
        analysis.has_float
    }
}
