use gpui::{
    AnyElement, BorderStyle, ClickEvent, Corners, Edges, ElementId, Hsla, ParentElement, Role,
    canvas, quad,
};
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct RadioButton {
    id: ElementId,
    is_selected: bool,
    invalid: bool,
    aria_label: Option<SharedString>,
    tab_index: Option<isize>,
    children: SmallVec<[AnyElement; 2]>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl RadioButton {
    pub fn new(id: impl Into<ElementId>, is_selected: bool) -> Self {
        Self {
            id: id.into(),
            is_selected,
            invalid: false,
            aria_label: None,
            tab_index: None,
            children: SmallVec::new(),
            on_click: None,
        }
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn tab_index(mut self, tab_index: impl Into<isize>) -> Self {
        self.tab_index = Some(tab_index.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl ParentElement for RadioButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for RadioButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let border_color = if self.invalid {
            Color::Error.color(cx)
        } else {
            cx.theme().colors().border.opacity(0.8)
        };
        let background = cx.theme().colors().editor_background;
        let row_background = if self.is_selected {
            background.blend(Color::Accent.color(cx).opacity(0.08))
        } else {
            background
        };
        let hover_background = if self.is_selected {
            background.blend(Color::Accent.color(cx).opacity(0.1))
        } else {
            cx.theme()
                .colors()
                .element_background
                .blend(cx.theme().colors().editor_foreground.opacity(0.025))
        };
        let focused_border = cx.theme().colors().border_focused;
        let is_selected = self.is_selected;
        let on_click = self.on_click;

        h_flex()
            .id(self.id)
            .role(Role::RadioButton)
            .when_some(self.aria_label, |this, label| this.aria_label(label))
            .aria_selected(is_selected)
            .w_full()
            .min_h(rems_from_px(28_f32))
            .items_start()
            .gap_1p5()
            .rounded_sm()
            .border_1()
            .border_color(border_color.opacity(0.5))
            .bg(row_background)
            .px_2()
            .py_1()
            .hover(move |this| this.bg(hover_background).cursor_pointer())
            .when_some(self.tab_index, |this, tab_index| {
                this.tab_index(tab_index)
                    .focus_visible(|this| this.border_color(focused_border))
            })
            .when_some(on_click, |this, on_click| this.on_click(on_click))
            .child(
                div()
                    .size(px(20.))
                    .flex_none()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Self::render_indicator(
                        is_selected,
                        border_color,
                        background,
                    )),
            )
            .children(self.children)
    }
}

impl RadioButton {
    fn render_indicator(
        is_selected: bool,
        border_color: Hsla,
        background: Hsla,
    ) -> impl IntoElement {
        div().size_3().flex_none().child(
            canvas(
                |_, _, _| {},
                move |bounds, _, window, cx| {
                    let bounds = window.pixel_snap_bounds(bounds);
                    let radius = bounds.size.width.min(bounds.size.height) / 2.;
                    window.paint_quad(quad(
                        bounds,
                        Corners::all(radius),
                        background,
                        Edges::all(px(1.)),
                        border_color,
                        BorderStyle::Solid,
                    ));

                    if is_selected {
                        // Equal whole-device-pixel insets keep both circles concentric
                        // when their independently rounded diameters would have different parity.
                        let scale_factor = window.scale_factor();
                        let inset =
                            px((f32::from(radius) * scale_factor / 2.).round() / scale_factor);
                        let dot_bounds = bounds.dilate(-inset);
                        window.paint_quad(quad(
                            dot_bounds,
                            Corners::all(dot_bounds.size.width.min(dot_bounds.size.height) / 2.),
                            Color::Accent.color(cx),
                            Edges::default(),
                            Hsla::transparent_black(),
                            BorderStyle::Solid,
                        ));
                    }
                },
            )
            .size_full(),
        )
    }
}

impl Component for RadioButton {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn description() -> &'static str {
        "A radio button for single-selection controls."
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> AnyElement {
        example_group_with_title(
            "Radio Button States",
            vec![
                single_example(
                    "Unselected",
                    Self::new("radio-unselected", false)
                        .aria_label("Unselected option")
                        .tab_index(0isize)
                        .child(Label::new("Unselected option").size(LabelSize::Small))
                        .into_any_element(),
                ),
                single_example(
                    "Selected",
                    Self::new("radio-selected", true)
                        .aria_label("Selected option")
                        .tab_index(0isize)
                        .child(Label::new("Selected option").size(LabelSize::Small))
                        .into_any_element(),
                ),
                single_example(
                    "Invalid",
                    Self::new("radio-invalid", false)
                        .invalid(true)
                        .aria_label("Invalid option")
                        .tab_index(0isize)
                        .child(Label::new("Invalid option").size(LabelSize::Small))
                        .into_any_element(),
                ),
            ],
        )
        .into_any_element()
    }
}
