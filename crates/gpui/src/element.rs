//! Elements are the workhorses of GPUI. They are responsible for laying out and painting all of
//! the contents of a window. Elements form a tree and are laid out according to the web layout
//! standards as implemented by [taffy](https://github.com/DioxusLabs/taffy). Most of the time,
//! you won't need to interact with this module or these APIs directly. Elements provide their
//! own APIs and GPUI, or other element implementation, uses the APIs in this module to convert
//! that element tree into the pixels you see on the screen.
//!
//! # Element Basics
//!
//! Elements are constructed by calling [`Render::render()`] on the root view of the window,
//! which recursively constructs the element tree from the current state of the application,.
//! These elements are then laid out by Taffy, and painted to the screen according to their own
//! implementation of [`Element::paint()`]. Before the start of the next frame, the entire element
//! tree and any callbacks they have registered with GPUI are dropped and the process repeats.
//!
//! But some state is too simple and voluminous to store in every view that needs it, e.g.
//! whether a hover has been started or not. For this, GPUI provides the [`Element::PrepaintState`], associated type.
//!
//! # Implementing your own elements
//!
//! Elements are intended to be the low level, imperative API to GPUI. They are responsible for upholding,
//! or breaking, GPUI's features as they deem necessary. As an example, most GPUI elements are expected
//! to stay in the bounds that their parent element gives them. But with [`Window::with_content_mask`],
//! you can ignore this restriction and paint anywhere inside of the window's bounds. This is useful for overlays
//! and popups and anything else that shows up 'on top' of other elements.
//! With great power, comes great responsibility.
//!
//! However, most of the time, you won't need to implement your own elements. GPUI provides a number of
//! elements that should cover most common use cases out of the box and it's recommended that you use those
//! to construct `components`, using the [`RenderOnce`] trait and the `#[derive(IntoElement)]` macro. Only implement
//! elements when you need to take manual control of the layout and painting process, such as when using
//! your own custom layout algorithm or rendering a code editor.

#[cfg(any(feature = "inspector", debug_assertions))]
use crate::InspectorElementPath;
use crate::{
    A11ySubtreeBuilder, App, ArenaBox, AvailableSpace, Bounds, Context, DispatchNodeId, ElementId,
    FocusHandle, InspectorElementId, LayoutId, Pixels, Point, Size, Style, Window,
    util::FluentBuilder, window::with_element_arena,
};
use collections::FxHashMap;
use parking_lot::{Mutex, RwLock};
use smallvec::SmallVec;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    fmt::{self, Debug, Display},
    mem, panic,
    rc::Rc,
    sync::{
        Arc, LazyLock, OnceLock,
        atomic::{self, AtomicU64},
    },
};

/// Implemented by types that participate in laying out and painting the contents of a window.
/// Elements form a tree and are laid out according to web-based layout rules, as implemented by Taffy.
/// You can create custom elements by implementing this trait, see the module-level documentation
/// for more details.
pub trait Element: 'static + IntoElement {
    /// The type of state returned from [`Element::request_layout`]. A mutable reference to this state is subsequently
    /// provided to [`Element::prepaint`] and [`Element::paint`].
    type RequestLayoutState: 'static;

    /// The type of state returned from [`Element::prepaint`]. A mutable reference to this state is subsequently
    /// provided to [`Element::paint`].
    type PrepaintState: 'static;

    /// If this element has a unique identifier, return it here. This is used to track elements across frames, and
    /// will cause a GlobalElementId to be passed to the request_layout, prepaint, and paint methods.
    ///
    /// The global id can in turn be used to access state that's connected to an element with the same id across
    /// frames. This id must be unique among children of the first containing element with an id.
    fn id(&self) -> Option<ElementId>;

    /// Source location where this element was constructed, used to disambiguate elements in the
    /// inspector and navigate to their source code.
    fn source_location(&self) -> Option<&'static panic::Location<'static>>;

    /// Before an element can be painted, we need to know where it's going to be and how big it is.
    /// Use this method to request a layout from Taffy and initialize the element's state.
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState);

    /// After laying out an element, we need to commit its bounds to the current frame for hitbox
    /// purposes. The state argument is the same state that was returned from [`Element::request_layout()`].
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState;

    /// Once layout has been completed, this method will be called to paint the element to the screen.
    /// The state argument is the same state that was returned from [`Element::request_layout()`].
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    );

    /// Returns the accessible role for this element, if any.
    /// Elements that return `None` are not included in the accessibility tree.
    ///
    /// Note: inclusion in accessibility tree requires non-`None` [`id`][Element::id].
    ///
    /// See the [accessibility guide](crate::_accessibility) for an overview.
    fn a11y_role(&self) -> Option<accesskit::Role> {
        None
    }

    /// Write accessibility properties to the given node.
    /// Called only when `a11y_role()` returns `Some`.
    ///
    /// See the [accessibility guide](crate::_accessibility) for an overview.
    fn write_a11y_info(&self, _node: &mut accesskit::Node) {}

    /// Add synthetic child nodes to an [`Element`] that has an
    /// [`.id()`][Element::id] and a [`.role()`][Element::a11y_role].
    ///
    /// Some elements may want to inject accessibility nodes that do not
    /// correspond to any GPUI element. For example, a custom text field element
    /// may want to inject synthetic child nodes for the text content.
    ///
    /// See [Synthetic children](crate::_accessibility#synthetic-children) in
    /// the accessibility guide for more detail.
    fn a11y_synthetic_children(
        &mut self,
        _prepaint: &mut Self::PrepaintState,
        _builder: &mut A11ySubtreeBuilder,
    ) {
    }

    /// Convert this element into a dynamically-typed [`AnyElement`].
    fn into_any(self) -> AnyElement {
        AnyElement::new(self)
    }
}

/// Implemented by any type that can be converted into an element.
pub trait IntoElement: Sized {
    /// The specific type of element into which the implementing type is converted.
    /// Useful for converting other types into elements automatically, like Strings
    type Element: Element;

    /// Convert self into a type that implements [`Element`].
    fn into_element(self) -> Self::Element;

    /// Convert self into a dynamically-typed [`AnyElement`].
    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }
}

impl<T: IntoElement> FluentBuilder for T {}

/// An object that can be drawn to the screen. This is the trait that distinguishes "views" from
/// other entities. Views are `Entity`'s which `impl Render` and drawn to the screen.
pub trait Render: 'static + Sized {
    /// Render this view into an element tree.
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement;
}

impl Render for Empty {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

/// You can derive [`IntoElement`] on any type that implements this trait.
/// It is used to construct reusable `components` out of plain data. Think of
/// components as a recipe for a certain pattern of elements. RenderOnce allows
/// you to invoke this pattern, without breaking the fluent builder pattern of
/// the element APIs.
pub trait RenderOnce: 'static {
    /// Render this component into an element tree. Note that this method
    /// takes ownership of self, as compared to [`Render::render()`] method
    /// which takes a mutable reference.
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement;
}

/// This is a helper trait to provide a uniform interface for constructing elements that
/// can accept any number of any kind of child elements
pub trait ParentElement {
    /// Extend this element's children with the given child elements.
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>);

    /// Add a single child element to this element.
    fn child(mut self, child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.extend(std::iter::once(child.into_any_element()));
        self
    }

    /// Add multiple child elements to this element.
    fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self
    where
        Self: Sized,
    {
        self.extend(children.into_iter().map(|child| child.into_any_element()));
        self
    }
}

/// A globally unique identifier for an element, used to track state across frames.
///
/// A window builds an ID from the number it interned for each [`ElementId`] on
/// the path, so two IDs from the same window compare as integer slices. Equality
/// and hashing are still by path, including across windows: the path is looked
/// up from the window only when it is read, printed, cloned, or compared with an
/// ID from elsewhere, and a clone owns it so that it stays readable for as long
/// as it is held.
pub struct GlobalElementId {
    hash: u64,
    /// The window interner that numbered `components`, or 0 for an ID built from
    /// a path outside a window, whose `components` are empty.
    interner_id: u64,
    components: ComponentPath,
    // A thin `Arc` keeps this ID, and every element's draw phase that holds
    // one, small.
    path: OnceLock<Arc<Vec<ElementId>>>,
}

/// Deeper paths spill to the heap. Every element's draw phase holds an
/// `Option<GlobalElementId>`, so a larger inline capacity slows down drawing
/// elements without IDs too.
const INLINE_COMPONENTS: usize = 8;

type ComponentPath = SmallVec<[u64; INLINE_COMPONENTS]>;

/// The hash of an empty id path; every path hash is folded from it with
/// [`extend_path_hash`], one id at a time, so the window can keep the hash
/// of each prefix of its id stack and never re-hash a path.
pub(crate) const EMPTY_PATH_HASH: u64 = 0;

/// The hash of the path `prefix` + an id whose own hash is `id_hash`.
pub(crate) fn extend_path_hash(prefix: u64, id_hash: u64) -> u64 {
    use std::hash::Hasher;
    let mut hasher = collections::FxHasher::with_seed(prefix as usize);
    hasher.write_u64(id_hash);
    hasher.finish()
}

fn element_id_hash(id: &ElementId) -> u64 {
    use std::hash::BuildHasher;
    collections::FxBuildHasher.hash_one(id)
}

impl GlobalElementId {
    pub(crate) fn state_key_ref<'a>(
        &self,
        components: &'a [u64],
        type_id: TypeId,
    ) -> ElementStateKeyRef<'a> {
        ElementStateKeyRef {
            hash: self.hash,
            type_id,
            components,
        }
    }

    pub(crate) fn new(ids: &[ElementId]) -> Self {
        Self {
            hash: ids.iter().fold(EMPTY_PATH_HASH, |hash, id| {
                extend_path_hash(hash, element_id_hash(id))
            }),
            interner_id: 0,
            components: ComponentPath::new(),
            path: OnceLock::from(Arc::new(ids.to_vec())),
        }
    }

    pub(crate) fn accesskit_node_id(&self) -> accesskit::NodeId {
        accesskit::NodeId(self.hash)
    }

    /// Looks the path up while the interner still holds every component, which
    /// is the whole frame the ID was built in.
    fn materialize(&self) -> Option<&Arc<Vec<ElementId>>> {
        if let Some(path) = self.path.get() {
            return Some(path);
        }
        let names = COMPONENT_NAMES.read().get(&self.interner_id)?.clone();
        let names = names.lock();
        let path = self
            .components
            .iter()
            .map(|&number| match names.get(slot_index(number)) {
                Some(Some((live_number, id))) if *live_number == number => Some(id.clone()),
                _ => None,
            })
            .collect::<Option<Vec<ElementId>>>()?;
        Some(self.path.get_or_init(|| Arc::new(path)))
    }

    /// The same ID without looking up its path, so the copy can only read its
    /// path while this ID's numbers are live.
    fn clone_unmaterialized(&self) -> Self {
        Self {
            hash: self.hash,
            interner_id: self.interner_id,
            components: self.components.clone(),
            path: self.path.clone(),
        }
    }
}

impl Default for GlobalElementId {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl Clone for GlobalElementId {
    fn clone(&self) -> Self {
        // A clone can outlive the frame, after which the interner may have
        // evicted its components; owning the path keeps it readable.
        self.materialize();
        self.clone_unmaterialized()
    }
}

impl std::ops::Deref for GlobalElementId {
    type Target = [ElementId];

    fn deref(&self) -> &Self::Target {
        self.materialize().map_or(&[], |path| path)
    }
}

impl PartialEq for GlobalElementId {
    fn eq(&self, other: &Self) -> bool {
        if self.hash != other.hash {
            return false;
        }
        // Numbers are never reused, so equal numbers from one interner are
        // equal paths. Otherwise the IDs come from different windows, or one
        // was held across frames, or their path hashes collide.
        if self.interner_id != 0
            && self.interner_id == other.interner_id
            && self.components == other.components
        {
            return true;
        }
        match (self.materialize(), other.materialize()) {
            (Some(this), Some(other)) => Arc::ptr_eq(this, other) || this == other,
            _ => false,
        }
    }
}

impl Eq for GlobalElementId {}

impl std::hash::Hash for GlobalElementId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl Debug for GlobalElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GlobalElementId")
            .field("ids", &&**self)
            .field("hash", &self.hash)
            .finish()
    }
}

impl Display for GlobalElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, element_id) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ".")?;
            }
            write!(f, "{}", element_id)?;
        }
        Ok(())
    }
}

/// A window's key for element state: the state's type and the numbers of the
/// element's path in the window's own interner. The numbers are shared, so a
/// key is cheap to move and clone between frames; lookups borrow them from the
/// [`GlobalElementId`] as an [`ElementStateKeyRef`] instead of allocating.
#[derive(Clone)]
pub(crate) struct ElementStateKey {
    hash: u64,
    type_id: TypeId,
    components: Rc<[u64]>,
}

#[derive(Clone, Copy)]
pub(crate) struct ElementStateKeyRef<'a> {
    hash: u64,
    type_id: TypeId,
    components: &'a [u64],
}

impl ElementStateKeyRef<'_> {
    pub(crate) fn to_key(self) -> ElementStateKey {
        ElementStateKey {
            hash: self.hash,
            type_id: self.type_id,
            components: Rc::from(self.components),
        }
    }
}

pub(crate) trait AsElementStateKeyRef {
    fn key_ref(&self) -> ElementStateKeyRef<'_>;
}

impl AsElementStateKeyRef for ElementStateKey {
    fn key_ref(&self) -> ElementStateKeyRef<'_> {
        ElementStateKeyRef {
            hash: self.hash,
            type_id: self.type_id,
            components: &self.components,
        }
    }
}

impl AsElementStateKeyRef for ElementStateKeyRef<'_> {
    fn key_ref(&self) -> ElementStateKeyRef<'_> {
        *self
    }
}

impl<'a> std::borrow::Borrow<dyn AsElementStateKeyRef + 'a> for ElementStateKey {
    fn borrow(&self) -> &(dyn AsElementStateKeyRef + 'a) {
        self
    }
}

impl std::hash::Hash for dyn AsElementStateKeyRef + '_ {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let key = self.key_ref();
        state.write_u64(key.hash);
        key.type_id.hash(state);
    }
}

impl PartialEq for dyn AsElementStateKeyRef + '_ {
    fn eq(&self, other: &Self) -> bool {
        let (this, other) = (self.key_ref(), other.key_ref());
        this.hash == other.hash
            && this.type_id == other.type_id
            && this.components == other.components
    }
}

impl Eq for dyn AsElementStateKeyRef + '_ {}

impl std::hash::Hash for ElementStateKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self as &dyn AsElementStateKeyRef).hash(state);
    }
}

impl PartialEq for ElementStateKey {
    fn eq(&self, other: &Self) -> bool {
        (self as &dyn AsElementStateKeyRef) == (other as &dyn AsElementStateKeyRef)
    }
}

impl Eq for ElementStateKey {}

/// The names behind each interner's numbers, for IDs that need their path
/// without access to the window.
static COMPONENT_NAMES: LazyLock<RwLock<FxHashMap<u64, Arc<Mutex<ComponentNames>>>>> =
    LazyLock::new(Default::default);

/// Indexed by slot; each entry keeps the full number so a recycled slot does
/// not answer for an evicted one.
type ComponentNames = Vec<Option<(u64, ElementId)>>;

fn component_number(index: u32, generation: u32) -> u64 {
    (u64::from(generation) << 32) | u64::from(index)
}

fn slot_index(number: u64) -> usize {
    (number & u64::from(u32::MAX)) as usize
}

fn slot_generation(number: u64) -> u32 {
    (number >> 32) as u32
}

struct ComponentSlot {
    generation: u32,
    live: bool,
    last_used: u64,
    id_hash: u64,
}

/// Numbers each [`ElementId`] a window draws. A number is a slot index and
/// that slot's generation, which is bumped whenever the slot is recycled, so a
/// number is never handed out twice and a stale one can't match a new ID.
pub(crate) struct ElementIdInterner {
    interner_id: u64,
    epoch: u64,
    numbers: FxHashMap<ElementId, u64>,
    slots: Vec<ComponentSlot>,
    free: Vec<u32>,
    names: Arc<Mutex<ComponentNames>>,
}

impl Default for ElementIdInterner {
    fn default() -> Self {
        static NEXT_INTERNER_ID: AtomicU64 = AtomicU64::new(1);
        let interner_id = NEXT_INTERNER_ID.fetch_add(1, atomic::Ordering::Relaxed);
        let names = Arc::new(Mutex::new(ComponentNames::new()));
        COMPONENT_NAMES.write().insert(interner_id, names.clone());
        Self {
            interner_id,
            epoch: 0,
            numbers: FxHashMap::default(),
            slots: Vec::new(),
            free: Vec::new(),
            names,
        }
    }
}

impl Drop for ElementIdInterner {
    fn drop(&mut self) {
        COMPONENT_NAMES.write().remove(&self.interner_id);
    }
}

impl ElementIdInterner {
    /// Returns the number for `id` and the hash of `id`.
    pub(crate) fn intern(&mut self, id: ElementId) -> (u64, u64) {
        if let Some(&number) = self.numbers.get(&id) {
            let slot = &mut self.slots[slot_index(number)];
            slot.last_used = self.epoch;
            return (number, slot.id_hash);
        }

        let id_hash = element_id_hash(&id);
        let mut names = self.names.lock();
        let index = match self.free.pop() {
            Some(index) => index,
            None => {
                let index = u32::try_from(self.slots.len()).expect("element id interner is full");
                self.slots.push(ComponentSlot {
                    generation: 0,
                    live: false,
                    last_used: 0,
                    id_hash: 0,
                });
                names.push(None);
                index
            }
        };
        let slot = &mut self.slots[index as usize];
        slot.live = true;
        slot.last_used = self.epoch;
        slot.id_hash = id_hash;
        let number = component_number(index, slot.generation);
        names[index as usize] = Some((number, id.clone()));
        drop(names);
        self.numbers.insert(id, number);
        (number, id_hash)
    }

    pub(crate) fn global_id(&self, components: &[u64], hash: u64) -> GlobalElementId {
        GlobalElementId {
            hash,
            interner_id: self.interner_id,
            components: ComponentPath::from_slice(components),
            path: OnceLock::new(),
        }
    }

    fn is_live(&self, number: u64) -> bool {
        self.slots
            .get(slot_index(number))
            .is_some_and(|slot| slot.live && slot.generation == slot_generation(number))
    }

    /// The numbers to key `id`'s state by in this window.
    pub(crate) fn state_components<'a>(&mut self, id: &'a GlobalElementId) -> Cow<'a, [u64]> {
        if id.interner_id == self.interner_id {
            // An ID whose path was never looked up was built this frame, when
            // all of its numbers are live; a clone may have been held longer.
            if id.path.get().is_none() {
                return Cow::Borrowed(&id.components);
            }
            if id.components.iter().all(|&number| self.is_live(number)) {
                self.mark_used(&id.components);
                return Cow::Borrowed(&id.components);
            }
        }
        Cow::Owned(
            id.iter()
                .map(|element_id| self.intern(element_id.clone()).0)
                .collect(),
        )
    }

    /// Keeps the numbers of a key whose state is reused without redrawing its
    /// element, as when a cached view is replayed.
    pub(crate) fn mark_state_used(&mut self, key: &ElementStateKey) {
        self.mark_used(&key.components);
    }

    fn mark_used(&mut self, components: &[u64]) {
        for &number in components {
            if let Some(slot) = self.slots.get_mut(slot_index(number))
                && slot.live
                && slot.generation == slot_generation(number)
            {
                slot.last_used = self.epoch;
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn key_path(&self, key: &ElementStateKey) -> GlobalElementId {
        let id = self.global_id(&key.components, key.hash);
        id.materialize();
        id
    }

    pub(crate) fn finish_frame(&mut self) {
        // Two frame epochs cover both frame-state buffers, including IDs used
        // only during request_layout.
        let mut names = None;
        for (index, slot) in self.slots.iter_mut().enumerate() {
            if !slot.live || self.epoch.saturating_sub(slot.last_used) <= 1 {
                continue;
            }
            let names = names.get_or_insert_with(|| self.names.lock());
            if let Some((_, id)) = names.get_mut(index).and_then(Option::take) {
                self.numbers.remove(&id);
            }
            slot.live = false;
            // Never wrap a generation: an exhausted slot is retired instead.
            if let Some(generation) = slot.generation.checked_add(1) {
                slot.generation = generation;
                self.free.push(index as u32);
            }
        }
        self.epoch += 1;
    }
}

trait ElementObject {
    fn inner_element(&mut self) -> &mut dyn Any;

    fn request_layout(&mut self, window: &mut Window, cx: &mut App) -> LayoutId;

    fn prepaint(&mut self, window: &mut Window, cx: &mut App);

    fn paint(&mut self, window: &mut Window, cx: &mut App);

    fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels>;
}

/// A wrapper around an implementer of [`Element`] that allows it to be drawn in a window.
pub struct Drawable<E: Element> {
    /// The drawn element.
    pub element: E,
    phase: ElementDrawPhase<E::RequestLayoutState, E::PrepaintState>,
}

#[derive(Default)]
enum ElementDrawPhase<RequestLayoutState, PrepaintState> {
    #[default]
    Start,
    RequestLayout {
        layout_id: LayoutId,
        global_id: Option<GlobalElementId>,
        inspector_id: Option<InspectorElementId>,
        request_layout: RequestLayoutState,
    },
    LayoutComputed {
        layout_id: LayoutId,
        global_id: Option<GlobalElementId>,
        inspector_id: Option<InspectorElementId>,
        available_space: Size<AvailableSpace>,
        request_layout: RequestLayoutState,
    },
    Prepaint {
        node_id: DispatchNodeId,
        global_id: Option<GlobalElementId>,
        inspector_id: Option<InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: RequestLayoutState,
        prepaint: PrepaintState,
    },
    Painted,
}

/// A wrapper around an implementer of [`Element`] that allows it to be drawn in a window.
impl<E: Element> Drawable<E> {
    pub(crate) fn new(element: E) -> Self {
        Drawable {
            element,
            phase: ElementDrawPhase::Start,
        }
    }

    fn request_layout(&mut self, window: &mut Window, cx: &mut App) -> LayoutId {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::Start => {
                let global_id = self
                    .element
                    .id()
                    .map(|element_id| prepare_element_id(element_id, window));

                let inspector_id;
                #[cfg(any(feature = "inspector", debug_assertions))]
                {
                    inspector_id = if window.inspector_enabled() {
                        self.element
                            .source_location()
                            .map(|source| prepare_inspector_id(source, window))
                    } else {
                        None
                    };
                }
                #[cfg(not(any(feature = "inspector", debug_assertions)))]
                {
                    inspector_id = None;
                }

                let (layout_id, request_layout) = self.element.request_layout(
                    global_id.as_ref(),
                    inspector_id.as_ref(),
                    window,
                    cx,
                );

                if global_id.is_some() {
                    window.element_id_stack.pop();
                }

                self.phase = ElementDrawPhase::RequestLayout {
                    layout_id,
                    global_id,
                    inspector_id,
                    request_layout,
                };
                layout_id
            }
            _ => panic!("must call request_layout only once"),
        }
    }

    pub(crate) fn prepaint(&mut self, window: &mut Window, cx: &mut App) {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::RequestLayout {
                layout_id,
                global_id,
                inspector_id,
                mut request_layout,
            }
            | ElementDrawPhase::LayoutComputed {
                layout_id,
                global_id,
                inspector_id,
                mut request_layout,
                ..
            } => {
                if let Some(element_id) = self.element.id() {
                    window
                        .element_id_stack
                        .push(element_id, &mut window.element_id_interner);
                    debug_assert_eq!(
                        global_id.as_ref().map(|id| &id.components[..]),
                        Some(window.element_id_stack.components())
                    );
                }

                let bounds = window.layout_bounds(layout_id);
                let mut pushed_a11y_node = false;
                if window.a11y.is_active() {
                    if let Some(global_id) = global_id.as_ref() {
                        if let Some(role) = self.element.a11y_role() {
                            let node_id = global_id.accesskit_node_id();
                            let mut node = accesskit::Node::new(role);
                            let scale = window.scale_factor();
                            node.set_bounds(accesskit::Rect {
                                x0: (bounds.origin.x.0 * scale) as f64,
                                y0: (bounds.origin.y.0 * scale) as f64,
                                x1: ((bounds.origin.x.0 + bounds.size.width.0) * scale) as f64,
                                y1: ((bounds.origin.y.0 + bounds.size.height.0) * scale) as f64,
                            });
                            self.element.write_a11y_info(&mut node);
                            window.a11y.node_bounds.insert(node_id, bounds);
                            pushed_a11y_node = window.a11y.nodes.push(node_id, node);
                            #[cfg(debug_assertions)]
                            if pushed_a11y_node {
                                let view = window
                                    .a11y
                                    .view_type_names
                                    .get(&window.current_view())
                                    .copied();
                                let source_location = self.element.source_location();
                                window.a11y.nodes.record_node_info(
                                    node_id,
                                    crate::window::a11y::debug::NodeDebugInfo {
                                        synthetic: false,
                                        view,
                                        element_id: global_id.last().map(|id| format!("{id:?}")),
                                        source_location,
                                    },
                                );
                            }
                        }
                    }
                }

                let node_id = window.next_frame.dispatch_tree.push_node();
                let mut prepaint = self.element.prepaint(
                    global_id.as_ref(),
                    inspector_id.as_ref(),
                    bounds,
                    &mut request_layout,
                    window,
                    cx,
                );
                window.next_frame.dispatch_tree.pop_node();

                if pushed_a11y_node {
                    if let Some(global_id) = global_id.as_ref() {
                        #[cfg(debug_assertions)]
                        let creator = crate::window::a11y::debug::NodeCreator {
                            view: window
                                .a11y
                                .view_type_names
                                .get(&window.current_view())
                                .copied(),
                            element_id: global_id.last().map(|id| format!("{id:?}")),
                            source_location: self.element.source_location(),
                        };
                        let mut builder = A11ySubtreeBuilder::new(
                            global_id.accesskit_node_id(),
                            &mut window.a11y.nodes,
                        );
                        #[cfg(debug_assertions)]
                        {
                            builder = builder.with_creator(creator);
                        }
                        self.element
                            .a11y_synthetic_children(&mut prepaint, &mut builder);
                    }
                    window.a11y.nodes.pop();
                }

                if global_id.is_some() {
                    window.element_id_stack.pop();
                }

                self.phase = ElementDrawPhase::Prepaint {
                    node_id,
                    global_id,
                    inspector_id,
                    bounds,
                    request_layout,
                    prepaint,
                };
            }
            _ => panic!("must call request_layout before prepaint"),
        }
    }

    pub(crate) fn paint(
        &mut self,
        window: &mut Window,
        cx: &mut App,
    ) -> (E::RequestLayoutState, E::PrepaintState) {
        match mem::take(&mut self.phase) {
            ElementDrawPhase::Prepaint {
                node_id,
                global_id,
                inspector_id,
                bounds,
                mut request_layout,
                mut prepaint,
                ..
            } => {
                if let Some(element_id) = self.element.id() {
                    window
                        .element_id_stack
                        .push(element_id, &mut window.element_id_interner);
                    debug_assert_eq!(
                        global_id.as_ref().map(|id| &id.components[..]),
                        Some(window.element_id_stack.components())
                    );
                }

                window.next_frame.dispatch_tree.set_active_node(node_id);
                self.element.paint(
                    global_id.as_ref(),
                    inspector_id.as_ref(),
                    bounds,
                    &mut request_layout,
                    &mut prepaint,
                    window,
                    cx,
                );

                if global_id.is_some() {
                    window.element_id_stack.pop();
                }

                self.phase = ElementDrawPhase::Painted;
                (request_layout, prepaint)
            }
            _ => panic!("must call prepaint before paint"),
        }
    }

    pub(crate) fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        if matches!(&self.phase, ElementDrawPhase::Start) {
            self.request_layout(window, cx);
        }

        let layout_id = match mem::take(&mut self.phase) {
            ElementDrawPhase::RequestLayout {
                layout_id,
                global_id,
                inspector_id,
                request_layout,
            } => {
                window.compute_layout(layout_id, available_space, cx);
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    global_id,
                    inspector_id,
                    available_space,
                    request_layout,
                };
                layout_id
            }
            ElementDrawPhase::LayoutComputed {
                layout_id,
                global_id,
                inspector_id,
                available_space: prev_available_space,
                request_layout,
            } => {
                if available_space != prev_available_space {
                    window.compute_layout(layout_id, available_space, cx);
                }
                self.phase = ElementDrawPhase::LayoutComputed {
                    layout_id,
                    global_id,
                    inspector_id,
                    available_space,
                    request_layout,
                };
                layout_id
            }
            _ => panic!("cannot measure after painting"),
        };

        window.layout_bounds(layout_id).size
    }
}

impl<E> ElementObject for Drawable<E>
where
    E: Element,
    E::RequestLayoutState: 'static,
{
    fn inner_element(&mut self) -> &mut dyn Any {
        &mut self.element
    }

    #[inline]
    fn request_layout(&mut self, window: &mut Window, cx: &mut App) -> LayoutId {
        Drawable::request_layout(self, window, cx)
    }

    #[inline]
    fn prepaint(&mut self, window: &mut Window, cx: &mut App) {
        Drawable::prepaint(self, window, cx);
    }

    #[inline]
    fn paint(&mut self, window: &mut Window, cx: &mut App) {
        Drawable::paint(self, window, cx);
    }

    #[inline]
    fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        Drawable::layout_as_root(self, available_space, window, cx)
    }
}

/// A dynamically typed element that can be used to store any element type.
pub struct AnyElement(ArenaBox<dyn ElementObject>);

impl AnyElement {
    pub(crate) fn new<E>(element: E) -> Self
    where
        E: 'static + Element,
        E::RequestLayoutState: Any,
    {
        let element = with_element_arena(|arena| arena.alloc(|| Drawable::new(element)))
            .map(|element| element as &mut dyn ElementObject);
        AnyElement(element)
    }

    /// Attempt to downcast a reference to the boxed element to a specific type.
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.inner_element().downcast_mut::<T>()
    }

    /// Request the layout ID of the element stored in this `AnyElement`.
    /// Used for laying out child elements in a parent element.
    pub fn request_layout(&mut self, window: &mut Window, cx: &mut App) -> LayoutId {
        self.0.request_layout(window, cx)
    }

    /// Prepares the element to be painted by storing its bounds, giving it a chance to draw hitboxes and
    /// request autoscroll before the final paint pass is confirmed.
    pub fn prepaint(&mut self, window: &mut Window, cx: &mut App) -> Option<FocusHandle> {
        let focus_assigned = window.next_frame.focus.is_some();

        self.0.prepaint(window, cx);

        if !focus_assigned && let Some(focus_id) = window.next_frame.focus {
            return FocusHandle::for_id(focus_id, &cx.focus_handles);
        }

        None
    }

    /// Paints the element stored in this `AnyElement`.
    pub fn paint(&mut self, window: &mut Window, cx: &mut App) {
        self.0.paint(window, cx);
    }

    /// Performs layout for this element within the given available space and returns its size.
    pub fn layout_as_root(
        &mut self,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        self.0.layout_as_root(available_space, window, cx)
    }

    /// Prepaints this element at the given absolute origin.
    /// If any element in the subtree beneath this element is focused, its FocusHandle is returned.
    pub fn prepaint_at(
        &mut self,
        origin: Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<FocusHandle> {
        window.with_absolute_element_offset(origin, |window| self.prepaint(window, cx))
    }

    /// Performs layout on this element in the available space, then prepaints it at the given absolute origin.
    /// If any element in the subtree beneath this element is focused, its FocusHandle is returned.
    pub fn prepaint_as_root(
        &mut self,
        origin: Point<Pixels>,
        available_space: Size<AvailableSpace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<FocusHandle> {
        self.layout_as_root(available_space, window, cx);
        window.with_absolute_element_offset(origin, |window| self.prepaint(window, cx))
    }
}

impl Element for AnyElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id = self.request_layout(window, cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.paint(window, cx);
    }
}

impl IntoElement for AnyElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }

    fn into_any_element(self) -> AnyElement {
        self
    }
}

/// The empty element, which renders nothing.
pub struct Empty;

impl IntoElement for Empty {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Empty {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (
            window.request_layout(
                Style {
                    display: crate::Display::None,
                    ..Default::default()
                },
                None,
                cx,
            ),
            (),
        )
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _state: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        _window: &mut Window,
        _cx: &mut App,
    ) {
    }
}

#[inline(never)]
fn prepare_element_id(element_id: ElementId, window: &mut Window) -> GlobalElementId {
    window
        .element_id_stack
        .push(element_id, &mut window.element_id_interner);
    window
        .element_id_stack
        .global_id(&window.element_id_interner)
}

#[cfg(any(feature = "inspector", debug_assertions))]
#[inline(never)]
fn prepare_inspector_id(
    source: &'static panic::Location<'static>,
    window: &mut Window,
) -> InspectorElementId {
    let global_id = window
        .element_id_stack
        .global_id(&window.element_id_interner);
    // The inspector holds this path across frames.
    global_id.materialize();
    let path = InspectorElementPath {
        global_id,
        source_location: source,
    };
    window.build_inspector_element_id(path)
}

#[cfg(test)]
mod global_element_id_tests {
    use super::*;
    use std::hash::{Hash, Hasher};

    fn path(depth: usize, leaf: u64) -> Vec<ElementId> {
        let mut ids: Vec<ElementId> = (0..depth as u64)
            .map(|level| ElementId::NamedInteger("some-container-element".into(), level))
            .collect();
        ids.push(ElementId::NamedInteger("leaf".into(), leaf));
        ids
    }

    fn fx_hash(value: &impl Hash) -> u64 {
        let mut hasher = collections::FxHasher::default();
        value.hash(&mut hasher);
        hasher.finish()
    }

    fn intern_path(interner: &mut ElementIdInterner, ids: &[ElementId]) -> GlobalElementId {
        let mut stack = crate::ElementIdStack::default();
        for id in ids {
            stack.push(id.clone(), interner);
        }
        stack.global_id(interner)
    }

    #[test]
    fn global_element_id_stays_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<GlobalElementId>();
    }

    #[test]
    fn window_ids_match_paths_across_windows() {
        let mut interner = ElementIdInterner::default();
        let ids = path(8, 1);
        let first = intern_path(&mut interner, &ids);
        let again = intern_path(&mut interner, &ids);
        assert_eq!(first.components, again.components);
        assert_eq!(first, again);
        assert!(first.path.get().is_none() && again.path.get().is_none());

        let built = GlobalElementId::new(&ids);
        assert_eq!(first.hash, built.hash);
        assert_eq!(first, built);
        assert_eq!(fx_hash(&first), fx_hash(&built));
        assert_eq!(first.to_string(), built.to_string());
        assert_eq!(format!("{first:?}"), format!("{built:?}"));
        assert_eq!(
            format!("{first:?}"),
            format!("GlobalElementId {{ ids: {ids:?}, hash: {} }}", built.hash)
        );
        assert_eq!(&*first, ids.as_slice());

        let mut other_window = ElementIdInterner::default();
        // Number the other window's ids differently from the first window's.
        intern_path(&mut other_window, &path(3, 9));
        let other = intern_path(&mut other_window, &ids);
        assert_ne!(first.components, other.components);
        assert_eq!(first, other);
        assert_eq!(fx_hash(&first), fx_hash(&other));
        assert_eq!(first.accesskit_node_id(), other.accesskit_node_id());
        assert_ne!(first, intern_path(&mut other_window, &path(8, 2)));
        assert_ne!(first, intern_path(&mut interner, &path(8, 2)));
        assert_ne!(first, intern_path(&mut interner, &path(7, 1)));

        // Another window's ID keys the same element state as this window's.
        let key = interner.state_components(&first);
        assert!(key == interner.state_components(&other));
        assert!(key == interner.state_components(&built));

        // Even a path-hash collision must not equate distinct paths.
        let mut collision = intern_path(&mut interner, &path(8, 2));
        collision.hash = first.hash;
        assert_ne!(first, collision);
        assert_eq!(intern_path(&mut interner, &[]), GlobalElementId::default());
    }

    #[test]
    fn evicted_numbers_never_match_new_ids() {
        let mut interner = ElementIdInterner::default();
        let original = intern_path(&mut interner, &["evicted".into()]);
        let unmaterialized = original.clone_unmaterialized();
        let held = original.clone();
        let stale_number = original.components[0];
        interner.finish_frame();
        interner.finish_frame();
        assert!(interner.is_live(stale_number));
        interner.finish_frame();
        assert!(!interner.is_live(stale_number));
        assert!(interner.numbers.is_empty());

        // The freed slot is recycled under a new number.
        let next = intern_path(&mut interner, &["next".into()]);
        assert_eq!(slot_index(next.components[0]), slot_index(stale_number));
        assert_ne!(next.components[0], stale_number);
        assert_ne!(held, next);
        assert!(interner.state_components(&held) != interner.state_components(&next));
        assert_eq!(
            interner.names.lock()[slot_index(stale_number)]
                .as_ref()
                .map(|(_, id)| id),
            Some(&"next".into())
        );

        // The original ID gets a new number too, but a held clone still equals
        // it by path, reads its path, and keys the same state.
        let revived = intern_path(&mut interner, &["evicted".into()]);
        assert_ne!(revived.components[0], stale_number);
        assert_eq!(held, revived);
        assert_eq!(held.to_string(), "evicted");
        assert!(interner.state_components(&held) == interner.state_components(&revived));
        // The unmaterialized original can no longer read its path.
        assert!(unmaterialized.materialize().is_none());
    }

    #[test]
    fn exhausted_generation_is_retired() {
        let mut interner = ElementIdInterner::default();
        let original = intern_path(&mut interner, &[1_u64.into()]);
        let index = slot_index(original.components[0]);
        interner.slots[index].generation = u32::MAX;
        for _ in 0..3 {
            interner.finish_frame();
        }
        let next = intern_path(&mut interner, &[2_u64.into()]);
        assert_ne!(slot_index(next.components[0]), index);
    }

    #[test]
    fn interner_is_bounded_after_one_thousand_churn_frames() {
        let mut interner = ElementIdInterner::default();
        for frame in 0..1000_u64 {
            for item in 0..64_u64 {
                intern_path(
                    &mut interner,
                    &[
                        "list".into(),
                        ElementId::NamedInteger("item".into(), frame * 64 + item),
                        "label".into(),
                    ],
                );
            }
            interner.finish_frame();
            // This frame's and the previous frame's churned ids, plus two stable ones.
            assert!(interner.numbers.len() <= 2 * 64 + 2);
            assert!(interner.slots.len() <= 3 * 64 + 2);
            assert!(interner.numbers.capacity() <= 512);
        }
        interner.finish_frame();
        interner.finish_frame();
        assert!(interner.numbers.is_empty());
        assert_eq!(interner.free.len(), interner.slots.len());
        assert!(interner.names.lock().iter().all(Option::is_none));
    }

    struct ChurningKeyedState {
        frame: u64,
        stable_states: Vec<crate::EntityId>,
    }

    impl crate::Render for ChurningKeyedState {
        fn render(
            &mut self,
            window: &mut Window,
            cx: &mut crate::Context<Self>,
        ) -> impl IntoElement {
            self.frame += 1;
            let stable = window.use_keyed_state("stable", cx, |_, _| 0_u64);
            self.stable_states.push(stable.entity_id());
            for item in 0..16 {
                window.use_keyed_state(("churn", self.frame * 16 + item), cx, |_, _| 0_u64);
            }
            crate::div()
        }
    }

    #[crate::test]
    fn window_keeps_state_and_bounds_interner_under_id_churn(cx: &mut crate::TestAppContext) {
        use crate::AppContext as _;
        let window = cx.add_window(|_, _| ChurningKeyedState {
            frame: 0,
            stable_states: Vec::new(),
        });
        let any_window = crate::AnyWindowHandle::from(window);
        let mut max_numbers = 0;
        for _ in 0..200 {
            let numbers = cx
                .update_window(any_window, |_, window, cx| {
                    window.refresh();
                    window.draw(cx).clear(cx);
                    window.element_id_interner.numbers.len()
                })
                .unwrap();
            max_numbers = max_numbers.max(numbers);
        }
        // This frame's churned ids, the previous frame's, and a few stable ones.
        assert!(
            max_numbers <= 3 * 16 + 8,
            "interner grew to {max_numbers} ids"
        );
        let stable_states = window
            .update(cx, |view, _, _| view.stable_states.clone())
            .unwrap();
        assert!(stable_states.len() >= 200);
        assert!(stable_states.iter().all(|id| *id == stable_states[0]));
    }

    struct CachedChild {
        states: Vec<crate::EntityId>,
    }

    impl crate::Render for CachedChild {
        fn render(
            &mut self,
            window: &mut Window,
            cx: &mut crate::Context<Self>,
        ) -> impl IntoElement {
            let state = window.use_keyed_state("child-state", cx, |_, _| 0_u64);
            self.states.push(state.entity_id());
            crate::div()
        }
    }

    struct ChurningParent {
        frame: u64,
        child: crate::Entity<CachedChild>,
    }

    impl crate::Render for ChurningParent {
        fn render(
            &mut self,
            window: &mut Window,
            cx: &mut crate::Context<Self>,
        ) -> impl IntoElement {
            use crate::ParentElement as _;
            self.frame += 1;
            for item in 0..16 {
                window.use_keyed_state(("churn", self.frame * 16 + item), cx, |_, _| 0_u64);
            }
            crate::div().child(
                crate::AnyView::from(self.child.clone()).cached(crate::StyleRefinement::default()),
            )
        }
    }

    #[crate::test]
    fn replayed_cached_view_keeps_its_state_ids_live(cx: &mut crate::TestAppContext) {
        use crate::AppContext as _;
        let child = cx.new(|_| CachedChild { states: Vec::new() });
        let window = cx.add_window({
            let child = child.clone();
            move |_, _| ChurningParent { frame: 0, child }
        });
        let any_window = crate::AnyWindowHandle::from(window);
        let draw = |cx: &mut crate::TestAppContext| {
            cx.update_window(any_window, |_, window, cx| window.draw(cx).clear(cx))
                .unwrap();
        };
        draw(cx);
        // Only the parent renders; the child's "child-state" id is never pushed.
        for _ in 0..20 {
            window.update(cx, |_, _, cx| cx.notify()).unwrap();
            draw(cx);
        }
        assert_eq!(child.read_with(cx, |child, _| child.states.len()), 1);
        child.update(cx, |_, cx| cx.notify());
        draw(cx);
        let states = child.read_with(cx, |child, _| child.states.clone());
        assert_eq!(states.len(), 2);
        assert_eq!(states[0], states[1]);
    }

    #[test]
    fn equal_paths_are_equal_and_hash_alike() {
        let a = GlobalElementId::new(&path(8, 1));
        let b = GlobalElementId::new(&path(8, 1));
        let c = GlobalElementId::new(&path(8, 2));
        let shallower = GlobalElementId::new(&path(7, 1));
        assert_eq!(a, b);
        assert_eq!(fx_hash(&a), fx_hash(&b));
        assert_eq!(a.accesskit_node_id(), b.accesskit_node_id());
        assert_ne!(a, c);
        assert_ne!(a, shallower);
        assert_ne!(fx_hash(&a), fx_hash(&c));
        assert_eq!(&*a, path(8, 1).as_slice());
        assert_eq!(a.to_string(), b.to_string());
        assert_eq!(GlobalElementId::default(), GlobalElementId::new(&[]));
    }

    /// A window of `items` columns of `depth` nested stateful divs — the
    /// shape of a list of cards — drawn from scratch on every frame.
    struct DeepStatefulTree {
        items: usize,
        depth: usize,
    }

    impl crate::Render for DeepStatefulTree {
        fn render(&mut self, _: &mut Window, _: &mut crate::Context<Self>) -> impl IntoElement {
            use crate::{InteractiveElement as _, ParentElement as _, Styled as _, div, px};
            let depth = self.depth;
            div()
                .flex()
                .flex_wrap()
                .size_full()
                .children((0..self.items).map(move |item| {
                    let mut element = div().id(("leaf", item as u64)).size(px(2.));
                    for level in (0..depth).rev() {
                        element = div()
                            .id(("some-container-element", level as u64))
                            .child(element);
                    }
                    element.id(("item", item as u64)).size(px(4.))
                }))
        }
    }

    /// Frame cost of a tree of stateful elements: every one of them builds a
    /// `GlobalElementId` and looks its state up in two frames' maps.
    ///
    /// `cargo test -p gpui --release --lib global_element_id_tests::frame_cost -- --ignored --nocapture`
    #[crate::test]
    #[ignore = "prints timings; run by hand"]
    fn frame_cost(cx: &mut crate::TestAppContext) {
        use crate::AppContext as _;
        for (items, depth) in [(500, 4), (500, 12), (2000, 12)] {
            let window = cx.add_window(move |_, _| DeepStatefulTree { items, depth });
            let window = crate::AnyWindowHandle::from(window);
            let frame = |cx: &mut crate::TestAppContext| {
                cx.update_window(window, |_, window, cx| {
                    window.refresh();
                    let started = std::time::Instant::now();
                    window.draw(cx).clear(cx);
                    started.elapsed()
                })
                .unwrap()
            };
            for _ in 0..5 {
                frame(cx);
            }
            let mut samples: Vec<_> = (0..40).map(|_| frame(cx)).collect();
            samples.sort();
            let median = samples[samples.len() / 2];
            let stateful = items * (depth + 2);
            eprintln!(
                "{items} items x {depth} deep ({stateful} stateful elements): median frame {:.2} ms, {:.0} ns per stateful element",
                median.as_secs_f64() * 1e3,
                median.as_nanos() as f64 / stateful as f64,
            );
        }
    }

    /// `cargo test -p gpui --release --lib global_element_id_tests::hash_cost -- --ignored --nocapture`
    #[test]
    #[ignore = "prints timings; run by hand"]
    fn hash_cost() {
        let ids = path(11, 7);
        let id = GlobalElementId::new(&ids);
        let n = 1_000_000u64;
        let mut acc = 0u64;
        let started = std::time::Instant::now();
        for _ in 0..n {
            acc ^= fx_hash(&std::hint::black_box(ids.as_slice()));
        }
        let walk = started.elapsed();
        let started = std::time::Instant::now();
        for _ in 0..n {
            acc ^= fx_hash(std::hint::black_box(&id));
        }
        let cached = started.elapsed();
        let started = std::time::Instant::now();
        for _ in 0..n {
            acc ^= GlobalElementId::new(std::hint::black_box(&ids)).hash;
        }
        let build = started.elapsed();
        eprintln!(
            "12-deep path: hash by walking ids {:.1} ns, cached {:.1} ns, build (clone + hash once) {:.1} ns [{acc}]",
            walk.as_nanos() as f64 / n as f64,
            cached.as_nanos() as f64 / n as f64,
            build.as_nanos() as f64 / n as f64,
        );
    }
}
