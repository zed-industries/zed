use assistant::assistant_settings::AssistantSettings;
use assistant::{AssistantPanel, InlineAssist};
use editor::{Editor, EditorSettings};

use gpui::{
    Action, ClickEvent, ElementId, EventEmitter, InteractiveElement, ParentElement, Render, Styled,
    Subscription, View, ViewContext, WeakView,
};
use search::{buffer_search, BufferSearchBar};
use settings::{Settings, SettingsStore};
use ui::{prelude::*, ButtonSize, ButtonStyle, IconButton, IconName, IconSize, Tooltip};
use workspace::{
    item::ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};

pub struct QuickActionBar {
    buffer_search_bar: View<BufferSearchBar>,
    active_item: Option<Box<dyn ItemHandle>>,
    _inlay_hints_enabled_subscription: Option<Subscription>,
    workspace: WeakView<Workspace>,
    show: bool,
}

impl QuickActionBar {
    pub fn new(
        buffer_search_bar: View<BufferSearchBar>,
        workspace: &Workspace,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut this = Self {
            buffer_search_bar,
            active_item: None,
            _inlay_hints_enabled_subscription: None,
            workspace: workspace.weak_handle(),
            show: true,
        };
        this.apply_settings(cx);
        cx.observe_global::<SettingsStore>(|this, cx| this.apply_settings(cx))
            .detach();
        this
    }

    fn active_editor(&self) -> Option<View<Editor>> {
        self.active_item
            .as_ref()
            .and_then(|item| item.downcast::<Editor>())
    }

    fn apply_settings(&mut self, cx: &mut ViewContext<Self>) {
        let new_show = EditorSettings::get_global(cx).toolbar.quick_actions;
        if new_show != self.show {
            self.show = new_show;
            cx.emit(ToolbarItemEvent::ChangeLocation(
                self.get_toolbar_item_location(),
            ));
        }
    }

    fn get_toolbar_item_location(&self) -> ToolbarItemLocation {
        if self.show && self.active_editor().is_some() {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl Render for QuickActionBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(editor) = self.active_editor() else {
            return div().id("empty quick action bar");
        };
        let inlay_hints_button = Some(QuickActionBarButton::new(
            "toggle inlay hints",
            IconName::InlayHint,
            editor.read(cx).inlay_hints_enabled(),
            Box::new(editor::actions::ToggleInlayHints),
            "Toggle Inlay Hints",
            {
                let editor = editor.clone();
                move |_, cx| {
                    editor.update(cx, |editor, cx| {
                        editor.toggle_inlay_hints(&editor::actions::ToggleInlayHints, cx);
                    });
                }
            },
        ))
        .filter(|_| editor.read(cx).supports_inlay_hints(cx));

        let search_button = Some(QuickActionBarButton::new(
            "toggle buffer search",
            IconName::MagnifyingGlass,
            !self.buffer_search_bar.read(cx).is_dismissed(),
            Box::new(buffer_search::Deploy { focus: false }),
            "Buffer Search",
            {
                let buffer_search_bar = self.buffer_search_bar.clone();
                move |_, cx| {
                    buffer_search_bar.update(cx, |search_bar, cx| {
                        search_bar.toggle(&buffer_search::Deploy { focus: true }, cx)
                    });
                }
            },
        ))
        .filter(|_| editor.is_singleton(cx));

        let assistant_button = QuickActionBarButton::new(
            "toggle inline assistant",
            IconName::MagicWand,
            false,
            Box::new(InlineAssist),
            "Inline Assist",
            {
                let workspace = self.workspace.clone();
                move |_, cx| {
                    if let Some(workspace) = workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            AssistantPanel::inline_assist(workspace, &InlineAssist, cx);
                        });
                    }
                }
            },
        );

        h_flex()
            .id("quick action bar")
            .gap_2()
            .children(inlay_hints_button)
            .children(search_button)
            .when(AssistantSettings::get_global(cx).button, |bar| {
                bar.child(assistant_button)
            })
    }
}

impl EventEmitter<ToolbarItemEvent> for QuickActionBar {}

#[derive(IntoElement)]
struct QuickActionBarButton {
    id: ElementId,
    icon: IconName,
    toggled: bool,
    action: Box<dyn Action>,
    tooltip: SharedString,
    on_click: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
}

impl QuickActionBarButton {
    fn new(
        id: impl Into<ElementId>,
        icon: IconName,
        toggled: bool,
        action: Box<dyn Action>,
        tooltip: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            toggled,
            action,
            tooltip: tooltip.into(),
            on_click: Box::new(on_click),
        }
    }
}

impl RenderOnce for QuickActionBarButton {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        let tooltip = self.tooltip.clone();
        let action = self.action.boxed_clone();

        IconButton::new(self.id.clone(), self.icon)
            .size(ButtonSize::Compact)
            .icon_size(IconSize::Small)
            .style(ButtonStyle::Subtle)
            .selected(self.toggled)
            .tooltip(move |cx| Tooltip::for_action(tooltip.clone(), &*action, cx))
            .on_click(move |event, cx| (self.on_click)(event, cx))
    }
}

impl ToolbarItemView for QuickActionBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        self.active_item = active_pane_item.map(ItemHandle::boxed_clone);
        if let Some(active_item) = active_pane_item {
            self._inlay_hints_enabled_subscription.take();

            if let Some(editor) = active_item.downcast::<Editor>() {
                let mut inlay_hints_enabled = editor.read(cx).inlay_hints_enabled();
                let mut supports_inlay_hints = editor.read(cx).supports_inlay_hints(cx);
                self._inlay_hints_enabled_subscription =
                    Some(cx.observe(&editor, move |_, editor, cx| {
                        let editor = editor.read(cx);
                        let new_inlay_hints_enabled = editor.inlay_hints_enabled();
                        let new_supports_inlay_hints = editor.supports_inlay_hints(cx);
                        let should_notify = inlay_hints_enabled != new_inlay_hints_enabled
                            || supports_inlay_hints != new_supports_inlay_hints;
                        inlay_hints_enabled = new_inlay_hints_enabled;
                        supports_inlay_hints = new_supports_inlay_hints;
                        if should_notify {
                            cx.notify()
                        }
                    }));
            }
        }
        self.get_toolbar_item_location()
    }
}
