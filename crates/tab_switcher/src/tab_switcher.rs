#[cfg(test)]
mod tab_switcher_tests;

use collections::HashMap;
use editor::items::{
    entry_diagnostic_aware_icon_decoration_and_color, entry_git_aware_label_color,
};
use fuzzy::StringMatchCandidate;
use gpui::{
    Action, AnyElement, App, Context, DismissEvent, Entity, EntityId, EventEmitter, FocusHandle,
    Focusable, Modifiers, ModifiersChangedEvent, MouseButton, MouseUpEvent, ParentElement, Point,
    Render, Styled, Task, WeakEntity, Window, actions, rems,
};
use picker::{Picker, PickerDelegate};
use project::Project;
use schemars::JsonSchema;
use serde::Deserialize;
use settings::Settings;
use std::{cmp::Reverse, sync::Arc};
use ui::{
    DecoratedIcon, IconDecoration, IconDecorationKind, ListItem, ListItemSpacing, Tooltip,
    prelude::*,
};
use util::ResultExt;
use workspace::{
    ModalView, Pane, SaveIntent, Workspace,
    item::{ItemHandle, ItemSettings, ShowDiagnostics, TabContentParams},
    pane::{Event as PaneEvent, render_item_indicator, tab_details},
};

const PANEL_WIDTH_REMS: f32 = 28.;

/// Toggles the tab switcher interface.
#[derive(PartialEq, Clone, Deserialize, JsonSchema, Default, Action)]
#[action(namespace = tab_switcher)]
#[serde(deny_unknown_fields)]
pub struct Toggle {
    #[serde(default)]
    pub select_last: bool,
}
actions!(
    tab_switcher,
    [
        /// Closes the selected item in the tab switcher.
        CloseSelectedItem,
        /// Toggles between showing all tabs or just the current pane's tabs.
        ToggleAll
    ]
);

pub struct TabSwitcher {
    picker: Entity<Picker<TabSwitcherDelegate>>,
    init_modifiers: Option<Modifiers>,
}

impl ModalView for TabSwitcher {}

pub fn init(cx: &mut App) {
    cx.observe_new(TabSwitcher::register).detach();
}

impl TabSwitcher {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, action: &Toggle, window, cx| {
            let Some(tab_switcher) = workspace.active_modal::<Self>(cx) else {
                Self::open(workspace, action.select_last, false, window, cx);
                return;
            };

            tab_switcher.update(cx, |tab_switcher, cx| {
                tab_switcher
                    .picker
                    .update(cx, |picker, cx| picker.cycle_selection(window, cx))
            });
        });
        workspace.register_action(|workspace, _action: &ToggleAll, window, cx| {
            let Some(tab_switcher) = workspace.active_modal::<Self>(cx) else {
                Self::open(workspace, false, true, window, cx);
                return;
            };

            tab_switcher.update(cx, |tab_switcher, cx| {
                tab_switcher
                    .picker
                    .update(cx, |picker, cx| picker.cycle_selection(window, cx))
            });
        });
    }

    fn open(
        workspace: &mut Workspace,
        select_last: bool,
        is_global: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let mut weak_pane = workspace.active_pane().downgrade();
        for dock in [
            workspace.left_dock(),
            workspace.bottom_dock(),
            workspace.right_dock(),
        ] {
            dock.update(cx, |this, cx| {
                let Some(panel) = this
                    .active_panel()
                    .filter(|panel| panel.panel_focus_handle(cx).contains_focused(window, cx))
                else {
                    return;
                };
                if let Some(pane) = panel.pane(cx) {
                    weak_pane = pane.downgrade();
                }
            })
        }

        let weak_workspace = workspace.weak_handle();

        let project = workspace.project().clone();
        let original_items: Vec<_> = workspace
            .panes()
            .iter()
            .map(|p| (p.clone(), p.read(cx).active_item_index()))
            .collect();
        workspace.toggle_modal(window, cx, |window, cx| {
            let delegate = TabSwitcherDelegate::new(
                project,
                select_last,
                cx.entity().downgrade(),
                weak_pane,
                weak_workspace,
                is_global,
                window,
                cx,
                original_items,
            );
            TabSwitcher::new(delegate, window, is_global, cx)
        });
    }

    fn new(
        delegate: TabSwitcherDelegate,
        window: &mut Window,
        is_global: bool,
        cx: &mut Context<Self>,
    ) -> Self {
        let init_modifiers = if is_global {
            None
        } else {
            window.modifiers().modified().then_some(window.modifiers())
        };
        Self {
            picker: cx.new(|cx| {
                if is_global {
                    Picker::uniform_list(delegate, window, cx)
                } else {
                    Picker::nonsearchable_uniform_list(delegate, window, cx)
                }
            }),
            init_modifiers,
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(init_modifiers) = self.init_modifiers else {
            return;
        };
        if !event.modified() || !init_modifiers.is_subset_of(event) {
            self.init_modifiers = None;
            if self.picker.read(cx).delegate.matches.is_empty() {
                cx.emit(DismissEvent)
            } else {
                window.dispatch_action(menu::Confirm.boxed_clone(), cx);
            }
        }
    }

    fn handle_close_selected_item(
        &mut self,
        _: &CloseSelectedItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .close_item_at(picker.delegate.selected_index(), window, cx)
        });
    }
}

impl EventEmitter<DismissEvent> for TabSwitcher {}

impl Focusable for TabSwitcher {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for TabSwitcher {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("TabSwitcher")
            .w(rems(PANEL_WIDTH_REMS))
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_action(cx.listener(Self::handle_close_selected_item))
            .child(self.picker.clone())
    }
}

#[derive(Clone)]
struct TabMatch {
    pane: WeakEntity<Pane>,
    item_index: usize,
    item: Box<dyn ItemHandle>,
    detail: usize,
    preview: bool,
}

pub struct TabSwitcherDelegate {
    select_last: bool,
    tab_switcher: WeakEntity<TabSwitcher>,
    selected_index: usize,
    pane: WeakEntity<Pane>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    matches: Vec<TabMatch>,
    original_items: Vec<(Entity<Pane>, usize)>,
    is_all_panes: bool,
    restored_items: bool,
}

impl TabMatch {
    fn icon(
        &self,
        project: &Entity<Project>,
        selected: bool,
        window: &Window,
        cx: &App,
    ) -> Option<DecoratedIcon> {
        let icon = self.item.tab_icon(window, cx)?;
        let item_settings = ItemSettings::get_global(cx);
        let show_diagnostics = item_settings.show_diagnostics;
        let git_status_color = item_settings
            .git_status
            .then(|| {
                let path = self.item.project_path(cx)?;
                let project = project.read(cx);
                let entry = project.entry_for_path(&path, cx)?;
                let git_status = project
                    .project_path_git_status(&path, cx)
                    .map(|status| status.summary())
                    .unwrap_or_default();
                Some(entry_git_aware_label_color(
                    git_status,
                    entry.is_ignored,
                    selected,
                ))
            })
            .flatten();
        let colored_icon = icon.color(git_status_color.unwrap_or_default());

        let most_severe_diagnostic_level = if show_diagnostics == ShowDiagnostics::Off {
            None
        } else {
            let buffer_store = project.read(cx).buffer_store().read(cx);
            let buffer = self
                .item
                .project_path(cx)
                .and_then(|path| buffer_store.get_by_path(&path))
                .map(|buffer| buffer.read(cx));
            buffer.and_then(|buffer| {
                buffer
                    .buffer_diagnostics(None)
                    .iter()
                    .map(|diagnostic_entry| diagnostic_entry.diagnostic.severity)
                    .min()
            })
        };

        let decorations =
            entry_diagnostic_aware_icon_decoration_and_color(most_severe_diagnostic_level)
                .filter(|(d, _)| {
                    *d != IconDecorationKind::Triangle
                        || show_diagnostics != ShowDiagnostics::Errors
                })
                .map(|(icon, color)| {
                    let knockout_item_color = if selected {
                        cx.theme().colors().element_selected
                    } else {
                        cx.theme().colors().element_background
                    };
                    IconDecoration::new(icon, knockout_item_color, cx)
                        .color(color.color(cx))
                        .position(Point {
                            x: px(-2.),
                            y: px(-2.),
                        })
                });
        Some(DecoratedIcon::new(colored_icon, decorations))
    }
}

impl TabSwitcherDelegate {
    #[allow(clippy::complexity)]
    fn new(
        project: Entity<Project>,
        select_last: bool,
        tab_switcher: WeakEntity<TabSwitcher>,
        pane: WeakEntity<Pane>,
        workspace: WeakEntity<Workspace>,
        is_all_panes: bool,
        window: &mut Window,
        cx: &mut Context<TabSwitcher>,
        original_items: Vec<(Entity<Pane>, usize)>,
    ) -> Self {
        Self::subscribe_to_updates(&pane, window, cx);
        Self {
            select_last,
            tab_switcher,
            selected_index: 0,
            pane,
            workspace,
            project,
            matches: Vec::new(),
            is_all_panes,
            original_items,
            restored_items: false,
        }
    }

    fn subscribe_to_updates(
        pane: &WeakEntity<Pane>,
        window: &mut Window,
        cx: &mut Context<TabSwitcher>,
    ) {
        let Some(pane) = pane.upgrade() else {
            return;
        };
        cx.subscribe_in(&pane, window, |tab_switcher, _, event, window, cx| {
            match event {
                PaneEvent::AddItem { .. }
                | PaneEvent::RemovedItem { .. }
                | PaneEvent::Remove { .. } => tab_switcher.picker.update(cx, |picker, cx| {
                    let query = picker.query(cx);
                    picker.delegate.update_matches(query, window, cx);
                    cx.notify();
                }),
                _ => {}
            };
        })
        .detach();
    }

    fn update_all_pane_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let mut all_items = Vec::new();
        let mut item_index = 0;
        for pane_handle in workspace.read(cx).panes() {
            let pane = pane_handle.read(cx);
            let items: Vec<Box<dyn ItemHandle>> =
                pane.items().map(|item| item.boxed_clone()).collect();
            for ((_detail, item), detail) in items
                .iter()
                .enumerate()
                .zip(tab_details(&items, window, cx))
            {
                all_items.push(TabMatch {
                    pane: pane_handle.downgrade(),
                    item_index,
                    item: item.clone(),
                    detail,
                    preview: pane.is_active_preview_item(item.item_id()),
                });
                item_index += 1;
            }
        }

        let matches = if query.is_empty() {
            let history = workspace.read(cx).recently_activated_items(cx);
            all_items
                .sort_by_key(|tab| (Reverse(history.get(&tab.item.item_id())), tab.item_index));
            all_items
        } else {
            let candidates = all_items
                .iter()
                .enumerate()
                .flat_map(|(ix, tab_match)| {
                    Some(StringMatchCandidate::new(
                        ix,
                        &tab_match.item.tab_content_text(0, cx),
                    ))
                })
                .collect::<Vec<_>>();
            smol::block_on(fuzzy::match_strings(
                &candidates,
                &query,
                true,
                true,
                10000,
                &Default::default(),
                cx.background_executor().clone(),
            ))
            .into_iter()
            .map(|m| all_items[m.candidate_id].clone())
            .collect()
        };

        let selected_item_id = self.selected_item_id();
        self.matches = matches;
        self.selected_index = self.compute_selected_index(selected_item_id, window, cx);
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        if self.is_all_panes {
            // needed because we need to borrow the workspace, but that may be borrowed when the picker
            // calls update_matches.
            let this = cx.entity();
            window.defer(cx, move |window, cx| {
                this.update(cx, |this, cx| {
                    this.delegate.update_all_pane_matches(query, window, cx);
                })
            });
            return;
        }
        let selected_item_id = self.selected_item_id();
        self.matches.clear();
        let Some(pane) = self.pane.upgrade() else {
            return;
        };

        let pane = pane.read(cx);
        let mut history_indices = HashMap::default();
        pane.activation_history().iter().rev().enumerate().for_each(
            |(history_index, history_entry)| {
                history_indices.insert(history_entry.entity_id, history_index);
            },
        );

        let items: Vec<Box<dyn ItemHandle>> = pane.items().map(|item| item.boxed_clone()).collect();
        items
            .iter()
            .enumerate()
            .zip(tab_details(&items, window, cx))
            .map(|((item_index, item), detail)| TabMatch {
                pane: self.pane.clone(),
                item_index,
                item: item.boxed_clone(),
                detail,
                preview: pane.is_active_preview_item(item.item_id()),
            })
            .for_each(|tab_match| self.matches.push(tab_match));

        let non_history_base = history_indices.len();
        self.matches.sort_by(move |a, b| {
            let a_score = *history_indices
                .get(&a.item.item_id())
                .unwrap_or(&(a.item_index + non_history_base));
            let b_score = *history_indices
                .get(&b.item.item_id())
                .unwrap_or(&(b.item_index + non_history_base));
            a_score.cmp(&b_score)
        });

        self.selected_index = self.compute_selected_index(selected_item_id, window, cx);
    }

    fn selected_item_id(&self) -> Option<EntityId> {
        self.matches
            .get(self.selected_index())
            .map(|tab_match| tab_match.item.item_id())
    }

    fn compute_selected_index(
        &mut self,
        prev_selected_item_id: Option<EntityId>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> usize {
        if self.matches.is_empty() {
            return 0;
        }

        if let Some(selected_item_id) = prev_selected_item_id {
            // If the previously selected item is still in the list, select its new position.
            if let Some(item_index) = self
                .matches
                .iter()
                .position(|tab_match| tab_match.item.item_id() == selected_item_id)
            {
                return item_index;
            }
            // Otherwise, try to preserve the previously selected index.
            return self.selected_index.min(self.matches.len() - 1);
        }

        if self.select_last {
            return self.matches.len() - 1;
        }

        // This only runs when initially opening the picker
        // Index 0 is already active, so don't preselect it for switching.
        if self.matches.len() > 1 {
            self.set_selected_index(1, window, cx);
            return 1;
        }

        0
    }

    fn close_item_at(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Picker<TabSwitcherDelegate>>,
    ) {
        let Some(tab_match) = self.matches.get(ix) else {
            return;
        };
        let Some(pane) = self.pane.upgrade() else {
            return;
        };
        pane.update(cx, |pane, cx| {
            pane.close_item_by_id(tab_match.item.item_id(), SaveIntent::Close, window, cx)
                .detach_and_log_err(cx);
        });
    }
}

impl PickerDelegate for TabSwitcherDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search all tabs…".into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No tabs".into())
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;

        let Some(selected_match) = self.matches.get(self.selected_index()) else {
            return;
        };
        selected_match
            .pane
            .update(cx, |pane, cx| {
                if let Some(index) = pane.index_for_item(selected_match.item.as_ref()) {
                    pane.activate_item(index, false, false, window, cx);
                }
            })
            .ok();
        cx.notify();
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        Vec::new()
    }

    fn update_matches(
        &mut self,
        raw_query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.update_matches(raw_query, window, cx);
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<Picker<TabSwitcherDelegate>>,
    ) {
        let Some(selected_match) = self.matches.get(self.selected_index()) else {
            return;
        };

        self.restored_items = true;
        for (pane, index) in self.original_items.iter() {
            pane.update(cx, |this, cx| {
                this.activate_item(*index, false, false, window, cx);
            })
        }
        selected_match
            .pane
            .update(cx, |pane, cx| {
                if let Some(index) = pane.index_for_item(selected_match.item.as_ref()) {
                    pane.activate_item(index, true, true, window, cx);
                }
            })
            .ok();
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<TabSwitcherDelegate>>) {
        if !self.restored_items {
            for (pane, index) in self.original_items.iter() {
                pane.update(cx, |this, cx| {
                    this.activate_item(*index, false, false, window, cx);
                })
            }
        }

        self.tab_switcher
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let tab_match = self.matches.get(ix)?;

        let params = TabContentParams {
            detail: Some(tab_match.detail),
            selected: true,
            preview: tab_match.preview,
            deemphasized: false,
        };
        let label = tab_match.item.tab_content(params, window, cx);

        let icon = tab_match.icon(&self.project, selected, window, cx);

        let indicator = render_item_indicator(tab_match.item.boxed_clone(), cx);
        let indicator_color = if let Some(ref indicator) = indicator {
            indicator.color
        } else {
            Color::default()
        };
        let indicator = h_flex()
            .flex_shrink_0()
            .children(indicator)
            .child(div().w_2())
            .into_any_element();
        let close_button = div()
            .id("close-button")
            .on_mouse_up(
                // We need this on_mouse_up here because on macOS you may have ctrl held
                // down to open the menu, and a ctrl-click comes through as a right click.
                MouseButton::Right,
                cx.listener(move |picker, _: &MouseUpEvent, window, cx| {
                    cx.stop_propagation();
                    picker.delegate.close_item_at(ix, window, cx);
                }),
            )
            .child(
                IconButton::new("close_tab", IconName::Close)
                    .icon_size(IconSize::Small)
                    .icon_color(indicator_color)
                    .tooltip(Tooltip::for_action_title("Close", &CloseSelectedItem))
                    .on_click(cx.listener(move |picker, _, window, cx| {
                        cx.stop_propagation();
                        picker.delegate.close_item_at(ix, window, cx);
                    })),
            )
            .into_any_element();

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .inset(true)
                .toggle_state(selected)
                .child(h_flex().w_full().child(label))
                .start_slot::<DecoratedIcon>(icon)
                .map(|el| {
                    if self.selected_index == ix {
                        el.end_slot::<AnyElement>(close_button)
                    } else {
                        el.end_slot::<AnyElement>(indicator)
                            .end_hover_slot::<AnyElement>(close_button)
                    }
                }),
        )
    }
}
