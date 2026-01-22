use crate::{
    Editor, EditorEvent,
    actions::{MoveDown, MoveUp},
    code_context_menus::{CodeContextMenu, ContextMenuOrigin},
};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AnyElement, App, Context, Entity, Focusable, InteractiveElement, ListSizingBehavior,
    MouseDownEvent, ParentElement, Pixels, ScrollStrategy, Size, Styled, Subscription, Task,
    UniformListScrollHandle, WeakEntity, Window, div, px, uniform_list,
};
use language::Buffer;
use multi_buffer::Anchor;
use settings::Settings;
use std::ops::Range;
use std::rc::Rc;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use task::TaskContext;
use theme::ThemeSettings;
use ui::{Popover, prelude::*, utils::WithRemSize};

pub struct FuzzyPopover<T: Clone> {
    pub items: Vec<T>,
    pub buffer: Entity<Buffer>,
    pub origin: Option<ContextMenuOrigin>,
    pub context: TaskContext,
    pub task_position: Option<Anchor>,
    search_editor: Entity<Editor>,
    filtered_items: Option<Vec<T>>,
    filter_matches: Option<Vec<StringMatch>>,
    pub selected_item: usize,
    pub scroll_handle: UniformListScrollHandle,
    last_query: String,
    filter_task: Task<()>,
    cancel_filter: Arc<AtomicBool>,
    parent_editor: WeakEntity<Editor>,
    get_label: Rc<dyn Fn(&T) -> String>,
    render_item: Rc<dyn Fn(&T, Vec<usize>, bool, &Context<Editor>) -> AnyElement>,
    on_confirm: Rc<dyn Fn(&T, usize, &mut Editor, &mut Window, &mut Context<Editor>)>,
    _editor_subscription: Subscription,
}

impl<T: Clone + 'static> FuzzyPopover<T> {
    pub fn new(
        items: Vec<T>,
        buffer: Entity<Buffer>,
        origin: Option<ContextMenuOrigin>,
        context: TaskContext,
        task_position: Option<Anchor>,
        scroll_handle: UniformListScrollHandle,
        get_label: impl Fn(&T) -> String + 'static,
        render_item: impl Fn(&T, Vec<usize>, bool, &Context<Editor>) -> AnyElement + 'static,
        on_confirm: impl Fn(&T, usize, &mut Editor, &mut Window, &mut Context<Editor>) + 'static,
        _parent_editor: WeakEntity<Editor>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Self {
        let search_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search actions…", window, cx);
            editor
        });

        let _editor_subscription =
            cx.subscribe(&search_editor, move |_this, _editor, event, cx| {
                if let EditorEvent::BufferEdited = event {
                    cx.notify();
                }
            });

        Self {
            items,
            origin,
            buffer,
            context,
            task_position,
            search_editor,
            filtered_items: None,
            filter_matches: None,
            selected_item: 0,
            scroll_handle,
            last_query: String::new(),
            filter_task: Task::ready(()),
            cancel_filter: Arc::new(AtomicBool::new(false)),
            parent_editor: _parent_editor,
            get_label: Rc::new(get_label),
            render_item: Rc::new(render_item),
            on_confirm: Rc::new(on_confirm),
            _editor_subscription,
        }
    }

    fn set_filter_results(&mut self, filtered: Vec<T>, matches: Vec<StringMatch>) {
        self.filtered_items = Some(filtered);
        self.filter_matches = Some(matches);
        self.selected_item = 0;
    }

    fn update_filtered_items(&mut self, window: &mut Window, cx: &mut Context<Editor>) {
        let query = self.search_editor.read(cx).text(cx);

        if query == self.last_query {
            return;
        }
        self.last_query = query.clone();

        if query.is_empty() {
            self.filtered_items = None;
            self.filter_matches = None;
            self.selected_item = 0;
            return;
        }

        self.cancel_filter.store(true, Ordering::Relaxed);
        self.cancel_filter = Arc::new(AtomicBool::new(false));

        let get_label = self.get_label.clone();
        let items = self.items.clone();
        let cancellation_flag = self.cancel_filter.clone();
        let background = cx.background_executor().clone();
        let parent_editor = self.parent_editor.clone();

        self.filter_task = cx.spawn_in(window, async move |_editor, cx| {
            let candidates: Vec<_> = items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let label = get_label(item);
                    StringMatchCandidate::new(i, label.as_str())
                })
                .collect();

            let case_sensitive = query.chars().any(|c| c.is_uppercase());
            let matches_task = fuzzy::match_strings(
                &candidates,
                &query,
                case_sensitive,
                false,
                100,
                &cancellation_flag,
                background,
            );

            let matches = matches_task.await;

            parent_editor
                .update(cx, |editor, cx| {
                    let mut context_menu = editor.context_menu.borrow_mut();
                    let Some(menu) = context_menu.as_mut() else {
                        return;
                    };

                    let CodeContextMenu::CodeActions(popover) = menu else {
                        return;
                    };

                    let mut filtered = Vec::new();
                    for mat in &matches {
                        filtered.push(popover.items[mat.candidate_id].clone());
                    }

                    popover.set_filter_results(filtered, matches);
                    drop(context_menu);

                    cx.notify();
                })
                .ok();
        });
    }

    pub fn visible_len(&self) -> usize {
        self.filtered_items
            .as_ref()
            .map_or_else(|| self.items.len(), |filtered| filtered.len())
    }

    pub fn get_item(&self, index: usize) -> Option<T> {
        if let Some(filtered) = &self.filtered_items {
            filtered.get(index).cloned()
        } else {
            self.items.get(index).cloned()
        }
    }

    pub fn visible(&self) -> bool {
        true
    }

    pub(crate) fn select_first(&mut self, cx: &mut Context<Editor>) {
        let len = self.visible_len();
        if len == 0 {
            return;
        }
        self.selected_item = if self.scroll_handle.y_flipped() {
            len - 1
        } else {
            0
        };
        self.scroll_handle
            .scroll_to_item(self.selected_item, ScrollStrategy::Top);
        cx.notify();
    }

    pub(crate) fn select_last(&mut self, cx: &mut Context<Editor>) {
        let len = self.visible_len();
        if len == 0 {
            return;
        }
        self.selected_item = if self.scroll_handle.y_flipped() {
            0
        } else {
            len - 1
        };
        self.scroll_handle
            .scroll_to_item(self.selected_item, ScrollStrategy::Top);
        cx.notify();
    }

    pub(crate) fn select_prev(&mut self, cx: &mut Context<Editor>) {
        let len = self.visible_len();
        if len == 0 {
            return;
        }
        let current = self.selected_item;
        self.selected_item = if self.scroll_handle.y_flipped() {
            if current + 1 < len { current + 1 } else { 0 }
        } else {
            if current > 0 { current - 1 } else { len - 1 }
        };

        self.scroll_handle
            .scroll_to_item(self.selected_item, ScrollStrategy::Top);
        cx.notify();
    }

    pub(crate) fn select_next(&mut self, cx: &mut Context<Editor>) {
        let len = self.visible_len();
        if len == 0 {
            return;
        }
        let current = self.selected_item;
        self.selected_item = if self.scroll_handle.y_flipped() {
            if current > 0 { current - 1 } else { len - 1 }
        } else {
            if current + 1 < len { current + 1 } else { 0 }
        };

        self.scroll_handle
            .scroll_to_item(self.selected_item, ScrollStrategy::Top);
        cx.notify();
    }

    pub fn origin(&self) -> ContextMenuOrigin {
        self.origin.unwrap_or(ContextMenuOrigin::Cursor)
    }

    pub fn focus(&self, window: &mut Window, cx: &mut Context<Editor>) {
        self.search_editor.update(cx, |editor, editor_cx| {
            editor.focus_handle(editor_cx).focus(window, editor_cx);
        });
    }

    pub fn focused(&self, window: &Window, cx: &App) -> bool {
        let focus_handle = self.search_editor.read(cx).focus_handle(cx);
        focus_handle.is_focused(window) || focus_handle.contains_focused(window, cx)
    }

    pub fn render(
        &mut self,
        max_height_in_lines: u32,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> AnyElement {
        self.update_filtered_items(window, cx);

        let selected_item = self.selected_item;
        let items_to_render = if let Some(filtered) = &self.filtered_items {
            filtered.clone()
        } else {
            self.items.clone()
        };

        let items_for_width = items_to_render.clone();
        let filter_matches = self.filter_matches.clone();
        let render_item = self.render_item.clone();
        let on_confirm_outer = self.on_confirm.clone();

        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let max_height = max_height_in_lines as f32 * ui_font_size;

        let list = uniform_list(
            "fuzzy_popover",
            items_to_render.len(),
            cx.processor(move |_this, range: Range<usize>, _, cx| {
                let on_confirm = on_confirm_outer.clone();
                items_to_render
                    .iter()
                    .skip(range.start)
                    .take(range.end - range.start)
                    .enumerate()
                    .map(|(ix, item)| {
                        let item_ix = range.start + ix;
                        let selected = item_ix == selected_item;
                        let match_positions = filter_matches
                            .as_ref()
                            .and_then(|matches| matches.get(item_ix))
                            .map(|m| m.positions.clone())
                            .unwrap_or_default();

                        let rendered = render_item(item, match_positions, selected, cx);
                        let item_clone = item.clone();
                        let on_confirm_inner = on_confirm.clone();

                        div()
                            .min_w(px(220.))
                            .max_w(px(540.))
                            .child(rendered)
                            .on_mouse_down(
                                gpui::MouseButton::Left,
                                cx.listener(move |editor, _, window, cx| {
                                    cx.stop_propagation();
                                    on_confirm_inner(&item_clone, item_ix, editor, window, cx);
                                }),
                            )
                    })
                    .collect()
            }),
        )
        .occlude()
        .max_h(max_height)
        .with_width_from_item(
            items_for_width
                .iter()
                .enumerate()
                .max_by_key(|(_, item)| (self.get_label)(item).chars().count())
                .map(|(ix, _)| ix),
        )
        .track_scroll(&self.scroll_handle)
        .with_sizing_behavior(ListSizingBehavior::Infer);

        Popover::new()
            .child(
                WithRemSize::new(ui_font_size)
                    .min_w_40()
                    .child(
                        v_flex()
                            .on_mouse_down_out(cx.listener(
                                |editor, _: &MouseDownEvent, window, cx| {
                                    editor.hide_context_menu(window, cx);
                                },
                            ))
                            .on_action(cx.listener(|editor, _: &menu::Cancel, window, cx| {
                                editor.hide_context_menu(window, cx);
                            }))
                            .on_action(cx.listener(|editor, _: &MoveUp, _window, cx| {
                                if let Some(menu) = editor.context_menu.borrow_mut().as_mut() {
                                    if let CodeContextMenu::CodeActions(popover) = menu {
                                        popover.select_prev(cx);
                                    }
                                }
                            }))
                            .on_action(cx.listener(|editor, _: &MoveDown, _window, cx| {
                                if let Some(menu) = editor.context_menu.borrow_mut().as_mut() {
                                    if let CodeContextMenu::CodeActions(popover) = menu {
                                        popover.select_next(cx);
                                    }
                                }
                            }))
                            .gap_1()
                            .child(
                                h_flex()
                                    .pb_1()
                                    .px_2p5()
                                    .border_b_1()
                                    .border_color(cx.theme().colors().border_variant)
                                    .flex_none()
                                    .overflow_hidden()
                                    .child(self.search_editor.clone()),
                            )
                            .when(self.visible_len() > 0, |this| {
                                this.child(list)
                            })
                            .when(self.visible_len() == 0, |this| {
                                this.child(
                                    h_flex().p_2().child(
                                        Label::new("No matches")
                                            .color(Color::Muted)
                                            .size(LabelSize::Small),
                                    ),
                                )
                            }),
                    ),
            )
            .into_any_element()
    }

    pub fn render_aside(
        &mut self,
        max_size: Size<Pixels>,
        window: &mut Window,
        _cx: &mut Context<Editor>,
    ) -> Option<AnyElement> {
        let item = self.get_item(self.selected_item)?;
        let label = (self.get_label)(&item);

        let text_system = window.text_system();
        let mut line_wrapper = text_system.line_wrapper(
            window.text_style().font(),
            window.text_style().font_size.to_pixels(window.rem_size()),
        );
        let is_truncated = line_wrapper.should_truncate_line(
            &label,
            px(540.0), // CODE_ACTION_MENU_MAX_WIDTH
            "…",
            gpui::TruncateFrom::End,
        );

        if is_truncated.is_none() {
            return None;
        }

        Some(
            Popover::new()
                .child(
                    div()
                        .child(label)
                        .id("fuzzy_popover_extended")
                        .px(px(8.0)) // MENU_ASIDE_X_PADDING / 2
                        .max_w(max_size.width)
                        .max_h(max_size.height)
                        .occlude(),
                )
                .into_any_element(),
        )
    }
}
