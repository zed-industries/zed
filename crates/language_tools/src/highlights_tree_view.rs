use editor::{
    Anchor, Editor, ExcerptId, HighlightKey, MultiBufferSnapshot, SelectionEffects, ToPoint,
    scroll::Autoscroll,
};
use gpui::{
    Action, App, AppContext as _, Context, Corner, Div, Entity, EntityId, EventEmitter,
    FocusHandle, Focusable, HighlightStyle, Hsla, InteractiveElement, IntoElement, MouseButton,
    MouseDownEvent, MouseMoveEvent, ParentElement, Render, ScrollStrategy, SharedString, Styled,
    Task, UniformListScrollHandle, WeakEntity, Window, actions, div, rems, uniform_list,
};
use menu::{SelectNext, SelectPrevious};
use std::{mem, ops::Range};
use theme::ActiveTheme;
use ui::{
    ButtonCommon, ButtonLike, ButtonStyle, Color, ContextMenu, FluentBuilder as _, IconButton,
    IconName, IconPosition, IconSize, Label, LabelCommon, LabelSize, PopoverMenu,
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
        /// Toggles showing text highlights.
        ToggleTextHighlights,
        /// Toggles showing semantic token highlights.
        ToggleSemanticTokens,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(move |workspace: &mut Workspace, _, _| {
        workspace.register_action(move |workspace, _: &OpenHighlightsTreeView, window, cx| {
            let active_item = workspace.active_item(cx);
            let workspace_handle = workspace.weak_handle();
            let highlights_tree_view =
                cx.new(|cx| HighlightsTreeView::new(workspace_handle, active_item, window, cx));
            workspace.split_item(
                SplitDirection::Right,
                Box::new(highlights_tree_view),
                window,
                cx,
            )
        });
    })
    .detach();
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HighlightCategory {
    Text(HighlightKey),
    SemanticToken {
        token_type: Option<SharedString>,
        token_modifiers: Option<SharedString>,
    },
}

impl HighlightCategory {
    fn label(&self) -> SharedString {
        match self {
            HighlightCategory::Text(key) => format!("text: {key:?}").into(),
            HighlightCategory::SemanticToken {
                token_type: Some(token_type),
                token_modifiers: Some(modifiers),
            } => format!("semantic token: {token_type} [{modifiers}]").into(),
            HighlightCategory::SemanticToken {
                token_type: Some(token_type),
                token_modifiers: None,
            } => format!("semantic token: {token_type}").into(),
            HighlightCategory::SemanticToken {
                token_type: None,
                token_modifiers: Some(modifiers),
            } => format!("semantic token [{modifiers}]").into(),
            HighlightCategory::SemanticToken {
                token_type: None,
                token_modifiers: None,
            } => "semantic token".into(),
        }
    }
}

#[derive(Debug, Clone)]
struct HighlightEntry {
    excerpt_id: ExcerptId,
    range: Range<Anchor>,
    range_display: SharedString,
    style: HighlightStyle,
    category: HighlightCategory,
    sort_key: (ExcerptId, u32, u32, u32, u32),
}

/// An item in the display list: either a separator between excerpts or a highlight entry.
#[derive(Debug, Clone)]
enum DisplayItem {
    ExcerptSeparator {
        label: SharedString,
    },
    Entry {
        /// Index into `cached_entries`.
        entry_ix: usize,
    },
}

pub struct HighlightsTreeView {
    workspace_handle: WeakEntity<Workspace>,
    editor: Option<EditorState>,
    list_scroll_handle: UniformListScrollHandle,
    selected_item_ix: Option<usize>,
    hovered_item_ix: Option<usize>,
    focus_handle: FocusHandle,
    cached_entries: Vec<HighlightEntry>,
    display_items: Vec<DisplayItem>,
    is_singleton: bool,
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
            hovered_item_ix: None,
            selected_item_ix: None,
            focus_handle: cx.focus_handle(),
            cached_entries: Vec::new(),
            display_items: Vec::new(),
            is_singleton: true,
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

        let is_different_editor = self
            .editor
            .as_ref()
            .is_none_or(|state| state.editor != editor);
        if is_different_editor {
            self.set_editor(editor, window, cx);
        }
    }

    fn handle_item_removed(
        &mut self,
        item_id: &EntityId,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self
            .editor
            .as_ref()
            .is_some_and(|state| state.editor.entity_id() == *item_id)
        {
            self.editor = None;
            self.cached_entries.clear();
            self.display_items.clear();
            cx.notify();
        }
    }

    fn set_editor(&mut self, editor: Entity<Editor>, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(state) = &self.editor {
            if state.editor == editor {
                return;
            }
            let key = HighlightKey::HighlightsTreeView(editor.entity_id().as_u64() as usize);
            editor.update(cx, |editor, cx| editor.clear_background_highlights(key, cx));
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
            self.display_items.clear();
            cx.notify();
            return;
        };

        let (display_map, project, multi_buffer, cursor_position) = {
            let editor = editor_state.editor.read(cx);
            let cursor = editor.selections.newest_anchor().head();
            (
                editor.display_map.clone(),
                editor.project().cloned(),
                editor.buffer().clone(),
                cursor,
            )
        };
        let Some(project) = project else {
            return;
        };

        let multi_buffer_snapshot = multi_buffer.read(cx).snapshot(cx);
        let is_singleton = multi_buffer_snapshot.is_singleton();
        self.is_singleton = is_singleton;

        let mut entries = Vec::new();

        display_map.update(cx, |display_map, cx| {
            for (key, text_highlights) in display_map.all_text_highlights() {
                for range in &text_highlights.1 {
                    let excerpt_id = range.start.excerpt_id;
                    let (range_display, sort_key) = format_anchor_range(
                        range,
                        excerpt_id,
                        &multi_buffer_snapshot,
                        is_singleton,
                    );
                    entries.push(HighlightEntry {
                        excerpt_id,
                        range: range.clone(),
                        range_display,
                        style: text_highlights.0,
                        category: HighlightCategory::Text(*key),
                        sort_key,
                    });
                }
            }

            project.read(cx).lsp_store().update(cx, |lsp_store, cx| {
                for (buffer_id, (tokens, interner)) in display_map.all_semantic_token_highlights() {
                    let language_name = multi_buffer
                        .read(cx)
                        .buffer(*buffer_id)
                        .and_then(|buf| buf.read(cx).language().map(|l| l.name()));
                    for token in tokens.iter() {
                        let range = token.range.start..token.range.end;
                        let excerpt_id = range.start.excerpt_id;
                        let (range_display, sort_key) = format_anchor_range(
                            &range,
                            excerpt_id,
                            &multi_buffer_snapshot,
                            is_singleton,
                        );
                        let Some(stylizer) = lsp_store.get_or_create_token_stylizer(
                            token.server_id,
                            language_name.as_ref(),
                            cx,
                        ) else {
                            continue;
                        };
                        entries.push(HighlightEntry {
                            excerpt_id,
                            range,
                            range_display,
                            style: interner[token.style],
                            category: HighlightCategory::SemanticToken {
                                token_type: stylizer.token_type_name(token.token_type).cloned(),
                                token_modifiers: stylizer
                                    .token_modifiers(token.token_modifiers)
                                    .map(SharedString::from),
                            },
                            sort_key,
                        });
                    }
                }
            });
        });

        entries.sort_by(|a, b| {
            a.sort_key
                .cmp(&b.sort_key)
                .then_with(|| a.category.cmp(&b.category))
        });
        entries.dedup_by(|a, b| a.sort_key == b.sort_key && a.category == b.category);

        self.cached_entries = entries;
        self.rebuild_display_items(&multi_buffer_snapshot, cx);

        if self.skip_next_scroll {
            self.skip_next_scroll = false;
        } else {
            self.scroll_to_cursor_position(&cursor_position, &multi_buffer_snapshot);
        }
        cx.notify();
    }

    fn rebuild_display_items(&mut self, snapshot: &MultiBufferSnapshot, cx: &App) {
        self.display_items.clear();

        let mut last_excerpt_id: Option<ExcerptId> = None;

        for (entry_ix, entry) in self.cached_entries.iter().enumerate() {
            if !self.should_show_entry(entry) {
                continue;
            }

            if !self.is_singleton {
                let excerpt_changed =
                    last_excerpt_id.is_none_or(|last_id| last_id != entry.excerpt_id);
                if excerpt_changed {
                    last_excerpt_id = Some(entry.excerpt_id);
                    let label = excerpt_label_for(entry.excerpt_id, snapshot, cx);
                    self.display_items
                        .push(DisplayItem::ExcerptSeparator { label });
                }
            }

            self.display_items.push(DisplayItem::Entry { entry_ix });
        }
    }

    fn should_show_entry(&self, entry: &HighlightEntry) -> bool {
        match entry.category {
            HighlightCategory::Text(_) => self.show_text_highlights,
            HighlightCategory::SemanticToken { .. } => self.show_semantic_tokens,
        }
    }

    fn scroll_to_cursor_position(&mut self, cursor: &Anchor, snapshot: &MultiBufferSnapshot) {
        let cursor_point = cursor.to_point(snapshot);
        let cursor_key = (cursor_point.row, cursor_point.column);
        let cursor_excerpt = cursor.excerpt_id;

        let best = self
            .display_items
            .iter()
            .enumerate()
            .filter_map(|(display_ix, item)| match item {
                DisplayItem::Entry { entry_ix } => {
                    let entry = &self.cached_entries[*entry_ix];
                    Some((display_ix, *entry_ix, entry))
                }
                _ => None,
            })
            .filter(|(_, _, entry)| {
                let (excerpt_id, start_row, start_col, end_row, end_col) = entry.sort_key;
                if !self.is_singleton && excerpt_id != cursor_excerpt {
                    return false;
                }
                let start = (start_row, start_col);
                let end = (end_row, end_col);
                cursor_key >= start && cursor_key <= end
            })
            .min_by_key(|(_, _, entry)| {
                let (_, start_row, start_col, end_row, end_col) = entry.sort_key;
                (end_row - start_row, end_col.saturating_sub(start_col))
            })
            .map(|(display_ix, entry_ix, _)| (display_ix, entry_ix));

        if let Some((display_ix, entry_ix)) = best {
            self.selected_item_ix = Some(entry_ix);
            self.list_scroll_handle
                .scroll_to_item(display_ix, ScrollStrategy::Center);
        }
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
        let style_preview = render_style_preview(entry.style, selected, cx);

        h_flex()
            .gap_1()
            .child(style_preview)
            .child(Label::new(entry.range_display.clone()).color(Color::Default))
            .child(
                Label::new(entry.category.label())
                    .size(LabelSize::Small)
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

    fn render_separator(&self, label: &SharedString, cx: &App) -> Div {
        let colors = cx.theme().colors();
        h_flex()
            .gap_1()
            .px(rems(0.5))
            .bg(colors.surface_background)
            .border_b_1()
            .border_color(colors.border_variant)
            .child(
                Label::new(label.clone())
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
    }

    fn compute_items(
        &mut self,
        visible_range: Range<usize>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Div> {
        let mut items = Vec::new();

        for display_ix in visible_range {
            let Some(display_item) = self.display_items.get(display_ix) else {
                continue;
            };

            match display_item {
                DisplayItem::ExcerptSeparator { label } => {
                    items.push(self.render_separator(label, cx));
                }
                DisplayItem::Entry { entry_ix } => {
                    let entry_ix = *entry_ix;
                    let entry = &self.cached_entries[entry_ix];
                    let selected = Some(entry_ix) == self.selected_item_ix;
                    let rendered = self
                        .render_entry(entry, selected, cx)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |tree_view, _: &MouseDownEvent, window, cx| {
                                tree_view.selected_item_ix = Some(entry_ix);
                                tree_view.skip_next_scroll = true;
                                tree_view.update_editor_with_range_for_entry(
                                    entry_ix,
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
                        .on_mouse_move(cx.listener(
                            move |tree_view, _: &MouseMoveEvent, window, cx| {
                                if tree_view.hovered_item_ix != Some(entry_ix) {
                                    tree_view.hovered_item_ix = Some(entry_ix);
                                    tree_view.update_editor_with_range_for_entry(
                                        entry_ix,
                                        window,
                                        cx,
                                        |editor, range, key, _, cx| {
                                            Self::set_editor_highlights(editor, key, &[range], cx);
                                        },
                                    );
                                    cx.notify();
                                }
                            },
                        ));

                    items.push(rendered);
                }
            }
        }

        items
    }

    fn set_editor_highlights(
        editor: &mut Editor,
        key: usize,
        ranges: &[Range<Anchor>],
        cx: &mut Context<Editor>,
    ) {
        editor.highlight_background_key(
            HighlightKey::HighlightsTreeView(key),
            ranges,
            |_, theme| theme.colors().editor_document_highlight_write_background,
            cx,
        );
    }

    fn clear_editor_highlights(editor: &Entity<Editor>, cx: &mut Context<Self>) {
        let highlight_key = HighlightKey::HighlightsTreeView(cx.entity_id().as_u64() as usize);
        editor.update(cx, |editor, cx| {
            editor.clear_background_highlights(highlight_key, cx);
        });
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        self.move_selection(-1, window, cx);
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        self.move_selection(1, window, cx);
    }

    fn move_selection(&mut self, delta: i32, window: &mut Window, cx: &mut Context<Self>) {
        if self.display_items.is_empty() {
            return;
        }

        let entry_display_items: Vec<(usize, usize)> = self
            .display_items
            .iter()
            .enumerate()
            .filter_map(|(display_ix, item)| match item {
                DisplayItem::Entry { entry_ix } => Some((display_ix, *entry_ix)),
                _ => None,
            })
            .collect();

        if entry_display_items.is_empty() {
            return;
        }

        let current_pos = self
            .selected_item_ix
            .and_then(|selected| {
                entry_display_items
                    .iter()
                    .position(|(_, entry_ix)| *entry_ix == selected)
            })
            .unwrap_or(0);

        let new_pos = if delta < 0 {
            current_pos.saturating_sub((-delta) as usize)
        } else {
            (current_pos + delta as usize).min(entry_display_items.len() - 1)
        };

        if let Some(&(display_ix, entry_ix)) = entry_display_items.get(new_pos) {
            self.selected_item_ix = Some(entry_ix);
            self.skip_next_scroll = true;
            self.list_scroll_handle
                .scroll_to_item(display_ix, ScrollStrategy::Center);

            self.update_editor_with_range_for_entry(
                entry_ix,
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

    fn entry_count(&self) -> usize {
        self.cached_entries
            .iter()
            .filter(|entry| self.should_show_entry(entry))
            .count()
    }
}

impl Render for HighlightsTreeView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let display_count = self.display_items.len();

        div()
            .flex_1()
            .track_focus(&self.focus_handle)
            .key_context("HighlightsTreeView")
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_next))
            .bg(cx.theme().colors().editor_background)
            .map(|this| {
                if display_count > 0 {
                    this.child(
                        uniform_list(
                            "HighlightsTreeView",
                            display_count,
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
        let filtered = tree_view.entry_count();

        let label = if filtered == total {
            format!("{} highlights", total)
        } else {
            format!("{} / {} highlights", filtered, total)
        };

        Some(ButtonLike::new("highlights header").child(Label::new(label)))
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
                                        let snapshot = view.editor.as_ref().map(|s| {
                                            s.editor.read(cx).buffer().read(cx).snapshot(cx)
                                        });
                                        if let Some(snapshot) = snapshot {
                                            view.rebuild_display_items(&snapshot, cx);
                                        }
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
                                        let snapshot = view.editor.as_ref().map(|s| {
                                            s.editor.read(cx).buffer().read(cx).snapshot(cx)
                                        });
                                        if let Some(snapshot) = snapshot {
                                            view.rebuild_display_items(&snapshot, cx);
                                        }
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

fn excerpt_label_for(
    excerpt_id: ExcerptId,
    snapshot: &MultiBufferSnapshot,
    cx: &App,
) -> SharedString {
    let buffer = snapshot.buffer_for_excerpt(excerpt_id);
    let path_label = buffer
        .and_then(|buf| buf.file())
        .map(|file| {
            let full_path = file.full_path(cx);
            full_path.to_string_lossy().to_string()
        })
        .unwrap_or_else(|| "untitled".to_string());
    path_label.into()
}

fn format_anchor_range(
    range: &Range<Anchor>,
    excerpt_id: ExcerptId,
    snapshot: &MultiBufferSnapshot,
    is_singleton: bool,
) -> (SharedString, (ExcerptId, u32, u32, u32, u32)) {
    if is_singleton {
        let start = range.start.to_point(snapshot);
        let end = range.end.to_point(snapshot);
        let display = SharedString::from(format!(
            "[{}:{} - {}:{}]",
            start.row + 1,
            start.column + 1,
            end.row + 1,
            end.column + 1,
        ));
        let sort_key = (excerpt_id, start.row, start.column, end.row, end.column);
        (display, sort_key)
    } else {
        let buffer = snapshot.buffer_for_excerpt(excerpt_id);
        if let Some(buffer) = buffer {
            let start = language::ToPoint::to_point(&range.start.text_anchor, buffer);
            let end = language::ToPoint::to_point(&range.end.text_anchor, buffer);
            let display = SharedString::from(format!(
                "[{}:{} - {}:{}]",
                start.row + 1,
                start.column + 1,
                end.row + 1,
                end.column + 1,
            ));
            let sort_key = (excerpt_id, start.row, start.column, end.row, end.column);
            (display, sort_key)
        } else {
            let start = range.start.to_point(snapshot);
            let end = range.end.to_point(snapshot);
            let display = SharedString::from(format!(
                "[{}:{} - {}:{}]",
                start.row + 1,
                start.column + 1,
                end.row + 1,
                end.column + 1,
            ));
            let sort_key = (excerpt_id, start.row, start.column, end.row, end.column);
            (display, sort_key)
        }
    }
}

fn render_style_preview(style: HighlightStyle, selected: bool, cx: &App) -> Div {
    let colors = cx.theme().colors();

    let display_color = style.color.or(style.background_color);

    let mut preview = div().px_1().rounded_sm();

    if let Some(color) = display_color {
        if selected {
            preview = preview.border_1().border_color(color).text_color(color);
        } else {
            preview = preview.bg(color);
        }
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

    preview.child(Label::new(label_text).size(LabelSize::Small).when_some(
        display_color.filter(|_| selected),
        |label, display_color| label.color(Color::Custom(display_color)),
    ))
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
