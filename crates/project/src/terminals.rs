use std::path::PathBuf;

use gpui::{ModelContext, ModelHandle, WeakModelHandle};
use settings::Settings;
use terminal::{Terminal, TerminalBuilder};

use crate::Project;

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakModelHandle<terminal::Terminal>>,
}

impl Project {
    pub fn create_terminal(
        &mut self,
        working_directory: Option<PathBuf>,
        window_id: usize,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<ModelHandle<Terminal>> {
        if self.is_remote() {
            return Err(anyhow::anyhow!(
                "creating terminals as a guest is not supported yet"
            ));
        } else {
            let settings = cx.global::<Settings>();
            let shell = settings.terminal_shell();
            let envs = settings.terminal_env();
            let scroll = settings.terminal_scroll();

            let terminal = TerminalBuilder::new(
                working_directory.clone(),
                shell,
                envs,
                settings.terminal_overrides.blinking.clone(),
                scroll,
                window_id,
            )
            .map(|builder| {
                let terminal_handle = cx.add_model(|cx| builder.subscribe(cx));

                self.terminals
                    .local_handles
                    .push(terminal_handle.downgrade());

                let id = terminal_handle.id();
                cx.observe_release(&terminal_handle, move |project, _terminal, _cx| {
                    let handles = &mut project.terminals.local_handles;

                    if let Some(index) = handles.iter().position(|terminal| terminal.id() == id) {
                        handles.remove(index);
                    }
                })
                .detach();

                terminal_handle
            });

            terminal
        }
    }
}

// TODO: Add a few tests for adding and removing terminal tabs
