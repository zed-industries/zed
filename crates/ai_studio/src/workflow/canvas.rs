use gpui::{Context, Window, Point, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ScrollWheelEvent, Bounds, Pixels, FocusHandle, Focusable, EventEmitter, Render, IntoElement, Size, px};
use ui::{prelude::*, ActiveTheme};

use crate::workflow::types::*;
use crate::workflow::execution::WorkflowExecutor;
use crate::workflow::interaction::InteractionHandler;
use crate::workflow::rendering::CanvasRenderer;

/// Workflow canvas for creating and managing AI workflows
pub struct WorkflowCanvas {
    executor: WorkflowExecutor,
    interaction: InteractionHandler,
    focus_handle: FocusHandle,
    canvas_bounds: Option<Bounds<Pixels>>,
}

impl WorkflowCanvas {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut canvas = Self {
            executor: WorkflowExecutor::new(),
            interaction: InteractionHandler::new(),
            focus_handle: cx.focus_handle(),
            canvas_bounds: None,
        };
        
        // Set up a proper initial viewport with reasonable bounds
        canvas.interaction.viewport_manager.viewport.bounds = Bounds::new(
            Point::new(px(0.0), px(0.0)), 
            Size::new(px(1200.0), px(800.0))
        );
        
        // Start with no offset and scale 1.0 for predictable positioning
        canvas.interaction.viewport_manager.viewport.offset = Point::new(0.0, 0.0);
        canvas.interaction.viewport_manager.viewport.scale = 1.0;
        
        // Add nodes at simple, predictable positions
        canvas.add_node(NodeType::Input, Point::new(50.0, 50.0), cx);
        canvas.add_node(NodeType::LLMPrompt, Point::new(300.0, 50.0), cx);
        canvas.add_node(NodeType::Output, Point::new(550.0, 50.0), cx);
        
        canvas
    }

    pub fn add_node(&mut self, node_type: NodeType, position: Point<f32>, cx: &mut Context<Self>) {
        self.executor.add_node(node_type, position);
        cx.notify();
    }

    pub fn center_on_nodes(&mut self, cx: &mut Context<Self>) {
        self.interaction.viewport_manager.center_on_nodes(&self.executor.nodes, cx);
    }

    pub fn connect_nodes(
        &mut self,
        from_node: NodeId,
        from_port: String,
        to_node: NodeId,
        to_port: String,
        cx: &mut Context<Self>,
    ) {
        self.executor.connect_nodes(from_node, from_port, to_node, to_port);
        cx.notify();
    }

    pub fn delete_node(&mut self, node_id: NodeId, cx: &mut Context<Self>) {
        self.executor.delete_node(node_id);
        if self.interaction.selected_node == Some(node_id) {
            self.interaction.selected_node = None;
        }
        cx.notify();
    }

    pub fn run_workflow(&mut self, cx: &mut Context<Self>) {
        if self.executor.is_running {
            return;
        }

        self.executor.is_running = true;
        self.executor.execution_state = ExecutionState::Running;

        // Reset all node states
        for node in self.executor.nodes.values_mut() {
            node.state = NodeState::Idle;
        }

        // Simulate workflow execution
        cx.spawn(async move |this: gpui::WeakEntity<Self>, cx| {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            
            this.update(cx, |this, cx| {
                this.executor.is_running = false;
                this.executor.execution_state = ExecutionState::Completed;
                
                for node in this.executor.nodes.values_mut() {
                    node.state = NodeState::Completed;
                }
                
                cx.notify();
            }).ok();
        }).detach();

        cx.notify();
    }

    pub fn stop_workflow(&mut self, cx: &mut Context<Self>) {
        self.executor.stop_workflow();
        cx.notify();
    }

    pub fn zoom_in(&mut self, cx: &mut Context<Self>) {
        self.interaction.viewport_manager.zoom_in(cx);
    }

    pub fn zoom_out(&mut self, cx: &mut Context<Self>) {
        self.interaction.viewport_manager.zoom_out(cx);
    }

    pub fn zoom_at_point(&mut self, screen_point: Point<f32>, zoom_factor: f32, cx: &mut Context<Self>) {
        self.interaction.viewport_manager.zoom_at_point(screen_point, zoom_factor, cx);
    }

    pub fn reset_zoom(&mut self, cx: &mut Context<Self>) {
        self.interaction.viewport_manager.reset_zoom(cx);
    }

    pub fn focus_on_node(&mut self, node_id: NodeId, cx: &mut Context<Self>) {
        if let Some(node) = self.executor.nodes.get(&node_id) {
            self.interaction.viewport_manager.focus_on_node(node, cx);
            self.interaction.selected_node = Some(node_id);
        }
    }

    pub fn focus_on_selected_node(&mut self, cx: &mut Context<Self>) {
        if let Some(selected_id) = self.interaction.selected_node {
            self.focus_on_node(selected_id, cx);
        }
    }

    pub fn screen_to_canvas(&self, screen_pos: Point<f32>) -> Point<f32> {
        self.interaction.viewport_manager.screen_to_canvas(screen_pos)
    }

    pub fn canvas_to_screen(&self, canvas_pos: Point<f32>) -> Point<f32> {
        self.interaction.viewport_manager.canvas_to_screen(canvas_pos)
    }

    pub fn get_node_at_position(&self, canvas_pos: Point<f32>) -> Option<NodeId> {
        self.interaction.get_node_at_position(canvas_pos, &self.executor.nodes)
    }

    pub fn handle_mouse_down(&mut self, event: &MouseDownEvent, cx: &mut Context<Self>) {
        self.interaction.handle_mouse_down(event, &self.executor.nodes, self.canvas_bounds, cx);
    }

    pub fn handle_mouse_move(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        self.interaction.handle_mouse_move(event, &mut self.executor.nodes, self.canvas_bounds, cx);
    }

    pub fn handle_mouse_up(&mut self, event: &MouseUpEvent, cx: &mut Context<Self>) {
        self.interaction.handle_mouse_up(event, cx);
    }

    pub fn handle_scroll_wheel(&mut self, event: &ScrollWheelEvent, cx: &mut Context<Self>) {
        self.interaction.handle_scroll_wheel(event, self.canvas_bounds, cx);
    }

    pub fn handle_key_down(&mut self, event: &gpui::KeyDownEvent, cx: &mut Context<Self>) -> bool {
        self.interaction.handle_key_down(event, &self.executor.nodes, cx)
    }

    pub fn handle_mouse_leave(&mut self, cx: &mut Context<Self>) {
        self.interaction.handle_mouse_leave(cx);
    }

    pub fn handle_mouse_enter(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        self.interaction.handle_mouse_enter(event, self.canvas_bounds, cx);
    }

    pub fn get_current_mouse_screen(&self) -> Option<Point<f32>> {
        self.interaction.get_current_mouse_screen()
    }

    pub fn get_selected_node(&self) -> Option<NodeId> {
        self.interaction.selected_node
    }

    pub fn get_current_mouse_canvas(&self) -> Option<Point<f32>> {
        self.interaction.get_current_mouse_canvas()
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        CanvasRenderer::render_toolbar(
            &self.interaction.viewport_manager.viewport,
            self.interaction.selected_node,
            &self.executor.nodes,
            self.executor.is_running,
            cx,
        )
    }

    fn render_canvas(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("workflow_canvas")
            .size_full()
            .relative()
            .overflow_hidden()
            .bg(cx.theme().colors().editor_background)
            .child(
                CanvasRenderer::render_canvas_content(
                    &self.executor.nodes,
                    self.interaction.selected_node,
                    &self.interaction.viewport_manager.viewport,
                    cx,
                )
            )
    }

    fn render_status_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        CanvasRenderer::render_status_bar(
            &self.executor.nodes,
            &self.executor.connections,
            &self.interaction.viewport_manager.viewport,
            &self.interaction.viewport_manager.interaction_state,
            self.interaction.viewport_manager.current_mouse_screen,
            self.interaction.viewport_manager.current_mouse_canvas,
            cx,
        )
    }

    pub fn update_viewport_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.interaction.viewport_manager.update_viewport_bounds(bounds);
    }

    fn render_debug_info(&self, mouse_pos: Point<f32>, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .top(px(10.0))
            .right(px(10.0))
            .p_2()
            .bg(cx.theme().colors().surface_background.opacity(0.9))
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_md()
            .child(
                Label::new(format!("Canvas: ({:.0}, {:.0})", mouse_pos.x, mouse_pos.y))
                    .size(LabelSize::Small)
            )
    }
}

impl Render for WorkflowCanvas {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let canvas_handle = cx.entity().downgrade();
        
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors().background)
            .track_focus(&self.focus_handle)
            .child(self.render_toolbar(cx))
            .child(
                div()
                    .on_children_prepainted(move |children_bounds, _window, cx| {
                        if let Some(canvas) = canvas_handle.upgrade() {
                            canvas.update(cx, |canvas, _cx| {
                                // The first child is the canvas div
                                if let Some(canvas_bounds) = children_bounds.first() {
                                    canvas.canvas_bounds = Some(*canvas_bounds);
                                    canvas.interaction.viewport_manager.update_viewport_bounds(*canvas_bounds);
                                }
                            });
                        }
                    })
                    .id("canvas_container")
                    .flex_1()
                    .relative()
                    .overflow_hidden()
                    .p_0()
                    .m_0()
                    .child(self.render_canvas(cx))
                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                        window.focus(&this.focus_handle);
                        this.handle_mouse_down(event, cx);
                    }))
                    .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                        this.handle_mouse_move(event, cx);
                    }))
                    .on_mouse_up(MouseButton::Left, cx.listener(|this, event: &MouseUpEvent, _window, cx| {
                        this.handle_mouse_up(event, cx);
                    }))
                    .on_scroll_wheel(cx.listener(|this, event: &ScrollWheelEvent, _window, cx| {
                        this.handle_scroll_wheel(event, cx);
                    }))
                    .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, _window, cx| {
                        this.handle_key_down(event, cx);
                    }))
            )
            .child(self.render_status_bar(cx))
    }
}

impl EventEmitter<()> for WorkflowCanvas {}

impl Focusable for WorkflowCanvas {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
} 