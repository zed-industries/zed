use std::{
    cell::{Cell, RefCell},
    ffi::{OsStr, OsString},
    os::windows::ffi::{OsStrExt as _, OsStringExt as _},
    path::{Path, PathBuf},
    rc::{Rc, Weak},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result, anyhow};
use futures::channel::oneshot::{self, Receiver};
use gpui_util::{ResultExt, get_powershell, new_std_command};
use itertools::Itertools;
use parking_lot::{Mutex, RwLock};
use smallvec::SmallVec;
use windows::{
    UI::ViewManagement::UISettings,
    Win32::{
        Foundation::*,
        Graphics::{Direct3D11::ID3D11Device, Gdi::*},
        Security::Credentials::*,
        System::{Com::*, LibraryLoader::*, Ole::*, Power::*, SystemInformation::*},
        UI::{Input::KeyboardAndMouse::*, Shell::*, WindowsAndMessaging::*},
    },
    core::*,
};

use crate::*;
use gpui::*;

pub struct WindowsPlatform {
    inner: Rc<WindowsPlatformInner>,
    raw_window_handles: Arc<RwLock<SmallVec<[SafeHwnd; 4]>>>,
    // The below members will never change throughout the entire lifecycle of the app.
    headless: bool,
    icon: HICON,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    direct_write_text_system: Option<Arc<DirectWriteTextSystem>>,
    drop_target_helper: Option<IDropTargetHelper>,
    /// Flag to instruct the `VSyncProvider` thread to invalidate the directx devices
    /// as resizing them has failed, causing us to have lost at least the render target.
    invalidate_devices: Arc<AtomicBool>,
    device_recovery: Arc<Mutex<SharedDeviceRecovery>>,
    handle: HWND,
    suspend_resume_notification: RefCell<Option<HPOWERNOTIFY>>,
    disable_direct_composition: bool,
    has_package_identity: bool,
    app_identity: RefCell<Option<(String, String)>>,
    system_notifications: RefCell<SystemNotificationState>,
}

#[derive(Default)]
struct SharedDeviceRecovery {
    generation: u64,
    last_generation: u64,
    new_windows: Vec<SafeHwnd>,
}

#[derive(Clone)]
pub(crate) enum WindowDeviceRecoveryAction {
    Suspend,
    Recover(DirectXDevices),
}

pub(crate) struct WindowDeviceRecoveryRequest {
    pub(crate) generation: u64,
    pub(crate) action: WindowDeviceRecoveryAction,
    pub(crate) outcome: Option<WindowDeviceRecoveryOutcome>,
}

#[derive(Debug)]
pub(crate) enum WindowDeviceRecoveryOutcome {
    Suspended,
    Active,
    Deferred,
    Stale,
    Destroyed,
    Failed(String),
}

struct GlobalDeviceRecoveryRequest {
    directx_devices: DirectXDevices,
    text_system: Arc<DirectWriteTextSystem>,
    gpu_state: Option<GPUState>,
    published: bool,
}

const DEVICE_RECOVERY_RETRY_DELAYS_MS: [u64; 8] = [0, 100, 250, 500, 1000, 2000, 4000, 8000];

#[cfg(any(debug_assertions, test))]
fn parse_injected_device_loss_vsyncs(value: &str) -> Vec<u64> {
    value
        .split(',')
        .filter_map(|value| value.trim().parse::<u64>().ok())
        .collect()
}

fn window_recovery_retry_delay(attempts: usize) -> Option<Duration> {
    DEVICE_RECOVERY_RETRY_DELAYS_MS
        .get(attempts)
        .copied()
        .map(Duration::from_millis)
}

fn take_device_invalidation(recovery_active: bool, invalidate_devices: &AtomicBool) -> bool {
    !recovery_active && invalidate_devices.fetch_and(false, Ordering::Acquire)
}

struct DeviceRecovery {
    generation: u64,
    started_at: Instant,
    phase: DeviceRecoveryPhase,
    windows: Vec<DeviceRecoveryWindow>,
}

enum DeviceRecoveryPhase {
    Suspending,
    RecreateGlobal {
        attempts: usize,
        next_attempt: Instant,
    },
    RecoverWindows {
        devices: DirectXDevices,
    },
}

struct DeviceRecoveryWindow {
    hwnd: SafeHwnd,
    suspended: bool,
    attempts: usize,
    next_attempt: Instant,
    outcome: DeviceRecoveryWindowOutcome,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DeviceRecoveryWindowOutcome {
    Pending,
    Active,
    Destroyed,
    Exhausted,
}

impl DeviceRecoveryWindow {
    fn new(hwnd: SafeHwnd, suspended: bool, now: Instant) -> Self {
        Self {
            hwnd,
            suspended,
            attempts: 0,
            next_attempt: now,
            outcome: DeviceRecoveryWindowOutcome::Pending,
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(
            self.outcome,
            DeviceRecoveryWindowOutcome::Active
                | DeviceRecoveryWindowOutcome::Destroyed
                | DeviceRecoveryWindowOutcome::Exhausted
        )
    }

    fn next_attempt_number(&self) -> usize {
        self.attempts + 1
    }

    fn record_active(&mut self) {
        self.attempts += 1;
        self.outcome = DeviceRecoveryWindowOutcome::Active;
    }

    fn record_destroyed(&mut self) {
        self.outcome = DeviceRecoveryWindowOutcome::Destroyed;
    }

    fn record_deferred(&mut self, now: Instant) {
        self.next_attempt = now;
    }

    fn record_retryable_failure(&mut self, now: Instant) -> bool {
        self.attempts += 1;
        let Some(delay) = window_recovery_retry_delay(self.attempts) else {
            self.outcome = DeviceRecoveryWindowOutcome::Exhausted;
            return true;
        };
        self.next_attempt = now + delay;
        false
    }

    fn reset_for_recovery(&mut self, now: Instant) {
        self.attempts = 0;
        self.next_attempt = now;
    }
}

struct WindowsPlatformInner {
    state: WindowsPlatformState,
    raw_window_handles: std::sync::Weak<RwLock<SmallVec<[SafeHwnd; 4]>>>,
    // The below members will never change throughout the entire lifecycle of the app.
    validation_number: usize,
    main_receiver: PriorityQueueReceiver<RunnableVariant>,
    dispatcher: Arc<WindowsDispatcher>,
}

pub(crate) struct WindowsPlatformState {
    callbacks: PlatformCallbacks,
    menus: RefCell<Vec<OwnedMenu>>,
    jump_list: RefCell<JumpList>,
    // NOTE: standard cursor handles don't need to close.
    pub(crate) current_cursor: Cell<Option<HCURSOR>>,
    /// Shared with each window so `WM_SETCURSOR` can read it directly.
    pub(crate) cursor_visible: Arc<AtomicBool>,
    /// Shared with each window to coordinate draws across windows on the UI
    /// thread; see [`DrawCoordinator`].
    pub(crate) draw_coordinator: Rc<DrawCoordinator>,
    directx_devices: RefCell<Option<DirectXDevices>>,
}

#[derive(Default)]
struct PlatformCallbacks {
    open_urls: Cell<Option<Box<dyn FnMut(Vec<String>)>>>,
    quit: Cell<Option<Box<dyn FnMut() -> bool>>>,
    reopen: Cell<Option<Box<dyn FnMut()>>>,
    app_menu_action: Cell<Option<Box<dyn FnMut(&dyn Action)>>>,
    will_open_app_menu: Cell<Option<Box<dyn FnMut()>>>,
    validate_app_menu_command: Cell<Option<Box<dyn FnMut(&dyn Action) -> bool>>>,
    keyboard_layout_change: Cell<Option<Box<dyn FnMut()>>>,
    system_wake: Cell<Option<Box<dyn FnMut()>>>,
}

impl WindowsPlatformState {
    fn new(directx_devices: Option<DirectXDevices>) -> Self {
        let callbacks = PlatformCallbacks::default();
        let jump_list = JumpList::new();
        let current_cursor = load_cursor(CursorStyle::Arrow);

        Self {
            callbacks,
            jump_list: RefCell::new(jump_list),
            current_cursor: Cell::new(current_cursor),
            cursor_visible: Arc::new(AtomicBool::new(true)),
            draw_coordinator: Rc::new(DrawCoordinator::new()),
            directx_devices: RefCell::new(directx_devices),
            menus: RefCell::new(Vec::new()),
        }
    }
}

impl WindowsPlatform {
    pub fn new(headless: bool) -> Result<Self> {
        unsafe {
            OleInitialize(None).context("unable to initialize Windows OLE")?;
        }
        let (directx_devices, text_system, direct_write_text_system) = if !headless {
            let devices = DirectXDevices::new().context("Creating DirectX devices")?;
            let dw_text_system = Arc::new(
                DirectWriteTextSystem::new(&devices)
                    .context("Error creating DirectWriteTextSystem")?,
            );
            (
                Some(devices),
                dw_text_system.clone() as Arc<dyn PlatformTextSystem>,
                Some(dw_text_system),
            )
        } else {
            (
                None,
                Arc::new(gpui::NoopTextSystem::new()) as Arc<dyn PlatformTextSystem>,
                None,
            )
        };

        let (main_sender, main_receiver) = PriorityQueueReceiver::new();
        let validation_number = if usize::BITS == 64 {
            rand::random::<u64>() as usize
        } else {
            rand::random::<u32>() as usize
        };
        let raw_window_handles = Arc::new(RwLock::new(SmallVec::new()));

        register_platform_window_class();
        let mut context = PlatformWindowCreateContext {
            inner: None,
            raw_window_handles: Arc::downgrade(&raw_window_handles),
            validation_number,
            main_sender: Some(main_sender),
            main_receiver: Some(main_receiver),
            directx_devices,
            dispatcher: None,
        };
        let result = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE(0),
                PLATFORM_WINDOW_CLASS_NAME,
                None,
                WINDOW_STYLE(0),
                0,
                0,
                0,
                0,
                Some(HWND_MESSAGE),
                None,
                None,
                Some(&raw const context as *const _),
            )
        };
        let inner = context
            .inner
            .take()
            .context("CreateWindowExW did not run correctly")??;
        let dispatcher = context
            .dispatcher
            .take()
            .context("CreateWindowExW did not run correctly")?;
        let handle = result?;

        let disable_direct_composition = std::env::var(DISABLE_DIRECT_COMPOSITION)
            .is_ok_and(|value| value == "true" || value == "1");
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);

        let drop_target_helper: Option<IDropTargetHelper> = if !headless {
            Some(unsafe {
                CoCreateInstance(&CLSID_DragDropHelper, None, CLSCTX_INPROC_SERVER)
                    .context("Error creating drop target helper.")?
            })
        } else {
            None
        };
        let icon = if !headless {
            load_icon().unwrap_or_default()
        } else {
            HICON::default()
        };

        Ok(Self {
            inner,
            handle,
            raw_window_handles,
            headless,
            icon,
            background_executor,
            foreground_executor,
            text_system,
            direct_write_text_system,
            suspend_resume_notification: RefCell::new(None),
            disable_direct_composition,
            has_package_identity: has_package_identity(),
            drop_target_helper,
            invalidate_devices: Arc::new(AtomicBool::new(false)),
            device_recovery: Arc::new(Mutex::new(SharedDeviceRecovery::default())),
            app_identity: RefCell::new(None),
            system_notifications: RefCell::new(SystemNotificationState::new()),
        })
    }

    pub(crate) fn window_from_hwnd(&self, hwnd: HWND) -> Option<Rc<WindowsWindowInner>> {
        self.raw_window_handles
            .read()
            .iter()
            .find(|entry| entry.as_raw() == hwnd)
            .and_then(|hwnd| window_from_hwnd(hwnd.as_raw()))
    }

    #[inline]
    fn post_message(&self, message: u32, wparam: WPARAM, lparam: LPARAM) {
        self.raw_window_handles
            .read()
            .iter()
            .for_each(|handle| unsafe {
                PostMessageW(Some(handle.as_raw()), message, wparam, lparam).log_err();
            });
    }

    fn generate_creation_info(&self, recovery_generation: u64) -> WindowCreationInfo {
        WindowCreationInfo {
            icon: self.icon,
            executor: self.foreground_executor.clone(),
            current_cursor: self.inner.state.current_cursor.get(),
            cursor_visible: self.inner.state.cursor_visible.clone(),
            drop_target_helper: self.drop_target_helper.clone().unwrap(),
            validation_number: self.inner.validation_number,
            main_receiver: self.inner.main_receiver.clone(),
            platform_window_handle: self.handle,
            disable_direct_composition: self.disable_direct_composition,
            directx_devices: if recovery_generation == 0 {
                self.inner.state.directx_devices.borrow().clone()
            } else {
                None
            },
            recovery_generation,
            invalidate_devices: self.invalidate_devices.clone(),
            draw_coordinator: self.inner.state.draw_coordinator.clone(),
        }
    }

    fn set_dock_menus(&self, menus: Vec<MenuItem>) {
        let mut actions = Vec::new();
        menus.into_iter().for_each(|menu| {
            if let Some(dock_menu) = DockMenuItem::new(menu).log_err() {
                actions.push(dock_menu);
            }
        });
        self.inner.state.jump_list.borrow_mut().dock_menus = actions;
        let borrow = self.inner.state.jump_list.borrow();
        let dock_menus = borrow
            .dock_menus
            .iter()
            .map(|menu| (menu.name.clone(), menu.description.clone()))
            .collect::<Vec<_>>();
        let recent_workspaces = borrow.recent_workspaces.clone();
        self.background_executor
            .spawn(async move {
                update_jump_list(&recent_workspaces, &dock_menus).log_err();
            })
            .detach();
    }

    fn update_jump_list(
        &self,
        menus: Vec<MenuItem>,
        entries: Vec<SmallVec<[PathBuf; 2]>>,
    ) -> Task<Vec<SmallVec<[PathBuf; 2]>>> {
        let mut actions = Vec::new();
        menus.into_iter().for_each(|menu| {
            if let Some(dock_menu) = DockMenuItem::new(menu).log_err() {
                actions.push(dock_menu);
            }
        });
        let mut jump_list = self.inner.state.jump_list.borrow_mut();
        jump_list.dock_menus = actions;
        jump_list.recent_workspaces = entries.into();
        let dock_menus = jump_list
            .dock_menus
            .iter()
            .map(|menu| (menu.name.clone(), menu.description.clone()))
            .collect::<Vec<_>>();
        let recent_workspaces = jump_list.recent_workspaces.clone();
        self.background_executor.spawn(async move {
            update_jump_list(&recent_workspaces, &dock_menus)
                .log_err()
                .unwrap_or_default()
        })
    }

    fn find_current_active_window(&self) -> Option<HWND> {
        let active_window_hwnd = unsafe { GetActiveWindow() };
        if active_window_hwnd.is_invalid() {
            return None;
        }
        self.raw_window_handles
            .read()
            .iter()
            .find(|hwnd| hwnd.as_raw() == active_window_hwnd)
            .map(|hwnd| hwnd.as_raw())
    }

    fn begin_vsync_thread(&self) {
        let Some(directx_devices) = self.inner.state.directx_devices.borrow().clone() else {
            return;
        };
        let Some(direct_write_text_system) = &self.direct_write_text_system else {
            return;
        };
        let mut directx_device = directx_devices;
        let platform_window: SafeHwnd = self.handle.into();
        let validation_number = self.inner.validation_number;
        let all_windows = Arc::downgrade(&self.raw_window_handles);
        let text_system = Arc::downgrade(direct_write_text_system);
        let invalidate_devices = self.invalidate_devices.clone();
        let shared_recovery = self.device_recovery.clone();
        #[cfg(debug_assertions)]
        let injected_device_loss_vsyncs = std::env::var("GPUI_TEST_DEVICE_LOSS_AT_VSYNCS")
            .ok()
            .map(|value| parse_injected_device_loss_vsyncs(&value))
            .unwrap_or_default();

        std::thread::Builder::new()
            .name("VSyncProvider".to_owned())
            .spawn(move || {
                let vsync_provider = VSyncProvider::new();
                let mut recovery = None;
                #[cfg(debug_assertions)]
                let mut vsync_count = 0u64;
                loop {
                    vsync_provider.wait_for_vsync();
                    #[cfg(debug_assertions)]
                    let inject_device_loss = {
                        vsync_count += 1;
                        injected_device_loss_vsyncs.contains(&vsync_count)
                    };
                    #[cfg(not(debug_assertions))]
                    let inject_device_loss = false;
                    #[cfg(debug_assertions)]
                    if inject_device_loss {
                        log::warn!(
                            "injecting synthetic DirectX device-loss recovery at vsync {vsync_count}"
                        );
                    }
                    // Preserve resize failures raised during recovery. Once
                    // the active generation completes, the retained flag
                    // starts a follow-up generation rather than leaving a
                    // window suspended indefinitely.
                    let devices_invalidated =
                        take_device_invalidation(recovery.is_some(), &invalidate_devices);
                    if recovery.is_none()
                        && (check_device_lost(&directx_device.device)
                            || devices_invalidated
                            || inject_device_loss)
                    {
                        recovery = DeviceRecovery::start(
                            &shared_recovery,
                            &all_windows,
                            &directx_device,
                            Instant::now(),
                        );
                    }
                    if recovery.as_mut().is_some_and(|recovery| {
                        recovery.advance(
                            platform_window.as_raw(),
                            validation_number,
                            &all_windows,
                            &text_system,
                            &shared_recovery,
                            &mut directx_device,
                            Instant::now(),
                        )
                    }) {
                        recovery = None;
                    }
                    let Some(all_windows) = all_windows.upgrade() else {
                        break;
                    };
                    for hwnd in all_windows.read().iter() {
                        unsafe {
                            let _ = RedrawWindow(Some(hwnd.as_raw()), None, None, RDW_INVALIDATE);
                        }
                    }
                }
            })
            .unwrap();
    }
}

fn translate_accelerator(msg: &MSG) -> Option<()> {
    if msg.message != WM_KEYDOWN && msg.message != WM_SYSKEYDOWN {
        return None;
    }

    let result = unsafe {
        SendMessageW(
            msg.hwnd,
            WM_GPUI_KEYDOWN,
            Some(msg.wParam),
            Some(msg.lParam),
        )
    };
    (result.0 == 0).then_some(())
}

fn encode_restart_arguments(arguments: &[OsString]) -> OsString {
    // `Start-Process` accepts a single native command line, so quote each argument according to
    // the Windows argv parsing rules before passing the complete string through the environment.
    let mut encoded = Vec::new();

    for (index, argument) in arguments.iter().enumerate() {
        if index > 0 {
            encoded.push(b' ' as u16);
        }
        encoded.push(b'"' as u16);

        let mut backslash_count = 0;
        for code_unit in argument.encode_wide() {
            if code_unit == b'\\' as u16 {
                backslash_count += 1;
            } else {
                if code_unit == b'"' as u16 {
                    encoded.extend(std::iter::repeat_n(b'\\' as u16, backslash_count * 2 + 1));
                } else {
                    encoded.extend(std::iter::repeat_n(b'\\' as u16, backslash_count));
                }
                backslash_count = 0;
                encoded.push(code_unit);
            }
        }

        encoded.extend(std::iter::repeat_n(b'\\' as u16, backslash_count * 2));
        encoded.push(b'"' as u16);
    }

    OsString::from_wide(&encoded)
}

impl Platform for WindowsPlatform {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(
            WindowsKeyboardLayout::new()
                .log_err()
                .unwrap_or(WindowsKeyboardLayout::unknown()),
        )
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(WindowsKeyboardMapper::new())
    }

    fn on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>) {
        self.inner
            .state
            .callbacks
            .keyboard_layout_change
            .set(Some(callback));
    }

    fn on_thermal_state_change(&self, _callback: Box<dyn FnMut()>) {}

    fn thermal_state(&self) -> ThermalState {
        ThermalState::Nominal
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        on_finish_launching();
        if !self.headless {
            self.begin_vsync_thread();
        }

        let mut msg = MSG::default();
        unsafe {
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                if translate_accelerator(&msg).is_none() {
                    _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }

        self.inner.with_callback(
            |callbacks| &callbacks.quit,
            |callback| {
                callback();
            },
        );
    }

    fn quit(&self) {
        self.foreground_executor()
            .spawn(async { unsafe { PostQuitMessage(0) } })
            .detach();
    }

    fn restart(&self, binary_path: Option<PathBuf>, arguments: Vec<OsString>) {
        let pid = std::process::id();
        let Some(app_path) = binary_path.or(self.app_path().log_err()) else {
            return;
        };
        let script = r#"
            $pidToWaitFor = $env:ZED_RESTART_PID
            $exePath = $env:ZED_RESTART_EXECUTABLE
            $argumentList = $env:ZED_RESTART_ARGUMENTS

            [Environment]::SetEnvironmentVariable("ZED_RESTART_PID", $null)
            [Environment]::SetEnvironmentVariable("ZED_RESTART_EXECUTABLE", $null)
            [Environment]::SetEnvironmentVariable("ZED_RESTART_ARGUMENTS", $null)

            while ($true) {
                $process = Get-Process -Id $pidToWaitFor -ErrorAction SilentlyContinue
                if (-not $process) {
                    if ([string]::IsNullOrEmpty($argumentList)) {
                        Start-Process -FilePath $exePath
                    } else {
                        Start-Process -FilePath $exePath -ArgumentList $argumentList
                    }
                    break
                }
                Start-Sleep -Seconds 0.1
            }
            "#;

        // Defer spawning to the foreground executor so it runs after the
        // current `AppCell` borrow is released. On Windows, `Command::spawn()`
        // can pump the Win32 message loop (via `CreateProcessW`), which
        // re-enters message handling possibly resulting in another mutable
        // borrow of the `AppCell` ending up with a double borrow panic
        let Some(powershell) = get_powershell() else {
            log::error!("failed to restart: PowerShell is unavailable");
            return;
        };
        self.foreground_executor
            .spawn(async move {
                let mut command = new_std_command(powershell);
                let arguments = encode_restart_arguments(&arguments);
                command
                    .arg("-command")
                    .arg(script)
                    .env("ZED_RESTART_PID", pid.to_string())
                    .env("ZED_RESTART_EXECUTABLE", app_path)
                    .env("ZED_RESTART_ARGUMENTS", arguments);
                #[allow(
                    clippy::disallowed_methods,
                    reason = "We are restarting ourselves, using std command thus is fine"
                )]
                let restart_process = command.spawn();

                match restart_process {
                    Ok(_) => unsafe { PostQuitMessage(0) },
                    Err(e) => log::error!("failed to spawn restart script: {:?}", e),
                }
            })
            .detach();
    }

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    // todo(windows)
    fn hide_other_apps(&self) {
        unimplemented!()
    }

    // todo(windows)
    fn unhide_other_apps(&self) {
        unimplemented!()
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        WindowsDisplay::displays()
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        WindowsDisplay::primary_monitor().map(|display| Rc::new(display) as Rc<dyn PlatformDisplay>)
    }

    #[cfg(feature = "screen-capture")]
    fn is_screen_capture_supported(&self) -> bool {
        true
    }

    #[cfg(feature = "screen-capture")]
    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<Result<Vec<Rc<dyn ScreenCaptureSource>>>> {
        gpui::scap_screen_capture::scap_screen_sources(&self.foreground_executor)
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        let active_window_hwnd = unsafe { GetActiveWindow() };
        self.window_from_hwnd(active_window_hwnd)
            .map(|inner| inner.handle)
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        let mut device_recovery = self.device_recovery.lock();
        let recovery_generation = device_recovery.generation;
        let window = WindowsWindow::new(
            handle,
            options,
            self.generate_creation_info(recovery_generation),
        )?;
        let handle = window.get_raw_handle();
        self.raw_window_handles.write().push(handle.into());
        if recovery_generation != 0 {
            device_recovery.new_windows.push(handle.into());
        }

        Ok(Box::new(window))
    }

    fn window_appearance(&self) -> WindowAppearance {
        system_appearance().log_err().unwrap_or_default()
    }

    fn open_url(&self, url: &str) {
        if url.is_empty() {
            return;
        }
        let url_string = url.to_string();
        self.background_executor()
            .spawn(async move {
                open_target(&url_string)
                    .with_context(|| format!("Opening url: {}", url_string))
                    .log_err();
            })
            .detach();
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.inner.state.callbacks.open_urls.set(Some(callback));
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> Receiver<Result<Option<Vec<PathBuf>>>> {
        let (tx, rx) = oneshot::channel();
        let window = self.find_current_active_window();
        self.foreground_executor()
            .spawn(async move {
                let _ = tx.send(file_open_dialog(options, window));
            })
            .detach();

        rx
    }

    fn prompt_for_new_path(
        &self,
        directory: &Path,
        suggested_name: Option<&str>,
    ) -> Receiver<Result<Option<PathBuf>>> {
        let directory = directory.to_owned();
        let suggested_name = suggested_name.map(|s| s.to_owned());
        let (tx, rx) = oneshot::channel();
        let window = self.find_current_active_window();
        self.foreground_executor()
            .spawn(async move {
                let _ = tx.send(file_save_dialog(directory, suggested_name, window));
            })
            .detach();

        rx
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        // The FOS_PICKFOLDERS flag toggles between "only files" and "only folders".
        false
    }

    fn reveal_path(&self, path: &Path) {
        if path.as_os_str().is_empty() {
            return;
        }
        let path = path.to_path_buf();
        self.background_executor()
            .spawn(async move {
                open_target_in_explorer(&path)
                    .with_context(|| format!("Revealing path {} in explorer", path.display()))
                    .log_err();
            })
            .detach();
    }

    fn open_with_system(&self, path: &Path) {
        if path.as_os_str().is_empty() {
            return;
        }
        let path = path.to_path_buf();
        self.background_executor()
            .spawn(async move {
                open_target(&path)
                    .with_context(|| format!("Opening {} with system", path.display()))
                    .log_err();
            })
            .detach();
    }

    fn on_quit(&self, callback: Box<dyn FnMut() -> bool>) {
        self.inner.state.callbacks.quit.set(Some(callback));
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.inner.state.callbacks.reopen.set(Some(callback));
    }

    fn on_system_wake(&self, callback: Box<dyn FnMut()>) {
        self.inner.state.callbacks.system_wake.set(Some(callback));
        let mut notification = self.suspend_resume_notification.borrow_mut();
        if notification.is_none() {
            *notification = unsafe {
                // SAFETY: self.handle is the platform window receiving WM_POWERBROADCAST.
                RegisterSuspendResumeNotification(
                    HANDLE(self.handle.0),
                    DEVICE_NOTIFY_WINDOW_HANDLE,
                )
                .log_err()
            };
        }
    }

    fn set_app_identity(&self, identifier: &str, name: &str) {
        // If the process has package identity, it's automatally granted an AUMID by the system.
        if self.has_package_identity {
            return;
        }

        let identifier_utf16 = windows::core::HSTRING::from(identifier);
        // SAFETY: `identifier_utf16` outlives the call and is null-terminated.
        if let Err(error) = unsafe {
            windows::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID(
                windows::core::PCWSTR(identifier_utf16.as_ptr()),
            )
        } {
            log::warn!("failed to set the process AppUserModelID: {error}");
        }
        *self.app_identity.borrow_mut() = Some((identifier.to_string(), name.to_string()));
    }

    fn show_system_notification(&self, notification: gpui::SystemNotification) {
        let app_identity = self.app_identity.borrow().clone();
        self.system_notifications
            .borrow_mut()
            .show(
                self.has_package_identity,
                app_identity
                    .as_ref()
                    .map(|(identifier, name)| (identifier.as_str(), name.as_str())),
                notification,
            )
            .log_err();
    }

    fn dismiss_system_notification(&self, tag: &str) {
        self.system_notifications.borrow_mut().dismiss(tag);
    }

    fn on_system_notification_response(
        &self,
        callback: Box<dyn FnMut(gpui::SystemNotificationResponse)>,
    ) {
        self.system_notifications
            .borrow_mut()
            .on_response(&self.foreground_executor, callback);
    }

    fn set_menus(&self, menus: Vec<Menu>, _keymap: &Keymap) {
        *self.inner.state.menus.borrow_mut() = menus.into_iter().map(|menu| menu.owned()).collect();
    }

    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        Some(self.inner.state.menus.borrow().clone())
    }

    fn set_dock_menu(&self, menus: Vec<MenuItem>, _keymap: &Keymap) {
        self.set_dock_menus(menus);
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.inner
            .state
            .callbacks
            .app_menu_action
            .set(Some(callback));
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.inner
            .state
            .callbacks
            .will_open_app_menu
            .set(Some(callback));
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.inner
            .state
            .callbacks
            .validate_app_menu_command
            .set(Some(callback));
    }

    fn app_path(&self) -> Result<PathBuf> {
        Ok(std::env::current_exe()?)
    }

    // todo(windows)
    fn path_for_auxiliary_executable(&self, _name: &str) -> Result<PathBuf> {
        anyhow::bail!("not yet implemented");
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        let hcursor = load_cursor(style);
        if self.inner.state.current_cursor.get().map(|c| c.0) != hcursor.map(|c| c.0) {
            self.post_message(
                WM_GPUI_CURSOR_STYLE_CHANGED,
                WPARAM(0),
                LPARAM(hcursor.map_or(0, |c| c.0 as isize)),
            );
            self.inner.state.current_cursor.set(hcursor);
        }
    }

    fn hide_cursor_until_mouse_moves(&self) {
        if !self
            .inner
            .state
            .cursor_visible
            .swap(false, Ordering::Relaxed)
        {
            return;
        }

        for handle in self.raw_window_handles.read().iter() {
            let Some(window) = window_from_hwnd(handle.as_raw()) else {
                continue;
            };
            if window.state.hovered.get() {
                unsafe { SetCursor(None) };
                break;
            }
        }
    }

    fn is_cursor_visible(&self) -> bool {
        self.inner.state.cursor_visible.load(Ordering::Relaxed)
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        should_auto_hide_scrollbars().log_err().unwrap_or(false)
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        write_to_clipboard(item);
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        read_from_clipboard()
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        // CredWriteW rejects larger blobs with the opaque RPC error
        // 0x800706F7 "The stub received bad data", so fail with a clear
        // message instead.
        if password.len() > CRED_MAX_CREDENTIAL_BLOB_SIZE as usize {
            return Task::ready(Err(anyhow!(
                "credential for {url} is {} bytes, which exceeds the Windows Credential Manager limit of {CRED_MAX_CREDENTIAL_BLOB_SIZE} bytes",
                password.len()
            )));
        }
        let password = password.to_vec();
        let mut username = username.encode_utf16().chain(Some(0)).collect_vec();
        let mut target_name = windows_credentials_target_name(url)
            .encode_utf16()
            .chain(Some(0))
            .collect_vec();
        self.foreground_executor().spawn(async move {
            let credentials = CREDENTIALW {
                LastWritten: unsafe { GetSystemTimeAsFileTime() },
                Flags: CRED_FLAGS(0),
                Type: CRED_TYPE_GENERIC,
                TargetName: PWSTR::from_raw(target_name.as_mut_ptr()),
                CredentialBlobSize: password.len() as u32,
                CredentialBlob: password.as_ptr() as *mut _,
                Persist: CRED_PERSIST_LOCAL_MACHINE,
                UserName: PWSTR::from_raw(username.as_mut_ptr()),
                ..CREDENTIALW::default()
            };
            unsafe {
                CredWriteW(&credentials, 0).map_err(|err| {
                    anyhow!(
                        "Failed to write credentials to Windows Credential Manager: {}",
                        err,
                    )
                })?;
            }
            Ok(())
        })
    }

    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        let target_name = windows_credentials_target_name(url)
            .encode_utf16()
            .chain(Some(0))
            .collect_vec();
        self.foreground_executor().spawn(async move {
            let mut credentials: *mut CREDENTIALW = std::ptr::null_mut();
            let result = unsafe {
                CredReadW(
                    PCWSTR::from_raw(target_name.as_ptr()),
                    CRED_TYPE_GENERIC,
                    None,
                    &mut credentials,
                )
            };

            if let Err(err) = result {
                // ERROR_NOT_FOUND means the credential doesn't exist.
                // Return Ok(None) to match macOS and Linux behavior.
                if err.code() == ERROR_NOT_FOUND.to_hresult() {
                    return Ok(None);
                }
                return Err(err.into());
            }

            if credentials.is_null() {
                Ok(None)
            } else {
                let username: String = unsafe { (*credentials).UserName.to_string()? };
                let credential_blob = unsafe {
                    std::slice::from_raw_parts(
                        (*credentials).CredentialBlob,
                        (*credentials).CredentialBlobSize as usize,
                    )
                };
                let password = credential_blob.to_vec();
                unsafe { CredFree(credentials as *const _ as _) };
                Ok(Some((username, password)))
            }
        })
    }

    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        let target_name = windows_credentials_target_name(url)
            .encode_utf16()
            .chain(Some(0))
            .collect_vec();
        self.foreground_executor().spawn(async move {
            unsafe {
                CredDeleteW(
                    PCWSTR::from_raw(target_name.as_ptr()),
                    CRED_TYPE_GENERIC,
                    None,
                )?
            };
            Ok(())
        })
    }

    fn register_url_scheme(&self, _: &str) -> Task<anyhow::Result<()>> {
        Task::ready(Err(anyhow!("register_url_scheme unimplemented")))
    }

    fn perform_dock_menu_action(&self, action: usize) {
        unsafe {
            PostMessageW(
                Some(self.handle),
                WM_GPUI_DOCK_MENU_ACTION,
                WPARAM(self.inner.validation_number),
                LPARAM(action as isize),
            )
            .log_err();
        }
    }

    fn update_jump_list(
        &self,
        menus: Vec<MenuItem>,
        entries: Vec<SmallVec<[PathBuf; 2]>>,
    ) -> Task<Vec<SmallVec<[PathBuf; 2]>>> {
        self.update_jump_list(menus, entries)
    }
}

impl WindowsPlatformInner {
    fn new(context: &mut PlatformWindowCreateContext) -> Result<Rc<Self>> {
        let state = WindowsPlatformState::new(context.directx_devices.take());
        Ok(Rc::new(Self {
            state,
            raw_window_handles: context.raw_window_handles.clone(),
            dispatcher: context
                .dispatcher
                .as_ref()
                .context("missing dispatcher")?
                .clone(),
            validation_number: context.validation_number,
            main_receiver: context
                .main_receiver
                .take()
                .context("missing main receiver")?,
        }))
    }

    /// Calls `project` to project to the corresponding callback field, removes it from callbacks, calls `f` with the callback and then puts the callback back.
    fn with_callback<T>(
        &self,
        project: impl Fn(&PlatformCallbacks) -> &Cell<Option<T>>,
        f: impl FnOnce(&mut T),
    ) {
        let callback = project(&self.state.callbacks).take();
        if let Some(mut callback) = callback {
            f(&mut callback);
            project(&self.state.callbacks).set(Some(callback));
        }
    }

    fn handle_msg(
        self: &Rc<Self>,
        handle: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        let handled = match msg {
            WM_GPUI_CLOSE_ONE_WINDOW
            | WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD
            | WM_GPUI_DOCK_MENU_ACTION
            | WM_GPUI_KEYBOARD_LAYOUT_CHANGED
            | WM_GPUI_GPU_DEVICE_LOST
            | WM_GPUI_END_SESSION => self.handle_gpui_events(msg, wparam, lparam),
            WM_POWERBROADCAST => self.handle_power_broadcast(wparam),
            _ => None,
        };
        if let Some(result) = handled {
            LRESULT(result)
        } else {
            unsafe { DefWindowProcW(handle, msg, wparam, lparam) }
        }
    }

    fn handle_gpui_events(&self, message: u32, wparam: WPARAM, lparam: LPARAM) -> Option<isize> {
        if wparam.0 != self.validation_number {
            log::error!("Wrong validation number while processing message: {message}");
            return None;
        }
        match message {
            WM_GPUI_CLOSE_ONE_WINDOW => {
                self.close_one_window(HWND(lparam.0 as _));
                Some(0)
            }
            WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD => self.run_foreground_task(),
            WM_GPUI_DOCK_MENU_ACTION => self.handle_dock_action_event(lparam.0 as _),
            WM_GPUI_KEYBOARD_LAYOUT_CHANGED => self.handle_keyboard_layout_change(),
            WM_GPUI_GPU_DEVICE_LOST => self.publish_device_recovery(lparam),
            WM_GPUI_END_SESSION => self.handle_end_session(),
            _ => unreachable!(),
        }
    }

    fn handle_end_session(&self) -> Option<isize> {
        let mut shutdown_completed = false;
        self.with_callback(
            |callbacks| &callbacks.quit,
            |callback| shutdown_completed = callback(),
        );
        log::logger().flush();
        if shutdown_completed {
            std::process::exit(0);
        }

        // Shutdown couldn't run synchronously, since the AppCell is already borrowed.
        // Windows may terminate the application as soon as we return from this handler, but if we post a WM_QUIT message now,
        // we may get to gracefully shut down the app before we're terminated by the OS.
        unsafe { PostQuitMessage(0) };
        Some(0)
    }

    fn close_one_window(&self, target_window: HWND) -> bool {
        let Some(all_windows) = self.raw_window_handles.upgrade() else {
            log::error!("Failed to upgrade raw window handles");
            return false;
        };
        let mut lock = all_windows.write();
        let index = lock
            .iter()
            .position(|handle| handle.as_raw() == target_window)
            .unwrap();
        lock.remove(index);

        lock.is_empty()
    }

    #[inline]
    fn run_foreground_task(&self) -> Option<isize> {
        const MAIN_TASK_TIMEOUT: u128 = 10;

        let start = std::time::Instant::now();
        'tasks: loop {
            'timeout_loop: loop {
                if start.elapsed().as_millis() >= MAIN_TASK_TIMEOUT {
                    log::debug!("foreground task timeout reached");
                    // we spent our budget on gpui tasks, we likely have a lot of work queued so drain system events first to stay responsive
                    // then quit out of foreground work to allow us to process other gpui events first before returning back to foreground task work
                    // if we don't we might not for example process window quit events
                    let mut msg = MSG::default();
                    let process_message = |msg: &_| {
                        if translate_accelerator(msg).is_none() {
                            _ = unsafe { TranslateMessage(msg) };
                            unsafe { DispatchMessageW(msg) };
                        }
                    };
                    let peek_msg = |msg: &mut _, msg_kind| unsafe {
                        PeekMessageW(msg, None, 0, 0, PM_REMOVE | msg_kind).as_bool()
                    };
                    // We need to process a paint message here as otherwise we will re-enter `run_foreground_task` before painting if we have work remaining.
                    // The reason for this is that windows prefers custom application message processing over system messages.
                    if peek_msg(&mut msg, PM_QS_PAINT) {
                        process_message(&msg);
                    }
                    while peek_msg(&mut msg, PM_QS_INPUT) {
                        process_message(&msg);
                    }
                    // Allow the main loop to process other gpui events before going back into `run_foreground_task`
                    unsafe {
                        if let Err(_) = PostMessageW(
                            Some(self.dispatcher.platform_window_handle.as_raw()),
                            WM_GPUI_TASK_DISPATCHED_ON_MAIN_THREAD,
                            WPARAM(self.validation_number),
                            LPARAM(0),
                        ) {
                            self.dispatcher.wake_posted.store(false, Ordering::Release);
                        };
                    }
                    break 'tasks;
                }
                let mut main_receiver = self.main_receiver.clone();
                match main_receiver.try_pop() {
                    Ok(Some(runnable)) => WindowsDispatcher::execute_runnable(runnable),
                    _ => break 'timeout_loop,
                }
            }

            // Someone could enqueue a Runnable here. The flag is still true, so they will not PostMessage.
            // We need to check for those Runnables after we clear the flag.
            self.dispatcher.wake_posted.store(false, Ordering::Release);
            let mut main_receiver = self.main_receiver.clone();
            match main_receiver.try_pop() {
                Ok(Some(runnable)) => {
                    self.dispatcher.wake_posted.store(true, Ordering::Release);

                    WindowsDispatcher::execute_runnable(runnable);
                }
                _ => break 'tasks,
            }
        }

        Some(0)
    }

    fn handle_dock_action_event(&self, action_idx: usize) -> Option<isize> {
        let Some(action) = self
            .state
            .jump_list
            .borrow()
            .dock_menus
            .get(action_idx)
            .map(|dock_menu| dock_menu.action.boxed_clone())
        else {
            log::error!("Dock menu for index {action_idx} not found");
            return Some(1);
        };
        self.with_callback(
            |callbacks| &callbacks.app_menu_action,
            |callback| callback(&*action),
        );
        Some(0)
    }

    fn handle_keyboard_layout_change(&self) -> Option<isize> {
        self.with_callback(
            |callbacks| &callbacks.keyboard_layout_change,
            |callback| callback(),
        );
        Some(0)
    }

    fn handle_power_broadcast(&self, wparam: WPARAM) -> Option<isize> {
        if wparam.0 as u32 == PBT_APMRESUMEAUTOMATIC {
            self.with_callback(|callbacks| &callbacks.system_wake, |callback| callback());
        }
        Some(1)
    }

    fn publish_device_recovery(&self, lparam: LPARAM) -> Option<isize> {
        if lparam.0 == 0 {
            log::error!("gpui_device_loss invalid global recovery message");
            return Some(0);
        }
        let request = lparam.0 as *mut GlobalDeviceRecoveryRequest;
        // SAFETY: the platform coordinator sends this live stack request with
        // synchronous `SendMessageW`; the call cannot return until this
        // handler releases the borrow.
        let (directx_devices, text_system, gpu_state) = unsafe {
            let request = &mut *request;
            (
                request.directx_devices.clone(),
                request.text_system.clone(),
                request.gpu_state.take(),
            )
        };
        let Some(gpu_state) = gpu_state else {
            log::error!("gpui_device_loss missing DirectWrite recovery candidate");
            return Some(0);
        };

        text_system.commit_gpu_recovery(gpu_state);
        *self.state.directx_devices.borrow_mut() = Some(directx_devices);
        // SAFETY: this is the same live synchronous request validated above;
        // the sender reads `published` only after the handler returns.
        unsafe { (*request).published = true };
        Some(0)
    }
}

impl Drop for WindowsPlatform {
    fn drop(&mut self) {
        unsafe {
            if let Some(notification) = self.suspend_resume_notification.borrow_mut().take() {
                // SAFETY: notification was returned by RegisterSuspendResumeNotification.
                UnregisterSuspendResumeNotification(notification).log_err();
            }
            DestroyWindow(self.handle)
                .context("Destroying platform window")
                .log_err();
            OleUninitialize();
        }
    }
}

pub(crate) struct WindowCreationInfo {
    pub(crate) icon: HICON,
    pub(crate) executor: ForegroundExecutor,
    pub(crate) current_cursor: Option<HCURSOR>,
    pub(crate) cursor_visible: Arc<AtomicBool>,
    pub(crate) drop_target_helper: IDropTargetHelper,
    pub(crate) validation_number: usize,
    pub(crate) main_receiver: PriorityQueueReceiver<RunnableVariant>,
    pub(crate) platform_window_handle: HWND,
    pub(crate) disable_direct_composition: bool,
    pub(crate) directx_devices: Option<DirectXDevices>,
    pub(crate) recovery_generation: u64,
    /// Flag to instruct the `VSyncProvider` thread to invalidate the directx devices
    /// as resizing them has failed, causing us to have lost at least the render target.
    pub(crate) invalidate_devices: Arc<AtomicBool>,
    /// Shared with [`WindowsPlatformState::draw_coordinator`] and every other window.
    pub(crate) draw_coordinator: Rc<DrawCoordinator>,
}

struct PlatformWindowCreateContext {
    inner: Option<Result<Rc<WindowsPlatformInner>>>,
    raw_window_handles: std::sync::Weak<RwLock<SmallVec<[SafeHwnd; 4]>>>,
    validation_number: usize,
    main_sender: Option<PriorityQueueSender<RunnableVariant>>,
    main_receiver: Option<PriorityQueueReceiver<RunnableVariant>>,
    directx_devices: Option<DirectXDevices>,
    dispatcher: Option<Arc<WindowsDispatcher>>,
}

fn has_package_identity() -> bool {
    let mut package_full_name_length = 0;
    let result = unsafe {
        windows::Win32::Storage::Packaging::Appx::GetCurrentPackageFullName(
            &mut package_full_name_length,
            None,
        )
    };
    if result == ERROR_INSUFFICIENT_BUFFER {
        true
    } else if result == APPMODEL_ERROR_NO_PACKAGE {
        false
    } else {
        log::warn!("failed to determine whether the process has package identity: {result:?}");
        false
    }
}

fn open_target(target: impl AsRef<OsStr>) -> Result<()> {
    let target = target.as_ref();
    let ret = unsafe {
        ShellExecuteW(
            None,
            windows::core::w!("open"),
            &HSTRING::from(target),
            None,
            None,
            SW_SHOWDEFAULT,
        )
    };
    if ret.0 as isize <= 32 {
        Err(anyhow::anyhow!(
            "Unable to open target: {}",
            std::io::Error::last_os_error()
        ))
    } else {
        Ok(())
    }
}

fn open_target_in_explorer(target: &Path) -> Result<()> {
    let dir = target.parent().context("No parent folder found")?;
    let desktop = unsafe { SHGetDesktopFolder()? };

    let mut dir_item = std::ptr::null_mut();
    unsafe {
        desktop.ParseDisplayName(
            HWND::default(),
            None,
            &HSTRING::from(dir),
            None,
            &mut dir_item,
            std::ptr::null_mut(),
        )?;
    }

    let mut file_item = std::ptr::null_mut();
    unsafe {
        desktop.ParseDisplayName(
            HWND::default(),
            None,
            &HSTRING::from(target),
            None,
            &mut file_item,
            std::ptr::null_mut(),
        )?;
    }

    let highlight = [file_item as *const _];
    unsafe { SHOpenFolderAndSelectItems(dir_item as _, Some(&highlight), 0) }.or_else(|err| {
        if err.code().0 == ERROR_FILE_NOT_FOUND.0 as i32 {
            // On some systems, the above call mysteriously fails with "file not
            // found" even though the file is there.  In these cases, ShellExecute()
            // seems to work as a fallback (although it won't select the file).
            open_target(dir).context("Opening target parent folder")
        } else {
            Err(anyhow::anyhow!("Can not open target path: {}", err))
        }
    })
}

fn file_open_dialog(
    options: PathPromptOptions,
    window: Option<HWND>,
) -> Result<Option<Vec<PathBuf>>> {
    let folder_dialog: IFileOpenDialog =
        unsafe { CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL)? };

    let mut dialog_options = FOS_FILEMUSTEXIST;
    if options.multiple {
        dialog_options |= FOS_ALLOWMULTISELECT;
    }
    if options.directories {
        dialog_options |= FOS_PICKFOLDERS;
    }

    unsafe {
        folder_dialog.SetOptions(dialog_options)?;

        if let Some(prompt) = options.prompt {
            let prompt: &str = &prompt;
            folder_dialog.SetOkButtonLabel(&HSTRING::from(prompt))?;
        }

        if folder_dialog.Show(window).is_err() {
            // User cancelled
            return Ok(None);
        }
    }

    let results = unsafe { folder_dialog.GetResults()? };
    let file_count = unsafe { results.GetCount()? };
    if file_count == 0 {
        return Ok(None);
    }

    let mut paths = Vec::with_capacity(file_count as usize);
    for i in 0..file_count {
        let item = unsafe { results.GetItemAt(i)? };
        let path = unsafe { item.GetDisplayName(SIGDN_FILESYSPATH)?.to_string()? };
        paths.push(PathBuf::from(path));
    }

    Ok(Some(paths))
}

fn file_save_dialog(
    directory: PathBuf,
    suggested_name: Option<String>,
    window: Option<HWND>,
) -> Result<Option<PathBuf>> {
    let dialog: IFileSaveDialog = unsafe { CoCreateInstance(&FileSaveDialog, None, CLSCTX_ALL)? };
    if !directory.to_string_lossy().is_empty()
        && let Some(full_path) = directory
            .canonicalize()
            .context("failed to canonicalize directory")
            .log_err()
    {
        let full_path = dunce::simplified(&full_path);
        let full_path_string = full_path.display().to_string();
        let path_item: IShellItem =
            unsafe { SHCreateItemFromParsingName(&HSTRING::from(full_path_string), None)? };
        unsafe {
            dialog
                .SetFolder(&path_item)
                .context("failed to set dialog folder")
                .log_err()
        };
    }

    if let Some(suggested_name) = suggested_name {
        unsafe {
            dialog
                .SetFileName(&HSTRING::from(suggested_name))
                .context("failed to set file name")
                .log_err()
        };
    }

    unsafe {
        dialog.SetFileTypes(&[Common::COMDLG_FILTERSPEC {
            pszName: windows::core::w!("All files"),
            pszSpec: windows::core::w!("*.*"),
        }])?;
        if dialog.Show(window).is_err() {
            // User cancelled
            return Ok(None);
        }
    }
    let shell_item = unsafe { dialog.GetResult()? };
    let file_path_string = unsafe {
        let pwstr = shell_item.GetDisplayName(SIGDN_FILESYSPATH)?;
        let string = pwstr.to_string()?;
        CoTaskMemFree(Some(pwstr.0 as _));
        string
    };
    Ok(Some(PathBuf::from(file_path_string)))
}

fn load_icon() -> Result<HICON> {
    let module = unsafe { GetModuleHandleW(None).context("unable to get module handle")? };
    let handle = unsafe {
        LoadImageW(
            Some(module.into()),
            windows::core::PCWSTR(1 as _),
            IMAGE_ICON,
            0,
            0,
            LR_DEFAULTSIZE | LR_SHARED,
        )
        .context("unable to load icon file")?
    };
    Ok(HICON(handle.0))
}

#[inline]
fn should_auto_hide_scrollbars() -> Result<bool> {
    let ui_settings = UISettings::new()?;
    Ok(ui_settings.AutoHideScrollBars()?)
}

fn check_device_lost(device: &ID3D11Device) -> bool {
    let device_state = unsafe { device.GetDeviceRemovedReason() };
    match device_state {
        Ok(_) => false,
        Err(err) => {
            log::error!(
                "gpui_device_loss removed_hresult=0x{:08x} error={err:?}",
                err.code().0 as u32
            );
            true
        }
    }
}

fn adapter_diagnostics(devices: &DirectXDevices) -> String {
    // SAFETY: `adapter` is an owned COM interface and `GetDesc1` writes into
    // the result value managed by the Windows binding.
    let Ok(description) = (unsafe { devices.adapter.GetDesc1() }) else {
        return "adapter=unknown".to_owned();
    };
    let name = String::from_utf16_lossy(&description.Description)
        .trim_matches(char::from(0))
        .to_owned();
    format!(
        "adapter={name:?} vendor=0x{:04x} device=0x{:04x} luid={:08x}:{:08x}",
        description.VendorId,
        description.DeviceId,
        description.AdapterLuid.HighPart as u32,
        description.AdapterLuid.LowPart,
    )
}

impl DeviceRecovery {
    fn start(
        shared_recovery: &Arc<Mutex<SharedDeviceRecovery>>,
        all_windows: &std::sync::Weak<RwLock<SmallVec<[SafeHwnd; 4]>>>,
        current_devices: &DirectXDevices,
        now: Instant,
    ) -> Option<Self> {
        let all_windows = all_windows.upgrade()?;
        let mut shared = shared_recovery.lock();
        shared.last_generation = shared.last_generation.wrapping_add(1).max(1);
        shared.generation = shared.last_generation;
        shared.new_windows.clear();
        let generation = shared.generation;
        let windows = all_windows
            .read()
            .iter()
            .copied()
            .map(|hwnd| DeviceRecoveryWindow::new(hwnd, false, now))
            .collect::<Vec<_>>();
        drop(shared);

        log::error!(
            "gpui_device_loss generation={generation} detected windows={} {}",
            windows.len(),
            adapter_diagnostics(current_devices),
        );
        Some(Self {
            generation,
            started_at: now,
            phase: DeviceRecoveryPhase::Suspending,
            windows,
        })
    }

    fn advance(
        &mut self,
        platform_window: HWND,
        validation_number: usize,
        all_windows: &std::sync::Weak<RwLock<SmallVec<[SafeHwnd; 4]>>>,
        text_system: &std::sync::Weak<DirectWriteTextSystem>,
        shared_recovery: &Arc<Mutex<SharedDeviceRecovery>>,
        current_devices: &mut DirectXDevices,
        now: Instant,
    ) -> bool {
        self.add_new_windows(shared_recovery, now);
        let Some(all_windows) = all_windows.upgrade() else {
            self.finish(shared_recovery);
            return true;
        };
        let live_windows = all_windows
            .read()
            .iter()
            .map(|window| window.as_raw())
            .collect::<Vec<_>>();
        for window in &mut self.windows {
            if !live_windows.contains(&window.hwnd.as_raw()) {
                window.record_destroyed();
            }
        }
        drop(all_windows);

        if matches!(self.phase, DeviceRecoveryPhase::Suspending) {
            for window in &mut self.windows {
                if window.suspended || window.outcome == DeviceRecoveryWindowOutcome::Destroyed {
                    continue;
                }
                match send_window_device_recovery(
                    window.hwnd,
                    validation_number,
                    self.generation,
                    WindowDeviceRecoveryAction::Suspend,
                ) {
                    WindowDeviceRecoveryOutcome::Suspended => window.suspended = true,
                    WindowDeviceRecoveryOutcome::Destroyed => window.record_destroyed(),
                    WindowDeviceRecoveryOutcome::Deferred => {}
                    outcome => log::warn!(
                        "gpui_device_loss generation={} window={:?} suspend={outcome:?}",
                        self.generation,
                        window.hwnd.as_raw()
                    ),
                }
            }
            if self.windows.iter().all(|window| {
                window.suspended || window.outcome == DeviceRecoveryWindowOutcome::Destroyed
            }) {
                self.phase = DeviceRecoveryPhase::RecreateGlobal {
                    attempts: 0,
                    next_attempt: now,
                };
            }
        }

        let should_recreate_global = match &self.phase {
            DeviceRecoveryPhase::RecreateGlobal { next_attempt, .. } => now >= *next_attempt,
            _ => false,
        };
        if should_recreate_global {
            let attempt = match &self.phase {
                DeviceRecoveryPhase::RecreateGlobal { attempts, .. } => *attempts + 1,
                _ => unreachable!(),
            };
            let candidate = DirectXDevices::new()
                .context("recreating global DirectX devices")
                .and_then(|devices| {
                    DirectWriteTextSystem::build_gpu_recovery_candidate(&devices)
                        .map(|gpu_state| (devices, gpu_state))
                });

            match candidate {
                Ok((devices, gpu_state)) => {
                    let Some(text_system) = text_system.upgrade() else {
                        self.finish(shared_recovery);
                        return true;
                    };
                    if publish_global_device_recovery(
                        platform_window,
                        validation_number,
                        devices.clone(),
                        text_system,
                        gpu_state,
                    ) {
                        log::info!(
                            "gpui_device_loss generation={} global_attempt={} active elapsed_ms={} {}",
                            self.generation,
                            attempt,
                            self.started_at.elapsed().as_millis(),
                            adapter_diagnostics(&devices),
                        );
                        *current_devices = devices.clone();
                        for window in &mut self.windows {
                            window.reset_for_recovery(now);
                        }
                        self.phase = DeviceRecoveryPhase::RecoverWindows { devices };
                    } else {
                        self.schedule_global_retry(now, attempt);
                    }
                }
                Err(error) => {
                    log::error!(
                        "gpui_device_loss generation={} global_attempt={} failed: {error:#}",
                        self.generation,
                        attempt
                    );
                    self.schedule_global_retry(now, attempt);
                }
            }
        }

        let recovery_devices = match &self.phase {
            DeviceRecoveryPhase::RecoverWindows { devices } => Some(devices.clone()),
            _ => None,
        };
        if let Some(devices) = recovery_devices {
            for window in &mut self.windows {
                if window.outcome != DeviceRecoveryWindowOutcome::Pending
                    || now < window.next_attempt
                {
                    continue;
                }

                let attempt = window.next_attempt_number();
                let outcome = send_window_device_recovery(
                    window.hwnd,
                    validation_number,
                    self.generation,
                    WindowDeviceRecoveryAction::Recover(devices.clone()),
                );
                match outcome {
                    WindowDeviceRecoveryOutcome::Active => {
                        window.record_active();
                        #[cfg(debug_assertions)]
                        if std::env::var_os("GPUI_TEST_DEVICE_RECOVERY_FAILURE").is_some() {
                            eprintln!(
                                "gpui_device_loss_test generation={} window={:?} attempt={} result=active",
                                self.generation,
                                window.hwnd.as_raw(),
                                attempt
                            );
                        }
                        log::info!(
                            "gpui_device_loss generation={} window={:?} attempt={} active",
                            self.generation,
                            window.hwnd.as_raw(),
                            attempt
                        );
                    }
                    WindowDeviceRecoveryOutcome::Destroyed => window.record_destroyed(),
                    WindowDeviceRecoveryOutcome::Deferred => window.record_deferred(now),
                    WindowDeviceRecoveryOutcome::Failed(error) => {
                        log::error!(
                            "gpui_device_loss generation={} window={:?} attempt={} failed: {}",
                            self.generation,
                            window.hwnd.as_raw(),
                            attempt,
                            error
                        );
                        if window.record_retryable_failure(now) {
                            log::error!(
                                "gpui_device_loss generation={} window={:?} attempt={} exhausted",
                                self.generation,
                                window.hwnd.as_raw(),
                                attempt
                            );
                            #[cfg(debug_assertions)]
                            if std::env::var_os("GPUI_TEST_DEVICE_RECOVERY_FAILURE").is_some() {
                                eprintln!(
                                    "gpui_device_loss_test generation={} window={:?} attempt={} result=exhausted",
                                    self.generation,
                                    window.hwnd.as_raw(),
                                    attempt
                                );
                            }
                        }
                    }
                    WindowDeviceRecoveryOutcome::Stale => {
                        log::warn!(
                            "gpui_device_loss generation={} window={:?} attempt={} stale candidate",
                            self.generation,
                            window.hwnd.as_raw(),
                            attempt
                        );
                        // A live resize or generation race invalidates this
                        // candidate without proving the replacement device is
                        // bad. Retry on the next vsync without consuming one of
                        // the eight device-recovery attempts.
                        window.record_deferred(now);
                    }
                    WindowDeviceRecoveryOutcome::Suspended => {}
                }
            }

            if self.windows.iter().all(DeviceRecoveryWindow::is_terminal) {
                let mut shared = shared_recovery.lock();
                if shared.generation != self.generation {
                    return true;
                }
                if shared.new_windows.is_empty() {
                    shared.generation = 0;
                    log::info!(
                        "gpui_device_loss generation={} complete elapsed_ms={}",
                        self.generation,
                        self.started_at.elapsed().as_millis()
                    );
                    return true;
                }
            }
        }
        false
    }

    fn add_new_windows(
        &mut self,
        shared_recovery: &Arc<Mutex<SharedDeviceRecovery>>,
        now: Instant,
    ) {
        let new_windows = {
            let mut shared = shared_recovery.lock();
            if shared.generation != self.generation {
                return;
            }
            std::mem::take(&mut shared.new_windows)
        };
        for hwnd in new_windows {
            if let Some(window) = self
                .windows
                .iter_mut()
                .find(|window| window.hwnd.as_raw() == hwnd.as_raw())
            {
                // Windows can recycle a destroyed HWND before this recovery
                // generation finishes. A queued new window with the same raw
                // handle owns a new suspended renderer, so replace the old
                // terminal record instead of discarding it as a duplicate.
                if window.is_terminal() {
                    *window = DeviceRecoveryWindow::new(hwnd, true, now);
                }
                continue;
            }
            self.windows
                .push(DeviceRecoveryWindow::new(hwnd, true, now));
        }
    }

    fn schedule_global_retry(&mut self, now: Instant, attempt: usize) {
        let delay_index = attempt.min(DEVICE_RECOVERY_RETRY_DELAYS_MS.len() - 1);
        self.phase = DeviceRecoveryPhase::RecreateGlobal {
            attempts: attempt,
            next_attempt: now + Duration::from_millis(DEVICE_RECOVERY_RETRY_DELAYS_MS[delay_index]),
        };
    }

    fn finish(&self, shared_recovery: &Arc<Mutex<SharedDeviceRecovery>>) {
        let mut shared = shared_recovery.lock();
        if shared.generation == self.generation {
            shared.generation = 0;
            shared.new_windows.clear();
        }
    }
}

fn send_window_device_recovery(
    hwnd: SafeHwnd,
    validation_number: usize,
    generation: u64,
    action: WindowDeviceRecoveryAction,
) -> WindowDeviceRecoveryOutcome {
    // SAFETY: the raw handle is used only as an identifier for the Win32
    // validity query; no application memory is dereferenced.
    if !unsafe { IsWindow(Some(hwnd.as_raw())).as_bool() } {
        return WindowDeviceRecoveryOutcome::Destroyed;
    }
    let mut request = WindowDeviceRecoveryRequest {
        generation,
        action,
        outcome: None,
    };
    // SAFETY: `SendMessageW` is synchronous, so `request` remains live while
    // the validated GPUI window procedure reads it and writes its outcome.
    unsafe {
        SendMessageW(
            hwnd.as_raw(),
            WM_GPUI_GPU_DEVICE_LOST,
            Some(WPARAM(validation_number)),
            Some(LPARAM(&raw mut request as *mut _ as isize)),
        );
    }
    request.outcome.unwrap_or_else(|| {
        // SAFETY: the raw handle is used only for a validity query after the
        // synchronous send; no application memory is dereferenced.
        if unsafe { IsWindow(Some(hwnd.as_raw())).as_bool() } {
            WindowDeviceRecoveryOutcome::Deferred
        } else {
            WindowDeviceRecoveryOutcome::Destroyed
        }
    })
}

fn publish_global_device_recovery(
    platform_window: HWND,
    validation_number: usize,
    directx_devices: DirectXDevices,
    text_system: Arc<DirectWriteTextSystem>,
    gpu_state: GPUState,
) -> bool {
    let mut request = GlobalDeviceRecoveryRequest {
        directx_devices,
        text_system,
        gpu_state: Some(gpu_state),
        published: false,
    };
    // SAFETY: `SendMessageW` is synchronous, so `request` remains live while
    // the validated platform window procedure consumes its candidate state.
    unsafe {
        SendMessageW(
            platform_window,
            WM_GPUI_GPU_DEVICE_LOST,
            Some(WPARAM(validation_number)),
            Some(LPARAM(&raw mut request as *mut _ as isize)),
        );
    }
    request.published
}

const PLATFORM_WINDOW_CLASS_NAME: PCWSTR = w!("Zed::PlatformWindow");

fn register_platform_window_class() {
    let wc = WNDCLASSW {
        lpfnWndProc: Some(window_procedure),
        lpszClassName: PCWSTR(PLATFORM_WINDOW_CLASS_NAME.as_ptr()),
        ..Default::default()
    };
    unsafe { RegisterClassW(&wc) };
}

unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_NCCREATE {
        let params = unsafe { &*(lparam.0 as *const CREATESTRUCTW) };
        let creation_context = params.lpCreateParams as *mut PlatformWindowCreateContext;
        let creation_context = unsafe { &mut *creation_context };

        let Some(main_sender) = creation_context.main_sender.take() else {
            creation_context.inner = Some(Err(anyhow!("missing main sender")));
            return LRESULT(0);
        };
        creation_context.dispatcher = Some(Arc::new(WindowsDispatcher::new(
            main_sender,
            hwnd,
            creation_context.validation_number,
        )));

        return match WindowsPlatformInner::new(creation_context) {
            Ok(inner) => {
                let weak = Box::new(Rc::downgrade(&inner));
                unsafe { set_window_long(hwnd, GWLP_USERDATA, Box::into_raw(weak) as isize) };
                creation_context.inner = Some(Ok(inner));
                unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
            }
            Err(error) => {
                creation_context.inner = Some(Err(error));
                LRESULT(0)
            }
        };
    }

    let ptr = unsafe { get_window_long(hwnd, GWLP_USERDATA) } as *mut Weak<WindowsPlatformInner>;
    if ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    }
    let inner = unsafe { &*ptr };
    let result = if let Some(inner) = inner.upgrade() {
        if cfg!(debug_assertions) {
            let inner = std::panic::AssertUnwindSafe(inner);
            match std::panic::catch_unwind(|| { inner }.handle_msg(hwnd, msg, wparam, lparam)) {
                Ok(result) => result,
                Err(_) => std::process::abort(),
            }
        } else {
            inner.handle_msg(hwnd, msg, wparam, lparam)
        }
    } else {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    };

    if msg == WM_NCDESTROY {
        unsafe { set_window_long(hwnd, GWLP_USERDATA, 0) };
        unsafe { drop(Box::from_raw(ptr)) };
    }

    result
}

#[cfg(test)]
mod tests {
    use std::ffi::{OsStr, OsString};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::{Duration, Instant};

    use crate::{read_from_clipboard, write_to_clipboard};
    use gpui::ClipboardItem;
    use parking_lot::Mutex;
    use windows::Win32::Foundation::HWND;

    use super::{
        DeviceRecovery, DeviceRecoveryPhase, DeviceRecoveryWindow, DeviceRecoveryWindowOutcome,
        SafeHwnd, SharedDeviceRecovery, encode_restart_arguments,
        parse_injected_device_loss_vsyncs, take_device_invalidation, window_recovery_retry_delay,
    };

    #[test]
    fn test_encode_restart_arguments() {
        assert_eq!(encode_restart_arguments(&[]), OsStr::new(""));
        assert_eq!(
            encode_restart_arguments(&[
                OsString::from("--user-data-dir"),
                OsString::from(r"C:\Zed Data"),
            ]),
            OsStr::new(r#""--user-data-dir" "C:\Zed Data""#)
        );
        assert_eq!(
            encode_restart_arguments(&[OsString::from(r"C:\")]),
            OsStr::new(r#""C:\\""#)
        );
    }

    #[test]
    fn parses_synthetic_device_loss_vsyncs() {
        assert_eq!(
            parse_injected_device_loss_vsyncs("30, 90,broken,120"),
            [30, 90, 120]
        );
    }

    #[test]
    fn window_recovery_has_eight_bounded_attempts() {
        let delays = (0..8)
            .map(|attempts| window_recovery_retry_delay(attempts).unwrap().as_millis())
            .collect::<Vec<_>>();
        assert_eq!(delays, [0, 100, 250, 500, 1000, 2000, 4000, 8000]);
        assert!(window_recovery_retry_delay(8).is_none());
    }
    #[test]
    fn retryable_window_failure_exhausts_on_attempt_eight() {
        let now = Instant::now();
        let mut window = DeviceRecoveryWindow::new(SafeHwnd::from(HWND::default()), true, now);

        for attempt in 1..8 {
            assert_eq!(window.next_attempt_number(), attempt);
            assert!(!window.record_retryable_failure(now));
            assert_eq!(window.attempts, attempt);
            assert_eq!(window.outcome, DeviceRecoveryWindowOutcome::Pending);
            assert_eq!(
                window.next_attempt,
                now + window_recovery_retry_delay(attempt).unwrap()
            );
        }

        assert_eq!(window.next_attempt_number(), 8);
        assert!(window.record_retryable_failure(now));
        assert_eq!(window.attempts, 8);
        assert_eq!(window.outcome, DeviceRecoveryWindowOutcome::Exhausted);
        assert!(window.is_terminal());
    }

    #[test]
    fn deferred_window_recovery_does_not_consume_an_attempt() {
        let now = Instant::now();
        let later = now + Duration::from_secs(1);
        let mut window = DeviceRecoveryWindow::new(SafeHwnd::from(HWND::default()), true, now);
        assert!(!window.record_retryable_failure(now));
        assert_eq!(window.attempts, 1);

        window.record_deferred(later);

        assert_eq!(window.attempts, 1);
        assert_eq!(window.next_attempt, later);
        assert_eq!(window.outcome, DeviceRecoveryWindowOutcome::Pending);
    }

    #[test]
    fn device_invalidation_is_retained_until_recovery_finishes() {
        let invalidated = AtomicBool::new(true);

        assert!(!take_device_invalidation(true, &invalidated));
        assert!(invalidated.load(Ordering::Acquire));
        assert!(take_device_invalidation(false, &invalidated));
        assert!(!invalidated.load(Ordering::Acquire));
    }

    #[test]
    fn active_and_destroyed_windows_are_terminal() {
        let now = Instant::now();
        let mut active = DeviceRecoveryWindow::new(SafeHwnd::from(HWND::default()), true, now);
        active.record_active();
        assert_eq!(active.attempts, 1);
        assert_eq!(active.outcome, DeviceRecoveryWindowOutcome::Active);
        assert!(active.is_terminal());

        let mut destroyed = DeviceRecoveryWindow::new(SafeHwnd::from(HWND::default()), true, now);
        destroyed.record_destroyed();
        assert_eq!(destroyed.attempts, 0);
        assert_eq!(destroyed.outcome, DeviceRecoveryWindowOutcome::Destroyed);
        assert!(destroyed.is_terminal());
    }

    #[test]
    fn recycled_window_handle_replaces_terminal_recovery_record() {
        let now = Instant::now();
        let hwnd = SafeHwnd::from(HWND::default());
        let mut old_window = DeviceRecoveryWindow::new(hwnd, true, now);
        old_window.record_active();
        let mut recovery = DeviceRecovery {
            generation: 7,
            started_at: now,
            phase: DeviceRecoveryPhase::Suspending,
            windows: vec![old_window],
        };
        let shared = Arc::new(Mutex::new(SharedDeviceRecovery {
            generation: 7,
            last_generation: 7,
            new_windows: vec![hwnd],
        }));

        recovery.add_new_windows(&shared, now + Duration::from_secs(1));

        assert_eq!(recovery.windows.len(), 1);
        let replacement = &recovery.windows[0];
        assert!(replacement.suspended);
        assert_eq!(replacement.attempts, 0);
        assert_eq!(replacement.outcome, DeviceRecoveryWindowOutcome::Pending);
    }

    #[test]
    fn test_clipboard() {
        let item = ClipboardItem::new_string("你好，我是张小白".to_string());
        write_to_clipboard(item.clone());
        assert_eq!(read_from_clipboard(), Some(item));

        let item = ClipboardItem::new_string("12345".to_string());
        write_to_clipboard(item.clone());
        assert_eq!(read_from_clipboard(), Some(item));

        let item = ClipboardItem::new_string_with_json_metadata("abcdef".to_string(), vec![3, 4]);
        write_to_clipboard(item.clone());
        assert_eq!(read_from_clipboard(), Some(item));
    }
}
