use std::thread::sleep;
use std::time::Duration;

use anyhow::Context as _;
use windows::Win32::Foundation::{RPC_E_CHANGED_MODE, VARIANT_FALSE};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, CLSCTX_LOCAL_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance,
    CoInitializeEx, IDispatch, IServiceProvider,
};
use windows::Win32::System::TaskScheduler::{
    IExecAction, IRegisteredTask, ITaskFolder, ITaskService, TASK_ACTION_EXEC,
    TASK_CREATE_OR_UPDATE, TASK_LOGON_INTERACTIVE_TOKEN, TASK_RUNLEVEL_LUA, TASK_STATE_RUNNING,
    TASK_STATE_UNKNOWN, TaskScheduler,
};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Shell::{
    CSIDL_DESKTOP, IShellBrowser, IShellDispatch2, IShellFolderViewDual, IShellWindows,
    SID_STopLevelBrowser, SVGIO_BACKGROUND, SWC_DESKTOP, SWFO_NEEDDISPATCH, ShellWindows,
};
use windows::core::{BSTR, Interface};

const TASK_NAME: &str = "ZedRemoteServerLauncher";

/// Spawns a process that outlives the current session, so that brief connection
/// interruptions don't immediately kill the server.
///
/// On Windows, processes spawned from SSH are killed when the session
/// closes, so we ask a more long-lived process to spawn it for us.
/// - Best way to do it is to schedule a task, so the Task Scheduler service
///   becomes the parent.
/// - Asking the desktop Explorer to `ShellExecute` makes Explorer the parent,
///   but needs a responsive interactive Explorer - unavailable when no one is
///   logged in or the machine is in Modern Standby, so it is only a fallback.
/// - We could also use WMI's `Win32_Process::Create`, but it could be
///   blocked by Defender's PsExec/WMI attack surface reduction rule
pub fn spawn_process_detached(file: &str, parameters: &str, directory: &str) -> anyhow::Result<()> {
    init_com()?;
    if let Err(task_error) = spawn_via_scheduled_task(file, parameters, directory) {
        log::warn!(
            "failed to spawn {file:?} via Task Scheduler, falling back to Explorer's ShellExecute: {task_error:#}"
        );
        shell_execute_from_explorer(file, parameters, directory)
            .context("spawning via Explorer's ShellExecute")?;
    }
    Ok(())
}

fn init_com() -> anyhow::Result<()> {
    let result = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    // RPC_E_CHANGED_MODE means COM is already initialized with a different
    // threading model, which is fine for our purposes.
    if result.is_err() && result != RPC_E_CHANGED_MODE {
        return Err(windows::core::Error::from_hresult(result)).context("initializing COM");
    }
    Ok(())
}

fn spawn_via_scheduled_task(file: &str, parameters: &str, directory: &str) -> anyhow::Result<()> {
    unsafe {
        let service: ITaskService = CoCreateInstance(&TaskScheduler, None, CLSCTX_INPROC_SERVER)
            .context("creating TaskScheduler service")?;
        service.Connect(
            &VARIANT::default(),
            &VARIANT::default(),
            &VARIANT::default(),
            &VARIANT::default(),
        )?;
        let root: ITaskFolder = service.GetFolder(&BSTR::from("\\"))?;

        // Clear any triggerless task leaked by a run that died before its own cleanup.
        let _ = root.DeleteTask(&BSTR::from(TASK_NAME), 0);

        let task = service.NewTask(0)?;
        let principal = task.Principal()?;
        principal.SetLogonType(TASK_LOGON_INTERACTIVE_TOKEN)?; // current user
        principal.SetRunLevel(TASK_RUNLEVEL_LUA)?; // unelevated

        // Battery policy must not block or later stop the launch.
        let settings = task.Settings()?;
        settings.SetDisallowStartIfOnBatteries(VARIANT_FALSE)?;
        settings.SetStopIfGoingOnBatteries(VARIANT_FALSE)?;
        settings.SetExecutionTimeLimit(&BSTR::from("PT0S"))?;

        let exec: IExecAction = task.Actions()?.Create(TASK_ACTION_EXEC)?.cast()?;
        exec.SetPath(&BSTR::from(file))?;
        if !parameters.is_empty() {
            exec.SetArguments(&BSTR::from(parameters))?;
        }
        if !directory.is_empty() {
            exec.SetWorkingDirectory(&BSTR::from(directory))?;
        }

        let registered: IRegisteredTask = root
            .RegisterTaskDefinition(
                &BSTR::from(TASK_NAME),
                &task,
                TASK_CREATE_OR_UPDATE.0,
                &VARIANT::default(),
                &VARIANT::default(),
                TASK_LOGON_INTERACTIVE_TOKEN,
                &VARIANT::default(),
            )
            .context("registering scheduled task")?;
        registered.Run(&VARIANT::default()).context("running scheduled task")?;

        // Wait until the action process is actually running before deleting the
        // task (deleting right after launching is ok tho)
        let mut launched = false;
        for _ in 0..200 {
            if registered.State().unwrap_or(TASK_STATE_UNKNOWN) == TASK_STATE_RUNNING {
                launched = true;
                break;
            }
            sleep(Duration::from_millis(50));
        }

        let _ = root.DeleteTask(&BSTR::from(TASK_NAME), 0);
        anyhow::ensure!(launched, "scheduled task did not reach the Running state");
        Ok(())
    }
}

fn shell_execute_from_explorer(
    file: &str,
    parameters: &str,
    directory: &str,
) -> anyhow::Result<()> {
    unsafe {
        let mut _hwnd = Default::default();
        let shell_dispatch: IShellDispatch2 =
            CoCreateInstance::<_, IShellWindows>(&ShellWindows, None, CLSCTX_LOCAL_SERVER)?
                .FindWindowSW(
                    &VARIANT::from(CSIDL_DESKTOP as i32),
                    &VARIANT::default(),
                    SWC_DESKTOP,
                    &mut _hwnd,
                    SWFO_NEEDDISPATCH,
                )?
                .cast::<IServiceProvider>()?
                .QueryService::<IShellBrowser>(&SID_STopLevelBrowser)?
                .QueryActiveShellView()?
                .GetItemObject::<IDispatch>(SVGIO_BACKGROUND)?
                .cast::<IShellFolderViewDual>()?
                .Application()?
                .cast()?;

        shell_dispatch.ShellExecute(
            &BSTR::from(file),
            &VARIANT::from(parameters),
            &VARIANT::from(directory),
            &VARIANT::from(""),
            &VARIANT::from(0i32),
        )?;

        Ok(())
    }
}
