use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use collections::HashMap;
use gpui::{App, Context, Entity, Task};
use language::LanguageServerId;
use lsp::LanguageServerName;
use project::Project;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};

/// How often the collector refreshes process stats (in seconds).
const POLL_INTERVAL_SECS: u64 = 2;

/// A single process row in the resource monitor.
#[derive(Debug, Clone)]
pub struct ProcessEntry {
    pub category: ProcessCategory,
    pub name: String,
    /// e.g. language name, worktree name, or empty.
    pub context: Option<String>,
    pub pid: Option<u32>,
    pub cpu_percent: f32,
    pub memory_bytes: u64,
    pub started_at: Option<Instant>,
    /// What actions are available for this process.
    pub actions: ProcessActions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProcessCategory {
    MainProcess,
    LanguageServer,
    Terminal,
}

impl ProcessCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::MainProcess => "Zed",
            Self::LanguageServer => "Language Servers",
            Self::Terminal => "Terminals",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProcessActions {
    LanguageServer {
        server_id: LanguageServerId,
        server_name: LanguageServerName,
    },
    Terminal {
        entity_id: gpui::EntityId,
    },
    None,
}

/// A snapshot of all process data at a point in time.
#[derive(Debug, Clone, Default)]
pub struct ProcessSnapshot {
    pub entries: Vec<ProcessEntry>,
    pub total_cpu_percent: f32,
    pub total_memory_bytes: u64,
    pub timestamp: Option<Instant>,
}

/// Internal struct used to pass PID info from the main thread to the
/// background sysinfo refresh.
#[derive(Debug, Clone)]
struct PidSource {
    pid: u32,
    category: ProcessCategory,
    name: String,
    context: Option<String>,
    actions: ProcessActions,
}

/// Background entity that polls `sysinfo` for process metrics.
/// Only runs while at least one `ResourceMonitorView` is alive.
pub struct ProcessCollector {
    project: Entity<Project>,
    snapshot: ProcessSnapshot,
    system: Arc<Mutex<System>>,
    _poll_task: Task<()>,
}

impl ProcessCollector {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let system = Arc::new(Mutex::new(System::new()));

        let poll_task = Self::start_polling(project.clone(), system.clone(), cx);

        Self {
            project,
            snapshot: ProcessSnapshot::default(),
            system,
            _poll_task: poll_task,
        }
    }

    pub fn snapshot(&self) -> &ProcessSnapshot {
        &self.snapshot
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    fn start_polling(
        project: Entity<Project>,
        system: Arc<Mutex<System>>,
        cx: &mut Context<Self>,
    ) -> Task<()> {
        cx.spawn(async move |this, cx| {
            loop {
                // 1. Collect PIDs from the project on the main thread.
                let sources = this.update(cx, |this, cx| {
                    this.collect_pid_sources(cx)
                });
                let Ok(sources) = sources else { break };

                // 2. Build the list of PIDs to refresh.
                let pids: Vec<Pid> = sources
                    .iter()
                    .map(|s| Pid::from_u32(s.pid))
                    .collect();

                // 3. Refresh sysinfo on a background thread.
                let sys = system.clone();
                let stats = cx.background_spawn(async move {
                    let mut system = sys.lock().unwrap();

                    // First do a targeted refresh for our PIDs (CPU + memory).
                    let refresh_kind = ProcessRefreshKind::nothing()
                        .with_cpu()
                        .with_memory();
                    system.refresh_processes_specifics(
                        ProcessesToUpdate::Some(&pids),
                        true,
                        refresh_kind,
                    );

                    // Now build a parent map for descendant memory calculation.
                    // We need all processes for this, do a lightweight refresh.
                    let all_refresh = RefreshKind::nothing()
                        .with_processes(ProcessRefreshKind::nothing().without_tasks().with_memory());
                    system.refresh_specifics(all_refresh);

                    let parent_map: HashMap<Pid, Pid> = system
                        .processes()
                        .iter()
                        .filter_map(|(&pid, process)| Some((pid, process.parent()?)))
                        .collect();

                    // For each source PID, compute tree-aggregated memory and CPU.
                    let mut results: HashMap<u32, (f32, u64)> = HashMap::default();
                    for pid in &pids {
                        let cpu = system
                            .process(*pid)
                            .map(|p| p.cpu_usage())
                            .unwrap_or(0.0);
                        let memory: u64 = system
                            .processes()
                            .iter()
                            .filter(|(p, _)| is_descendant_of(**p, *pid, &parent_map))
                            .map(|(_, process)| process.memory())
                            .sum();
                        results.insert(pid.as_u32(), (cpu, memory));
                    }
                    results
                }).await;

                // 4. Build entries and update snapshot on the main thread.
                let update_result = this.update(cx, |this, cx| {
                    let mut entries: Vec<ProcessEntry> = sources
                        .into_iter()
                        .map(|source| {
                            let (cpu, mem) = stats
                                .get(&source.pid)
                                .copied()
                                .unwrap_or((0.0, 0));
                            ProcessEntry {
                                category: source.category,
                                name: source.name,
                                context: source.context,
                                pid: Some(source.pid),
                                cpu_percent: cpu,
                                memory_bytes: mem,
                                started_at: None,
                                actions: source.actions,
                            }
                        })
                        .collect();

                    // Sort by category then by CPU (descending).
                    entries.sort_by(|a, b| {
                        a.category
                            .cmp(&b.category)
                            .then(b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap_or(std::cmp::Ordering::Equal))
                    });

                    let total_cpu: f32 = entries.iter().map(|e| e.cpu_percent).sum();
                    let total_mem: u64 = entries.iter().map(|e| e.memory_bytes).sum();

                    this.snapshot = ProcessSnapshot {
                        entries,
                        total_cpu_percent: total_cpu,
                        total_memory_bytes: total_mem,
                        timestamp: Some(Instant::now()),
                    };
                    cx.notify();
                });
                if update_result.is_err() {
                    break;
                }

                // 5. Sleep before next poll.
                cx.background_spawn(async move {
                    smol::Timer::after(Duration::from_secs(POLL_INTERVAL_SECS)).await;
                }).await;
            }
        })
    }

    /// Gather PIDs for all known Zed-managed processes.
    fn collect_pid_sources(&self, cx: &App) -> Vec<PidSource> {
        let mut sources = Vec::new();

        // 1. Main Zed process.
        sources.push(PidSource {
            pid: std::process::id(),
            category: ProcessCategory::MainProcess,
            name: "Zed".into(),
            context: None,
            actions: ProcessActions::None,
        });

        // 2. Language servers.
        let project = self.project.read(cx);
        let lsp_store = project.lsp_store().read(cx);
        for (server_id, status) in lsp_store.language_server_statuses() {
            if let Some(pid) = status.process_id {
                let worktree_name = status
                    .worktree
                    .and_then(|wt_id| project.worktree_for_id(wt_id, cx))
                    .map(|wt| wt.read(cx).root_name_str().to_string());

                sources.push(PidSource {
                    pid,
                    category: ProcessCategory::LanguageServer,
                    name: status.name.to_string(),
                    context: worktree_name,
                    actions: ProcessActions::LanguageServer {
                        server_id,
                        server_name: status.name.clone(),
                    },
                });
            }
        }

        // 3. Terminals.
        for weak_terminal in project.local_terminal_handles() {
            if let Some(terminal) = weak_terminal.upgrade() {
                let term = terminal.read(cx);
                if let Some(pid) = term.pid() {
                    sources.push(PidSource {
                        pid: pid.as_u32(),
                        category: ProcessCategory::Terminal,
                        name: term.title(true),
                        context: None,
                        actions: ProcessActions::Terminal {
                            entity_id: terminal.entity_id(),
                        },
                    });
                }
            }
        }

        sources
    }
}

/// Checks whether `pid` is a descendant of `root_pid` using the parent map.
fn is_descendant_of(pid: Pid, root_pid: Pid, parent_map: &HashMap<Pid, Pid>) -> bool {
    let mut current = pid;
    let mut depth = 0;
    while current != root_pid {
        if depth > 100 {
            // Guard against cycles.
            return false;
        }
        match parent_map.get(&current) {
            Some(&parent) => current = parent,
            None => return false,
        }
        depth += 1;
    }
    true
}
