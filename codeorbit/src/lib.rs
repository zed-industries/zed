//! CodeOrbit - AI-Powered Development Assistant for Zed
//! 
//! This crate provides AI-powered development assistance within the Zed editor,
//! featuring a multi-agent system for handling various development tasks.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

use std::sync::Arc;
use zed::{
    AppContext, Command, Context, Global, View, ViewContext, WindowContext, WindowHandle,
};

mod core;
mod ui;
mod agents;

use crate::core::Orchestrator;
use crate::ui::PromptPanel;

/// The main state of the CodeOrbit extension.
pub struct CodeOrbit {
    orchestrator: Arc<tokio::sync::Mutex<Orchestrator>>,
    panel: Option<View<PromptPanel>>,
}

impl Default for CodeOrbit {
    fn default() -> Self {
        Self {
            orchestrator: Arc::new(tokio::sync::Mutex::new(Orchestrator::new())),
            panel: None,
        }
    }
}

impl Global for CodeOrbit {
    fn new() -> Self {
        Self::default()
    }

    fn initialize(&mut self, _: &mut AppContext) {
        // Initialize the orchestrator
        let orchestrator = self.orchestrator.clone();
        tokio::spawn(async move {
            if let Ok(mut orch) = orchestrator.lock().await {
                if let Err(e) = orch.initialize().await {
                    log::error!("Failed to initialize orchestrator: {}", e);
                }
            }
        });
    }
}

impl CodeOrbit {
    /// Toggles the visibility of the CodeOrbit panel.
    pub fn toggle_panel(
        &mut self,
        _: &Command,
        cx: &mut ViewContext<Self>,
    ) {
        if self.panel.is_none() {
            let orchestrator = self.orchestrator.clone();
            let panel = cx.new_view(move |_cx| PromptPanel::new(orchestrator));
            self.panel = Some(panel);
        } else {
            self.panel = None;
        }
    }
}

/// Registers the CodeOrbit extension with Zed.
#[no_mangle]
pub fn init(_: &zed::AppContext) {
    // Initialize logging
    env_logger::init();
    
    // Register commands
    zed::register_global(CodeOrbit::new);
    
    zed::register_action("toggle_codeorbit_panel", |_cx| {
        Box::new(|cx: &mut WindowContext| {
            if let Some(global) = cx.global::<CodeOrbit>() {
                cx.dispatch_action(global, |codeorbit, cx| {
                    codeorbit.toggle_panel(&Default::default(), cx);
                });
            }
        })
    });
    
    log::info!("CodeOrbit extension initialized");
}

/// A simple test function to verify the extension is loaded.
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extension_initialization() {
        // This is a simple test to verify the extension can be initialized
        let codeorbit = CodeOrbit::default();
        assert!(codeorbit.panel.is_none());
    }
}
