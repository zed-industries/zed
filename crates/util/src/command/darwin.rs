use mach2::exception_types::{
    EXC_MASK_ALL, EXCEPTION_DEFAULT, exception_behavior_t, exception_mask_t,
};
use mach2::port::{MACH_PORT_NULL, mach_port_t};
use mach2::thread_status::{THREAD_STATE_NONE, thread_state_flavor_t};
use parking_lot::{Condvar, Mutex, MutexGuard};
use smol::Unblock;
use std::collections::BTreeMap;
use std::ffi::{CString, OsStr, OsString};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::FromRawFd;
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Output};
use std::ptr;
use std::sync::{Arc, OnceLock};
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Stdio {
    /// A new pipe should be arranged to connect the parent and child processes.
    #[default]
    Piped,
    /// The child inherits from the corresponding parent descriptor.
    Inherit,
    /// This stream will be ignored (redirected to `/dev/null`).
    Null,
}

impl Stdio {
    pub fn piped() -> Self {
        Self::Piped
    }

    pub fn inherit() -> Self {
        Self::Inherit
    }

    pub fn null() -> Self {
        Self::Null
    }
}

unsafe extern "C" {
    fn posix_spawnattr_setexceptionports_np(
        attr: *mut libc::posix_spawnattr_t,
        mask: exception_mask_t,
        new_port: mach_port_t,
        behavior: exception_behavior_t,
        new_flavor: thread_state_flavor_t,
    ) -> libc::c_int;

    fn posix_spawn_file_actions_addchdir_np(
        file_actions: *mut libc::posix_spawn_file_actions_t,
        path: *const libc::c_char,
    ) -> libc::c_int;

    fn posix_spawn_file_actions_addinherit_np(
        file_actions: *mut libc::posix_spawn_file_actions_t,
        filedes: libc::c_int,
    ) -> libc::c_int;

    static environ: *const *mut libc::c_char;
}

#[derive(Debug)]
pub struct Command {
    program: OsString,
    args: Vec<OsString>,
    envs: BTreeMap<OsString, Option<OsString>>,
    env_clear: bool,
    current_dir: Option<PathBuf>,
    stdin_cfg: Option<Stdio>,
    stdout_cfg: Option<Stdio>,
    stderr_cfg: Option<Stdio>,
    kill_on_drop: bool,
}

impl Command {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            program: program.as_ref().to_owned(),
            args: Vec::new(),
            envs: BTreeMap::new(),
            env_clear: false,
            current_dir: None,
            stdin_cfg: None,
            stdout_cfg: None,
            stderr_cfg: None,
            kill_on_drop: false,
        }
    }

    pub fn arg(&mut self, arg: impl AsRef<OsStr>) -> &mut Self {
        self.args.push(arg.as_ref().to_owned());
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.args
            .extend(args.into_iter().map(|a| a.as_ref().to_owned()));
        self
    }

    pub fn get_args(&self) -> impl Iterator<Item = &OsStr> {
        self.args.iter().map(|s| s.as_os_str())
    }

    pub fn env(&mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> &mut Self {
        self.envs
            .insert(key.as_ref().to_owned(), Some(val.as_ref().to_owned()));
        self
    }

    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        for (key, val) in vars {
            self.envs
                .insert(key.as_ref().to_owned(), Some(val.as_ref().to_owned()));
        }
        self
    }

    pub fn env_remove(&mut self, key: impl AsRef<OsStr>) -> &mut Self {
        let key = key.as_ref().to_owned();
        if self.env_clear {
            self.envs.remove(&key);
        } else {
            self.envs.insert(key, None);
        }
        self
    }

    pub fn env_clear(&mut self) -> &mut Self {
        self.env_clear = true;
        self.envs.clear();
        self
    }

    pub fn current_dir(&mut self, dir: impl AsRef<Path>) -> &mut Self {
        self.current_dir = Some(dir.as_ref().to_owned());
        self
    }

    pub fn stdin(&mut self, cfg: Stdio) -> &mut Self {
        self.stdin_cfg = Some(cfg);
        self
    }

    pub fn stdout(&mut self, cfg: Stdio) -> &mut Self {
        self.stdout_cfg = Some(cfg);
        self
    }

    pub fn stderr(&mut self, cfg: Stdio) -> &mut Self {
        self.stderr_cfg = Some(cfg);
        self
    }

    pub fn kill_on_drop(&mut self, kill_on_drop: bool) -> &mut Self {
        self.kill_on_drop = kill_on_drop;
        self
    }

    pub fn spawn(&mut self) -> io::Result<Child> {
        let current_dir = self
            .current_dir
            .as_deref()
            .unwrap_or_else(|| Path::new("."));

        // Optimization: if no environment modifications were requested, pass None
        // to spawn_posix so it uses the `environ` global directly, avoiding a
        // full copy of the environment. This matches std::process::Command behavior.
        let envs = if self.env_clear || !self.envs.is_empty() {
            let mut result = BTreeMap::<OsString, OsString>::new();
            if !self.env_clear {
                for (key, val) in std::env::vars_os() {
                    result.insert(key, val);
                }
            }
            for (key, maybe_val) in &self.envs {
                if let Some(val) = maybe_val {
                    result.insert(key.clone(), val.clone());
                } else {
                    result.remove(key);
                }
            }
            Some(result.into_iter().collect::<Vec<_>>())
        } else {
            None
        };

        spawn_posix_spawn(
            &self.program,
            &self.args,
            current_dir,
            envs.as_deref(),
            self.stdin_cfg.unwrap_or_default(),
            self.stdout_cfg.unwrap_or_default(),
            self.stderr_cfg.unwrap_or_default(),
            self.kill_on_drop,
        )
    }

    pub async fn output(&mut self) -> io::Result<Output> {
        self.stdin_cfg.get_or_insert(Stdio::null());
        self.stdout_cfg.get_or_insert(Stdio::piped());
        self.stderr_cfg.get_or_insert(Stdio::piped());

        let child = self.spawn()?;
        child.output().await
    }

    pub async fn status(&mut self) -> io::Result<ExitStatus> {
        let mut child = self.spawn()?;
        child.status().await
    }

    pub fn get_program(&self) -> &OsStr {
        self.program.as_os_str()
    }
}

#[derive(Debug)]
pub struct Child {
    pid: libc::pid_t,
    pub stdin: Option<Unblock<std::fs::File>>,
    pub stdout: Option<Unblock<std::fs::File>>,
    pub stderr: Option<Unblock<std::fs::File>>,
    kill_on_drop: bool,
    state: Arc<SharedChildState>,
}

#[derive(Debug)]
struct SharedChildState {
    state: Mutex<ChildState>,
    status_changed: Condvar,
}

#[derive(Debug, Default)]
struct ChildState {
    status: Option<ExitStatus>,
    wait_error: Option<WaitError>,
    wait_started: bool,
}

#[derive(Debug, Clone)]
struct WaitError {
    raw_os_error: Option<i32>,
    kind: io::ErrorKind,
    message: String,
}

impl WaitError {
    fn from_io(error: io::Error) -> Self {
        Self {
            raw_os_error: error.raw_os_error(),
            kind: error.kind(),
            message: error.to_string(),
        }
    }

    fn to_io_error(&self) -> io::Error {
        if let Some(raw_os_error) = self.raw_os_error {
            io::Error::from_raw_os_error(raw_os_error)
        } else {
            io::Error::new(self.kind, self.message.clone())
        }
    }
}

impl SharedChildState {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(ChildState::default()),
            status_changed: Condvar::new(),
        })
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        if cached_status(&self.state).is_some() {
            return;
        }

        if self.kill_on_drop {
            if let Err(error) = self.kill() {
                if !is_no_such_process_error(&error) {
                    log::debug!("failed to kill child process {} on drop: {error}", self.pid);
                }
            }
        }

        if let Err(error) = register_wait(self.pid, self.state.clone(), WaitPurpose::Reap) {
            log::debug!(
                "failed to register child process {} for reaping: {error}",
                self.pid
            );
        }
    }
}

impl Child {
    pub fn id(&self) -> u32 {
        self.pid as u32
    }

    pub fn kill(&mut self) -> io::Result<()> {
        if cached_status(&self.state).is_some() {
            return Ok(());
        }

        kill_pid(self.pid)
    }

    pub fn try_status(&mut self) -> io::Result<Option<ExitStatus>> {
        {
            let state = lock_child_state(&self.state);
            if let Some(status) = state.status {
                return Ok(Some(status));
            }
            if let Some(error) = &state.wait_error {
                return Err(error.to_io_error());
            }
            if state.wait_started {
                return Ok(None);
            }
        }

        match try_wait_for_pid(self.pid)? {
            Some(status) => {
                store_status(self.pid, &self.state, status);
                Ok(Some(status))
            }
            None => Ok(None),
        }
    }

    pub fn status(
        &mut self,
    ) -> impl std::future::Future<Output = io::Result<ExitStatus>> + Send + 'static {
        self.stdin.take();

        let state = self.state.clone();
        let pid = self.pid;

        async move {
            if let Err(error) = register_wait(pid, state.clone(), WaitPurpose::Status) {
                log::debug!("falling back to waitpid for child process {pid}: {error}");
                return smol::unblock(move || wait_for_pid_and_store_status(pid, state)).await;
            }

            if let Some(status) = cached_status(&state) {
                return Ok(status);
            }
            smol::unblock(move || wait_for_cached_status(state)).await
        }
    }

    pub async fn output(mut self) -> io::Result<Output> {
        use futures_lite::AsyncReadExt;

        self.stdin.take();

        let stdout = self.stdout.take();
        let stdout_future = async move {
            let mut data = Vec::new();
            if let Some(mut stdout) = stdout {
                stdout.read_to_end(&mut data).await?;
            }
            io::Result::Ok(data)
        };

        let stderr = self.stderr.take();
        let stderr_future = async move {
            let mut data = Vec::new();
            if let Some(mut stderr) = stderr {
                stderr.read_to_end(&mut data).await?;
            }
            io::Result::Ok(data)
        };

        let (stdout_data, stderr_data) =
            futures_lite::future::try_zip(stdout_future, stderr_future).await?;
        let status = self.status().await?;

        Ok(Output {
            status,
            stdout: stdout_data,
            stderr: stderr_data,
        })
    }
}

#[derive(Clone, Copy)]
enum WaitPurpose {
    Status,
    Reap,
}

fn kill_pid(pid: libc::pid_t) -> io::Result<()> {
    let result = unsafe { libc::kill(pid, libc::SIGKILL) };
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn lock_child_state(state: &SharedChildState) -> MutexGuard<'_, ChildState> {
    state.state.lock()
}

fn cached_status(state: &SharedChildState) -> Option<ExitStatus> {
    lock_child_state(state).status
}

fn store_status(pid: libc::pid_t, shared_state: &SharedChildState, status: ExitStatus) {
    {
        let mut state = lock_child_state(shared_state);
        state.status = Some(status);
        state.wait_error = None;
        state.wait_started = false;
    }
    shared_state.status_changed.notify_all();
    record_reaped_pid(pid, Some(status));
}

fn store_wait_error(shared_state: &SharedChildState, error: WaitError) {
    {
        let mut state = lock_child_state(shared_state);
        state.wait_error = Some(error);
        state.wait_started = false;
    }
    shared_state.status_changed.notify_all();
}

fn wait_for_cached_status(state: Arc<SharedChildState>) -> io::Result<ExitStatus> {
    let mut guard = lock_child_state(&state);
    loop {
        if let Some(status) = guard.status {
            return Ok(status);
        }
        if let Some(error) = &guard.wait_error {
            return Err(error.to_io_error());
        }
        state.status_changed.wait(&mut guard);
    }
}

fn register_wait(
    pid: libc::pid_t,
    state: Arc<SharedChildState>,
    purpose: WaitPurpose,
) -> io::Result<()> {
    {
        let state = lock_child_state(&state);
        if state.status.is_some() {
            return Ok(());
        }
        if let Some(error) = &state.wait_error {
            if matches!(purpose, WaitPurpose::Status) {
                return Err(error.to_io_error());
            }
        }
        if state.wait_started {
            return Ok(());
        }
    }

    match try_wait_for_pid(pid) {
        Ok(Some(status)) => return finish_wait(pid, &state, Ok(status), purpose),
        Ok(None) => {}
        Err(error) => return finish_wait(pid, &state, Err(error), purpose),
    }

    {
        let mut state = lock_child_state(&state);
        if state.status.is_some() || state.wait_started {
            return Ok(());
        }
        if let Some(error) = &state.wait_error {
            if matches!(purpose, WaitPurpose::Status) {
                return Err(error.to_io_error());
            }
        }
        if matches!(purpose, WaitPurpose::Reap) {
            state.wait_error = None;
        }
        state.wait_started = true;
    }

    let reaper = match global_process_reaper() {
        Ok(reaper) => reaper,
        Err(error) if matches!(purpose, WaitPurpose::Status) => return Err(error),
        Err(error) => return finish_wait(pid, &state, Err(error), purpose),
    };

    if matches!(purpose, WaitPurpose::Status) && reaper_forced_to_fail_for_test(pid) {
        return Err(io::Error::other("process reaper disabled for test"));
    }

    match reaper.register_and_try_wait(pid, state.clone(), purpose) {
        Ok(Some(status)) => finish_wait(pid, &state, Ok(status), purpose),
        Ok(None) => Ok(()),
        Err(error) if matches!(purpose, WaitPurpose::Status) => Err(error),
        Err(error) => finish_wait(pid, &state, Err(error), purpose),
    }
}

fn finish_wait(
    pid: libc::pid_t,
    state: &SharedChildState,
    result: io::Result<ExitStatus>,
    purpose: WaitPurpose,
) -> io::Result<()> {
    match result {
        Ok(status) => {
            store_status(pid, state, status);
            Ok(())
        }
        Err(_) if cached_status(state).is_some() => Ok(()),
        Err(error) if matches!(purpose, WaitPurpose::Reap) && is_no_child_error(&error) => {
            {
                let mut state = lock_child_state(state);
                state.wait_started = false;
            }
            state.status_changed.notify_all();
            record_reaped_pid(pid, None);
            Ok(())
        }
        Err(error) => {
            let wait_error = WaitError::from_io(error);
            if matches!(purpose, WaitPurpose::Reap) {
                log::debug!("failed to reap child process {pid}: {}", wait_error.message);
            }
            store_wait_error(state, wait_error.clone());
            record_reaped_pid(pid, None);
            if matches!(purpose, WaitPurpose::Status) {
                Err(wait_error.to_io_error())
            } else {
                Ok(())
            }
        }
    }
}

#[derive(Clone)]
struct ReaperEntry {
    state: Arc<SharedChildState>,
    purpose: WaitPurpose,
    needs_poll: bool,
}

struct ProcessReaper {
    kqueue: libc::c_int,
    children: Mutex<BTreeMap<libc::pid_t, ReaperEntry>>,
}

enum ProcessReaperEvent {
    ProcessExited(libc::pid_t),
    ScanChildren,
}

const REAPER_WAKE_IDENT: libc::uintptr_t = usize::MAX as libc::uintptr_t;
const REAPER_POLL_INTERVAL: libc::timespec = libc::timespec {
    tv_sec: 0,
    tv_nsec: 50_000_000,
};

impl ProcessReaper {
    fn new() -> io::Result<Arc<Self>> {
        let kqueue = unsafe { libc::kqueue() };
        if kqueue == -1 {
            return Err(io::Error::last_os_error());
        }

        if let Err(error) = add_reaper_wake_event(kqueue) {
            let close_result = unsafe { libc::close(kqueue) };
            if close_result == -1 {
                log::debug!(
                    "failed to close process reaper kqueue after wake registration failed: {}",
                    io::Error::last_os_error()
                );
            }
            return Err(error);
        }

        let reaper = Arc::new(Self {
            kqueue,
            children: Mutex::new(BTreeMap::new()),
        });

        let reaper_thread = reaper.clone();
        if let Err(error) = thread::Builder::new()
            .name("zed-process-reaper".to_string())
            .spawn(move || reaper_thread.run())
        {
            return Err(error);
        }

        Ok(reaper)
    }

    fn register_and_try_wait(
        &self,
        pid: libc::pid_t,
        state: Arc<SharedChildState>,
        purpose: WaitPurpose,
    ) -> io::Result<Option<ExitStatus>> {
        self.register_entry_and_try_wait(
            pid,
            ReaperEntry {
                state,
                purpose,
                needs_poll: false,
            },
        )
    }

    fn register_entry_and_try_wait(
        &self,
        pid: libc::pid_t,
        entry: ReaperEntry,
    ) -> io::Result<Option<ExitStatus>> {
        let mut children = lock_reaper_children(&self.children);
        if let Some(existing) = children.get_mut(&pid) {
            if matches!(entry.purpose, WaitPurpose::Status) {
                existing.purpose = WaitPurpose::Status;
            }
            return Ok(None);
        }

        children.insert(pid, entry);
        if let Err(error) = add_process_exit_event(self.kqueue, pid) {
            let wait_result = try_wait_for_pid(pid);
            return match wait_result {
                Ok(Some(status)) => {
                    children.remove(&pid);
                    Ok(Some(status))
                }
                Ok(None) if is_no_such_process_error(&error) => {
                    if let Some(entry) = children.get_mut(&pid) {
                        entry.needs_poll = true;
                    }
                    if let Err(error) = trigger_reaper_wake_event(self.kqueue) {
                        children.remove(&pid);
                        return Err(error);
                    }
                    Ok(None)
                }
                Ok(None) => {
                    children.remove(&pid);
                    Err(error)
                }
                Err(wait_error) => {
                    children.remove(&pid);
                    Err(wait_error)
                }
            };
        }

        if let Some(entry) = children.get_mut(&pid) {
            entry.needs_poll = false;
        }

        let result = try_wait_for_pid(pid);
        if !matches!(result, Ok(None)) {
            children.remove(&pid);
            self.delete_process_exit_event(pid);
        }

        result
    }

    fn run(&self) {
        loop {
            let poll_children = self.has_polling_children();
            match wait_for_process_exit_event(self.kqueue, poll_children) {
                Ok(ProcessReaperEvent::ProcessExited(pid)) => self.reap_pid(pid),
                Ok(ProcessReaperEvent::ScanChildren) => self.reap_polling_children(),
                Err(error) => {
                    log::debug!("failed to wait for child process exit event: {error}");
                    thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
    }

    fn has_polling_children(&self) -> bool {
        lock_reaper_children(&self.children)
            .values()
            .any(|entry| entry.needs_poll)
    }

    fn reap_polling_children(&self) {
        let pids = lock_reaper_children(&self.children)
            .iter()
            .filter_map(|(pid, entry)| entry.needs_poll.then_some(*pid))
            .collect::<Vec<_>>();

        for pid in pids {
            self.reap_pid(pid);
        }
    }

    fn reap_pid(&self, pid: libc::pid_t) {
        let entry = match lock_reaper_children(&self.children).remove(&pid) {
            Some(entry) => entry,
            None => return,
        };

        match try_wait_for_pid(pid) {
            Ok(Some(status)) => {
                store_status(pid, &entry.state, status);
            }
            Ok(None) => {
                let state = entry.state.clone();
                let purpose = entry.purpose;
                match self.register_entry_and_try_wait(pid, entry) {
                    Ok(Some(status)) => {
                        store_status(pid, &state, status);
                    }
                    Ok(None) => {}
                    Err(error) => {
                        if let Err(error) = finish_wait(pid, &state, Err(error), purpose) {
                            if !matches!(purpose, WaitPurpose::Reap) {
                                log::debug!(
                                    "failed to finish child process wait for {pid}: {error}"
                                );
                            }
                        }
                    }
                }
            }
            Err(error) => {
                if let Err(error) = finish_wait(pid, &entry.state, Err(error), entry.purpose) {
                    if !matches!(entry.purpose, WaitPurpose::Reap) {
                        log::debug!("failed to finish child process wait for {pid}: {error}");
                    }
                }
            }
        }
    }

    fn delete_process_exit_event(&self, pid: libc::pid_t) {
        if let Err(error) = delete_process_exit_event(self.kqueue, pid) {
            if !is_missing_kqueue_event_error(&error) {
                log::debug!("failed to unregister child process {pid} from reaper: {error}");
            }
        }
    }
}

impl Drop for ProcessReaper {
    fn drop(&mut self) {
        let result = unsafe { libc::close(self.kqueue) };
        if result == -1 {
            log::debug!(
                "failed to close process reaper kqueue: {}",
                io::Error::last_os_error()
            );
        }
    }
}

fn global_process_reaper() -> io::Result<Arc<ProcessReaper>> {
    static REAPER: OnceLock<Result<Arc<ProcessReaper>, WaitError>> = OnceLock::new();

    match REAPER.get_or_init(|| ProcessReaper::new().map_err(WaitError::from_io)) {
        Ok(reaper) => Ok(reaper.clone()),
        Err(error) => Err(error.to_io_error()),
    }
}

#[cfg(test)]
static FORCE_REAPER_FAILURE_PIDS: Mutex<Vec<libc::pid_t>> = Mutex::new(Vec::new());

#[cfg(test)]
fn reaper_forced_to_fail_for_test(pid: libc::pid_t) -> bool {
    FORCE_REAPER_FAILURE_PIDS.lock().contains(&pid)
}

#[cfg(not(test))]
fn reaper_forced_to_fail_for_test(_pid: libc::pid_t) -> bool {
    false
}

fn lock_reaper_children(
    children: &Mutex<BTreeMap<libc::pid_t, ReaperEntry>>,
) -> MutexGuard<'_, BTreeMap<libc::pid_t, ReaperEntry>> {
    children.lock()
}

fn add_reaper_wake_event(kqueue: libc::c_int) -> io::Result<()> {
    change_reaper_wake_event(kqueue, (libc::EV_ADD | libc::EV_CLEAR) as libc::c_ushort, 0)
}

fn trigger_reaper_wake_event(kqueue: libc::c_int) -> io::Result<()> {
    change_reaper_wake_event(kqueue, 0, libc::NOTE_TRIGGER)
}

fn change_reaper_wake_event(
    kqueue: libc::c_int,
    flags: libc::c_ushort,
    fflags: libc::c_uint,
) -> io::Result<()> {
    let event = libc::kevent {
        ident: REAPER_WAKE_IDENT,
        filter: libc::EVFILT_USER,
        flags,
        fflags,
        data: 0,
        udata: ptr::null_mut(),
    };

    loop {
        let result = unsafe { libc::kevent(kqueue, &event, 1, ptr::null_mut(), 0, ptr::null()) };
        if result == -1 {
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(error);
        }
        return Ok(());
    }
}

fn add_process_exit_event(kqueue: libc::c_int, pid: libc::pid_t) -> io::Result<()> {
    change_process_exit_event(
        kqueue,
        pid,
        (libc::EV_ADD | libc::EV_ENABLE | libc::EV_ONESHOT) as libc::c_ushort,
        libc::NOTE_EXIT,
    )
}

fn delete_process_exit_event(kqueue: libc::c_int, pid: libc::pid_t) -> io::Result<()> {
    change_process_exit_event(kqueue, pid, libc::EV_DELETE as libc::c_ushort, 0)
}

fn change_process_exit_event(
    kqueue: libc::c_int,
    pid: libc::pid_t,
    flags: libc::c_ushort,
    fflags: libc::c_uint,
) -> io::Result<()> {
    let event = libc::kevent {
        ident: pid as libc::uintptr_t,
        filter: libc::EVFILT_PROC,
        flags,
        fflags,
        data: 0,
        udata: ptr::null_mut(),
    };

    loop {
        let result = unsafe { libc::kevent(kqueue, &event, 1, ptr::null_mut(), 0, ptr::null()) };
        if result == -1 {
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(error);
        }
        return Ok(());
    }
}

fn wait_for_process_exit_event(
    kqueue: libc::c_int,
    poll_children: bool,
) -> io::Result<ProcessReaperEvent> {
    loop {
        let mut event = libc::kevent {
            ident: 0,
            filter: 0,
            flags: 0,
            fflags: 0,
            data: 0,
            udata: ptr::null_mut(),
        };
        let timeout = if poll_children {
            &REAPER_POLL_INTERVAL
        } else {
            ptr::null()
        };
        let result = unsafe { libc::kevent(kqueue, ptr::null(), 0, &mut event, 1, timeout) };

        if result == -1 {
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(error);
        }

        if result == 0 {
            return Ok(ProcessReaperEvent::ScanChildren);
        }

        if event.flags & libc::EV_ERROR as libc::c_ushort != 0 {
            if event.data == 0 {
                continue;
            }
            return Err(io::Error::from_raw_os_error(event.data as i32));
        }

        if event.filter == libc::EVFILT_USER && event.ident == REAPER_WAKE_IDENT {
            return Ok(ProcessReaperEvent::ScanChildren);
        }

        if event.filter == libc::EVFILT_PROC && event.fflags & libc::NOTE_EXIT != 0 {
            return Ok(ProcessReaperEvent::ProcessExited(
                event.ident as libc::pid_t,
            ));
        }
    }
}

fn wait_for_pid_and_store_status(
    pid: libc::pid_t,
    state: Arc<SharedChildState>,
) -> io::Result<ExitStatus> {
    if let Some(status) = cached_status(&state) {
        return Ok(status);
    }

    match wait_for_pid(pid) {
        Ok(status) => {
            store_status(pid, &state, status);
            Ok(status)
        }
        Err(error) => {
            if let Some(status) = cached_status(&state) {
                return Ok(status);
            }

            let wait_error = WaitError::from_io(error);
            store_wait_error(&state, wait_error.clone());
            record_reaped_pid(pid, None);
            Err(wait_error.to_io_error())
        }
    }
}

fn wait_for_pid(pid: libc::pid_t) -> io::Result<ExitStatus> {
    let mut status: libc::c_int = 0;
    loop {
        let result = unsafe { libc::waitpid(pid, &mut status, 0) };
        if result == -1 {
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(error);
        }

        return Ok(ExitStatus::from_raw(status));
    }
}

fn try_wait_for_pid(pid: libc::pid_t) -> io::Result<Option<ExitStatus>> {
    let mut status: libc::c_int = 0;
    loop {
        let result = unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) };

        if result == -1 {
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(error);
        } else if result == 0 {
            return Ok(None);
        } else {
            return Ok(Some(ExitStatus::from_raw(status)));
        }
    }
}

fn is_no_child_error(error: &io::Error) -> bool {
    error.raw_os_error() == Some(libc::ECHILD)
}

fn is_no_such_process_error(error: &io::Error) -> bool {
    error.raw_os_error() == Some(libc::ESRCH)
}

fn is_missing_kqueue_event_error(error: &io::Error) -> bool {
    matches!(error.raw_os_error(), Some(libc::ENOENT | libc::ESRCH))
}

#[cfg(test)]
static REAPED_PIDS: Mutex<Vec<(libc::pid_t, Option<ExitStatus>)>> = Mutex::new(Vec::new());

#[cfg(test)]
fn record_reaped_pid(pid: libc::pid_t, status: Option<ExitStatus>) {
    REAPED_PIDS.lock().push((pid, status));
}

#[cfg(not(test))]
fn record_reaped_pid(_pid: libc::pid_t, _status: Option<ExitStatus>) {}

fn spawn_posix_spawn(
    program: &OsStr,
    args: &[OsString],
    current_dir: &Path,
    envs: Option<&[(OsString, OsString)]>,
    stdin_cfg: Stdio,
    stdout_cfg: Stdio,
    stderr_cfg: Stdio,
    kill_on_drop: bool,
) -> io::Result<Child> {
    let program_cstr = CString::new(program.as_bytes()).map_err(|_| invalid_input_error())?;

    let current_dir_cstr =
        CString::new(current_dir.as_os_str().as_bytes()).map_err(|_| invalid_input_error())?;

    let mut argv_cstrs = vec![program_cstr.clone()];
    for arg in args {
        let cstr = CString::new(arg.as_bytes()).map_err(|_| invalid_input_error())?;
        argv_cstrs.push(cstr);
    }
    let mut argv_ptrs: Vec<*mut libc::c_char> = argv_cstrs
        .iter()
        .map(|s| s.as_ptr() as *mut libc::c_char)
        .collect();
    argv_ptrs.push(ptr::null_mut());

    let envp: Vec<CString> = if let Some(envs) = envs {
        envs.iter()
            .map(|(key, value)| {
                let mut env_str = key.as_bytes().to_vec();
                env_str.push(b'=');
                env_str.extend_from_slice(value.as_bytes());
                CString::new(env_str)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| invalid_input_error())?
    } else {
        Vec::new()
    };
    let mut envp_ptrs: Vec<*mut libc::c_char> = envp
        .iter()
        .map(|s| s.as_ptr() as *mut libc::c_char)
        .collect();
    envp_ptrs.push(ptr::null_mut());

    let (stdin_read, stdin_write) = match stdin_cfg {
        Stdio::Piped => {
            let (r, w) = create_pipe()?;
            (Some(r), Some(w))
        }
        Stdio::Null => {
            let fd = open_dev_null(libc::O_RDONLY)?;
            (Some(fd), None)
        }
        Stdio::Inherit => (None, None),
    };

    let (stdout_read, stdout_write) = match stdout_cfg {
        Stdio::Piped => {
            let (r, w) = create_pipe()?;
            (Some(r), Some(w))
        }
        Stdio::Null => {
            let fd = open_dev_null(libc::O_WRONLY)?;
            (None, Some(fd))
        }
        Stdio::Inherit => (None, None),
    };

    let (stderr_read, stderr_write) = match stderr_cfg {
        Stdio::Piped => {
            let (r, w) = create_pipe()?;
            (Some(r), Some(w))
        }
        Stdio::Null => {
            let fd = open_dev_null(libc::O_WRONLY)?;
            (None, Some(fd))
        }
        Stdio::Inherit => (None, None),
    };

    let mut attr: libc::posix_spawnattr_t = ptr::null_mut();
    let mut file_actions: libc::posix_spawn_file_actions_t = ptr::null_mut();

    unsafe {
        cvt_nz(libc::posix_spawnattr_init(&mut attr))?;
        cvt_nz(libc::posix_spawn_file_actions_init(&mut file_actions))?;

        cvt_nz(libc::posix_spawnattr_setflags(
            &mut attr,
            libc::POSIX_SPAWN_CLOEXEC_DEFAULT as libc::c_short,
        ))?;

        cvt_nz(posix_spawnattr_setexceptionports_np(
            &mut attr,
            EXC_MASK_ALL,
            MACH_PORT_NULL,
            EXCEPTION_DEFAULT as exception_behavior_t,
            THREAD_STATE_NONE,
        ))?;

        cvt_nz(posix_spawn_file_actions_addchdir_np(
            &mut file_actions,
            current_dir_cstr.as_ptr(),
        ))?;

        if let Some(fd) = stdin_read {
            cvt_nz(libc::posix_spawn_file_actions_adddup2(
                &mut file_actions,
                fd,
                libc::STDIN_FILENO,
            ))?;
            cvt_nz(posix_spawn_file_actions_addinherit_np(
                &mut file_actions,
                libc::STDIN_FILENO,
            ))?;
        }

        if let Some(fd) = stdout_write {
            cvt_nz(libc::posix_spawn_file_actions_adddup2(
                &mut file_actions,
                fd,
                libc::STDOUT_FILENO,
            ))?;
            cvt_nz(posix_spawn_file_actions_addinherit_np(
                &mut file_actions,
                libc::STDOUT_FILENO,
            ))?;
        }

        if let Some(fd) = stderr_write {
            cvt_nz(libc::posix_spawn_file_actions_adddup2(
                &mut file_actions,
                fd,
                libc::STDERR_FILENO,
            ))?;
            cvt_nz(posix_spawn_file_actions_addinherit_np(
                &mut file_actions,
                libc::STDERR_FILENO,
            ))?;
        }

        let mut pid: libc::pid_t = 0;

        let spawn_result = libc::posix_spawnp(
            &mut pid,
            program_cstr.as_ptr(),
            &file_actions,
            &attr,
            argv_ptrs.as_ptr(),
            if envs.is_some() {
                envp_ptrs.as_ptr()
            } else {
                environ
            },
        );

        libc::posix_spawnattr_destroy(&mut attr);
        libc::posix_spawn_file_actions_destroy(&mut file_actions);

        if let Some(fd) = stdin_read {
            libc::close(fd);
        }
        if let Some(fd) = stdout_write {
            libc::close(fd);
        }
        if let Some(fd) = stderr_write {
            libc::close(fd);
        }

        cvt_nz(spawn_result)?;

        Ok(Child {
            pid,
            stdin: stdin_write.map(|fd| Unblock::new(std::fs::File::from_raw_fd(fd))),
            stdout: stdout_read.map(|fd| Unblock::new(std::fs::File::from_raw_fd(fd))),
            stderr: stderr_read.map(|fd| Unblock::new(std::fs::File::from_raw_fd(fd))),
            kill_on_drop,
            state: SharedChildState::new(),
        })
    }
}

fn create_pipe() -> io::Result<(libc::c_int, libc::c_int)> {
    let mut fds: [libc::c_int; 2] = [0; 2];
    let result = unsafe { libc::pipe(fds.as_mut_ptr()) };
    if result == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok((fds[0], fds[1]))
}

fn open_dev_null(flags: libc::c_int) -> io::Result<libc::c_int> {
    let fd = unsafe { libc::open(c"/dev/null".as_ptr() as *const libc::c_char, flags) };
    if fd == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(fd)
}

/// Zero means `Ok()`, all other values are treated as raw OS errors. Does not look at `errno`.
/// Mirrored after Rust's std `cvt_nz` function.
fn cvt_nz(error: libc::c_int) -> io::Result<()> {
    if error == 0 {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(error))
    }
}

fn invalid_input_error() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        "invalid argument: path or argument contains null byte",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_lite::AsyncWriteExt;
    use std::time::{Duration, Instant};

    fn reap_checkpoint() -> usize {
        REAPED_PIDS.lock().len()
    }

    fn recorded_reap(pid: libc::pid_t, checkpoint: usize) -> Option<Option<ExitStatus>> {
        REAPED_PIDS
            .lock()
            .iter()
            .skip(checkpoint)
            .rev()
            .find_map(|(reaped_pid, status)| (*reaped_pid == pid).then_some(*status))
    }

    fn wait_for_recorded_reap(pid: libc::pid_t, checkpoint: usize) -> Option<ExitStatus> {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(status) = recorded_reap(pid, checkpoint) {
                return status;
            }
            assert!(
                Instant::now() < deadline,
                "timed out waiting for pid {pid} to be reaped"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn assert_no_child_to_wait_for(pid: libc::pid_t) {
        let mut status: libc::c_int = 0;
        let result = unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) };
        assert_eq!(result, -1);
        assert_eq!(
            io::Error::last_os_error().raw_os_error(),
            Some(libc::ECHILD)
        );
    }

    struct ForceReaperFailureForTest {
        pid: libc::pid_t,
    }

    impl ForceReaperFailureForTest {
        fn new(pid: libc::pid_t) -> Self {
            FORCE_REAPER_FAILURE_PIDS.lock().push(pid);
            Self { pid }
        }
    }

    impl Drop for ForceReaperFailureForTest {
        fn drop(&mut self) {
            FORCE_REAPER_FAILURE_PIDS
                .lock()
                .retain(|pid| *pid != self.pid);
        }
    }

    #[test]
    fn test_spawn_echo() {
        smol::block_on(async {
            let output = Command::new("/bin/echo")
                .args(["-n", "hello world"])
                .output()
                .await
                .expect("failed to run command");

            assert!(output.status.success());
            assert_eq!(output.stdout, b"hello world");
        });
    }

    #[test]
    fn test_spawn_cat_stdin() {
        smol::block_on(async {
            let mut child = Command::new("/bin/cat")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to spawn");

            if let Some(ref mut stdin) = child.stdin {
                stdin
                    .write_all(b"hello from stdin")
                    .await
                    .expect("failed to write");
                stdin.close().await.expect("failed to close");
            }
            drop(child.stdin.take());

            let output = child.output().await.expect("failed to get output");
            assert!(output.status.success());
            assert_eq!(output.stdout, b"hello from stdin");
        });
    }

    #[test]
    fn test_spawn_stderr() {
        smol::block_on(async {
            let output = Command::new("/bin/sh")
                .args(["-c", "echo error >&2"])
                .output()
                .await
                .expect("failed to run command");

            assert!(output.status.success());
            assert_eq!(output.stderr, b"error\n");
        });
    }

    #[test]
    fn test_spawn_exit_code() {
        smol::block_on(async {
            let output = Command::new("/bin/sh")
                .args(["-c", "exit 42"])
                .output()
                .await
                .expect("failed to run command");

            assert!(!output.status.success());
            assert_eq!(output.status.code(), Some(42));
        });
    }

    #[test]
    fn test_spawn_current_dir() {
        smol::block_on(async {
            let output = Command::new("/bin/pwd")
                .current_dir("/tmp")
                .output()
                .await
                .expect("failed to run command");

            assert!(output.status.success());
            let pwd = String::from_utf8_lossy(&output.stdout);
            assert!(pwd.trim() == "/tmp" || pwd.trim() == "/private/tmp");
        });
    }

    #[test]
    fn test_spawn_env() {
        smol::block_on(async {
            let output = Command::new("/bin/sh")
                .args(["-c", "echo $MY_TEST_VAR"])
                .env("MY_TEST_VAR", "test_value")
                .output()
                .await
                .expect("failed to run command");

            assert!(output.status.success());
            assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "test_value");
        });
    }

    #[test]
    fn test_spawn_status() {
        smol::block_on(async {
            let status = Command::new("/usr/bin/true")
                .status()
                .await
                .expect("failed to run command");

            assert!(status.success());

            let status = Command::new("/usr/bin/false")
                .status()
                .await
                .expect("failed to run command");

            assert!(!status.success());
        });
    }

    #[test]
    fn test_drop_reaps_child() {
        let checkpoint = reap_checkpoint();
        let pid = {
            let child = Command::new("/bin/sh")
                .args(["-c", "exit 7"])
                .spawn()
                .expect("failed to spawn");
            child.id() as libc::pid_t
        };

        let status = wait_for_recorded_reap(pid, checkpoint).expect("missing exit status");
        assert_eq!(status.code(), Some(7));
        assert_no_child_to_wait_for(pid);
    }

    #[test]
    fn test_kill_on_drop_kills_and_reaps_child() {
        let checkpoint = reap_checkpoint();
        let pid = {
            let mut command = Command::new("/bin/sh");
            let child = command
                .args(["-c", "sleep 10"])
                .kill_on_drop(true)
                .spawn()
                .expect("failed to spawn");
            child.id() as libc::pid_t
        };

        let status = wait_for_recorded_reap(pid, checkpoint).expect("missing exit status");
        assert_eq!(status.signal(), Some(libc::SIGKILL));
        assert_no_child_to_wait_for(pid);
    }

    #[test]
    fn test_drop_reaper_does_not_block_on_running_child() {
        let checkpoint = reap_checkpoint();
        let long_running_pid = {
            let child = Command::new("/bin/sh")
                .args(["-c", "sleep 1"])
                .spawn()
                .expect("failed to spawn");
            child.id() as libc::pid_t
        };
        let short_lived_pid = {
            let child = Command::new("/bin/sh")
                .args(["-c", "exit 9"])
                .spawn()
                .expect("failed to spawn");
            child.id() as libc::pid_t
        };

        let status =
            wait_for_recorded_reap(short_lived_pid, checkpoint).expect("missing exit status");
        assert_eq!(status.code(), Some(9));
        assert_no_child_to_wait_for(short_lived_pid);

        let status =
            wait_for_recorded_reap(long_running_pid, checkpoint).expect("missing exit status");
        assert!(status.success());
        assert_no_child_to_wait_for(long_running_pid);
    }

    #[test]
    fn test_status_is_cached_before_kill() {
        smol::block_on(async {
            let mut child = Command::new("/bin/sh")
                .args(["-c", "exit 3"])
                .spawn()
                .expect("failed to spawn");

            let status = child.status().await.expect("failed to wait for status");
            assert_eq!(status.code(), Some(3));
            child.kill().expect("kill should be a no-op after status");

            let status = child
                .try_status()
                .expect("failed to read cached status")
                .expect("missing cached status");
            assert_eq!(status.code(), Some(3));
        });
    }

    #[test]
    fn test_status_falls_back_to_waitpid_when_reaper_fails() {
        smol::block_on(async {
            let mut child = Command::new("/bin/sh")
                .args(["-c", "exit 11"])
                .spawn()
                .expect("failed to spawn");
            let pid = child.id() as libc::pid_t;
            let _force_reaper_failure = ForceReaperFailureForTest::new(pid);

            let status = child.status().await.expect("failed to wait for status");
            assert_eq!(status.code(), Some(11));
            assert_no_child_to_wait_for(pid);
        });
    }

    #[test]
    fn test_env_remove_removes_set_env() {
        smol::block_on(async {
            let output = Command::new("/bin/sh")
                .args(["-c", "echo ${MY_VAR:-unset}"])
                .env("MY_VAR", "set_value")
                .env_remove("MY_VAR")
                .output()
                .await
                .expect("failed to run command");

            assert!(output.status.success());
            assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "unset");
        });
    }

    #[test]
    fn test_env_remove_removes_inherited_env() {
        smol::block_on(async {
            // SAFETY: This test is single-threaded and we clean up the var at the end
            unsafe { std::env::set_var("TEST_INHERITED_VAR", "inherited_value") };

            let output = Command::new("/bin/sh")
                .args(["-c", "echo ${TEST_INHERITED_VAR:-unset}"])
                .env_remove("TEST_INHERITED_VAR")
                .output()
                .await
                .expect("failed to run command");

            assert!(output.status.success());
            assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "unset");

            // SAFETY: Cleaning up test env var
            unsafe { std::env::remove_var("TEST_INHERITED_VAR") };
        });
    }

    #[test]
    fn test_env_after_env_remove() {
        smol::block_on(async {
            let output = Command::new("/bin/sh")
                .args(["-c", "echo ${MY_VAR:-unset}"])
                .env_remove("MY_VAR")
                .env("MY_VAR", "new_value")
                .output()
                .await
                .expect("failed to run command");

            assert!(output.status.success());
            assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "new_value");
        });
    }

    #[test]
    fn test_env_remove_after_env_clear() {
        smol::block_on(async {
            let output = Command::new("/bin/sh")
                .args(["-c", "echo ${MY_VAR:-unset}"])
                .env_clear()
                .env("MY_VAR", "set_value")
                .env_remove("MY_VAR")
                .output()
                .await
                .expect("failed to run command");

            assert!(output.status.success());
            assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "unset");
        });
    }

    #[test]
    fn test_stdio_null_stdin() {
        smol::block_on(async {
            let child = Command::new("/bin/cat")
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to spawn");

            let output = child.output().await.expect("failed to get output");
            assert!(output.status.success());
            assert!(
                output.stdout.is_empty(),
                "stdin from /dev/null should produce no output from cat"
            );
        });
    }

    #[test]
    fn test_stdio_null_stdout() {
        smol::block_on(async {
            let mut child = Command::new("/bin/echo")
                .args(["hello"])
                .stdout(Stdio::null())
                .spawn()
                .expect("failed to spawn");

            assert!(
                child.stdout.is_none(),
                "stdout should be None when Stdio::null() is used"
            );

            let status = child.status().await.expect("failed to get status");
            assert!(status.success());
        });
    }

    #[test]
    fn test_stdio_null_stderr() {
        smol::block_on(async {
            let mut child = Command::new("/bin/sh")
                .args(["-c", "echo error >&2"])
                .stderr(Stdio::null())
                .spawn()
                .expect("failed to spawn");

            assert!(
                child.stderr.is_none(),
                "stderr should be None when Stdio::null() is used"
            );

            let status = child.status().await.expect("failed to get status");
            assert!(status.success());
        });
    }

    #[test]
    fn test_stdio_piped_stdin() {
        smol::block_on(async {
            let mut child = Command::new("/bin/cat")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to spawn");

            assert!(
                child.stdin.is_some(),
                "stdin should be Some when Stdio::piped() is used"
            );

            if let Some(ref mut stdin) = child.stdin {
                stdin
                    .write_all(b"piped input")
                    .await
                    .expect("failed to write");
                stdin.close().await.expect("failed to close");
            }
            drop(child.stdin.take());

            let output = child.output().await.expect("failed to get output");
            assert!(output.status.success());
            assert_eq!(output.stdout, b"piped input");
        });
    }
}
