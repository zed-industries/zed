use std::{
    io::{LineWriter, Write},
    sync::Arc,
};

use anyhow::Result;
use futures::{channel::mpsc, StreamExt};
use gpui::{AppContext, AsyncApp, Context, Entity, Task, WeakEntity};
use mlua::{Lua, MultiValue};
use parking_lot::Mutex;
use project::Project;

use crate::print;

pub struct ScriptOutput {
    stdout: String,
}

pub struct Session {
    project: Entity<Project>,
    foreground_fns_tx: mpsc::Sender<Box<dyn FnOnce(WeakEntity<Self>, AsyncApp) + Send>>,
    _invoke_foreground_fns: Task<()>,
}

impl Session {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let (foreground_fns_tx, mut foreground_fns_rx) = mpsc::channel(128);
        Session {
            project,
            foreground_fns_tx,
            _invoke_foreground_fns: cx.spawn(|this, cx| async move {
                while let Some(foreground_fn) = foreground_fns_rx.next().await {
                    foreground_fn(this.clone(), cx.clone());
                }
            }),
        }
    }

    /// Runs a Lua script in a sandboxed environment and returns the printed lines
    fn run_script(&mut self, script: String, cx: &mut Context<Self>) -> Task<Result<ScriptOutput>> {
        const SANDBOX_PREAMBLE: &str = include_str!("sandbox_preamble.lua");

        cx.background_spawn(async move {
            let lua = Lua::new();
            lua.set_memory_limit(2 * 1024 * 1024 * 1024)?; // 2 GB
            let globals = lua.globals();
            let stdout = Arc::new(Mutex::new(String::new()));
            globals.set(
                "sb_print",
                lua.create_function({
                    let stdout = stdout.clone();
                    move |_, args: MultiValue| Self::print(args, &stdout)
                })?,
            )?;
            // globals.set("search", search(&lua, fs.clone(), root_dir.clone())?)?;
            // globals.set("sb_io_open", io_open(&lua, fs.clone(), root_dir)?)?;
            globals.set("user_script", script)?;

            lua.load(SANDBOX_PREAMBLE).exec()?;

            // Drop Lua instance to decrement reference count.
            drop(lua);

            let stdout = Arc::try_unwrap(stdout)
                .expect("no more references to stdout")
                .into_inner();
            Ok(ScriptOutput { stdout })
        })
    }

    /// Sandboxed print() function in Lua.
    fn print(args: MultiValue, stdout: &Mutex<String>) -> mlua::Result<()> {
        for (index, arg) in args.into_iter().enumerate() {
            // Lua's `print()` prints tab characters between each argument.
            if index > 0 {
                stdout.lock().push('\t');
            }

            // If the argument's to_string() fails, have the whole function call fail.
            stdout.lock().push_str(&arg.to_string()?);
        }
        stdout.lock().push('\n');

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use project::FakeFs;
    use settings::SettingsStore;

    use super::*;

    #[gpui::test]
    async fn test_print(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        let project = Project::test(fs, [], cx).await;
        let session = cx.new(|cx| Session::new(project, cx));
        let script = r#"
            print("Hello", "world!")
            print("Goodbye", "moon!")
        "#;
        let output = session
            .update(cx, |session, cx| session.run_script(script.to_string(), cx))
            .await
            .unwrap();
        assert_eq!(output.stdout, "Hello\tworld!\nGoodbye\tmoon!\n");
    }

    fn init_test(cx: &mut TestAppContext) {
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.update(Project::init_settings);
    }
}
