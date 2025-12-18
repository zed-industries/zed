use configuration::{ConfigurationTemplate, ConfigurationType};
use gpui::{
    Context, EventEmitter, IntoElement, ParentElement, Render, Styled, Subscription, WeakEntity,
    Window,
};
use project::ConfigurationSourceKind;
use std::path::PathBuf;
use task::SpawnInTerminal;
use ui::{prelude::*, ContextMenu, DropdownMenu, IconName, Tooltip};
use workspace::{ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace};

pub struct ConfigurationsToolbar {
    workspace: WeakEntity<Workspace>,
    selected_configuration: Option<(ConfigurationSourceKind, ConfigurationTemplate)>,
    available_configurations: Vec<(ConfigurationSourceKind, ConfigurationTemplate)>,
    _subscriptions: Vec<Subscription>,
}

impl ConfigurationsToolbar {
    pub fn new(workspace: WeakEntity<Workspace>, cx: &mut Context<Self>) -> Self {
        let toolbar = Self {
            workspace: workspace.clone(),
            selected_configuration: None,
            available_configurations: Vec::new(),
            _subscriptions: Vec::new(),
        };
        
        // Defer subscription setup to avoid reading workspace while it's being updated
        let weak_self = cx.weak_entity();
        cx.spawn(async move |_, cx| {
            cx.update(|cx| {
                if let Some(this) = weak_self.upgrade() {
                    this.update(cx, |this, cx| {
                        this.setup_subscriptions(cx);
                        this.load_configurations(cx);
                    });
                }
            }).ok();
        }).detach();
        
        toolbar
    }
    
    fn setup_subscriptions(&mut self, cx: &mut Context<Self>) {
        // Subscribe to global configuration selection changes
        self._subscriptions.push(cx.observe_global::<crate::SelectedConfiguration>(
            |toolbar, cx| {
                let selected = cx.global::<crate::SelectedConfiguration>().selection.clone();
                if let Some((source, template)) = selected {
                    log::info!("Toolbar: Configuration changed to '{}'", template.label);
                    toolbar.selected_configuration = Some((source, (*template).clone()));
                    cx.notify();
                }
            },
        ));
        
        // Subscribe to project changes to reload configurations
        if let Some(workspace_entity) = self.workspace.upgrade() {
            let project = workspace_entity.read(cx).project().clone();
            
            // Clone entities before observing to avoid borrow checker issues
            let config_store_opt = project.read(cx).configuration_store().cloned();
            let inventory_opt = config_store_opt.as_ref()
                .and_then(|store| store.read(cx).configuration_inventory().cloned());
            
            self._subscriptions.push(cx.observe(&project, |toolbar, _project, cx| {
                toolbar.load_configurations(cx);
            }));
            
            // Also observe the configuration store if available
            if let Some(config_store) = config_store_opt {
                self._subscriptions.push(cx.observe(&config_store, |toolbar, _store, cx| {
                    toolbar.load_configurations(cx);
                }));
            }
            
            // Observe the inventory if available
            if let Some(inventory) = inventory_opt {
                self._subscriptions.push(cx.observe(&inventory, |toolbar, _inventory, cx| {
                    toolbar.load_configurations(cx);
                }));
            }
        }
    }

    fn load_configurations(&mut self, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            let project = workspace.read(cx).project().clone();
            if let Some(config_store) = project.read(cx).configuration_store() {
                if let Some(inventory) = config_store.read(cx).configuration_inventory() {
                    // Collect configurations from all worktrees
                    let mut all_configurations = Vec::new();
                    
                    // Get worktrees from the project
                    let worktrees: Vec<_> = project.read(cx).worktrees(cx).collect();
                    
                    if worktrees.is_empty() {
                        // No worktrees, just get global configurations
                        all_configurations = inventory.read(cx).list_configurations(None);
                    } else {
                        // Get configurations for each worktree (which includes global configs too)
                        // We'll deduplicate by using a set to track seen configurations
                        use std::collections::HashSet;
                        let mut seen = HashSet::new();
                        
                        for worktree in worktrees {
                            let worktree_id = worktree.read(cx).id();
                            let configs = inventory.read(cx).list_configurations(Some(worktree_id));
                            for config in configs {
                                // Use label as a simple deduplication key
                                let key = config.1.label.to_string();
                                if seen.insert(key) {
                                    all_configurations.push(config);
                                }
                            }
                        }
                    }
                    
                    self.available_configurations = all_configurations;
                    
                    // Auto-select first configuration if none selected
                    if self.selected_configuration.is_none() && !self.available_configurations.is_empty() {
                        self.selected_configuration = Some(self.available_configurations[0].clone());
                    }
                    
                    cx.notify();
                }
            }
        }
    }

    fn run_configuration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        log::info!("Run configuration clicked");
        if let Some((_source, template)) = &self.selected_configuration {
            log::info!("Running configuration: {}", template.label);
            if let Some(workspace) = self.workspace.upgrade() {
                let template = template.clone();
                workspace.update(cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    
                    // All configurations must have a recipe
                    let recipe_name = &template.recipe;
                    log::info!("Using recipe: {}", recipe_name);
                    
                    // Determine the command to run using the recipe
                    let (command, args) = {
                        let Some(config_store) = project.read(cx).configuration_store() else {
                            log::error!("Configuration store not available");
                            return;
                        };
                        
                        let Some(recipe_store) = config_store.read(cx).recipe_store() else {
                            log::error!("Recipe store not available");
                            return;
                        };
                        
                        // Get worktree ID for recipe lookup
                        let worktree_id = project.read(cx).worktrees(cx).next()
                            .map(|wt| wt.read(cx).id());
                        
                        let Some(recipe) = recipe_store.read(cx).get_recipe(recipe_name, worktree_id) else {
                            log::error!("Recipe '{}' not found", recipe_name);
                            return;
                        };
                        
                        log::info!("Found recipe, rendering with variables: {:?}", template.variables);
                        
                        match recipe.render_run(&template.variables) {
                            Ok(rendered_command) => {
                                log::info!("Rendered command: {}", rendered_command);
                                // Parse the rendered command into command and args
                                let parts: Vec<&str> = rendered_command.split_whitespace().collect();
                                if parts.is_empty() {
                                    log::error!("Empty command from recipe '{}'", recipe_name);
                                    return;
                                }
                                (parts[0].to_string(), parts[1..].iter().map(|s| s.to_string()).collect::<Vec<String>>())
                            }
                            Err(err) => {
                                log::error!("Failed to render recipe '{}': {}", recipe_name, err);
                                return;
                            }
                        }
                    };
                    
                    let command_label = if args.is_empty() {
                        command.clone()
                    } else {
                        format!("{} {}", command, args.join(" "))
                    };
                    
                    let spawn_in_terminal = SpawnInTerminal {
                        id: task::TaskId(template.label.clone().into()),
                        full_label: template.label.clone(),
                        label: template.label.clone(),
                        command: Some(command),
                        args,
                        command_label,
                        cwd: template.cwd.as_ref().map(|s| PathBuf::from(s)),
                        env: template.env.clone(),
                        use_new_terminal: false,
                        allow_concurrent_runs: false,
                        reveal: task::RevealStrategy::default(),
                        reveal_target: task::RevealTarget::default(),
                        hide: task::HideStrategy::default(),
                        shell: task::Shell::default(),
                        show_summary: true,
                        show_command: true,
                        show_rerun: true,
                    };
                    
                    workspace.spawn_in_terminal(spawn_in_terminal, window, cx).detach();
                });
            }
        }
    }

    fn debug_configuration(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        log::info!("Debug configuration clicked");
        if let Some((_source, template)) = &self.selected_configuration {
            log::info!("Debugging configuration: {}", template.label);
            
            if let Some(workspace) = self.workspace.upgrade() {
                let template = template.clone();
                workspace.update(cx, |workspace, cx| {
                    let project = workspace.project().clone();
                    
                    // All configurations must have a recipe
                    let recipe_name = &template.recipe;
                    log::info!("Using recipe for debug: {}", recipe_name);
                    
                    // Determine the debug command to run using the recipe
                    let (command, args) = {
                        let Some(config_store) = project.read(cx).configuration_store() else {
                            log::error!("Configuration store not available");
                            return;
                        };
                        
                        let Some(recipe_store) = config_store.read(cx).recipe_store() else {
                            log::error!("Recipe store not available");
                            return;
                        };
                        
                        let worktree_id = project.read(cx).worktrees(cx).next()
                            .map(|wt| wt.read(cx).id());
                        
                        let Some(recipe) = recipe_store.read(cx).get_recipe(recipe_name, worktree_id) else {
                            log::error!("Recipe '{}' not found", recipe_name);
                            return;
                        };
                        
                        log::info!("Found recipe for debug, rendering with variables: {:?}", template.variables);
                        
                        match recipe.render_debug(&template.variables) {
                            Ok(rendered_command) => {
                                log::info!("Rendered debug command: {}", rendered_command);
                                let parts: Vec<&str> = rendered_command.split_whitespace().collect();
                                if parts.is_empty() {
                                    log::error!("Empty debug command from recipe '{}'", recipe_name);
                                    return;
                                }
                                (parts[0].to_string(), parts[1..].iter().map(|s| s.to_string()).collect::<Vec<String>>())
                            }
                            Err(err) => {
                                log::error!("Failed to render debug recipe '{}': {}", recipe_name, err);
                                return;
                            }
                        }
                    };
                    
                    let command_label = if args.is_empty() {
                        command.clone()
                    } else {
                        format!("{} {}", command, args.join(" "))
                    };
                    
                    let spawn_in_terminal = SpawnInTerminal {
                        id: task::TaskId(template.label.clone().into()),
                        full_label: template.label.clone(),
                        label: format!("{} (Debug)", template.label),
                        command: Some(command),
                        args,
                        command_label,
                        cwd: template.cwd.as_ref().map(|s| PathBuf::from(s)),
                        env: template.env.clone(),
                        use_new_terminal: false,
                        allow_concurrent_runs: false,
                        reveal: task::RevealStrategy::default(),
                        reveal_target: task::RevealTarget::default(),
                        hide: task::HideStrategy::default(),
                        shell: task::Shell::default(),
                        show_summary: true,
                        show_command: true,
                        show_rerun: true,
                    };
                    
                    workspace.spawn_in_terminal(spawn_in_terminal, window, cx).detach();
                });
            }
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for ConfigurationsToolbar {}

impl ToolbarItemView for ConfigurationsToolbar {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn workspace::ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        // Load configurations on first render or when pane changes
        if self.available_configurations.is_empty() {
            self.load_configurations(cx);
        }
        ToolbarItemLocation::PrimaryRight
    }
}

impl Render for ConfigurationsToolbar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_label = self
            .selected_configuration
            .as_ref()
            .map(|(_, config)| config.label.to_string())
            .unwrap_or_else(|| "No configuration".to_string());

        let has_selection = self.selected_configuration.is_some();
        
        // Build the dropdown menu
        let weak = cx.weak_entity();
        let configurations = self.available_configurations.clone();
        
        let dropdown_menu = ContextMenu::build(window, cx, move |mut menu, _, _cx| {
            if configurations.is_empty() {
                menu = menu.entry("No configurations available", None, |_, _| {});
            } else {
                for (source, config) in configurations {
                    let weak = weak.clone();
                    let source = source.clone();
                    let config = config.clone();
                    menu = menu.entry(config.label.clone(), None, move |_, cx| {
                        if let Some(this) = weak.upgrade() {
                            this.update(cx, |this, cx| {
                                this.selected_configuration = Some((source.clone(), config.clone()));
                                crate::set_selected_configuration(source.clone(), config.clone(), cx);
                                cx.notify();
                            });
                        }
                    });
                }
            }
            menu
        });

        h_flex()
            .gap_2()
            .child(
                // Configuration dropdown
                DropdownMenu::new("select-configuration", selected_label, dropdown_menu)
                    .style(ui::DropdownStyle::Subtle)
            )
            .child(
                // Separator/spacer
                div().w_1()
            )
            .child(
                // Run button
                IconButton::new("run-configuration", IconName::PlayOutlined)
                    .icon_color(Color::Success)
                    .style(ButtonStyle::Filled)
                    .disabled(!has_selection)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.run_configuration(window, cx);
                    }))
                    .tooltip(|window, cx| Tooltip::text("Run configuration")(window, cx)),
            )
            .child(
                // Debug button
                IconButton::new("debug-configuration", IconName::Debug)
                    .icon_color(Color::Warning)
                    .style(ButtonStyle::Filled)
                    .disabled(!has_selection)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.debug_configuration(window, cx);
                    }))
                    .tooltip(|window, cx| Tooltip::text("Debug configuration")(window, cx)),
            )
            .child(
                // Extra spacer before sign-in button
                div().w_4()
            )
    }
}
