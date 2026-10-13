//! Host-tool channel (P5, ADR-0022): OMP drives the workflow through
//! `cedian_workflow_update` and claims completion through `cedian_complete`.
//!
//! The engine stays pure: this module only adapts JSON args to
//! `WorkflowState` calls. Two seams keep it free of the runtime crates:
//! - [`WorkflowStore`]: every call loads, mutates and saves, so the store on
//!   disk stays the one truth even when CLI verbs run between turns.
//! - `current: Fn() -> CurrentState`: the workspace hashed now (ADR-0024).
//!   Evidence binds to the files its call named, else the whole tree; gates
//!   compare against a fresh `CurrentState` on every evaluation.
//! - the resolver `Fn(tool, needle) -> Option<BoundCall>`: the agent names
//!   the tool that produced the evidence (`from_tool`, optional `match` on
//!   its args); the caller binds that to the most recent call the router log
//!   saw finish successfully, excluding [`is_channel_call`]s (ADR-0031: the
//!   model never sees `tool_call_id`s). The id always comes from the log, so
//!   the agent cannot attribute evidence by saying so.

use crate::{
    Claim, CompletionAttempt, CurrentState, Evidence, EvidenceKind, FeatureRef, GateFloor,
    GateKind, GateSpec, GateStatus, Outcome, ProfileLedger, Risk, Stage, TaskKind, TaskProfile,
    WorkflowState, WorkflowStatus, feature_map, state::ContinueOutcome,
};
use omp_rpc::HostTool;
use serde_json::{Map, Value, json};
use std::sync::{Arc, Mutex};

/// Serializes load → mutate → save of `workflow.json` in this process:
/// the channel's handler threads and the host's own changes (an
/// escalation, Resume, the turn boundary) take it, so none overwrites
/// another's update.
static STORE_LOCK: Mutex<()> = Mutex::new(());

/// Hold while loading, changing and saving a workflow outside the channel.
pub fn store_lock() -> std::sync::MutexGuard<'static, ()> {
    STORE_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

/// Host tool: start / evidence / advance.
pub const WORKFLOW_UPDATE_TOOL: &str = "cedian_workflow_update";
/// Host tool: `can_complete` check at the completion boundary.
pub const COMPLETE_TOOL: &str = "cedian_complete";
/// Every tool that reports TO cedian. Citing one as evidence would be
/// self-certification, so verifiers must reject them.
pub const CHANNEL_TOOLS: &[&str] = &[
    WORKFLOW_UPDATE_TOOL,
    COMPLETE_TOOL,
    "cedian_worktree_request",
    "cedian_review_request",
    "cedian_review_finding",
    "cedian_correction_class",
];

/// True when a logged call is a channel report: called by name, or through
/// OMP's `xd://<tool>` device form (`read`/`write` with that preview).
pub fn is_channel_call(tool_name: &str, args_preview: &str) -> bool {
    CHANNEL_TOOLS
        .iter()
        .any(|t| tool_name == *t || args_preview.contains(&format!("xd://{t}")))
}

/// OMP tools that only observe. Any other finished call may have changed
/// files, so evidence bound to an earlier call is stale on arrival.
pub const READ_ONLY_TOOLS: &[&str] = &[
    "read", "grep", "glob", "find", "ast_grep", "lsp", "ask", "todo",
];

/// True when a logged call may have changed workspace files (conservative:
/// unknown tools, `bash` and `eval` count). Channel reports never do.
pub fn may_mutate(tool_name: &str, args_preview: &str) -> bool {
    !READ_ONLY_TOOLS.contains(&tool_name) && !is_channel_call(tool_name, args_preview)
}

/// Where the workflow lives between calls (`workflow.json` in the CLI state dir).
pub trait WorkflowStore: Send + Sync {
    /// `Ok(None)` when no workflow was ever started.
    fn load(&self) -> Result<Option<WorkflowState>, String>;
    fn save(&self, state: &WorkflowState) -> Result<(), String>;
}

/// A logged call evidence was bound to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundCall {
    pub tool_call_id: String,
    pub tool_name: String,
    pub args_preview: String,
    /// A call that may have changed files finished after this one (its name
    /// and args): evidence reported now would describe code the call never
    /// saw, so it is stored stale.
    pub mutated_after: Option<String>,
}

/// Verification profiles (ADR-0025): the ledger cedian keeps
/// (`verify.json` in the CLI state dir) and read-only access to the profile skills
/// (`.omp/skills/<profile>/SKILL.md`, never written — §77).
pub trait ProfileStore: Send + Sync {
    fn load(&self) -> Result<ProfileLedger, String>;
    fn save(&self, ledger: &ProfileLedger) -> Result<(), String>;
    /// The skill text, `None` when the project has no such skill.
    fn skill(&self, profile: &str) -> Option<String>;
}

/// No profiles: every profile op says so.
pub struct NoProfiles;

impl ProfileStore for NoProfiles {
    fn load(&self) -> Result<ProfileLedger, String> {
        Ok(ProfileLedger::default())
    }
    fn save(&self, _: &ProfileLedger) -> Result<(), String> {
        Err("no verification profile store".to_string())
    }
    fn skill(&self, _: &str) -> Option<String> {
        None
    }
}

type Resolver = dyn Fn(&str, &str) -> Option<BoundCall> + Send + Sync;
type Current = dyn Fn() -> CurrentState + Send + Sync;

/// The two workflow host tools over one store + one resolver.
pub struct WorkflowChannel {
    task_id: String,
    store: Box<dyn WorkflowStore>,
    resolve: Box<Resolver>,
    current: Box<Current>,
    floor: GateFloor,
    profiles: Box<dyn ProfileStore>,
    /// Open review blockers (S3), one line each; any refuses completion.
    blockers: Mutex<Box<Blockers>>,
    /// Optional `worker` on `cedian_complete` (ADR-0033 decision 4): the
    /// panel marks that worker Done when its verify gates pass.
    worker_complete: Mutex<Option<Box<WorkerComplete>>>,
    /// Optional progress touch when evidence is stored for a gate
    /// (ADR-0033 decision 5): the panel advances matching workers.
    worker_progress: Mutex<Option<Box<WorkerProgress>>>,
}

type Blockers = dyn Fn() -> Vec<String> + Send + Sync;
type WorkerComplete =
    dyn Fn(&str, &[crate::Evidence], &CurrentState) -> Result<(), String> + Send + Sync;
type WorkerProgress = dyn Fn(&str) + Send + Sync;

impl WorkflowChannel {
    pub fn new(
        task_id: impl Into<String>,
        store: Box<dyn WorkflowStore>,
        resolve: impl Fn(&str, &str) -> Option<BoundCall> + Send + Sync + 'static,
        current: impl Fn() -> CurrentState + Send + Sync + 'static,
    ) -> Arc<Self> {
        Self::with_policy(
            task_id,
            store,
            resolve,
            current,
            GateFloor::default(),
            Box::new(NoProfiles),
        )
    }

    /// Same, with the cedian gate floor every started workflow gets and the
    /// project's verification profiles.
    pub fn with_policy(
        task_id: impl Into<String>,
        store: Box<dyn WorkflowStore>,
        resolve: impl Fn(&str, &str) -> Option<BoundCall> + Send + Sync + 'static,
        current: impl Fn() -> CurrentState + Send + Sync + 'static,
        floor: GateFloor,
        profiles: Box<dyn ProfileStore>,
    ) -> Arc<Self> {
        Arc::new(Self {
            task_id: task_id.into(),
            store,
            resolve: Box::new(resolve),
            current: Box::new(current),
            floor,
            profiles,
            blockers: Mutex::new(Box::new(Vec::new)),
            worker_complete: Mutex::new(None),
            worker_progress: Mutex::new(None),
        })
    }

    /// Where open review blockers come from (the CLI's findings store).
    pub fn set_blockers(&self, blockers: impl Fn() -> Vec<String> + Send + Sync + 'static) {
        *self.blockers.lock().unwrap_or_else(|e| e.into_inner()) = Box::new(blockers);
    }

    /// Where a `cedian_complete { worker }` goes (the panel's worker trees).
    pub fn set_worker_complete(
        &self,
        complete: impl Fn(&str, &[crate::Evidence], &CurrentState) -> Result<(), String>
        + Send
        + Sync
        + 'static,
    ) {
        *self
            .worker_complete
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(Box::new(complete));
    }

    /// Where evidence for a gate advances worker `last_progress`.
    pub fn set_worker_progress(&self, progress: impl Fn(&str) + Send + Sync + 'static) {
        *self
            .worker_progress
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = Some(Box::new(progress));
    }

    /// `cedian_workflow_update`: `op` is `start`, `evidence` or `advance`.
    pub fn update(&self, args: &Map<String, Value>) -> Result<String, String> {
        let _guard = store_lock();
        let op = str_arg(args, "op")?;
        match op {
            "start" => {
                if let Some(cur) = self.store.load()? {
                    if matches!(
                        cur.status,
                        WorkflowStatus::Running | WorkflowStatus::Blocked
                    ) {
                        return Err(format!(
                            "a workflow is already active: {:?} ({:?}); finish it with {COMPLETE_TOOL}",
                            cur.task.title, cur.status
                        ));
                    }
                }
                let kind: TaskKind = enum_arg(args, "kind")?.ok_or(
                    "missing `kind` (investigation|bug_fix|feature|refactor|performance|prototype)",
                )?;
                let mut profile = TaskProfile::new(str_arg(args, "title")?, kind);
                if let Some(risk) = enum_arg::<Risk>(args, "risk")? {
                    profile.risk = risk;
                }
                let state = WorkflowState::start_with_floor(profile, &self.floor)
                    .map_err(|e| e.to_string())?;
                self.store.save(&state)?;
                Ok(format!(
                    "workflow started\n{}",
                    summary(&state, &(self.current)())
                ))
            }
            "evidence" => {
                let mut state = self.active()?;
                let gate = str_arg(args, "gate")?;
                let kind = enum_arg::<EvidenceKind>(args, "kind")?.unwrap_or(EvidenceKind::Command);
                let outcome = match (enum_arg::<Outcome>(args, "outcome")?, args.get("ok")) {
                    (Some(outcome), _) => outcome,
                    (None, Some(Value::Bool(ok))) => Outcome::from_ok(*ok),
                    _ => return Err("missing `outcome` (pass|fail|inconclusive)".to_string()),
                };
                let text = str_arg(args, "summary")?;
                let id = format!("e{}", state.evidence.len() + 1);
                let text_arg = |key: &str| args.get(key).and_then(Value::as_str).unwrap_or("");
                let (from_tool, needle) = (text_arg("from_tool"), text_arg("match"));
                let bound = match from_tool {
                    "" => None,
                    tool => (self.resolve)(tool, needle),
                };
                let current = (self.current)();
                let (item, origin) = match &bound {
                    Some(call) => {
                        // ADR-0031 consequence: the bound tool decides what
                        // the evidence can show (a `read` is never a test).
                        let kind = kind_for(&call.tool_name, kind);
                        let mut item = Evidence::attributed(
                            &id,
                            kind,
                            &[gate],
                            format!("{text} [{} {}]", call.tool_name, call.args_preview),
                            outcome,
                            &self.task_id,
                            &call.tool_call_id,
                        )
                        .with_code_state(current.bind(&named_paths(&call.args_preview, &current)));
                        let mut origin = format!(
                            "attributed to {} call `{}`",
                            call.tool_name, call.args_preview
                        );
                        if let Some(later) = &call.mutated_after {
                            let reason = format!("{later} ran after it, before this report");
                            origin.push_str(&format!("; STALE: {reason} (re-run it)"));
                            item.born_stale = Some(reason);
                        }
                        (item, origin)
                    }
                    None => (
                        Evidence::unattributed(&id, kind, &[gate], text, outcome)
                            .with_code_state(current.bind(&[])),
                        if from_tool.is_empty() {
                            "UNATTRIBUTED: no from_tool given".to_string()
                        } else {
                            format!(
                                "UNATTRIBUTED: no finished, successful {from_tool:?} call{} in this session",
                                if needle.is_empty() {
                                    String::new()
                                } else {
                                    format!(" matching {needle:?}")
                                }
                            )
                        },
                    ),
                };
                let mut item = item;
                let mut origin = origin;
                if let Some(feature) = self.feature_arg(args)? {
                    // ADR-0025: a draft profile or an unhealthy instance
                    // makes the observation inconclusive, whatever it says.
                    let instance = text_arg("instance");
                    let skill = self.profiles.skill(&feature.profile).unwrap_or_default();
                    let check = if instance.is_empty() {
                        Err("no `instance` given".to_string())
                    } else {
                        self.profiles
                            .load()?
                            .check(&feature.profile, &skill, instance)
                    };
                    if let Err(why) = check {
                        item.outcome = Outcome::Inconclusive;
                        item.summary.push_str(&format!(" [inconclusive: {why}]"));
                        origin.push_str(&format!("; INCONCLUSIVE: {why}"));
                    }
                    item.feature = Some(feature);
                }
                if let Some(m) = args.get("measurement") {
                    item.measurement = Some(
                        serde_json::from_value(m.clone())
                            .map_err(|_| format!("bad `measurement`: {m}"))?,
                    );
                }
                state.attach(item).map_err(|e| e.to_string())?;
                self.store.save(&state)?;
                if let Some(progress) = self
                    .worker_progress
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .as_ref()
                {
                    progress(gate);
                }
                let gate_line = match state.gate_result(gate, &current) {
                    Ok(r) => format!("gate {gate}: {:?} — {}", r.status, r.reason),
                    Err(e) => e.to_string(),
                };
                Ok(format!("evidence {id} {origin}\n{gate_line}"))
            }
            "advance" => {
                let mut state = self.active()?;
                let passed = args
                    .get("passed")
                    .and_then(Value::as_bool)
                    .ok_or("missing `passed`")?;
                let current = (self.current)();
                state.advance(passed, &current).map_err(|e| e.to_string())?;
                self.store.save(&state)?;
                Ok(summary(&state, &current))
            }
            "gate" => {
                let mut state = self.active()?;
                let id = str_arg(args, "gate")?;
                let kind: GateKind = enum_arg(args, "gate_kind")?.ok_or(
                    "missing `gate_kind` (build|test|lint|behavior|visual|performance|review)",
                )?;
                let min_items = args.get("min_items").and_then(Value::as_u64).unwrap_or(1);
                let gate = GateSpec {
                    id: id.to_string(),
                    gate_kind: kind,
                    evidence_kinds: enum_arg(args, "evidence_kinds")?.unwrap_or_default(),
                    min_items: usize::try_from(min_items).unwrap_or(1),
                    feature: self.feature_arg(args)?,
                }
                .build()
                .map_err(|e| format!("{e:?}"))?;
                state.add_gate(gate)?;
                self.store.save(&state)?;
                Ok(format!(
                    "gate {id} added (required)\n{}",
                    summary(&state, &(self.current)())
                ))
            }
            "profile" => {
                let profile = str_arg(args, "profile")?;
                let instance = str_arg(args, "instance")?;
                let stage: Stage = enum_arg(args, "stage")?
                    .ok_or("missing `stage` (launch|doctor|drive|evidence|cleanup)")?;
                let ok = args
                    .get("ok")
                    .and_then(Value::as_bool)
                    .ok_or("missing `ok`")?;
                let surprising = args
                    .get("surprising")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                let skill = self.profiles.skill(profile).ok_or_else(|| {
                    format!("no verification profile skill {profile:?} in .omp/skills/")
                })?;
                let text_arg = |key: &str| args.get(key).and_then(Value::as_str).unwrap_or("");
                let bound = match text_arg("from_tool") {
                    "" => None,
                    tool => (self.resolve)(tool, text_arg("match")),
                };
                let Some(call) = bound else {
                    return Ok(format!(
                        "{profile} {stage:?} NOT recorded: no finished, successful matching call                          (from_tool + match name the tool call that ran this stage)"
                    ));
                };
                let mut ledger = self.profiles.load()?;
                let line = ledger.record(profile, &skill, instance, stage, ok, surprising);
                self.profiles.save(&ledger)?;
                Ok(format!("{line} [{} {}]", call.tool_name, call.args_preview))
            }
            other => Err(format!(
                "unknown op {other:?} (start|evidence|advance|gate|profile)"
            )),
        }
    }

    /// `cedian_complete {claims?}`. No workflow → nothing to check (fast
    /// lane, ADR-0026). The claims ledger is checked and stored with the
    /// result either way (ADR-0024); flagged claims never block, gates do.
    /// Unmet required gates → error with what is missing; each counts one
    /// continue, and at `MAX_CONTINUE` the workflow is `blocked`.
    pub fn complete(&self, args: &Map<String, Value>) -> Result<String, String> {
        let _guard = store_lock();
        if let Some(worker) = args
            .get("worker")
            .and_then(Value::as_str)
            .filter(|id| !id.is_empty())
        {
            let evidence = self
                .store
                .load()?
                .map(|state| state.evidence_list())
                .unwrap_or_default();
            let current = (self.current)();
            let lock = self
                .worker_complete
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            let Some(complete) = lock.as_ref() else {
                return Err("cedian_complete { worker } is not wired in this host".to_string());
            };
            complete(worker, &evidence, &current)?;
            return Ok(format!("worker {worker} complete"));
        }
        let Some(mut state) = self.store.load()? else {
            return Ok("no cedian workflow is active; nothing to check".to_string());
        };
        match state.status {
            WorkflowStatus::Complete => return Ok("workflow already complete".to_string()),
            WorkflowStatus::Blocked => {
                return Err(
                    "workflow is BLOCKED: stop and report the missing gates to the user"
                        .to_string(),
                );
            }
            _ => {}
        }
        let claims: Vec<Claim> = match args.get("claims") {
            None | Some(Value::Null) => Vec::new(),
            Some(v) => serde_json::from_value(v.clone()).map_err(|e| {
                format!("bad `claims` ({e}): each is {{text, label: measured|inferred|guess, evidence: [ids]}}")
            })?,
        };
        let current = (self.current)();
        let checked = state.check_claims(claims, &current);
        let ledger = ledger_lines(&checked);
        let blockers = (self.blockers.lock().unwrap_or_else(|e| e.into_inner()))();
        let missing = match state.can_complete(&current) {
            Ok(_) if blockers.is_empty() => {
                state.complete(&current).map_err(|m| m.join("; "))?;
                state.last_completion = Some(CompletionAttempt {
                    claims: checked,
                    accepted: true,
                    missing: Vec::new(),
                    turn_ended: false,
                });
                self.store.save(&state)?;
                return Ok(format!("complete\n{}{ledger}", summary(&state, &current)));
            }
            Ok(_) => blockers,
            Err(mut missing) => {
                missing.extend(blockers);
                missing
            }
        };
        state.last_completion = Some(CompletionAttempt {
            claims: checked,
            accepted: false,
            missing: missing.clone(),
            turn_ended: false,
        });
        let failing: Vec<String> = state
            .all_gates(&current)
            .into_iter()
            .filter(|(id, r)| {
                r.status != GateStatus::Passed
                    && state
                        .playbook
                        .gates
                        .iter()
                        .any(|g| &g.id == id && g.required)
            })
            .map(|(id, _)| id)
            .collect();
        let mut budget = Vec::new();
        for gate in &failing {
            match state
                .record_continue(gate, &[], &current)
                .map_err(|e| e.to_string())?
            {
                ContinueOutcome::Continue { attempts_left } => {
                    budget.push(format!("{gate}: {attempts_left} attempt(s) left"));
                }
                ContinueOutcome::Blocked { attempts, .. } => {
                    budget.push(format!("{gate}: BLOCKED after {attempts} attempts"));
                }
            }
        }
        self.store.save(&state)?;
        let next = if state.status == WorkflowStatus::Blocked {
            "workflow is now BLOCKED: stop and report the missing gates to the user"
        } else {
            "not complete: produce the missing evidence, report it, then call cedian_complete again"
        };
        Err(format!(
            "{next}\n  - {}\n{}{ledger}",
            missing.join("\n  - "),
            budget.join("\n")
        ))
    }

    /// `profile` + `feature` args → a feature-map entry that exists in that
    /// profile skill. Neither → `None`.
    fn feature_arg(&self, args: &Map<String, Value>) -> Result<Option<FeatureRef>, String> {
        let text_arg = |key: &str| args.get(key).and_then(Value::as_str).unwrap_or("");
        let (profile, id) = (text_arg("profile"), text_arg("feature"));
        if id.is_empty() {
            return Ok(None);
        }
        if profile.is_empty() {
            return Err("`feature` needs `profile` (the verify-<app> skill name)".to_string());
        }
        let skill = self
            .profiles
            .skill(profile)
            .ok_or_else(|| format!("no verification profile skill {profile:?} in .omp/skills/"))?;
        let features = feature_map(&skill);
        if !features.iter().any(|f| f == id) {
            return Err(format!(
                "{profile} has no feature {id:?} (feature map: {})",
                features.join(", ")
            ));
        }
        Ok(Some(FeatureRef {
            profile: profile.to_string(),
            id: id.to_string(),
        }))
    }

    /// The running workflow's gates that name `kind` among the evidence
    /// they take; empty without one (the fast lane, ADR-0026). A gate that
    /// takes any kind is not among them: an observation cedian makes on
    /// its own counts only where a gate asked for its kind.
    pub fn gates_taking(&self, kind: EvidenceKind) -> Result<Vec<String>, String> {
        let _guard = store_lock();
        let Some(state) = self.store.load()? else {
            return Ok(Vec::new());
        };
        if state.status != WorkflowStatus::Running {
            return Ok(Vec::new());
        }
        Ok(state
            .playbook
            .gates
            .iter()
            .filter(|g| g.predicate.kinds.contains(&kind))
            .map(|g| g.id.clone())
            .collect())
    }

    /// Store an observation cedian made of one of OMP's calls (a browser
    /// capture at the end of OMP's `browser` call, ADR-0055) under the
    /// next evidence id, bound to the workspace now unless it carries its
    /// own code state. `None` without a running workflow.
    pub fn observe(&self, mut item: Evidence) -> Result<Option<String>, String> {
        let _guard = store_lock();
        let Some(mut state) = self.store.load()? else {
            return Ok(None);
        };
        if state.status != WorkflowStatus::Running {
            return Ok(None);
        }
        item.id = format!("e{}", state.evidence.len() + 1);
        if item.code_state.is_none() {
            item.code_state = Some((self.current)().bind(&[]));
        }
        let id = item.id.clone();
        state.attach(item).map_err(|e| e.to_string())?;
        self.store.save(&state)?;
        Ok(Some(id))
    }

    /// The state gates are judged against now.
    pub fn current(&self) -> CurrentState {
        (self.current)()
    }

    /// Evidence cedian produced itself, such as a review it ran (ADR-0039):
    /// attributed to the host-tool call that asked for it, unattributed
    /// when a person ran it from the CLI. `None` without an active workflow.
    pub fn cedian_evidence(
        &self,
        gate: &str,
        outcome: Outcome,
        summary: &str,
        tool_call_id: Option<&str>,
    ) -> Result<Option<String>, String> {
        let _guard = store_lock();
        let Some(mut state) = self.store.load()? else {
            return Ok(None);
        };
        let id = format!("e{}", state.evidence.len() + 1);
        let current = (self.current)();
        let item = match tool_call_id {
            Some(call) => Evidence::attributed(
                &id,
                EvidenceKind::Custom,
                &[gate],
                summary,
                outcome,
                &self.task_id,
                call,
            ),
            None => Evidence::unattributed(&id, EvidenceKind::Custom, &[gate], summary, outcome),
        }
        .with_code_state(current.bind(&[]));
        state.attach(item).map_err(|e| e.to_string())?;
        self.store.save(&state)?;
        Ok(Some(id))
    }

    fn active(&self) -> Result<WorkflowState, String> {
        self.store.load()?.ok_or_else(|| {
            format!("no cedian workflow is active; start one with {WORKFLOW_UPDATE_TOOL} op=start")
        })
    }

    /// Both tools, for one `set_host_tools` call with the rest of the set.
    pub fn host_tools(self: &Arc<Self>) -> Vec<HostTool> {
        let update = Arc::clone(self);
        let complete = Arc::clone(self);
        vec![
            HostTool::new(
                WORKFLOW_UPDATE_TOOL,
                "cedian's own host tool (trusted). Report workflow progress to the cedian IDE. \
                 op=start {kind,title,risk?} begins a workflow; op=evidence {gate,summary,outcome,from_tool,match?,kind?} \
                 records what a tool call you already ran showed: from_tool = that tool's name (e.g. bash, read), \
                 match = a substring of its arguments; cedian binds the evidence to your most recent successful \
                 matching call (none found → stored unattributed, which cannot pass a required gate); \
                 outcome = pass | fail | inconclusive (could not run); evidence goes stale when a file it saw changes; \
                 performance evidence adds measurement {runs,median,range,limiter,build_profile} (any missing → inconclusive); \
                 op=advance {passed} moves to the next phase; op=gate {gate,gate_kind,evidence_kinds?,min_items?} \
                 adds a required gate (you can add gates, never remove or weaken one; profile+feature makes it \
                 \"feature <id> proven\"); op=profile {profile,stage,instance,ok,surprising?,from_tool,match?} \
                 records one verification-profile stage you ran (launch, doctor, drive, evidence, cleanup); \
                 evidence for a profile feature adds profile, feature and instance.",
                update_parameters(),
                move |args, _ctx| update.update(&args).map(Into::into).map_err(Into::into),
            ),
            HostTool::new(
                COMPLETE_TOOL,
                "cedian's own host tool (trusted). Call before saying a task under a cedian workflow is done. \
                 claims = what you say is true, each {text, label, evidence}: label measured (you ran it and \
                 reported the evidence), inferred (follows from evidence) or guess; evidence = the evidence ids \
                 (e1, e2, ...) cedian returned. Returns an error listing missing gates when it is not done; keep \
                 working on those, or stop and report when it says BLOCKED.",
                complete_parameters(),
                move |args, _ctx| complete.complete(&args).map(Into::into).map_err(Into::into),
            ),
        ]
    }
}

/// JSON Schema of `cedian_workflow_update` arguments (playbook skills are
/// linted against it).
pub fn update_parameters() -> Map<String, Value> {
    object(
        json!({
            "op": {"type": "string", "enum": ["start", "evidence", "advance", "gate", "profile"]},
            "profile": {"type": "string"},
            "feature": {"type": "string"},
            "instance": {"type": "string"},
            "stage": {"type": "string", "enum": ["launch", "doctor", "drive", "evidence", "cleanup"]},
            "surprising": {"type": "boolean"},
            "gate_kind": {"type": "string", "enum": ["build", "test", "lint", "reproduction", "behavior", "visual", "performance", "review"]},
            "evidence_kinds": {"type": "array", "items": {"type": "string"}},
            "min_items": {"type": "integer"},
            "kind": {"type": "string"},
            "title": {"type": "string"},
            "risk": {"type": "string", "enum": ["low", "medium", "high"]},
            "gate": {"type": "string"},
            "summary": {"type": "string"},
            "outcome": {"type": "string", "enum": ["pass", "fail", "inconclusive"]},
            "ok": {"type": "boolean"},
            "from_tool": {"type": "string"},
            "match": {"type": "string"},
            "passed": {"type": "boolean"},
            "measurement": {"type": "object", "properties": {
                "runs": {"type": "integer"}, "median": {"type": "number"},
                "range": {"type": "array", "items": {"type": "number"}},
                "limiter": {"type": "string"}, "build_profile": {"type": "string"}
            }}
        }),
        &["op"],
    )
}

/// JSON Schema of `cedian_complete` arguments.
pub fn complete_parameters() -> Map<String, Value> {
    object(
        json!({
            "summary": {"type": "string"},
            "worker": {"type": "string", "description": "complete this worker's brief verify gates (ADR-0033)"},
            "claims": {"type": "array", "items": {"type": "object", "properties": {
                "text": {"type": "string"},
                "label": {"type": "string", "enum": ["measured", "inferred", "guess"]},
                "evidence": {"type": "array", "items": {"type": "string"}}
            }, "required": ["text", "label"]}}
        }),
        &[],
    )
}

/// The claims ledger as the completion view shows it: every claim with its
/// label, evidence and flag (never hidden).
pub fn ledger_lines(claims: &[crate::CheckedClaim]) -> String {
    if claims.is_empty() {
        return "\nclaims: none given".to_string();
    }
    let mut out = String::from("\nclaims:");
    for c in claims {
        let label = format!("{:?}", c.claim.label).to_lowercase();
        let ids = if c.claim.evidence.is_empty() {
            "-".to_string()
        } else {
            c.claim.evidence.join(",")
        };
        let flag = c
            .flag
            .as_deref()
            .map(|f| format!("  ⚑ {f}"))
            .unwrap_or_default();
        out.push_str(&format!("\n  [{label}] {} ({ids}){flag}", c.claim.text));
    }
    out
}

/// One-screen status the agent reads back after each call.
fn summary(state: &WorkflowState, current: &CurrentState) -> String {
    let phase = state.current_phase.as_deref().unwrap_or("-");
    let gates: Vec<String> = state
        .all_gates(current)
        .into_iter()
        .map(|(id, r)| format!("{id}={:?}", r.status))
        .collect();
    format!(
        "{:?} · {:?} · phase {phase} · gates {}",
        state.task.kind,
        state.status,
        gates.join(" ")
    )
}

/// The evidence kind a bound call can support: exec tools show command or
/// test results, read-only tools show files, the browser shows pages. The
/// agent's `kind` only picks within that (test vs command, screenshot vs
/// browser); anything else is overridden.
pub fn kind_for(tool: &str, claimed: EvidenceKind) -> EvidenceKind {
    use EvidenceKind as K;
    match tool {
        "bash" | "eval" => match claimed {
            K::Test => K::Test,
            _ => K::Command,
        },
        "browser" => match claimed {
            K::Screenshot => K::Screenshot,
            _ => K::Browser,
        },
        "debug" => K::Debugger,
        t if READ_ONLY_TOOLS.contains(&t) => K::File,
        _ => K::Custom,
    }
}

/// Workspace files a call's args name (`read notes.txt`, `path=src/a.rs`).
/// None named → the evidence binds to the whole tree.
pub fn named_paths(args_preview: &str, current: &CurrentState) -> Vec<String> {
    args_preview
        .split(|c: char| c.is_whitespace() || "\"'`,=:()[]{}".contains(c))
        .map(|t| t.trim_start_matches("./").trim_start_matches('/'))
        .filter(|t| current.files.contains_key(*t))
        .map(str::to_string)
        .collect()
}

fn str_arg<'a>(args: &'a Map<String, Value>, key: &str) -> Result<&'a str, String> {
    args.get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("missing `{key}`"))
}

fn enum_arg<T: serde::de::DeserializeOwned>(
    args: &Map<String, Value>,
    key: &str,
) -> Result<Option<T>, String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(v) => serde_json::from_value(v.clone())
            .map(Some)
            .map_err(|_| format!("bad `{key}`: {v}")),
    }
}

fn object(properties: Value, required: &[&str]) -> Map<String, Value> {
    json!({
        "type": "object",
        "properties": properties,
        "required": required,
        "additionalProperties": false
    })
    .as_object()
    .cloned()
    .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Gate, GatePredicate, MAX_CONTINUE};

    #[derive(Default)]
    struct MemStore(Mutex<Option<WorkflowState>>);

    impl WorkflowStore for Arc<MemStore> {
        fn load(&self) -> Result<Option<WorkflowState>, String> {
            Ok(self.0.lock().unwrap().clone())
        }
        fn save(&self, state: &WorkflowState) -> Result<(), String> {
            *self.0.lock().unwrap() = Some(state.clone());
            Ok(())
        }
    }

    type Ws = Arc<Mutex<CurrentState>>;

    fn files(text: &str) -> CurrentState {
        CurrentState::from_files([("notes.txt".to_string(), text.as_bytes())])
    }

    /// Channel whose log holds two good calls: `bash-1` (`cargo test`) and
    /// `read-1` (`notes.txt`); `ws` is the workspace the gates see.
    fn channel() -> (Arc<WorkflowChannel>, Arc<MemStore>, Ws) {
        let store = Arc::new(MemStore::default());
        let ws: Ws = Arc::new(Mutex::new(files("beta")));
        let now = Arc::clone(&ws);
        let ch = WorkflowChannel::new(
            "t1",
            Box::new(Arc::clone(&store)),
            |tool, needle| {
                let (id, preview) = match tool {
                    "bash" => ("bash-1", "cargo test"),
                    "read" => ("read-1", "notes.txt"),
                    _ => return None,
                };
                preview.contains(needle).then(|| BoundCall {
                    tool_call_id: id.into(),
                    tool_name: tool.into(),
                    args_preview: preview.into(),
                    mutated_after: None,
                })
            },
            move || now.lock().unwrap().clone(),
        );
        (ch, store, ws)
    }

    fn args(v: Value) -> Map<String, Value> {
        v.as_object().unwrap().clone()
    }

    fn start(ch: &WorkflowChannel) {
        ch.update(&args(
            json!({"op": "start", "kind": "bug_fix", "title": "fix it", "risk": "low"}),
        ))
        .unwrap();
    }

    fn evidence(ch: &WorkflowChannel, gate: &str, ok: bool, tool: &str, needle: &str) -> String {
        // `ok` is the P5 wire alias for `outcome`.
        ch.update(&args(json!({
            "op": "evidence", "gate": gate, "summary": "ran it", "ok": ok,
            "from_tool": tool, "match": needle
        })))
        .unwrap()
    }

    #[test]
    fn no_workflow_completes_and_evidence_needs_one() {
        let (ch, _, _) = channel();
        assert!(
            ch.complete(&Map::new())
                .unwrap()
                .contains("nothing to check")
        );
        assert!(
            ch.update(&args(json!({"op": "advance", "passed": true})))
                .unwrap_err()
                .contains("no cedian workflow")
        );
    }

    #[test]
    fn evidence_is_attributed_only_through_the_verifier() {
        let (ch, store, _) = channel();
        start(&ch);
        assert!(evidence(&ch, "reproduce", false, "bash", "cargo").contains("attributed to bash"));
        assert!(evidence(&ch, "reproduce", false, "bash", "npm").contains("UNATTRIBUTED"));
        assert!(evidence(&ch, "reproduce", false, "grep", "").contains("UNATTRIBUTED"));
        assert!(
            ch.update(&args(
                json!({"op": "evidence", "gate": "verify", "summary": "s", "ok": true})
            ))
            .unwrap()
            .contains("no from_tool")
        );
        let state = store.load().unwrap().unwrap();
        let attributed: Vec<_> = state
            .evidence
            .values()
            .filter(|e| e.is_attributed())
            .collect();
        assert_eq!(attributed.len(), 1);
        assert_eq!(attributed[0].id, "e1");
        assert_eq!(
            attributed[0].provenance,
            crate::Provenance::Attributed {
                task_id: "t1".into(),
                tool_call_id: "bash-1".into()
            }
        );
    }

    #[test]
    fn second_start_is_refused_while_running() {
        let (ch, _, _) = channel();
        start(&ch);
        let err = ch
            .update(&args(
                json!({"op": "start", "kind": "feature", "title": "x"}),
            ))
            .unwrap_err();
        assert!(err.contains("already active"), "{err}");
    }

    #[test]
    fn complete_lists_missing_then_blocks_at_max_continue() {
        let (ch, store, _) = channel();
        start(&ch);
        // Unattributed support only: required gates still fail.
        evidence(&ch, "verify", true, "bash", "npm");
        for attempt in 1..=MAX_CONTINUE {
            let err = ch.complete(&Map::new()).unwrap_err();
            assert!(err.contains("required gate \"verify\""), "{err}");
            if attempt < MAX_CONTINUE {
                assert!(err.contains("not complete"), "{err}");
            } else {
                assert!(err.contains("now BLOCKED"), "{err}");
            }
        }
        assert_eq!(
            store.load().unwrap().unwrap().status,
            WorkflowStatus::Blocked
        );
        assert!(ch.complete(&Map::new()).unwrap_err().contains("BLOCKED"));
    }

    #[test]
    fn an_open_review_blocker_refuses_completion_until_it_closes() {
        let (ch, store, _) = channel();
        let open = Arc::new(Mutex::new(vec![
            "review: blocker f1 on /notes.txt hunk 0 is open".to_string(),
        ]));
        let source = Arc::clone(&open);
        ch.set_blockers(move || source.lock().unwrap().clone());
        start(&ch);
        evidence(&ch, "reproduce", false, "bash", "");
        evidence(&ch, "verify", true, "bash", "test");
        for _ in 0..4 {
            ch.update(&args(json!({"op": "advance", "passed": true})))
                .unwrap();
        }
        let err = ch.complete(&Map::new()).unwrap_err();
        assert!(
            err.contains("blocker f1"),
            "every gate passed, the blocker still refuses: {err}"
        );
        assert_ne!(
            store.load().unwrap().unwrap().status,
            WorkflowStatus::Complete
        );
        open.lock().unwrap().clear();
        assert!(ch.complete(&Map::new()).unwrap().starts_with("complete"));
    }

    #[test]
    fn a_review_cedian_ran_is_evidence_for_the_review_gate() {
        let (ch, store, _) = channel();
        assert_eq!(
            ch.cedian_evidence("review", Outcome::Pass, "no workflow", Some("call-1"))
                .unwrap(),
            None,
            "nothing to attach to without a workflow"
        );
        start(&ch);
        let same = ch
            .cedian_evidence(
                "review",
                Outcome::Inconclusive,
                "same model",
                Some("call-2"),
            )
            .unwrap()
            .unwrap();
        let state = store.load().unwrap().unwrap();
        let item = &state.evidence[&same];
        assert_eq!(item.outcome, Outcome::Inconclusive);
        assert!(matches!(
            &item.provenance,
            crate::Provenance::Attributed { tool_call_id, .. } if tool_call_id == "call-2"
        ));
        let gate = state.gate_result("review", &files("beta")).unwrap();
        assert_ne!(
            gate.status,
            GateStatus::Passed,
            "a same-model review never passes"
        );
        ch.cedian_evidence(
            "review",
            Outcome::Pass,
            "independent, no blocker",
            Some("call-3"),
        )
        .unwrap();
        let state = store.load().unwrap().unwrap();
        assert_eq!(
            state.gate_result("review", &files("beta")).unwrap().status,
            GateStatus::Passed
        );
    }

    #[test]
    fn attributed_evidence_and_phases_complete() {
        let (ch, store, _) = channel();
        start(&ch);
        evidence(&ch, "reproduce", false, "bash", "");
        evidence(&ch, "verify", true, "bash", "test");
        // reproduce → investigate → implement → verify; review skipped (low risk).
        for _ in 0..4 {
            ch.update(&args(json!({"op": "advance", "passed": true})))
                .unwrap();
        }
        let done = ch.complete(&Map::new()).unwrap();
        assert!(done.starts_with("complete"), "{done}");
        assert_eq!(
            store.load().unwrap().unwrap().status,
            WorkflowStatus::Complete
        );
    }

    #[test]
    fn verify_evidence_goes_stale_when_a_file_it_saw_changes() {
        let (ch, store, ws) = channel();
        start(&ch);
        evidence(&ch, "reproduce", false, "read", "notes");
        // Repo-wide test run: binds to the whole tree.
        evidence(&ch, "verify", true, "bash", "cargo test");
        let state = store.load().unwrap().unwrap();
        assert_eq!(
            state.evidence["e1"].code_state,
            Some(crate::CodeState::Files(
                [("notes.txt".to_string(), crate::content_hash(b"beta"))].into()
            ))
        );
        assert!(matches!(
            state.evidence["e2"].code_state,
            Some(crate::CodeState::Tree(_))
        ));
        *ws.lock().unwrap() = files("BETA");
        for _ in 0..4 {
            ch.update(&args(json!({"op": "advance", "passed": true})))
                .unwrap_or_else(|e| panic!("reproduce counts stale; only verify blocks: {e}"));
            if store.load().unwrap().unwrap().current_phase.as_deref() == Some("verify") {
                break;
            }
        }
        let err = ch.complete(&Map::new()).unwrap_err();
        assert!(err.contains("1 stale"), "{err}");
        // Re-capture after the edit: fresh, completes.
        evidence(&ch, "verify", true, "bash", "cargo test");
        ch.update(&args(json!({"op": "advance", "passed": true})))
            .unwrap();
        assert!(ch.complete(&Map::new()).unwrap().starts_with("complete"));
    }

    #[test]
    fn outcome_arg_and_born_stale_report() {
        let store = Arc::new(MemStore::default());
        let ch = WorkflowChannel::new(
            "t1",
            Box::new(Arc::clone(&store)),
            |_, _| {
                Some(BoundCall {
                    tool_call_id: "bash-1".into(),
                    tool_name: "bash".into(),
                    args_preview: "cargo test".into(),
                    mutated_after: Some("edit notes.txt".into()),
                })
            },
            || files("x"),
        );
        start(&ch);
        let out = ch
            .update(&args(json!({
                "op": "evidence", "gate": "verify", "summary": "s",
                "outcome": "inconclusive", "from_tool": "bash"
            })))
            .unwrap();
        assert!(out.contains("STALE: edit notes.txt ran after it"), "{out}");
        let e = &store.load().unwrap().unwrap().evidence["e1"];
        assert_eq!(e.outcome, Outcome::Inconclusive);
        assert!(e.born_stale.is_some());
        let err = ch
            .update(&args(
                json!({"op": "evidence", "gate": "verify", "summary": "s"}),
            ))
            .unwrap_err();
        assert!(err.contains("missing `outcome`"), "{err}");
    }

    #[test]
    fn complete_stores_and_shows_the_claims_ledger() {
        let (ch, store, _) = channel();
        start(&ch);
        evidence(&ch, "verify", true, "bash", "cargo test");
        let claims = json!({"claims": [
            {"text": "tests pass", "label": "measured", "evidence": ["e1"]},
            {"text": "no other callers", "label": "guess"}
        ]});
        let err = ch.complete(&args(claims)).unwrap_err();
        assert!(err.contains("[measured] tests pass (e1)\n"), "{err}");
        assert!(
            err.contains("[guess] no other callers (-)  ⚑ no evidence cited"),
            "{err}"
        );
        let last = store.load().unwrap().unwrap().last_completion.unwrap();
        assert!(!last.accepted);
        assert_eq!(last.claims.len(), 2);
        assert!(
            last.missing.iter().any(|m| m.contains("reproduce")),
            "{last:?}"
        );
        let bad = ch
            .complete(&args(json!({"claims": [{"text": "x", "label": "sure"}]})))
            .unwrap_err();
        assert!(bad.contains("bad `claims`"), "{bad}");
    }

    #[test]
    fn omp_adds_gates_but_cannot_weaken_the_floor() {
        let floor_gate = Gate::register(
            "lint",
            GateKind::Lint,
            true,
            GatePredicate {
                kinds: vec![],
                min_items: 1,
                require_ok: true,
                fresh: true,
                feature: None,
            },
            false,
        )
        .unwrap();
        let floor = GateFloor {
            rules: vec![crate::FloorRule {
                kind: TaskKind::BugFix,
                min_risk: Risk::Low,
                gates: vec![floor_gate],
            }],
        };
        let store = Arc::new(MemStore::default());
        let ch = WorkflowChannel::with_policy(
            "t1",
            Box::new(Arc::clone(&store)),
            |_, _| None,
            || files("x"),
            floor,
            Box::new(NoProfiles),
        );
        start(&ch);
        let gate = |id: &str, kind: &str| {
            ch.update(&args(json!({"op": "gate", "gate": id, "gate_kind": kind})))
        };
        assert!(
            gate("lint", "lint")
                .unwrap_err()
                .contains("cedian floor gate")
        );
        assert!(
            gate("bench", "performance")
                .unwrap()
                .contains("gate bench added")
        );
        assert!(gate("x", "nope").unwrap_err().contains("bad `gate_kind`"));
        let err = ch.complete(&Map::new()).unwrap_err();
        assert!(
            err.contains("\"lint\"") && err.contains("\"bench\""),
            "{err}"
        );
    }

    #[derive(Default)]
    struct MemProfiles {
        ledger: Mutex<ProfileLedger>,
        skill: Mutex<String>,
    }

    impl ProfileStore for Arc<MemProfiles> {
        fn load(&self) -> Result<ProfileLedger, String> {
            Ok(self.ledger.lock().unwrap().clone())
        }
        fn save(&self, ledger: &ProfileLedger) -> Result<(), String> {
            *self.ledger.lock().unwrap() = ledger.clone();
            Ok(())
        }
        fn skill(&self, profile: &str) -> Option<String> {
            (profile == "verify-notes").then(|| self.skill.lock().unwrap().clone())
        }
    }

    #[test]
    fn feature_gate_needs_a_proven_profile_and_a_healthy_instance() {
        let store = Arc::new(MemStore::default());
        let profiles = Arc::new(MemProfiles::default());
        *profiles.skill.lock().unwrap() =
            "# verify-notes\n## Feature map\n- `search`: finds the note\n".to_string();
        let ch = WorkflowChannel::with_policy(
            "t1",
            Box::new(Arc::clone(&store)),
            |tool, _| {
                Some(BoundCall {
                    tool_call_id: format!("{tool}-1"),
                    tool_name: tool.into(),
                    args_preview: "./run".into(),
                    mutated_after: None,
                })
            },
            || files("x"),
            GateFloor::default(),
            Box::new(Arc::clone(&profiles)),
        );
        start(&ch);
        let gate = |feature: &str| {
            ch.update(&args(json!({
                "op": "gate", "gate": "search", "gate_kind": "behavior",
                "profile": "verify-notes", "feature": feature
            })))
        };
        assert!(gate("nope").unwrap_err().contains("no feature \"nope\""));
        gate("search").unwrap();
        let prove = |n: usize, instance: &str| {
            let ev = json!({
                "op": "evidence", "gate": "search", "summary": "found it", "outcome": "pass",
                "from_tool": "bash", "profile": "verify-notes", "feature": "search",
                "instance": instance
            });
            let out = ch.update(&args(ev)).unwrap();
            let e = store.load().unwrap().unwrap().evidence[&format!("e{n}")].clone();
            (out, e.outcome)
        };
        let stage = |stage: &str, ok: bool| {
            ch.update(&args(json!({
                "op": "profile", "profile": "verify-notes", "instance": "i1",
                "stage": stage, "ok": ok, "from_tool": "bash"
            })))
            .unwrap()
        };
        // Draft: inconclusive.
        let (out, outcome) = prove(1, "i1");
        assert!(out.contains("is a draft"), "{out}");
        assert_eq!(outcome, Outcome::Inconclusive);
        for s in ["launch", "doctor", "drive", "evidence"] {
            stage(s, true);
        }
        assert!(stage("cleanup", true).contains("proven end to end"));
        let (_, outcome) = prove(2, "i1");
        assert_eq!(outcome, Outcome::Pass);
        // A surprising drive: the instance needs a passing doctor again.
        ch.update(&args(json!({
            "op": "profile", "profile": "verify-notes", "instance": "i1",
            "stage": "drive", "ok": true, "surprising": true, "from_tool": "bash"
        })))
        .unwrap();
        let (out, outcome) = prove(3, "i1");
        assert!(out.contains("no passing Doctor"), "{out}");
        assert_eq!(outcome, Outcome::Inconclusive);
        let st = store.load().unwrap().unwrap();
        assert_eq!(
            st.gate_result("search", &files("x")).unwrap().status,
            GateStatus::Passed,
            "e2 (proven, healthy) satisfies the feature gate"
        );
    }

    #[test]
    fn evidence_kind_follows_the_bound_tool() {
        use EvidenceKind as K;
        assert_eq!(kind_for("read", K::Test), K::File);
        assert_eq!(kind_for("bash", K::Test), K::Test);
        assert_eq!(kind_for("bash", K::File), K::Command);
        assert_eq!(kind_for("browser", K::Screenshot), K::Screenshot);
        assert_eq!(kind_for("mcp_x", K::Test), K::Custom);
        // A read reported as a test cannot pass `verify` (test|command).
        let (ch, store, _) = channel();
        start(&ch);
        ch.update(&args(json!({
            "op": "evidence", "gate": "verify", "kind": "test", "outcome": "pass",
            "summary": "looks right", "from_tool": "read"
        })))
        .unwrap();
        let state = store.load().unwrap().unwrap();
        assert_eq!(state.evidence["e1"].kind, K::File);
        assert_ne!(
            state.gate_result("verify", &files("beta")).unwrap().status,
            GateStatus::Passed
        );
    }

    #[test]
    fn named_paths_finds_workspace_files_in_args() {
        let now = CurrentState::from_files([
            ("notes.txt".to_string(), b"".as_slice()),
            ("src/a.rs".to_string(), b"".as_slice()),
        ]);
        assert_eq!(named_paths("notes.txt", &now), ["notes.txt"]);
        assert_eq!(named_paths("{\"path\":\"./src/a.rs\"}", &now), ["src/a.rs"]);
        assert!(named_paths("cargo test", &now).is_empty());
    }

    #[test]
    fn channel_calls_are_recognized_by_name_and_device() {
        assert!(is_channel_call("cedian_complete", ""));
        assert!(is_channel_call("write", "xd://cedian_workflow_update"));
        assert!(!is_channel_call("write", "xd://some_host_tool"));
        assert!(!is_channel_call("bash", "cargo test"));
        assert!(may_mutate("edit", "notes.txt"));
        assert!(may_mutate("bash", "ls"));
        assert!(!may_mutate("read", "notes.txt"));
        assert!(!may_mutate("write", "xd://cedian_complete"));
    }
}
