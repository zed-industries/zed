pub mod components;
mod jupyter_settings;
pub mod kernels;
pub mod notebook;
mod outputs;
mod repl_editor;
mod repl_sessions_ui;
mod repl_store;
mod session;

use std::{sync::Arc, time::Duration};

use async_dispatcher::{Dispatcher, Runnable, set_dispatcher};
use gpui::{App, PlatformDispatcher};
use project::Fs;
pub use runtimelib::ExecutionState;
use settings::Settings as _;

pub use crate::jupyter_settings::JupyterSettings;
pub use crate::kernels::{Kernel, KernelSpecification, KernelStatus};
pub use crate::repl_editor::*;
pub use crate::repl_sessions_ui::{
    ClearOutputs, Interrupt, ReplSessionsPage, Restart, Run, Sessions, Shutdown,
};
use crate::repl_store::ReplStore;
pub use crate::session::Session;

pub const KERNEL_DOCS_URL: &str = "https://CodeOrbit.dev/docs/repl#changing-kernels";

pub fn init(fs: Arc<dyn Fs>, cx: &mut App) {
    set_dispatcher(codeorbit_dispatcher(cx));
    JupyterSettings::register(cx);
    ::editor::init_settings(cx);
    repl_sessions_ui::init(cx);
    ReplStore::init(fs, cx);
}

fn codeorbit_dispatcher(cx: &mut App) -> impl Dispatcher {
    struct CodeOrbitDispatcher {
        dispatcher: Arc<dyn PlatformDispatcher>,
    }

    // PlatformDispatcher is _super_ close to the same interface we put in
    // async-dispatcher, except for the task label in dispatch. Later we should
    // just make that consistent so we have this dispatcher ready to go for
    // other crates in CodeOrbit.
    impl Dispatcher for CodeOrbitDispatcher {
        fn dispatch(&self, runnable: Runnable) {
            self.dispatcher.dispatch(runnable, None)
        }

        fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
            self.dispatcher.dispatch_after(duration, runnable);
        }
    }

    CodeOrbitDispatcher {
        dispatcher: cx.background_executor().dispatcher.clone(),
    }
}
