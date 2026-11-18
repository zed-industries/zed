//! Linux sandboxing by syscall interception.
//!
//! - [`SandboxContext`] owns a supervisor process (this binary, re-executed
//!   with a hidden flag).
//! - [`SandboxContext::create_sandbox`] registers a [`SandboxPolicy`] with
//!   the supervisor. [`Sandbox::update_policy`] replaces it while commands
//!   run.
//! - [`Sandbox::execute`] spawns the helper (also this binary), which sets up
//!   namespaces, installs a seccomp filter whose notifications go to the
//!   supervisor, and runs the command.
//!
//! The sandbox sees the host filesystem at the same paths, mounted read-only.
//! Every open and every write-intent syscall goes to the supervisor, which
//! checks the policy and performs the operation itself.
//!
//! Each command gets a network namespace from which nothing outside is
//! reachable; the supervisor answers its DNS queries and relays the TCP
//! connections and UDP flows the policy allows (see `net.rs`).
//!
//! The binary must call [`run_hidden_entrypoint_if_invoked`] first thing in
//! `main`.

#![cfg(target_os = "linux")]

mod broker;
mod dns;
mod helper;
mod net;
mod policy;
mod process;
mod protocol;
mod resolve;
mod seccomp;
mod supervisor;
mod sys;

use std::collections::{HashMap, HashSet};
use std::os::fd::{AsFd as _, AsRawFd as _, OwnedFd, RawFd};
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::JoinHandle;

use anyhow::{Context as _, Result, anyhow, bail};
use nix::sys::socket::{AddressFamily, SockFlag, SockType, socketpair};
use serde::{Deserialize, Serialize};

pub use policy::{
    Access, AccessRequest, AddressClass, Decision, DomainPattern, FsRule, Handler, IpNet, NetHost,
    NetRule, NetTarget, PathPattern, Permission, PortSet, Protocol, RequestedAccess, Rule,
    SandboxPolicy,
};
pub use protocol::{Denial, OpenFile};

use policy::{HandlerId, WireAccess, WireFsRule, WirePolicy};
use protocol::{
    HelperSetup, PromptId, Request, RequestId, Response, SandboxId, SupervisorMessage, ZedMessage,
};

/// A command to run inside a sandbox.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UntrustedCommand {
    pub cmd: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub cwd: PathBuf,
}

/// Answers one [`AccessRequest`] passed to a [`Handler`]. Answer from any
/// thread, at any time.
///
/// Every request must be answered with [`Responder::respond`]: a decision
/// usually also remembers a rule, and silently dropping the responder would
/// lose that. Dropping it unanswered is treated as a bug: it panics in debug
/// builds, and logs a warning and denies in release builds. Dropping is fine
/// once [`Responder::is_cancelled`] is true, or while unwinding from a panic
/// (both deny).
pub struct Responder {
    prompt: PromptId,
    sandbox: SandboxId,
    handlers: Arc<HandlerTable>,
    control: Arc<OwnedFd>,
    cancelled: Arc<AtomicBool>,
    answered: bool,
}

impl Responder {
    pub fn respond(mut self, decision: Decision) {
        self.send(decision);
    }

    /// Whether the blocked syscall was abandoned (e.g. the process was
    /// killed) or timed out, so an answer would no longer have any effect.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    fn send(&mut self, decision: Decision) {
        if std::mem::replace(&mut self.answered, true) {
            return;
        }
        self.handlers.forget_prompt(self.prompt);
        let mut register = |handler: &Handler| self.handlers.register(self.sandbox, handler);
        let remember = decision
            .remember
            .iter()
            .map(|rule| rule.to_wire(&mut register))
            .collect();
        let reply = ZedMessage::PromptReply {
            prompt: self.prompt,
            allow: decision.allow,
            remember,
        };
        if let Err(error) = protocol::send_message(self.control.as_fd(), &reply, &[]) {
            eprintln!("zed sandbox: answering prompt failed: {error:#}");
        }
    }
}

impl Drop for Responder {
    fn drop(&mut self) {
        if self.answered {
            return;
        }
        let is_bug = !self.is_cancelled() && !std::thread::panicking();
        // Deny before panicking, so the syscall isn't left blocked until the
        // prompt times out.
        self.send(Decision::deny());
        if is_bug {
            if cfg!(debug_assertions) {
                panic!("a sandbox Responder was dropped without responding");
            }
            log::warn!("[sandbox] a Responder was dropped without responding; denying");
        }
    }
}

/// Every sandbox's handlers, by the id the supervisor refers to them with.
#[derive(Default)]
struct HandlerTable {
    state: Mutex<HandlerTableState>,
}

type DenialHandler = Arc<dyn Fn(Denial) + Send + Sync>;

#[derive(Default)]
struct HandlerTableState {
    handlers: HashMap<HandlerId, (SandboxId, Handler)>,
    denial_handlers: HashMap<SandboxId, DenialHandler>,
    /// Unanswered prompts, so a cancellation can reach their responders.
    prompts: HashMap<PromptId, Arc<AtomicBool>>,
    next: HandlerId,
}

impl HandlerTable {
    fn state(&self) -> std::sync::MutexGuard<'_, HandlerTableState> {
        match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    /// The id of `handler` for `sandbox`, reusing the existing one if it was
    /// registered before.
    fn register(&self, sandbox: SandboxId, handler: &Handler) -> HandlerId {
        let mut state = self.state();
        let existing = state
            .handlers
            .iter()
            .find(|(_, (owner, known))| *owner == sandbox && Arc::ptr_eq(&known.0, &handler.0));
        if let Some((&id, _)) = existing {
            return id;
        }
        let id = state.next;
        state.next += 1;
        state.handlers.insert(id, (sandbox, handler.clone()));
        id
    }

    fn get(&self, id: HandlerId) -> Option<(SandboxId, Handler)> {
        self.state().handlers.get(&id).cloned()
    }

    /// Drop `sandbox`'s handlers except those in `keep`.
    fn retain(&self, sandbox: SandboxId, keep: &HashSet<HandlerId>) {
        self.state()
            .handlers
            .retain(|id, (owner, _)| *owner != sandbox || keep.contains(id));
    }

    fn denial_handler(&self, sandbox: SandboxId) -> Option<DenialHandler> {
        self.state().denial_handlers.get(&sandbox).cloned()
    }

    fn track_prompt(&self, prompt: PromptId) -> Arc<AtomicBool> {
        let cancelled = Arc::new(AtomicBool::new(false));
        self.state().prompts.insert(prompt, cancelled.clone());
        cancelled
    }

    fn forget_prompt(&self, prompt: PromptId) {
        self.state().prompts.remove(&prompt);
    }

    fn cancel_prompt(&self, prompt: PromptId) {
        if let Some(cancelled) = self.state().prompts.remove(&prompt) {
            cancelled.store(true, Ordering::SeqCst);
        }
    }

    /// The supervisor is gone: no answer can have an effect any more.
    fn cancel_all_prompts(&self) {
        for (_, cancelled) in self.state().prompts.drain() {
            cancelled.store(true, Ordering::SeqCst);
        }
    }
}

/// Run the supervisor or helper if this process was started as one; never
/// returns in that case. Call before anything else in `main`: the helper's
/// command line must not be interpreted as the host program's own.
pub fn run_hidden_entrypoint_if_invoked() {
    match std::env::args_os()
        .nth(1)
        .as_deref()
        .and_then(|flag| flag.to_str())
    {
        Some(supervisor::SUPERVISOR_FLAG) => run_supervisor(),
        Some(helper::HELPER_FLAG) => helper::run(),
        _ => {}
    }
}

/// The supervisor process's main loop. Expects its control socket at fd 3.
pub fn run_supervisor() -> ! {
    supervisor::run()
}

/// Owns the supervisor process. Dropping it stops the supervisor; commands
/// still running then fail every trapped syscall with `ENOSYS`.
pub struct SandboxContext {
    inner: Arc<ContextInner>,
}

struct ContextInner {
    executable: PathBuf,
    control: Arc<OwnedFd>,
    supervisor: Mutex<Child>,
    reader: Mutex<Option<JoinHandle<()>>>,
    pending: Arc<Mutex<HashMap<RequestId, mpsc::Sender<Response>>>>,
    handlers: Arc<HandlerTable>,
    next_request: AtomicU64,
    next_sandbox: AtomicU64,
}

impl SandboxContext {
    /// Start a supervisor by re-executing the current binary.
    pub fn new() -> Result<Self> {
        let executable = std::env::current_exe().context("locating the current executable")?;
        Self::with_executable(executable)
    }

    /// Start a supervisor using `executable`, which must call
    /// [`run_hidden_entrypoint_if_invoked`].
    #[allow(
        clippy::disallowed_methods,
        reason = "PoC: blocking spawn; callers must stay off the UI thread"
    )]
    pub fn with_executable(executable: PathBuf) -> Result<Self> {
        let (ours, theirs) = seqpacket_pair()?;
        let mut command = Command::new(&executable);
        command
            .arg(supervisor::SUPERVISOR_FLAG)
            .stdin(Stdio::null());
        let child_fds = inherit_fds(&mut command, &[(theirs, supervisor::CONTROL_FD)])?;
        let supervisor = command.spawn().context("starting the sandbox supervisor")?;
        drop(child_fds);

        let control = Arc::new(ours);
        let pending: Arc<Mutex<HashMap<RequestId, mpsc::Sender<Response>>>> = Arc::default();
        let handlers: Arc<HandlerTable> = Arc::default();
        let reader = std::thread::Builder::new()
            .name("zed-sandbox-control".to_string())
            .spawn({
                let control = control.clone();
                let pending = pending.clone();
                let handlers = handlers.clone();
                move || read_control(&control, &pending, &handlers)
            })
            .context("starting the sandbox control thread")?;
        Ok(Self {
            inner: Arc::new(ContextInner {
                executable,
                control,
                supervisor: Mutex::new(supervisor),
                reader: Mutex::new(Some(reader)),
                pending,
                handlers,
                next_request: AtomicU64::new(0),
                next_sandbox: AtomicU64::new(0),
            }),
        })
    }

    pub fn create_sandbox(&self, policy: SandboxPolicy) -> Result<Sandbox> {
        let id = self.inner.next_sandbox.fetch_add(1, Ordering::Relaxed);
        let temp_dir = create_temp_dir()?;
        // Dropping `sandbox` on an error below cleans up its directory and
        // handlers without asking the supervisor to destroy it.
        let mut sandbox = Sandbox {
            context: self.inner.clone(),
            id,
            temp_dir,
            created: false,
        };
        let policy = sandbox.to_wire(&policy)?;
        self.inner.call(
            Request::CreateSandbox {
                sandbox: id,
                policy,
            },
            &[],
        )?;
        sandbox.created = true;
        Ok(sandbox)
    }
}

fn read_control(
    control: &Arc<OwnedFd>,
    pending: &Mutex<HashMap<RequestId, mpsc::Sender<Response>>>,
    handlers: &Arc<HandlerTable>,
) {
    loop {
        let message = match protocol::receive_message::<SupervisorMessage>(control.as_fd()) {
            Ok(Some((message, _))) => message,
            Ok(None) => break,
            Err(error) => {
                eprintln!("zed sandbox: reading from supervisor failed: {error:#}");
                break;
            }
        };
        match message {
            SupervisorMessage::Response { id, response } => {
                let sender = pending
                    .lock()
                    .ok()
                    .and_then(|mut pending| pending.remove(&id));
                if let Some(sender) = sender {
                    sender.send(response).ok();
                }
            }
            SupervisorMessage::Prompt(request) => {
                // An unknown (or another sandbox's) handler, e.g. one a policy
                // update just removed: deny.
                let handler = handlers
                    .get(request.handler)
                    .filter(|(owner, _)| *owner == request.sandbox);
                let Some((_, handler)) = handler else {
                    let reply = ZedMessage::PromptReply {
                        prompt: request.prompt,
                        allow: false,
                        remember: Vec::new(),
                    };
                    if let Err(error) = protocol::send_message(control.as_fd(), &reply, &[]) {
                        eprintln!("zed sandbox: answering prompt failed: {error:#}");
                    }
                    continue;
                };
                let responder = Responder {
                    prompt: request.prompt,
                    sandbox: request.sandbox,
                    handlers: handlers.clone(),
                    control: control.clone(),
                    cancelled: handlers.track_prompt(request.prompt),
                    answered: false,
                };
                let access_request = AccessRequest {
                    items: request.requests,
                    pid: request.pid,
                    executable: request.executable,
                };
                // A panicking handler drops the responder while unwinding,
                // which denies; keep this thread alive for everyone else.
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    (handler.0)(access_request, responder)
                }));
                if result.is_err() {
                    eprintln!("zed sandbox: an access handler panicked; denying");
                }
            }
            SupervisorMessage::PromptCancelled { prompt } => handlers.cancel_prompt(prompt),
            SupervisorMessage::Denied { sandbox, denial } => {
                if let Some(handler) = handlers.denial_handler(sandbox) {
                    let result =
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handler(denial)));
                    if result.is_err() {
                        eprintln!("zed sandbox: a denial handler panicked");
                    }
                }
            }
        }
    }
    // Fail anyone still waiting for a response.
    if let Ok(mut pending) = pending.lock() {
        pending.clear();
    }
    handlers.cancel_all_prompts();
}

impl ContextInner {
    fn call(&self, request: Request, fds: &[RawFd]) -> Result<Response> {
        let id = self.next_request.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = mpsc::channel();
        self.pending
            .lock()
            .map_err(|_| anyhow!("sandbox state poisoned"))?
            .insert(id, sender);
        protocol::send_message(
            self.control.as_fd(),
            &ZedMessage::Request { id, request },
            fds,
        )?;
        match receiver.recv().context("sandbox supervisor exited")? {
            Response::Error { message } => Err(anyhow!(message)),
            response => Ok(response),
        }
    }
}

impl Drop for ContextInner {
    fn drop(&mut self) {
        // The supervisor exits on EOF. Shut down rather than close: the
        // control thread (and any unanswered prompt) shares the socket.
        // SAFETY: shutdown on a socket we own.
        unsafe { libc::shutdown(self.control.as_raw_fd(), libc::SHUT_WR) };
        if let Ok(mut supervisor) = self.supervisor.lock()
            && let Err(error) = supervisor.wait()
        {
            eprintln!("zed sandbox: waiting for supervisor failed: {error}");
        }
        if let Some(reader) = self.reader.lock().ok().and_then(|mut reader| reader.take()) {
            reader.join().ok();
        }
    }
}

/// A policy registered with the supervisor. Commands run in it share the
/// policy (and its updates), but each gets its own namespaces.
///
/// Each sandbox has a private temporary directory, passed to commands as
/// `TMPDIR` and usable (read, write, connect) regardless of the policy.
/// Device files commands commonly need (`/dev/null`, terminals, ...) are readable and writable
/// unless the policy says otherwise.
pub struct Sandbox {
    context: Arc<ContextInner>,
    id: SandboxId,
    temp_dir: PathBuf,
    /// Whether the supervisor knows about this sandbox.
    created: bool,
}

/// Device files a sandbox may use unless its policy overrides them.
const DEFAULT_DEVICES: [&str; 8] = [
    "/dev/null",
    "/dev/zero",
    "/dev/full",
    "/dev/random",
    "/dev/urandom",
    "/dev/tty",
    "/dev/ptmx",
    "/dev/pts",
];

impl Sandbox {
    pub fn temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    /// Call `handler` whenever the policy, or an [`Access::Ask`] handler,
    /// denies an operation in this sandbox (after the syscall has failed).
    /// Like handlers, it runs on a background thread shared by every
    /// sandbox in the context and must return promptly. Replaces any
    /// previous one.
    pub fn on_denial(&self, handler: impl Fn(Denial) + Send + Sync + 'static) {
        self.context
            .handlers
            .state()
            .denial_handlers
            .insert(self.id, Arc::new(handler));
    }

    /// Replace the policy, including any rules handlers asked to remember.
    /// Applies to every operation from now on, including in commands already
    /// running; descriptors they already hold keep working, and are returned
    /// if the new policy wouldn't grant them.
    pub fn update_policy(&self, policy: &SandboxPolicy) -> Result<Vec<OpenFile>> {
        let policy = self.to_wire(policy)?;
        let request = Request::UpdatePolicy {
            sandbox: self.id,
            policy,
        };
        let still_open = match self.context.call(request, &[])? {
            Response::PolicyUpdated { still_open } => still_open,
            other => bail!("unexpected supervisor response: {other:?}"),
        };
        Ok(still_open)
    }

    /// The policy as the supervisor enforces it, registering its handlers:
    /// default devices first (so the policy can override them), the
    /// temporary directory last (so it can't).
    ///
    /// Handlers no longer referenced are dropped, which denies any prompt
    /// still waiting on them.
    fn to_wire(&self, policy: &SandboxPolicy) -> Result<WirePolicy> {
        let handlers = &self.context.handlers;
        let mut used = HashSet::new();
        let mut register = |handler: &Handler| {
            let id = handlers.register(self.id, handler);
            used.insert(id);
            id
        };
        let allow_all = |path: PathPattern| WireFsRule {
            path,
            read: Some(WireAccess::Allow),
            write: Some(WireAccess::Allow),
            connect: Some(WireAccess::Allow),
        };
        let mut fs = Vec::with_capacity(policy.fs.len() + DEFAULT_DEVICES.len() + 1);
        for device in DEFAULT_DEVICES {
            fs.push(allow_all(PathPattern::new(device)?));
        }
        fs.extend(policy.fs.iter().map(|rule| rule.to_wire(&mut register)));
        fs.push(allow_all(PathPattern::literal(&self.temp_dir)?));
        let net = policy
            .net
            .iter()
            .map(|rule| rule.to_wire(&mut register))
            .collect();
        handlers.retain(self.id, &used);
        Ok(WirePolicy { fs, net })
    }

    /// Spawn `command` in this sandbox with inherited stdio.
    pub fn execute(&self, command: &UntrustedCommand) -> Result<Child> {
        Ok(self.prepare(command)?.spawn()?)
    }

    /// Prepare `command` without spawning it. The supervisor is told about
    /// the execution now; if it's never spawned, the supervisor forgets it
    /// once the returned value is dropped.
    pub fn prepare(&self, command: &UntrustedCommand) -> Result<SandboxedCommand> {
        // The helper hands its seccomp listener to the supervisor over this
        // pair; the supervisor learns which sandbox it belongs to from the
        // request that delivered the other end.
        let (supervisor_end, helper_attach) = seqpacket_pair()?;
        self.context.call(
            Request::AttachExecution { sandbox: self.id },
            &[supervisor_end.as_raw_fd()],
        )?;
        drop(supervisor_end);

        let mut command = command.clone();
        command.env.insert(
            "TMPDIR".to_string(),
            self.temp_dir.to_string_lossy().into_owned(),
        );
        // The setup message sits in the socket buffer until the helper reads
        // it, so it can be sent before spawning.
        let (zed_setup, helper_setup) = seqpacket_pair()?;
        protocol::send_message(zed_setup.as_fd(), &HelperSetup { command }, &[])?;
        drop(zed_setup);

        let mut process = Command::new(&self.context.executable);
        process.arg(helper::HELPER_FLAG);
        let child_fds = inherit_fds(
            &mut process,
            &[
                (helper_setup, helper::SETUP_FD),
                (helper_attach, helper::ATTACH_FD),
            ],
        )?;
        Ok(SandboxedCommand {
            command: process,
            _child_fds: child_fds,
        })
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        self.context.handlers.retain(self.id, &HashSet::new());
        if self.created
            && let Err(error) = self
                .context
                .call(Request::DestroySandbox { sandbox: self.id }, &[])
        {
            eprintln!("zed sandbox: destroying sandbox failed: {error:#}");
        }
        // Only now: the destroy round trip went through the control thread
        // after every denial the supervisor sent before it, so all of them
        // have been delivered.
        self.context
            .handlers
            .state()
            .denial_handlers
            .remove(&self.id);
        if let Err(error) = std::fs::remove_dir_all(&self.temp_dir) {
            eprintln!(
                "zed sandbox: removing {} failed: {error}",
                self.temp_dir.display()
            );
        }
    }
}

/// A sandboxed command that hasn't been spawned yet. Configure stdio through
/// [`SandboxedCommand::command_mut`]; don't change its program or arguments.
pub struct SandboxedCommand {
    command: Command,
    _child_fds: Vec<OwnedFd>,
}

impl SandboxedCommand {
    pub fn command_mut(&mut self) -> &mut Command {
        &mut self.command
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "PoC: blocking spawn; callers must stay off the UI thread"
    )]
    pub fn spawn(mut self) -> std::io::Result<Child> {
        self.command.spawn()
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "PoC: blocking spawn; callers must stay off the UI thread"
    )]
    pub fn output(mut self) -> std::io::Result<Output> {
        self.command.output()
    }
}

fn create_temp_dir() -> Result<PathBuf> {
    use std::os::unix::fs::DirBuilderExt as _;
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let base = std::env::temp_dir()
        .canonicalize()
        .context("resolving the temporary directory")?;
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.subsec_nanos());
    let path = base.join(format!(
        "zed-sandbox-{}-{}-{nanos}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::DirBuilder::new()
        .mode(0o700)
        .create(&path)
        .with_context(|| format!("creating {}", path.display()))?;
    Ok(path)
}

fn seqpacket_pair() -> Result<(OwnedFd, OwnedFd)> {
    socketpair(
        AddressFamily::Unix,
        SockType::SeqPacket,
        None,
        SockFlag::SOCK_CLOEXEC,
    )
    .context("creating sandbox socketpair")
}

/// Arrange for each `(fd, number)` to be inherited by `command` at `number`.
/// Returns the descriptors the caller must keep open until spawning.
fn inherit_fds(command: &mut Command, fds: &[(OwnedFd, RawFd)]) -> Result<Vec<OwnedFd>> {
    // Move every source above the target range first, so one `dup2` can't
    // clobber another source.
    let mut moved = Vec::new();
    let mut mapping = Vec::new();
    for (fd, number) in fds {
        let high = sys::duplicate_above(fd.as_fd(), 64)?;
        mapping.push((high.as_raw_fd(), *number));
        moved.push(high);
    }
    // SAFETY: only async-signal-safe `dup2` calls between fork and exec.
    unsafe {
        command.pre_exec(move || {
            for (source, target) in &mapping {
                // `dup2` leaves the new descriptor without CLOEXEC.
                sys::cvt(libc::dup2(*source, *target))?;
            }
            Ok(())
        });
    }
    Ok(moved)
}
