use std::collections::HashMap;

use gpui::{
    Action as _, App, Context, Entity, IntoElement, Render, Subscription, Task, WeakEntity, Window,
};
use project::{Event as ProjectEvent, Project};
use task::TaskId;
use tasks_ui::{
    Rerun, RunConfiguration, RunConfigurationId, discover_run_configurations,
    load_selected_run_configuration, persist_selected_run_configuration, task_contexts,
};
use terminal::TaskStatus;
use ui::{
    Button, ButtonSize, ButtonStyle, ContextMenu, Icon, IconButton, IconButtonShape, IconName,
    IconPosition, IconSize, PopoverMenu, Tooltip, prelude::*,
};
use util::{ResultExt as _, truncate_and_trailoff};
use workspace::{Toast, Workspace, notifications::NotificationId};

pub struct RunConfigurationsToolbar {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    configurations: Vec<RunConfiguration>,
    selected: Option<RunConfigurationId>,
    task_ids: HashMap<RunConfigurationId, TaskId>,
    _terminal_subscriptions: Vec<Subscription>,
    _subscriptions: Vec<Subscription>,
    _discovery_task: Task<()>,
}

impl RunConfigurationsToolbar {
    pub fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let workspace_entity = workspace.weak_handle();
        let project = workspace.project().clone();
        let selected = load_selected_run_configuration(workspace.database_id(), cx);
        let mut subscriptions = Vec::new();

        if let Some(task_inventory) = project
            .read(cx)
            .task_store()
            .read(cx)
            .task_inventory()
            .cloned()
        {
            subscriptions.push(cx.observe(&task_inventory, |this, _, cx| {
                this.refresh(cx);
            }));
        }

        subscriptions.push(cx.subscribe(&project, |this, _, event, cx| {
            if matches!(
                event,
                ProjectEvent::WorktreeAdded(_)
                    | ProjectEvent::WorktreeOrderChanged
                    | ProjectEvent::WorktreeRemoved(_)
            ) {
                this.refresh(cx);
            }
        }));
        subscriptions.push(cx.observe(&project, |this, _, cx| {
            this.sync_terminal_subscriptions(cx);
            cx.notify();
        }));

        let mut this = Self {
            workspace: workspace_entity,
            project,
            configurations: Vec::new(),
            selected,
            task_ids: HashMap::new(),
            _terminal_subscriptions: Vec::new(),
            _subscriptions: subscriptions,
            _discovery_task: Task::ready(()),
        };
        this.refresh(cx);
        this.sync_terminal_subscriptions(cx);
        this
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        let project = self.project.clone();
        self._discovery_task = cx.spawn(async move |this, cx| {
            let discovery = cx.update(|cx| discover_run_configurations(project, cx));
            let configurations = discovery.await;
            this.update(cx, |this, cx| {
                this.configurations = configurations;
                let selected_exists = this
                    .selected
                    .as_ref()
                    .is_some_and(|selected| this.configuration(selected).is_some());
                if !selected_exists {
                    this.selected = this
                        .configurations
                        .first()
                        .map(|configuration| configuration.id.clone());
                    this.persist_selection(cx);
                }
                cx.notify();
            })
            .log_err();
        });
    }

    fn sync_terminal_subscriptions(&mut self, cx: &mut Context<Self>) {
        let terminals = self
            .project
            .read(cx)
            .local_terminal_handles()
            .iter()
            .filter_map(WeakEntity::upgrade)
            .collect::<Vec<_>>();
        self._terminal_subscriptions = terminals
            .into_iter()
            .map(|terminal| cx.observe(&terminal, |_, _, cx| cx.notify()))
            .collect();
    }

    fn configuration(&self, id: &RunConfigurationId) -> Option<&RunConfiguration> {
        self.configurations
            .iter()
            .find(|configuration| &configuration.id == id)
    }

    fn selected_configuration(&self) -> Option<&RunConfiguration> {
        self.selected
            .as_ref()
            .and_then(|selected| self.configuration(selected))
    }

    fn select(&mut self, id: RunConfigurationId, cx: &mut Context<Self>) {
        self.selected = Some(id);
        self.persist_selection(cx);
        cx.notify();
    }

    fn persist_selection(&self, cx: &App) {
        let workspace_id = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace.read(cx).database_id());
        if let Some(selected) = self.selected.clone() {
            persist_selected_run_configuration(workspace_id, selected, cx);
        }
    }

    fn selected_task_id(&self) -> Option<&TaskId> {
        self.selected
            .as_ref()
            .and_then(|selected| self.task_ids.get(selected))
    }

    fn selected_task_status(&self, cx: &App) -> Option<TaskStatus> {
        let task_id = self.selected_task_id()?;
        self.project
            .read(cx)
            .local_terminal_handles()
            .iter()
            .filter_map(WeakEntity::upgrade)
            .filter_map(|terminal| {
                terminal
                    .read(cx)
                    .task()
                    .and_then(|task| (&task.spawned_task.id == task_id).then_some(task.status))
            })
            .max_by_key(|status| match status {
                TaskStatus::Running => 2,
                TaskStatus::Completed { .. } => 1,
                TaskStatus::Unknown => 0,
            })
    }

    fn run(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(configuration) = self.selected_configuration().cloned() else {
            return;
        };
        let Some(task_contexts) = self
            .workspace
            .update(cx, |workspace, cx| task_contexts(workspace, window, cx))
            .log_err()
        else {
            return;
        };
        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |this, cx| {
            let task_contexts = task_contexts.await;
            let Some(resolved_task) = configuration.resolve(&task_contexts) else {
                workspace
                    .update(cx, |workspace, cx| {
                        workspace.show_toast(
                            Toast::new(
                                NotificationId::named("run-configuration-resolution".into()),
                                format!(
                                    "Cannot run '{}': its task variables are unavailable",
                                    configuration.task_template.label
                                ),
                            ),
                            cx,
                        );
                    })
                    .log_err();
                return;
            };
            let task_id = resolved_task.id.clone();
            if workspace
                .update_in(cx, |workspace, window, cx| {
                    workspace.schedule_resolved_task(
                        configuration.task_source_kind,
                        resolved_task,
                        false,
                        window,
                        cx,
                    );
                })
                .log_err()
                .is_some()
            {
                this.update(cx, |this, cx| {
                    this.task_ids.insert(configuration.id, task_id);
                    cx.notify();
                })
                .log_err();
            }
        })
        .detach();
    }

    fn stop(&mut self, cx: &mut Context<Self>) {
        let Some(task_id) = self.selected_task_id().cloned() else {
            return;
        };
        self.project.update(cx, |project, cx| {
            project.kill_terminal_task(&task_id, cx);
        });
    }

    fn restart(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(task_id) = self.selected_task_id() else {
            return;
        };
        window.dispatch_action(
            Box::new(Rerun {
                task_id: Some(task_id.0.clone()),
                allow_concurrent_runs: Some(true),
                use_new_terminal: Some(false),
                reevaluate_context: false,
            }),
            cx,
        );
    }

    fn build_context_menu(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let configurations = self.configurations.clone();
        let selected = self.selected.clone();
        let this = cx.weak_entity();
        ContextMenu::build(window, cx, move |mut menu, _, _| {
            for configuration in configurations {
                let is_selected = selected.as_ref() == Some(&configuration.id);
                let this = this.clone();
                menu = menu.toggleable_entry(
                    configuration.task_template.label.clone(),
                    is_selected,
                    IconPosition::Start,
                    None,
                    move |_, cx| {
                        this.update(cx, |this, cx| {
                            this.select(configuration.id.clone(), cx);
                        })
                        .log_err();
                    },
                );
            }
            menu
        })
    }

    fn build_configuration_menu(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        ContextMenu::build(window, cx, |menu, _, _| {
            menu.action(
                "Edit Project Tasks",
                zed_actions::OpenProjectTasks.boxed_clone(),
            )
            .action("Edit Global Tasks", zed_actions::OpenTasks.boxed_clone())
            .separator()
            .action("Create Task", zed_actions::CreateTask.boxed_clone())
        })
    }
}

impl Render for RunConfigurationsToolbar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_label = self
            .selected_configuration()
            .map(|configuration| truncate_and_trailoff(&configuration.task_template.label, 32))
            .unwrap_or_else(|| "No run configurations".to_string());
        let has_configuration = self.selected_configuration().is_some();
        let task_status = self.selected_task_status(cx);
        let is_running = task_status == Some(TaskStatus::Running);
        let can_restart = self.selected_task_id().is_some();
        let this = cx.weak_entity();
        let menu_this = this.clone();

        h_flex()
            .gap_0p5()
            .child(
                PopoverMenu::new("run-configuration-selector")
                    .trigger(
                        Button::new("run-configuration-trigger", selected_label)
                            .label_size(LabelSize::Small)
                            .selected_style(ButtonStyle::Tinted(ui::TintColor::Accent))
                            .end_icon(
                                Icon::new(IconName::ChevronDown)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted),
                            )
                            .disabled(!has_configuration),
                    )
                    .anchor(gpui::Anchor::TopRight)
                    .menu(move |window, cx| {
                        this.update(cx, |this, cx| this.build_context_menu(window, cx))
                            .log_err()
                    }),
            )
            .child(
                IconButton::new("run-configuration-run", IconName::PlayFilled)
                    .shape(IconButtonShape::Square)
                    .size(ButtonSize::Compact)
                    .aria_label("Run configuration")
                    .disabled(!has_configuration)
                    .tooltip(|_, cx| Tooltip::simple("Run", cx))
                    .on_click(cx.listener(|this, _, window, cx| this.run(window, cx))),
            )
            .child(
                IconButton::new("run-configuration-stop", IconName::Stop)
                    .shape(IconButtonShape::Square)
                    .size(ButtonSize::Compact)
                    .aria_label("Stop configuration")
                    .disabled(!is_running)
                    .tooltip(|_, cx| Tooltip::simple("Stop", cx))
                    .on_click(cx.listener(|this, _, _, cx| this.stop(cx))),
            )
            .child(
                IconButton::new("run-configuration-restart", IconName::Rerun)
                    .shape(IconButtonShape::Square)
                    .size(ButtonSize::Compact)
                    .aria_label("Restart configuration")
                    .disabled(!can_restart)
                    .tooltip(|_, cx| Tooltip::simple("Restart", cx))
                    .on_click(cx.listener(|this, _, window, cx| this.restart(window, cx))),
            )
            .child(
                PopoverMenu::new("run-configuration-menu")
                    .trigger(
                        IconButton::new("run-configuration-menu-trigger", IconName::Ellipsis)
                            .shape(IconButtonShape::Square)
                            .size(ButtonSize::Compact)
                            .aria_label("Run configuration menu")
                            .tooltip(|_, cx| Tooltip::simple("Run Configuration Menu", cx)),
                    )
                    .anchor(gpui::Anchor::TopRight)
                    .menu(move |window, cx| {
                        menu_this
                            .update(cx, |this, cx| this.build_configuration_menu(window, cx))
                            .log_err()
                    }),
            )
    }
}
