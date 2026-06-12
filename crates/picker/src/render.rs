use gpui::{MouseButton, anchored, canvas};
use settings::Settings;
use theme_settings::ThemeSettings;
use ui::{
    Color, Context, Disableable, DocumentationAside, DocumentationSide, FluentBuilder,
    InteractiveElement, IntoElement, Label, LabelCommon, ListItem, ListItemSpacing, ParentElement,
    Render, ScrollAxes, Scrollbars, Styled, StyledExt, Window, WithScrollbar, div, h_flex,
    rems_from_px, utils::WithRemSize, v_flex,
};

use crate::{
    ElementContainer, Picker, PickerDelegate, PickerEditorPosition, Preview, Shape, ViewPortLength,
    head::Head,
    preview::PreviewLayout,
    render::window_controls::{Bottom, Left, LeftCorner, Middle, Right, RightCorner},
};

pub mod window_controls;

impl<D: PickerDelegate> Render for Picker<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Shape::Resizing(pos) = self.shape
            && !cx.has_active_drag()
        {
            self.shape = Shape::centered_and_relative(pos, window);
        }

        let content = match &self.preview {
            Some(
                preview @ Preview {
                    layout: PreviewLayout::Below,
                    ..
                },
            ) => self
                .render_with_preview_below(preview, window, cx)
                .into_any_element(),
            Some(
                preview @ Preview {
                    layout: PreviewLayout::Right,
                    ..
                },
            ) => self
                .render_with_preview_right(preview, window, cx)
                .into_any_element(),
            Some(Preview {
                layout: PreviewLayout::Hidden,
                ..
            })
            | None => self.render_results(window, cx).into_any_element(),
        };

        // Position relative to the window so shape fully controls placement
        anchored()
            .position(self.shape.origin(window))
            .snap_to_window_with_margin(ViewPortLength(0.05).as_pixels(window))
            .child(
                div()
                    // Below the picker there is a layer that dismisses the
                    // picker modal on click. Do not propegate clicks to that
                    // if the clicks are on the picker
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    .child(content),
            )
            .child(self.render_resize::<Left>(window, cx))
            .child(self.render_resize::<Right>(window, cx))
            .child(self.render_resize::<Bottom>(window, cx))
            .child(self.render_resize::<LeftCorner>(window, cx))
            .child(self.render_resize::<RightCorner>(window, cx))
    }
}

impl<D: PickerDelegate> Picker<D> {
    fn render_results(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let window_size = window.viewport_size();
        let rem_size = window.rem_size();
        let is_wide_window = window_size.width / rem_size > rems_from_px(800.).0;

        let aside = self.delegate.documentation_aside(window, cx);

        let editor_position = self.delegate.editor_position();
        let picker_bounds = self.picker_bounds.clone();
        let menu = v_flex()
            .key_context("Picker")
            .relative()
            .map(|this| self.shape.apply_picker_size(&self.preview, this, window))
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
            .on_action(cx.listener(Self::toggle_layout))
            .on_action(cx.listener(Self::to_multibuffer))
            .children(match &self.head {
                Head::Editor(editor) => {
                    if editor_position == PickerEditorPosition::Start {
                        Some(
                            h_flex()
                                .w_full()
                                .child(div().flex_1().child(self.delegate.render_editor(
                                    &editor.clone(),
                                    window,
                                    cx,
                                )))
                                .when_some(
                                    self.render_header_controls(window, cx),
                                    |this, controls| this.child(div().pr_2().child(controls)),
                                ),
                        )
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
                        .flex_grow()
                        .min_h_0()
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
    fn render_with_preview_below(
        &self,
        preview: &Preview,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        // TODO!(yara) minimize the number of flex/divs etc needed
        h_flex().relative().child(
            v_flex()
                .h(self.shape.height(window))
                .child(
                    div()
                        .h(self.shape.results_height(preview, window))
                        .overflow_hidden()
                        .child(self.render_results(window, cx)),
                )
                .child(self.render_resize::<Middle>(window, cx))
                .child(
                    div()
                        .h(self.shape.preview_height(preview, window))
                        .child(preview.render(cx)),
                ),
        )
    }

    pub(crate) fn render_with_preview_right(
        &self,
        preview: &Preview,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex().relative().child(
            h_flex()
                .h(self.shape.height(window))
                .child(
                    div()
                        .flex_1()
                        .h_full()
                        .overflow_hidden()
                        .child(self.render_results(window, cx)),
                )
                .child(self.render_resize::<Middle>(window, cx))
                .child(
                    div()
                        .w(self.shape.preview_width(preview, window))
                        .map(|this| self.shape.apply_height(this, window))
                        .overflow_hidden()
                        .child(preview.render(cx)),
                ),
        )
    }
}
