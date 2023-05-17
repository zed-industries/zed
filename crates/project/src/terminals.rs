use crate::Project;
use gpui::{ModelContext, ModelHandle, WeakModelHandle};
use std::path::PathBuf;
use terminal::{Terminal, TerminalBuilder, TerminalSettings};

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
            let settings = settings::get::<TerminalSettings>(cx);

            let terminal = TerminalBuilder::new(
                working_directory.clone(),
                settings.shell.clone(),
                settings.env.clone(),
                Some(settings.blinking.clone()),
                settings.alternate_scroll,
                window_id,
            )
            .map(|builder| {
                let terminal_handle = cx.add_model(|cx| builder.subscribe(cx));

                self.terminals
                    .local_handles
                    .push(terminal_handle.downgrade());

                let id = terminal_handle.id();
                cx.observe_release(&terminal_handle, move |project, _terminal, cx| {
                    let handles = &mut project.terminals.local_handles;

                    if let Some(index) = handles.iter().position(|terminal| terminal.id() == id) {
                        handles.remove(index);
                        cx.notify();
                    }
                })
                .detach();

                terminal_handle
            });

            terminal
        }
    }

    pub fn local_terminal_handles(&self) -> &Vec<WeakModelHandle<terminal::Terminal>> {
        &self.terminals.local_handles
    }
}

// TODO: Add a few tests for adding and removing terminal tabs
