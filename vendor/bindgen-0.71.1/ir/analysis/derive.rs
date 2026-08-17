//! Determining which types for which we cannot emit `#[derive(Trait)]`.

use std::fmt;

use super::{generate_dependencies, ConstrainResult, MonotoneFramework};
use crate::ir::analysis::has_vtable::HasVtable;
use crate::ir::comp::CompKind;
use crate::ir::context::{BindgenContext, ItemId};
use crate::ir::derive::CanDerive;
use crate::ir::function::FunctionSig;
use crate::ir::item::{IsOpaque, Item};
use crate::ir::layout::Layout;
use crate::ir::template::TemplateParameters;
use crate::ir::traversal::{EdgeKind, Trace};
use crate::ir::ty::RUST_DERIVE_IN_ARRAY_LIMIT;
use crate::ir::ty::{Type, TypeKind};
use crate::{Entry, HashMap, HashSet};

/// Which trait to consider when doing the `CannotDerive` analysis.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DeriveTrait {
    /// The `Copy` trait.
    Copy,
    /// The `Debug` trait.
    Debug,
    /// The `Default` trait.
    Default,
    /// The `Hash` trait.
    Hash,
    /// The `PartialEq` and `PartialOrd` traits.
    PartialEqOrPartialOrd,
}

/// An analysis that finds for each IR item whether a trait cannot be derived.
///
/// We use the monotone constraint function `cannot_derive`, defined as follows
/// for type T:
///
/// * If T is Opaque and the layout of the type is known, get this layout as an
///   opaquetype and check whether it can derive using trivial checks.
///
/// * If T is Array, a trait cannot be derived if the array is incomplete,
///   if the length of the array is larger than the limit (unless the trait
///   allows it), or the trait cannot be derived for the type of data the array
///   contains.
///
/// * If T is Vector, a trait cannot be derived if the trait cannot be derived
///   for the type of data the vector contains.
///
/// * If T is a type alias, a templated alias or an indirection to another type,
///   the trait cannot be derived if the trait cannot be derived for type T
///   refers to.
///
/// * If T is a compound type, the trait cannot be derived if the trait cannot
///   be derived for any of its base members or fields.
///
/// * If T is an instantiation of an abstract template definition, the trait
///   cannot be derived if any of the template arguments or template definition
///   cannot derive the trait.
///
/// * For all other (simple) types, compiler and standard library limitations
///   dictate whether the trait is implemented.
#[derive(Debug, Clone)]
pub(crate) struct CannotDerive<'ctx> {
    ctx: &'ctx BindgenContext,

    derive_trait: DeriveTrait,

    // The incremental result of this analysis's computation.
    // Contains information whether particular item can derive `derive_trait`
    can_derive: HashMap<ItemId, CanDerive>,

    // Dependencies saying that if a key ItemId has been inserted into the
    // `cannot_derive_partialeq_or_partialord` set, then each of the ids
    // in Vec<ItemId> need to be considered again.
    //
    // This is a subset of the natural IR graph with reversed edges, where we
    // only include the edges from the IR graph that can affect whether a type
    // can derive `derive_trait`.
    dependencies: HashMap<ItemId, Vec<ItemId>>,
}

type EdgePredicate = fn(EdgeKind) -> bool;

fn consider_edge_default(kind: EdgeKind) -> bool {
    match kind {
        // These are the only edges that can affect whether a type can derive
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
        EdgeKind::Method |
        EdgeKind::Generic => false,
    }
}

impl CannotDerive<'_> {
    fn insert<Id: Into<ItemId>>(
        &mut self,
        id: Id,
        can_derive: CanDerive,
    ) -> ConstrainResult {
        let id = id.into();
        trace!(
            "inserting {id:?} can_derive<{}>={can_derive:?}",
            self.derive_trait,
        );

        if let CanDerive::Yes = can_derive {
            return ConstrainResult::Same;
        }

        match self.can_derive.entry(id) {
            Entry::Occupied(mut entry) => {
                if *entry.get() < can_derive {
                    entry.insert(can_derive);
                    ConstrainResult::Changed
                } else {
                    ConstrainResult::Same
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(can_derive);
                ConstrainResult::Changed
            }
        }
    }

    fn constrain_type(&mut self, item: &Item, ty: &Type) -> CanDerive {
        if !self.ctx.allowlisted_items().contains(&item.id()) {
            let can_derive = self
                .ctx
                .blocklisted_type_implements_trait(item, self.derive_trait);
            match can_derive {
                CanDerive::Yes => trace!(
                    "    blocklisted type explicitly implements {}",
                    self.derive_trait
                ),
                CanDerive::Manually => trace!(
                    "    blocklisted type requires manual implementation of {}",
                    self.derive_trait
                ),
                CanDerive::No => trace!(
                    "    cannot derive {} for blocklisted type",
                    self.derive_trait
                ),
            }
            return can_derive;
        }

        if self.derive_trait.not_by_name(self.ctx, item) {
            trace!(
                "    cannot derive {} for explicitly excluded type",
                self.derive_trait
            );
            return CanDerive::No;
        }

        trace!("ty: {ty:?}");
        if item.is_opaque(self.ctx, &()) {
            if !self.derive_trait.can_derive_union() &&
                ty.is_union() &&
                self.ctx.options().untagged_union
            {
                trace!(
                    "    cannot derive {} for Rust unions",
                    self.derive_trait
                );
                return CanDerive::No;
            }

            let layout_can_derive =
                ty.layout(self.ctx).map_or(CanDerive::Yes, |l| {
                    l.opaque().array_size_within_derive_limit()
                });

            match layout_can_derive {
                CanDerive::Yes => {
                    trace!(
                        "    we can trivially derive {} for the layout",
                        self.derive_trait
                    );
                }
                _ => {
                    trace!(
                        "    we cannot derive {} for the layout",
                        self.derive_trait
                    );
                }
            };
            return layout_can_derive;
        }

        match *ty.kind() {
            // Handle the simple cases. These can derive traits without further
            // information.
            TypeKind::Void |
            TypeKind::NullPtr |
            TypeKind::Int(..) |
            TypeKind::Complex(..) |
            TypeKind::Float(..) |
            TypeKind::Enum(..) |
            TypeKind::TypeParam |
            TypeKind::UnresolvedTypeRef(..) |
            TypeKind::Reference(..) |
            TypeKind::ObjCInterface(..) |
            TypeKind::ObjCId |
            TypeKind::ObjCSel => self.derive_trait.can_derive_simple(ty.kind()),
            TypeKind::Pointer(inner) => {
                let inner_type =
                    self.ctx.resolve_type(inner).canonical_type(self.ctx);
                if let TypeKind::Function(ref sig) = *inner_type.kind() {
                    self.derive_trait.can_derive_fnptr(sig)
                } else {
                    self.derive_trait.can_derive_pointer()
                }
            }
            TypeKind::Function(ref sig) => {
                self.derive_trait.can_derive_fnptr(sig)
            }

            // Complex cases need more information
            TypeKind::Array(t, len) => {
                let inner_type =
                    self.can_derive.get(&t.into()).copied().unwrap_or_default();
                if inner_type != CanDerive::Yes {
                    trace!(
                        "    arrays of T for which we cannot derive {} \
                         also cannot derive {}",
                        self.derive_trait,
                        self.derive_trait
                    );
                    return CanDerive::No;
                }

                if len == 0 && !self.derive_trait.can_derive_incomplete_array()
                {
                    trace!(
                        "    cannot derive {} for incomplete arrays",
                        self.derive_trait
                    );
                    return CanDerive::No;
                }

                if self.derive_trait.can_derive_large_array(self.ctx) {
                    trace!("    array can derive {}", self.derive_trait);
                    return CanDerive::Yes;
                }

                if len > RUST_DERIVE_IN_ARRAY_LIMIT {
                    trace!(
                        "    array is too large to derive {}, but it may be implemented", self.derive_trait
                    );
                    return CanDerive::Manually;
                }
                trace!(
                    "    array is small enough to derive {}",
                    self.derive_trait
                );
                CanDerive::Yes
            }
            TypeKind::Vector(t, len) => {
                let inner_type =
                    self.can_derive.get(&t.into()).copied().unwrap_or_default();
                if inner_type != CanDerive::Yes {
                    trace!(
                        "    vectors of T for which we cannot derive {} \
                         also cannot derive {}",
                        self.derive_trait,
                        self.derive_trait
                    );
                    return CanDerive::No;
                }
                assert_ne!(len, 0, "vectors cannot have zero length");
                self.derive_trait.can_derive_vector()
            }

            TypeKind::Comp(ref info) => {
                assert!(
                    !info.has_non_type_template_params(),
                    "The early ty.is_opaque check should have handled this case"
                );

                if !self.derive_trait.can_derive_compound_forward_decl() &&
                    info.is_forward_declaration()
                {
                    trace!(
                        "    cannot derive {} for forward decls",
                        self.derive_trait
                    );
                    return CanDerive::No;
                }

                // NOTE: Take into account that while unions in C and C++ are copied by
                // default, the may have an explicit destructor in C++, so we can't
                // defer this check just for the union case.
                if !self.derive_trait.can_derive_compound_with_destructor() &&
                    self.ctx.lookup_has_destructor(
                        item.id().expect_type_id(self.ctx),
                    )
                {
                    trace!(
                        "    comp has destructor which cannot derive {}",
                        self.derive_trait
                    );
                    return CanDerive::No;
                }

                if info.kind() == CompKind::Union {
                    if self.derive_trait.can_derive_union() {
                        if self.ctx.options().untagged_union &&
                           // https://github.com/rust-lang/rust/issues/36640
                           (!info.self_template_params(self.ctx).is_empty() ||
                            !item.all_template_params(self.ctx).is_empty())
                        {
                            trace!(
                                "    cannot derive {} for Rust union because issue 36640", self.derive_trait
                            );
                            return CanDerive::No;
                        }
                    // fall through to be same as non-union handling
                    } else {
                        if self.ctx.options().untagged_union {
                            trace!(
                                "    cannot derive {} for Rust unions",
                                self.derive_trait
                            );
                            return CanDerive::No;
                        }

                        let layout_can_derive =
                            ty.layout(self.ctx).map_or(CanDerive::Yes, |l| {
                                l.opaque().array_size_within_derive_limit()
                            });
                        match layout_can_derive {
                            CanDerive::Yes => {
                                trace!(
                                    "    union layout can trivially derive {}",
                                    self.derive_trait
                                );
                            }
                            _ => {
                                trace!(
                                    "    union layout cannot derive {}",
                                    self.derive_trait
                                );
                            }
                        };
                        return layout_can_derive;
                    }
                }

                if !self.derive_trait.can_derive_compound_with_vtable() &&
                    item.has_vtable(self.ctx)
                {
                    trace!(
                        "    cannot derive {} for comp with vtable",
                        self.derive_trait
                    );
                    return CanDerive::No;
                }

                // Bitfield units are always represented as arrays of u8, but
                // they're not traced as arrays, so we need to check here
                // instead.
                if !self.derive_trait.can_derive_large_array(self.ctx) &&
                    info.has_too_large_bitfield_unit() &&
                    !item.is_opaque(self.ctx, &())
                {
                    trace!(
                        "    cannot derive {} for comp with too large bitfield unit",
                        self.derive_trait
                    );
                    return CanDerive::No;
                }

                let pred = self.derive_trait.consider_edge_comp();
                self.constrain_join(item, pred)
            }

            TypeKind::ResolvedTypeRef(..) |
            TypeKind::TemplateAlias(..) |
            TypeKind::Alias(..) |
            TypeKind::BlockPointer(..) => {
                let pred = self.derive_trait.consider_edge_typeref();
                self.constrain_join(item, pred)
            }

            TypeKind::TemplateInstantiation(..) => {
                let pred = self.derive_trait.consider_edge_tmpl_inst();
                self.constrain_join(item, pred)
            }

            TypeKind::Opaque => unreachable!(
                "The early ty.is_opaque check should have handled this case"
            ),
        }
    }

    fn constrain_join(
        &mut self,
        item: &Item,
        consider_edge: EdgePredicate,
    ) -> CanDerive {
        let mut candidate = None;

        item.trace(
            self.ctx,
            &mut |sub_id, edge_kind| {
                // Ignore ourselves, since union with ourself is a
                // no-op. Ignore edges that aren't relevant to the
                // analysis.
                if sub_id == item.id() || !consider_edge(edge_kind) {
                    return;
                }

                let can_derive = self.can_derive
                    .get(&sub_id)
                    .copied()
                    .unwrap_or_default();

                match can_derive {
                    CanDerive::Yes => trace!("    member {sub_id:?} can derive {}", self.derive_trait),
                    CanDerive::Manually => trace!("    member {sub_id:?} cannot derive {}, but it may be implemented", self.derive_trait),
                    CanDerive::No => trace!("    member {sub_id:?} cannot derive {}", self.derive_trait),
                }

                *candidate.get_or_insert(CanDerive::Yes) |= can_derive;
            },
            &(),
        );

        if candidate.is_none() {
            trace!(
                "    can derive {} because there are no members",
                self.derive_trait
            );
        }
        candidate.unwrap_or_default()
    }
}

impl DeriveTrait {
    fn not_by_name(&self, ctx: &BindgenContext, item: &Item) -> bool {
        match self {
            DeriveTrait::Copy => ctx.no_copy_by_name(item),
            DeriveTrait::Debug => ctx.no_debug_by_name(item),
            DeriveTrait::Default => ctx.no_default_by_name(item),
            DeriveTrait::Hash => ctx.no_hash_by_name(item),
            DeriveTrait::PartialEqOrPartialOrd => {
                ctx.no_partialeq_by_name(item)
            }
        }
    }

    fn consider_edge_comp(&self) -> EdgePredicate {
        match self {
            DeriveTrait::PartialEqOrPartialOrd => consider_edge_default,
            _ => |kind| matches!(kind, EdgeKind::BaseMember | EdgeKind::Field),
        }
    }

    fn consider_edge_typeref(&self) -> EdgePredicate {
        match self {
            DeriveTrait::PartialEqOrPartialOrd => consider_edge_default,
            _ => |kind| kind == EdgeKind::TypeReference,
        }
    }

    fn consider_edge_tmpl_inst(&self) -> EdgePredicate {
        match self {
            DeriveTrait::PartialEqOrPartialOrd => consider_edge_default,
            _ => |kind| {
                matches!(
                    kind,
                    EdgeKind::TemplateArgument | EdgeKind::TemplateDeclaration
                )
            },
        }
    }

    fn can_derive_large_array(&self, ctx: &BindgenContext) -> bool {
        if ctx.options().rust_features().larger_arrays {
            !matches!(self, DeriveTrait::Default)
        } else {
            matches!(self, DeriveTrait::Copy)
        }
    }

    fn can_derive_union(&self) -> bool {
        matches!(self, DeriveTrait::Copy)
    }

    fn can_derive_compound_with_destructor(&self) -> bool {
        !matches!(self, DeriveTrait::Copy)
    }

    fn can_derive_compound_with_vtable(&self) -> bool {
        !matches!(self, DeriveTrait::Default)
    }

    fn can_derive_compound_forward_decl(&self) -> bool {
        matches!(self, DeriveTrait::Copy | DeriveTrait::Debug)
    }

    fn can_derive_incomplete_array(&self) -> bool {
        !matches!(
            self,
            DeriveTrait::Copy |
                DeriveTrait::Hash |
                DeriveTrait::PartialEqOrPartialOrd
        )
    }

    fn can_derive_fnptr(&self, f: &FunctionSig) -> CanDerive {
        match (self, f.function_pointers_can_derive()) {
            (DeriveTrait::Copy, _) | (DeriveTrait::Default, _) | (_, true) => {
                trace!("    function pointer can derive {self}");
                CanDerive::Yes
            }
            (DeriveTrait::Debug, false) => {
                trace!("    function pointer cannot derive {self}, but it may be implemented");
                CanDerive::Manually
            }
            (_, false) => {
                trace!("    function pointer cannot derive {self}");
                CanDerive::No
            }
        }
    }

    fn can_derive_vector(&self) -> CanDerive {
        if *self == DeriveTrait::PartialEqOrPartialOrd {
            // FIXME: vectors always can derive PartialEq, but they should
            // not derive PartialOrd:
            // https://github.com/rust-lang-nursery/packed_simd/issues/48
            trace!("    vectors cannot derive PartialOrd");
            CanDerive::No
        } else {
            trace!("    vector can derive {self}");
            CanDerive::Yes
        }
    }

    fn can_derive_pointer(&self) -> CanDerive {
        if *self == DeriveTrait::Default {
            trace!("    pointer cannot derive Default");
            CanDerive::No
        } else {
            trace!("    pointer can derive {self}");
            CanDerive::Yes
        }
    }

    fn can_derive_simple(&self, kind: &TypeKind) -> CanDerive {
        match (self, kind) {
            // === Default ===
            (DeriveTrait::Default, TypeKind::Void) |
            (DeriveTrait::Default, TypeKind::NullPtr) |
            (DeriveTrait::Default, TypeKind::Enum(..)) |
            (DeriveTrait::Default, TypeKind::Reference(..)) |
            (DeriveTrait::Default, TypeKind::TypeParam) |
            (DeriveTrait::Default, TypeKind::ObjCInterface(..)) |
            (DeriveTrait::Default, TypeKind::ObjCId) |
            (DeriveTrait::Default, TypeKind::ObjCSel) => {
                trace!("    types that always cannot derive Default");
                CanDerive::No
            }
            (DeriveTrait::Default, TypeKind::UnresolvedTypeRef(..)) => {
                unreachable!(
                    "Type with unresolved type ref can't reach derive default"
                )
            }
            // === Hash ===
            (DeriveTrait::Hash, TypeKind::Float(..)) |
            (DeriveTrait::Hash, TypeKind::Complex(..)) => {
                trace!("    float cannot derive Hash");
                CanDerive::No
            }
            // === others ===
            _ => {
                trace!("    simple type that can always derive {self}");
                CanDerive::Yes
            }
        }
    }
}

impl fmt::Display for DeriveTrait {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            DeriveTrait::Copy => "Copy",
            DeriveTrait::Debug => "Debug",
            DeriveTrait::Default => "Default",
            DeriveTrait::Hash => "Hash",
            DeriveTrait::PartialEqOrPartialOrd => "PartialEq/PartialOrd",
        };
        s.fmt(f)
    }
}

impl<'ctx> MonotoneFramework for CannotDerive<'ctx> {
    type Node = ItemId;
    type Extra = (&'ctx BindgenContext, DeriveTrait);
    type Output = HashMap<ItemId, CanDerive>;

    fn new(
        (ctx, derive_trait): (&'ctx BindgenContext, DeriveTrait),
    ) -> CannotDerive<'ctx> {
        let can_derive = HashMap::default();
        let dependencies = generate_dependencies(ctx, consider_edge_default);

        CannotDerive {
            ctx,
            derive_trait,
            can_derive,
            dependencies,
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
        trace!("constrain: {id:?}");

        if let Some(CanDerive::No) = self.can_derive.get(&id) {
            trace!("    already know it cannot derive {}", self.derive_trait);
            return ConstrainResult::Same;
        }

        let item = self.ctx.resolve_item(id);
        let can_derive = match item.as_type() {
            Some(ty) => {
                let mut can_derive = self.constrain_type(item, ty);
                if let CanDerive::Yes = can_derive {
                    let is_reached_limit =
                        |l: Layout| l.align > RUST_DERIVE_IN_ARRAY_LIMIT;
                    if !self.derive_trait.can_derive_large_array(self.ctx) &&
                        ty.layout(self.ctx).is_some_and(is_reached_limit)
                    {
                        // We have to be conservative: the struct *could* have enough
                        // padding that we emit an array that is longer than
                        // `RUST_DERIVE_IN_ARRAY_LIMIT`. If we moved padding calculations
                        // into the IR and computed them before this analysis, then we could
                        // be precise rather than conservative here.
                        can_derive = CanDerive::Manually;
                    }
                }
                can_derive
            }
            None => self.constrain_join(item, consider_edge_default),
        };

        self.insert(id, can_derive)
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

impl<'ctx> From<CannotDerive<'ctx>> for HashMap<ItemId, CanDerive> {
    fn from(analysis: CannotDerive<'ctx>) -> Self {
        extra_assert!(analysis
            .can_derive
            .values()
            .all(|v| *v != CanDerive::Yes));

        analysis.can_derive
    }
}

/// Convert a `HashMap<ItemId, CanDerive>` into a `HashSet<ItemId>`.
///
/// Elements that are not `CanDerive::Yes` are kept in the set, so that it
/// represents all items that cannot derive.
pub(crate) fn as_cannot_derive_set(
    can_derive: HashMap<ItemId, CanDerive>,
) -> HashSet<ItemId> {
    can_derive
        .into_iter()
        .filter_map(|(k, v)| if v == CanDerive::Yes { None } else { Some(k) })
        .collect()
}
