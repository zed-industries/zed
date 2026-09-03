mod buffer_search_button;
mod code_actions_button;
mod editor_settings_menu;
mod inline_assist_button;
mod preview;
mod quick_action_bar_item;
mod repl_menu;
mod selections_menu;

use editor::EditorSettings;
use gpui::{
    Action, Anchor, AnchoredPositionMode, AnyElement, AnyView, Context, ElementId, Entity,
    EventEmitter, FocusHandle, InteractiveElement, ParentElement, Render, SharedString, Styled,
    Subscription, WeakEntity, Window, anchored, deferred, point,
};
use repl::components::KernelSelector;
use search::BufferSearchBar;
use settings::{Settings, SettingsStore};
use ui::{
    ButtonLike, ButtonStyle, CommonAnimationExt, ContextMenu, IconButton, IconSize,
    IconWithIndicator, PopoverMenu, PopoverTrigger, Tooltip, prelude::*,
};
use workspace::{
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace, item::ItemHandle,
};

use quick_action_bar_item::{
    AnyQuickActionItem, QuickActionBarItem, QuickActionButton, QuickActionElement,
    QuickActionKernelSelector, QuickActionMenu, QuickActionTarget, VisibilityTrigger, erase,
};

use buffer_search_button::BufferSearchButton;
use code_actions_button::CodeActionsButton;
use editor_settings_menu::EditorSettingsMenu;
use inline_assist_button::InlineAssistButton;
use preview::PreviewButton;
use repl_menu::ReplMenu;
use selections_menu::SelectionsMenu;

pub struct QuickActionBar {
    items: Vec<Box<dyn AnyQuickActionItem>>,
    target: Option<QuickActionTarget>,
    /// Mirrors the `toolbar.quick_actions` editor setting.
    show: bool,
    workspace: WeakEntity<Workspace>,
    _global_subscriptions: Vec<Subscription>,
    _editor_subscription: Option<Subscription>,
}

impl QuickActionBar {
    pub fn new(
        buffer_search_bar: Entity<BufferSearchBar>,
        workspace: &Workspace,
        cx: &mut Context<Self>,
    ) -> Self {
        let items = vec![
            erase(ReplMenu),
            erase(PreviewButton),
            erase(BufferSearchButton::new(buffer_search_bar)),
            erase(InlineAssistButton),
            erase(CodeActionsButton),
            erase(SelectionsMenu),
            erase(EditorSettingsMenu),
        ];
        Self::with_items(items, workspace.weak_handle(), cx)
    }

    fn with_items(
        items: Vec<Box<dyn AnyQuickActionItem>>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            items,
            target: None,
            show: EditorSettings::get_global(cx).toolbar.quick_actions,
            workspace,
            _global_subscriptions: Vec::new(),
            _editor_subscription: None,
        };

        // Settings are always observed because the bar's own visibility depends on them.
        this._global_subscriptions
            .push(cx.observe_global::<SettingsStore>(|this, cx| {
                this.refresh(Some(VisibilityTrigger::Settings), cx)
            }));
        // `ReplStore::global` requires `repl::init` to have run before the bar is built.
        if this.uses_trigger(VisibilityTrigger::ReplStore) {
            this._global_subscriptions.push(
                cx.observe(&repl::ReplStore::global(cx), |this, _, cx| {
                    this.refresh(Some(VisibilityTrigger::ReplStore), cx)
                }),
            );
        }

        this
    }

    fn uses_trigger(&self, trigger: VisibilityTrigger) -> bool {
        self.items
            .iter()
            .any(|item| item.triggers().contains(&trigger))
    }

    fn toolbar_item_location(&self) -> ToolbarItemLocation {
        if self.show && self.items.iter().any(|item| item.is_visible()) {
            ToolbarItemLocation::PrimaryRight
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    /// Re-resolves the items declaring `trigger`, or every item when `trigger` is `None`.
    fn refresh(&mut self, trigger: Option<VisibilityTrigger>, cx: &mut Context<Self>) {
        let previous_location = self.toolbar_item_location();
        if trigger == Some(VisibilityTrigger::Settings) {
            self.show = EditorSettings::get_global(cx).toolbar.quick_actions;
        }

        let mut changed = false;
        for item in &mut self.items {
            if trigger.is_none_or(|trigger| item.triggers().contains(&trigger)) {
                changed |= item.refresh(self.target.as_ref(), cx);
            }
        }
        if changed {
            cx.notify();
        }

        let location = self.toolbar_item_location();
        if location != previous_location {
            cx.emit(ToolbarItemEvent::ChangeLocation(location));
        }
    }

    /// Element ids are scoped by their parents, so composite variants wrap their children in
    /// an identified container and hand out short child ids instead of composing names.
    fn render_element(
        id: ElementId,
        element: QuickActionElement,
        focus_handle: &FocusHandle,
        cx: &mut App,
    ) -> AnyElement {
        match element {
            QuickActionElement::Button(button) => {
                Self::render_button(ButtonLike::new(id), button, focus_handle, cx)
            }
            QuickActionElement::Dropdown { icon, menu } => Self::render_menu(
                id,
                menu,
                IconButton::new("trigger", icon).icon_size(IconSize::Small),
            )
            .into_any_element(),
            QuickActionElement::SplitButton { button, menu } => {
                let trigger = ButtonLike::new_rounded_right("trigger")
                    .width(rems(1.))
                    .child(
                        Icon::new(IconName::ChevronDown)
                            .size(IconSize::XSmall)
                            .color(Color::Muted),
                    );
                h_flex()
                    .id(id)
                    .child(Self::render_button(
                        ButtonLike::new_rounded_left("button"),
                        button,
                        focus_handle,
                        cx,
                    ))
                    .child(Self::render_menu("menu".into(), menu, trigger))
                    .into_any_element()
            }
            QuickActionElement::KernelSelector(selector) => {
                Self::render_kernel_selector(id, selector).into_any_element()
            }
            QuickActionElement::Group(elements) => h_flex()
                .id(id)
                .children(elements.into_iter().enumerate().map(|(index, element)| {
                    Self::render_element(index.into(), element, focus_handle, cx)
                }))
                .into_any_element(),
        }
    }

    fn render_button(
        base: ButtonLike,
        button: QuickActionButton,
        focus_handle: &FocusHandle,
        cx: &App,
    ) -> AnyElement {
        let QuickActionButton {
            icon,
            tooltip,
            action,
            tooltip_meta,
            toggled,
            disabled,
            icon_color,
            indicator,
            animating,
            popup,
            on_click,
        } = button;

        let icon_color = if disabled {
            Color::Disabled
        } else if toggled {
            Color::Selected
        } else {
            icon_color.unwrap_or_default()
        };
        let icon = Icon::new(icon).size(IconSize::Small).color(icon_color);
        let icon = if animating {
            icon.with_rotate_animation(5).into_any_element()
        } else {
            IconWithIndicator::new(icon, indicator)
                .indicator_border_color(Some(cx.theme().colors().toolbar_background))
                .into_any_element()
        };

        let button = base
            .style(ButtonStyle::Subtle)
            .toggle_state(toggled)
            .disabled(disabled)
            .child(icon)
            .when(popup.is_none(), |this| {
                this.tooltip(Self::tooltip(
                    tooltip,
                    action,
                    tooltip_meta,
                    focus_handle.clone(),
                ))
            })
            .on_click(move |_, window, cx| on_click(window, cx));

        match popup {
            None => button.into_any_element(),
            Some(popup) => v_flex()
                .child(button)
                .child(deferred(
                    anchored()
                        .position_mode(AnchoredPositionMode::Local)
                        .position(point(px(20.), px(20.)))
                        .anchor(Anchor::TopRight)
                        .child(popup),
                ))
                .into_any_element(),
        }
    }

    fn render_menu<T: PopoverTrigger + ButtonCommon + Disableable>(
        id: ElementId,
        menu: QuickActionMenu,
        trigger: T,
    ) -> PopoverMenu<ContextMenu> {
        let QuickActionMenu {
            tooltip,
            disabled,
            build_menu,
        } = menu;

        PopoverMenu::new(id)
            .trigger_with_tooltip(
                trigger.style(ButtonStyle::Subtle).disabled(disabled),
                Tooltip::text(tooltip),
            )
            .anchor(Anchor::TopRight)
            .menu(move |window, cx| Some(build_menu(window, cx)))
    }

    fn render_kernel_selector(
        id: ElementId,
        selector: QuickActionKernelSelector,
    ) -> impl IntoElement {
        let QuickActionKernelSelector {
            current_kernel,
            worktree_id,
            on_select,
        } = selector;

        let label_color = if current_kernel.is_some() {
            Color::Default
        } else {
            Color::Placeholder
        };
        let trigger = ButtonLike::new(id)
            .style(ButtonStyle::Subtle)
            .size(ButtonSize::Compact)
            .child(
                h_flex()
                    .w_full()
                    .gap_0p5()
                    .child(
                        div()
                            .overflow_x_hidden()
                            .flex_grow_1()
                            .whitespace_nowrap()
                            .child(
                                Label::new(current_kernel.unwrap_or("Select Kernel".into()))
                                    .size(LabelSize::Small)
                                    .color(label_color),
                            ),
                    )
                    .child(
                        Icon::new(IconName::ChevronDown)
                            .color(Color::Muted)
                            .size(IconSize::XSmall),
                    ),
            );

        KernelSelector::new(
            on_select,
            worktree_id,
            trigger,
            Tooltip::text("Select Kernel"),
        )
    }

    fn tooltip(
        title: SharedString,
        action: Option<Box<dyn Action>>,
        meta: Option<SharedString>,
        focus_handle: FocusHandle,
    ) -> impl Fn(&mut Window, &mut App) -> AnyView {
        move |_window, cx| match (&action, &meta) {
            (Some(action), Some(meta)) => Tooltip::with_meta_in(
                title.clone(),
                Some(action.as_ref()),
                meta.clone(),
                &focus_handle,
                cx,
            ),
            (Some(action), None) => {
                Tooltip::for_action_in(title.clone(), action.as_ref(), &focus_handle, cx)
            }
            (None, Some(meta)) => Tooltip::with_meta(title.clone(), None, meta.clone(), cx),
            (None, None) => Tooltip::simple(title.clone(), cx),
        }
    }
}

impl Render for QuickActionBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(target) = self.target.as_ref() else {
            return div().id("empty quick action bar").into_any_element();
        };

        let focus_handle = target.item().item_focus_handle(cx);

        let mut container = h_flex()
            .id("quick action bar")
            .gap(DynamicSpacing::Base01.rems(cx));

        for item in &self.items {
            let Some(element) = item.render(window, cx) else {
                continue;
            };
            container = container.child(Self::render_element(
                item.id().into(),
                element,
                &focus_handle,
                cx,
            ));
        }

        container.into_any_element()
    }
}

impl EventEmitter<ToolbarItemEvent> for QuickActionBar {}

impl ToolbarItemView for QuickActionBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.target =
            active_pane_item.map(|item| QuickActionTarget::new(item, self.workspace.clone()));

        self._editor_subscription = self
            .target
            .as_ref()
            .and_then(|target| target.editor())
            .filter(|_| self.uses_trigger(VisibilityTrigger::Editor))
            .map(|editor| {
                cx.observe(editor, |this, _, cx| {
                    this.refresh(Some(VisibilityTrigger::Editor), cx)
                })
            });

        // Drop contexts resolved against the previous target right away; they may hold
        // handles to it.
        for item in &mut self.items {
            item.refresh(None, cx);
        }

        // Toolbar items are routinely activated from within an update of another entity (for
        // example the workspace opening a path), so resolving items right here would forbid
        // them from reading such entities. Report the bar as hidden for now and resolve once
        // the current effect cycle ends, which still happens before the next frame is drawn.
        let bar = cx.weak_entity();
        cx.defer(move |cx| {
            bar.update(cx, |bar, cx| bar.refresh(None, cx)).ok();
        });
        self.toolbar_item_location()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zed::tests::init_test;
    use editor::Editor;
    use gpui::{TestAppContext, UpdateGlobal, WindowHandle};
    use project::Project;
    use std::{
        cell::{Cell, RefCell},
        marker::PhantomData,
        rc::Rc,
    };
    use workspace::MultiWorkspace;

    trait TestConfig: 'static {
        const ID: &'static str;
        const TRIGGERS: &'static [VisibilityTrigger];
    }

    struct NoTriggers;
    impl TestConfig for NoTriggers {
        const ID: &'static str = "none";
        const TRIGGERS: &'static [VisibilityTrigger] = &[];
    }

    struct SettingsTrigger;
    impl TestConfig for SettingsTrigger {
        const ID: &'static str = "settings";
        const TRIGGERS: &'static [VisibilityTrigger] = &[VisibilityTrigger::Settings];
    }

    struct EditorTrigger;
    impl TestConfig for EditorTrigger {
        const ID: &'static str = "editor";
        const TRIGGERS: &'static [VisibilityTrigger] = &[VisibilityTrigger::Editor];
    }

    struct SettingsAndEditorTriggers;
    impl TestConfig for SettingsAndEditorTriggers {
        const ID: &'static str = "both";
        const TRIGGERS: &'static [VisibilityTrigger] =
            &[VisibilityTrigger::Settings, VisibilityTrigger::Editor];
    }

    struct ReplStoreTrigger;
    impl TestConfig for ReplStoreTrigger {
        const ID: &'static str = "repl";
        const TRIGGERS: &'static [VisibilityTrigger] = &[VisibilityTrigger::ReplStore];
    }

    #[derive(Clone, Default)]
    struct TestItemState {
        visible: Rc<Cell<bool>>,
        context_checks: Rc<Cell<usize>>,
        clicks: Rc<Cell<usize>>,
    }

    struct TestItem<C: TestConfig> {
        state: TestItemState,
        _config: PhantomData<C>,
    }

    impl<C: TestConfig> TestItem<C> {
        fn new(visible: bool) -> (Self, TestItemState) {
            let state = TestItemState::default();
            state.visible.set(visible);
            let item = Self {
                state: state.clone(),
                _config: PhantomData,
            };
            (item, state)
        }
    }

    impl<C: TestConfig> QuickActionBarItem for TestItem<C> {
        type Context = ();
        const ID: &'static str = C::ID;
        const TRIGGERS: &'static [VisibilityTrigger] = C::TRIGGERS;

        fn context(&self, _target: &QuickActionTarget, _cx: &mut App) -> Option<()> {
            self.state
                .context_checks
                .set(self.state.context_checks.get() + 1);
            self.state.visible.get().then_some(())
        }

        fn render(&self, _: &(), _window: &mut Window, _cx: &mut App) -> QuickActionElement {
            let clicks = self.state.clicks.clone();
            QuickActionElement::Button(QuickActionButton::new(
                IconName::Check,
                C::ID,
                move |_, _| clicks.set(clicks.get() + 1),
            ))
        }
    }

    struct TestBar {
        window: WindowHandle<MultiWorkspace>,
        editor: Entity<Editor>,
        bar: Entity<QuickActionBar>,
        locations: Rc<RefCell<Vec<ToolbarItemLocation>>>,
    }

    async fn build_bar(
        cx: &mut TestAppContext,
        items: Vec<Box<dyn AnyQuickActionItem>>,
    ) -> TestBar {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .unwrap();
        let locations = Rc::new(RefCell::new(Vec::new()));

        let (editor, bar) = window
            .update(cx, |_, window, cx| {
                let editor = cx.new(|cx| Editor::single_line(window, cx));
                let bar = cx.new(|cx| {
                    QuickActionBar::with_items(items, workspace.read(cx).weak_handle(), cx)
                });
                (editor, bar)
            })
            .unwrap();
        cx.update(|cx| {
            cx.subscribe(&bar, {
                let locations = locations.clone();
                move |_, event, _| {
                    let ToolbarItemEvent::ChangeLocation(location) = event;
                    locations.borrow_mut().push(*location);
                }
            })
            .detach();
        });

        TestBar {
            window,
            editor,
            bar,
            locations,
        }
    }

    impl TestBar {
        /// Activates `item` and returns the location the bar settled on once the deferred
        /// evaluation ran; `set_active_pane_item` itself always reports `Hidden`.
        fn set_active_item(
            &self,
            item: Option<Entity<Editor>>,
            cx: &mut TestAppContext,
        ) -> ToolbarItemLocation {
            let provisional_location = self
                .window
                .update(cx, |_, window, cx| {
                    self.bar.update(cx, |bar, cx| {
                        bar.set_active_pane_item(
                            item.as_ref().map(|editor| editor as &dyn ItemHandle),
                            window,
                            cx,
                        )
                    })
                })
                .unwrap();
            assert_eq!(provisional_location, ToolbarItemLocation::Hidden);
            cx.run_until_parked();
            self.bar.read_with(cx, |bar, _| bar.toolbar_item_location())
        }

        fn notify_editor(&self, cx: &mut TestAppContext) {
            self.editor.update(cx, |_, cx| cx.notify());
            cx.run_until_parked();
        }

        fn notify_repl_store(&self, cx: &mut TestAppContext) {
            cx.update(|cx| repl::ReplStore::global(cx).update(cx, |_, cx| cx.notify()));
            cx.run_until_parked();
        }

        fn set_quick_actions_setting(&self, enabled: bool, cx: &mut TestAppContext) {
            cx.update(|cx| {
                SettingsStore::update_global(cx, |store, cx| {
                    store.update_user_settings(cx, |settings| {
                        settings
                            .editor
                            .toolbar
                            .get_or_insert_default()
                            .quick_actions = Some(enabled);
                    });
                });
            });
            cx.run_until_parked();
        }

        fn visible_ids(&self, cx: &mut TestAppContext) -> Vec<&'static str> {
            self.bar.read_with(cx, |bar, _| {
                bar.items
                    .iter()
                    .filter(|item| item.is_visible())
                    .map(|item| item.id())
                    .collect()
            })
        }

        /// Renders every item the bar would show and clicks each resulting button.
        fn click_all_visible(&self, cx: &mut TestAppContext) {
            self.window
                .update(cx, |_, window, cx| {
                    self.bar.update(cx, |bar, cx| {
                        for item in &bar.items {
                            match item.render(window, cx) {
                                Some(QuickActionElement::Button(button)) => {
                                    (button.on_click)(window, cx)
                                }
                                Some(_) => panic!("test items only render buttons"),
                                None => {}
                            }
                        }
                    })
                })
                .unwrap();
        }

        fn take_locations(&self) -> Vec<ToolbarItemLocation> {
            std::mem::take(&mut *self.locations.borrow_mut())
        }
    }

    #[gpui::test]
    async fn test_context_is_only_refreshed_for_declared_triggers(cx: &mut TestAppContext) {
        let (none, none_state) = TestItem::<NoTriggers>::new(true);
        let (settings, settings_state) = TestItem::<SettingsTrigger>::new(true);
        let (editor, editor_state) = TestItem::<EditorTrigger>::new(true);
        let (both, both_state) = TestItem::<SettingsAndEditorTriggers>::new(true);
        let states = [none_state, settings_state, editor_state, both_state];
        let checks = || {
            states
                .iter()
                .map(|state| state.context_checks.get())
                .collect::<Vec<_>>()
        };

        let bar = build_bar(
            cx,
            vec![erase(none), erase(settings), erase(editor), erase(both)],
        )
        .await;
        assert_eq!(
            checks(),
            [0, 0, 0, 0],
            "nothing is resolved without a target"
        );

        let location = bar.set_active_item(Some(bar.editor.clone()), cx);
        assert_eq!(location, ToolbarItemLocation::PrimaryRight);
        assert_eq!(checks(), [1, 1, 1, 1], "a new target resolves every item");

        bar.notify_editor(cx);
        assert_eq!(
            checks(),
            [1, 1, 2, 2],
            "editor notifications only hit editor triggers"
        );

        // The editor reacts to settings changes by notifying, so editor-triggered items may
        // legitimately be re-resolved here as well; only the other two counts are exact.
        bar.set_quick_actions_setting(false, cx);
        let checks = checks();
        assert_eq!(checks[0], 1, "items without triggers are never re-resolved");
        assert_eq!(checks[1], 2, "settings changes hit settings triggers");

        // Hiding an item is only picked up through one of its declared triggers.
        states[0].visible.set(false);
        states[1].visible.set(false);
        bar.notify_editor(cx);
        assert_eq!(bar.visible_ids(cx), ["none", "settings", "editor", "both"]);
        bar.set_quick_actions_setting(true, cx);
        assert_eq!(bar.visible_ids(cx), ["none", "editor", "both"]);

        states[2].visible.set(false);
        bar.notify_editor(cx);
        assert_eq!(bar.visible_ids(cx), ["none", "both"]);
    }

    #[gpui::test]
    async fn test_repl_store_trigger_is_only_observed_when_declared(cx: &mut TestAppContext) {
        let (none, none_state) = TestItem::<NoTriggers>::new(true);
        let (repl, repl_state) = TestItem::<ReplStoreTrigger>::new(true);
        let bar = build_bar(cx, vec![erase(none), erase(repl)]).await;
        bar.set_active_item(Some(bar.editor.clone()), cx);

        repl_state.visible.set(false);
        bar.notify_repl_store(cx);
        assert_eq!(none_state.context_checks.get(), 1);
        assert_eq!(repl_state.context_checks.get(), 2);
        assert_eq!(bar.visible_ids(cx), ["none"]);

        let (none, _) = TestItem::<NoTriggers>::new(true);
        let bar_without_repl = build_bar(cx, vec![erase(none)]).await;
        assert_eq!(
            bar_without_repl
                .bar
                .read_with(cx, |bar, _| bar._global_subscriptions.len()),
            1,
            "only the settings store is observed when no item needs the repl store"
        );
    }

    #[gpui::test]
    async fn test_location_follows_visibility_and_setting(cx: &mut TestAppContext) {
        let (item, state) = TestItem::<EditorTrigger>::new(false);
        let bar = build_bar(cx, vec![erase(item)]).await;

        assert_eq!(
            bar.set_active_item(Some(bar.editor.clone()), cx),
            ToolbarItemLocation::Hidden,
            "hidden while no item is visible"
        );

        state.visible.set(true);
        bar.notify_editor(cx);
        assert_eq!(bar.take_locations(), [ToolbarItemLocation::PrimaryRight]);

        bar.notify_editor(cx);
        assert_eq!(bar.take_locations(), [], "no event when nothing changed");

        bar.set_quick_actions_setting(false, cx);
        assert_eq!(bar.take_locations(), [ToolbarItemLocation::Hidden]);

        bar.set_quick_actions_setting(true, cx);
        assert_eq!(bar.take_locations(), [ToolbarItemLocation::PrimaryRight]);

        state.visible.set(false);
        bar.notify_editor(cx);
        assert_eq!(bar.take_locations(), [ToolbarItemLocation::Hidden]);

        state.visible.set(true);
        assert_eq!(
            bar.set_active_item(None, cx),
            ToolbarItemLocation::Hidden,
            "hidden without a target regardless of item state"
        );
        assert_eq!(bar.visible_ids(cx), Vec::<&str>::new());
    }

    #[gpui::test]
    async fn test_only_items_with_a_context_are_rendered(cx: &mut TestAppContext) {
        let (hidden, hidden_state) = TestItem::<NoTriggers>::new(false);
        let (shown, shown_state) = TestItem::<EditorTrigger>::new(true);
        let bar = build_bar(cx, vec![erase(hidden), erase(shown)]).await;
        bar.set_active_item(Some(bar.editor.clone()), cx);

        bar.click_all_visible(cx);
        assert_eq!(hidden_state.clicks.get(), 0);
        assert_eq!(shown_state.clicks.get(), 1);

        shown_state.visible.set(false);
        bar.notify_editor(cx);
        bar.click_all_visible(cx);
        assert_eq!(
            shown_state.clicks.get(),
            1,
            "items lose their context when hidden"
        );
    }
}
