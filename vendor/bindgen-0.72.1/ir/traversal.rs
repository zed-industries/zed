//! Traversal of the graph of IR items and types.

use super::context::{BindgenContext, ItemId};
use super::item::ItemSet;
use std::collections::{BTreeMap, VecDeque};

/// An outgoing edge in the IR graph is a reference from some item to another
/// item:
///
///   from --> to
///
/// The `from` is left implicit: it is the concrete `Trace` implementer which
/// yielded this outgoing edge.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Edge {
    to: ItemId,
    kind: EdgeKind,
}

impl Edge {
    /// Construct a new edge whose referent is `to` and is of the given `kind`.
    pub(crate) fn new(to: ItemId, kind: EdgeKind) -> Edge {
        Edge { to, kind }
    }
}

impl From<Edge> for ItemId {
    fn from(val: Edge) -> Self {
        val.to
    }
}

/// The kind of edge reference. This is useful when we wish to only consider
/// certain kinds of edges for a particular traversal or analysis.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum EdgeKind {
    /// A generic, catch-all edge.
    Generic,

    /// An edge from a template declaration, to the definition of a named type
    /// parameter. For example, the edge from `Foo<T>` to `T` in the following
    /// snippet:
    ///
    /// ```C++
    /// template<typename T>
    /// class Foo { };
    /// ```
    TemplateParameterDefinition,

    /// An edge from a template instantiation to the template declaration that
    /// is being instantiated. For example, the edge from `Foo<int>` to
    /// to `Foo<T>`:
    ///
    /// ```C++
    /// template<typename T>
    /// class Foo { };
    ///
    /// using Bar = Foo<ant>;
    /// ```
    TemplateDeclaration,

    /// An edge from a template instantiation to its template argument. For
    /// example, `Foo<Bar>` to `Bar`:
    ///
    /// ```C++
    /// template<typename T>
    /// class Foo { };
    ///
    /// class Bar { };
    ///
    /// using FooBar = Foo<Bar>;
    /// ```
    TemplateArgument,

    /// An edge from a compound type to one of its base member types. For
    /// example, the edge from `Bar` to `Foo`:
    ///
    /// ```C++
    /// class Foo { };
    ///
    /// class Bar : public Foo { };
    /// ```
    BaseMember,

    /// An edge from a compound type to the types of one of its fields. For
    /// example, the edge from `Foo` to `int`:
    ///
    /// ```C++
    /// class Foo {
    ///     int x;
    /// };
    /// ```
    Field,

    /// An edge from an class or struct type to an inner type member. For
    /// example, the edge from `Foo` to `Foo::Bar` here:
    ///
    /// ```C++
    /// class Foo {
    ///     struct Bar { };
    /// };
    /// ```
    InnerType,

    /// An edge from an class or struct type to an inner static variable. For
    /// example, the edge from `Foo` to `Foo::BAR` here:
    ///
    /// ```C++
    /// class Foo {
    ///     static const char* BAR;
    /// };
    /// ```
    InnerVar,

    /// An edge from a class or struct type to one of its method functions. For
    /// example, the edge from `Foo` to `Foo::bar`:
    ///
    /// ```C++
    /// class Foo {
    ///     bool bar(int x, int y);
    /// };
    /// ```
    Method,

    /// An edge from a class or struct type to one of its constructor
    /// functions. For example, the edge from `Foo` to `Foo::Foo(int x, int y)`:
    ///
    /// ```C++
    /// class Foo {
    ///     int my_x;
    ///     int my_y;
    ///
    ///   public:
    ///     Foo(int x, int y);
    /// };
    /// ```
    Constructor,

    /// An edge from a class or struct type to its destructor function. For
    /// example, the edge from `Doggo` to `Doggo::~Doggo()`:
    ///
    /// ```C++
    /// struct Doggo {
    ///     char* wow;
    ///
    ///   public:
    ///     ~Doggo();
    /// };
    /// ```
    Destructor,

    /// An edge from a function declaration to its return type. For example, the
    /// edge from `foo` to `int`:
    ///
    /// ```C++
    /// int foo(char* string);
    /// ```
    FunctionReturn,

    /// An edge from a function declaration to one of its parameter types. For
    /// example, the edge from `foo` to `char*`:
    ///
    /// ```C++
    /// int foo(char* string);
    /// ```
    FunctionParameter,

    /// An edge from a static variable to its type. For example, the edge from
    /// `FOO` to `const char*`:
    ///
    /// ```C++
    /// static const char* FOO;
    /// ```
    VarType,

    /// An edge from a non-templated alias or typedef to the referenced type.
    TypeReference,
}

/// A predicate to allow visiting only sub-sets of the whole IR graph by
/// excluding certain edges from being followed by the traversal.
///
/// The predicate must return true if the traversal should follow this edge
/// and visit everything that is reachable through it.
pub(crate) type TraversalPredicate =
    for<'a> fn(&'a BindgenContext, Edge) -> bool;

/// A `TraversalPredicate` implementation that follows all edges, and therefore
/// traversals using this predicate will see the whole IR graph reachable from
/// the traversal's roots.
pub(crate) fn all_edges(_: &BindgenContext, _: Edge) -> bool {
    true
}

/// A `TraversalPredicate` implementation that only follows
/// `EdgeKind::InnerType` edges, and therefore traversals using this predicate
/// will only visit the traversal's roots and their inner types. This is used
/// in no-recursive-allowlist mode, where inner types such as anonymous
/// structs/unions still need to be processed.
pub(crate) fn only_inner_type_edges(_: &BindgenContext, edge: Edge) -> bool {
    edge.kind == EdgeKind::InnerType
}

/// A `TraversalPredicate` implementation that only follows edges to items that
/// are enabled for code generation. This lets us skip considering items for
/// which are not reachable from code generation.
pub(crate) fn codegen_edges(ctx: &BindgenContext, edge: Edge) -> bool {
    let cc = &ctx.options().codegen_config;
    match edge.kind {
        EdgeKind::Generic => {
            ctx.resolve_item(edge.to).is_enabled_for_codegen(ctx)
        }

        // We statically know the kind of item that non-generic edges can point
        // to, so we don't need to actually resolve the item and check
        // `Item::is_enabled_for_codegen`.
        EdgeKind::TemplateParameterDefinition |
        EdgeKind::TemplateArgument |
        EdgeKind::TemplateDeclaration |
        EdgeKind::BaseMember |
        EdgeKind::Field |
        EdgeKind::InnerType |
        EdgeKind::FunctionReturn |
        EdgeKind::FunctionParameter |
        EdgeKind::VarType |
        EdgeKind::TypeReference => cc.types(),
        EdgeKind::InnerVar => cc.vars(),
        EdgeKind::Method => cc.methods(),
        EdgeKind::Constructor => cc.constructors(),
        EdgeKind::Destructor => cc.destructors(),
    }
}

/// The storage for the set of items that have been seen (although their
/// outgoing edges might not have been fully traversed yet) in an active
/// traversal.
pub(crate) trait TraversalStorage<'ctx> {
    /// Construct a new instance of this `TraversalStorage`, for a new traversal.
    fn new(ctx: &'ctx BindgenContext) -> Self;

    /// Add the given item to the storage. If the item has never been seen
    /// before, return `true`. Otherwise, return `false`.
    ///
    /// The `from` item is the item from which we discovered this item, or is
    /// `None` if this item is a root.
    fn add(&mut self, from: Option<ItemId>, item: ItemId) -> bool;
}

impl<'ctx> TraversalStorage<'ctx> for ItemSet {
    fn new(_: &'ctx BindgenContext) -> Self {
        ItemSet::new()
    }

    fn add(&mut self, _: Option<ItemId>, item: ItemId) -> bool {
        self.insert(item)
    }
}

/// A `TraversalStorage` implementation that keeps track of how we first reached
/// each item. This is useful for providing debug assertions with meaningful
/// diagnostic messages about dangling items.
#[derive(Debug)]
pub(crate) struct Paths<'ctx>(BTreeMap<ItemId, ItemId>, &'ctx BindgenContext);

impl<'ctx> TraversalStorage<'ctx> for Paths<'ctx> {
    fn new(ctx: &'ctx BindgenContext) -> Self {
        Paths(BTreeMap::new(), ctx)
    }

    fn add(&mut self, from: Option<ItemId>, item: ItemId) -> bool {
        let newly_discovered =
            self.0.insert(item, from.unwrap_or(item)).is_none();

        if self.1.resolve_item_fallible(item).is_none() {
            let mut path = vec![];
            let mut current = item;
            loop {
                let predecessor = *self.0.get(&current).expect(
                    "We know we found this item id, so it must have a \
                     predecessor",
                );
                if predecessor == current {
                    break;
                }
                path.push(predecessor);
                current = predecessor;
            }
            path.reverse();
            panic!(
                "Found reference to dangling id = {item:?}\nvia path = {path:?}"
            );
        }

        newly_discovered
    }
}

/// The queue of seen-but-not-yet-traversed items.
///
/// Using a FIFO queue with a traversal will yield a breadth-first traversal,
/// while using a LIFO queue will result in a depth-first traversal of the IR
/// graph.
pub(crate) trait TraversalQueue: Default {
    /// Add a newly discovered item to the queue.
    fn push(&mut self, item: ItemId);

    /// Pop the next item to traverse, if any.
    fn next(&mut self) -> Option<ItemId>;
}

impl TraversalQueue for Vec<ItemId> {
    fn push(&mut self, item: ItemId) {
        self.push(item);
    }

    fn next(&mut self) -> Option<ItemId> {
        self.pop()
    }
}

impl TraversalQueue for VecDeque<ItemId> {
    fn push(&mut self, item: ItemId) {
        self.push_back(item);
    }

    fn next(&mut self) -> Option<ItemId> {
        self.pop_front()
    }
}

/// Something that can receive edges from a `Trace` implementation.
pub(crate) trait Tracer {
    /// Note an edge between items. Called from within a `Trace` implementation.
    fn visit_kind(&mut self, item: ItemId, kind: EdgeKind);

    /// A synonym for `tracer.visit_kind(item, EdgeKind::Generic)`.
    fn visit(&mut self, item: ItemId) {
        self.visit_kind(item, EdgeKind::Generic);
    }
}

impl<F> Tracer for F
where
    F: FnMut(ItemId, EdgeKind),
{
    fn visit_kind(&mut self, item: ItemId, kind: EdgeKind) {
        (*self)(item, kind);
    }
}

/// Trace all of the outgoing edges to other items. Implementations should call
/// one of `tracer.visit(edge)` or `tracer.visit_kind(edge, EdgeKind::Whatever)`
/// for each of their outgoing edges.
pub(crate) trait Trace {
    /// If a particular type needs extra information beyond what it has in
    /// `self` and `context` to find its referenced items, its implementation
    /// can define this associated type, forcing callers to pass the needed
    /// information through.
    type Extra;

    /// Trace all of this item's outgoing edges to other items.
    fn trace<T>(
        &self,
        context: &BindgenContext,
        tracer: &mut T,
        extra: &Self::Extra,
    ) where
        T: Tracer;
}

/// An graph traversal of the transitive closure of references between items.
///
/// See `BindgenContext::allowlisted_items` for more information.
pub(crate) struct ItemTraversal<'ctx, Storage, Queue>
where
    Storage: TraversalStorage<'ctx>,
    Queue: TraversalQueue,
{
    ctx: &'ctx BindgenContext,

    /// The set of items we have seen thus far in this traversal.
    seen: Storage,

    /// The set of items that we have seen, but have yet to traverse.
    queue: Queue,

    /// The predicate that determines which edges this traversal will follow.
    predicate: TraversalPredicate,

    /// The item we are currently traversing.
    currently_traversing: Option<ItemId>,
}

impl<'ctx, Storage, Queue> ItemTraversal<'ctx, Storage, Queue>
where
    Storage: TraversalStorage<'ctx>,
    Queue: TraversalQueue,
{
    /// Begin a new traversal, starting from the given roots.
    pub(crate) fn new<R>(
        ctx: &'ctx BindgenContext,
        roots: R,
        predicate: TraversalPredicate,
    ) -> ItemTraversal<'ctx, Storage, Queue>
    where
        R: IntoIterator<Item = ItemId>,
    {
        let mut seen = Storage::new(ctx);
        let mut queue = Queue::default();

        for id in roots {
            seen.add(None, id);
            queue.push(id);
        }

        ItemTraversal {
            ctx,
            seen,
            queue,
            predicate,
            currently_traversing: None,
        }
    }
}

impl<'ctx, Storage, Queue> Tracer for ItemTraversal<'ctx, Storage, Queue>
where
    Storage: TraversalStorage<'ctx>,
    Queue: TraversalQueue,
{
    fn visit_kind(&mut self, item: ItemId, kind: EdgeKind) {
        let edge = Edge::new(item, kind);
        if !(self.predicate)(self.ctx, edge) {
            return;
        }

        let is_newly_discovered =
            self.seen.add(self.currently_traversing, item);
        if is_newly_discovered {
            self.queue.push(item);
        }
    }
}

impl<'ctx, Storage, Queue> Iterator for ItemTraversal<'ctx, Storage, Queue>
where
    Storage: TraversalStorage<'ctx>,
    Queue: TraversalQueue,
{
    type Item = ItemId;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.queue.next()?;

        let newly_discovered = self.seen.add(None, id);
        debug_assert!(
            !newly_discovered,
            "should have already seen anything we get out of our queue"
        );
        debug_assert!(
            self.ctx.resolve_item_fallible(id).is_some(),
            "should only get IDs of actual items in our context during traversal"
        );

        self.currently_traversing = Some(id);
        id.trace(self.ctx, self, &());
        self.currently_traversing = None;

        Some(id)
    }
}

/// An iterator to find any dangling items.
///
/// See `BindgenContext::assert_no_dangling_item_traversal` for more
/// information.
pub(crate) type AssertNoDanglingItemsTraversal<'ctx> =
    ItemTraversal<'ctx, Paths<'ctx>, VecDeque<ItemId>>;
