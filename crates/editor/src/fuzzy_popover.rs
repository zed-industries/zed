use crate::{Editor, code_context_menus::ContextMenuOrigin};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AnyElement, Context, Entity, ListSizingBehavior, Pixels, ScrollStrategy, Size,
    UniformListScrollHandle, Window, div, px, uniform_list,
};
use language::Buffer;
use multi_buffer::Anchor;
use std::ops::Range;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use task::TaskContext;
use ui::{Popover, h_flex, prelude::*};

pub struct FuzzyPopover<T: Clone> {
    pub items: Vec<T>,
    pub buffer: Entity<Buffer>,
    pub origin: Option<ContextMenuOrigin>,
    pub context: TaskContext,
    pub task_position: Option<Anchor>,
    filter_query: String,
    filtered_items: Option<Vec<T>>,
    filter_matches: Option<Vec<StringMatch>>,
    pub selected_item: usize,
    pub scroll_handle: UniformListScrollHandle,
    get_label: Rc<dyn Fn(&T) -> String>,
    render_item: Rc<dyn Fn(&T, Vec<usize>, bool, &Context<Editor>) -> AnyElement>,
    on_confirm: Rc<dyn Fn(&T, usize, &mut Editor, &mut Window, &mut Context<Editor>)>,
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
    ) -> Self {
        Self {
            items,
            origin,
            buffer,
            context,
            task_position,
            filter_query: String::new(),
            filtered_items: None,
            filter_matches: None,
            selected_item: 0,
            scroll_handle,
            get_label: Rc::new(get_label),
            render_item: Rc::new(render_item),
            on_confirm: Rc::new(on_confirm),
        }
    }

    pub fn filter(&mut self, query: &str, cx: &mut Context<Editor>) {
        self.filter_query.push_str(query);

        if self.filter_query.is_empty() {
            self.filtered_items = None;
            self.filter_matches = None;
        } else {
            let candidates: Vec<_> = self
                .items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let label = (self.get_label)(item);
                    StringMatchCandidate::new(i, label.as_str())
                })
                .collect();

            let cancellation_flag = AtomicBool::new(false);
            let matches_task = fuzzy::match_strings(
                &candidates,
                &self.filter_query,
                self.filter_query.chars().any(|c| c.is_uppercase()),
                false,
                100,
                &cancellation_flag,
                cx.background_executor().clone(),
            );

            let matches = smol::block_on(matches_task);
            let mut filtered = Vec::new();
            for mat in &matches {
                filtered.push(self.items[mat.candidate_id].clone());
            }

            self.filtered_items = Some(filtered);
            self.filter_matches = Some(matches);
        }

        self.selected_item = 0;
        cx.notify();
    }

    pub fn backspace_filter(&mut self, cx: &mut Context<Editor>) {
        if self.filter_query.pop().is_some() {
            if self.filter_query.is_empty() {
                self.filtered_items = None;
                self.filter_matches = None;
            } else {
                let query = self.filter_query.clone();
                self.filter_query.clear();
                self.filter(&query, cx);
                return;
            }
            self.selected_item = 0;
        }
        cx.notify();
    }

    pub fn clear_filter(&mut self, cx: &mut Context<Editor>) {
        self.filter_query.clear();
        self.filtered_items = None;
        self.filter_matches = None;
        self.selected_item = 0;
        cx.notify();
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
        self.visible_len() > 0
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

    pub fn render(
        &self,
        max_height_in_lines: u32,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> AnyElement {
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
        .max_h(max_height_in_lines as f32 * window.line_height())
        .track_scroll(&self.scroll_handle)
        .with_width_from_item(
            items_for_width
                .iter()
                .enumerate()
                .max_by_key(|(_, item)| (self.get_label)(item).chars().count())
                .map(|(ix, _)| ix),
        )
        .with_sizing_behavior(ListSizingBehavior::Infer);

        let children = if !self.filter_query.is_empty() {
            vec![
                div()
                    .id("fuzzy_popover_filter")
                    .px_2()
                    .py_1()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        h_flex()
                            .child(
                                ui::Icon::new(ui::IconName::MagnifyingGlass)
                                    .size(ui::IconSize::XSmall)
                                    .color(ui::Color::Muted),
                            )
                            .child(
                                div()
                                    .ml_2()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child(format!("Filter: {}", self.filter_query)),
                            ),
                    )
                    .into_any_element(),
                list.into_any_element(),
            ]
        } else {
            vec![list.into_any_element()]
        };

        Popover::new().children(children).into_any_element()
    }

    pub fn render_aside(
        &mut self,
        _max_size: Size<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Editor>,
    ) -> Option<AnyElement> {
        None
    }
}
