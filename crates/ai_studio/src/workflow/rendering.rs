use gpui::{Context, Point, px, FontWeight, IntoElement};
use ui::{prelude::*, Label, LabelSize, Button, ButtonStyle};
use std::collections::HashMap;

use crate::workflow::types::*;

pub struct CanvasRenderer;

impl CanvasRenderer {
    pub fn render_toolbar<T>(
        viewport: &CanvasViewport,
        selected_node: Option<NodeId>,
        nodes: &HashMap<NodeId, WorkflowNode>,
        is_running: bool,
        cx: &mut Context<T>,
    ) -> impl IntoElement 
    where
        T: 'static,
    {
        div()
            .flex()
            .items_center()
            .justify_between()
            .p_2()
            .bg(cx.theme().colors().toolbar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("add_input", "Input")
                            .style(ButtonStyle::Filled)
                    )
                    .child(
                        Button::new("add_llm", "LLM")
                            .style(ButtonStyle::Filled)
                    )
                    .child(
                        Button::new("add_output", "Output")
                            .style(ButtonStyle::Filled)
                    )
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("zoom_out", "−")
                            .style(ButtonStyle::Subtle)
                    )
                    .child(
                        Label::new(format!("{}%", (viewport.scale * 100.0) as i32))
                            .size(LabelSize::Small)
                    )
                    .child(
                        Button::new("zoom_in", "+")
                            .style(ButtonStyle::Subtle)
                    )
                    .child(
                        Button::new("reset_zoom", "Reset")
                            .style(ButtonStyle::Subtle)
                    )
                    .child(
                        Button::new("focus_node", "Focus")
                            .style(ButtonStyle::Subtle)
                            .disabled(selected_node.is_none())
                    )
                    .child(
                        Button::new("fit_node", "Fit")
                            .style(ButtonStyle::Subtle)
                            .disabled(selected_node.is_none())
                    )
                    .child(
                        Button::new("center_all", "Center All")
                            .style(ButtonStyle::Subtle)
                            .disabled(nodes.is_empty())
                    )
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("run", "Run")
                            .style(ButtonStyle::Filled)
                            .disabled(is_running)
                    )
                    .child(
                        Button::new("stop", "Stop")
                            .style(ButtonStyle::Subtle)
                            .disabled(!is_running)
                    )
            )
    }

    pub fn render_grid_background<T>(viewport: &CanvasViewport, cx: &mut Context<T>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let _grid_size = 20.0 * viewport.scale;
        let grid_color = theme.colors().border.opacity(0.1);
        
        div()
            .absolute()
            .size_full()
            .bg(theme.colors().editor_background)
            .child(
                div()
                    .size_full()
                    .bg(theme.colors().editor_background)
                    .child(
                        div()
                            .absolute()
                            .size_full()
                            .opacity(0.1)
                            .border_1()
                            .border_color(grid_color)
                    )
            )
    }

    pub fn render_canvas_content<T>(
        nodes: &HashMap<NodeId, WorkflowNode>,
        selected_node: Option<NodeId>,
        viewport: &CanvasViewport,
        cx: &mut Context<T>,
    ) -> impl IntoElement 
    where
        T: 'static,
    {
        let theme = cx.theme().clone();
        
        div()
            .size_full()
            .relative()
            .child(Self::render_grid_background(viewport, cx))
            .children(
                nodes.values().map(|node| {
                    // Transform canvas coordinates to screen coordinates
                    let screen_x = node.position.x * viewport.scale + viewport.offset.x;
                    let screen_y = node.position.y * viewport.scale + viewport.offset.y;
                    let screen_width = node.size.width * viewport.scale;
                    let screen_height = node.size.height * viewport.scale;
                    
                    let is_selected = selected_node == Some(node.id);
                    
                    div()
                        .absolute()
                        .left(px(screen_x))
                        .top(px(screen_y))
                        .w(px(screen_width))
                        .h(px(screen_height))
                        .bg(if is_selected {
                            theme.colors().element_selected
                        } else {
                            theme.colors().surface_background
                        })
                        .when(is_selected, |div| div.border_4())
                        .when(!is_selected, |div| div.border_2())
                        .border_color(if is_selected { 
                            theme.colors().text_accent
                        } else { 
                            theme.colors().border
                        })
                        .rounded_md()
                        .shadow_sm()
                        .cursor_pointer()
                        .hover({
                            let theme = theme.clone();
                            move |style| style.bg(theme.colors().element_hover)
                        })
                        .child(
                            div()
                                .p_2()
                                .child(
                                    Label::new(format!("{}\nCanvas: ({:.0},{:.0})\nScreen: ({:.0},{:.0})", 
                                        node.title, 
                                        node.position.x, node.position.y,
                                        screen_x, screen_y
                                    ))
                                    .size(LabelSize::Small)
                                    .weight(FontWeight::BOLD)
                                )
                        )
                })
            )
    }

    pub fn render_status_bar<T>(
        nodes: &HashMap<NodeId, WorkflowNode>,
        connections: &[NodeConnection],
        viewport: &CanvasViewport,
        interaction_state: &InteractionState,
        current_mouse_screen: Option<Point<f32>>,
        current_mouse_canvas: Option<Point<f32>>,
        trackpad_state: &crate::workflow::interaction::TrackpadState,
        cx: &mut Context<T>,
    ) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .p_2()
            .bg(cx.theme().colors().toolbar_background)
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(
                        Label::new(format!("Nodes: {}", nodes.len()))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
                    .child(
                        Label::new(format!("Connections: {}", connections.len()))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Label::new(format!("Scale: {:.2}", viewport.scale))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
                    .child(
                        Label::new(format!("Offset: ({:.0}, {:.0})", viewport.offset.x, viewport.offset.y))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
                    .child(
                        Label::new(
                            if let Some(screen_pos) = current_mouse_screen {
                                format!("Screen: ({:.0}, {:.0})", screen_pos.x, screen_pos.y)
                            } else {
                                "Screen: --".to_string()
                            }
                        )
                        .size(LabelSize::Small)
                        .color(Color::Accent)
                    )
                    .child(
                        Label::new(
                            if let Some(canvas_pos) = current_mouse_canvas {
                                format!("Canvas: ({:.0}, {:.0})", canvas_pos.x, canvas_pos.y)
                            } else {
                                "Canvas: --".to_string()
                            }
                        )
                        .size(LabelSize::Small)
                        .color(Color::Accent)
                    )
                    .child(
                        Label::new(
                            if trackpad_state.is_pinch_zooming {
                                "Pinch Zoom"
                            } else if trackpad_state.momentum_velocity.x.abs() > 0.1 || trackpad_state.momentum_velocity.y.abs() > 0.1 {
                                "Momentum"
                            } else {
                                "Trackpad"
                            }
                        )
                        .size(LabelSize::Small)
                        .color(if trackpad_state.is_pinch_zooming { Color::Success } else { Color::Muted })
                    )
            )
            .child(
                Label::new(match interaction_state {
                    InteractionState::None => "Ready",
                    InteractionState::NodeDrag { .. } => "Dragging",
                    InteractionState::CanvasPan { .. } => "Panning",
                })
                .size(LabelSize::Small)
                .color(Color::Accent)
            )
    }
} 