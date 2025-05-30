use gpui::{
    Context, Window, Entity, FocusHandle, EventEmitter, Focusable, Render, 
    IntoElement, Subscription, Task, MouseButton
};
use ui::{prelude::*, IconName, Label, LabelSize, Button, ButtonStyle, Icon, ListItem, h_flex, v_flex, ListItemSpacing, Tooltip};
use std::sync::Arc;

use crate::workflow::{
    WorkflowCanvas, WorkflowManager, SerializableWorkflow, WorkflowMetadata, WorkflowNode
};

/// Combined workflow manager that includes both canvas and workflow list
pub struct WorkflowManagerView {
    workflow_canvas: Entity<WorkflowCanvas>,
    workflow_manager: Option<Arc<WorkflowManager>>,
    workflows: Vec<(uide::RecordId, SerializableWorkflow)>,
    selected_workflow: Option<uide::RecordId>,
    active_panel: WorkflowPanel,
    focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
    loading_workflows: bool,
    loading_task: Option<Task<()>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorkflowPanel {
    Canvas,
    List,
}

impl WorkflowManagerView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workflow_canvas = cx.new(|cx| WorkflowCanvas::new(window, cx));
        
        let mut view = Self {
            workflow_canvas,
            workflow_manager: None,
            workflows: Vec::new(),
            selected_workflow: None,
            active_panel: WorkflowPanel::Canvas,
            focus_handle: cx.focus_handle(),
            _subscriptions: Vec::new(),
            loading_workflows: false,
            loading_task: None,
        };

        // Initialize UIDE workflow manager asynchronously
        view.initialize_workflow_manager(cx);
        
        view
    }

    fn initialize_workflow_manager(&mut self, cx: &mut Context<Self>) {
        let task = cx.spawn(async move |this, cx| {
            // Try to initialize the workflow manager
            match WorkflowManager::new("./ai_studio_data").await {
                Ok(manager) => {
                    let manager = Arc::new(manager);
                    // Load existing workflows
                    match manager.list_workflows().await {
                        Ok(workflows) => {
                            this.update(cx, |this, cx| {
                                this.workflow_manager = Some(manager);
                                this.workflows = workflows;
                                this.loading_workflows = false;
                                cx.notify();
                            }).ok();
                        }
                        Err(err) => {
                            eprintln!("Failed to load workflows: {}", err);
                            this.update(cx, |this, cx| {
                                this.workflow_manager = Some(manager);
                                this.loading_workflows = false;
                                cx.notify();
                            }).ok();
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to initialize workflow manager: {}", err);
                    this.update(cx, |this, cx| {
                        this.loading_workflows = false;
                        cx.notify();
                    }).ok();
                }
            }
        });
        
        self.loading_workflows = true;
        self.loading_task = Some(task);
    }

    pub fn set_active_panel(&mut self, panel: WorkflowPanel, cx: &mut Context<Self>) {
        self.active_panel = panel;
        cx.notify();
    }

    pub fn save_current_workflow(&mut self, cx: &mut Context<Self>) {
        if let Some(manager) = &self.workflow_manager {
            let manager = manager.clone();
            
            // Extract actual workflow data from the canvas and clone it for the async task
            let (nodes, connections) = self.workflow_canvas.read(cx).extract_workflow_data();
            let nodes_owned: Vec<WorkflowNode> = nodes.iter().map(|&node| node.clone()).collect();
            let connections_owned = connections.clone();
            
            let task = cx.spawn(async move |this, cx| {
                let timestamp = chrono::Utc::now();
                
                // Convert runtime data to serializable format
                let serializable_nodes: Vec<crate::workflow::persistence::SerializableNode> = nodes_owned.iter()
                    .map(|node| crate::workflow::persistence::SerializableNode::from(node))
                    .collect();
                
                let serializable_connections: Vec<crate::workflow::persistence::SerializableConnection> = connections_owned.iter()
                    .map(|conn| crate::workflow::persistence::SerializableConnection::from(conn))
                    .collect();
                
                let workflow = SerializableWorkflow {
                    id: uuid::Uuid::new_v4(),
                    name: format!("Workflow {}", timestamp.format("%m-%d %H:%M")),
                    description: format!("Created from AI Studio canvas with {} nodes", serializable_nodes.len()),
                    version: "1.0.0".to_string(),
                    created_at: timestamp,
                    updated_at: timestamp,
                    tags: vec!["ai_studio_workflow".to_string(), "canvas".to_string()],
                    nodes: serializable_nodes,
                    connections: serializable_connections,
                    metadata: WorkflowMetadata {
                        author: Some("AI Studio User".to_string()),
                        category: "general".to_string(),
                        complexity: if nodes_owned.len() <= 3 { "simple" } else if nodes_owned.len() <= 8 { "medium" } else { "complex" }.to_string(),
                        estimated_runtime: Some((nodes_owned.len() * 5) as u32), // Rough estimate
                        dependencies: vec![],
                        ai_config_id: None,
                    },
                };

                let workflow_name = workflow.name.clone();
                match manager.save_workflow(&workflow).await {
                    Ok(workflow_id) => {
                        this.update(cx, |this, cx| {
                            this.workflows.push((workflow_id, workflow));
                            // Switch to list view to show the newly saved workflow
                            this.set_active_panel(WorkflowPanel::List, cx);
                            // Select the newly saved workflow
                            this.selected_workflow = Some(workflow_id);
                            println!("✅ Saved workflow: '{}' with {} nodes and {} connections", 
                                workflow_name, nodes_owned.len(), connections_owned.len());
                            cx.notify();
                        }).ok();
                    }
                    Err(err) => {
                        eprintln!("❌ Failed to save workflow: {}", err);
                    }
                }
            });
            
            task.detach();
        } else {
            eprintln!("❌ Workflow manager not initialized yet");
        }
    }

    pub fn load_workflow(&mut self, workflow_id: uide::RecordId, cx: &mut Context<Self>) {
        if let Some(manager) = &self.workflow_manager {
            let manager = manager.clone();
            
            // Set selected workflow immediately for UI feedback
            self.selected_workflow = Some(workflow_id);
            cx.notify();
            
            let task = cx.spawn(async move |this, cx| {
                match manager.load_workflow(workflow_id).await {
                    Ok(Some(workflow)) => {
                        this.update(cx, |this, cx| {
                            // Convert serializable workflow to runtime data
                            match workflow.to_runtime_data() {
                                Ok((nodes, connections)) => {
                                    // Load the data into the canvas
                                    this.workflow_canvas.update(cx, |canvas, cx| {
                                        canvas.load_workflow_data(nodes, connections, cx);
                                    });
                                    
                                    // Switch to canvas view to show the loaded workflow
                                    this.set_active_panel(WorkflowPanel::Canvas, cx);
                                    
                                    println!("✅ Loaded workflow: '{}' with {} nodes and {} connections", 
                                        workflow.name, workflow.nodes.len(), workflow.connections.len());
                                }
                                Err(err) => {
                                    eprintln!("❌ Failed to convert workflow data: {}", err);
                                    this.selected_workflow = None;
                                }
                            }
                            cx.notify();
                        }).ok();
                    }
                    Ok(None) => {
                        eprintln!("❌ Workflow not found: {}", workflow_id);
                        this.update(cx, |this, cx| {
                            // Clear selection if workflow not found
                            this.selected_workflow = None;
                            cx.notify();
                        }).ok();
                    }
                    Err(err) => {
                        eprintln!("❌ Failed to load workflow: {}", err);
                        this.update(cx, |this, cx| {
                            // Clear selection on error
                            this.selected_workflow = None;
                            cx.notify();
                        }).ok();
                    }
                }
            });
            
            task.detach();
        }
    }

    pub fn delete_workflow(&mut self, workflow_id: uide::RecordId, cx: &mut Context<Self>) {
        if let Some(manager) = &self.workflow_manager {
            // Get the workflow name for user feedback
            let workflow_name = self.workflows.iter()
                .find(|(id, _)| *id == workflow_id)
                .map(|(_, workflow)| workflow.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            
            let manager = manager.clone();
            
            let task = cx.spawn(async move |this, cx| {
                match manager.delete_workflow(workflow_id).await {
                    Ok(true) => {
                        this.update(cx, |this, cx| {
                            this.workflows.retain(|(id, _)| *id != workflow_id);
                            if this.selected_workflow == Some(workflow_id) {
                                this.selected_workflow = None;
                            }
                            println!("🗑️  Deleted workflow: '{}'", workflow_name);
                            cx.notify();
                        }).ok();
                    }
                    Ok(false) => {
                        eprintln!("❌ Workflow '{}' not found for deletion", workflow_name);
                    }
                    Err(err) => {
                        eprintln!("❌ Failed to delete workflow '{}': {}", workflow_name, err);
                    }
                }
            });
            
            task.detach();
        } else {
            eprintln!("❌ Workflow manager not initialized yet");
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
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("toggle_canvas", "Canvas")
                            .style(if self.active_panel == WorkflowPanel::Canvas {
                                ButtonStyle::Filled
                            } else {
                                ButtonStyle::Subtle
                            })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.set_active_panel(WorkflowPanel::Canvas, cx);
                            }))
                    )
                    .child(
                        Button::new("toggle_list", "Workflows")
                            .style(if self.active_panel == WorkflowPanel::List {
                                ButtonStyle::Filled
                            } else {
                                ButtonStyle::Subtle
                            })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.set_active_panel(WorkflowPanel::List, cx);
                            }))
                    )
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("save_workflow", "Save Workflow")
                            .style(ButtonStyle::Filled)
                            .icon(IconName::Save)
                            .tooltip(|window, cx| Tooltip::text("Save current workflow (Ctrl+S)")(window, cx))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.save_current_workflow(cx);
                            }))
                    )
                    .child(
                        Button::new("new_workflow", "New")
                            .style(ButtonStyle::Subtle)
                            .icon(IconName::Plus)
                            .tooltip(|window, cx| Tooltip::text("Create new workflow")(window, cx))
                            .on_click(cx.listener(|this, _, _, cx| {
                                // Clear the canvas and start new workflow
                                this.workflow_canvas.update(cx, |canvas, cx| {
                                    canvas.clear_canvas(cx);
                                });
                                this.set_active_panel(WorkflowPanel::Canvas, cx);
                                // Clear any selected workflow in the list
                                this.selected_workflow = None;
                                println!("📄 Started new workflow - canvas cleared");
                            }))
                    )
            )
    }

    fn render_workflow_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .p_4()
            .gap_2()
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .mb_4()
                    .child(
                        Label::new("Saved Workflows")
                            .size(LabelSize::Large)
                    )
                    .child(
                        Label::new(format!("{} workflows", self.workflows.len()))
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
            )
            .child(
                if self.loading_workflows {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size_full()
                        .child(
                            Label::new("Loading workflows...")
                                .color(Color::Muted)
                        )
                } else if self.workflows.is_empty() {
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .size_full()
                        .gap_4()
                        .child(
                            Icon::new(IconName::Route)
                                .size(IconSize::XLarge)
                                .color(Color::Muted)
                        )
                        .child(
                            Label::new("No workflows yet")
                                .size(LabelSize::Large)
                                .color(Color::Muted)
                        )
                        .child(
                            Label::new("Create your first workflow in the Canvas tab")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        )
                        .child(
                            Button::new("go_to_canvas", "Go to Canvas")
                                .style(ButtonStyle::Filled)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.set_active_panel(WorkflowPanel::Canvas, cx);
                                }))
                        )
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .size_full()
                        .gap_1()
                        .children(
                            self.workflows.iter().enumerate().map(|(idx, (workflow_id, workflow))| {
                                let workflow_id = *workflow_id;
                                let is_selected = self.selected_workflow == Some(workflow_id);
                                
                                div()
                                    .w_full()
                                    .cursor_pointer()
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                        this.load_workflow(workflow_id, cx);
                                    }))
                                    .on_mouse_down(MouseButton::Right, cx.listener(move |this, _, _, cx| {
                                        // Simple delete confirmation via console for now
                                        // TODO: Add proper context menu or confirmation dialog
                                        println!("🗑️  Right-clicked workflow {} - Delete functionality placeholder", workflow_id);
                                        // For now, just delete immediately (in real app, would show confirmation)
                                        this.delete_workflow(workflow_id, cx);
                                    }))
                                    .child(
                                        ListItem::new(("workflow", idx))
                                            .spacing(ListItemSpacing::Dense)
                                            .toggle_state(is_selected)
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_1()
                                                    .child(
                                                        h_flex()
                                                            .items_center()
                                                            .justify_between()
                                                            .child(
                                                                Label::new(&workflow.name)
                                                                    .size(LabelSize::Default)
                                                            )
                                                            .child(
                                                                Label::new(&workflow.metadata.complexity)
                                                                    .size(LabelSize::Small)
                                                                    .color(match workflow.metadata.complexity.as_str() {
                                                                        "simple" => Color::Success,
                                                                        "complex" => Color::Error,
                                                                        _ => Color::Warning,
                                                                    })
                                                            )
                                                    )
                                                    .child(
                                                        Label::new(&workflow.description)
                                                            .size(LabelSize::Small)
                                                            .color(Color::Muted)
                                                    )
                                                    .child(
                                                        h_flex()
                                                            .gap_2()
                                                            .children(
                                                                workflow.tags.iter().map(|tag| {
                                                                    div()
                                                                        .px_2()
                                                                        .py_1()
                                                                        .bg(cx.theme().colors().surface_background)
                                                                        .rounded_md()
                                                                        .child(
                                                                            Label::new(tag)
                                                                                .size(LabelSize::XSmall)
                                                                                .color(Color::Muted)
                                                                        )
                                                                })
                                                            )
                                                    )
                                            )
                                    )
                            })
                        )
                }
            )
    }

    fn render_content(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_panel {
            WorkflowPanel::Canvas => {
                self.workflow_canvas.clone().into_any_element()
            }
            WorkflowPanel::List => {
                self.render_workflow_list(cx).into_any_element()
            }
        }
    }
}

impl Render for WorkflowManagerView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().colors().background)
            .track_focus(&self.focus_handle)
            .child(self.render_toolbar(cx))
            .child(
                div()
                    .flex_1()
                    .child(self.render_content(window, cx))
            )
    }
}

impl EventEmitter<()> for WorkflowManagerView {}

impl Focusable for WorkflowManagerView {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
} 