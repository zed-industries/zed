use acp_thread::AgentSessionConfigOptions;
use agent_client_protocol as acp;
use agent_servers::AgentServer;
use fs::Fs;
use gpui::{Context, Entity, FocusHandle, Window, prelude::*};
use std::rc::Rc;
use std::sync::Arc;
use ui::prelude::*;

use super::config_option_selector::ConfigOptionSelector;

pub struct ConfigOptionsView {
    config_options: Rc<dyn AgentSessionConfigOptions>,
    selectors: Vec<Entity<ConfigOptionSelector>>,
    #[allow(dead_code)]
    agent_server: Rc<dyn AgentServer>,
    #[allow(dead_code)]
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
}

impl ConfigOptionsView {
    pub fn new(
        config_options: Rc<dyn AgentSessionConfigOptions>,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
        focus_handle: FocusHandle,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let selectors = Self::build_selectors(&config_options, focus_handle.clone(), window, cx);

        Self {
            config_options,
            selectors,
            agent_server,
            fs,
            focus_handle,
        }
    }

    fn build_selectors(
        config_options: &Rc<dyn AgentSessionConfigOptions>,
        focus_handle: FocusHandle,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Entity<ConfigOptionSelector>> {
        config_options
            .config_options()
            .into_iter()
            .map(|option| {
                let config_options = config_options.clone();
                let focus_handle = focus_handle.clone();
                cx.new(|_cx| {
                    ConfigOptionSelector::new(config_options, option.id.clone(), focus_handle)
                })
            })
            .collect()
    }

    /// Rebuild selectors when config options change.
    /// This should be called when a `ConfigOptionsUpdated` event is received.
    pub fn rebuild_selectors(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.selectors =
            Self::build_selectors(&self.config_options, self.focus_handle.clone(), window, cx);
        cx.notify();
    }

    /// Update the config options provider and rebuild selectors.
    /// This is useful when the session config options are replaced entirely.
    pub fn update_config_options(
        &mut self,
        config_options: Rc<dyn AgentSessionConfigOptions>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config_options = config_options;
        self.rebuild_selectors(window, cx);
    }

    /// Get the current config options
    pub fn config_options(&self) -> Vec<acp::SessionConfigOption> {
        self.config_options.config_options()
    }

    /// Check if there are any config options to display
    pub fn is_empty(&self) -> bool {
        self.selectors.is_empty()
    }
}

impl Render for ConfigOptionsView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if self.selectors.is_empty() {
            return div().into_any_element();
        }

        h_flex()
            .gap_1()
            .children(self.selectors.iter().cloned())
            .into_any_element()
    }
}
