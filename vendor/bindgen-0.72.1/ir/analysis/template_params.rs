//! Discover which template type parameters are actually used.
//!
//! ### Why do we care?
//!
//! C++ allows ignoring template parameters, while Rust does not. Usually we can
//! blindly stick a `PhantomData<T>` inside a generic Rust struct to make up for
//! this. That doesn't work for templated type aliases, however:
//!
//! ```C++
//! template <typename T>
//! using Fml = int;
//! ```
//!
//! If we generate the naive Rust code for this alias, we get:
//!
//! ```ignore
//! pub(crate) type Fml<T> = ::std::os::raw::int;
//! ```
//!
//! And this is rejected by `rustc` due to the unused type parameter.
//!
//! (Aside: in these simple cases, `libclang` will often just give us the
//! aliased type directly, and we will never even know we were dealing with
//! aliases, let alone templated aliases. It's the more convoluted scenarios
//! where we get to have some fun...)
//!
//! For such problematic template aliases, we could generate a tuple whose
//! second member is a `PhantomData<T>`. Or, if we wanted to go the extra mile,
//! we could even generate some smarter wrapper that implements `Deref`,
//! `DerefMut`, `From`, `Into`, `AsRef`, and `AsMut` to the actually aliased
//! type. However, this is still lackluster:
//!
//! 1. Even with a billion conversion-trait implementations, using the generated
//!    bindings is rather un-ergonomic.
//! 2. With either of these solutions, we need to keep track of which aliases
//!    we've transformed like this in order to generate correct uses of the
//!    wrapped type.
//!
//! Given that we have to properly track which template parameters ended up used
//! for (2), we might as well leverage that information to make ergonomic
//! bindings that don't contain any unused type parameters at all, and
//! completely avoid the pain of (1).
//!
//! ### How do we determine which template parameters are used?
//!
//! Determining which template parameters are actually used is a trickier
//! problem than it might seem at a glance. On the one hand, trivial uses are
//! easy to detect:
//!
//! ```C++
//! template <typename T>
//! class Foo {
//!     T trivial_use_of_t;
//! };
//! ```
//!
//! It gets harder when determining if one template parameter is used depends on
//! determining if another template parameter is used. In this example, whether
//! `U` is used depends on whether `T` is used.
//!
//! ```C++
//! template <typename T>
//! class DoesntUseT {
//!     int x;
//! };
//!
//! template <typename U>
//! class Fml {
//!     DoesntUseT<U> lololol;
//! };
//! ```
//!
//! We can express the set of used template parameters as a constraint solving
//! problem (where the set of template parameters used by a given IR item is the
//! union of its sub-item's used template parameters) and iterate to a
//! fixed-point.
//!
//! We use the `ir::analysis::MonotoneFramework` infrastructure for this
//! fix-point analysis, where our lattice is the mapping from each IR item to
//! the powerset of the template parameters that appear in the input C++ header,
//! our join function is set union. The set of template parameters appearing in
//! the program is finite, as is the number of IR items. We start at our
//! lattice's bottom element: every item mapping to an empty set of template
//! parameters. Our analysis only adds members to each item's set of used
//! template parameters, never removes them, so it is monotone. Because our
//! lattice is finite and our constraint function is monotone, iteration to a
//! fix-point will terminate.
//!
//! See `src/ir/analysis.rs` for more.

use super::{ConstrainResult, MonotoneFramework};
use crate::ir::context::{BindgenContext, ItemId};
use crate::ir::item::{Item, ItemSet};
use crate::ir::template::{TemplateInstantiation, TemplateParameters};
use crate::ir::traversal::{EdgeKind, Trace};
use crate::ir::ty::TypeKind;
use crate::{HashMap, HashSet};

/// An analysis that finds for each IR item its set of template parameters that
/// it uses.
///
/// We use the monotone constraint function `template_param_usage`, defined as
/// follows:
///
/// * If `T` is a named template type parameter, it trivially uses itself:
///
/// ```ignore
/// template_param_usage(T) = { T }
/// ```
///
/// * If `inst` is a template instantiation, `inst.args` are the template
///   instantiation's template arguments, `inst.def` is the template definition
///   being instantiated, and `inst.def.params` is the template definition's
///   template parameters, then the instantiation's usage is the union of each
///   of its arguments' usages *if* the corresponding template parameter is in
///   turn used by the template definition:
///
/// ```ignore
/// template_param_usage(inst) = union(
///     template_param_usage(inst.args[i])
///         for i in 0..length(inst.args.length)
///             if inst.def.params[i] in template_param_usage(inst.def)
/// )
/// ```
///
/// * Finally, for all other IR item kinds, we use our lattice's `join`
///   operation: set union with each successor of the given item's template
///   parameter usage:
///
/// ```ignore
/// template_param_usage(v) =
///     union(template_param_usage(w) for w in successors(v))
/// ```
///
/// Note that we ignore certain edges in the graph, such as edges from a
/// template declaration to its template parameters' definitions for this
/// analysis. If we didn't, then we would mistakenly determine that ever
/// template parameter is always used.
///
/// The final wrinkle is handling of blocklisted types. Normally, we say that
/// the set of allowlisted items is the transitive closure of items explicitly
/// called out for allowlisting, *without* any items explicitly called out as
/// blocklisted. However, for the purposes of this analysis's correctness, we
/// simplify and consider run the analysis on the full transitive closure of
/// allowlisted items. We do, however, treat instantiations of blocklisted items
/// specially; see `constrain_instantiation_of_blocklisted_template` and its
/// documentation for details.
#[derive(Debug, Clone)]
pub(crate) struct UsedTemplateParameters<'ctx> {
    ctx: &'ctx BindgenContext,

    // The Option is only there for temporary moves out of the hash map. See the
    // comments in `UsedTemplateParameters::constrain` below.
    used: HashMap<ItemId, Option<ItemSet>>,

    dependencies: HashMap<ItemId, Vec<ItemId>>,

    // The set of allowlisted items, without any blocklisted items reachable
    // from the allowlisted items which would otherwise be considered
    // allowlisted as well.
    allowlisted_items: HashSet<ItemId>,
}

impl UsedTemplateParameters<'_> {
    fn consider_edge(kind: EdgeKind) -> bool {
        match kind {
            // For each of these kinds of edges, if the referent uses a template
            // parameter, then it should be considered that the origin of the
            // edge also uses the template parameter.
            EdgeKind::TemplateArgument |
            EdgeKind::BaseMember |
            EdgeKind::Field |
            EdgeKind::Constructor |
            EdgeKind::Destructor |
            EdgeKind::VarType |
            EdgeKind::FunctionReturn |
            EdgeKind::FunctionParameter |
            EdgeKind::TypeReference => true,

            // An inner var or type using a template parameter is orthogonal
            // from whether we use it. See template-param-usage-{6,11}.hpp.
            EdgeKind::InnerVar | EdgeKind::InnerType => false,

            // We can't emit machine code for new monomorphizations of class
            // templates' methods (and don't detect explicit instantiations) so
            // we must ignore template parameters that are only used by
            // methods. This doesn't apply to a function type's return or
            // parameter types, however, because of type aliases of function
            // pointers that use template parameters, eg
            // tests/headers/struct_with_typedef_template_arg.hpp
            EdgeKind::Method => false,

            // If we considered these edges, we would end up mistakenly claiming
            // that every template parameter always used.
            EdgeKind::TemplateDeclaration |
            EdgeKind::TemplateParameterDefinition => false,

            // Since we have to be careful about which edges we consider for
            // this analysis to be correct, we ignore generic edges. We also
            // avoid a `_` wild card to force authors of new edge kinds to
            // determine whether they need to be considered by this analysis.
            EdgeKind::Generic => false,
        }
    }

    fn take_this_id_usage_set<Id: Into<ItemId>>(
        &mut self,
        this_id: Id,
    ) -> ItemSet {
        let this_id = this_id.into();
        self.used
            .get_mut(&this_id)
            .expect(
                "Should have a set of used template params for every item \
                 id",
            )
            .take()
            .expect(
                "Should maintain the invariant that all used template param \
                 sets are `Some` upon entry of `constrain`",
            )
    }

    /// We say that blocklisted items use all of their template parameters. The
    /// blocklisted type is most likely implemented explicitly by the user,
    /// since it won't be in the generated bindings, and we don't know exactly
    /// what they'll to with template parameters, but we can push the issue down
    /// the line to them.
    fn constrain_instantiation_of_blocklisted_template(
        &self,
        this_id: ItemId,
        used_by_this_id: &mut ItemSet,
        instantiation: &TemplateInstantiation,
    ) {
        trace!(
            "    instantiation of blocklisted template, uses all template \
             arguments"
        );

        let args = instantiation
            .template_arguments()
            .iter()
            .map(|a| {
                a.into_resolver()
                    .through_type_refs()
                    .through_type_aliases()
                    .resolve(self.ctx)
                    .id()
            })
            .filter(|a| *a != this_id)
            .flat_map(|a| {
                self.used
                    .get(&a)
                    .expect("Should have a used entry for the template arg")
                    .as_ref()
                    .expect(
                        "Because a != this_id, and all used template \
                         param sets other than this_id's are `Some`, \
                         a's used template param set should be `Some`",
                    )
                    .iter()
            });

        used_by_this_id.extend(args);
    }

    /// A template instantiation's concrete template argument is only used if
    /// the template definition uses the corresponding template parameter.
    fn constrain_instantiation(
        &self,
        this_id: ItemId,
        used_by_this_id: &mut ItemSet,
        instantiation: &TemplateInstantiation,
    ) {
        trace!("    template instantiation");

        let decl = self.ctx.resolve_type(instantiation.template_definition());
        let args = instantiation.template_arguments();

        let params = decl.self_template_params(self.ctx);

        debug_assert!(this_id != instantiation.template_definition());
        let used_by_def = self.used
            .get(&instantiation.template_definition().into())
            .expect("Should have a used entry for instantiation's template definition")
            .as_ref()
            .expect("And it should be Some because only this_id's set is None, and an \
                     instantiation's template definition should never be the \
                     instantiation itself");

        for (arg, param) in args.iter().zip(params.iter()) {
            trace!(
                "      instantiation's argument {arg:?} is used if definition's \
                 parameter {param:?} is used",
            );

            if used_by_def.contains(&param.into()) {
                trace!("        param is used by template definition");

                let arg = arg
                    .into_resolver()
                    .through_type_refs()
                    .through_type_aliases()
                    .resolve(self.ctx)
                    .id();

                if arg == this_id {
                    continue;
                }

                let used_by_arg = self
                    .used
                    .get(&arg)
                    .expect("Should have a used entry for the template arg")
                    .as_ref()
                    .expect(
                        "Because arg != this_id, and all used template \
                         param sets other than this_id's are `Some`, \
                         arg's used template param set should be \
                         `Some`",
                    )
                    .iter();
                used_by_this_id.extend(used_by_arg);
            }
        }
    }

    /// The join operation on our lattice: the set union of all of this ID's
    /// successors.
    fn constrain_join(&self, used_by_this_id: &mut ItemSet, item: &Item) {
        trace!("    other item: join with successors' usage");

        item.trace(
            self.ctx,
            &mut |sub_id, edge_kind| {
                // Ignore ourselves, since union with ourself is a
                // no-op. Ignore edges that aren't relevant to the
                // analysis.
                if sub_id == item.id() || !Self::consider_edge(edge_kind) {
                    return;
                }

                let used_by_sub_id = self
                    .used
                    .get(&sub_id)
                    .expect("Should have a used set for the sub_id successor")
                    .as_ref()
                    .expect(
                        "Because sub_id != id, and all used template \
                         param sets other than id's are `Some`, \
                         sub_id's used template param set should be \
                         `Some`",
                    )
                    .iter();

                trace!(
                    "      union with {sub_id:?}'s usage: {:?}",
                    used_by_sub_id.clone().collect::<Vec<_>>()
                );

                used_by_this_id.extend(used_by_sub_id);
            },
            &(),
        );
    }
}

impl<'ctx> MonotoneFramework for UsedTemplateParameters<'ctx> {
    type Node = ItemId;
    type Extra = &'ctx BindgenContext;
    type Output = HashMap<ItemId, ItemSet>;

    fn new(ctx: &'ctx BindgenContext) -> UsedTemplateParameters<'ctx> {
        let mut used = HashMap::default();
        let mut dependencies = HashMap::default();
        let allowlisted_items: HashSet<_> =
            ctx.allowlisted_items().iter().copied().collect();

        let allowlisted_and_blocklisted_items: ItemSet = allowlisted_items
            .iter()
            .copied()
            .flat_map(|i| {
                let mut reachable = vec![i];
                i.trace(
                    ctx,
                    &mut |s, _| {
                        reachable.push(s);
                    },
                    &(),
                );
                reachable
            })
            .collect();

        for item in allowlisted_and_blocklisted_items {
            dependencies.entry(item).or_insert_with(Vec::new);
            used.entry(item).or_insert_with(|| Some(ItemSet::new()));

            {
                // We reverse our natural IR graph edges to find dependencies
                // between nodes.
                item.trace(
                    ctx,
                    &mut |sub_item: ItemId, _| {
                        used.entry(sub_item)
                            .or_insert_with(|| Some(ItemSet::new()));
                        dependencies
                            .entry(sub_item)
                            .or_insert_with(Vec::new)
                            .push(item);
                    },
                    &(),
                );
            }

            // Additionally, whether a template instantiation's template
            // arguments are used depends on whether the template declaration's
            // generic template parameters are used.
            let item_kind =
                ctx.resolve_item(item).as_type().map(|ty| ty.kind());
            if let Some(TypeKind::TemplateInstantiation(inst)) = item_kind {
                let decl = ctx.resolve_type(inst.template_definition());
                let args = inst.template_arguments();

                // Although template definitions should always have
                // template parameters, there is a single exception:
                // opaque templates. Hence the unwrap_or.
                let params = decl.self_template_params(ctx);

                for (arg, param) in args.iter().zip(params.iter()) {
                    let arg = arg
                        .into_resolver()
                        .through_type_aliases()
                        .through_type_refs()
                        .resolve(ctx)
                        .id();

                    let param = param
                        .into_resolver()
                        .through_type_aliases()
                        .through_type_refs()
                        .resolve(ctx)
                        .id();

                    used.entry(arg).or_insert_with(|| Some(ItemSet::new()));
                    used.entry(param).or_insert_with(|| Some(ItemSet::new()));

                    dependencies
                        .entry(arg)
                        .or_insert_with(Vec::new)
                        .push(param);
                }
            }
        }

        if cfg!(feature = "__testing_only_extra_assertions") {
            // Invariant: The `used` map has an entry for every allowlisted
            // item, as well as all explicitly blocklisted items that are
            // reachable from allowlisted items.
            //
            // Invariant: the `dependencies` map has an entry for every
            // allowlisted item.
            //
            // (This is so that every item we call `constrain` on is guaranteed
            // to have a set of template parameters, and we can allow
            // blocklisted templates to use all of their parameters).
            for item in &allowlisted_items {
                extra_assert!(used.contains_key(item));
                extra_assert!(dependencies.contains_key(item));
                item.trace(
                    ctx,
                    &mut |sub_item, _| {
                        extra_assert!(used.contains_key(&sub_item));
                        extra_assert!(dependencies.contains_key(&sub_item));
                    },
                    &(),
                );
            }
        }

        UsedTemplateParameters {
            ctx,
            used,
            dependencies,
            allowlisted_items,
        }
    }

    fn initial_worklist(&self) -> Vec<ItemId> {
        // The transitive closure of all allowlisted items, including explicitly
        // blocklisted items.
        self.ctx
            .allowlisted_items()
            .iter()
            .copied()
            .flat_map(|i| {
                let mut reachable = vec![i];
                i.trace(
                    self.ctx,
                    &mut |s, _| {
                        reachable.push(s);
                    },
                    &(),
                );
                reachable
            })
            .collect()
    }

    fn constrain(&mut self, id: ItemId) -> ConstrainResult {
        // Invariant: all hash map entries' values are `Some` upon entering and
        // exiting this method.
        extra_assert!(self.used.values().all(|v| v.is_some()));

        // Take the set for this ID out of the hash map while we mutate it based
        // on other hash map entries. We *must* put it back into the hash map at
        // the end of this method. This allows us to side-step HashMap's lack of
        // an analog to slice::split_at_mut.
        let mut used_by_this_id = self.take_this_id_usage_set(id);

        trace!("constrain {id:?}");
        trace!("  initially, used set is {used_by_this_id:?}");

        let original_len = used_by_this_id.len();

        let item = self.ctx.resolve_item(id);
        let ty_kind = item.as_type().map(|ty| ty.kind());
        match ty_kind {
            // Named template type parameters trivially use themselves.
            Some(&TypeKind::TypeParam) => {
                trace!("    named type, trivially uses itself");
                used_by_this_id.insert(id);
            }
            // Template instantiations only use their template arguments if the
            // template definition uses the corresponding template parameter.
            Some(TypeKind::TemplateInstantiation(inst)) => {
                if self
                    .allowlisted_items
                    .contains(&inst.template_definition().into())
                {
                    self.constrain_instantiation(
                        id,
                        &mut used_by_this_id,
                        inst,
                    );
                } else {
                    self.constrain_instantiation_of_blocklisted_template(
                        id,
                        &mut used_by_this_id,
                        inst,
                    );
                }
            }
            // Otherwise, add the union of each of its referent item's template
            // parameter usage.
            _ => self.constrain_join(&mut used_by_this_id, item),
        }

        trace!("  finally, used set is {used_by_this_id:?}");

        let new_len = used_by_this_id.len();
        assert!(
            new_len >= original_len,
            "This is the property that ensures this function is monotone -- \
             if it doesn't hold, the analysis might never terminate!"
        );

        // Put the set back in the hash map and restore our invariant.
        debug_assert!(self.used[&id].is_none());
        self.used.insert(id, Some(used_by_this_id));
        extra_assert!(self.used.values().all(|v| v.is_some()));

        if new_len == original_len {
            ConstrainResult::Same
        } else {
            ConstrainResult::Changed
        }
    }

    fn each_depending_on<F>(&self, item: ItemId, mut f: F)
    where
        F: FnMut(ItemId),
    {
        if let Some(edges) = self.dependencies.get(&item) {
            for item in edges {
                trace!("enqueue {item:?} into worklist");
                f(*item);
            }
        }
    }
}

impl<'ctx> From<UsedTemplateParameters<'ctx>> for HashMap<ItemId, ItemSet> {
    fn from(used_templ_params: UsedTemplateParameters<'ctx>) -> Self {
        used_templ_params
            .used
            .into_iter()
            .map(|(k, v)| (k, v.unwrap()))
            .collect()
    }
}
