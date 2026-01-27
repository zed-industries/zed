use command_palette_hooks::CommandPaletteFilter;
use editor::{Anchor, Editor, MultiBufferSnapshot, SelectionEffects, ToPoint, scroll::Autoscroll};
use gpui::{
    Action, App, AppContext as _, Context, Corner, Div, Entity, EntityId, EventEmitter,
    FocusHandle, Focusable, HighlightStyle, Hsla, InteractiveElement, IntoElement, MouseButton,
    MouseDownEvent, MouseMoveEvent, ParentElement, Render, ScrollStrategy, SharedString, Styled,
    Task, UniformListScrollHandle, WeakEntity, Window, actions, div, rems, uniform_list,
};
use menu::{SelectNext, SelectPrevious};
use std::{any::TypeId, mem, ops::Range};
use theme::ActiveTheme;
use ui::{
    ButtonCommon, ButtonLike, ButtonStyle, Clickable, Color, ContextMenu, FluentBuilder as _,
    IconButton, IconName, IconPosition, IconSize, Label, LabelCommon, LabelSize, PopoverMenu,
    PopoverMenuHandle, StyledExt, Toggleable, Tooltip, WithScrollbar, h_flex, v_flex,
};
use workspace::{
    Event as WorkspaceEvent, SplitDirection, ToolbarItemEvent, ToolbarItemLocation,
    ToolbarItemView, Workspace,
    item::{Item, ItemHandle},
};

actions!(
    dev,
    [
        /// Opens the highlights tree view for the current file.
        OpenHighlightsTreeView,
    ]
);

actions!(
    highlights_tree_view,
    [
        /// Update the highlights tree view to show the last focused file.
        UseActiveEditor,
        /// Toggles showing text highlights.
        ToggleTextHighlights,
        /// Toggles showing semantic token highlights.
        ToggleSemanticTokens,
    ]
);

pub fn init(cx: &mut App) {
    let highlights_tree_actions = [TypeId::of::<UseActiveEditor>()];

    CommandPaletteFilter::update_global(cx, |this, _| {
        this.hide_action_types(&highlights_tree_actions);
    });

    cx.observe_new(move |workspace: &mut Workspace, _, _| {
        workspace.register_action(move |workspace, _: &OpenHighlightsTreeView, window, cx| {
            CommandPaletteFilter::update_global(cx, |this, _| {
                this.show_action_types(&highlights_tree_actions);
            });

            let active_item = workspace.active_item(cx);
            let workspace_handle = workspace.weak_handle();
            let highlights_tree_view = cx.new(|cx| {
                cx.on_release(move |view: &mut HighlightsTreeView, cx| {
                    if view
                        .workspace_handle
                        .read_with(cx, |workspace, cx| {
                            workspace.item_of_type::<HighlightsTreeView>(cx).is_none()
                        })
                        .unwrap_or_default()
                    {
                        CommandPaletteFilter::update_global(cx, |this, _| {
                            this.hide_action_types(&highlights_tree_actions);
                        });
                    }
                })
                .detach();

                HighlightsTreeView::new(workspace_handle, active_item, window, cx)
            });
            workspace.split_item(
                SplitDirection::Right,
                Box::new(highlights_tree_view),
                window,
                cx,
            )
        });
        workspace.register_action(|workspace, _: &UseActiveEditor, window, cx| {
            if let Some(tree_view) = workspace.item_of_type::<HighlightsTreeView>(cx) {
                tree_view.update(cx, |view, cx| {
                    view.update_active_editor(&Default::default(), window, cx)
                })
            }
        });
    })
    .detach();
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HighlightCategory {
    Text,
    SemanticToken(Option<SharedString>),
}

impl HighlightCategory {
    fn label(&self) -> SharedString {
        match self {
            HighlightCategory::Text => "text".into(),
            HighlightCategory::SemanticToken(Some(token_type)) => {
                format!("semantic token: {token_type}").into()
            }
            HighlightCategory::SemanticToken(None) => "semantic token".into(),
        }
    }
}

#[derive(Debug, Clone)]
struct HighlightEntry {
    range: Range<Anchor>,
    range_display: SharedString,
    style: HighlightStyle,
    category: HighlightCategory,
    sort_key: (u32, u32, u32, u32),
}

pub struct HighlightsTreeView {
    workspace_handle: WeakEntity<Workspace>,
    editor: Option<EditorState>,
    list_scroll_handle: UniformListScrollHandle,
    last_active_editor: Option<Entity<Editor>>,
    selected_item_ix: Option<usize>,
    hovered_item_ix: Option<usize>,
    focus_handle: FocusHandle,
    cached_entries: Vec<HighlightEntry>,
    show_text_highlights: bool,
    show_semantic_tokens: bool,
    skip_next_scroll: bool,
}

pub struct HighlightsTreeToolbarItemView {
    tree_view: Option<Entity<HighlightsTreeView>>,
    _subscription: Option<gpui::Subscription>,
    toggle_settings_handle: PopoverMenuHandle<ContextMenu>,
}

struct EditorState {
    editor: Entity<Editor>,
    _subscription: gpui::Subscription,
}

impl HighlightsTreeView {
    pub fn new(
        workspace_handle: WeakEntity<Workspace>,
        active_item: Option<Box<dyn ItemHandle>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            workspace_handle: workspace_handle.clone(),
            list_scroll_handle: UniformListScrollHandle::new(),
            editor: None,
            last_active_editor: None,
            hovered_item_ix: None,
            selected_item_ix: None,
            focus_handle: cx.focus_handle(),
            cached_entries: Vec::new(),
            show_text_highlights: true,
            show_semantic_tokens: true,
            skip_next_scroll: false,
        };

        this.handle_item_updated(active_item, window, cx);

        cx.subscribe_in(
            &workspace_handle.upgrade().unwrap(),
            window,
            move |this, workspace, event, window, cx| match event {
                WorkspaceEvent::ItemAdded { .. } | WorkspaceEvent::ActiveItemChanged => {
                    this.handle_item_updated(workspace.read(cx).active_item(cx), window, cx)
                }
                WorkspaceEvent::ItemRemoved { item_id } => {
                    this.handle_item_removed(item_id, window, cx);
                }
                _ => {}
            },
        )
        .detach();

        this
    }

    fn handle_item_updated(
        &mut self,
        active_item: Option<Box<dyn ItemHandle>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(editor) = active_item
            .filter(|item| item.item_id() != cx.entity_id())
            .and_then(|item| item.act_as::<Editor>(cx))
        else {
            return;
        };

        if let Some(editor_state) = self.editor.as_ref() {
            self.last_active_editor = (editor_state.editor != editor).then_some(editor);
        } else {
            self.set_editor(editor, window, cx);
        }
    }

    fn handle_item_removed(
        &mut self,
        item_id: &EntityId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .editor
            .as_ref()
            .is_some_and(|state| state.editor.entity_id() == *item_id)
        {
            self.editor = None;
            self.cached_entries.clear();
            self.update_active_editor(&Default::default(), window, cx);
            cx.notify();
        }
    }

    fn update_active_editor(
        &mut self,
        _: &UseActiveEditor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(editor) = self.last_active_editor.take() else {
            return;
        };
        self.set_editor(editor, window, cx);
    }

    fn set_editor(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(state) = &self.editor {
            if state.editor == editor {
                return;
            }
            editor.update(cx, |editor, cx| {
                editor.clear_background_highlights::<Self>(cx)
            });
        }

        let subscription =
            cx.subscribe_in(&editor, window, |this, _, event, window, cx| match event {
                editor::EditorEvent::Reparsed(_)
                | editor::EditorEvent::SelectionsChanged { .. } => {
                    this.refresh_highlights(window, cx);
                }
                _ => return,
            });

        self.editor = Some(EditorState {
            editor,
            _subscription: subscription,
        });
        self.refresh_highlights(window, cx);
    }

    fn refresh_highlights(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(editor_state) = self.editor.as_ref() else {
            self.cached_entries.clear();
            cx.notify();
            return;
        };

        let (display_map, multi_buffer, cursor_position) = {
            let editor = editor_state.editor.read(cx);
            let cursor = editor.selections.newest_anchor().head();
            (editor.display_map.clone(), editor.buffer().clone(), cursor)
        };

        let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);

        let mut entries = Vec::new();

        display_map.update(cx, |display_map, _cx| {
            for (_, arc) in display_map.all_text_highlights() {
                let style = arc.0;
                let ranges = &arc.1;

                for range in ranges.iter() {
                    let (range_display, sort_key) =
                        format_anchor_range(range, &multi_buffer_snapshot);
                    entries.push(HighlightEntry {
                        range: range.clone(),
                        range_display,
                        style,
                        category: HighlightCategory::Text,
                        sort_key,
                    });
                }
            }

            for (_, tokens) in display_map.all_semantic_token_highlights() {
                for token in tokens.iter() {
                    let (range_display, sort_key) =
                        format_anchor_range(&token.range, &multi_buffer_snapshot);
                    entries.push(HighlightEntry {
                        range: token.range.clone(),
                        range_display,
                        style: token.style,
                        category: HighlightCategory::SemanticToken(token.token_type.clone()),
                        sort_key,
                    });
                }
            }
        });

        entries.sort_by(|a, b| a.sort_key.cmp(&b.sort_key));

        self.cached_entries = entries;

        if self.skip_next_scroll {
            self.skip_next_scroll = false;
        } else {
            self.scroll_to_cursor_position(&cursor_position, &multi_buffer_snapshot);
        }
        cx.notify();
    }

    fn scroll_to_cursor_position(&mut self, cursor: &Anchor, snapshot: &MultiBufferSnapshot) {
        let cursor_point = cursor.to_point(snapshot);
        let cursor_key = (cursor_point.row, cursor_point.column);

        let filtered = self.filtered_entries();

        let best_ix = filtered
            .iter()
            .enumerate()
            .filter(|(_, (_, entry))| {
                let (start_row, start_col, end_row, end_col) = entry.sort_key;
                let start = (start_row, start_col);
                let end = (end_row, end_col);
                cursor_key >= start && cursor_key <= end
            })
            .min_by_key(|(_, (_, entry))| {
                let (start_row, start_col, end_row, end_col) = entry.sort_key;
                (end_row - start_row, end_col.saturating_sub(start_col))
            })
            .map(|(filtered_ix, (original_ix, _))| (filtered_ix, original_ix));

        if let Some((filtered_ix, original_ix)) = best_ix {
            self.selected_item_ix = Some(*original_ix);
            self.list_scroll_handle
                .scroll_to_item(filtered_ix, ScrollStrategy::Center);
        }
    }

    fn filtered_entries(&self) -> Vec<(usize, &HighlightEntry)> {
        self.cached_entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| match entry.category {
                HighlightCategory::Text => self.show_text_highlights,
                HighlightCategory::SemanticToken(_) => self.show_semantic_tokens,
            })
            .collect()
    }

    fn update_editor_with_range_for_entry(
        &self,
        entry_ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
        mut f: impl FnMut(&mut Editor, Range<Anchor>, usize, &mut Window, &mut Context<Editor>),
    ) -> Option<()> {
        let editor_state = self.editor.as_ref()?;
        let entry = self.cached_entries.get(entry_ix)?;
        let range = entry.range.clone();
        let key = cx.entity_id().as_u64() as usize;

        editor_state.editor.update(cx, |editor, cx| {
            f(editor, range, key, window, cx);
        });
        Some(())
    }

    fn render_entry(&self, entry: &HighlightEntry, selected: bool, cx: &App) -> Div {
        let colors = cx.theme().colors();
        let style_preview = render_style_preview(entry.style, cx);

        h_flex()
            .gap_1()
            .child(style_preview)
            .child(Label::new(entry.range_display.clone()).color(Color::Default))
            .child(
                Label::new(entry.category.label())
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
            .text_bg(if selected {
                colors.element_selected
            } else {
                Hsla::default()
            })
            .pl(rems(0.5))
            .hover(|style| style.bg(colors.element_hover))
    }

    fn compute_items(
        &mut self,
        visible_range: Range<usize>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Div> {
        let filtered = self.filtered_entries();
        let mut items = Vec::new();

        for visible_ix in visible_range {
            let Some(&(original_ix, entry)) = filtered.get(visible_ix) else {
                continue;
            };

            let selected = Some(original_ix) == self.selected_item_ix;
            let rendered = self
                .render_entry(entry, selected, cx)
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |tree_view, _: &MouseDownEvent, window, cx| {
                        tree_view.selected_item_ix = Some(original_ix);
                        tree_view.skip_next_scroll = true;
                        tree_view.update_editor_with_range_for_entry(
                            original_ix,
                            window,
                            cx,
                            |editor, mut range, _, window, cx| {
                                mem::swap(&mut range.start, &mut range.end);
                                editor.change_selections(
                                    SelectionEffects::scroll(Autoscroll::newest()),
                                    window,
                                    cx,
                                    |selections| {
                                        selections.select_ranges([range]);
                                    },
                                );
                            },
                        );
                        cx.notify();
                    }),
                )
                .on_mouse_move(
                    cx.listener(move |tree_view, _: &MouseMoveEvent, window, cx| {
                        if tree_view.hovered_item_ix != Some(original_ix) {
                            tree_view.hovered_item_ix = Some(original_ix);
                            tree_view.update_editor_with_range_for_entry(
                                original_ix,
                                window,
                                cx,
                                |editor, range, key, _, cx| {
                                    Self::set_editor_highlights(editor, key, &[range], cx);
                                },
                            );
                            cx.notify();
                        }
                    }),
                );

            items.push(rendered);
        }

        items
    }

    fn set_editor_highlights(
        editor: &mut Editor,
        key: usize,
        ranges: &[Range<Anchor>],
        cx: &mut Context<Editor>,
    ) {
        editor.highlight_background_key::<Self>(
            key,
            ranges,
            |_, theme| theme.colors().editor_document_highlight_write_background,
            cx,
        );
    }

    fn clear_editor_highlights(editor: &Entity<Editor>, cx: &mut Context<Self>) {
        let highlight_key = cx.entity_id().as_u64() as usize;
        editor.update(cx, |editor, cx| {
            editor.clear_background_highlights_key::<Self>(highlight_key, cx);
        });
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        self.move_selection(-1, window, cx);
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        self.move_selection(1, window, cx);
    }

    fn move_selection(&mut self, delta: i32, window: &mut Window, cx: &mut Context<Self>) {
        let filtered = self.filtered_entries();
        if filtered.is_empty() {
            return;
        }

        let current_filtered_ix = self
            .selected_item_ix
            .and_then(|selected| {
                filtered
                    .iter()
                    .position(|(original_ix, _)| *original_ix == selected)
            })
            .unwrap_or(0);

        let new_filtered_ix = if delta < 0 {
            current_filtered_ix.saturating_sub((-delta) as usize)
        } else {
            (current_filtered_ix + delta as usize).min(filtered.len() - 1)
        };

        if let Some(&(original_ix, _)) = filtered.get(new_filtered_ix) {
            self.selected_item_ix = Some(original_ix);
            self.skip_next_scroll = true;
            self.list_scroll_handle
                .scroll_to_item(new_filtered_ix, ScrollStrategy::Center);

            self.update_editor_with_range_for_entry(
                original_ix,
                window,
                cx,
                |editor, mut range, _, window, cx| {
                    mem::swap(&mut range.start, &mut range.end);
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::newest()),
                        window,
                        cx,
                        |selections| {
                            selections.select_ranges([range]);
                        },
                    );
                },
            );

            cx.notify();
        }
    }
}

impl Render for HighlightsTreeView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let filtered_count = self.filtered_entries().len();

        div()
            .flex_1()
            .track_focus(&self.focus_handle)
            .key_context("HighlightsTreeView")
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .bg(cx.theme().colors().editor_background)
            .map(|this| {
                if filtered_count > 0 {
                    this.child(
                        uniform_list(
                            "HighlightsTreeView",
                            filtered_count,
                            cx.processor(move |this, range: Range<usize>, window, cx| {
                                this.compute_items(range, window, cx)
                            }),
                        )
                        .size_full()
                        .track_scroll(&self.list_scroll_handle)
                        .text_bg(cx.theme().colors().background)
                        .into_any_element(),
                    )
                    .vertical_scrollbar_for(&self.list_scroll_handle, window, cx)
                    .into_any_element()
                } else {
                    let inner_content = v_flex()
                        .items_center()
                        .text_center()
                        .gap_2()
                        .max_w_3_5()
                        .map(|this| {
                            if self.editor.is_some() {
                                let has_any = !self.cached_entries.is_empty();
                                if has_any {
                                    this.child(Label::new("All highlights are filtered out"))
                                        .child(
                                            Label::new(
                                                "Enable text or semantic highlights in the toolbar",
                                            )
                                            .size(LabelSize::Small),
                                        )
                                } else {
                                    this.child(Label::new("No highlights found")).child(
                                        Label::new(
                                            "The editor has no text or semantic token highlights",
                                        )
                                        .size(LabelSize::Small),
                                    )
                                }
                            } else {
                                this.child(Label::new("Not attached to an editor")).child(
                                    Label::new("Focus an editor to show highlights")
                                        .size(LabelSize::Small),
                                )
                            }
                        });

                    this.h_flex()
                        .size_full()
                        .justify_center()
                        .child(inner_content)
                        .into_any_element()
                }
            })
    }
}

impl EventEmitter<()> for HighlightsTreeView {}

impl Focusable for HighlightsTreeView {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for HighlightsTreeView {
    type Event = ();

    fn to_item_events(_: &Self::Event, _: impl FnMut(workspace::item::ItemEvent)) {}

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Highlights".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(Some(cx.new(|cx| {
            let mut clone = Self::new(self.workspace_handle.clone(), None, window, cx);
            clone.show_text_highlights = self.show_text_highlights;
            clone.show_semantic_tokens = self.show_semantic_tokens;
            clone.skip_next_scroll = false;
            if let Some(editor) = &self.editor {
                clone.set_editor(editor.editor.clone(), window, cx)
            }
            clone
        })))
    }

    fn on_removed(&self, cx: &mut Context<Self>) {
        if let Some(state) = self.editor.as_ref() {
            Self::clear_editor_highlights(&state.editor, cx);
        }
    }
}

impl Default for HighlightsTreeToolbarItemView {
    fn default() -> Self {
        Self::new()
    }
}

impl HighlightsTreeToolbarItemView {
    pub fn new() -> Self {
        Self {
            tree_view: None,
            _subscription: None,
            toggle_settings_handle: PopoverMenuHandle::default(),
        }
    }

    fn render_header(&self, cx: &Context<Self>) -> Option<ButtonLike> {
        let tree_view = self.tree_view.as_ref()?;
        let tree_view = tree_view.read(cx);

        let total = tree_view.cached_entries.len();
        let filtered = tree_view.filtered_entries().len();

        let label = if filtered == total {
            format!("{} highlights", total)
        } else {
            format!("{} / {} highlights", filtered, total)
        };

        Some(ButtonLike::new("highlights header").child(Label::new(label)))
    }

    fn render_update_button(&mut self, cx: &mut Context<Self>) -> Option<IconButton> {
        self.tree_view.as_ref().and_then(|view| {
            view.update(cx, |view, cx| {
                view.last_active_editor.as_ref().map(|editor| {
                    IconButton::new("highlights-view-update", IconName::RotateCw)
                        .tooltip({
                            let active_tab_name = editor.read_with(cx, |editor, cx| {
                                editor.tab_content_text(Default::default(), cx)
                            });

                            Tooltip::text(format!("Update view to '{active_tab_name}'"))
                        })
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.update_active_editor(&Default::default(), window, cx);
                        }))
                })
            })
        })
    }

    fn render_settings_button(&self, cx: &Context<Self>) -> PopoverMenu<ContextMenu> {
        let (show_text, show_semantic) = self
            .tree_view
            .as_ref()
            .map(|view| {
                let v = view.read(cx);
                (v.show_text_highlights, v.show_semantic_tokens)
            })
            .unwrap_or((true, true));

        let tree_view = self.tree_view.as_ref().map(|v| v.downgrade());

        PopoverMenu::new("highlights-tree-settings")
            .trigger_with_tooltip(
                IconButton::new("toggle-highlights-settings-icon", IconName::Sliders)
                    .icon_size(IconSize::Small)
                    .style(ButtonStyle::Subtle)
                    .toggle_state(self.toggle_settings_handle.is_deployed()),
                Tooltip::text("Highlights Settings"),
            )
            .anchor(Corner::TopRight)
            .with_handle(self.toggle_settings_handle.clone())
            .menu(move |window, cx| {
                let tree_view_for_text = tree_view.clone();
                let tree_view_for_semantic = tree_view.clone();

                let menu = ContextMenu::build(window, cx, move |menu, _, _| {
                    menu.toggleable_entry(
                        "Text Highlights",
                        show_text,
                        IconPosition::Start,
                        Some(ToggleTextHighlights.boxed_clone()),
                        {
                            let tree_view = tree_view_for_text.clone();
                            move |_, cx| {
                                if let Some(view) = tree_view.as_ref() {
                                    view.update(cx, |view, cx| {
                                        view.show_text_highlights = !view.show_text_highlights;
                                        cx.notify();
                                    })
                                    .ok();
                                }
                            }
                        },
                    )
                    .toggleable_entry(
                        "Semantic Tokens",
                        show_semantic,
                        IconPosition::Start,
                        Some(ToggleSemanticTokens.boxed_clone()),
                        {
                            move |_, cx| {
                                if let Some(view) = tree_view_for_semantic.as_ref() {
                                    view.update(cx, |view, cx| {
                                        view.show_semantic_tokens = !view.show_semantic_tokens;
                                        cx.notify();
                                    })
                                    .ok();
                                }
                            }
                        },
                    )
                });

                Some(menu)
            })
    }
}

impl Render for HighlightsTreeToolbarItemView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_1()
            .children(self.render_header(cx))
            .children(self.render_update_button(cx))
            .child(self.render_settings_button(cx))
    }
}

impl EventEmitter<ToolbarItemEvent> for HighlightsTreeToolbarItemView {}

impl ToolbarItemView for HighlightsTreeToolbarItemView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        if let Some(item) = active_pane_item
            && let Some(view) = item.downcast::<HighlightsTreeView>()
        {
            self.tree_view = Some(view.clone());
            self._subscription = Some(cx.observe_in(&view, window, |_, _, _, cx| cx.notify()));
            return ToolbarItemLocation::PrimaryLeft;
        }
        self.tree_view = None;
        self._subscription = None;
        ToolbarItemLocation::Hidden
    }
}

fn format_anchor_range(
    range: &Range<Anchor>,
    snapshot: &MultiBufferSnapshot,
) -> (SharedString, (u32, u32, u32, u32)) {
    let start = range.start.to_point(snapshot);
    let end = range.end.to_point(snapshot);
    let display = SharedString::from(format!(
        "[{}:{} - {}:{}]",
        start.row + 1,
        start.column + 1,
        end.row + 1,
        end.column + 1,
    ));
    let sort_key = (start.row, start.column, end.row, end.column);
    (display, sort_key)
}

fn render_style_preview(style: HighlightStyle, cx: &App) -> Div {
    let colors = cx.theme().colors();

    let display_color = style.color.or(style.background_color);

    let mut preview = div().px_1().rounded_sm();

    if let Some(color) = display_color {
        preview = preview.bg(color);
    } else {
        preview = preview.bg(colors.element_background);
    }

    let mut parts = Vec::new();

    if let Some(color) = display_color {
        parts.push(format_hsla_as_hex(color));
    }
    if style.font_weight.is_some() {
        parts.push("bold".to_string());
    }
    if style.font_style.is_some() {
        parts.push("italic".to_string());
    }
    if style.strikethrough.is_some() {
        parts.push("strike".to_string());
    }
    if style.underline.is_some() {
        parts.push("underline".to_string());
    }

    let label_text = if parts.is_empty() {
        "none".to_string()
    } else {
        parts.join(" ")
    };

    preview.child(Label::new(label_text).size(LabelSize::XSmall))
}

fn format_hsla_as_hex(color: Hsla) -> String {
    let rgba = color.to_rgb();
    let r = (rgba.r * 255.0).round() as u8;
    let g = (rgba.g * 255.0).round() as u8;
    let b = (rgba.b * 255.0).round() as u8;
    let a = (rgba.a * 255.0).round() as u8;
    if a == 255 {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    } else {
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }
}
