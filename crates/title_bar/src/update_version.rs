use auto_update::{AutoUpdateStatus, AutoUpdater, VersionCheckType};
use gpui::{Empty, EventEmitter, Render};
use ui::{UpdateButton, prelude::*};

pub enum UpdateVersionEvent {
    Dismissed,
}

/// Simulated auto-update states for testing/debugging
#[derive(Clone)]
pub enum SimulatedUpdateState {
    Checking,
    Downloading { version: String },
    Installing { version: String },
    Updated { version: String },
    Errored { error: String },
}

pub struct UpdateVersion {
    simulated_state: Option<SimulatedUpdateState>,
}

impl UpdateVersion {
    pub fn new() -> Self {
        Self {
            simulated_state: None,
        }
    }

    pub fn simulate_state(&mut self, state: SimulatedUpdateState) {
        self.simulated_state = Some(state);
    }

    pub fn clear_simulation(&mut self) {
        self.simulated_state = None;
    }

    pub fn is_simulating(&self) -> bool {
        self.simulated_state.is_some()
    }

    pub fn is_simulating_updated(&self) -> bool {
        matches!(
            self.simulated_state,
            Some(SimulatedUpdateState::Updated { .. })
        )
    }

    pub fn simulated_state(&self) -> Option<&SimulatedUpdateState> {
        self.simulated_state.as_ref()
    }

    fn version_tooltip_message(version: &VersionCheckType) -> String {
        format!("Version: {}", {
            match version {
                VersionCheckType::Sha(sha) => format!("{}…", sha.short()),
                VersionCheckType::Semantic(semantic_version) => semantic_version.to_string(),
            }
        })
    }
}

impl EventEmitter<UpdateVersionEvent> for UpdateVersion {}

impl Render for UpdateVersion {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(state) = &self.simulated_state {
            let is_simulating = true;
            return self
                .render_simulated_button(state.clone(), is_simulating, cx)
                .into_any_element();
        }

        let Some(auto_updater) = AutoUpdater::get(cx) else {
            return Empty.into_any_element();
        };

        let status = auto_updater.read(cx).status();

        match status {
            AutoUpdateStatus::Idle => Empty.into_any_element(),

            AutoUpdateStatus::Checking => UpdateButton::checking().into_any_element(),

            AutoUpdateStatus::Downloading { version } => {
                let tooltip = Self::version_tooltip_message(&version);
                UpdateButton::downloading(tooltip).into_any_element()
            }

            AutoUpdateStatus::Installing { version } => {
                let tooltip = Self::version_tooltip_message(&version);
                UpdateButton::installing(tooltip).into_any_element()
            }

            AutoUpdateStatus::Updated { version } => {
                let tooltip = Self::version_tooltip_message(&version);
                UpdateButton::updated(tooltip)
                    .on_click(|_, _, cx| {
                        workspace::reload(cx);
                    })
                    .on_dismiss(cx.listener(|_this, _, _window, cx| {
                        cx.emit(UpdateVersionEvent::Dismissed);
                    }))
                    .into_any_element()
            }

            AutoUpdateStatus::Errored { error } => {
                let error_str = error.to_string();
                UpdateButton::errored(error_str)
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(workspace::OpenLog), cx);
                    })
                    .on_dismiss(cx.listener(|_this, _, _window, cx| {
                        cx.emit(UpdateVersionEvent::Dismissed);
                    }))
                    .into_any_element()
            }
        }
    }
}

impl UpdateVersion {
    fn render_simulated_button(
        &self,
        state: SimulatedUpdateState,
        _is_simulating: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match state {
            SimulatedUpdateState::Checking => UpdateButton::new(
                IconName::ArrowCircle,
                "Checking for Zed updates… (simulated)",
            )
            .icon_animate(true),
            SimulatedUpdateState::Downloading { version } => {
                UpdateButton::new(IconName::Download, "Downloading Zed update… (simulated)")
                    .tooltip(format!("Version: {} (simulated)", version))
            }
            SimulatedUpdateState::Installing { version } => {
                UpdateButton::new(IconName::ArrowCircle, "Installing Zed update… (simulated)")
                    .icon_animate(true)
                    .tooltip(format!("Version: {} (simulated)", version))
            }
            SimulatedUpdateState::Updated { version } => {
                UpdateButton::updated(format!("Version: {} (simulated)", version)).on_dismiss(
                    cx.listener(|_this, _, _window, cx| {
                        cx.emit(UpdateVersionEvent::Dismissed);
                    }),
                )
            }
            SimulatedUpdateState::Errored { error } => {
                UpdateButton::errored(format!("{} (simulated)", error)).on_dismiss(cx.listener(
                    |_this, _, _window, cx| {
                        cx.emit(UpdateVersionEvent::Dismissed);
                    },
                ))
            }
        }
    }
}
