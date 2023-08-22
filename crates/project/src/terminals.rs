use crate::Project;
use gpui::{AnyWindowHandle, ModelContext, ModelHandle, WeakModelHandle};
use std::path::PathBuf;
use terminal::{Terminal, TerminalBuilder, TerminalSettings};

#[cfg(target_os = "macos")]
use std::os::unix::ffi::OsStrExt;

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakModelHandle<terminal::Terminal>>,
}

impl Project {
    pub fn create_terminal(
        &mut self,
        working_directory: Option<PathBuf>,
        window: AnyWindowHandle,
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
                window,
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

                let setting = settings::get::<TerminalSettings>(cx);

                if setting.automatically_activate_python_virtual_environment {
                    self.set_up_python_virtual_environment(&terminal_handle, cx);
                }

                terminal_handle
            });

            terminal
        }
    }

    fn set_up_python_virtual_environment(
        &mut self,
        terminal_handle: &ModelHandle<Terminal>,
        cx: &mut ModelContext<Project>,
    ) {
        let virtual_environment = self.find_python_virtual_environment(cx);
        if let Some(virtual_environment) = virtual_environment {
            // Paths are not strings so we need to jump through some hoops to format the command without `format!`
            let mut command = Vec::from("source ".as_bytes());
            command.extend_from_slice(virtual_environment.as_os_str().as_bytes());
            command.push(b'\n');

            terminal_handle.update(cx, |this, _| this.input_bytes(command));
        }
    }

    pub fn find_python_virtual_environment(
        &mut self,
        cx: &mut ModelContext<Project>,
    ) -> Option<PathBuf> {
        const VIRTUAL_ENVIRONMENT_NAMES: [&str; 4] = [".env", "env", ".venv", "venv"];

        let worktree_paths = self
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path());

        for worktree_path in worktree_paths {
            for virtual_environment_name in VIRTUAL_ENVIRONMENT_NAMES {
                let mut path = worktree_path.join(virtual_environment_name);
                path.push("bin/activate");

                if path.exists() {
                    return Some(path);
                }
            }
        }

        None
    }

    pub fn local_terminal_handles(&self) -> &Vec<WeakModelHandle<terminal::Terminal>> {
        &self.terminals.local_handles
    }
}

// TODO: Add a few tests for adding and removing terminal tabs
