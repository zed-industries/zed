use crate::Project;
use gpui::{AnyWindowHandle, ModelContext, ModelHandle, WeakModelHandle};
use std::path::PathBuf;
use terminal::{Shell, Terminal, TerminalBuilder, TerminalSettings};

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
            let automatically_activate_python_virtual_environment = settings
                .automatically_activate_python_virtual_environment
                .clone();
            let shell = settings.shell.clone();

            let terminal = TerminalBuilder::new(
                working_directory.clone(),
                shell.clone(),
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

                if automatically_activate_python_virtual_environment {
                    let activate_script_path = self.find_activate_script_path(&shell, cx);
                    self.activate_python_virtual_environment(
                        activate_script_path,
                        &terminal_handle,
                        cx,
                    );
                }

                terminal_handle
            });

            terminal
        }
    }

    pub fn find_activate_script_path(
        &mut self,
        shell: &Shell,
        cx: &mut ModelContext<Project>,
    ) -> Option<PathBuf> {
        let program = match shell {
            terminal::Shell::System => "Figure this out",
            terminal::Shell::Program(program) => program,
            terminal::Shell::WithArguments { program, args } => program,
        };

        // This is so hacky - find a better way to do this
        let script_name = if program.contains("fish") {
            "activate.fish"
        } else {
            "activate"
        };

        let worktree_paths = self
            .worktrees(cx)
            .map(|worktree| worktree.read(cx).abs_path());

        const VIRTUAL_ENVIRONMENT_NAMES: [&str; 4] = [".env", "env", ".venv", "venv"];

        for worktree_path in worktree_paths {
            for virtual_environment_name in VIRTUAL_ENVIRONMENT_NAMES {
                let mut path = worktree_path.join(virtual_environment_name);
                path.push("bin/");
                path.push(script_name);

                if path.exists() {
                    return Some(path);
                }
            }
        }

        None
    }

    fn activate_python_virtual_environment(
        &mut self,
        activate_script: Option<PathBuf>,
        terminal_handle: &ModelHandle<Terminal>,
        cx: &mut ModelContext<Project>,
    ) {
        if let Some(activate_script) = activate_script {
            // Paths are not strings so we need to jump through some hoops to format the command without `format!`
            let mut command = Vec::from("source ".as_bytes());
            command.extend_from_slice(activate_script.as_os_str().as_bytes());
            command.push(b'\n');

            terminal_handle.update(cx, |this, _| this.input_bytes(command));
        }
    }

    pub fn local_terminal_handles(&self) -> &Vec<WeakModelHandle<terminal::Terminal>> {
        &self.terminals.local_handles
    }
}

// TODO: Add a few tests for adding and removing terminal tabs
