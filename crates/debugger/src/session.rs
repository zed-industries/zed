// crates/debugger/src/session.rs

use crate::dap_client::*;
use anyhow::Result;
use collections::HashMap;
use futures::channel::mpsc;
use gpui::{AppContext, EventEmitter, Model, ModelContext};
use std::{path::PathBuf, sync::Arc};

pub struct DebugSession {
    pub id: usize,
    pub name: String,
    pub state: DebugSessionState,
    pub client: Option<Arc<DapClient>>,
    pub breakpoints: HashMap<PathBuf, Vec<BreakpointState>>,
    pub threads: Vec<Thread>,
    pub current_thread_id: Option<i64>,
    pub stack_frames: HashMap<i64, Vec<StackFrame>>,
    pub variables: HashMap<i64, Vec<Variable>>,
    pub watch_expressions: Vec<WatchExpression>,
    pub console_output: Vec<ConsoleMessage>,
}

#[derive(Debug, Clone)]
pub struct BreakpointState {
    pub line: u32,
    pub verified: bool,
    pub condition: Option<String>,
    pub hit_condition: Option<String>,
    pub log_message: Option<String>,
    pub hit_count: u32,
}

#[derive(Debug, Clone)]
pub struct WatchExpression {
    pub id: usize,
    pub expression: String,
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    pub category: ConsoleCategory,
    pub message: String,
    pub timestamp: std::time::Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConsoleCategory {
    Stdout,
    Stderr,
    Console,
    Debug,
    Error,
}

#[derive(Debug, Clone)]
pub enum DebugSessionEvent {
    StateChanged(DebugSessionState),
    BreakpointHit { thread_id: i64, breakpoint_ids: Vec<i64> },
    Stopped { thread_id: i64, reason: StopReason },
    ThreadStarted { thread_id: i64 },
    ThreadExited { thread_id: i64 },
    OutputReceived(ConsoleMessage),
    VariablesUpdated,
    BreakpointsChanged(PathBuf),
    Terminated,
}

impl EventEmitter<DebugSessionEvent> for DebugSession {}

impl DebugSession {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            state: DebugSessionState::Initializing,
            client: None,
            breakpoints: HashMap::default(),
            threads: Vec::new(),
            current_thread_id: None,
            stack_frames: HashMap::default(),
            variables: HashMap::default(),
            watch_expressions: Vec::new(),
            console_output: Vec::new(),
        }
    }

    pub async fn start(
        &mut self,
        adapter_path: PathBuf,
        adapter_args: Vec<String>,
        launch_config: LaunchConfig,
        cx: &mut ModelContext<'_, Self>,
    ) -> Result<()> {
        let (event_tx, mut event_rx) = mpsc::unbounded();
        
        let client = DapClient::new(adapter_path, adapter_args, event_tx).await?;
        let client = Arc::new(client);
        
        // Initialize
        let capabilities = client.initialize("zed").await?;
        log::info!("Debug adapter capabilities: {:?}", capabilities);
        
        // Launch
        client.launch(launch_config).await?;
        
        // Configuration done
        client.send_request::<()>("configurationDone", None).await?;
        
        self.client = Some(client);
        self.state = DebugSessionState::Running;
        
        cx.emit(DebugSessionEvent::StateChanged(self.state.clone()));
        
        Ok(())
    }

    pub fn toggle_breakpoint(
        &mut self,
        file_path: &PathBuf,
        line: u32,
        cx: &mut ModelContext<'_, Self>,
    ) {
        let breakpoints = self.breakpoints.entry(file_path.clone()).or_default();
        
        if let Some(idx) = breakpoints.iter().position(|bp| bp.line == line) {
            breakpoints.remove(idx);
        } else {
            breakpoints.push(BreakpointState {
                line,
                verified: false,
                condition: None,
                hit_condition: None,
                log_message: None,
                hit_count: 0,
            });
        }
        
        cx.emit(DebugSessionEvent::BreakpointsChanged(file_path.clone()));
        
        // Sync with debug adapter if connected
        if let Some(client) = &self.client {
            let client = client.clone();
            let path = file_path.clone();
            let bps: Vec<SourceBreakpoint> = breakpoints
                .iter()
                .map(|bp| SourceBreakpoint {
                    line: bp.line as i64,
                    column: None,
                    condition: bp.condition.clone(),
                    hit_condition: bp.hit_condition.clone(),
                    log_message: bp.log_message.clone(),
                })
                .collect();
            
            cx.spawn(|this, mut cx| async move {
                if let Ok(verified_bps) = client
                    .set_breakpoints(path.to_str().unwrap(), bps)
                    .await
                {
                    this.update(&mut cx, |this, cx| {
                        if let Some(stored_bps) = this.breakpoints.get_mut(&path) {
                            for (i, bp) in verified_bps.iter().enumerate() {
                                if let Some(stored) = stored_bps.get_mut(i) {
                                    stored.verified = bp.verified;
                                }
                            }
                        }
                        cx.emit(DebugSessionEvent::BreakpointsChanged(path));
                    })
                    .ok();
                }
            })
            .detach();
        }
    }

    pub fn continue_execution(&self, cx: &mut ModelContext<'_, Self>) {
        if let (Some(client), Some(thread_id)) = (&self.client, self.current_thread_id) {
            let client = client.clone();
            cx.spawn(|this, mut cx| async move {
                if client.continue_execution(thread_id).await.is_ok() {
                    this.update(&mut cx, |this, cx| {
                        this.state = DebugSessionState::Running;
                        cx.emit(DebugSessionEvent::StateChanged(this.state.clone()));
                    })
                    .ok();
                }
            })
            .detach();
        }
    }

    pub fn step_over(&self, cx: &mut ModelContext<'_, Self>) {
        if let (Some(client), Some(thread_id)) = (&self.client, self.current_thread_id) {
            let client = client.clone();
            cx.spawn(|_this, _cx| async move {
                client.step_over(thread_id).await.ok();
            })
            .detach();
        }
    }

    pub fn step_into(&self, cx: &mut ModelContext<'_, Self>) {
        if let (Some(client), Some(thread_id)) = (&self.client, self.current_thread_id) {
            let client = client.clone();
            cx.spawn(|_this, _cx| async move {
                client.step_into(thread_id).await.ok();
            })
            .detach();
        }
    }

    pub fn step_out(&self, cx: &mut ModelContext<'_, Self>) {
        if let (Some(client), Some(thread_id)) = (&self.client, self.current_thread_id) {
            let client = client.clone();
            cx.spawn(|_this, _cx| async move {
                client.step_out(thread_id).await.ok();
            })
            .detach();
        }
    }

    pub fn refresh_call_stack(&self, cx: &mut ModelContext<'_, Self>) {
        if let (Some(client), Some(thread_id)) = (&self.client, self.current_thread_id) {
            let client = client.clone();
            cx.spawn(|this, mut cx| async move {
                if let Ok(frames) = client.stack_trace(thread_id).await {
                    this.update(&mut cx, |this, cx| {
                        this.stack_frames.insert(thread_id, frames);
                        cx.emit(DebugSessionEvent::VariablesUpdated);
                    })
                    .ok();
                }
            })
            .detach();
        }
    }

    pub fn add_watch_expression(&mut self, expression: String, cx: &mut ModelContext<'_, Self>) {
        let id = self.watch_expressions.len();
        self.watch_expressions.push(WatchExpression {
            id,
            expression: expression.clone(),
            result: None,
            error: None,
        });
        
        self.evaluate_watch_expression(id, cx);
    }

    fn evaluate_watch_expression(&self, watch_id: usize, cx: &mut ModelContext<'_, Self>) {
        if let Some(client) = &self.client {
            let expression = self.watch_expressions
                .get(watch_id)
                .map(|w| w.expression.clone());
            
            if let Some(expr) = expression {
                let client = client.clone();
                let frame_id = self.current_frame_id();
                
                cx.spawn(|this, mut cx| async move {
                    match client.evaluate(&expr, frame_id, EvaluateContext::Watch).await {
                        Ok(result) => {
                            this.update(&mut cx, |this, cx| {
                                if let Some(watch) = this.watch_expressions.get_mut(watch_id) {
                                    watch.result = Some(result.result);
                                    watch.error = None;
                                }
                                cx.emit(DebugSessionEvent::VariablesUpdated);
                            })
                            .ok();
                        }
                        Err(e) => {
                            this.update(&mut cx, |this, cx| {
                                if let Some(watch) = this.watch_expressions.get_mut(watch_id) {
                                    watch.result = None;
                                    watch.error = Some(e.to_string());
                                }
                                cx.emit(DebugSessionEvent::VariablesUpdated);
                            })
                            .ok();
                        }
                    }
                })
                .detach();
            }
        }
    }

    fn current_frame_id(&self) -> Option<i64> {
        self.current_thread_id
            .and_then(|tid| self.stack_frames.get(&tid))
            .and_then(|frames| frames.first())
            .map(|f| f.id)
    }

    pub async fn stop(&mut self) -> Result<()> {
        if let Some(client) = &self.client {
            client.terminate().await?;
            client.disconnect(true).await?;
        }
        self.state = DebugSessionState::Terminated;
        Ok(())
    }
}