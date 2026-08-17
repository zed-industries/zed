//! Determining which types has typed parameters in array.

use super::{generate_dependencies, ConstrainResult, MonotoneFramework};
use crate::ir::comp::Field;
use crate::ir::comp::FieldMethods;
use crate::ir::context::{BindgenContext, ItemId};
use crate::ir::traversal::EdgeKind;
use crate::ir::ty::TypeKind;
use crate::{HashMap, HashSet};

/// An analysis that finds for each IR item whether it has array or not.
///
/// We use the monotone constraint function `has_type_parameter_in_array`,
/// defined as follows:
///
/// * If T is Array type with type parameter, T trivially has.
/// * If T is a type alias, a templated alias or an indirection to another type,
///   it has type parameter in array if the type T refers to has.
/// * If T is a compound type, it has array if any of base memter or field
///   has type parameter in array.
/// * If T is an instantiation of an abstract template definition, T has
///   type parameter in array if any of the template arguments or template definition
///   has.
#[derive(Debug, Clone)]
pub(crate) struct HasTypeParameterInArray<'ctx> {
    ctx: &'ctx BindgenContext,

    // The incremental result of this analysis's computation. Everything in this
    // set has array.
    has_type_parameter_in_array: HashSet<ItemId>,

    // Dependencies saying that if a key ItemId has been inserted into the
    // `has_type_parameter_in_array` set, then each of the ids in Vec<ItemId> need to be
    // considered again.
    //
    // This is a subset of the natural IR graph with reversed edges, where we
    // only include the edges from the IR graph that can affect whether a type
    // has array or not.
    dependencies: HashMap<ItemId, Vec<ItemId>>,
}

impl HasTypeParameterInArray<'_> {
    fn consider_edge(kind: EdgeKind) -> bool {
        match kind {
            // These are the only edges that can affect whether a type has type parameter
            // in array or not.
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
        trace!("inserting {id:?} into the has_type_parameter_in_array set");

        let was_not_already_in_set =
            self.has_type_parameter_in_array.insert(id);
        assert!(
            was_not_already_in_set,
            "We shouldn't try and insert {id:?} twice because if it was \
             already in the set, `constrain` should have exited early."
        );

        ConstrainResult::Changed
    }
}

impl<'ctx> MonotoneFramework for HasTypeParameterInArray<'ctx> {
    type Node = ItemId;
    type Extra = &'ctx BindgenContext;
    type Output = HashSet<ItemId>;

    fn new(ctx: &'ctx BindgenContext) -> HasTypeParameterInArray<'ctx> {
        let has_type_parameter_in_array = HashSet::default();
        let dependencies = generate_dependencies(ctx, Self::consider_edge);

        HasTypeParameterInArray {
            ctx,
            has_type_parameter_in_array,
            dependencies,
        }
    }

    fn initial_worklist(&self) -> Vec<ItemId> {
        self.ctx.allowlisted_items().iter().copied().collect()
    }

    fn constrain(&mut self, id: ItemId) -> ConstrainResult {
        trace!("constrain: {id:?}");

        if self.has_type_parameter_in_array.contains(&id) {
            trace!("    already know it do not have array");
            return ConstrainResult::Same;
        }

        let item = self.ctx.resolve_item(id);
        let Some(ty) = item.as_type() else {
            trace!("    not a type; ignoring");
            return ConstrainResult::Same;
        };

        match *ty.kind() {
            // Handle the simple cases. These cannot have array in type parameter
            // without further information.
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::Int(..) |
            TypeKind::Float(..) |
            TypeKind::Vector(..) |
            TypeKind::Complex(..) |
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
                trace!("    simple type that do not have array");
                ConstrainResult::Same
            }

            TypeKind::Array(t, _) => {
                let inner_ty =
                    self.ctx.resolve_type(t).canonical_type(self.ctx);
                if let TypeKind::TypeParam = *inner_ty.kind() {
                    trace!("    Array with Named type has type parameter");
                    self.insert(id)
                } else {
                    trace!(
                        "    Array without Named type does have type parameter"
                    );
                    ConstrainResult::Same
                }
            }

            TypeKind::ResolvedTypeRef(t) |
            TypeKind::TemplateAlias(t, _) |
            TypeKind::Alias(t) |
            TypeKind::BlockPointer(t) => {
                if self.has_type_parameter_in_array.contains(&t.into()) {
                    trace!(
                        "    aliases and type refs to T which have array \
                         also have array"
                    );
                    self.insert(id)
                } else {
                    trace!(
                        "    aliases and type refs to T which do not have array \
                            also do not have array"
                    );
                    ConstrainResult::Same
                }
            }

            TypeKind::Comp(ref info) => {
                let bases_have = info.base_members().iter().any(|base| {
                    self.has_type_parameter_in_array.contains(&base.ty.into())
                });
                if bases_have {
                    trace!("    bases have array, so we also have");
                    return self.insert(id);
                }
                let fields_have = info.fields().iter().any(|f| match *f {
                    Field::DataMember(ref data) => self
                        .has_type_parameter_in_array
                        .contains(&data.ty().into()),
                    Field::Bitfields(..) => false,
                });
                if fields_have {
                    trace!("    fields have array, so we also have");
                    return self.insert(id);
                }

                trace!("    comp doesn't have array");
                ConstrainResult::Same
            }

            TypeKind::TemplateInstantiation(ref template) => {
                let args_have =
                    template.template_arguments().iter().any(|arg| {
                        self.has_type_parameter_in_array.contains(&arg.into())
                    });
                if args_have {
                    trace!(
                        "    template args have array, so \
                         instantiation also has array"
                    );
                    return self.insert(id);
                }

                let def_has = self
                    .has_type_parameter_in_array
                    .contains(&template.template_definition().into());
                if def_has {
                    trace!(
                        "    template definition has array, so \
                         instantiation also has"
                    );
                    return self.insert(id);
                }

                trace!("    template instantiation do not have array");
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

impl<'ctx> From<HasTypeParameterInArray<'ctx>> for HashSet<ItemId> {
    fn from(analysis: HasTypeParameterInArray<'ctx>) -> Self {
        analysis.has_type_parameter_in_array
    }
}
