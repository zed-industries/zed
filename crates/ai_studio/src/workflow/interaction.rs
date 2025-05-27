use gpui::{Point, Bounds, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ScrollWheelEvent, Context, MouseButton, ScrollDelta, TouchPhase};
use std::collections::HashMap;
use std::time::Instant;

use crate::workflow::types::*;

pub struct ViewportManager {
    pub viewport: CanvasViewport,
    pub interaction_state: InteractionState,
    pub last_mouse_position: Option<Point<f32>>,
    pub current_mouse_screen: Option<Point<f32>>,
    pub current_mouse_canvas: Option<Point<f32>>,
    pub trackpad_state: TrackpadState,
}

#[derive(Clone, Debug)]
pub struct TrackpadState {
    pub momentum_velocity: Point<f32>,
    pub last_scroll_time: Option<Instant>,
    pub accumulated_zoom: f32,
    pub is_pinch_zooming: bool,
    pub last_touch_phase: TouchPhase,
    pub scroll_momentum_decay: f32,
}

impl Default for TrackpadState {
    fn default() -> Self {
        Self {
            momentum_velocity: Point::new(0.0, 0.0),
            last_scroll_time: None,
            accumulated_zoom: 0.0,
            is_pinch_zooming: false,
            last_touch_phase: TouchPhase::Moved,
            scroll_momentum_decay: 0.95,
        }
    }
}

impl ViewportManager {
    pub fn new() -> Self {
        Self {
            viewport: CanvasViewport::default(),
            interaction_state: InteractionState::None,
            last_mouse_position: None,
            current_mouse_screen: None,
            current_mouse_canvas: None,
            trackpad_state: TrackpadState::default(),
        }
    }

    /// Convert screen coordinates to canvas coordinates
    /// Screen coordinates: (0,0) at top-left of viewport
    /// Canvas coordinates: world space coordinates of nodes
    pub fn screen_to_canvas(&self, screen_pos: Point<f32>) -> Point<f32> {
        Point::new(
            (screen_pos.x - self.viewport.offset.x) / self.viewport.scale,
            (screen_pos.y - self.viewport.offset.y) / self.viewport.scale,
        )
    }

    /// Convert canvas coordinates to screen coordinates
    pub fn canvas_to_screen(&self, canvas_pos: Point<f32>) -> Point<f32> {
        Point::new(
            canvas_pos.x * self.viewport.scale + self.viewport.offset.x,
            canvas_pos.y * self.viewport.scale + self.viewport.offset.y,
        )
    }

    /// Test coordinate transformation consistency
    pub fn test_coordinate_transform(&self, test_screen: Point<f32>) -> (Point<f32>, Point<f32>, bool) {
        let canvas = self.screen_to_canvas(test_screen);
        let back_to_screen = self.canvas_to_screen(canvas);
        let is_consistent = (test_screen.x - back_to_screen.x).abs() < 0.1 
                         && (test_screen.y - back_to_screen.y).abs() < 0.1;
        (canvas, back_to_screen, is_consistent)
    }

    pub fn zoom_in<T: 'static>(&mut self, cx: &mut Context<T>) {
        let viewport_center = Point::new(
            self.viewport.bounds.size.width.0 / 2.0,
            self.viewport.bounds.size.height.0 / 2.0,
        );
        self.zoom_at_point(viewport_center, 1.2, cx);
    }

    pub fn zoom_out<T: 'static>(&mut self, cx: &mut Context<T>) {
        let viewport_center = Point::new(
            self.viewport.bounds.size.width.0 / 2.0,
            self.viewport.bounds.size.height.0 / 2.0,
        );
        self.zoom_at_point(viewport_center, 1.0 / 1.2, cx);
    }

    /// Enhanced zoom function with better limits for trackpad interaction
    pub fn zoom_at_point<T: 'static>(&mut self, screen_point: Point<f32>, zoom_factor: f32, cx: &mut Context<T>) {
        let old_scale = self.viewport.scale;
        // Improved zoom limits: 0.05x to 5.0x for better trackpad experience
        let new_scale = (old_scale * zoom_factor).clamp(0.05, 5.0);
        
        if (new_scale - old_scale).abs() < 0.001 {
            return;
        }
        
        // Get the canvas point under the mouse before zoom
        let canvas_point = self.screen_to_canvas(screen_point);
        
        // Update scale
        self.viewport.scale = new_scale;
        
        // Calculate new screen position of the same canvas point
        let new_screen_point = self.canvas_to_screen(canvas_point);
        
        // Adjust offset to keep the canvas point under the mouse
        self.viewport.offset.x += screen_point.x - new_screen_point.x;
        self.viewport.offset.y += screen_point.y - new_screen_point.y;
        
        cx.notify();
    }

    pub fn reset_zoom<T: 'static>(&mut self, cx: &mut Context<T>) {
        self.viewport.scale = 1.0;
        self.viewport.offset = Point::new(0.0, 0.0);
        cx.notify();
    }

    pub fn focus_on_node<T: 'static>(&mut self, node: &WorkflowNode, cx: &mut Context<T>) {
        let node_center_canvas = Point::new(
            node.position.x + node.size.width / 2.0,
            node.position.y + node.size.height / 2.0,
        );
        
        let viewport_center_screen = Point::new(
            self.viewport.bounds.size.width.0 / 2.0,
            self.viewport.bounds.size.height.0 / 2.0,
        );
        
        // Calculate offset to center the node
        let node_center_screen = self.canvas_to_screen(node_center_canvas);
        self.viewport.offset.x += viewport_center_screen.x - node_center_screen.x;
        self.viewport.offset.y += viewport_center_screen.y - node_center_screen.y;
        
        cx.notify();
    }

    pub fn center_on_nodes<T: 'static>(&mut self, nodes: &HashMap<NodeId, WorkflowNode>, cx: &mut Context<T>) {
        if nodes.is_empty() {
            return;
        }

        // Find bounding box of all nodes
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for node in nodes.values() {
            min_x = min_x.min(node.position.x);
            min_y = min_y.min(node.position.y);
            max_x = max_x.max(node.position.x + node.size.width);
            max_y = max_y.max(node.position.y + node.size.height);
        }

        let nodes_center_canvas = Point::new(
            (min_x + max_x) / 2.0,
            (min_y + max_y) / 2.0,
        );

        let viewport_center_screen = Point::new(
            self.viewport.bounds.size.width.0 / 2.0,
            self.viewport.bounds.size.height.0 / 2.0,
        );

        // Calculate offset to center all nodes
        let nodes_center_screen = self.canvas_to_screen(nodes_center_canvas);
        self.viewport.offset.x += viewport_center_screen.x - nodes_center_screen.x;
        self.viewport.offset.y += viewport_center_screen.y - nodes_center_screen.y;

        cx.notify();
    }

    pub fn update_viewport_bounds(&mut self, bounds: Bounds<gpui::Pixels>) {
        self.viewport.bounds = bounds;
    }

    /// Apply smooth trackpad panning with immediate momentum
    pub fn apply_trackpad_pan<T: 'static>(&mut self, delta: Point<f32>, cx: &mut Context<T>) {
        // Apply pan directly to viewport offset
        self.viewport.offset.x += delta.x;
        self.viewport.offset.y += delta.y;
        
        // Store momentum for potential future use (could be used for inertial scrolling)
        self.trackpad_state.momentum_velocity = delta * 0.3; // Momentum factor
        
        cx.notify();
    }

    /// Update momentum (simplified - no longer requires timer)
    pub fn update_momentum<T: 'static>(&mut self, _cx: &mut Context<T>) {
        // This method is kept for potential future enhancements
        // For now, momentum is applied directly in scroll events
        if self.trackpad_state.momentum_velocity.x.abs() < 0.1 
            && self.trackpad_state.momentum_velocity.y.abs() < 0.1 {
            return;
        }

        // Decay momentum
        self.trackpad_state.momentum_velocity.x *= self.trackpad_state.scroll_momentum_decay;
        self.trackpad_state.momentum_velocity.y *= self.trackpad_state.scroll_momentum_decay;
    }

    /// Handle pinch-to-zoom gesture with improved smoothing
    pub fn handle_pinch_zoom<T: 'static>(&mut self, zoom_delta: f32, center_point: Point<f32>, cx: &mut Context<T>) {
        // Accumulate zoom for smoother experience
        self.trackpad_state.accumulated_zoom += zoom_delta;
        
        // Apply zoom when accumulated change is significant enough
        // Reduced threshold for more responsive pinch gestures
        if self.trackpad_state.accumulated_zoom.abs() > 0.02 {
            let zoom_factor = 1.0 + self.trackpad_state.accumulated_zoom;
            self.zoom_at_point(center_point, zoom_factor, cx);
            self.trackpad_state.accumulated_zoom = 0.0;
        }
    }
}

pub struct InteractionHandler {
    pub viewport_manager: ViewportManager,
    pub selected_node: Option<NodeId>,
}

impl InteractionHandler {
    pub fn new() -> Self {
        Self {
            viewport_manager: ViewportManager::new(),
            selected_node: None,
        }
    }

    /// Transform mouse coordinates from window space to canvas space
    /// This uses GPUI's element bounds to handle all coordinate transformations properly
    fn transform_mouse_position(&self, window_pos: Point<f32>, canvas_bounds: Option<Bounds<gpui::Pixels>>) -> Point<f32> {
        if let Some(bounds) = canvas_bounds {
            // Convert window coordinates to canvas-relative coordinates
            Point::new(
                window_pos.x - bounds.origin.x.0,
                window_pos.y - bounds.origin.y.0,
            )
        } else {
            // Fallback to window coordinates if bounds not available
            window_pos
        }
    }

    pub fn get_node_at_position(&self, screen_pos: Point<f32>, nodes: &HashMap<NodeId, WorkflowNode>) -> Option<NodeId> {
        // Check nodes in screen coordinates (matching visual positioning EXACTLY)
        for (node_id, node) in nodes.iter() {
            // Use EXACTLY the same calculation as in rendering.rs
            let screen_x = node.position.x * self.viewport_manager.viewport.scale + self.viewport_manager.viewport.offset.x;
            let screen_y = node.position.y * self.viewport_manager.viewport.scale + self.viewport_manager.viewport.offset.y;
            let screen_width = node.size.width * self.viewport_manager.viewport.scale;
            let screen_height = node.size.height * self.viewport_manager.viewport.scale;
            
            // Check exact bounds
            if screen_pos.x >= screen_x
                && screen_pos.x <= screen_x + screen_width
                && screen_pos.y >= screen_y 
                && screen_pos.y <= screen_y + screen_height {
                return Some(*node_id);
            }
        }
        None
    }

    pub fn handle_node_click<T: 'static>(&mut self, node_id: NodeId, nodes: &HashMap<NodeId, WorkflowNode>, _cx: &mut Context<T>) {
        if let Some(_node) = nodes.get(&node_id) {
            // You can add any action here, like opening a node editor, showing properties, etc.
        }
    }

    pub fn handle_mouse_down<T: 'static>(&mut self, event: &MouseDownEvent, nodes: &HashMap<NodeId, WorkflowNode>, canvas_bounds: Option<Bounds<gpui::Pixels>>, cx: &mut Context<T>) {
        let window_pos = Point::new(event.position.x.0, event.position.y.0);
        let screen_pos = self.transform_mouse_position(window_pos, canvas_bounds);
        let canvas_pos = self.viewport_manager.screen_to_canvas(screen_pos);
        
        // Update mouse tracking
        self.viewport_manager.current_mouse_screen = Some(screen_pos);
        self.viewport_manager.current_mouse_canvas = Some(canvas_pos);

        let hit_node = self.get_node_at_position(screen_pos, nodes);

        match event.button {
            MouseButton::Left => {
                if let Some(node_id) = hit_node {
                    // Handle node click
                    self.handle_node_click(node_id, nodes, cx);
                    
                    // Start node drag
                    if let Some(node) = nodes.get(&node_id) {
                        let drag_offset = Point::new(
                            canvas_pos.x - node.position.x,
                            canvas_pos.y - node.position.y,
                        );
                        
                        self.viewport_manager.interaction_state = InteractionState::NodeDrag {
                            node_id,
                            drag_offset,
                        };
                        self.selected_node = Some(node_id);
                    }
                } else {
                    // Start canvas pan
                    self.viewport_manager.interaction_state = InteractionState::CanvasPan {
                        start_screen_pos: screen_pos,
                        start_offset: self.viewport_manager.viewport.offset,
                    };
                    self.selected_node = None;
                }
                cx.notify();
            }
            _ => {}
        }
    }

    pub fn handle_mouse_move<T: 'static>(&mut self, event: &MouseMoveEvent, nodes: &mut HashMap<NodeId, WorkflowNode>, canvas_bounds: Option<Bounds<gpui::Pixels>>, cx: &mut Context<T>) {
        let window_pos = Point::new(event.position.x.0, event.position.y.0);
        let screen_pos = self.transform_mouse_position(window_pos, canvas_bounds);
        let canvas_pos = self.viewport_manager.screen_to_canvas(screen_pos);
        
        // Always update mouse tracking
        self.viewport_manager.current_mouse_screen = Some(screen_pos);
        self.viewport_manager.current_mouse_canvas = Some(canvas_pos);

        match &self.viewport_manager.interaction_state {
            InteractionState::NodeDrag { node_id, drag_offset } => {
                // Update node position in canvas coordinates
                if let Some(node) = nodes.get_mut(node_id) {
                    node.position = Point::new(
                        canvas_pos.x - drag_offset.x,
                        canvas_pos.y - drag_offset.y,
                    );
                }
                cx.notify();
            }
            InteractionState::CanvasPan { start_screen_pos, start_offset } => {
                // Calculate screen delta and apply to offset
                let screen_delta = Point::new(
                    screen_pos.x - start_screen_pos.x,
                    screen_pos.y - start_screen_pos.y,
                );
                
                self.viewport_manager.viewport.offset = Point::new(
                    start_offset.x + screen_delta.x,
                    start_offset.y + screen_delta.y,
                );
                cx.notify();
            }
            InteractionState::None => {
                // Just update mouse position, no need to notify constantly
            }
        }
    }

    pub fn handle_mouse_up<T: 'static>(&mut self, _event: &MouseUpEvent, cx: &mut Context<T>) {
        self.viewport_manager.interaction_state = InteractionState::None;
        cx.notify();
    }

    pub fn handle_scroll_wheel<T: 'static>(&mut self, event: &ScrollWheelEvent, canvas_bounds: Option<Bounds<gpui::Pixels>>, cx: &mut Context<T>) {
        let window_pos = Point::new(event.position.x.0, event.position.y.0);
        let mouse_pos = self.transform_mouse_position(window_pos, canvas_bounds);
        
        // Update trackpad state
        self.viewport_manager.trackpad_state.last_scroll_time = Some(std::time::Instant::now());
        self.viewport_manager.trackpad_state.last_touch_phase = event.touch_phase;
        
        // Handle different types of scroll input
        match event.delta {
            ScrollDelta::Pixels(pixels) => {
                // This is typically from a trackpad with precise pixel deltas
                self.handle_trackpad_scroll(pixels, mouse_pos, event, cx);
            }
            ScrollDelta::Lines(lines) => {
                // This is typically from a mouse wheel or older trackpad
                self.handle_mouse_wheel_scroll(lines, mouse_pos, event, cx);
            }
        }
    }

    fn handle_trackpad_scroll<T: 'static>(&mut self, pixels: Point<gpui::Pixels>, mouse_pos: Point<f32>, event: &ScrollWheelEvent, cx: &mut Context<T>) {
        let delta = Point::new(pixels.x.0, pixels.y.0);
        
        // Enhanced pinch-to-zoom detection
        // On macOS trackpads, pinch gestures often come with:
        // 1. Cmd key modifier for zoom
        // 2. Control key for accessibility zoom
        // 3. Small horizontal movement with larger vertical movement
        // 4. Specific touch phase patterns
        let is_pinch_gesture = event.modifiers.platform || 
                              event.modifiers.control ||
                              (delta.x.abs() < 3.0 && delta.y.abs() > 5.0 && 
                               self.viewport_manager.trackpad_state.is_pinch_zooming);
        
        // Detect start of pinch gesture
        if !self.viewport_manager.trackpad_state.is_pinch_zooming && 
           (event.modifiers.platform || event.modifiers.control) &&
           matches!(event.touch_phase, TouchPhase::Started) {
            self.viewport_manager.trackpad_state.is_pinch_zooming = true;
        }
        
        if is_pinch_gesture {
            // Handle pinch-to-zoom with improved sensitivity
            let zoom_sensitivity = if event.modifiers.platform { 0.008 } else { 0.012 };
            let zoom_delta = -delta.y * zoom_sensitivity;
            
            // Apply zoom smoothing for better UX
            self.viewport_manager.handle_pinch_zoom(zoom_delta, mouse_pos, cx);
            
            // Reset pinch state on gesture end
            if matches!(event.touch_phase, TouchPhase::Ended) {
                self.viewport_manager.trackpad_state.is_pinch_zooming = false;
                self.viewport_manager.trackpad_state.accumulated_zoom = 0.0;
            }
        } else {
            // Handle two-finger pan with improved momentum
            self.viewport_manager.trackpad_state.is_pinch_zooming = false;
            
            // Apply different sensitivities based on zoom level for better control
            let zoom_factor = self.viewport_manager.viewport.scale;
            let pan_sensitivity = if zoom_factor > 1.5 { 0.8 } else if zoom_factor < 0.5 { 1.2 } else { 1.0 };
            let pan_delta = Point::new(delta.x * pan_sensitivity, delta.y * pan_sensitivity);
            
            match event.touch_phase {
                TouchPhase::Started => {
                    // Reset momentum on new gesture
                    self.viewport_manager.trackpad_state.momentum_velocity = Point::new(0.0, 0.0);
                    self.viewport_manager.apply_trackpad_pan(pan_delta, cx);
                }
                TouchPhase::Moved => {
                    // Continue panning with momentum
                    self.viewport_manager.apply_trackpad_pan(pan_delta, cx);
                }
                TouchPhase::Ended => {
                    // Apply final momentum based on gesture velocity
                    let momentum_factor = 0.4;
                    self.viewport_manager.trackpad_state.momentum_velocity = pan_delta * momentum_factor;
                }
            }
        }
    }

    fn handle_mouse_wheel_scroll<T: 'static>(&mut self, lines: Point<f32>, mouse_pos: Point<f32>, _event: &ScrollWheelEvent, cx: &mut Context<T>) {
        // Traditional mouse wheel behavior - zoom on vertical scroll
        let zoom_sensitivity = 0.1;
        let zoom_factor = if lines.y > 0.0 { 
            1.0 - (lines.y * zoom_sensitivity) 
        } else { 
            1.0 + (-lines.y * zoom_sensitivity) 
        };
        
        // Handle horizontal scrolling as panning
        if lines.x.abs() > 0.1 {
            let pan_delta = Point::new(lines.x * 20.0, 0.0);
            self.viewport_manager.apply_trackpad_pan(pan_delta, cx);
        } else {
            // Zoom on vertical scroll
            self.viewport_manager.zoom_at_point(mouse_pos, zoom_factor, cx);
        }
    }

    pub fn handle_key_down<T: 'static>(&mut self, event: &gpui::KeyDownEvent, nodes: &HashMap<NodeId, WorkflowNode>, cx: &mut Context<T>) -> bool {
        match event.keystroke.key.as_str() {
            "=" | "+" if event.keystroke.modifiers.platform => {
                self.viewport_manager.zoom_in(cx);
                true
            }
            "-" if event.keystroke.modifiers.platform => {
                self.viewport_manager.zoom_out(cx);
                true
            }
            "0" if event.keystroke.modifiers.platform => {
                self.viewport_manager.reset_zoom(cx);
                true
            }
            "f" if event.keystroke.modifiers.platform => {
                if let Some(selected_id) = self.selected_node {
                    if let Some(node) = nodes.get(&selected_id) {
                        self.viewport_manager.focus_on_node(node, cx);
                    }
                } else {
                    // Focus on all nodes if none selected
                    self.viewport_manager.center_on_nodes(nodes, cx);
                }
                true
            }
            // Arrow keys for precise panning (like trackpad)
            "ArrowLeft" => {
                let pan_amount = if event.keystroke.modifiers.shift { 50.0 } else { 20.0 };
                self.viewport_manager.apply_trackpad_pan(Point::new(pan_amount, 0.0), cx);
                true
            }
            "ArrowRight" => {
                let pan_amount = if event.keystroke.modifiers.shift { 50.0 } else { 20.0 };
                self.viewport_manager.apply_trackpad_pan(Point::new(-pan_amount, 0.0), cx);
                true
            }
            "ArrowUp" => {
                let pan_amount = if event.keystroke.modifiers.shift { 50.0 } else { 20.0 };
                self.viewport_manager.apply_trackpad_pan(Point::new(0.0, pan_amount), cx);
                true
            }
            "ArrowDown" => {
                let pan_amount = if event.keystroke.modifiers.shift { 50.0 } else { 20.0 };
                self.viewport_manager.apply_trackpad_pan(Point::new(0.0, -pan_amount), cx);
                true
            }
            // Space bar for fit-to-screen (common in design apps)
            " " if !event.keystroke.modifiers.platform => {
                self.viewport_manager.center_on_nodes(nodes, cx);
                true
            }
            _ => false,
        }
    }

    pub fn get_current_mouse_screen(&self) -> Option<Point<f32>> {
        self.viewport_manager.current_mouse_screen
    }

    pub fn get_current_mouse_canvas(&self) -> Option<Point<f32>> {
        self.viewport_manager.current_mouse_canvas
    }

    pub fn handle_mouse_leave<T: 'static>(&mut self, cx: &mut Context<T>) {
        self.viewport_manager.current_mouse_screen = None;
        self.viewport_manager.current_mouse_canvas = None;
        cx.notify();
    }

    pub fn handle_mouse_enter<T: 'static>(&mut self, event: &MouseMoveEvent, canvas_bounds: Option<Bounds<gpui::Pixels>>, cx: &mut Context<T>) {
        let window_pos = Point::new(event.position.x.0, event.position.y.0);
        let screen_pos = self.transform_mouse_position(window_pos, canvas_bounds);
        let canvas_pos = self.viewport_manager.screen_to_canvas(screen_pos);
        
        self.viewport_manager.current_mouse_screen = Some(screen_pos);
        self.viewport_manager.current_mouse_canvas = Some(canvas_pos);
        cx.notify();
    }
} 