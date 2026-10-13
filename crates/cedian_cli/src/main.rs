//! `cedian` CLI harness (throwaway): the full agent loop without GPUI.
//!
//! Wires the headless crates end-to-end in one process:
//! `OmpRuntime` + `HostTools` + `Panel`. Commands mirror the future palette
//! entries (§2.5) so the wiring transfers to the app shell. Reviewing hunks
//! (accept, reject, revert a turn) lives in the app's panel, not here.
//!
//! Usage:
//! ```text
//! cedian prompt "fix the typo"        # one turn: prompt → stream → cards
//! cedian review                       # list the reviewer's findings
//! cedian review --agent [focus]       # run the reviewer on the task's hunks
//! cedian review dismiss <id> <reason> # close a finding, audited
//! cedian review reset                 # drop the review task (new baseline next prompt)
//! cedian state                        # session snapshot (model, streaming, queue)
//! ```
//!
//! State lives in-process per invocation EXCEPT the OMP session (adopted via
//! `--session-dir` + `open_session`), workspace files, and the review task in
//! `review.json` in the state dir (baseline + turn log, see `session.rs`).
//! `cedian review reset` starts a new review task.
#![allow(
    clippy::disallowed_methods,
    reason = "headless, synchronous process control (OMP, LSP, DAP, Chrome, git, sandbox-exec): Zed's async spawn helpers do not apply"
)]

mod session;
mod shell;
mod shell_lock;
mod state;
#[cfg(test)]
mod test_dir;

use cedian_agent_ui::Panel;
use cedian_omp::{OmpBinary, OmpRuntime, RuntimeConfig};
use cedian_review::FileDiff;
use cedian_shell::{
    corrections, review_agent, review_findings, timing, workflow_host, workflow_store,
    workspace_files,
};
use cedian_workspace::{HostTools, WorkspaceHost};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn main() {
    let result = run(std::env::args().skip(1).collect());
    if let Err(e) = result {
        eprintln!("cedian: {e}");
        std::process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    dispatch(args, false)
}

/// Read-only verbs: allowed while a `cedian shell` holds the workspace.
/// Everything else mutates a store or a live resource (ADR-0021 §4).
fn is_read_only(args: &[String]) -> bool {
    let sub = args.get(1).map(String::as_str);
    match args.first().map(String::as_str).unwrap_or("help") {
        "review" => sub.is_none(),
        "state" | "palette" | "help" | "shell" => true,
        "workflow" => sub == Some("status"),
        "worker" => sub == Some("preview"),
        _ => false,
    }
}

/// Run one command. `in_shell` = issued inside `cedian shell`, which holds
/// the workspace lock itself, so the one-shot lock gate does not apply.
pub(crate) fn dispatch(args: Vec<String>, in_shell: bool) -> Result<(), String> {
    let session_dir = std::env::var("CEDIAN_SESSION_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("cedian-cli-session"));
    // Canonicalize: `/tmp` → `/private/tmp` on macOS (else rootUri,
    // buffer keys, and didOpen URIs disagree and diagnostics never match).
    let workdir: PathBuf = std::env::var("CEDIAN_WORKDIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    let workdir = workdir.canonicalize().unwrap_or(workdir);

    let cmd = args.first().map(|s| s.as_str()).unwrap_or("help");
    if !in_shell && !is_read_only(&args) {
        shell_lock::refuse_if_shell_live(&workdir, &args.join(" "))?;
    }
    // One settings document per command (the user's `cedian.toml`, ADR-0018).
    // The dangerous tier Deny refuses prompt (the shell rule).
    let settings = cedian_shell::resolve_settings(&workdir).map_err(|e| e.to_string())?;
    match cmd {
        "prompt" => {
            let message = args.get(1).ok_or("usage: cedian prompt <message>")?;
            if settings.permissions.dangerous == cedian_shell::Verdict::Deny {
                return Err("refused: settings [permissions] dangerous = deny".to_string());
            }
            cmd_prompt(&session_dir, &workdir, &settings, message)
        }
        "review" => match args.get(1).map(|s| s.as_str()) {
            None => cmd_review(&workdir),
            Some("reset") => {
                session::reset(&workdir);
                println!("review task reset (next prompt takes a fresh baseline)");
                Ok(())
            }
            Some("dismiss") => {
                let usage = "usage: cedian review dismiss <finding id> <reason>";
                let id = args.get(2).ok_or(usage)?;
                let reason = args[3.min(args.len())..].join(" ");
                let turn_touching = |path: &str| {
                    Ok(session::load(&workdir)?.and_then(|r| r.last_turn_touching(path)))
                };
                println!(
                    "{}",
                    review_findings::dismiss(
                        &state::dir(&workdir)?,
                        session::CLI_TASK,
                        id,
                        &reason,
                        turn_touching
                    )?
                );
                Ok(())
            }
            Some("--agent") => {
                let focus = args[2.min(args.len())..].join(" ");
                let channel = workflow_channel(&workdir, &settings, |_, _| None)?;
                let requester = review_agent::Requester {
                    channel: Some(&channel),
                    ..Default::default()
                };
                let reply = review_agent::run_review(
                    &review_place(&workdir, &session_dir)?,
                    &settings,
                    &focus,
                    read_review_task(&workdir)?,
                    task_diff_reader(&workdir),
                    requester,
                )?;
                println!("{reply}");
                Ok(())
            }
            Some(other) => Err(format!(
                "usage: cedian review [reset | --agent [focus] | dismiss <id> <reason>] (got {other:?})"
            )),
        },
        "state" => cmd_state(&session_dir, &workdir, &settings),
        "shell" if in_shell => Err("already inside cedian shell".to_string()),
        "shell" => shell::run(&session_dir, &workdir, &settings),
        "palette" => {
            let query = args.get(1).map(|s| s.as_str()).unwrap_or("");
            for action in cedian_shell::Palette::filter(query) {
                println!("{} — {} ({})", action.id, action.title, action.hint);
            }
            Ok(())
        }
        "workflow" => cmd_workflow(&workdir, &settings, &args[1..]),
        "worker" => cmd_worker(&workdir, &args[1..]),
        _ => {
            eprintln!(
                "usage: cedian <prompt|shell|review|state|\
                palette|workflow|worker> …"
            );
            eprintln!("env: CEDIAN_SESSION_DIR, CEDIAN_WORKDIR, CEDIAN_OMP_BINARY");
            Ok(())
        }
    }
}

/// The complete cedian host-tool set for these settings (ADR-0004: one
/// `set_host_tools` call replaces the whole set, so register all of it).
/// `cedian_worktree_request` writes `.worktrees/` + a branch: present only
/// when project writes are allowed.
fn host_tool_names(settings: &cedian_shell::Settings) -> Vec<&'static str> {
    let mut names = vec![
        cedian_workflow::WORKFLOW_UPDATE_TOOL,
        cedian_workflow::COMPLETE_TOOL,
        review_agent::REVIEW_REQUEST_TOOL,
        corrections::CORRECTION_CLASS_TOOL,
    ];
    if cedian_shell::launch::registers_worktree_request(settings) {
        names.push(cedian_worker::WORKTREE_REQUEST_TOOL);
    }
    names
}

/// The workflow channel over `workdir`'s store, with the user's floor and
/// the project's verification profiles. `resolve` binds reported evidence
/// to a finished tool call; the CLI's own paths pass one that binds none.
fn workflow_channel(
    workdir: &Path,
    settings: &cedian_shell::Settings,
    resolve: impl Fn(&str, &str) -> Option<cedian_workflow::BoundCall> + Send + Sync + 'static,
) -> Result<std::sync::Arc<cedian_workflow::WorkflowChannel>, String> {
    Ok(workflow_host::channel(
        session::CLI_TASK,
        workdir,
        &state::dir(workdir)?,
        settings,
        resolve,
        |_| {},
    ))
}

/// Turn boundary (ADR-0036): a refused `cedian_complete` in this turn
/// blocks the workflow (or leaves it failed when the agent failed a phase);
/// say so with the missing gates.
fn end_workflow_turn(workdir: &Path) -> Result<(), String> {
    let next_turn = session::load(workdir)?
        .map(|s| s.turns.len() as u32 + 1)
        .unwrap_or(1);
    let Some((status, missing)) =
        workflow_host::end_turn(&state::dir(workdir)?, session::CLI_TASK, Some(next_turn))?
    else {
        return Ok(());
    };
    let (word, next) = if status == cedian_workflow::WorkflowStatus::Failed {
        ("FAILED", "start a new workflow to retry")
    } else {
        ("BLOCKED", "`cedian workflow resume` to continue")
    };
    println!("[!] workflow {word}: the agent claimed done with required gates unmet");
    for m in &missing {
        println!("    - {m}");
    }
    println!("    (`cedian workflow status` for the claims; {next})");
    Ok(())
}

/// The workspace hashed now (ADR-0024; headless code state, row H).
fn current_state(workdir: &Path) -> cedian_workflow::CurrentState {
    workflow_host::current_state(workdir)
}

/// Every host tool, with the P5 evidence resolver over this runtime's router
/// log: evidence binds to the most recent call of the named tool (args
/// containing `needle`) that finished without error and is not itself a
/// channel report (ADR-0022, ADR-0031).
fn host_tools(
    rt: &OmpRuntime,
    session_dir: &Path,
    workdir: &Path,
    settings: &cedian_shell::Settings,
) -> Result<Vec<omp_rpc::HostTool>, String> {
    let router = rt.router();
    let resolve = move |tool: &str, needle: &str| {
        workflow_host::bound_call(&router.finished_tool_calls(), tool, needle)
    };
    let channel = workflow_channel(workdir, settings, resolve)?;
    let root = workdir.to_path_buf();
    let state_dir = state::dir(workdir)?;
    let blockers_dir = state_dir.clone();
    channel.set_blockers(move || {
        // Bring this turn's edits in first: a hunk fixed this turn is fixed.
        match flush_turn_for_review(&root) {
            Ok(()) => review_findings::open_blockers(&blockers_dir, || {
                task_diffs(&root).map(|(_, diffs)| diffs)
            }),
            Err(e) => vec![format!("review: cannot bring this turn's edits in ({e})")],
        }
    });
    let names = host_tool_names(settings);
    let mut tools = channel.host_tools();
    let root = workdir.to_path_buf();
    tools.push(review_agent::review_request_tool(
        review_place(workdir, session_dir)?,
        settings.clone(),
        std::sync::Arc::new(move || {
            flush_turn_for_review(&root)?;
            read_review_task(&root)
        }),
        task_diff_reader(workdir),
        {
            let router = rt.router();
            std::sync::Arc::new(move || router.answered_models())
        },
        std::sync::Arc::clone(&channel),
    ));
    let class_root = workdir.to_path_buf();
    tools.push(corrections::correction_class_tool(
        state_dir.clone(),
        move || current_state(&class_root),
    ));
    if names.contains(&cedian_worker::WORKTREE_REQUEST_TOOL) {
        tools.push(cedian_worker::worktree_request_tool(
            workdir.to_path_buf(),
            state_dir,
        ));
    }
    debug_assert_eq!(tools.len(), names.len());
    Ok(tools)
}

/// Spawn the runtime with workspace host tools + cedian:// wired.
fn spawn(
    session_dir: &Path,
    workdir: &Path,
    settings: &cedian_shell::Settings,
    host: &std::sync::Arc<HostTools>,
) -> Result<OmpRuntime, String> {
    let omp = cedian_shell::launch::omp_binary()?;
    if let Some(warning) = &omp.warning {
        println!("warning: {warning}");
    }
    let binary = OmpBinary::Bundled(omp.binary.clone());
    // Every spawn here has a person at the terminal; reviewers and
    // automations will pass `RunKind::Unattended` (ADR-0035 decision 5).
    let choice = settings.policy_for(workdir, cedian_shell::RunKind::Interactive);
    for note in &choice.notes {
        println!("note: {note}");
    }
    if choice.policy == cedian_shell::Policy::Omp {
        println!(
            "{}",
            cedian_shell::launch::omp_policy_badge(&omp.binary, workdir)
        );
        let p = &settings.permissions;
        if [p.safe, p.project_write, p.dangerous].contains(&cedian_shell::Verdict::Ask) {
            println!(
                "note: [permissions] ask tiers do not apply under policy = \"omp\" (OMP decides); deny tiers still do"
            );
        }
    }
    let mut policy =
        cedian_shell::launch::spawn_policy(settings, choice.policy, &host_tool_names(settings));
    if choice.policy == cedian_shell::Policy::Cedian {
        policy.config_allows = cedian_shell::launch::config_allows(&omp.binary, workdir)?;
    }
    let rt = OmpRuntime::spawn(RuntimeConfig {
        binary,
        session_dir: session_dir.to_path_buf(),
        sessions: cedian_omp::Sessions::InSessionDir,
        cwd: workdir.to_path_buf(),
        ask_dialog: true,
        prompt_timeout: Duration::from_secs(600),
        policy,
    })
    .map_err(|e| e.to_string())?;
    // Nothing in the CLI can answer an OMP dialog: refuse at once (P5 gap).
    rt.deny_ui_requests();
    rt.set_host_tools(host_tools(&rt, session_dir, workdir, settings)?)
        .map_err(|e| e.to_string())?;
    rt.set_host_uris(vec![host.cedian_uri_scheme()])
        .map_err(|e| e.to_string())?;
    Ok(rt)
}

/// Load every text file under workdir into the host buffers (headless scan).
/// Re-read each turn: in `cedian shell` the host lives across turns, and
/// OMP's own writes land on disk (row G).
fn load_workspace(host: &HostTools, workdir: &Path) -> Vec<PathBuf> {
    workspace_files::scan_text_files(workdir)
        .into_iter()
        .filter_map(|path| {
            let key = workspace_files::buffer_key(workdir, &path)?;
            let text = std::fs::read_to_string(&path).ok()?;
            host.open(&key, &text);
            Some(key)
        })
        .collect()
}

/// One ambient line while a workflow is open (ADR-0022: the model must be
/// told when to call the channel; compliance is checked, never assumed).
fn workflow_ambient(workdir: &Path) -> Option<String> {
    use cedian_workflow::WorkflowStatus;
    let state_dir = state::dir(workdir).ok()?;
    if !workflow_store::exists(&state_dir) {
        return None;
    }
    let state = workflow_store::load(&state_dir).ok()?;
    if !matches!(
        state.status,
        WorkflowStatus::Running | WorkflowStatus::Blocked
    ) {
        return None;
    }
    Some(format!(
        "cedian workflow active: {:?} ({:?}, {:?}, phase {}). Report evidence with {} \
         (from_tool = the tool whose call produced it) and call {} before saying it is done.",
        state.task.title,
        state.task.kind,
        state.status,
        state.current_phase.as_deref().unwrap_or("-"),
        cedian_workflow::WORKFLOW_UPDATE_TOOL,
        cedian_workflow::COMPLETE_TOOL,
    ))
}

fn cmd_prompt(
    session_dir: &Path,
    workdir: &Path,
    settings: &cedian_shell::Settings,
    message: &str,
) -> Result<(), String> {
    let host = HostTools::shared(workdir);
    let started = Instant::now();
    let mut rt = spawn(session_dir, workdir, settings, &host)?;
    rt.open_session("cli").map_err(|e| e.to_string())?;
    timing::record(timing::Row::Spawn(started.elapsed()));
    let result = run_turn(
        &mut rt,
        &host,
        workdir,
        message,
        false,
        session::TurnKind::Prompt,
        message,
    );
    let shutdown = rt.shutdown();
    result?;
    shutdown.map_err(|e| e.to_string())
}

/// One OMP turn on an already-running runtime: prompt → cards → write-back →
/// task baseline + turn log. Shared by one-shot `prompt` and `cedian shell`.
/// `live` streams assistant text to stdout as it arrives (the shell).
/// `kind` + `label` describe the turn in the turn log.
pub(crate) fn run_turn(
    rt: &mut OmpRuntime,
    host: &std::sync::Arc<HostTools>,
    workdir: &Path,
    message: &str,
    live: bool,
    kind: session::TurnKind,
    label: &str,
) -> Result<(), String> {
    let turn_started = Instant::now();
    let mut store = session::load(workdir)?.unwrap_or_default();
    let keys = load_workspace(host, workdir);

    // Pre-turn texts (disk == buffer right after load). The task baseline for
    // a file is its pre-turn text the FIRST time a turn changes it (§16), so
    // the store holds only files this task touched — user edits to other
    // files never enter review.
    let mut pre: HashMap<PathBuf, String> = HashMap::new();
    for key in &keys {
        if let Some(t) = host.read_buffer(key) {
            pre.insert(key.clone(), t);
        }
    }
    let _turn = TurnPre::set(&pre);

    let mut panel = Panel::new();
    let task_id = panel.new_task("cli", workdir.to_path_buf());
    let router = rt.router();
    let (sub, rx) = router.subscribe();
    let (settled_tx, settled_rx) = std::sync::mpsc::channel::<()>();

    let approvals = rt.approvals();
    let mut audit = cedian_shell::audit::AuditLog::open(&state::dir(workdir)?, approvals)?;

    // Pump router events into the panel on a thread while the turn runs.
    let pump = std::thread::spawn(move || {
        use std::io::Write as _;
        let mut panel = panel;
        let mut audit_error = None;
        for event in rx.iter() {
            if let Err(e) = audit.record(&event) {
                audit_error.get_or_insert(e);
            }
            if let cedian_omp::RouterEvent::MessageDelta {
                kind: cedian_omp::DeltaKind::Text,
                delta,
                ..
            } = &event
            {
                if live {
                    print!("{delta}");
                    let _ = std::io::stdout().flush();
                }
            }
            let done = matches!(event, cedian_omp::RouterEvent::Settled);
            panel.dispatch(&event);
            if done {
                let _ = settled_tx.send(());
                break;
            }
        }
        (panel, audit, audit_error)
    });

    // Ambient context travels with the prompt (§39).
    let ambient =
        cedian_workspace::render_snapshot(&cedian_workspace::capture_ambient(host.as_ref()));
    let ambient = match workflow_ambient(workdir) {
        Some(line) => format!("{ambient}{line}\n"),
        None => ambient,
    };
    let full = if ambient.is_empty() {
        message.to_string()
    } else {
        format!("{ambient}\n{message}")
    };
    let omp_started = Instant::now();
    let turn = rt.prompt(&full, vec![]);
    let omp_returned = Instant::now();
    // `Settled` follows `prompt_result`; give it a moment, then unsubscribe —
    // dropping our sender always ends the pump, even when the turn failed or
    // no `Settled` arrived.
    if turn.is_ok() {
        let _ = settled_rx.recv_timeout(Duration::from_secs(5));
    }
    router.unsubscribe(sub);
    drop(router);
    let (panel, mut audit, mut audit_error) =
        pump.join().map_err(|_| "pump thread died".to_string())?;
    let refused = rt.take_refused_ui_requests();
    for r in &refused {
        if let Err(e) = audit.dialog(r) {
            audit_error.get_or_insert(e);
        }
    }
    turn.map_err(|e| e.to_string())?;
    if let Some(e) = audit_error {
        return Err(format!(
            "{e} — the turn is not audited, so it fails (ADR-0035)"
        ));
    }

    // Render the turn from the thread the stream built, not from
    // `prompt_result`'s copy of the text (S0 exit).
    let task = panel.get(&task_id).ok_or("task vanished")?;
    let (messages, cards) = cedian_agent_ui::render_thread(task.thread().events());
    if live {
        println!();
    } else {
        for m in messages
            .iter()
            .filter(|m| m.role == cedian_agent_ui::MessageRole::Assistant && !m.text.is_empty())
        {
            println!("{}", m.text);
        }
    }
    for card in &cards {
        // Headless refuses every dialog, so an exec-tier call that completed
        // under the opt-in was approved by OMP's config, not by a person. A
        // call OMP blocks still starts and then ends in error: no label.
        let label = if approvals == cedian_omp::Approvals::Omp
            && cedian_omp::spawn_profile::EXEC_TOOLS.contains(&card.name.as_str())
            && card.status == cedian_agent_ui::ToolCardStatus::Done
        {
            " · approved by OMP"
        } else {
            ""
        };
        println!("[{}] {}{label}", card.status_glyph(), card.display_line());
    }
    for r in &refused {
        println!("[✗] refused (no UI to approve): {}", r.label);
    }
    end_workflow_turn(workdir)?;

    let mut turn_files = Vec::new();
    for (key, before, after) in changed_since(workdir, &pre) {
        store.baseline_once(&key, &before); // first change in task (new file: empty)
        turn_files.push(session::TurnFile {
            file: key.to_string_lossy().into_owned(),
            before,
            after,
            created: !pre.contains_key(&key),
        });
    }
    store.record_turn(kind, label, turn_files);
    store.models.extend(rt.router().answered_models());
    let saved = session::save(workdir, &store);
    timing::record(timing::Row::Turn {
        started: turn_started,
        sent: omp_started,
        returned: omp_returned,
        ended: Instant::now(),
    });
    saved
}

/// Files on disk whose text differs from their pre-turn text (a new file's
/// pre-turn text is empty): `(key, before, after)`.
fn changed_since(workdir: &Path, pre: &HashMap<PathBuf, String>) -> Vec<(PathBuf, String, String)> {
    workspace_files::scan_text_files(workdir)
        .into_iter()
        .filter_map(|path| {
            let key = workspace_files::buffer_key(workdir, &path)?;
            let after = std::fs::read_to_string(&path).ok()?;
            let before = pre.get(&key).cloned().unwrap_or_default();
            (after != before).then_some((key, before, after))
        })
        .collect()
}

/// The running turn's pre-turn texts, so a review asked for mid-turn
/// (`cedian_review_request`) can see this turn's changes. One turn runs at a
/// time in this process; the guard clears it when the turn ends.
static TURN_PRE: std::sync::Mutex<Option<HashMap<PathBuf, String>>> = std::sync::Mutex::new(None);

struct TurnPre;

impl TurnPre {
    fn set(pre: &HashMap<PathBuf, String>) -> Self {
        *TURN_PRE.lock().unwrap_or_else(|e| e.into_inner()) = Some(pre.clone());
        Self
    }
}

impl Drop for TurnPre {
    fn drop(&mut self) {
        *TURN_PRE.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
}

/// Bring this turn's changes into review before a mid-turn review:
/// baseline every file the turn changed on disk. Provenance is still
/// recorded when the turn ends. A no-op between turns.
fn flush_turn_for_review(workdir: &Path) -> Result<(), String> {
    let Some(pre) = TURN_PRE.lock().unwrap_or_else(|e| e.into_inner()).clone() else {
        return Ok(());
    };
    let mut store = session::load(workdir)?.unwrap_or_default();
    for (key, before, _) in changed_since(workdir, &pre) {
        store.baseline_once(&key, &before);
    }
    session::save(workdir, &store)
}

/// The task's diffs: baseline → current disk text for every file the task
/// changed. No task yet → none. A baselined file gone from disk reviews as
/// a deletion.
fn task_diffs(workdir: &Path) -> Result<(session::ReviewStore, Vec<FileDiff>), String> {
    let store = session::load(workdir)?.unwrap_or_default();
    let mut diffs = Vec::new();
    for (key, before) in &store.baseline {
        let after = workspace_files::local_path(workdir, Path::new(key))
            .and_then(|local| std::fs::read_to_string(local).ok())
            .unwrap_or_default();
        let hunks = cedian_review::line_diff(before, &after);
        diffs.push(FileDiff {
            path: key.clone(),
            statuses: vec![cedian_review::HunkStatus::Pending; hunks.len()],
            hunks,
            snapshot: after,
        });
    }
    Ok((store, diffs))
}

/// The task as a review reads it, from the stored task and disk.
fn read_review_task(workdir: &Path) -> Result<review_agent::ReviewTask, String> {
    let (store, diffs) = task_diffs(workdir)?;
    Ok(review_agent::ReviewTask {
        diffs,
        baseline: store
            .baseline
            .iter()
            .map(|(key, text)| (PathBuf::from(key), text.clone()))
            .collect(),
        editors: store.models.into_iter().collect(),
        editors_unknown: false,
    })
}

fn task_diff_reader(workdir: &Path) -> review_findings::TaskDiffs {
    let root = workdir.to_path_buf();
    std::sync::Arc::new(move || task_diffs(&root).map(|(_, diffs)| diffs))
}

fn review_place(workdir: &Path, session_dir: &Path) -> Result<review_agent::ReviewPlace, String> {
    Ok(review_agent::ReviewPlace {
        workdir: workdir.to_path_buf(),
        state_dir: state::dir(workdir)?,
        session_dir: session_dir.to_path_buf(),
    })
}

fn cmd_review(workdir: &Path) -> Result<(), String> {
    let findings = review_findings::states(&state::dir(workdir)?, || {
        task_diffs(workdir).map(|(_, d)| d)
    })?;
    if findings.is_empty() {
        println!("no review findings");
        return Ok(());
    }
    println!("findings:");
    for (f, state) in findings {
        println!(
            "  {} [{state}] {:?} {} hunk {}: {}",
            f.id, f.finding.severity, f.finding.path, f.hunk, f.finding.message
        );
    }
    Ok(())
}

fn cmd_state(
    session_dir: &Path,
    workdir: &Path,
    settings: &cedian_shell::Settings,
) -> Result<(), String> {
    let host = HostTools::shared(workdir);
    let rt = spawn(session_dir, workdir, settings, &host)?;
    let state = rt.get_state().map_err(|e| e.to_string())?;
    let (model, thinking) = cedian_agent::state::model_from_state(&state);
    println!("model: {}/{}", model.provider, model.id);
    println!("thinking: {:?}", thinking);
    println!("streaming: {}", state.is_streaming);
    println!("settled: {}", state.is_settled);
    println!("messages: {}", state.message_count);
    rt.shutdown().map_err(|e| e.to_string())?;
    Ok(())
}

/// Workflow commands (S2 headless surface):
/// ```text
/// cedian workflow run <kind> <title> [--risk low|medium|high]  # start (overwrites)
/// cedian workflow status                                       # §51 render + gates
/// cedian workflow evidence <gate> <summary> [--fail|--inconclusive]  # always unattributed
/// cedian workflow advance [--fail]                             # pass/fail current phase
/// cedian workflow resume                                       # unblock (ADR-0036)
/// cedian workflow complete                                     # §55 completion gate
/// ```
fn cmd_workflow(
    workdir: &Path,
    settings: &cedian_shell::Settings,
    args: &[String],
) -> Result<(), String> {
    let state_dir = state::dir(workdir)?;
    match args.first().map(|s| s.as_str()) {
        Some("run") => {
            let kind = args
                .get(1)
                .ok_or("usage: cedian workflow run <kind> <title> [--risk R]")?;
            let title = args
                .get(2)
                .ok_or("usage: cedian workflow run <kind> <title> [--risk R]")?;
            let kind = match kind.as_str() {
                "investigation" => cedian_workflow::TaskKind::Investigation,
                "bug_fix" => cedian_workflow::TaskKind::BugFix,
                "feature" => cedian_workflow::TaskKind::Feature,
                "refactor" => cedian_workflow::TaskKind::Refactor,
                "performance" => cedian_workflow::TaskKind::Performance,
                "prototype" => cedian_workflow::TaskKind::Prototype,
                _ => {
                    return Err(format!(
                        "unknown kind {kind:?} (investigation|bug_fix|feature|refactor|performance|prototype)"
                    ));
                }
            };
            let mut profile = cedian_workflow::TaskProfile::new(title, kind);
            let mut i = 3;
            while i < args.len() {
                match args[i].as_str() {
                    "--risk" => {
                        let r = args.get(i + 1).ok_or("usage: --risk low|medium|high")?;
                        profile.risk = match r.as_str() {
                            "low" => cedian_workflow::Risk::Low,
                            "medium" => cedian_workflow::Risk::Medium,
                            "high" => cedian_workflow::Risk::High,
                            _ => return Err(format!("unknown risk {r:?}")),
                        };
                        i += 2;
                    }
                    flag => return Err(format!("unknown flag {flag:?}")),
                }
            }
            let state = cedian_workflow::WorkflowState::start_with_floor(profile, &settings.floor)
                .map_err(|e| e.to_string())?;
            workflow_store::save(&state_dir, &state)?;
            render_workflow(&state, &current_state(workdir));
            Ok(())
        }
        Some("status") => {
            let state = workflow_store::load(&state_dir)?;
            render_workflow(&state, &current_state(workdir));
            Ok(())
        }
        Some("evidence") => {
            const USAGE: &str =
                "usage: cedian workflow evidence <gate> <summary> [--fail|--inconclusive]";
            let gate = args.get(1).ok_or(USAGE)?;
            let summary = args.get(2).ok_or(USAGE)?;
            let mut outcome = cedian_workflow::Outcome::Pass;
            for flag in &args[3..] {
                match flag.as_str() {
                    "--fail" => outcome = cedian_workflow::Outcome::Fail,
                    "--inconclusive" => outcome = cedian_workflow::Outcome::Inconclusive,
                    _ => return Err(format!("unknown flag {flag:?}")),
                }
            }
            let mut state = workflow_store::load(&state_dir)?;
            // S2 exit: typed evidence has no router-log call behind it, so
            // it is unattributed and can never pass a required gate.
            let id = format!("e{}", state.evidence.len() + 1);
            let current = current_state(workdir);
            let item = cedian_workflow::Evidence::unattributed(
                &id,
                cedian_workflow::EvidenceKind::File,
                &[gate],
                summary,
                outcome,
            )
            .with_code_state(current.bind(&[]));
            state.attach(item).map_err(|e| e.to_string())?;
            workflow_store::save(&state_dir, &state)?;
            match state.gate_result(gate, &current) {
                Ok(r) => println!(
                    "evidence {id} → gate {gate:?}: {:?} ({})",
                    r.status, r.reason
                ),
                Err(e) => println!("evidence {id} attached ({}).", e),
            }
            Ok(())
        }
        Some("advance") => {
            let passed = !args[1..].contains(&"--fail".to_string());
            let mut state = workflow_store::load(&state_dir)?;
            state
                .advance(passed, &current_state(workdir))
                .map_err(|e| e.to_string())?;
            if !passed {
                println!("phase failed — workflow failed");
            }
            workflow_store::save(&state_dir, &state)?;
            render_workflow(&state, &current_state(workdir));
            Ok(())
        }
        Some("resume") => {
            let mut state = workflow_store::load(&state_dir)?;
            state.resume()?;
            workflow_store::save(&state_dir, &state)?;
            render_workflow(&state, &current_state(workdir));
            Ok(())
        }
        Some("complete") => {
            let mut state = workflow_store::load(&state_dir)?;
            match state.complete(&current_state(workdir)) {
                Ok(()) => {
                    workflow_store::save(&state_dir, &state)?;
                    println!("complete");
                    Ok(())
                }
                Err(missing) => {
                    workflow_store::save(&state_dir, &state)?;
                    Err(format!("blocked:\n  - {}", missing.join("\n  - ")))
                }
            }
        }
        _ => Err(
            "usage: cedian workflow <run|status|evidence|advance|resume|complete> …".to_string(),
        ),
    }
}

/// Read `--base <branch>` from `args[from..]`; defaults to `HEAD`.
fn worker_base(args: &[String], from: usize) -> Result<String, String> {
    let mut base = "HEAD".to_string();
    let mut i = from;
    while i < args.len() {
        match args[i].as_str() {
            "--base" => {
                base = args
                    .get(i + 1)
                    .cloned()
                    .ok_or("usage: cedian worker … [--base <branch>]")?;
                i += 2;
            }
            flag => return Err(format!("unknown flag {flag:?}")),
        }
    }
    Ok(base)
}

/// Worker commands (S5 headless surface — one-shot per invocation; `list`
/// and `steer` were deleted at S9 U8, ADR-0050: the app's subagent rows
/// and `steer_subagent` replace them):
/// ```text
/// cedian worker spawn <id> --brief <file> [--kind <k>] [--base <branch>]
/// cedian worker preview <id> [--base <branch>]
/// cedian worker merge-back <id> [--base <branch>]
/// cedian worker remove <id>
/// ```
///
/// `CEDIAN_WORKDIR` must be the repo root: the registry lives at
/// `workers.json` in its state dir (ADR-0044), worktrees at `<repo>/.worktrees/<id>`.
/// Base defaults to `HEAD` unless `--base <branch>` is given. `spawn` takes
/// the ADR-0033 brief from `--brief <file>` (the same validator the
/// `cedian_worktree_request` host tool uses); without a whole brief it
/// refuses and creates nothing. `kind` defaults to `worker`.
fn cmd_worker(workdir: &Path, args: &[String]) -> Result<(), String> {
    let state = state::dir(workdir)?;
    match args.first().map(|s| s.as_str()) {
        Some("spawn") => {
            let usage = "usage: cedian worker spawn <id> --brief <file> [--kind K] [--base B]";
            let id = args.get(1).ok_or(usage)?;
            let mut brief_file: Option<&String> = None;
            let mut kind = "worker".to_string();
            let mut base = "HEAD".to_string();
            let mut i = 2;
            while i < args.len() {
                let value = || args.get(i + 1).ok_or(usage);
                match args[i].as_str() {
                    "--brief" => brief_file = Some(value()?),
                    "--kind" => kind = value()?.clone(),
                    "--base" => base = value()?.clone(),
                    flag => return Err(format!("unknown flag {flag:?}")),
                }
                i += 2;
            }
            let brief_file = brief_file.ok_or(usage)?;
            // Checked before the lock or any tree: a refused brief creates
            // nothing (ADR-0033 decision 1).
            let brief = cedian_worker::Brief::from_file(Path::new(brief_file))
                .map_err(|e| e.to_string())?;
            let _lock = cedian_worker::Registry::lock(&state).map_err(|e| e.to_string())?;
            let (mut reg, _) = cedian_worker::Registry::open(&state).map_err(|e| e.to_string())?;
            if reg.get(id).is_some() {
                return Err(cedian_worker::WorkerError::Exists(id.clone()).to_string());
            }
            cedian_worker::validate_id(id).map_err(|e| e.to_string())?;
            let brief_path = brief.store(&state, id, 1)?;
            let mut head = match cedian_worker::spawn(workdir, id, &base) {
                Ok(head) => head,
                Err(e) => {
                    let _ = std::fs::remove_file(&brief_path);
                    return Err(e.to_string());
                }
            };
            head.status = cedian_worker::WorkerStatus::Running;
            head.task_title = brief.goal.clone();
            head.kind = kind;
            head.timebox_min = brief.timebox_min;
            let (worktree, branch) = (head.worktree.clone(), head.branch.clone());
            reg.insert(head).map_err(|e| e.to_string())?;
            reg.save(&state).map_err(|e| e.to_string())?;
            println!("worker {id} → {worktree} (branch {branch})");
            Ok(())
        }
        Some("preview") => {
            let id = args
                .get(1)
                .ok_or("usage: cedian worker preview <id> [--base <branch>]")?;
            let base = worker_base(args, 2)?;
            let (reg, _) = cedian_worker::Registry::open(&state).map_err(|e| e.to_string())?;
            let head = reg
                .get(id)
                .cloned()
                .ok_or_else(|| cedian_worker::WorkerError::NoSuch(id.clone()).to_string())?;
            let plan =
                cedian_worker::merge_preview(workdir, &head, &base).map_err(|e| e.to_string())?;
            println!("clean:");
            for f in &plan.clean {
                println!("  {f}");
            }
            println!("conflicted(STALE):");
            for f in &plan.conflicted {
                println!("  {f}");
            }
            Ok(())
        }
        Some("merge-back") => {
            let id = args
                .get(1)
                .ok_or("usage: cedian worker merge-back <id> [--base <branch>]")?;
            let base = worker_base(args, 2)?;
            let _lock = cedian_worker::Registry::lock(&state).map_err(|e| e.to_string())?;
            let (mut reg, _) = cedian_worker::Registry::open(&state).map_err(|e| e.to_string())?;
            let head = reg
                .get(id)
                .cloned()
                .ok_or_else(|| cedian_worker::WorkerError::NoSuch(id.clone()).to_string())?;
            match cedian_worker::merge_back(workdir, &head, &base) {
                Ok(()) => {
                    reg.set_status(id, cedian_worker::WorkerStatus::Done, String::new())
                        .map_err(|e| e.to_string())?;
                    reg.save(&state).map_err(|e| e.to_string())?;
                    println!("merged {} → {base}", head.branch);
                    Ok(())
                }
                Err(cedian_worker::WorkerError::Conflicted(files)) => {
                    println!("refused (STALE):");
                    for f in &files {
                        println!("  {f}");
                    }
                    Err(format!("refused (STALE): {}", files.join(", ")))
                }
                Err(e) => Err(e.to_string()),
            }
        }
        Some("remove") => {
            let id = args.get(1).ok_or("usage: cedian worker remove <id>")?;
            if args.len() > 2 {
                return Err("usage: cedian worker remove <id>".to_string());
            }
            let _lock = cedian_worker::Registry::lock(&state).map_err(|e| e.to_string())?;
            let (mut reg, _) = cedian_worker::Registry::open(&state).map_err(|e| e.to_string())?;
            let head = reg
                .get(id)
                .cloned()
                .ok_or_else(|| cedian_worker::WorkerError::NoSuch(id.clone()).to_string())?;
            cedian_worker::remove(workdir, &head).map_err(|e| e.to_string())?;
            reg.remove(id);
            reg.save(&state).map_err(|e| e.to_string())?;
            println!("removed {id}");
            Ok(())
        }
        _ => Err("usage: cedian worker <spawn|preview|merge-back|remove> …".into()),
    }
}

/// §51 render: title, kind · risk, phase checklist, gate states.
fn render_workflow(
    state: &cedian_workflow::WorkflowState,
    current: &cedian_workflow::CurrentState,
) {
    let kind = match state.task.kind {
        cedian_workflow::TaskKind::Investigation => "Investigation",
        cedian_workflow::TaskKind::BugFix => "Bug Fix",
        cedian_workflow::TaskKind::Feature => "Feature",
        cedian_workflow::TaskKind::Refactor => "Refactor",
        cedian_workflow::TaskKind::Performance => "Performance",
        cedian_workflow::TaskKind::Prototype => "Prototype",
    };
    let risk = match state.task.risk {
        cedian_workflow::Risk::Low => "Low",
        cedian_workflow::Risk::Medium => "Medium",
        cedian_workflow::Risk::High => "High",
    };
    println!("{}", state.task.title);
    println!("{kind} · {risk} risk · {:?}", state.status);
    println!();
    for ps in &state.phases {
        let glyph = match ps.status {
            cedian_workflow::PhaseStatus::Pending => "○",
            cedian_workflow::PhaseStatus::Running => "●",
            cedian_workflow::PhaseStatus::Passed => "✓",
            cedian_workflow::PhaseStatus::Failed => "✗",
            cedian_workflow::PhaseStatus::Blocked => "!",
            cedian_workflow::PhaseStatus::Skipped => "–",
        };
        if ps.status == cedian_workflow::PhaseStatus::Skipped {
            let reason = ps.skip_reason.as_deref().unwrap_or("conditional");
            println!("{glyph} {} (skipped: {reason})", ps.id);
            continue;
        }
        println!("{glyph} {}", ps.id);
    }
    println!();
    for (id, r) in state.all_gates(current) {
        let glyph = match r.status {
            cedian_workflow::GateStatus::Pending => "○",
            cedian_workflow::GateStatus::Passed => "✓",
            cedian_workflow::GateStatus::Failed => "✗",
            cedian_workflow::GateStatus::Blocked => "!",
            cedian_workflow::GateStatus::Skipped => "–",
        };
        let unverified = if r.unverified_origin {
            " [unverified-origin]"
        } else {
            ""
        };
        println!(
            "{glyph} gate {id}: {:?}{unverified} — {}",
            r.status, r.reason
        );
    }
    // Completion view (§55, ADR-0024): the last claim and every claim in it.
    if let Some(last) = &state.last_completion {
        println!();
        if last.accepted {
            println!("last completion: accepted");
        } else {
            println!("last completion: REFUSED");
            for m in &last.missing {
                println!("  - {m}");
            }
        }
        println!(
            "{}",
            cedian_workflow::ledger_lines(&last.claims).trim_start()
        );
    }
}

/// Card status glyph for the turn render.
trait CardGlyph {
    fn status_glyph(&self) -> &'static str;
}

impl CardGlyph for cedian_agent_ui::ToolCard {
    fn status_glyph(&self) -> &'static str {
        match self.status {
            cedian_agent_ui::ToolCardStatus::Running => "…",
            cedian_agent_ui::ToolCardStatus::Done => "✓",
            cedian_agent_ui::ToolCardStatus::Error => "✗",
            cedian_agent_ui::ToolCardStatus::Interrupted => "!",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_goes_stale_on_lockfile_and_large_file_changes() {
        let dir = crate::test_dir::TestDir::new("code-state");
        std::fs::write(dir.join("Cargo.lock"), "serde 1.0.0").unwrap();
        let big = vec![b'x'; workspace_files::MAX_FILE_BYTES as usize + 1];
        std::fs::write(dir.join("fixture.dat"), &big).unwrap();

        let bound = current_state(&dir).bind(&[]);
        std::fs::write(dir.join("Cargo.lock"), "serde 1.0.1").unwrap();
        assert!(
            current_state(&dir).stale_reason(&bound).is_some(),
            "a lockfile bump makes tree-bound evidence stale"
        );

        let bound = current_state(&dir).bind(&[]);
        let mut bigger = big;
        bigger.push(b'y');
        std::fs::write(dir.join("fixture.dat"), &bigger).unwrap();
        assert!(
            current_state(&dir).stale_reason(&bound).is_some(),
            "a change to a file over the buffer size cap makes tree-bound evidence stale"
        );
    }

    #[test]
    fn every_reporting_host_tool_is_a_channel_call() {
        let mut settings = cedian_shell::Settings::default();
        let names = host_tool_names(&settings);
        assert!(names.contains(&cedian_worker::WORKTREE_REQUEST_TOOL));
        for name in &names {
            assert!(cedian_workflow::is_channel_call(name, ""), "{name}");
        }
        for verdict in [cedian_shell::Verdict::Ask, cedian_shell::Verdict::Deny] {
            settings.permissions.project_write = verdict;
            assert!(
                !host_tool_names(&settings).contains(&cedian_worker::WORKTREE_REQUEST_TOOL),
                "a worktree request makes a tree with no dialog: only under allow (ADR-0012)"
            );
        }
    }

    #[test]
    fn refused_claim_blocks_at_turn_end_and_resume_unblocks() {
        use cedian_workflow::{CompletionAttempt, TaskKind, TaskProfile, WorkflowStatus};
        let d = crate::test_dir::TestDir::new("u5");
        end_workflow_turn(&d).unwrap(); // fast lane: no workflow, nothing to do
        let sd = state::dir(&d).unwrap();
        assert!(!workflow_store::exists(&sd));
        let mut state =
            cedian_workflow::WorkflowState::start(TaskProfile::new("t", TaskKind::BugFix)).unwrap();
        state.last_completion = Some(CompletionAttempt {
            claims: vec![],
            accepted: false,
            missing: vec!["required gate \"verify\"".into()],
            turn_ended: false,
        });
        workflow_store::save(&sd, &state).unwrap();
        end_workflow_turn(&d).unwrap();
        assert_eq!(
            workflow_store::load(&sd).unwrap().status,
            WorkflowStatus::Blocked
        );
        cmd_workflow(
            &d,
            &cedian_shell::Settings::default(),
            &["resume".to_string()],
        )
        .unwrap();
        assert_eq!(
            workflow_store::load(&sd).unwrap().status,
            WorkflowStatus::Running
        );
        assert!(
            cmd_workflow(
                &d,
                &cedian_shell::Settings::default(),
                &["resume".to_string()]
            )
            .is_err()
        );
    }

    /// The verify-notes profile skill (test fixture copy) only uses keys and
    /// enum values the channel accepts, names every stage, and has a feature
    /// map cedian can read (ADR-0025).
    #[test]
    fn verify_notes_skill_matches_the_channel_schema() {
        let skill = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/skills/verify-notes/SKILL.md"
        ))
        .unwrap();
        assert!(skill.contains("name: verify-notes"));
        assert_eq!(cedian_workflow::feature_map(&skill), ["add-note"]);
        let update = cedian_workflow::update_parameters();
        let props = update["properties"].as_object().unwrap();
        let mut stages = Vec::new();
        for block in skill.split("```json").skip(1) {
            let body = block.split("```").next().unwrap();
            let obj: serde_json::Map<String, serde_json::Value> =
                serde_json::from_str(body.trim()).unwrap_or_else(|e| panic!("{body}: {e}"));
            for (key, value) in &obj {
                let prop = props
                    .get(key)
                    .unwrap_or_else(|| panic!("unknown key {key}"));
                if let (Some(allowed), Some(v)) = (prop.get("enum"), value.as_str()) {
                    assert!(
                        v.starts_with('<') || allowed.as_array().unwrap().iter().any(|a| a == v),
                        "{key}={v}"
                    );
                }
            }
            if let Some(stage) = obj.get("stage").and_then(|v| v.as_str()) {
                stages.push(stage.to_string());
            }
        }
        assert_eq!(stages, ["launch", "doctor", "drive", "evidence", "cleanup"]);
    }

    /// U8: the bug-fix playbook skill (a test fixture copy — cedian never
    /// writes the user's `.omp/`, §77) only uses tool names, ops, keys and
    /// enum values the channel actually accepts, and the gates it reports
    /// are the bug_fix playbook's.
    #[test]
    fn bug_fix_skill_matches_the_channel_schema() {
        use serde_json::Value;
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/skills/bug-fix/SKILL.md"
        );
        let skill = std::fs::read_to_string(path).unwrap();
        let front = skill.split("---").nth(1).expect("frontmatter");
        assert!(front.contains("name: bug-fix"), "{front}");
        assert!(front.contains("description: "), "{front}");

        for word in skill.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
            if word.starts_with("cedian_") {
                assert!(
                    cedian_workflow::CHANNEL_TOOLS.contains(&word),
                    "unknown tool {word}"
                );
            }
        }
        let check = |obj: &serde_json::Map<String, Value>,
                     schema: &serde_json::Map<String, Value>,
                     ctx: &str| {
            let props = schema["properties"].as_object().unwrap();
            for (key, value) in obj {
                let prop = props
                    .get(key)
                    .unwrap_or_else(|| panic!("{ctx}: unknown key {key:?}"));
                if let (Some(allowed), Some(v)) = (prop.get("enum"), value.as_str()) {
                    if !v.starts_with('<') {
                        assert!(
                            allowed.as_array().unwrap().iter().any(|a| a == v),
                            "{ctx}: {key}={v:?} not in {allowed}"
                        );
                    }
                }
            }
        };
        let update = cedian_workflow::update_parameters();
        let complete = cedian_workflow::complete_parameters();
        let bug_fix = cedian_workflow::Playbook::bug_fix();
        let mut ops = Vec::new();
        for block in skill.split("```json").skip(1) {
            let body = block.split("```").next().unwrap();
            let obj: serde_json::Map<String, Value> = serde_json::from_str(body.trim())
                .unwrap_or_else(|e| panic!("bad JSON {body}: {e}"));
            if let Some(op) = obj.get("op").and_then(Value::as_str) {
                check(&obj, &update, op);
                ops.push(op.to_string());
                if let Some(gate) = obj.get("gate").and_then(Value::as_str) {
                    assert!(bug_fix.gates.iter().any(|g| g.id == gate), "no gate {gate}");
                }
            } else {
                check(&obj, &complete, "cedian_complete");
                let claim_schema = complete["properties"]["claims"]["items"]
                    .as_object()
                    .unwrap();
                for claim in obj["claims"].as_array().unwrap() {
                    check(claim.as_object().unwrap(), claim_schema, "claim");
                }
            }
        }
        for op in ["start", "evidence", "advance"] {
            assert!(ops.iter().any(|o| o == op), "skill never shows op {op}");
        }
    }
}
