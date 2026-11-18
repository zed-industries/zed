//! The supervisor process (`zed --zed-sandbox-linux-supervisor`).
//!
//! One per [`SandboxContext`](crate::SandboxContext). The main thread polls:
//! - the control socket from Zed ([`CONTROL_FD`]): requests, prompt replies;
//! - pending attach sockets, each waiting for one helper's listener fd;
//! - seccomp listeners, one per running execution, for when it ends.
//!
//! Each execution's notifications are received and handled by its own
//! [`Receivers`].
//!
//! A notification can only arrive on the listener of the filter that trapped
//! it, so the listener identifies the sandbox; nothing the sandboxed process
//! sends can claim another identity.
//!
//! The process exits when Zed shuts down the control socket. Listeners close
//! with it, after which every trapped syscall fails with `ENOSYS`.

use std::collections::{HashMap, VecDeque};
use std::os::fd::{AsFd as _, AsRawFd as _, BorrowedFd, OwnedFd, RawFd};
use std::path::Path;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock, RwLock, mpsc};
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
/// Per execution.
const MAX_RECEIVERS: usize = 64;
/// Idle receivers beyond this many exit.
const SPARE_RECEIVERS: usize = 2;

// From <linux/seccomp.h>.
const SECCOMP_IOCTL_NOTIF_SET_FLAGS: libc::c_ulong = 0x4008_2104;
const SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP: libc::c_ulong = 1;

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
    /// The root of its mount namespace, once opened.
    pub root: OnceLock<OwnedFd>,
    receivers: Receivers,
}

/// The threads answering one execution's notifications, as leader and
/// followers: one thread at a time waits on the listener, and once it
/// receives a notification it leaves the waiting to another and handles the
/// notification itself. Answering on the receiving thread saves a hand-off
/// and lets the kernel wake it synchronously when the next notification
/// arrives. Handling can block (a prompt, a FIFO, a slow filesystem), so a
/// thread is added whenever none is left to wait.
#[derive(Default)]
struct Receivers {
    leader: Mutex<()>,
    /// Threads not handling a notification.
    idle: AtomicUsize,
    threads: AtomicUsize,
}

fn start_receiver(shared: &Arc<Shared>, execution: &Arc<Execution>) {
    let receivers = &execution.receivers;
    if receivers.threads.fetch_add(1, Ordering::SeqCst) >= MAX_RECEIVERS {
        receivers.threads.fetch_sub(1, Ordering::SeqCst);
        return;
    }
    receivers.idle.fetch_add(1, Ordering::SeqCst);
    let spawned = std::thread::Builder::new()
        .name("zed-sandbox-receiver".to_string())
        .spawn({
            let shared = shared.clone();
            let execution = execution.clone();
            move || run_receiver(&shared, &execution)
        });
    if let Err(error) = spawned {
        receivers.threads.fetch_sub(1, Ordering::SeqCst);
        receivers.idle.fetch_sub(1, Ordering::SeqCst);
        eprintln!("zed sandbox supervisor: starting a receiver: {error}");
    }
}

fn run_receiver(shared: &Arc<Shared>, execution: &Arc<Execution>) {
    let receivers = &execution.receivers;
    let exit = || {
        receivers.idle.fetch_sub(1, Ordering::SeqCst);
        receivers.threads.fetch_sub(1, Ordering::SeqCst);
    };
    loop {
        let notification = {
            let _leader = net::lock(&receivers.leader);
            loop {
                let mut entry = poll_fd(execution.listener.as_raw_fd());
                // SAFETY: one valid pollfd.
                let ready = unsafe { libc::poll(&mut entry, 1, -1) };
                if ready < 0 {
                    if std::io::Error::last_os_error().raw_os_error() == Some(libc::EINTR) {
                        continue;
                    }
                    return exit();
                }
                if entry.revents & libc::POLLIN != 0 {
                    if let Some(notification) = broker::receive(execution.listener.as_fd()) {
                        break notification;
                    }
                } else if entry.revents & (libc::POLLHUP | libc::POLLERR | libc::POLLNVAL) != 0 {
                    // Every task under this filter has exited.
                    return exit();
                }
            }
        };
        if receivers.idle.fetch_sub(1, Ordering::SeqCst) == 1 {
            start_receiver(shared, execution);
        }
        broker::handle(shared, execution, notification);
        if receivers.idle.fetch_add(1, Ordering::SeqCst) >= SPARE_RECEIVERS {
            return exit();
        }
    }
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
    host_root: OwnedFd,
}

impl Shared {
    /// `/`, opened once.
    pub fn host_root(&self) -> BorrowedFd<'_> {
        self.host_root.as_fd()
    }

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
    let host_root = match sys::open(Path::new("/"), libc::O_PATH | libc::O_DIRECTORY) {
        Ok(root) => root,
        Err(error) => {
            eprintln!("zed sandbox supervisor: opening /: {error}");
            std::process::exit(1);
        }
    };
    let shared = Arc::new(Shared {
        control,
        prompts: Mutex::new(HashMap::new()),
        next_prompt: AtomicU64::new(0),
        queues: Mutex::new(HashMap::new()),
        host_root,
    });
    Supervisor {
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
            // Only for hang-ups: receivers take the notifications.
            poll_fds.extend(self.executions.iter().map(|execution| libc::pollfd {
                fd: execution.listener.as_raw_fd(),
                events: 0,
                revents: 0,
            }));
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
                if entry.revents & (libc::POLLHUP | libc::POLLERR | libc::POLLNVAL) != 0 {
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
                        // Wake the receiver that answers on the target's
                        // CPU right away (Linux 6.6). Older kernels refuse the
                        // flag, which only costs that optimization.
                        // SAFETY: SET_FLAGS takes the flags by value.
                        unsafe {
                            libc::ioctl(
                                listener.as_raw_fd(),
                                SECCOMP_IOCTL_NOTIF_SET_FLAGS as _,
                                SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP,
                            )
                        };
                        let execution = Arc::new(Execution {
                            listener,
                            sandbox,
                            tree: ProcessTree {
                                helper: info.helper_pid,
                            },
                            // From now on the helper serves socket requests
                            // on the attach socket.
                            network: Network::new(socket),
                            root: OnceLock::new(),
                            receivers: Receivers::default(),
                        });
                        start_receiver(&self.shared, &execution);
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
