// crates/debugger/src/dap_client.rs

use anyhow::{Context, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::{channel::mpsc, SinkExt, StreamExt};
use gpui::{AppContext, AsyncAppContext, Model, Task};
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    sync::Arc,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::Mutex,
};

/// Debug Adapter Protocol message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DapMessage {
    #[serde(rename = "request")]
    Request(DapRequest),
    #[serde(rename = "response")]
    Response(DapResponse),
    #[serde(rename = "event")]
    Event(DapEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DapRequest {
    pub seq: i64,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DapResponse {
    pub seq: i64,
    pub request_seq: i64,
    pub success: bool,
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DapEvent {
    pub seq: i64,
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

/// Breakpoint representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: Option<i64>,
    pub verified: bool,
    pub source: Option<Source>,
    pub line: Option<i64>,
    pub column: Option<i64>,
    pub end_line: Option<i64>,
    pub end_column: Option<i64>,
    pub instruction_reference: Option<String>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub name: Option<String>,
    pub path: Option<String>,
    pub source_reference: Option<i64>,
}

/// Stack frame for call stack display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub id: i64,
    pub name: String,
    pub source: Option<Source>,
    pub line: i64,
    pub column: i64,
    pub end_line: Option<i64>,
    pub end_column: Option<i64>,
    pub module_id: Option<serde_json::Value>,
    pub presentation_hint: Option<String>,
}

/// Variable representation for watch/locals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub value: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub variables_reference: i64,
    pub named_variables: Option<i64>,
    pub indexed_variables: Option<i64>,
    pub evaluate_name: Option<String>,
}

/// Thread state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: i64,
    pub name: String,
}

/// Debug session state
#[derive(Debug, Clone, PartialEq)]
pub enum DebugSessionState {
    Initializing,
    Running,
    Stopped { reason: StopReason, thread_id: i64 },
    Terminated,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StopReason {
    Breakpoint,
    Step,
    Exception,
    Pause,
    Entry,
    Goto,
    FunctionBreakpoint,
    DataBreakpoint,
    InstructionBreakpoint,
}

/// Main DAP client handling communication with debug adapters
pub struct DapClient {
    process: Child,
    stdin: Arc<Mutex<ChildStdin>>,
    stdout: Arc<Mutex<BufReader<ChildStdout>>>,
    sequence: Arc<Mutex<i64>>,
    pending_requests: Arc<Mutex<HashMap<i64, mpsc::Sender<DapResponse>>>>,
    event_tx: mpsc::UnboundedSender<DapEvent>,
    capabilities: Arc<Mutex<Option<Capabilities>>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Capabilities {
    pub supports_configuration_done_request: Option<bool>,
    pub supports_function_breakpoints: Option<bool>,
    pub supports_conditional_breakpoints: Option<bool>,
    pub supports_hit_conditional_breakpoints: Option<bool>,
    pub supports_evaluate_for_hovers: Option<bool>,
    pub supports_step_back: Option<bool>,
    pub supports_set_variable: Option<bool>,
    pub supports_restart_frame: Option<bool>,
    pub supports_goto_targets_request: Option<bool>,
    pub supports_step_in_targets_request: Option<bool>,
    pub supports_completions_request: Option<bool>,
    pub supports_modules_request: Option<bool>,
    pub supports_restart_request: Option<bool>,
    pub supports_exception_options: Option<bool>,
    pub supports_value_formatting_options: Option<bool>,
    pub supports_exception_info_request: Option<bool>,
    pub supports_terminate_debuggee: Option<bool>,
    pub supports_suspend_debuggee: Option<bool>,
    pub supports_delayed_stack_trace_loading: Option<bool>,
    pub supports_loaded_sources_request: Option<bool>,
    pub supports_log_points: Option<bool>,
    pub supports_terminate_threads_request: Option<bool>,
    pub supports_set_expression: Option<bool>,
    pub supports_terminate_request: Option<bool>,
    pub supports_data_breakpoints: Option<bool>,
    pub supports_read_memory_request: Option<bool>,
    pub supports_write_memory_request: Option<bool>,
    pub supports_disassemble_request: Option<bool>,
    pub supports_cancel_request: Option<bool>,
    pub supports_breakpoint_locations_request: Option<bool>,
    pub supports_clipboard_context: Option<bool>,
    pub supports_stepping_granularity: Option<bool>,
    pub supports_instruction_breakpoints: Option<bool>,
    pub supports_exception_filter_options: Option<bool>,
    pub supports_single_thread_execution_requests: Option<bool>,
}

impl DapClient {
    /// Spawn a new debug adapter process
    pub async fn new(
        adapter_path: PathBuf,
        adapter_args: Vec<String>,
        event_tx: mpsc::UnboundedSender<DapEvent>,
    ) -> Result<Self> {
        let mut process = Command::new(&adapter_path)
            .args(&adapter_args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn debug adapter")?;

        let stdin = process.stdin.take().context("Failed to get stdin")?;
        let stdout = process.stdout.take().context("Failed to get stdout")?;

        Ok(Self {
            process,
            stdin: Arc::new(Mutex::new(stdin)),
            stdout: Arc::new(Mutex::new(BufReader::new(stdout))),
            sequence: Arc::new(Mutex::new(1)),
            pending_requests: Arc::new(Mutex::new(HashMap::default())),
            event_tx,
            capabilities: Arc::new(Mutex::new(None)),
        })
    }

    /// Send a DAP request and wait for response
    pub async fn send_request<T: Serialize>(
        &self,
        command: &str,
        arguments: Option<T>,
    ) -> Result<DapResponse> {
        let seq = {
            let mut seq = self.sequence.lock().await;
            let current = *seq;
            *seq += 1;
            current
        };

        let request = DapRequest {
            seq,
            command: command.to_string(),
            arguments: arguments.map(|a| serde_json::to_value(a).unwrap()),
        };

        let message = DapMessage::Request(request);
        let json = serde_json::to_string(&message)?;
        let content = format!("Content-Length: {}\r\n\r\n{}", json.len(), json);

        // Set up response channel
        let (tx, mut rx) = mpsc::channel(1);
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(seq, tx);
        }

        // Send request
        {
            let mut stdin = self.stdin.lock().await;
            stdin.write_all(content.as_bytes()).await?;
            stdin.flush().await?;
        }

        // Wait for response
        rx.next().await.context("No response received")
    }

    /// Initialize the debug session
    pub async fn initialize(&self, adapter_id: &str) -> Result<Capabilities> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct InitializeArgs {
            client_id: String,
            client_name: String,
            adapter_id: String,
            locale: String,
            lines_start_at1: bool,
            columns_start_at1: bool,
            path_format: String,
            supports_variable_type: bool,
            supports_variable_paging: bool,
            supports_run_in_terminal_request: bool,
            supports_memory_references: bool,
            supports_progress_reporting: bool,
            supports_invalidated_event: bool,
            supports_memory_event: bool,
        }

        let args = InitializeArgs {
            client_id: "zed".to_string(),
            client_name: "Zed Editor".to_string(),
            adapter_id: adapter_id.to_string(),
            locale: "en-US".to_string(),
            lines_start_at1: true,
            columns_start_at1: true,
            path_format: "path".to_string(),
            supports_variable_type: true,
            supports_variable_paging: true,
            supports_run_in_terminal_request: true,
            supports_memory_references: true,
            supports_progress_reporting: true,
            supports_invalidated_event: true,
            supports_memory_event: true,
        };

        let response = self.send_request("initialize", Some(args)).await?;
        
        if response.success {
            let capabilities: Capabilities = response
                .body
                .map(|b| serde_json::from_value(b).unwrap_or_default())
                .unwrap_or_default();
            
            *self.capabilities.lock().await = Some(capabilities.clone());
            Ok(capabilities)
        } else {
            anyhow::bail!("Initialize failed: {:?}", response.message)
        }
    }

    /// Launch a program for debugging
    pub async fn launch(&self, config: LaunchConfig) -> Result<()> {
        let response = self.send_request("launch", Some(config)).await?;
        
        if response.success {
            Ok(())
        } else {
            anyhow::bail!("Launch failed: {:?}", response.message)
        }
    }

    /// Attach to a running process
    pub async fn attach(&self, config: AttachConfig) -> Result<()> {
        let response = self.send_request("attach", Some(config)).await?;
        
        if response.success {
            Ok(())
        } else {
            anyhow::bail!("Attach failed: {:?}", response.message)
        }
    }

    /// Set breakpoints for a source file
    pub async fn set_breakpoints(
        &self,
        source_path: &str,
        breakpoints: Vec<SourceBreakpoint>,
    ) -> Result<Vec<Breakpoint>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct SetBreakpointsArgs {
            source: Source,
            breakpoints: Vec<SourceBreakpoint>,
            source_modified: bool,
        }

        let args = SetBreakpointsArgs {
            source: Source {
                path: Some(source_path.to_string()),
                name: None,
                source_reference: None,
            },
            breakpoints,
            source_modified: false,
        };

        let response = self.send_request("setBreakpoints", Some(args)).await?;
        
        if response.success {
            #[derive(Deserialize)]
            struct SetBreakpointsResponse {
                breakpoints: Vec<Breakpoint>,
            }
            
            let body: SetBreakpointsResponse = response
                .body
                .map(|b| serde_json::from_value(b).unwrap())
                .unwrap();
            
            Ok(body.breakpoints)
        } else {
            anyhow::bail!("Set breakpoints failed: {:?}", response.message)
        }
    }

    /// Continue execution
    pub async fn continue_execution(&self, thread_id: i64) -> Result<()> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ContinueArgs {
            thread_id: i64,
            single_thread: bool,
        }

        let response = self
            .send_request("continue", Some(ContinueArgs { thread_id, single_thread: false }))
            .await?;
        
        if response.success {
            Ok(())
        } else {
            anyhow::bail!("Continue failed: {:?}", response.message)
        }
    }

    /// Step over (next line)
    pub async fn step_over(&self, thread_id: i64) -> Result<()> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct NextArgs {
            thread_id: i64,
            granularity: String,
        }

        let response = self
            .send_request("next", Some(NextArgs { 
                thread_id, 
                granularity: "statement".to_string() 
            }))
            .await?;
        
        if response.success {
            Ok(())
        } else {
            anyhow::bail!("Step over failed: {:?}", response.message)
        }
    }

    /// Step into function
    pub async fn step_into(&self, thread_id: i64) -> Result<()> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct StepInArgs {
            thread_id: i64,
            granularity: String,
        }

        let response = self
            .send_request("stepIn", Some(StepInArgs { 
                thread_id, 
                granularity: "statement".to_string() 
            }))
            .await?;
        
        if response.success {
            Ok(())
        } else {
            anyhow::bail!("Step into failed: {:?}", response.message)
        }
    }

    /// Step out of function
    pub async fn step_out(&self, thread_id: i64) -> Result<()> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct StepOutArgs {
            thread_id: i64,
            granularity: String,
        }

        let response = self
            .send_request("stepOut", Some(StepOutArgs { 
                thread_id, 
                granularity: "statement".to_string() 
            }))
            .await?;
        
        if response.success {
            Ok(())
        } else {
            anyhow::bail!("Step out failed: {:?}", response.message)
        }
    }

    /// Get current stack trace
    pub async fn stack_trace(&self, thread_id: i64) -> Result<Vec<StackFrame>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct StackTraceArgs {
            thread_id: i64,
            start_frame: Option<i64>,
            levels: Option<i64>,
        }

        let response = self
            .send_request("stackTrace", Some(StackTraceArgs { 
                thread_id, 
                start_frame: Some(0),
                levels: Some(100),
            }))
            .await?;
        
        if response.success {
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct StackTraceResponse {
                stack_frames: Vec<StackFrame>,
                total_frames: Option<i64>,
            }
            
            let body: StackTraceResponse = response
                .body
                .map(|b| serde_json::from_value(b).unwrap())
                .unwrap();
            
            Ok(body.stack_frames)
        } else {
            anyhow::bail!("Stack trace failed: {:?}", response.message)
        }
    }

    /// Get scopes for a stack frame
    pub async fn scopes(&self, frame_id: i64) -> Result<Vec<Scope>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ScopesArgs {
            frame_id: i64,
        }

        let response = self
            .send_request("scopes", Some(ScopesArgs { frame_id }))
            .await?;
        
        if response.success {
            #[derive(Deserialize)]
            struct ScopesResponse {
                scopes: Vec<Scope>,
            }
            
            let body: ScopesResponse = response
                .body
                .map(|b| serde_json::from_value(b).unwrap())
                .unwrap();
            
            Ok(body.scopes)
        } else {
            anyhow::bail!("Scopes failed: {:?}", response.message)
        }
    }

    /// Get variables for a reference
    pub async fn variables(&self, variables_reference: i64) -> Result<Vec<Variable>> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct VariablesArgs {
            variables_reference: i64,
        }

        let response = self
            .send_request("variables", Some(VariablesArgs { variables_reference }))
            .await?;
        
        if response.success {
            #[derive(Deserialize)]
            struct VariablesResponse {
                variables: Vec<Variable>,
            }
            
            let body: VariablesResponse = response
                .body
                .map(|b| serde_json::from_value(b).unwrap())
                .unwrap();
            
            Ok(body.variables)
        } else {
            anyhow::bail!("Variables failed: {:?}", response.message)
        }
    }

    /// Evaluate an expression
    pub async fn evaluate(
        &self,
        expression: &str,
        frame_id: Option<i64>,
        context: EvaluateContext,
    ) -> Result<EvaluateResult> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct EvaluateArgs {
            expression: String,
            frame_id: Option<i64>,
            context: String,
        }

        let response = self
            .send_request("evaluate", Some(EvaluateArgs { 
                expression: expression.to_string(),
                frame_id,
                context: context.as_str().to_string(),
            }))
            .await?;
        
        if response.success {
            let result: EvaluateResult = response
                .body
                .map(|b| serde_json::from_value(b).unwrap())
                .unwrap();
            
            Ok(result)
        } else {
            anyhow::bail!("Evaluate failed: {:?}", response.message)
        }
    }

    /// Terminate the debug session
    pub async fn terminate(&self) -> Result<()> {
        let _ = self.send_request::<()>("terminate", None).await;
        Ok(())
    }

    /// Disconnect from the debug adapter
    pub async fn disconnect(&self, terminate_debuggee: bool) -> Result<()> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct DisconnectArgs {
            restart: bool,
            terminate_debuggee: bool,
            suspend_debuggee: bool,
        }

        let _ = self
            .send_request("disconnect", Some(DisconnectArgs {
                restart: false,
                terminate_debuggee,
                suspend_debuggee: false,
            }))
            .await;
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceBreakpoint {
    pub line: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    pub name: String,
    pub presentation_hint: Option<String>,
    pub variables_reference: i64,
    pub named_variables: Option<i64>,
    pub indexed_variables: Option<i64>,
    pub expensive: bool,
    pub source: Option<Source>,
    pub line: Option<i64>,
    pub column: Option<i64>,
    pub end_line: Option<i64>,
    pub end_column: Option<i64>,
}

#[derive(Debug, Clone)]
pub enum EvaluateContext {
    Watch,
    Repl,
    Hover,
    Clipboard,
    Variables,
}

impl EvaluateContext {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Watch => "watch",
            Self::Repl => "repl",
            Self::Hover => "hover",
            Self::Clipboard => "clipboard",
            Self::Variables => "variables",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateResult {
    pub result: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub presentation_hint: Option<VariablePresentationHint>,
    pub variables_reference: i64,
    pub named_variables: Option<i64>,
    pub indexed_variables: Option<i64>,
    pub memory_reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariablePresentationHint {
    pub kind: Option<String>,
    pub attributes: Option<Vec<String>>,
    pub visibility: Option<String>,
    pub lazy: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchConfig {
    pub program: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_on_entry: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console: Option<String>,
    // Allow additional adapter-specific fields
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}