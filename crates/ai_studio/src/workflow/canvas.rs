use gpui::{Context, Window, Point, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ScrollWheelEvent, Bounds, Pixels, FocusHandle, Focusable, EventEmitter, Render, IntoElement, Size, px};
use ui::{prelude::*, ActiveTheme, IconName};

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
        
        // Start with just one example node to demonstrate functionality
        canvas.add_node(NodeType::Input, Point::new(100.0, 100.0), cx);
        
        println!("🎨 Canvas initialized with 1 example node");
        println!("💡 Use the toolbar buttons or keyboard shortcuts to add more nodes:");
        println!("   • Cmd+1: Input  • Cmd+2: LLM  • Cmd+3: Processor");
        println!("   • Cmd+4: Condition  • Cmd+5: Output  • Cmd+6: Data  • Cmd+7: Transform");
        println!("   • Delete/Backspace: Delete selected node");
        
        canvas
    }

    pub fn add_node(&mut self, node_type: NodeType, position: Point<f32>, cx: &mut Context<Self>) {
        self.executor.add_node(node_type, position);
        cx.notify();
    }

    /// Add a node of the specified type at the center of the current view
    pub fn add_node_at_center(&mut self, node_type: NodeType, cx: &mut Context<Self>) {
        // Calculate center position in canvas coordinates
        let center_screen = Point::new(
            self.interaction.viewport_manager.viewport.bounds.size.width.0 / 2.0,
            self.interaction.viewport_manager.viewport.bounds.size.height.0 / 2.0,
        );
        let center_canvas = self.screen_to_canvas(center_screen);
        
        println!("➕ Added {:?} node at canvas position ({:.0}, {:.0})", 
            node_type, center_canvas.x, center_canvas.y);
        self.add_node(node_type, center_canvas, cx);
    }
    
    /// Add a node of the specified type at a random position near the center
    pub fn add_node_near_center(&mut self, node_type: NodeType, cx: &mut Context<Self>) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Generate a pseudo-random offset based on current node count
        let mut hasher = DefaultHasher::new();
        self.executor.nodes.len().hash(&mut hasher);
        let hash = hasher.finish();
        
        let offset_x = ((hash % 200) as f32) - 100.0; // -100 to +100
        let offset_y = (((hash / 200) % 200) as f32) - 100.0; // -100 to +100
        
        let center_screen = Point::new(
            self.interaction.viewport_manager.viewport.bounds.size.width.0 / 2.0,
            self.interaction.viewport_manager.viewport.bounds.size.height.0 / 2.0,
        );
        let center_canvas = self.screen_to_canvas(center_screen);
        let position = Point::new(center_canvas.x + offset_x, center_canvas.y + offset_y);
        
        println!("➕ Added {:?} node at canvas position ({:.0}, {:.0})", 
            node_type, position.x, position.y);
        self.add_node(node_type, position, cx);
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
        // First try to handle keyboard shortcuts
        if self.handle_keyboard_shortcuts(event, cx) {
            return true;
        }
        
        // Fall back to interaction handler for other keys
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

    /// Extract the current workflow data for saving
    pub fn extract_workflow_data(&self) -> (Vec<&WorkflowNode>, &Vec<NodeConnection>) {
        let nodes: Vec<&WorkflowNode> = self.executor.nodes.values().collect();
        (nodes, &self.executor.connections)
    }
    
    /// Load workflow data into the canvas
    pub fn load_workflow_data(&mut self, nodes: Vec<WorkflowNode>, connections: Vec<NodeConnection>, cx: &mut Context<Self>) {
        println!("🎨 Canvas: Loading {} nodes and {} connections", nodes.len(), connections.len());
        
        // Clear existing data
        self.executor.nodes.clear();
        self.executor.connections.clear();
        
        // Load nodes
        for node in nodes {
            println!("📍 Loading node: {} at ({:.0}, {:.0})", node.title, node.position.x, node.position.y);
            self.executor.nodes.insert(node.id, node);
        }
        
        // Load connections
        self.executor.connections = connections;
        
        // Reset execution state
        self.executor.is_running = false;
        self.executor.execution_state = ExecutionState::Stopped;
        
        // Clear interaction state
        self.interaction.selected_node = None;
        
        // Center view on loaded nodes if there are any
        if !self.executor.nodes.is_empty() {
            println!("🔍 Centering view on {} loaded nodes", self.executor.nodes.len());
            self.center_on_nodes(cx);
        } else {
            println!("⚠️  No nodes to center on after loading");
        }
        
        println!("✅ Canvas: Workflow data loaded successfully");
        cx.notify();
    }
    
    /// Clear the canvas
    pub fn clear_canvas(&mut self, cx: &mut Context<Self>) {
        self.executor.nodes.clear();
        self.executor.connections.clear();
        self.executor.is_running = false;
        self.executor.execution_state = ExecutionState::Stopped;
        self.interaction.selected_node = None;
        cx.notify();
    }

    pub fn delete_selected_node(&mut self, cx: &mut Context<Self>) {
        if let Some(selected_id) = self.interaction.selected_node {
            if let Some(node) = self.executor.nodes.get(&selected_id) {
                let node_title = node.title.clone();
                self.delete_node(selected_id, cx);
                println!("🗑️  Deleted node: {}", node_title);
            }
        } else {
            println!("❌ No node selected to delete");
        }
    }
    
    /// Handle keyboard shortcuts for node operations
    pub fn handle_keyboard_shortcuts(&mut self, event: &gpui::KeyDownEvent, cx: &mut Context<Self>) -> bool {
        match event.keystroke.key.as_str() {
            "1" if event.keystroke.modifiers.platform => {
                self.add_node_near_center(NodeType::Input, cx);
                true
            }
            "2" if event.keystroke.modifiers.platform => {
                self.add_node_near_center(NodeType::LLMPrompt, cx);
                true
            }
            "3" if event.keystroke.modifiers.platform => {
                self.add_node_near_center(NodeType::TextProcessor, cx);
                true
            }
            "4" if event.keystroke.modifiers.platform => {
                self.add_node_near_center(NodeType::Conditional, cx);
                true
            }
            "5" if event.keystroke.modifiers.platform => {
                self.add_node_near_center(NodeType::Output, cx);
                true
            }
            "6" if event.keystroke.modifiers.platform => {
                self.add_node_near_center(NodeType::DataSource, cx);
                true
            }
            "7" if event.keystroke.modifiers.platform => {
                self.add_node_near_center(NodeType::Transform, cx);
                true
            }
            "Delete" | "Backspace" => {
                self.delete_selected_node(cx);
                true
            }
            "a" if event.keystroke.modifiers.platform => {
                // Select all nodes (for future use)
                println!("🔍 Select all functionality (not implemented yet)");
                true
            }
            "r" if event.keystroke.modifiers.platform => {
                self.run_workflow(cx);
                true
            }
            "s" if event.keystroke.modifiers.platform => {
                self.stop_workflow(cx);
                true
            }
            "0" if event.keystroke.modifiers.platform => {
                self.reset_zoom(cx);
                true
            }
            "=" | "+" if event.keystroke.modifiers.platform => {
                self.zoom_in(cx);
                true
            }
            "-" if event.keystroke.modifiers.platform => {
                self.zoom_out(cx);
                true
            }
            "f" if event.keystroke.modifiers.platform => {
                self.center_on_nodes(cx);
                true
            }
            _ => false,
        }
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
                        Label::new("Add Nodes:")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
                    .child(
                        Button::new("add_input", "Input")
                            .style(ButtonStyle::Filled)
                            .icon(IconName::FileText)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.add_node_near_center(NodeType::Input, cx);
                            }))
                    )
                    .child(
                        Button::new("add_llm", "LLM")
                            .style(ButtonStyle::Filled)
                            .icon(IconName::Brain)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.add_node_near_center(NodeType::LLMPrompt, cx);
                            }))
                    )
                    .child(
                        Button::new("add_processor", "Processor")
                            .style(ButtonStyle::Filled)
                            .icon(IconName::Sliders)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.add_node_near_center(NodeType::TextProcessor, cx);
                            }))
                    )
                    .child(
                        Button::new("add_condition", "Condition")
                            .style(ButtonStyle::Filled)
                            .icon(IconName::GitBranch)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.add_node_near_center(NodeType::Conditional, cx);
                            }))
                    )
                    .child(
                        Button::new("add_output", "Output")
                            .style(ButtonStyle::Filled)
                            .icon(IconName::FileText)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.add_node_near_center(NodeType::Output, cx);
                            }))
                    )
                    .child(
                        Button::new("add_data_source", "Data")
                            .style(ButtonStyle::Filled)
                            .icon(IconName::FileText)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.add_node_near_center(NodeType::DataSource, cx);
                            }))
                    )
                    .child(
                        Button::new("add_transform", "Transform")
                            .style(ButtonStyle::Filled)
                            .icon(IconName::ArrowDown)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.add_node_near_center(NodeType::Transform, cx);
                            }))
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
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.zoom_out(cx);
                            }))
                    )
                    .child(
                        Label::new(format!("{}%", (self.interaction.viewport_manager.viewport.scale * 100.0) as i32))
                            .size(LabelSize::Small)
                    )
                    .child(
                        Button::new("zoom_in", "+")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.zoom_in(cx);
                            }))
                    )
                    .child(
                        Button::new("reset_zoom", "Reset")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.reset_zoom(cx);
                            }))
                    )
                    .child(
                        Button::new("center_all", "Center All")
                            .style(ButtonStyle::Subtle)
                            .disabled(self.executor.nodes.is_empty())
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.center_on_nodes(cx);
                            }))
                    )
                    .child(
                        Button::new("focus_selected", "Focus")
                            .style(ButtonStyle::Subtle)
                            .disabled(self.interaction.selected_node.is_none())
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.focus_on_selected_node(cx);
                            }))
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
                            .icon(IconName::Play)
                            .disabled(self.executor.is_running)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.run_workflow(cx);
                            }))
                    )
                    .child(
                        Button::new("stop", "Stop")
                            .style(ButtonStyle::Subtle)
                            .icon(IconName::X)
                            .disabled(!self.executor.is_running)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.stop_workflow(cx);
                            }))
                    )
                    .child(
                        Label::new(format!("{} nodes", self.executor.nodes.len()))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
                    .child(
                        Label::new("💡 ⌘1-7: Add nodes, Del: Remove")
                            .size(LabelSize::XSmall)
                            .color(Color::Muted)
                    )
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
            &self.interaction.viewport_manager.trackpad_state,
            cx,
        )
    }

    pub fn update_viewport_bounds(&mut self, bounds: Bounds<Pixels>) {
        self.interaction.viewport_manager.update_viewport_bounds(bounds);
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