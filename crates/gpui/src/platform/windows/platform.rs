use std::{
    cell::RefCell,
    ffi::OsStr,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use ::util::{ResultExt, paths::SanitizedPath};
use anyhow::{Context as _, Result, anyhow};
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use parking_lot::RwLock;
use smallvec::SmallVec;
use windows::{
    UI::ViewManagement::UISettings,
    Win32::{
        Foundation::*,
        Graphics::{Direct3D11::ID3D11Device, Gdi::*},
        Security::Credentials::*,
        System::{Com::*, LibraryLoader::*, Ole::*, SystemInformation::*},
        UI::{Input::KeyboardAndMouse::*, Shell::*, WindowsAndMessaging::*},
    },
    core::*,
};

use crate::*;

pub(crate) struct WindowsPlatform {
    inner: Rc<WindowsPlatformInner>,
    raw_window_handles: Arc<RwLock<SmallVec<[SafeHwnd; 4]>>>,
    // The below members will never change throughout the entire lifecycle of the app.
    icon: HICON,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<DirectWriteTextSystem>,
    windows_version: WindowsVersion,
    drop_target_helper: IDropTargetHelper,
    /// Flag to instruct the `VSyncProvider` thread to invalidate the directx devices
    /// as resizing them has failed, causing us to have lost at least the render target.
    invalidate_devices: Arc<AtomicBool>,
    handle: HWND,
    disable_direct_composition: bool,
}

struct WindowsPlatformInner {
    state: RefCell<WindowsPlatformState>,
    raw_window_handles: std::sync::Weak<RwLock<SmallVec<[SafeHwnd; 4]>>>,
    // The below members will never change throughout the entire lifecycle of the app.
    validation_number: usize,
    main_receiver: flume::Receiver<RunnableVariant>,
    dispatcher: Arc<WindowsDispatcher>,
}

pub(crate) struct WindowsPlatformState {
    callbacks: PlatformCallbacks,
    menus: Vec<OwnedMenu>,
    jump_list: JumpList,
    // NOTE: standard cursor handles don't need to close.
    pub(crate) current_cursor: Option<HCURSOR>,
    directx_devices: Option<DirectXDevices>,
}

#[derive(Default)]
struct PlatformCallbacks {
    open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    quit: Option<Box<dyn FnMut()>>,
    reopen: Option<Box<dyn FnMut()>>,
    app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    will_open_app_menu: Option<Box<dyn FnMut()>>,
    validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
    keyboard_layout_change: Option<Box<dyn FnMut()>>,
}

impl WindowsPlatformState {
    fn new(directx_devices: DirectXDevices) -> Self {
        let callbacks = PlatformCallbacks::default();
        let jump_list = JumpList::new();
        let current_cursor = load_cursor(CursorStyle::Arrow);
        let directx_devices = Some(directx_devices);

        Self {
            callbacks,
            jump_list,
            current_cursor,
            directx_devices,
            menus: Vec::new(),
        }
    }
}

impl WindowsPlatform {
    pub(crate) fn new() -> Result<Self> {
        unsafe {
            OleInitialize(None).context("unable to initialize Windows OLE")?;
        }
        let directx_devices = DirectXDevices::new().context("Creating DirectX devices")?;
        let (main_sender, main_receiver) = flume::unbounded::<RunnableVariant>();
        let validation_number = if usize::BITS == 64 {
            rand::random::<u64>() as usize
        } else {
            rand::random::<u32>() as usize
        };
        let raw_window_handles = Arc::new(RwLock::new(SmallVec::new()));
        let text_system = Arc::new(
            DirectWriteTextSystem::new(&directx_devices)
                .context("Error creating DirectWriteTextSystem")?,
        );
        register_platform_window_class();
        let mut context = PlatformWindowCreateContext {
            inner: None,
            raw_window_handles: Arc::downgrade(&raw_window_handles),
            validation_number,
            main_sender: Some(main_sender),
            main_receiver: Some(main_receiver),
            directx_devices: Some(directx_devices),
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

        let drop_target_helper: IDropTargetHelper = unsafe {
            CoCreateInstance(&CLSID_DragDropHelper, None, CLSCTX_INPROC_SERVER)
                .context("Error creating drop target helper.")?
        };
        let icon = load_icon().unwrap_or_default();
        let windows_version = WindowsVersion::new().context("Error retrieve windows version")?;

        Ok(Self {
            inner,
            handle,
            raw_window_handles,
            icon,
            background_executor,
            foreground_executor,
            text_system,
            disable_direct_composition,
            windows_version,
            drop_target_helper,
            invalidate_devices: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn window_from_hwnd(&self, hwnd: HWND) -> Option<Rc<WindowsWindowInner>> {
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

    fn generate_creation_info(&self) -> WindowCreationInfo {
        WindowCreationInfo {
            icon: self.icon,
            executor: self.foreground_executor.clone(),
            current_cursor: self.inner.state.borrow().current_cursor,
            windows_version: self.windows_version,
            drop_target_helper: self.drop_target_helper.clone(),
            validation_number: self.inner.validation_number,
            main_receiver: self.inner.main_receiver.clone(),
            platform_window_handle: self.handle,
            disable_direct_composition: self.disable_direct_composition,
            directx_devices: self.inner.state.borrow().directx_devices.clone().unwrap(),
            invalidate_devices: self.invalidate_devices.clone(),
        }
    }

    fn set_dock_menus(&self, menus: Vec<MenuItem>) {
        let mut actions = Vec::new();
        menus.into_iter().for_each(|menu| {
            if let Some(dock_menu) = DockMenuItem::new(menu).log_err() {
                actions.push(dock_menu);
            }
        });
        let mut lock = self.inner.state.borrow_mut();
        lock.jump_list.dock_menus = actions;
        update_jump_list(&lock.jump_list).log_err();
    }

    fn update_jump_list(
        &self,
        menus: Vec<MenuItem>,
        entries: Vec<SmallVec<[PathBuf; 2]>>,
    ) -> Vec<SmallVec<[PathBuf; 2]>> {
        let mut actions = Vec::new();
        menus.into_iter().for_each(|menu| {
            if let Some(dock_menu) = DockMenuItem::new(menu).log_err() {
                actions.push(dock_menu);
            }
        });
        let mut lock = self.inner.state.borrow_mut();
        lock.jump_list.dock_menus = actions;
        lock.jump_list.recent_workspaces = entries;
        update_jump_list(&lock.jump_list)
            .log_err()
            .unwrap_or_default()
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
        let mut directx_device = self.inner.state.borrow().directx_devices.clone().unwrap();
        let platform_window: SafeHwnd = self.handle.into();
        let validation_number = self.inner.validation_number;
        let all_windows = Arc::downgrade(&self.raw_window_handles);
        let text_system = Arc::downgrade(&self.text_system);
        let invalidate_devices = self.invalidate_devices.clone();

        std::thread::Builder::new()
            .name("VSyncProvider".to_owned())
            .spawn(move || {
                let vsync_provider = VSyncProvider::new();
                loop {
                    vsync_provider.wait_for_vsync();
                    if check_device_lost(&directx_device.device)
                        || invalidate_devices.fetch_and(false, Ordering::Acquire)
                    {
                        if let Err(err) = handle_gpu_device_lost(
                            &mut directx_device,
                            platform_window.as_raw(),
                            validation_number,
                            &all_windows,
                            &text_system,
                        ) {
                            panic!("Device lost: {err}");
                        }
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
            .borrow_mut()
            .callbacks
            .keyboard_layout_change = Some(callback);
    }

    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        on_finish_launching();
        self.begin_vsync_thread();

        let mut msg = MSG::default();
        unsafe {
            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                if translate_accelerator(&msg).is_none() {
                    _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }

        self.inner
            .with_callback(|callbacks| &mut callbacks.quit, |callback| callback());
    }

    fn quit(&self) {
        self.foreground_executor()
            .spawn(async { unsafe { PostQuitMessage(0) } })
            .detach();
    }

    fn restart(&self, binary_path: Option<PathBuf>) {
        let pid = std::process::id();
        let Some(app_path) = binary_path.or(self.app_path().log_err()) else {
            return;
        };
        let script = format!(
            r#"
            $pidToWaitFor = {}
            $exePath = "{}"

            while ($true) {{
                $process = Get-Process -Id $pidToWaitFor -ErrorAction SilentlyContinue
                if (-not $process) {{
                    Start-Process -FilePath $exePath
                    break
                }}
                Start-Sleep -Seconds 0.1
            }}
            "#,
            pid,
            app_path.display(),
        );

        #[allow(
            clippy::disallowed_methods,
            reason = "We are restarting ourselves, using std command thus is fine"
        )]
        let restart_process = util::command::new_std_command("powershell.exe")
            .arg("-command")
            .arg(script)
            .spawn();

        match restart_process {
            Ok(_) => self.quit(),
            Err(e) => log::error!("failed to spawn restart script: {:?}", e),
        }
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
        crate::platform::scap_screen_capture::scap_screen_sources(&self.foreground_executor)
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
        let window = WindowsWindow::new(handle, options, self.generate_creation_info())?;
        let handle = window.get_raw_handle();
        self.raw_window_handles.write().push(handle.into());

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
        self.inner.state.borrow_mut().callbacks.open_urls = Some(callback);
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

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.inner.state.borrow_mut().callbacks.quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.inner.state.borrow_mut().callbacks.reopen = Some(callback);
    }

    fn set_menus(&self, menus: Vec<Menu>, _keymap: &Keymap) {
        self.inner.state.borrow_mut().menus = menus.into_iter().map(|menu| menu.owned()).collect();
    }

    fn get_menus(&self) -> Option<Vec<OwnedMenu>> {
        Some(self.inner.state.borrow().menus.clone())
    }

    fn set_dock_menu(&self, menus: Vec<MenuItem>, _keymap: &Keymap) {
        self.set_dock_menus(menus);
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.inner.state.borrow_mut().callbacks.app_menu_action = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.inner.state.borrow_mut().callbacks.will_open_app_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.inner
            .state
            .borrow_mut()
            .callbacks
            .validate_app_menu_command = Some(callback);
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
        if self.inner.state.borrow_mut().current_cursor.map(|c| c.0) != hcursor.map(|c| c.0) {
            self.post_message(
                WM_GPUI_CURSOR_STYLE_CHANGED,
                WPARAM(0),
                LPARAM(hcursor.map_or(0, |c| c.0 as isize)),
            );
            self.inner.state.borrow_mut().current_cursor = hcursor;
        }
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
        let mut password = password.to_vec();
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
            unsafe { CredWriteW(&credentials, 0) }?;
            Ok(())
        })
    }

    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        let mut target_name = windows_credentials_target_name(url)
            .encode_utf16()
            .chain(Some(0))
            .collect_vec();
        self.foreground_executor().spawn(async move {
            let mut credentials: *mut CREDENTIALW = std::ptr::null_mut();
            unsafe {
                CredReadW(
                    PCWSTR::from_raw(target_name.as_ptr()),
                    CRED_TYPE_GENERIC,
                    None,
                    &mut credentials,
                )?
            };

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
        let mut target_name = windows_credentials_target_name(url)
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
    ) -> Vec<SmallVec<[PathBuf; 2]>> {
        self.update_jump_list(menus, entries)
    }
}

impl WindowsPlatformInner {
    fn new(context: &mut PlatformWindowCreateContext) -> Result<Rc<Self>> {
        let state = RefCell::new(WindowsPlatformState::new(
            context
                .directx_devices
                .take()
                .context("missing directx devices")?,
        ));
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
        project: impl Fn(&mut PlatformCallbacks) -> &mut Option<T>,
        f: impl FnOnce(&mut T),
    ) {
        let callback = project(&mut self.state.borrow_mut().callbacks).take();
        if let Some(mut callback) = callback {
            f(&mut callback);
            *project(&mut self.state.borrow_mut().callbacks) = Some(callback)
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
            | WM_GPUI_GPU_DEVICE_LOST => self.handle_gpui_events(msg, wparam, lparam),
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
            WM_GPUI_GPU_DEVICE_LOST => self.handle_device_lost(lparam),
            _ => unreachable!(),
        }
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
        loop {
            for runnable in self.main_receiver.drain() {
                WindowsDispatcher::execute_runnable(runnable);
            }

            // Someone could enqueue a Runnable here. The flag is still true, so they will not PostMessage.
            // We need to check for those Runnables after we clear the flag.
            let dispatcher = self.dispatcher.clone();

            dispatcher.wake_posted.store(false, Ordering::Release);
            match self.main_receiver.try_recv() {
                Ok(runnable) => {
                    let _ = dispatcher.wake_posted.swap(true, Ordering::AcqRel);

                    WindowsDispatcher::execute_runnable(runnable);
                    continue;
                }
                _ => {
                    break;
                }
            }
        }

        Some(0)
    }

    fn handle_dock_action_event(&self, action_idx: usize) -> Option<isize> {
        let Some(action) = self
            .state
            .borrow_mut()
            .jump_list
            .dock_menus
            .get(action_idx)
            .map(|dock_menu| dock_menu.action.boxed_clone())
        else {
            log::error!("Dock menu for index {action_idx} not found");
            return Some(1);
        };
        self.with_callback(
            |callbacks| &mut callbacks.app_menu_action,
            |callback| callback(&*action),
        );
        Some(0)
    }

    fn handle_keyboard_layout_change(&self) -> Option<isize> {
        self.with_callback(
            |callbacks| &mut callbacks.keyboard_layout_change,
            |callback| callback(),
        );
        Some(0)
    }

    fn handle_device_lost(&self, lparam: LPARAM) -> Option<isize> {
        let directx_devices = lparam.0 as *const DirectXDevices;
        let directx_devices = unsafe { &*directx_devices };
        let mut lock = self.state.borrow_mut();
        lock.directx_devices.take();
        lock.directx_devices = Some(directx_devices.clone());

        Some(0)
    }
}

impl Drop for WindowsPlatform {
    fn drop(&mut self) {
        unsafe {
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
    pub(crate) windows_version: WindowsVersion,
    pub(crate) drop_target_helper: IDropTargetHelper,
    pub(crate) validation_number: usize,
    pub(crate) main_receiver: flume::Receiver<RunnableVariant>,
    pub(crate) platform_window_handle: HWND,
    pub(crate) disable_direct_composition: bool,
    pub(crate) directx_devices: DirectXDevices,
    /// Flag to instruct the `VSyncProvider` thread to invalidate the directx devices
    /// as resizing them has failed, causing us to have lost at least the render target.
    pub(crate) invalidate_devices: Arc<AtomicBool>,
}

struct PlatformWindowCreateContext {
    inner: Option<Result<Rc<WindowsPlatformInner>>>,
    raw_window_handles: std::sync::Weak<RwLock<SmallVec<[SafeHwnd; 4]>>>,
    validation_number: usize,
    main_sender: Option<flume::Sender<RunnableVariant>>,
    main_receiver: Option<flume::Receiver<RunnableVariant>>,
    directx_devices: Option<DirectXDevices>,
    dispatcher: Option<Arc<WindowsDispatcher>>,
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
        let full_path = SanitizedPath::new(&full_path);
        let full_path_string = full_path.to_string();
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
            log::error!("DirectX device lost detected: {:?}", err);
            true
        }
    }
}

fn handle_gpu_device_lost(
    directx_devices: &mut DirectXDevices,
    platform_window: HWND,
    validation_number: usize,
    all_windows: &std::sync::Weak<RwLock<SmallVec<[SafeHwnd; 4]>>>,
    text_system: &std::sync::Weak<DirectWriteTextSystem>,
) -> Result<()> {
    // Here we wait a bit to ensure the system has time to recover from the device lost state.
    // If we don't wait, the final drawing result will be blank.
    std::thread::sleep(std::time::Duration::from_millis(350));

    *directx_devices = try_to_recover_from_device_lost(|| {
        DirectXDevices::new().context("Failed to recreate new DirectX devices after device lost")
    })?;
    log::info!("DirectX devices successfully recreated.");

    let lparam = LPARAM(directx_devices as *const _ as _);
    unsafe {
        SendMessageW(
            platform_window,
            WM_GPUI_GPU_DEVICE_LOST,
            Some(WPARAM(validation_number)),
            Some(lparam),
        );
    }

    if let Some(text_system) = text_system.upgrade() {
        text_system.handle_gpu_lost(&directx_devices)?;
    }
    if let Some(all_windows) = all_windows.upgrade() {
        for window in all_windows.read().iter() {
            unsafe {
                SendMessageW(
                    window.as_raw(),
                    WM_GPUI_GPU_DEVICE_LOST,
                    Some(WPARAM(validation_number)),
                    Some(lparam),
                );
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
        for window in all_windows.read().iter() {
            unsafe {
                SendMessageW(
                    window.as_raw(),
                    WM_GPUI_FORCE_UPDATE_WINDOW,
                    Some(WPARAM(validation_number)),
                    None,
                );
            }
        }
    }
    Ok(())
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
        inner.handle_msg(hwnd, msg, wparam, lparam)
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
    use crate::{ClipboardItem, read_from_clipboard, write_to_clipboard};

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
