use editor::Editor;
use gpui::{Action, AnyElement, App, Entity, SharedString, WeakEntity, Window};
use project::WorktreeId;
use repl::KernelSpecification;
use ui::{Color, ContextMenu, IconName, Indicator};
use workspace::{Workspace, item::ItemHandle};

/// The pane item the quick action bar is currently attached to.
pub struct QuickActionTarget {
    item: Box<dyn ItemHandle>,
    editor: Option<Entity<Editor>>,
    workspace: WeakEntity<Workspace>,
}

impl QuickActionTarget {
    pub(super) fn new(item: &dyn ItemHandle, workspace: WeakEntity<Workspace>) -> Self {
        Self {
            item: item.boxed_clone(),
            editor: item.downcast::<Editor>(),
            workspace,
        }
    }

    pub fn item(&self) -> &dyn ItemHandle {
        self.item.as_ref()
    }

    pub fn editor(&self) -> Option<&Entity<Editor>> {
        self.editor.as_ref()
    }

    pub fn workspace(&self) -> &WeakEntity<Workspace> {
        &self.workspace
    }
}

/// A source of change the bar knows how to observe. Items declare which of these affect
/// their [`QuickActionBarItem::context`]; the active pane item changing always does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisibilityTrigger {
    Settings,
    /// The active editor called `cx.notify()`.
    Editor,
    /// Needed because kernelspecs are loaded asynchronously, after the editor became active.
    ReplStore,
}

/// An icon button rendered by the bar.
pub struct QuickActionButton {
    pub icon: IconName,
    pub tooltip: SharedString,
    /// When set, the tooltip shows this action's keybinding, resolved in the target item's
    /// focus context.
    pub action: Option<Box<dyn Action>>,
    /// Secondary line shown in the tooltip.
    pub tooltip_meta: Option<SharedString>,
    pub toggled: bool,
    pub disabled: bool,
    pub icon_color: Option<Color>,
    /// Status dot drawn on the icon. Ignored while `animating`.
    pub indicator: Option<Indicator>,
    /// Continuously rotates the icon, e.g. while a kernel is busy.
    pub animating: bool,
    /// Rendered deferred and anchored below the button for as long as it is present, for
    /// popups that are not a plain [`ContextMenu`] (e.g. the editor's code actions menu).
    /// The tooltip is suppressed while a popup is shown.
    pub popup: Option<AnyElement>,
    pub on_click: Box<dyn Fn(&mut Window, &mut App)>,
}

impl QuickActionButton {
    pub fn new(
        icon: IconName,
        tooltip: impl Into<SharedString>,
        on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            icon,
            tooltip: tooltip.into(),
            action: None,
            tooltip_meta: None,
            toggled: false,
            disabled: false,
            icon_color: None,
            indicator: None,
            animating: false,
            popup: None,
            on_click: Box::new(on_click),
        }
    }

    pub fn action(mut self, action: Box<dyn Action>) -> Self {
        self.action = Some(action);
        self
    }

    pub fn tooltip_meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.tooltip_meta = Some(meta.into());
        self
    }

    pub fn toggled(mut self, toggled: bool) -> Self {
        self.toggled = toggled;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn icon_color(mut self, color: Color) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn indicator(mut self, indicator: Option<Indicator>) -> Self {
        self.indicator = indicator;
        self
    }

    pub fn animating(mut self, animating: bool) -> Self {
        self.animating = animating;
        self
    }

    pub fn popup(mut self, popup: Option<AnyElement>) -> Self {
        self.popup = popup;
        self
    }
}

/// A [`ContextMenu`] deployed from a trigger in the bar.
///
/// The popover owns the trigger's toggle state and hides the tooltip while open, which is
/// why this is a smaller descriptor than [`QuickActionButton`].
pub struct QuickActionMenu {
    pub tooltip: SharedString,
    pub disabled: bool,
    pub build_menu: Box<dyn Fn(&mut Window, &mut App) -> Entity<ContextMenu>>,
}

impl QuickActionMenu {
    pub fn new(
        tooltip: impl Into<SharedString>,
        build_menu: impl Fn(&mut Window, &mut App) -> Entity<ContextMenu> + 'static,
    ) -> Self {
        Self {
            tooltip: tooltip.into(),
            disabled: false,
            build_menu: Box::new(build_menu),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// A labeled trigger that opens the REPL kernel picker for `worktree_id`.
pub struct QuickActionKernelSelector {
    /// Name of the kernel currently in use; a placeholder is shown when `None`.
    pub current_kernel: Option<SharedString>,
    pub worktree_id: WorktreeId,
    pub on_select: Box<dyn Fn(KernelSpecification, &mut Window, &mut App)>,
}

/// What an item looks like in the bar. Click behavior lives on the respective component.
pub enum QuickActionElement {
    Button(QuickActionButton),
    /// An icon button that opens a menu.
    Dropdown {
        icon: IconName,
        menu: QuickActionMenu,
    },
    /// A button with an attached chevron that opens a menu, drawn as one rounded control.
    SplitButton {
        button: QuickActionButton,
        menu: QuickActionMenu,
    },
    KernelSelector(QuickActionKernelSelector),
    /// Tightly coupled controls rendered side by side.
    Group(Vec<QuickActionElement>),
}

/// A single entry in the quick action bar.
///
/// The bar owns the visibility lifecycle: it calls [`context`](Self::context) when the active
/// pane item changes and whenever one of the declared [`TRIGGERS`](Self::TRIGGERS) fires,
/// stores the result, and only calls [`render`](Self::render) while a context exists. Items
/// therefore never re-check their own visibility while rendering, and everything `render`
/// needs about the target can be resolved once in `context`.
pub trait QuickActionBarItem: 'static {
    /// Everything the item resolved about the target that `render` needs. Use `()` when the
    /// item only needs to know that it is applicable. Equality decides whether a refresh
    /// needs a re-render, so this must compare everything `render` depends on.
    type Context: PartialEq + 'static;

    /// Stable identifier used to derive element ids. Must be unique within the bar.
    const ID: &'static str;

    /// Which changes, besides the active pane item changing, require re-evaluating
    /// [`context`](Self::context).
    const TRIGGERS: &'static [VisibilityTrigger];

    /// Resolves the item against `target`. `None` hides the item. Only called by the bar in
    /// response to a trigger, never during render.
    fn context(&self, target: &QuickActionTarget, cx: &mut App) -> Option<Self::Context>;

    /// Describes the item for the current frame. Called on every render while a context
    /// exists. The returned value must be consumed within the same frame and never cached.
    fn render(
        &self,
        context: &Self::Context,
        window: &mut Window,
        cx: &mut App,
    ) -> QuickActionElement;
}

/// Object-safe view of a [`QuickActionBarItem`] together with its stored context, so the bar
/// can hold items with different `Context` types in one list.
pub(super) trait AnyQuickActionItem {
    fn id(&self) -> &'static str;
    fn triggers(&self) -> &'static [VisibilityTrigger];
    fn is_visible(&self) -> bool;
    /// Re-resolves the context against `target`. Returns whether it changed.
    fn refresh(&mut self, target: Option<&QuickActionTarget>, cx: &mut App) -> bool;
    fn render(&self, window: &mut Window, cx: &mut App) -> Option<QuickActionElement>;
}

struct ItemState<T: QuickActionBarItem> {
    item: T,
    context: Option<T::Context>,
}

pub(super) fn erase(item: impl QuickActionBarItem) -> Box<dyn AnyQuickActionItem> {
    Box::new(ItemState {
        item,
        context: None,
    })
}

impl<T: QuickActionBarItem> AnyQuickActionItem for ItemState<T> {
    fn id(&self) -> &'static str {
        T::ID
    }

    fn triggers(&self) -> &'static [VisibilityTrigger] {
        T::TRIGGERS
    }

    fn is_visible(&self) -> bool {
        self.context.is_some()
    }

    fn refresh(&mut self, target: Option<&QuickActionTarget>, cx: &mut App) -> bool {
        let context = target.and_then(|target| self.item.context(target, cx));
        let changed = context != self.context;
        self.context = context;
        changed
    }

    fn render(&self, window: &mut Window, cx: &mut App) -> Option<QuickActionElement> {
        let context = self.context.as_ref()?;
        Some(self.item.render(context, window, cx))
    }
}
