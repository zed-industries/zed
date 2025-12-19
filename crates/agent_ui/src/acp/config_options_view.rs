use acp_thread::AgentSessionConfigOptions;
use agent_servers::AgentServer;
use fs::Fs;
use gpui::{Context, Entity, Task, Window, prelude::*};
use std::rc::Rc;
use std::sync::Arc;
use ui::prelude::*;
use util::ResultExt as _;

use super::config_option_selector::ConfigOptionSelector;

pub struct ConfigOptionsView {
    config_options: Rc<dyn AgentSessionConfigOptions>,
    selectors: Vec<Entity<ConfigOptionSelector>>,
    agent_server: Rc<dyn AgentServer>,
    fs: Arc<dyn Fs>,
    _refresh_task: Task<()>,
}

impl ConfigOptionsView {
    pub fn new(
        config_options: Rc<dyn AgentSessionConfigOptions>,
        agent_server: Rc<dyn AgentServer>,
        fs: Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let selectors = Self::build_selectors(&config_options, &agent_server, &fs, window, cx);

        let rx = config_options.watch(cx);
        let refresh_task = cx.spawn_in(window, async move |this, cx| {
            if let Some(mut rx) = rx {
                while let Ok(()) = rx.recv().await {
                    this.update_in(cx, |this, window, cx| {
                        this.rebuild_selectors(window, cx);
                    })
                    .log_err();
                }
            }
        });

        Self {
            config_options,
            selectors,
            agent_server,
            fs,
            _refresh_task: refresh_task,
        }
    }

    fn build_selectors(
        config_options: &Rc<dyn AgentSessionConfigOptions>,
        agent_server: &Rc<dyn AgentServer>,
        fs: &Arc<dyn Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<Entity<ConfigOptionSelector>> {
        config_options
            .config_options()
            .into_iter()
            .map(|option| {
                let config_options = config_options.clone();
                let agent_server = agent_server.clone();
                let fs = fs.clone();
                cx.new(|cx| {
                    ConfigOptionSelector::new(
                        config_options,
                        option.id.clone(),
                        agent_server,
                        fs,
                        window,
                        cx,
                    )
                })
            })
            .collect()
    }

    fn rebuild_selectors(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.selectors = Self::build_selectors(
            &self.config_options,
            &self.agent_server,
            &self.fs,
            window,
            cx,
        );
        cx.notify();
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
