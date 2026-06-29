use gpui::{KeyContext, canvas};
use settings::Settings;
use theme_settings::ThemeSettings;
use ui::{
    ActiveTheme, Color, Context, Disableable, DocumentationAside, DocumentationSide, FluentBuilder,
    InteractiveElement, IntoElement, Label, LabelCommon, ListItem, ListItemSpacing, ParentElement,
    Render, ScrollAxes, Scrollbars, Styled, StyledExt, Window, WithScrollbar, div, h_flex,
    rems_from_px, utils::WithRemSize, v_flex,
};

use crate::shape::Shape;
use crate::{
    ElementContainer, Picker, PickerDelegate, PickerEditorPosition, Preview,
    head::Head,
    preview::Layout,
    render::window_controls::{Bottom, Left, LeftCorner, Middle, Right, RightCorner},
};
use crate::{persistence, preview};

pub mod window_controls;

impl<D: PickerDelegate> Render for Picker<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.finish_any_completed_resize(window, cx);

        let content = match &self.preview {
            Some(
                preview @ Preview {
                    layout: Layout::Below,
                    ..
                },
            ) => self
                .render_with_preview_below(preview, window, cx)
                .into_any_element(),
            Some(
                preview @ Preview {
                    layout: Layout::Right,
                    ..
                },
            ) => self
                .render_with_preview_right(preview, window, cx)
                .into_any_element(),
            Some(Preview {
                layout: Layout::Hidden,
                ..
            })
            | None => self.render_results(window, cx).into_any_element(),
        };

        // The border, background and rounding wrap the whole picker (results and
        // preview together). When there's a preview, clip the contents so its
        // corners follow the rounded border. Avoid clipping otherwise, so a
        // documentation aside (which is positioned outside the picker) isn't cut
        // off.
        let has_preview = self.preview.is_some();
        let content = div()
            .when(self.draws_own_container(), |this| this.elevation_3(cx))
            .when(has_preview, |this| this.overflow_hidden())
            .child(content);

        let layout = self.preview_layout().unwrap_or(Layout::Hidden);

        div()
            .relative()
            .child(content)
            .when(self.is_resizable(), |this| {
                this.left(self.shape.horizontal_offset(window))
                    .child(self.render_resize(Left, window, cx))
                    .child(self.render_resize(Right(layout), window, cx))
                    .child(self.render_resize(Bottom(layout), window, cx))
                    .child(self.render_resize(LeftCorner(layout), window, cx))
                    .child(self.render_resize(RightCorner(layout), window, cx))
            })
    }
}

impl<D: PickerDelegate> Picker<D> {
    pub(crate) fn render_results(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let window_size = window.viewport_size();
        let rem_size = window.rem_size();
        let is_wide_window = window_size.width / rem_size > rems_from_px(800.).0;

        let aside = self.delegate.documentation_aside(window, cx);

        let editor_position = self.delegate.editor_position();
        let picker_bounds = self.picker_bounds.clone();

        let mut key_context = KeyContext::default();
        key_context.add("Picker");
        if self.preview.is_some() {
            key_context.add("with_preview");
        }

        let menu = v_flex()
            .key_context(key_context)
            .relative()
            .map(|this| {
                self.shape.apply_results_size(
                    self.preview_layout(),
                    &self.size_bounds,
                    self.fill_height(),
                    this,
                    window,
                )
            })
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
            .on_action(cx.listener(Self::toggle_preview))
            .on_action(cx.listener(Self::set_preview_right))
            .on_action(cx.listener(Self::set_preview_below))
            .on_action(cx.listener(Self::set_preview_hidden))
            .on_action(cx.listener(Self::toggle_actions_menu))
            .children(match &self.head {
                Head::Editor(editor) => {
                    if editor_position == PickerEditorPosition::Start {
                        Some(h_flex().w_full().child(
                            div().flex_1().child(self.delegate.render_editor(
                                &editor.clone(),
                                window,
                                cx,
                            )),
                        ))
                    } else {
                        None
                    }
                }
                Head::Empty(empty_head) => Some(h_flex().child(empty_head.clone())),
            })
            .when(self.delegate.match_count() > 0, |el| {
                el.child(
                    v_flex()
                        .id("element-container")
                        .relative()
                        .flex_grow_1()
                        .min_h_0()
                        .when_some(
                            self.shape.results_max_height(
                                &self.size_bounds,
                                self.fill_height(),
                                window,
                            ),
                            |this, max_height| this.max_h(max_height),
                        )
                        .overflow_hidden()
                        .children(self.delegate.render_header(window, cx))
                        .child(self.render_element_container(cx))
                        .when(self.show_scrollbar, |this| {
                            let base_scrollbar_config = Scrollbars::new(ScrollAxes::Vertical);

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
                        v_flex().flex_grow_1().py_2().child(
                            ListItem::new("empty_state")
                                .inset(true)
                                .spacing(ListItemSpacing::Sparse)
                                .disabled(true)
                                .child(Label::new(text).color(Color::Muted)),
                        ),
                    )
                })
            })
            .children(self.render_footer(window, cx))
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

    fn render_with_preview_below(
        &self,
        preview: &Preview,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .relative()
            .child(
                v_flex()
                    .h(self
                        .shape
                        .height(Some(preview::Layout::Below), &self.size_bounds, window))
                    .child(
                        div()
                            .h(self.shape.results_height(
                                preview::Layout::Below,
                                &self.size_bounds,
                                window,
                            ))
                            .overflow_hidden()
                            .child(self.render_results(window, cx)),
                    )
                    .child(
                        div()
                            .h(self.shape.preview_height(
                                preview::Layout::Below,
                                &self.size_bounds,
                                window,
                            ))
                            .border_t_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(preview.render(cx)),
                    ),
            )
            .when(self.is_resizable(), |this| {
                this.child(self.render_resize(window_controls::Middle(preview.layout), window, cx))
            })
    }

    pub(crate) fn render_with_preview_right(
        &self,
        preview: &Preview,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .relative()
            .child(
                h_flex()
                    .h(self
                        .shape
                        .height(Some(Layout::Right), &self.size_bounds, window))
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .overflow_hidden()
                            .child(self.render_results(window, cx)),
                    )
                    .child(
                        div()
                            .w(self
                                .shape
                                .preview_width(Layout::Right, &self.size_bounds, window))
                            .map(|this| {
                                self.shape.apply_height(
                                    Some(Layout::Right),
                                    &self.size_bounds,
                                    this,
                                    window,
                                )
                            })
                            .border_l_1()
                            .border_color(cx.theme().colors().border_variant)
                            .overflow_hidden()
                            .child(preview.render(cx)),
                    ),
            )
            .when(self.is_resizable(), |this| {
                this.child(self.render_resize(Middle(Layout::Right), window, cx))
            })
    }

    fn finish_any_completed_resize(
        &mut self,
        window: &mut Window,
        cx: &mut Context<'_, Picker<D>>,
    ) {
        if let Shape::Resizing(pos) = self.shape
            && !cx.has_active_drag()
        {
            let centered = Shape::centered_and_relative(pos, self.preview_layout(), window);
            persistence::store_shape_for_this_layout(
                D::name(),
                self.preview_layout(),
                centered,
                window,
                cx,
            );
            self.shape = Shape::HorizontallyCentered(centered);
            if let Some(preview) = &mut self.preview {
                preview.adjust_to_new_size(window, cx);
            }
        }
    }
}
