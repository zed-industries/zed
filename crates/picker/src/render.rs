use gpui::{AppContext, DragMoveEvent, MouseButton, canvas};
use settings::Settings;
use theme_settings::ThemeSettings;
use ui::{
    ActiveTheme, App, Color, Context, Disableable, DocumentationAside, DocumentationSide,
    FluentBuilder, InteractiveElement, IntoElement, Label, LabelCommon, ListItem, ListItemSpacing,
    ParentElement, Render, ScrollAxes, Scrollbars, StatefulInteractiveElement, Styled, StyledExt,
    Window, WithScrollbar, div, h_flex, px, rems_from_px, utils::WithRemSize, v_flex,
};

use crate::{
    ElementContainer, Picker, PickerDelegate, PickerEditorPosition, Preview,
    head::Head,
    preview::state::{LayoutMode, StackedLayout, TelescopeLayout},
    render::window_controls::{DragPreview, ResizeDrag, ResizeSide},
};

pub mod window_controls;

impl<D: PickerDelegate> Render for Picker<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match &self.preview {
            Some(
                preview @ Preview {
                    layout: LayoutMode::Stacked(stacked),
                    ..
                },
            ) => self
                .render_stacked_content(preview, *stacked, window, cx)
                .into_any_element(),
            Some(
                preview @ Preview {
                    layout: LayoutMode::Telescope(telescope),
                    ..
                },
            ) => self
                .render_telescope_content(preview, *telescope, window, cx)
                .into_any_element(),
            None => self.render2(window, cx).into_any_element(),
        }
    }
}

impl<D: PickerDelegate> Picker<D> {
    fn render2(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let window_size = window.viewport_size();
        let rem_size = window.rem_size();
        let is_wide_window = window_size.width / rem_size > rems_from_px(800.).0;

        let aside = self.delegate.documentation_aside(window, cx);

        let editor_position = self.delegate.editor_position();
        let picker_bounds = self.picker_bounds.clone();
        let menu = v_flex()
            .key_context("Picker")
            .size_full()
            .when_some(self.width, |el, width| el.w(width))
            .overflow_hidden()
            .child(
                canvas(
                    move |bounds, _window, _cx| {
                        picker_bounds.set(Some(bounds));
                    },
                    |_bounds, _state, _window, _cx| {},
                )
                .size_full()
                .absolute()
                .top_0()
                .left_0(),
            )
            // This is a bit of a hack to remove the modal styling when we're rendering the `Picker`
            // as a part of a modal rather than the entire modal.
            //
            // We should revisit how the `Picker` is styled to make it more composable.
            .when(self.is_modal, |this| this.elevation_3(cx))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::editor_move_down))
            .on_action(cx.listener(Self::editor_move_up))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::secondary_confirm))
            .on_action(cx.listener(Self::confirm_completion))
            .on_action(cx.listener(Self::confirm_input))
            .children(match &self.head {
                Head::Editor(editor) => {
                    if editor_position == PickerEditorPosition::Start {
                        Some(self.delegate.render_editor(&editor.clone(), window, cx))
                    } else {
                        None
                    }
                }
                Head::Empty(empty_head) => Some(div().child(empty_head.clone())),
            })
            .when(self.delegate.match_count() > 0, |el| {
                el.child(
                    v_flex()
                        .id("element-container")
                        .relative()
                        .flex_grow()
                        .when_some(self.max_height, |div, max_h| div.max_h(max_h))
                        .overflow_hidden()
                        .children(self.delegate.render_header(window, cx))
                        .child(self.render_element_container(cx))
                        .when(self.show_scrollbar, |this| {
                            let base_scrollbar_config =
                                Scrollbars::new(ScrollAxes::Vertical).width_sm();

                            this.map(|this| match &self.element_container {
                                ElementContainer::List(state) => this.custom_scrollbars(
                                    base_scrollbar_config.tracked_scroll_handle(state),
                                    window,
                                    cx,
                                ),
                                ElementContainer::UniformList(state) => this.custom_scrollbars(
                                    base_scrollbar_config.tracked_scroll_handle(state),
                                    window,
                                    cx,
                                ),
                            })
                        }),
                )
            })
            .when(self.delegate.match_count() == 0, |el| {
                el.when_some(self.delegate.no_matches_text(window, cx), |el, text| {
                    el.child(
                        v_flex().flex_grow().py_2().child(
                            ListItem::new("empty_state")
                                .inset(true)
                                .spacing(ListItemSpacing::Sparse)
                                .disabled(true)
                                .child(Label::new(text).color(Color::Muted)),
                        ),
                    )
                })
            })
            .children(self.delegate.render_footer(window, cx))
            .children(match &self.head {
                Head::Editor(editor) => {
                    if editor_position == PickerEditorPosition::End {
                        Some(self.delegate.render_editor(&editor.clone(), window, cx))
                    } else {
                        None
                    }
                }
                Head::Empty(empty_head) => Some(div().child(empty_head.clone())),
            });

        let Some(aside) = aside else {
            return menu;
        };

        let render_aside = |aside: DocumentationAside, cx: &mut Context<Self>| {
            WithRemSize::new(ui_font_size)
                .occlude()
                .elevation_2(cx)
                .w_full()
                .p_2()
                .overflow_hidden()
                .when(is_wide_window, |this| this.max_w_96())
                .when(!is_wide_window, |this| this.max_w_48())
                .child((aside.render)(cx))
        };

        if is_wide_window {
            let aside_index = self.delegate.documentation_aside_index();
            let picker_bounds = self.picker_bounds.get();
            let item_bounds =
                aside_index.and_then(|ix| self.item_bounds.borrow().get(&ix).copied());

            let item_position = match (picker_bounds, item_bounds) {
                (Some(picker_bounds), Some(item_bounds)) => {
                    let relative_top = item_bounds.origin.y - picker_bounds.origin.y;
                    let height = item_bounds.size.height;
                    Some((relative_top, height))
                }
                _ => None,
            };

            div()
                .relative()
                .child(menu)
                // Only render the aside once we have bounds to avoid flicker
                .when_some(item_position, |this, (top, height)| {
                    this.child(
                        h_flex()
                            .absolute()
                            .when(aside.side == DocumentationSide::Left, |el| {
                                el.right_full().mr_1()
                            })
                            .when(aside.side == DocumentationSide::Right, |el| {
                                el.left_full().ml_1()
                            })
                            .top(top)
                            .h(height)
                            .child(render_aside(aside, cx)),
                    )
                })
        } else {
            v_flex()
                .w_full()
                .gap_1()
                .justify_end()
                .child(render_aside(aside, cx))
                .child(menu)
        }
    }
}

impl<D: PickerDelegate> Picker<D> {
    fn render_stacked_content(
        &self,
        preview: &Preview,
        layout: StackedLayout,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .child(
                div()
                    .h(layout.results_height)
                    .overflow_hidden()
                    .child(self.render2(window, cx)),
            )
            .child(self.render_results_resize(window, cx))
            .child(preview.render(window, cx))
            .child(self.render_vertical_resize(ResizeSide::End, window, cx))
    }

    pub(crate) fn render_telescope_content(
        &self,
        preview: &Preview,
        layout: TelescopeLayout,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .relative()
            .child(
                h_flex()
                    .h(layout.content_height)
                    .child(
                        div()
                            .flex_1()
                            .h(layout.content_height)
                            .overflow_hidden()
                            .child(self.render2(window, cx)),
                    )
                    .child(self.render_telescope_preview_resize(window, cx))
                    .child(
                        div()
                            .w(layout.preview_width)
                            .h(layout.content_height)
                            .overflow_hidden()
                            .child(preview.render(window, cx)),
                    ),
            )
            .child(self.render_telescope_height_resize(ResizeSide::End, window, cx))
    }
}

/// This is to make resizable pickers
impl<D: PickerDelegate> Picker<D> {
    fn render_results_resize(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_highlighted = window.use_state(cx, |_window, _cx| false);
        let divider_size = px(window_controls::RESIZE_DIVIDER_SIZE);
        let handle_height = px(window_controls::RESIZE_HANDLE_HEIGHT);
        let handle_offset = (handle_height - divider_size) / 2.0;

        div()
            .id("resize-divider")
            .relative()
            .h(divider_size)
            .w_full()
            .bg(cx.theme().colors().border)
            .when(*is_highlighted.read(cx), |this| {
                this.bg(cx.theme().colors().border_focused)
            })
            .child(
                div()
                    .id("resize-handle")
                    .absolute()
                    .top(-handle_offset)
                    .left_0()
                    .right_0()
                    .h(handle_height)
                    .cursor_row_resize()
                    .block_mouse_except_scroll()
                    .on_hover(resize_hover_handler(is_highlighted.clone()))
                    .on_mouse_down(MouseButton::Left, handle_resize_mouse_down)
                    .on_drag(
                        ResizeDrag {
                            mouse_start_y: window.mouse_position().y,
                            results_height_start: self.picker_height,
                            preview_height_start: self
                                .preview
                                .and_then(|p| {
                                    if let LayoutMode::Stacked(layout) = p.layout {
                                        Some(layout.preview_height)
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or(self.picker_height),
                        },
                        highlighted_drag_preview(is_highlighted.clone()),
                    )
                    .on_drop::<ResizeDrag>(clear_resize_highlight(is_highlighted.clone())),
            )
            .on_drag_move::<ResizeDrag>(cx.listener(
                |this, event: &DragMoveEvent<ResizeDrag>, _window, cx| {
                    let drag = event.drag(cx);
                    let delta = event.event.position.y - drag.mouse_start_y;
                    let total_height = drag.results_height_start + drag.preview_height_start;

                    let new_results = (drag.results_height_start + delta)
                        .max(px(StackedLayout::MIN_PANEL_HEIGHT))
                        .min(total_height - px(StackedLayout::MIN_PANEL_HEIGHT));
                    let new_preview = total_height - new_results;

                    this.picker_height = new_results;
                    if let Some(Preview {
                        layout: LayoutMode::Stacked(StackedLayout { preview_height, .. }),
                        ..
                    }) = &mut this.preview
                    {
                        *preview_height = new_preview
                    }
                    cx.notify();
                },
            ))
    }
}

fn highlighted_drag_preview<T>(
    is_highlighted: gpui::Entity<bool>,
) -> impl Fn(&T, gpui::Point<ui::Pixels>, &mut Window, &mut App) -> gpui::Entity<DragPreview> {
    move |_, _, _, cx| {
        is_highlighted.write(cx, true);
        cx.new(|_| DragPreview)
    }
}

fn clear_resize_highlight<T>(
    is_highlighted: gpui::Entity<bool>,
) -> impl Fn(&T, &mut Window, &mut App) {
    move |_, _, cx| is_highlighted.write(cx, false)
}
