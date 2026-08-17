// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

//! End-to-end performance tests for the pet JSONRPC server.
//!
//! These tests spawn the pet server as a subprocess and communicate via JSONRPC
//! to measure discovery performance from a client perspective.

use pet_core::telemetry::refresh_progress::{
    RefreshProgress, RefreshProgressPhase, RefreshProgressStatus,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::env;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

mod common;

/// JSONRPC request ID counter
static REQUEST_ID: AtomicU32 = AtomicU32::new(1);

/// Number of iterations for statistical tests
const STAT_ITERATIONS: usize = 10;
const PERFORMANCE_METRICS_SCHEMA_VERSION: u8 = 2;
const STDERR_TAIL_LINES: usize = 100;

/// Statistical metrics with percentile calculations
#[derive(Debug, Clone, Default)]
pub struct StatisticalMetrics {
    samples: Vec<u128>,
}

impl StatisticalMetrics {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn add(&mut self, value: u128) {
        self.samples.push(value);
    }

    pub fn count(&self) -> usize {
        self.samples.len()
    }

    pub fn min(&self) -> Option<u128> {
        self.samples.iter().copied().min()
    }

    pub fn max(&self) -> Option<u128> {
        self.samples.iter().copied().max()
    }

    pub fn mean(&self) -> Option<f64> {
        if self.samples.is_empty() {
            return None;
        }
        let sum: u128 = self.samples.iter().sum();
        Some(sum as f64 / self.samples.len() as f64)
    }

    pub fn std_dev(&self) -> Option<f64> {
        let mean = self.mean()?;
        if self.samples.len() < 2 {
            return None;
        }
        let variance: f64 = self
            .samples
            .iter()
            .map(|&x| {
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / (self.samples.len() - 1) as f64;
        Some(variance.sqrt())
    }

    fn sorted(&self) -> Vec<u128> {
        let mut sorted = self.samples.clone();
        sorted.sort();
        sorted
    }

    fn percentile(&self, p: f64) -> Option<u128> {
        if self.samples.is_empty() {
            return None;
        }
        let sorted = self.sorted();
        let n = sorted.len();
        if n == 1 {
            return Some(sorted[0]);
        }
        // Linear interpolation between closest ranks
        let rank = p / 100.0 * (n - 1) as f64;
        let lower = rank.floor() as usize;
        let upper = rank.ceil() as usize;
        let weight = rank - lower as f64;

        if upper >= n {
            return Some(sorted[n - 1]);
        }

        let result = sorted[lower] as f64 * (1.0 - weight) + sorted[upper] as f64 * weight;
        Some(result.round() as u128)
    }

    pub fn p50(&self) -> Option<u128> {
        self.percentile(50.0)
    }

    pub fn p95(&self) -> Option<u128> {
        self.percentile(95.0)
    }

    pub fn p99(&self) -> Option<u128> {
        self.percentile(99.0)
    }

    pub fn to_json(&self) -> Value {
        json!({
            "count": self.count(),
            "min": self.min(),
            "max": self.max(),
            "mean": self.mean(),
            "std_dev": self.std_dev(),
            "p50": self.p50(),
            "p95": self.p95(),
            "p99": self.p99()
        })
    }

    pub fn print_summary(&self, label: &str) {
        println!(
            "{}: P50={}ms, P95={}ms, P99={}ms, mean={:.1}ms, std_dev={:.1}ms (n={})",
            label,
            self.p50().unwrap_or(0),
            self.p95().unwrap_or(0),
            self.p99().unwrap_or(0),
            self.mean().unwrap_or(0.0),
            self.std_dev().unwrap_or(0.0),
            self.count()
        );
    }
}

/// Performance metrics collected during tests
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Time to spawn server and get first response (configure)
    pub server_startup_ms: u128,
    /// Time for full machine refresh
    pub full_refresh_ms: u128,
    /// Time for workspace-scoped refresh
    pub workspace_refresh_ms: Option<u128>,
    /// Time for kind-specific refresh
    pub kind_refresh_ms: HashMap<String, u128>,
    /// Number of environments discovered
    pub environments_count: usize,
    /// Number of managers discovered
    pub managers_count: usize,
    /// Time to first environment notification
    pub time_to_first_env_ms: Option<u128>,
    /// Resolve times (cold and warm)
    pub resolve_times_ms: Vec<u128>,
}

/// Refresh result from server
#[derive(Debug, Clone, Deserialize)]
pub struct RefreshResult {
    pub duration: u128,
}

/// Environment notification from server
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    pub executable: Option<String>,
    pub kind: Option<String>,
    #[allow(dead_code)]
    pub version: Option<String>,
}

/// Manager notification from server
#[derive(Debug, Clone, Deserialize)]
pub struct Manager {
    #[allow(dead_code)]
    pub tool: Option<String>,
    #[allow(dead_code)]
    pub executable: Option<String>,
}

/// Shared state for handling notifications
struct SharedState {
    environments: Mutex<Vec<Environment>>,
    managers: Mutex<Vec<Manager>>,
    refresh_progress: Mutex<Vec<RefreshProgress>>,
    capture_refresh_progress: bool,
    first_env_time: Mutex<Option<Instant>>,
}

impl SharedState {
    fn new(capture_refresh_progress: bool) -> Self {
        Self {
            environments: Mutex::new(Vec::new()),
            managers: Mutex::new(Vec::new()),
            refresh_progress: Mutex::new(Vec::new()),
            capture_refresh_progress,
            first_env_time: Mutex::new(None),
        }
    }

    fn handle_notification(&self, method: &str, params: Value) {
        match method {
            "environment" => {
                // Record time to first environment
                {
                    let mut first_env = self.first_env_time.lock().unwrap();
                    if first_env.is_none() {
                        *first_env = Some(Instant::now());
                    }
                }

                if let Ok(env) = serde_json::from_value::<Environment>(params) {
                    self.environments.lock().unwrap().push(env);
                }
            }
            "manager" => {
                if let Ok(mgr) = serde_json::from_value::<Manager>(params) {
                    self.managers.lock().unwrap().push(mgr);
                }
            }
            "telemetry" if self.capture_refresh_progress => {
                if params.get("event").and_then(Value::as_str) == Some("RefreshProgress") {
                    if let Some(progress) = params
                        .get("data")
                        .and_then(|data| data.get("refreshProgress"))
                        .and_then(|value| {
                            serde_json::from_value::<RefreshProgress>(value.clone()).ok()
                        })
                    {
                        self.refresh_progress
                            .lock()
                            .expect("refresh progress mutex poisoned")
                            .push(progress);
                    }
                }
            }
            "log" => {}
            _ => {
                // Unknown notification
            }
        }
    }

    fn clear(&self) {
        self.environments.lock().unwrap().clear();
        self.managers.lock().unwrap().clear();
        self.refresh_progress
            .lock()
            .expect("refresh progress mutex poisoned")
            .clear();
        *self.first_env_time.lock().unwrap() = None;
    }
}

/// JSONRPC client for communicating with the pet server
pub struct PetClient {
    process: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    stderr_tail: Arc<Mutex<VecDeque<String>>>,
    interpreter_probe_timeouts: Arc<Mutex<BTreeMap<String, usize>>>,
    stderr_handle: Option<JoinHandle<()>>,
    state: Arc<SharedState>,
    start_time: Instant,
}

impl PetClient {
    /// Spawn the pet server and create a client
    pub fn spawn() -> Result<Self, String> {
        Self::spawn_with_options(false)
    }

    fn spawn_with_refresh_progress() -> Result<Self, String> {
        Self::spawn_with_options(true)
    }

    fn spawn_with_options(capture_refresh_progress: bool) -> Result<Self, String> {
        let pet_exe = get_pet_executable();

        if !pet_exe.exists() {
            return Err(format!(
                "pet executable not found at {:?}. Run `cargo build --release` first.",
                pet_exe
            ));
        }

        let start_time = Instant::now();

        let mut process = Command::new(&pet_exe)
            .arg("server")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn pet server: {}", e))?;
        let stdin = process
            .stdin
            .take()
            .expect("PET stdin must be piped by the command above");
        let stdout = process
            .stdout
            .take()
            .expect("PET stdout must be piped by the command above");
        let stderr = process
            .stderr
            .take()
            .expect("PET stderr must be piped by the command above");
        let stderr_tail = Arc::new(Mutex::new(VecDeque::with_capacity(STDERR_TAIL_LINES)));
        let interpreter_probe_timeouts = Arc::new(Mutex::new(BTreeMap::new()));
        let stderr_handle = spawn_stderr_reader(
            stderr,
            stderr_tail.clone(),
            interpreter_probe_timeouts.clone(),
        );

        Ok(Self {
            process,
            stdin,
            stdout: BufReader::new(stdout),
            stderr_tail,
            interpreter_probe_timeouts,
            stderr_handle: Some(stderr_handle),
            state: Arc::new(SharedState::new(capture_refresh_progress)),
            start_time,
        })
    }

    /// Send a JSONRPC request and wait for response
    fn send_request(&mut self, method: &str, params: Value) -> Result<Value, String> {
        let id = REQUEST_ID.fetch_add(1, Ordering::SeqCst);
        let request = json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        let request_str = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let content_length = request_str.len();
        let message = format!("Content-Length: {}\r\n\r\n{}", content_length, request_str);

        // Write request
        {
            self.stdin
                .write_all(message.as_bytes())
                .map_err(|e| format!("Failed to write request: {}", e))?;
            self.stdin
                .flush()
                .map_err(|e| format!("Failed to flush stdin: {}", e))?;
        }

        // Clone state reference for use in the loop
        let state = self.state.clone();

        // Read response - handle notifications until we get our response.
        // The reader lives for the process lifetime so read-ahead bytes are never discarded.
        loop {
            let value = read_jsonrpc_message(&mut self.stdout).map_err(|error| {
                let stderr = self.stderr_output();
                if stderr.is_empty() {
                    error
                } else {
                    format!("{error}; PET stderr tail:\n{stderr}")
                }
            })?;

            // Check if this is a notification or our response
            if let Some(notif_method) = value.get("method").and_then(|m| m.as_str()) {
                // Handle notifications using the cloned state reference
                state.handle_notification(
                    notif_method,
                    value.get("params").cloned().unwrap_or(Value::Null),
                );
                continue;
            }

            // Check if this is our response
            if let Some(response_id) = value.get("id").and_then(|i| i.as_u64()) {
                if response_id as u32 == id {
                    if let Some(error) = value.get("error") {
                        return Err(format!("JSONRPC error: {:?}", error));
                    }
                    return Ok(value.get("result").cloned().unwrap_or(Value::Null));
                }
            }
        }
    }

    fn stderr_output(&self) -> String {
        self.stderr_tail
            .lock()
            .expect("PET stderr tail mutex poisoned")
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn interpreter_probe_timeout_counts(&self) -> BTreeMap<String, usize> {
        self.interpreter_probe_timeouts
            .lock()
            .expect("interpreter probe timeout mutex poisoned")
            .clone()
    }

    /// Configure the server
    pub fn configure(&mut self, config: Value) -> Result<Duration, String> {
        let start = Instant::now();
        self.send_request("configure", config)?;
        Ok(start.elapsed())
    }

    /// Refresh environments
    pub fn refresh(&mut self, params: Option<Value>) -> Result<(RefreshResult, Duration), String> {
        // Clear previous results
        self.state.clear();

        let start = Instant::now();
        let result = self.send_request("refresh", params.unwrap_or(json!({})))?;
        let elapsed = start.elapsed();

        let refresh_result: RefreshResult = serde_json::from_value(result)
            .map_err(|e| format!("Failed to parse refresh result: {}", e))?;

        Ok((refresh_result, elapsed))
    }

    /// Resolve a Python executable
    pub fn resolve(&mut self, executable: &str) -> Result<(Value, Duration), String> {
        let start = Instant::now();
        let result = self.send_request("resolve", json!({ "executable": executable }))?;
        Ok((result, start.elapsed()))
    }

    /// Get collected environments
    pub fn get_environments(&self) -> Vec<Environment> {
        self.state.environments.lock().unwrap().clone()
    }

    /// Get collected managers
    pub fn get_managers(&self) -> Vec<Manager> {
        self.state.managers.lock().unwrap().clone()
    }

    fn get_refresh_progress(&self) -> Vec<RefreshProgress> {
        self.state
            .refresh_progress
            .lock()
            .expect("refresh progress mutex poisoned")
            .clone()
    }

    /// Get time from start to first environment
    pub fn time_to_first_env(&self) -> Option<Duration> {
        self.state
            .first_env_time
            .lock()
            .unwrap()
            .map(|t| t.duration_since(self.start_time))
    }

    /// Get startup time
    #[allow(dead_code)]
    pub fn startup_time(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl Drop for PetClient {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
        if let Some(stderr_handle) = self.stderr_handle.take() {
            let _ = stderr_handle.join();
        }
    }
}

/// Get the path to the pet executable
fn get_pet_executable() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target_dir = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target");

    let exe_name = if cfg!(windows) { "pet.exe" } else { "pet" };

    // When building with --target <triple>, cargo outputs to target/<triple>/release/
    // Check for target-specific builds first (used in CI)
    let target_triples = [
        "x86_64-pc-windows-msvc",
        "x86_64-unknown-linux-musl",
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
    ];

    // Check target-specific release builds first
    for triple in target_triples {
        let target_release_exe = target_dir.join(triple).join("release").join(exe_name);
        if target_release_exe.exists() {
            return target_release_exe;
        }
    }

    // Fall back to standard release build (no --target flag)
    let release_exe = target_dir.join("release").join(exe_name);
    if release_exe.exists() {
        return release_exe;
    }

    // Check target-specific debug builds
    for triple in target_triples {
        let target_debug_exe = target_dir.join(triple).join("debug").join(exe_name);
        if target_debug_exe.exists() {
            return target_debug_exe;
        }
    }

    // Fall back to standard debug build
    target_dir.join("debug").join(exe_name)
}

/// Get a temporary cache directory for tests
fn get_test_cache_dir(test_name: &str) -> PathBuf {
    let tmp = env::temp_dir();
    tmp.join("pet-e2e-perf-tests")
        .join(format!("cache-{}", std::process::id()))
        .join(test_name)
}

fn benchmark_iteration_cache_dir(cache_root: &Path, workload: &str, iteration: usize) -> PathBuf {
    cache_root
        .join(workload)
        .join(format!("iteration-{}", iteration + 1))
}

fn reset_cache_dir(cache_dir: &Path) {
    match std::fs::remove_dir_all(cache_dir) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => panic!("Failed to remove cache directory {cache_dir:?}: {error}"),
    }
    std::fs::create_dir_all(cache_dir)
        .unwrap_or_else(|error| panic!("Failed to create cache directory {cache_dir:?}: {error}"));
}

fn assert_stable_inventory(
    expected: &mut Option<(usize, usize)>,
    actual: (usize, usize),
    workload: &str,
    iteration: usize,
) {
    if let Some(expected) = expected {
        assert_eq!(
            actual,
            *expected,
            "{workload} inventory changed at iteration {}",
            iteration + 1
        );
    } else {
        *expected = Some(actual);
    }
}

/// Get workspace directory (current project root)
fn get_workspace_dir() -> PathBuf {
    env::var("GITHUB_WORKSPACE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .to_path_buf()
        })
}

fn interpreter_probe_timeout_label(line: &str) -> Option<&'static str> {
    if !line.contains("Timed out after") || !line.contains("resolving Python via spawn") {
        return None;
    }
    if line.contains("/usr/bin/python3") {
        Some("usrBinPython3")
    } else if line.contains("CommandLineTools") {
        Some("commandLineTools")
    } else if line.contains("hostedtoolcache") {
        Some("hostedToolcache")
    } else if line.contains("/Library/Frameworks/Python.framework") {
        Some("pythonOrgFramework")
    } else if line.contains("/usr/local/bin") {
        Some("usrLocalBin")
    } else {
        Some("other")
    }
}

#[test]
fn interpreter_probe_timeouts_are_classified_without_exposing_paths() {
    assert_eq!(
        interpreter_probe_timeout_label(
            r#"Timed out after 15s resolving Python via spawn for "/usr/bin/python3"; killing child."#
        ),
        Some("usrBinPython3")
    );
    assert_eq!(
        interpreter_probe_timeout_label("ordinary PET warning"),
        None
    );
}

fn read_jsonrpc_message(reader: &mut impl BufRead) -> Result<Value, String> {
    let mut content_length = None;
    loop {
        let mut header_line = String::new();
        let bytes_read = reader
            .read_line(&mut header_line)
            .map_err(|error| format!("Failed to read header: {error}"))?;
        if bytes_read == 0 {
            return Err("PET stdout closed while reading a JSONRPC header".to_string());
        }

        let trimmed = header_line.trim();
        if trimmed.is_empty() {
            break;
        }
        if let Some(length) = trimmed.strip_prefix("Content-Length: ") {
            content_length = Some(
                length
                    .parse::<usize>()
                    .map_err(|error| format!("Failed to parse content length: {error}"))?,
            );
        }
    }

    let content_length = content_length.ok_or("Missing Content-Length header")?;
    let mut body = vec![0u8; content_length];
    reader
        .read_exact(&mut body)
        .map_err(|error| format!("Failed to read body: {error}"))?;
    serde_json::from_slice(&body).map_err(|error| format!("Failed to parse response: {error}"))
}

fn spawn_stderr_reader(
    stderr: impl Read + Send + 'static,
    stderr_tail: Arc<Mutex<VecDeque<String>>>,
    interpreter_probe_timeouts: Arc<Mutex<BTreeMap<String, usize>>>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        for line in BufReader::new(stderr).lines() {
            let line = match line {
                Ok(line) => line,
                Err(error) => format!("Failed to read PET stderr: {error}"),
            };
            if let Some(label) = interpreter_probe_timeout_label(&line) {
                *interpreter_probe_timeouts
                    .lock()
                    .expect("interpreter probe timeout mutex poisoned")
                    .entry(label.to_string())
                    .or_default() += 1;
            }
            let mut tail = stderr_tail.lock().expect("PET stderr tail mutex poisoned");
            if tail.len() == STDERR_TAIL_LINES {
                tail.pop_front();
            }
            tail.push_back(line);
        }
    })
}

#[test]
fn jsonrpc_reader_preserves_buffered_follow_up_message() {
    let first = json!({"jsonrpc": "2.0", "id": 1, "result": {"value": 1}});
    let second = json!({"jsonrpc": "2.0", "id": 2, "result": {"value": 2}});
    let framed = [first.clone(), second.clone()]
        .into_iter()
        .map(|message| {
            let body = serde_json::to_string(&message).unwrap();
            format!("Content-Length: {}\r\n\r\n{}", body.len(), body)
        })
        .collect::<String>();
    let mut reader = BufReader::new(std::io::Cursor::new(framed.into_bytes()));

    assert_eq!(read_jsonrpc_message(&mut reader).unwrap(), first);
    assert_eq!(read_jsonrpc_message(&mut reader).unwrap(), second);
}

#[test]
fn stderr_reader_drains_input_and_bounds_diagnostic_tail() {
    let timeout_line =
        r#"Timed out after 15s resolving Python via spawn for "/usr/bin/python3"; killing child."#;
    let input = format!(
        "{timeout_line}\n{}",
        (0..STDERR_TAIL_LINES + 5)
            .map(|index| format!("line {index}\n"))
            .collect::<String>()
    );
    let tail = Arc::new(Mutex::new(VecDeque::new()));
    let timeout_counts = Arc::new(Mutex::new(BTreeMap::new()));
    let handle = spawn_stderr_reader(
        std::io::Cursor::new(input.into_bytes()),
        tail.clone(),
        timeout_counts.clone(),
    );
    handle.join().unwrap();

    let tail = tail.lock().unwrap();
    assert_eq!(tail.len(), STDERR_TAIL_LINES);
    assert_eq!(tail.front().map(String::as_str), Some("line 5"));
    assert_eq!(tail.back().map(String::as_str), Some("line 104"));
    drop(tail);
    assert_eq!(
        timeout_counts.lock().unwrap().get("usrBinPython3"),
        Some(&1)
    );
}

fn refresh_phase_name(phase: RefreshProgressPhase) -> &'static str {
    match phase {
        RefreshProgressPhase::Locators => "locators",
        RefreshProgressPhase::Path => "path",
        RefreshProgressPhase::GlobalVirtualEnvs => "globalVirtualEnvs",
        RefreshProgressPhase::Workspaces => "workspaces",
    }
}

fn collect_refresh_progress(
    progress: &[RefreshProgress],
    phase_stats: &mut BTreeMap<String, StatisticalMetrics>,
    locator_stats: &mut BTreeMap<String, StatisticalMetrics>,
) {
    for event in progress
        .iter()
        .filter(|event| event.status == RefreshProgressStatus::Completed)
    {
        if let (Some(locator), Some(duration)) = (&event.locator_name, event.locator_elapsed_ms) {
            locator_stats
                .entry(locator.clone())
                .or_default()
                .add(duration);
        } else if let Some(duration) = event.phase_elapsed_ms {
            phase_stats
                .entry(refresh_phase_name(event.phase).to_string())
                .or_default()
                .add(duration);
        }
    }
}

fn statistics_json(statistics: &BTreeMap<String, StatisticalMetrics>) -> BTreeMap<String, Value> {
    statistics
        .iter()
        .map(|(name, metrics)| (name.clone(), metrics.to_json()))
        .collect()
}

fn record_interpreter_probe_timeouts(
    client: &PetClient,
    probe_timeout_counts: &mut BTreeMap<String, usize>,
) {
    let timeout_counts = client.interpreter_probe_timeout_counts();
    for (label, count) in &timeout_counts {
        *probe_timeout_counts.entry(label.clone()).or_default() += count;
    }
    if !timeout_counts.is_empty() {
        println!("    Interpreter probe timeouts: {timeout_counts:?}");
    }
}

fn collect_refresh_diagnostics(
    workspace_dir: &Path,
    cache_dir: &Path,
    phase_stats: &mut BTreeMap<String, StatisticalMetrics>,
    locator_stats: &mut BTreeMap<String, StatisticalMetrics>,
    probe_timeout_counts: &mut BTreeMap<String, usize>,
    expected_inventory: (usize, usize),
) {
    let diagnostic_cache_root = cache_dir.join("refresh-progress");
    reset_cache_dir(&diagnostic_cache_root);

    println!("\nCollecting untimed cold-refresh diagnostics...");
    for iteration in 0..STAT_ITERATIONS {
        let diagnostic_cache_dir =
            benchmark_iteration_cache_dir(&diagnostic_cache_root, "cold", iteration);
        reset_cache_dir(&diagnostic_cache_dir);
        let mut client =
            PetClient::spawn_with_refresh_progress().expect("Failed to spawn diagnostic server");
        client
            .configure(json!({
                "workspaceDirectories": [workspace_dir],
                "cacheDirectory": diagnostic_cache_dir,
            }))
            .expect("Failed to configure diagnostic server");
        let (result, _) = client
            .refresh(None)
            .expect("Failed to run diagnostic refresh");

        collect_refresh_progress(&client.get_refresh_progress(), phase_stats, locator_stats);
        let inventory = (client.get_environments().len(), client.get_managers().len());
        assert_eq!(
            inventory,
            expected_inventory,
            "Cold diagnostic inventory changed at iteration {}",
            iteration + 1,
        );
        record_interpreter_probe_timeouts(&client, probe_timeout_counts);
        println!(
            "  Cold diagnostic iteration {}: refresh={}ms, envs={}",
            iteration + 1,
            result.duration,
            inventory.0,
        );
    }
}

#[test]
fn refresh_progress_notifications_are_collected_only_when_enabled() {
    let notification = json!({
        "event": "RefreshProgress",
        "data": {
            "refreshProgress": {
                "refreshId": 7,
                "phase": "locators",
                "status": "completed",
                "elapsedMs": 25,
                "locatorName": "Conda",
                "locatorElapsedMs": 20
            }
        }
    });

    let disabled_state = SharedState::new(false);
    disabled_state.handle_notification("telemetry", notification.clone());
    assert!(disabled_state.refresh_progress.lock().unwrap().is_empty());

    let state = SharedState::new(true);
    state.handle_notification("telemetry", notification);
    let progress = state.refresh_progress.lock().unwrap();
    assert_eq!(progress.len(), 1);
    assert_eq!(progress[0].locator_name.as_deref(), Some("Conda"));
    assert_eq!(progress[0].locator_elapsed_ms, Some(20));
}

#[test]
fn refresh_progress_aggregation_separates_phases_and_locators() {
    let progress = vec![
        RefreshProgress {
            refresh_id: 1,
            phase: RefreshProgressPhase::Locators,
            status: RefreshProgressStatus::Completed,
            elapsed_ms: 30,
            phase_elapsed_ms: Some(30),
            locator_name: None,
            locator_elapsed_ms: None,
        },
        RefreshProgress {
            refresh_id: 1,
            phase: RefreshProgressPhase::Locators,
            status: RefreshProgressStatus::Completed,
            elapsed_ms: 25,
            phase_elapsed_ms: None,
            locator_name: Some("Conda".to_string()),
            locator_elapsed_ms: Some(20),
        },
    ];
    let mut phases = BTreeMap::new();
    let mut locators = BTreeMap::new();

    collect_refresh_progress(&progress, &mut phases, &mut locators);

    assert_eq!(phases["locators"].samples, vec![30]);
    assert_eq!(locators["Conda"].samples, vec![20]);
}

#[test]
fn benchmark_cache_directories_are_isolated_by_workload_and_iteration() {
    let root = Path::new("benchmark-cache");
    let first_cold = benchmark_iteration_cache_dir(root, "cold", 0);
    let second_cold = benchmark_iteration_cache_dir(root, "cold", 1);
    let first_warm = benchmark_iteration_cache_dir(root, "warm", 0);

    assert_eq!(first_cold, root.join("cold").join("iteration-1"));
    assert_ne!(first_cold, second_cold);
    assert_ne!(first_cold, first_warm);

    assert_ne!(
        get_test_cache_dir("first-test"),
        get_test_cache_dir("second-test")
    );
}

// ============================================================================
// Performance Tests
// ============================================================================

#[cfg_attr(feature = "ci-perf", test)]
#[allow(dead_code)]
fn test_server_startup_performance() {
    let mut spawn_stats = StatisticalMetrics::new();
    let mut configure_stats = StatisticalMetrics::new();
    let mut total_stats = StatisticalMetrics::new();

    let cache_dir = get_test_cache_dir("server-startup");
    let workspace_dir = get_workspace_dir();

    println!(
        "=== Server Startup Performance ({} iterations) ===",
        STAT_ITERATIONS
    );

    for i in 0..STAT_ITERATIONS {
        let start = Instant::now();
        let mut client = PetClient::spawn().expect("Failed to spawn server");
        let spawn_time = start.elapsed();

        let config = json!({
            "workspaceDirectories": [workspace_dir.clone()],
            "cacheDirectory": cache_dir.clone()
        });

        let configure_time = client.configure(config).expect("Failed to configure");
        let total_time = spawn_time + configure_time;

        spawn_stats.add(spawn_time.as_millis());
        configure_stats.add(configure_time.as_millis());
        total_stats.add(total_time.as_millis());

        println!(
            "  Iteration {}: spawn={}ms, configure={}ms, total={}ms",
            i + 1,
            spawn_time.as_millis(),
            configure_time.as_millis(),
            total_time.as_millis()
        );
    }

    println!();
    spawn_stats.print_summary("Server spawn");
    configure_stats.print_summary("Configure");
    total_stats.print_summary("Total startup");

    // Output JSON for CI
    let json_output = serde_json::to_string_pretty(&json!({
        "spawn": spawn_stats.to_json(),
        "configure": configure_stats.to_json(),
        "total": total_stats.to_json()
    }))
    .unwrap();
    println!("\nJSON metrics:\n{}", json_output);

    // Assert reasonable startup time (P95 should be under 5 seconds)
    assert!(
        spawn_stats.p95().unwrap_or(0) < 5000,
        "Server spawn P95 took too long: {}ms",
        spawn_stats.p95().unwrap_or(0)
    );
    assert!(
        configure_stats.p95().unwrap_or(0) < 1000,
        "Configure P95 took too long: {}ms",
        configure_stats.p95().unwrap_or(0)
    );
}

#[cfg_attr(feature = "ci-perf", test)]
#[allow(dead_code)]
fn test_full_refresh_performance() {
    let mut server_duration_stats = StatisticalMetrics::new();
    let mut client_duration_stats = StatisticalMetrics::new();
    let mut time_to_first_env_stats = StatisticalMetrics::new();
    let mut env_count = 0usize;
    let mut manager_count = 0usize;
    let mut kind_counts: HashMap<String, usize> = HashMap::new();

    let cache_dir = get_test_cache_dir("full-refresh");
    let workspace_dir = get_workspace_dir();

    println!(
        "=== Full Refresh Performance ({} iterations) ===",
        STAT_ITERATIONS
    );

    for i in 0..STAT_ITERATIONS {
        // Fresh server each iteration for consistent cold-start measurement
        let mut client = PetClient::spawn().expect("Failed to spawn server");

        let config = json!({
            "workspaceDirectories": [workspace_dir.clone()],
            "cacheDirectory": cache_dir.clone()
        });

        client.configure(config).expect("Failed to configure");

        // Full machine refresh
        let (result, client_elapsed) = client.refresh(None).expect("Failed to refresh");
        let environments = client.get_environments();
        let managers = client.get_managers();

        server_duration_stats.add(result.duration);
        client_duration_stats.add(client_elapsed.as_millis());

        if let Some(time_to_first) = client.time_to_first_env() {
            time_to_first_env_stats.add(time_to_first.as_millis());
        }

        // Track counts from last iteration
        env_count = environments.len();
        manager_count = managers.len();

        // Aggregate kind counts
        if i == STAT_ITERATIONS - 1 {
            for env in &environments {
                if let Some(kind) = &env.kind {
                    *kind_counts.entry(kind.clone()).or_insert(0) += 1;
                }
            }
        }

        println!(
            "  Iteration {}: server={}ms, client={}ms, envs={}",
            i + 1,
            result.duration,
            client_elapsed.as_millis(),
            environments.len()
        );
    }

    println!();
    server_duration_stats.print_summary("Server duration");
    client_duration_stats.print_summary("Client duration");
    if time_to_first_env_stats.count() > 0 {
        time_to_first_env_stats.print_summary("Time to first env");
    }
    println!("Environments discovered: {}", env_count);
    println!("Managers discovered: {}", manager_count);
    println!("Environment kinds: {:?}", kind_counts);

    // Output JSON for CI
    let json_output = serde_json::to_string_pretty(&json!({
        "server_duration": server_duration_stats.to_json(),
        "client_duration": client_duration_stats.to_json(),
        "time_to_first_env": time_to_first_env_stats.to_json(),
        "environments_count": env_count,
        "managers_count": manager_count
    }))
    .unwrap();
    println!("\nJSON metrics:\n{}", json_output);

    // Assert we found at least some environments (CI should always have Python installed)
    assert!(
        env_count > 0,
        "No environments discovered - this is unexpected"
    );
}

#[cfg_attr(feature = "ci-perf", test)]
#[allow(dead_code)]
fn test_workspace_scoped_refresh_performance() {
    let mut server_duration_stats = StatisticalMetrics::new();
    let mut client_duration_stats = StatisticalMetrics::new();
    let mut env_count = 0usize;

    let cache_dir = get_test_cache_dir("workspace-refresh");
    let workspace_dir = get_workspace_dir();

    println!(
        "=== Workspace-Scoped Refresh Performance ({} iterations) ===",
        STAT_ITERATIONS
    );

    for i in 0..STAT_ITERATIONS {
        let mut client = PetClient::spawn().expect("Failed to spawn server");

        let config = json!({
            "workspaceDirectories": [workspace_dir.clone()],
            "cacheDirectory": cache_dir.clone()
        });

        client.configure(config).expect("Failed to configure");

        // Workspace-scoped refresh
        let (result, client_elapsed) = client
            .refresh(Some(json!({ "searchPaths": [workspace_dir.clone()] })))
            .expect("Failed to refresh");

        let environments = client.get_environments();

        server_duration_stats.add(result.duration);
        client_duration_stats.add(client_elapsed.as_millis());
        env_count = environments.len();

        println!(
            "  Iteration {}: server={}ms, client={}ms, envs={}",
            i + 1,
            result.duration,
            client_elapsed.as_millis(),
            environments.len()
        );
    }

    println!();
    server_duration_stats.print_summary("Server duration");
    client_duration_stats.print_summary("Client duration");
    println!("Environments discovered: {}", env_count);

    // Output JSON for CI
    let json_output = serde_json::to_string_pretty(&json!({
        "server_duration": server_duration_stats.to_json(),
        "client_duration": client_duration_stats.to_json(),
        "environments_count": env_count
    }))
    .unwrap();
    println!("\nJSON metrics:\n{}", json_output);
}

#[cfg_attr(feature = "ci-perf", test)]
#[allow(dead_code)]
fn test_kind_specific_refresh_performance() {
    let cache_dir = get_test_cache_dir("kind-refresh");
    let workspace_dir = get_workspace_dir();

    // Test different environment kinds
    let kinds = ["Conda", "Venv", "VirtualEnv", "Pyenv"];

    println!(
        "=== Kind-Specific Refresh Performance ({} iterations per kind) ===",
        STAT_ITERATIONS
    );

    let mut all_kind_stats: HashMap<String, Value> = HashMap::new();

    for kind in kinds {
        let mut server_duration_stats = StatisticalMetrics::new();
        let mut env_count = 0usize;

        println!("\n  Testing kind: {}", kind);

        for i in 0..STAT_ITERATIONS {
            let mut client = PetClient::spawn().expect("Failed to spawn server");

            let config = json!({
                "workspaceDirectories": [workspace_dir.clone()],
                "cacheDirectory": cache_dir.clone()
            });

            client.configure(config).expect("Failed to configure");

            let (result, _) = client
                .refresh(Some(json!({ "searchKind": kind })))
                .unwrap_or_else(|_| panic!("Failed to refresh for kind {}", kind));

            let environments = client.get_environments();
            server_duration_stats.add(result.duration);
            env_count = environments.len();

            println!(
                "    Iteration {}: {}ms, {} envs",
                i + 1,
                result.duration,
                environments.len()
            );
        }

        server_duration_stats.print_summary(&format!("  {}", kind));
        println!("  {} environments found: {}", kind, env_count);

        all_kind_stats.insert(
            kind.to_string(),
            json!({
                "duration": server_duration_stats.to_json(),
                "environments_count": env_count
            }),
        );
    }

    // Output JSON for CI
    let json_output = serde_json::to_string_pretty(&json!(all_kind_stats)).unwrap();
    println!("\nJSON metrics:\n{}", json_output);
}

#[cfg_attr(feature = "ci-perf", test)]
#[allow(dead_code)]
fn test_resolve_performance() {
    let mut cold_resolve_stats = StatisticalMetrics::new();
    let mut warm_resolve_stats = StatisticalMetrics::new();

    let cache_dir = get_test_cache_dir("resolve");
    let workspace_dir = get_workspace_dir();

    println!(
        "=== Resolve Performance ({} iterations) ===",
        STAT_ITERATIONS
    );

    // First, find an executable to test with (use a single server)
    let exe_to_test: String;
    {
        let mut client = PetClient::spawn().expect("Failed to spawn server");
        let config = json!({
            "workspaceDirectories": [workspace_dir.clone()],
            "cacheDirectory": cache_dir.clone()
        });
        client.configure(config).expect("Failed to configure");
        client.refresh(None).expect("Failed to refresh");
        let environments = client.get_environments();

        if environments.is_empty() {
            println!("No environments found to test resolve performance");
            return;
        }

        let env_with_exe = environments.iter().find(|e| e.executable.is_some());
        if let Some(env) = env_with_exe {
            exe_to_test = env.executable.as_ref().unwrap().clone();
        } else {
            println!("No environment with executable found");
            return;
        }
    }

    println!("Testing with executable: {}", exe_to_test);

    // Cold resolve tests (fresh server each time)
    println!("\n  Cold resolve iterations:");
    for i in 0..STAT_ITERATIONS {
        let mut client = PetClient::spawn().expect("Failed to spawn server");
        let config = json!({
            "workspaceDirectories": [workspace_dir.clone()],
            "cacheDirectory": cache_dir.clone()
        });
        client.configure(config).expect("Failed to configure");

        let (_, cold_time) = client
            .resolve(&exe_to_test)
            .expect("Failed to resolve (cold)");
        cold_resolve_stats.add(cold_time.as_millis());
        println!("    Iteration {}: {}ms", i + 1, cold_time.as_millis());
    }

    // Warm resolve tests (same server, multiple resolves)
    println!("\n  Warm resolve iterations:");
    {
        let mut client = PetClient::spawn().expect("Failed to spawn server");
        let config = json!({
            "workspaceDirectories": [workspace_dir.clone()],
            "cacheDirectory": cache_dir.clone()
        });
        client.configure(config).expect("Failed to configure");

        // Prime the cache with a first resolve
        client.resolve(&exe_to_test).expect("Failed to prime cache");

        for i in 0..STAT_ITERATIONS {
            let (_, warm_time) = client
                .resolve(&exe_to_test)
                .expect("Failed to resolve (warm)");
            warm_resolve_stats.add(warm_time.as_millis());
            println!("    Iteration {}: {}ms", i + 1, warm_time.as_millis());
        }
    }

    println!();
    cold_resolve_stats.print_summary("Cold resolve");
    warm_resolve_stats.print_summary("Warm resolve");

    // Calculate speedup
    if let (Some(cold_p50), Some(warm_p50)) = (cold_resolve_stats.p50(), warm_resolve_stats.p50()) {
        if warm_p50 > 0 {
            println!(
                "Cache speedup (P50): {:.2}x",
                cold_p50 as f64 / warm_p50 as f64
            );
        }
    }

    // Output JSON for CI
    let json_output = serde_json::to_string_pretty(&json!({
        "cold_resolve": cold_resolve_stats.to_json(),
        "warm_resolve": warm_resolve_stats.to_json()
    }))
    .unwrap();
    println!("\nJSON metrics:\n{}", json_output);
}

#[cfg_attr(feature = "ci-perf", test)]
#[allow(dead_code)]
fn test_concurrent_resolve_performance() {
    let mut client = PetClient::spawn().expect("Failed to spawn server");

    let cache_dir = get_test_cache_dir("concurrent-resolve");
    let workspace_dir = get_workspace_dir();

    let config = json!({
        "workspaceDirectories": [workspace_dir],
        "cacheDirectory": cache_dir
    });

    client.configure(config).expect("Failed to configure");

    // First, discover environments
    client.refresh(None).expect("Failed to refresh");
    let environments = client.get_environments();

    // Get up to 5 environments with executables
    let exes: Vec<String> = environments
        .iter()
        .filter_map(|e| e.executable.clone())
        .take(5)
        .collect();

    if exes.is_empty() {
        println!("No environments with executables found");
        return;
    }

    println!("=== Sequential Resolve Performance ===");
    println!("Resolving {} executables sequentially", exes.len());

    let start = Instant::now();
    for exe in &exes {
        client.resolve(exe).expect("Failed to resolve");
    }
    let sequential_time = start.elapsed();
    println!("Sequential time: {:?}", sequential_time);
    println!(
        "Average per resolve: {:?}",
        sequential_time / exes.len() as u32
    );
}

#[cfg_attr(feature = "ci-perf", test)]
#[allow(dead_code)]
fn test_refresh_warm_vs_cold_cache() {
    // Clean cache directory
    let cache_dir = get_test_cache_dir("warm-vs-cold");
    let _ = std::fs::remove_dir_all(&cache_dir);
    std::fs::create_dir_all(&cache_dir).expect("Failed to create cache dir");

    let workspace_dir = get_workspace_dir();

    println!("=== Cold vs Warm Cache Performance ===");

    // Cold cache test
    {
        let mut client = PetClient::spawn().expect("Failed to spawn server");
        let config = json!({
            "workspaceDirectories": [workspace_dir.clone()],
            "cacheDirectory": cache_dir.clone()
        });
        client.configure(config).expect("Failed to configure");

        let (result, elapsed) = client.refresh(None).expect("Failed to refresh");
        println!(
            "Cold cache: {}ms (server), {:?} (client)",
            result.duration, elapsed
        );
    }

    // Warm cache test (reuse same cache directory)
    {
        let mut client = PetClient::spawn().expect("Failed to spawn server");
        let config = json!({
            "workspaceDirectories": [workspace_dir],
            "cacheDirectory": cache_dir
        });
        client.configure(config).expect("Failed to configure");

        let (result, elapsed) = client.refresh(None).expect("Failed to refresh");
        println!(
            "Warm cache: {}ms (server), {:?} (client)",
            result.duration, elapsed
        );
    }
}

#[cfg_attr(feature = "ci-perf", test)]
#[allow(dead_code)]
fn test_performance_summary() {
    let mut startup_stats = StatisticalMetrics::new();
    let mut cold_refresh_stats = StatisticalMetrics::new();
    let mut warm_refresh_stats = StatisticalMetrics::new();
    let mut cold_time_to_first_env_stats = StatisticalMetrics::new();
    let mut warm_time_to_first_env_stats = StatisticalMetrics::new();
    let mut phase_stats = BTreeMap::new();
    let mut locator_stats = BTreeMap::new();
    let mut probe_timeout_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut expected_inventory = None;

    let cache_root = get_test_cache_dir("performance-summary");
    reset_cache_dir(&cache_root);
    let workspace_dir = get_workspace_dir();

    println!("\n========================================");
    println!(
        "  COLD/WARM PERFORMANCE SUMMARY ({} pairs)",
        STAT_ITERATIONS
    );
    println!("========================================\n");

    for iteration in 0..STAT_ITERATIONS {
        let iteration_cache = benchmark_iteration_cache_dir(&cache_root, "measured", iteration);
        reset_cache_dir(&iteration_cache);

        let spawn_start = Instant::now();
        let mut cold_client = PetClient::spawn().expect("Failed to spawn cold server");
        cold_client
            .configure(json!({
                "workspaceDirectories": [workspace_dir.clone()],
                "cacheDirectory": iteration_cache.clone(),
            }))
            .expect("Failed to configure cold server");
        let startup_time = spawn_start.elapsed().as_millis();
        startup_stats.add(startup_time);

        let (cold_result, _) = cold_client
            .refresh(None)
            .expect("Failed to run cold refresh");
        cold_refresh_stats.add(cold_result.duration);
        let cold_inventory = (
            cold_client.get_environments().len(),
            cold_client.get_managers().len(),
        );
        assert_stable_inventory(
            &mut expected_inventory,
            cold_inventory,
            "Cold refresh",
            iteration,
        );
        let cold_ttfe = cold_client.time_to_first_env().unwrap_or_else(|| {
            panic!(
                "Cold refresh iteration {} produced no environment notification",
                iteration + 1
            )
        });
        cold_time_to_first_env_stats.add(cold_ttfe.as_millis());
        record_interpreter_probe_timeouts(&cold_client, &mut probe_timeout_counts);

        println!(
            "  Cold iteration {}: startup={}ms, refresh={}ms, envs={}",
            iteration + 1,
            startup_time,
            cold_result.duration,
            cold_inventory.0,
        );
        drop(cold_client);

        let mut warm_client = PetClient::spawn().expect("Failed to spawn warm server");
        warm_client
            .configure(json!({
                "workspaceDirectories": [workspace_dir.clone()],
                "cacheDirectory": iteration_cache,
            }))
            .expect("Failed to configure warm server");
        let (warm_result, _) = warm_client
            .refresh(None)
            .expect("Failed to run warm refresh");
        warm_refresh_stats.add(warm_result.duration);
        let warm_inventory = (
            warm_client.get_environments().len(),
            warm_client.get_managers().len(),
        );
        assert_stable_inventory(
            &mut expected_inventory,
            warm_inventory,
            "Warm refresh",
            iteration,
        );
        let warm_ttfe = warm_client.time_to_first_env().unwrap_or_else(|| {
            panic!(
                "Warm refresh iteration {} produced no environment notification",
                iteration + 1
            )
        });
        warm_time_to_first_env_stats.add(warm_ttfe.as_millis());
        record_interpreter_probe_timeouts(&warm_client, &mut probe_timeout_counts);

        println!(
            "  Warm iteration {}: refresh={}ms, envs={}",
            iteration + 1,
            warm_result.duration,
            warm_inventory.0,
        );
    }

    let (env_count, manager_count) =
        expected_inventory.expect("Performance summary must run at least one iteration");
    for (label, count) in [
        ("startup", startup_stats.count()),
        ("cold refresh", cold_refresh_stats.count()),
        ("warm refresh", warm_refresh_stats.count()),
        ("cold time-to-first", cold_time_to_first_env_stats.count()),
        ("warm time-to-first", warm_time_to_first_env_stats.count()),
    ] {
        assert_eq!(
            count, STAT_ITERATIONS,
            "Expected one {label} sample per benchmark pair"
        );
    }
    collect_refresh_diagnostics(
        &workspace_dir,
        &cache_root,
        &mut phase_stats,
        &mut locator_stats,
        &mut probe_timeout_counts,
        (env_count, manager_count),
    );

    for phase in ["locators", "path", "globalVirtualEnvs", "workspaces"] {
        let count = phase_stats
            .get(phase)
            .map(StatisticalMetrics::count)
            .unwrap_or_default();
        assert_eq!(
            count, STAT_ITERATIONS,
            "Expected one completed {phase} phase per refresh iteration"
        );
    }
    assert!(
        !locator_stats.is_empty(),
        "Expected per-locator timing in RefreshProgress telemetry"
    );

    // Print statistical summary
    println!("\n----------------------------------------");
    println!("             STATISTICS                 ");
    println!("----------------------------------------");
    startup_stats.print_summary("Server startup");
    cold_refresh_stats.print_summary("Cold full refresh");
    warm_refresh_stats.print_summary("Warm full refresh");
    cold_time_to_first_env_stats.print_summary("Cold time to first env");
    warm_time_to_first_env_stats.print_summary("Warm time to first env");
    for (phase, metrics) in &phase_stats {
        metrics.print_summary(&format!("Phase {phase}"));
    }
    for (locator, metrics) in &locator_stats {
        metrics.print_summary(&format!("Locator {locator}"));
    }
    println!("Environments found:    {}", env_count);
    println!("Managers found:        {}", manager_count);
    println!("========================================\n");

    let phase_json = statistics_json(&phase_stats);
    let locator_json = statistics_json(&locator_stats);

    // Output as JSON for CI parsing
    // Existing top-level refresh fields remain warm-cache values for schema compatibility.
    let json_output = serde_json::to_string_pretty(&json!({
        "metrics_schema_version": PERFORMANCE_METRICS_SCHEMA_VERSION,
        "server_startup_ms": startup_stats.p50().unwrap_or(0),
        "full_refresh_ms": warm_refresh_stats.p50().unwrap_or(0),
        "cold_refresh_ms": cold_refresh_stats.p50().unwrap_or(0),
        "time_to_first_env_ms": warm_time_to_first_env_stats.p50(),
        "cold_time_to_first_env_ms": cold_time_to_first_env_stats.p50(),
        "environments_count": env_count,
        "managers_count": manager_count,
        "stats": {
            "server_startup": startup_stats.to_json(),
            "full_refresh": warm_refresh_stats.to_json(),
            "cold_refresh": cold_refresh_stats.to_json(),
            "time_to_first_env": warm_time_to_first_env_stats.to_json(),
            "cold_time_to_first_env": cold_time_to_first_env_stats.to_json(),
        },
        "phases": phase_json,
        "locators": locator_json,
        "interpreter_probe_timeouts": probe_timeout_counts
    }))
    .unwrap();

    println!("JSON metrics:\n{}", json_output);
}
