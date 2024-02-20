use std::sync::Arc;

use gpui::{AppContext, Model};
use runnable::{Runnable, RunnableId, Source};
use settings::Settings;
use terminal::terminal_settings::{Shell, TerminalSettings};
use ui::Context;

pub struct OneshotSource {
    runnables: Vec<Arc<dyn runnable::Runnable>>,
}

#[derive(Clone)]
struct OneshotRunnable {
    id: RunnableId,
    shell: String,
    args: Vec<String>,
}

impl OneshotRunnable {
    fn new(prompt: String, cx: &mut AppContext) -> Option<Self> {
        let (shell, args) = match TerminalSettings::get_global(cx).shell.clone() {
            Shell::System => std::env::var("SHELL").ok().map(|shell| (shell, vec![])),
            Shell::Program(shell) => Some((shell, vec![])),
            Shell::WithArguments { program, args } => Some((program, args)),
        }?;

        Some(Self {
            id: RunnableId(prompt),
            shell,
            args,
        })
    }
}

impl Runnable for OneshotRunnable {
    fn id(&self) -> &runnable::RunnableId {
        &self.id
    }

    fn name(&self) -> &str {
        &self.id.0
    }

    fn cwd(&self) -> Option<&std::path::Path> {
        None
    }

    fn exec(&self, cwd: Option<std::path::PathBuf>) -> Option<runnable::SpawnInTerminal> {
        let escaped = self.id().0.clone();
        if escaped.is_empty() {
            return None;
        }
        let command = self.shell.clone();
        let mut args = self.args.clone();
        args.extend(["-c".to_owned(), escaped]);
        Some(runnable::SpawnInTerminal {
            id: self.id().clone(),
            label: self.name().to_owned(),
            command,
            args,
            cwd,
            env: Default::default(),
            use_new_terminal: Default::default(),
            allow_concurrent_runs: Default::default(),
        })
    }
}

impl OneshotSource {
    pub fn new(cx: &mut AppContext) -> Model<Box<dyn Source>> {
        cx.new_model(|_| Box::new(Self { runnables: vec![] }) as Box<dyn Source>)
    }

    pub fn spawn(
        &mut self,
        prompt: String,
        cx: &mut AppContext,
    ) -> Option<Arc<dyn runnable::Runnable>> {
        let ret = Arc::new(OneshotRunnable::new(prompt, cx)?);
        self.runnables.push(ret.clone());
        Some(ret)
    }
}

impl Source for OneshotSource {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn runnables_for_path(
        &mut self,
        _path: Option<&std::path::Path>,
        _cx: &mut gpui::ModelContext<Box<dyn Source>>,
    ) -> Vec<Arc<dyn runnable::Runnable>> {
        self.runnables.clone()
    }
}
