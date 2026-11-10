//! The supervisor process (`zed --zed-sandbox-linux-supervisor`).
//!
//! One per [`SandboxContext`](crate::SandboxContext). The main thread polls:
//! - the control socket from Zed ([`CONTROL_FD`]): requests, prompt replies;
//! - pending attach sockets, each waiting for one helper's listener fd;
//! - seccomp listeners, one per running execution, keyed to its sandbox.
//!
//! It only receives notifications; workers handle them, since handling can
//! block (opening a FIFO, a slow filesystem, waiting for the user).
//!
//! A notification can only arrive on the listener of the filter that trapped
//! it, so the listener identifies the sandbox; nothing the sandboxed process
//! sends can claim another identity.
//!
//! The process exits when Zed shuts down the control socket. Listeners close
//! with it, after which every trapped syscall fails with `ENOSYS`.

use std::collections::{HashMap, VecDeque};
use std::os::fd::{AsFd as _, AsRawFd as _, OwnedFd, RawFd};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock, mpsc};
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result, bail};

use crate::broker;
use crate::dns;
use crate::net::{self, Network};
use crate::policy::{HandlerId, Permission, RequestedAccess, WireAccess, WirePolicy, WireRule};
use crate::process::ProcessTree;
use crate::protocol::{
    AttachInfo, Denial, OpenFile, PromptId, PromptRequest, Request, Response, SandboxId,
    SupervisorMessage, ZedMessage, receive_message, send_message,
};
use crate::sys;

pub(crate) const SUPERVISOR_FLAG: &str = "--zed-sandbox-linux-supervisor";
pub(crate) const CONTROL_FD: RawFd = 3;

const PROMPT_TIMEOUT: Duration = Duration::from_secs(5 * 60);
const MAX_WORKERS: usize = 64;

pub(crate) struct SandboxState {
    pub id: SandboxId,
    policy: RwLock<Arc<WirePolicy>>,
    pub names: Mutex<dns::Names>,
}

impl SandboxState {
    pub fn policy(&self) -> Arc<WirePolicy> {
        match self.policy.read() {
            Ok(policy) => policy.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        }
    }

    fn modify_policy(&self, modify: impl FnOnce(&mut WirePolicy)) {
        let mut current = match self.policy.write() {
            Ok(current) => current,
            Err(poisoned) => poisoned.into_inner(),
        };
        modify(Arc::make_mut(&mut current));
    }
}

struct PromptReply {
    allow: bool,
    remember: Vec<WireRule>,
}

/// One running command (and everything it spawns).
pub(crate) struct Execution {
    pub listener: OwnedFd,
    pub sandbox: Arc<SandboxState>,
    pub tree: ProcessTree,
    pub network: Network,
}

/// What [`Shared::ask`] concluded.
pub(crate) enum AskOutcome {
    Decided(bool),
    /// Another prompt to the same handler finished while this one waited;
    /// re-evaluate the (possibly extended) policy before asking.
    Retry,
}

/// The prompts of one handler in one sandbox, shown one at a time.
///
/// While one is open, an identical request waits for and shares its
/// decision; any other request waits for it to finish and then retries, so
/// it sees rules the first one remembered.
#[derive(Default)]
struct PromptQueue {
    state: Mutex<PromptQueueState>,
    changed: Condvar,
}

#[derive(Default)]
struct PromptQueueState {
    open: Option<Vec<RequestedAccess>>,
    /// Number of prompts finished so far.
    finished: u64,
    /// Recent `(prompt number, requests, decision)`, for identical requests
    /// that waited on them.
    recent: VecDeque<(u64, Vec<RequestedAccess>, bool)>,
}

const RECENT_DECISIONS: usize = 64;

impl PromptQueue {
    fn lock(&self) -> std::sync::MutexGuard<'_, PromptQueueState> {
        match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}

/// State workers share with the main thread.
pub(crate) struct Shared {
    control: OwnedFd,
    prompts: Mutex<HashMap<PromptId, mpsc::Sender<PromptReply>>>,
    next_prompt: AtomicU64,
    queues: Mutex<HashMap<(SandboxId, HandlerId), Arc<PromptQueue>>>,
}

impl Shared {
    fn send(&self, message: &SupervisorMessage) {
        // SEQPACKET sends are atomic, so workers and the main thread can
        // share the socket.
        if let Err(error) = send_message(self.control.as_fd(), message, &[]) {
            eprintln!("zed sandbox supervisor: {error:#}");
        }
    }

    /// Ask Zed's `handler` whether to allow `requests`, through the
    /// handler's [`PromptQueue`]. Blocks until it answers, the prompt times
    /// out (deny), or the syscall is abandoned (`still_valid` turns false).
    /// Rules the handler asks to remember are added to the sandbox's policy
    /// before any waiting request wakes up.
    pub fn ask(
        &self,
        sandbox: &SandboxState,
        handler: HandlerId,
        requests: Vec<RequestedAccess>,
        tid: i32,
        still_valid: impl Fn() -> bool,
    ) -> AskOutcome {
        let queue = match self.queues.lock() {
            Ok(mut queues) => queues.entry((sandbox.id, handler)).or_default().clone(),
            Err(_) => return AskOutcome::Decided(false),
        };
        let mut state = queue.lock();
        if let Some(open) = &state.open {
            let identical = *open == requests;
            let awaited = state.finished + 1;
            while state.finished < awaited {
                if !still_valid() {
                    return AskOutcome::Decided(false);
                }
                state = match queue.changed.wait_timeout(state, Duration::from_secs(1)) {
                    Ok((state, _)) => state,
                    Err(poisoned) => poisoned.into_inner().0,
                };
            }
            let shared = state
                .recent
                .iter()
                .find(|(number, _, _)| *number == awaited)
                .filter(|(_, decided, _)| identical && *decided == requests)
                .map(|(_, _, allow)| *allow);
            return match shared {
                Some(allow) => AskOutcome::Decided(allow),
                None => AskOutcome::Retry,
            };
        }
        state.open = Some(requests.clone());
        drop(state);

        let allow = self.prompt(sandbox, handler, requests.clone(), tid, still_valid);

        let mut state = queue.lock();
        state.open = None;
        state.finished += 1;
        let number = state.finished;
        state.recent.push_back((number, requests, allow));
        if state.recent.len() > RECENT_DECISIONS {
            state.recent.pop_front();
        }
        queue.changed.notify_all();
        AskOutcome::Decided(allow)
    }

    fn prompt(
        &self,
        sandbox: &SandboxState,
        handler: HandlerId,
        requests: Vec<RequestedAccess>,
        tid: i32,
        still_valid: impl Fn() -> bool,
    ) -> bool {
        let prompt = self.next_prompt.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = mpsc::channel();
        if let Ok(mut prompts) = self.prompts.lock() {
            prompts.insert(prompt, sender);
        }
        self.send(&SupervisorMessage::Prompt(PromptRequest {
            prompt,
            sandbox: sandbox.id,
            handler,
            requests,
            pid: tid,
            executable: std::fs::read_link(format!("/proc/{tid}/exe")).ok(),
        }));
        let start = Instant::now();
        let decision = loop {
            match receiver.recv_timeout(Duration::from_secs(1)) {
                Ok(reply) => break Some(reply),
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if !still_valid() || start.elapsed() > PROMPT_TIMEOUT {
                        break None;
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => break None,
            }
        };
        if let Ok(mut prompts) = self.prompts.lock() {
            prompts.remove(&prompt);
        }
        let Some(reply) = decision else {
            self.send(&SupervisorMessage::PromptCancelled { prompt });
            return false;
        };
        if !reply.remember.is_empty() {
            sandbox.modify_policy(|policy| {
                for rule in reply.remember {
                    policy.add(rule);
                }
            });
        }
        reply.allow
    }

    /// Whether `request` is allowed, asking its handler if the policy says
    /// so. Denials are reported.
    pub fn authorize(
        &self,
        sandbox: &SandboxState,
        request: &RequestedAccess,
        tid: i32,
        still_valid: &dyn Fn() -> bool,
    ) -> bool {
        let allowed = loop {
            match sandbox.policy().access(request) {
                WireAccess::Allow => break true,
                WireAccess::Deny => break false,
                WireAccess::Ask(handler) => {
                    match self.ask(sandbox, handler, vec![request.clone()], tid, still_valid) {
                        AskOutcome::Decided(allowed) => break allowed,
                        // Re-check the policy, which may now decide.
                        AskOutcome::Retry => {}
                    }
                }
            }
        };
        if !allowed {
            self.report_denial(sandbox.id, vec![request.clone()], tid);
        }
        allowed
    }

    /// Tell Zed a policy rule (or a handler) denied an operation.
    pub fn report_denial(&self, sandbox: SandboxId, requests: Vec<RequestedAccess>, tid: i32) {
        self.send(&SupervisorMessage::Denied {
            sandbox,
            denial: Denial {
                requests,
                pid: tid,
                executable: std::fs::read_link(format!("/proc/{tid}/exe")).ok(),
            },
        });
    }

    fn reply(&self, prompt: PromptId, reply: PromptReply) {
        let sender = self
            .prompts
            .lock()
            .ok()
            .and_then(|mut prompts| prompts.remove(&prompt));
        if let Some(sender) = sender {
            sender.send(reply).ok();
        }
    }
}

type Job = (Arc<Execution>, libc::seccomp_notif);

/// Workers are added whenever none is idle (up to [`MAX_WORKERS`]), so a
/// few syscalls blocked on prompts don't stall everything else.
struct Pool {
    shared: Arc<Shared>,
    sender: mpsc::Sender<Job>,
    receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    idle: Arc<AtomicUsize>,
    workers: usize,
}

impl Pool {
    fn new(shared: Arc<Shared>) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            shared,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
            idle: Arc::new(AtomicUsize::new(0)),
            workers: 0,
        }
    }

    fn dispatch(&mut self, job: Job) {
        if self.idle.load(Ordering::SeqCst) == 0 && self.workers < MAX_WORKERS {
            let shared = self.shared.clone();
            let receiver = self.receiver.clone();
            let idle = self.idle.clone();
            let spawned = std::thread::Builder::new()
                .name("zed-sandbox-worker".to_string())
                .spawn(move || run_worker(&shared, &receiver, &idle));
            match spawned {
                Ok(_) => self.workers += 1,
                Err(error) => eprintln!("zed sandbox supervisor: spawning worker: {error}"),
            }
        }
        self.sender.send(job).ok();
    }
}

fn run_worker(shared: &Arc<Shared>, receiver: &Mutex<mpsc::Receiver<Job>>, idle: &AtomicUsize) {
    loop {
        idle.fetch_add(1, Ordering::SeqCst);
        let job = match receiver.lock() {
            Ok(receiver) => receiver.recv(),
            Err(_) => return,
        };
        idle.fetch_sub(1, Ordering::SeqCst);
        let Ok((execution, notification)) = job else {
            return;
        };
        broker::handle(shared, &execution, notification);
    }
}

pub(crate) fn run() -> ! {
    // Relays write to sockets whose peer may be gone: that's an error to
    // handle, not a reason to die.
    // SAFETY: plain signal disposition change.
    unsafe { libc::signal(libc::SIGPIPE, libc::SIG_IGN) };
    if let Err(error) = sys::set_parent_death_signal() {
        eprintln!("zed sandbox supervisor: {error}");
        std::process::exit(1);
    }
    // SAFETY: Zed spawns the supervisor with exactly this descriptor.
    let control = match unsafe { sys::inherited_fd(CONTROL_FD) } {
        Ok(control) => control,
        Err(error) => {
            eprintln!("zed sandbox supervisor: missing control socket: {error}");
            std::process::exit(1);
        }
    };
    let shared = Arc::new(Shared {
        control,
        prompts: Mutex::new(HashMap::new()),
        next_prompt: AtomicU64::new(0),
        queues: Mutex::new(HashMap::new()),
    });
    Supervisor {
        pool: Pool::new(shared.clone()),
        shared,
        sandboxes: HashMap::new(),
        pending: Vec::new(),
        executions: Vec::new(),
    }
    .run();
    std::process::exit(0);
}

struct Supervisor {
    shared: Arc<Shared>,
    pool: Pool,
    sandboxes: HashMap<SandboxId, Arc<SandboxState>>,
    /// Attach sockets whose helper hasn't sent its listener yet.
    pending: Vec<(OwnedFd, Arc<SandboxState>)>,
    /// Running executions. Each holds its own reference to its sandbox, so
    /// destroying a sandbox doesn't strand its commands.
    executions: Vec<Arc<Execution>>,
}

impl Supervisor {
    fn run(&mut self) {
        loop {
            let mut poll_fds = vec![poll_fd(self.shared.control.as_raw_fd())];
            poll_fds.extend(self.pending.iter().map(|(fd, _)| poll_fd(fd.as_raw_fd())));
            poll_fds.extend(
                self.executions
                    .iter()
                    .map(|execution| poll_fd(execution.listener.as_raw_fd())),
            );
            // SAFETY: a valid array of pollfds.
            let ready = unsafe { libc::poll(poll_fds.as_mut_ptr(), poll_fds.len() as _, -1) };
            if ready < 0 {
                let error = std::io::Error::last_os_error();
                if error.raw_os_error() == Some(libc::EINTR) {
                    continue;
                }
                eprintln!("zed sandbox supervisor: poll failed: {error}");
                return;
            }

            let pending_count = self.pending.len();
            let (control_ready, rest) = poll_fds.split_at(1);
            let (pending_ready, listeners_ready) = rest.split_at(pending_count);

            // Walk backwards so removals don't shift indices still to visit.
            for (index, entry) in listeners_ready.iter().enumerate().rev() {
                if entry.revents & libc::POLLIN != 0 {
                    let execution = &self.executions[index];
                    if let Some(notification) = broker::receive(execution.listener.as_fd()) {
                        self.pool.dispatch((execution.clone(), notification));
                    }
                } else if entry.revents & (libc::POLLHUP | libc::POLLERR | libc::POLLNVAL) != 0 {
                    // Every task under this filter has exited.
                    self.executions.swap_remove(index).network.close();
                }
            }
            for (index, entry) in pending_ready.iter().enumerate().rev() {
                if entry.revents == 0 {
                    continue;
                }
                let (socket, sandbox) = self.pending.swap_remove(index);
                match receive_message::<AttachInfo>(socket.as_fd()) {
                    Ok(Some((info, mut fds))) if !fds.is_empty() => {
                        let listener = fds.remove(0);
                        let execution = Arc::new(Execution {
                            listener,
                            sandbox,
                            tree: ProcessTree {
                                helper: info.helper_pid,
                            },
                            // From now on the helper serves socket requests
                            // on the attach socket.
                            network: Network::new(socket),
                        });
                        net::start(&self.shared, &execution, fds);
                        self.executions.push(execution);
                    }
                    // The helper exited (or misbehaved) before attaching. Its
                    // command never ran, since it attaches before exec.
                    Ok(_) => {}
                    Err(error) => eprintln!("zed sandbox supervisor: attach failed: {error:#}"),
                }
            }
            if control_ready[0].revents != 0 && !self.handle_control() {
                return;
            }
        }
    }

    /// Handle one control message. Returns `false` once Zed has gone away.
    fn handle_control(&mut self) -> bool {
        let (message, fds) = match receive_message::<ZedMessage>(self.shared.control.as_fd()) {
            Ok(Some(message)) => message,
            Ok(None) => return false,
            Err(error) => {
                eprintln!("zed sandbox supervisor: bad control message: {error:#}");
                return false;
            }
        };
        match message {
            ZedMessage::PromptReply {
                prompt,
                allow,
                remember,
            } => self.shared.reply(prompt, PromptReply { allow, remember }),
            ZedMessage::Request { id, request } => {
                let response =
                    self.handle_request(request, fds)
                        .unwrap_or_else(|error| Response::Error {
                            message: format!("{error:#}"),
                        });
                self.shared
                    .send(&SupervisorMessage::Response { id, response });
            }
        }
        true
    }

    fn handle_request(&mut self, request: Request, mut fds: Vec<OwnedFd>) -> Result<Response> {
        match request {
            Request::CreateSandbox { sandbox, policy } => {
                if self.sandboxes.contains_key(&sandbox) {
                    bail!("sandbox {sandbox} already exists");
                }
                self.sandboxes.insert(
                    sandbox,
                    Arc::new(SandboxState {
                        id: sandbox,
                        policy: RwLock::new(Arc::new(policy)),
                        names: Mutex::default(),
                    }),
                );
                Ok(Response::Done)
            }
            Request::UpdatePolicy { sandbox, policy } => {
                let state = self.sandbox(sandbox)?;
                state.modify_policy(|current| *current = policy);
                let policy = state.policy();
                let mut still_open = Vec::new();
                for execution in &self.executions {
                    if Arc::ptr_eq(&execution.sandbox, &state) {
                        still_open.extend(open_files_beyond(execution, &policy));
                        net::ensure_udp_ports(&self.shared, execution);
                    }
                }
                Ok(Response::PolicyUpdated { still_open })
            }
            Request::DestroySandbox { sandbox } => {
                self.sandboxes.remove(&sandbox);
                if let Ok(mut queues) = self.shared.queues.lock() {
                    queues.retain(|(owner, _), _| *owner != sandbox);
                }
                Ok(Response::Done)
            }
            Request::AttachExecution { sandbox } => {
                let state = self.sandbox(sandbox)?;
                if fds.len() != 1 {
                    bail!("attach request needs exactly one socket");
                }
                self.pending.push((fds.remove(0), state));
                Ok(Response::Done)
            }
        }
    }

    fn sandbox(&self, sandbox: SandboxId) -> Result<Arc<SandboxState>> {
        self.sandboxes
            .get(&sandbox)
            .cloned()
            .with_context(|| format!("unknown sandbox {sandbox}"))
    }
}

/// Files the execution's processes hold open with access `policy` no longer
/// grants.
fn open_files_beyond(execution: &Execution, policy: &WirePolicy) -> Vec<OpenFile> {
    let mut open_files = Vec::new();
    for process in execution.tree.members() {
        let Ok(entries) = std::fs::read_dir(format!("/proc/{}/fd", process.pid)) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(path) = std::fs::read_link(entry.path()) else {
                continue;
            };
            if !path.is_absolute() || path.is_dir() {
                continue;
            }
            let fdinfo = format!(
                "/proc/{}/fdinfo/{}",
                process.pid,
                entry.file_name().to_string_lossy()
            );
            let Some(flags) = std::fs::read_to_string(fdinfo).ok().and_then(|info| {
                info.lines()
                    .find_map(|line| line.strip_prefix("flags:"))
                    .and_then(|flags| i32::from_str_radix(flags.trim(), 8).ok())
            }) else {
                continue;
            };
            if flags & libc::O_PATH != 0 {
                continue;
            }
            let access_mode = flags & libc::O_ACCMODE;
            let read = access_mode != libc::O_WRONLY
                && policy.fs_access(&path, Permission::Read) != WireAccess::Allow;
            let write = access_mode != libc::O_RDONLY
                && policy.fs_access(&path, Permission::Write) != WireAccess::Allow;
            if read || write {
                open_files.push(OpenFile {
                    pid: process.pid,
                    path,
                    read,
                    write,
                });
            }
        }
    }
    open_files
}

fn poll_fd(fd: RawFd) -> libc::pollfd {
    libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    }
}
