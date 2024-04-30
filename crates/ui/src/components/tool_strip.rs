use crate::{prelude::*, IconButtonShape, LabelLike};
use gpui::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ToolStripStyle {
    /// A flat style meant to blend in to the element
    /// it is placed on.
    Inline,
    /// A elevated style with a shadow, meant to appear
    /// as if it is floating above the element it is attached to.
    Popover,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ToolStripLabelStyle {
    AlwaysVisible,
    // VisibleOnHover,
    Hidden,
}

pub struct ToolStripItem {
    pub id: ElementId,
    pub icon: IconName,
    pub label: SharedString,
    pub keybinding: Option<KeyBinding>,
    pub on_click: Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>,
}

#[derive(IntoElement)]
pub struct ToolStrip {
    id: ElementId,
    tools: Vec<Vec<ToolStripItem>>,
    show_labels: ToolStripLabelStyle,
    axis: Axis,
    style: ToolStripStyle,
    // anchor_position: Option<Point<Pixels>>,
}

impl ToolStrip {
    pub fn inline(id: impl Into<ElementId>, tools: Vec<Vec<ToolStripItem>>) -> Self {
        Self {
            id: id.into(),
            tools,
            show_labels: ToolStripLabelStyle::Hidden,
            axis: Axis::Horizontal,
            style: ToolStripStyle::Inline,
            // anchor_position: None,
        }
    }

    pub fn popover(
        id: impl Into<ElementId>,
        tools: Vec<Vec<ToolStripItem>>,
        // anchor_position: Point<Pixels>,
    ) -> Self {
        Self {
            id: id.into(),
            tools,
            show_labels: ToolStripLabelStyle::AlwaysVisible,
            axis: Axis::Horizontal,
            style: ToolStripStyle::Popover,
            // anchor_position: None,
        }
    }

    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    pub fn style(mut self, style: ToolStripStyle) -> Self {
        self.style = style;
        self
    }

    fn render_label(label: &SharedString, keybinding: &Option<KeyBinding>) -> impl IntoElement {
        LabelLike::new()
            .line_height_style(LineHeightStyle::UiLabel)
            .child(
                h_flex()
                    .gap_1()
                    .child(label.clone())
                    .when_some(keybinding.clone(), |this, keybinding| {
                        this.child(crate::KeyBinding::new(keybinding.clone()))
                    }),
            )
    }
}

impl RenderOnce for ToolStrip {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(self.id.clone())
            .when_else(
                self.axis == Axis::Horizontal,
                |axis_horizontal| axis_horizontal.h_flex(),
                |axis_vertical| axis_vertical.v_flex(),
            )
            .flex_none()
            .when(self.style == ToolStripStyle::Popover, |this| {
                this.elevation_2(cx)
            })
            .group("tool_strip")
            .gap_2()
            .children(self.tools.into_iter().map(|section| {
                div()
                    .when_else(
                        self.axis == Axis::Horizontal,
                        |axis_horizontal| axis_horizontal.h_flex(),
                        |axis_vertical| axis_vertical.v_flex(),
                    )
                    .flex_none()
                    .gap_1p5()
                    .p_px()
                    .children(section.into_iter().map(|item| {
                        div()
                            .relative()
                            .flex_none()
                            .size(px(20.))
                            .child(
                                IconButton::new(item.id.clone(), item.icon)
                                    .shape(IconButtonShape::Square)
                                    .size(ButtonSize::Compact)
                                    .icon_size(IconSize::XSmall)
                                    .on_click(item.on_click),
                            )
                            .when(
                                self.show_labels == ToolStripLabelStyle::AlwaysVisible,
                                |this| {
                                    this.child(
                                        div().absolute().top_0().neg_right_3().child(
                                            Self::render_label(&item.label, &item.keybinding),
                                        ),
                                    )
                                },
                            )
                    }))
            }))
    }
}
