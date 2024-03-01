use std::{
    alloc::GlobalAlloc,
    cell::RefCell,
    fmt::Write,
    os::windows::process::CommandExt,
    path::{Path, PathBuf},
    process::Stdio,
    rc::Rc,
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use async_task::Runnable;
use futures::channel::oneshot;
use time::UtcOffset;
use windows::{
    core::{HSTRING, PCWSTR},
    Wdk::System::SystemServices::RtlGetVersion,
    Win32::{
        Foundation::{HANDLE, HGLOBAL, HWND, LPARAM, LRESULT, STATUS_SUCCESS, WPARAM},
        Globalization::{u_memcpy, MultiByteToWideChar},
        Security::Credentials::{
            CredEnumerateW, CredWriteDomainCredentialsW, CREDENTIAL_TARGET_INFORMATIONW,
        },
        System::{
            Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize, CreateBindCtx, CLSCTX_ALL,
                COINIT_DISABLE_OLE1DDE, COINIT_MULTITHREADED, DVASPECT_CONTENT, FORMATETC,
                TYMED_HGLOBAL,
            },
            DataExchange::{
                CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData,
            },
            Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE},
            Ole::{OleGetClipboard, OleInitialize, OleUninitialize, ReleaseStgMedium},
            Threading::CREATE_NO_WINDOW,
            Time::{GetTimeZoneInformation, TIME_ZONE_ID_INVALID},
        },
        UI::{
            HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE},
            Input::KeyboardAndMouse::{
                GetDoubleClickTime, ToUnicode, VIRTUAL_KEY, VK_BACK, VK_DELETE, VK_DOWN, VK_END,
                VK_ESCAPE, VK_F1, VK_F10, VK_F11, VK_F12, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7,
                VK_F8, VK_F9, VK_HOME, VK_LEFT, VK_NEXT, VK_PRIOR, VK_RETURN, VK_RIGHT, VK_UP,
            },
            Shell::{
                FileOpenDialog, FileSaveDialog, IFileOpenDialog, IFileSaveDialog, IShellItem,
                SHCreateItemFromParsingName, ShellExecuteW, FILEOPENDIALOGOPTIONS,
                FOS_ALLOWMULTISELECT, FOS_PICKFOLDERS, SIGDN_PARENTRELATIVEPARSING,
            },
            WindowsAndMessaging::{
                AppendMenuW, CreateAcceleratorTableW, CreateMenu, DefWindowProcW, DestroyWindow,
                DispatchMessageW, GetMessageW, LoadCursorW, LoadImageW, PostQuitMessage, SetCursor,
                TranslateMessage, ACCEL, ACCEL_VIRT_FLAGS, HCURSOR, HMENU, IDC_ARROW, IDC_CROSS,
                IDC_HAND, IDC_IBEAM, IDC_NO, IDC_SIZENS, IDC_SIZEWE, IMAGE_CURSOR, LR_DEFAULTSIZE,
                LR_SHARED, MF_POPUP, MF_SEPARATOR, MF_STRING, SW_SHOWDEFAULT, WM_DESTROY,
            },
        },
    },
};

use crate::{
    encode_wide, log_windows_error, log_windows_error_with_message,
    platform::cross_platform::CosmicTextSystem, set_windowdata, Keystroke, WindowsWindow,
    WindowsWindowBase, WindowsWinodwDataWrapper, ACCEL_FALT, ACCEL_FCONTROL, ACCEL_FSHIFT,
    ACCEL_FVIRTKEY, CF_UNICODETEXT, DISPATCH_WINDOW_CLASS, DISPATCH_WINDOW_EXSTYLE,
    DISPATCH_WINDOW_STYLE, MAIN_DISPATCH, MENU_ACTIONS, WINDOW_CLOSE,
};
use crate::{
    Action, AnyWindowHandle, BackgroundExecutor, ClipboardItem, CursorStyle, DisplayId,
    ForegroundExecutor, Keymap, Menu, PathPromptOptions, Platform, PlatformDisplay, PlatformInput,
    PlatformTextSystem, PlatformWindow, Result, SemanticVersion, Task, WindowOptions,
};

use super::{WindowsDispatcher, WindowsDisplay};

#[derive(Default)]
pub struct PlatformCallbacks {
    pub quit: Option<Box<dyn FnMut()>>,
    pub open_urls: Option<Box<dyn FnMut(Vec<String>)>>,
    pub become_active: Option<Box<dyn FnMut()>>,
    pub resign_active: Option<Box<dyn FnMut()>>,
    pub reopen: Option<Box<dyn FnMut()>>,
    pub event: Option<Box<dyn FnMut(PlatformInput) -> bool>>,
    pub app_menu_action: Option<Box<dyn FnMut(&dyn Action)>>,
    pub will_open_app_menu: Option<Box<dyn FnMut()>>,
    pub validate_app_menu_command: Option<Box<dyn FnMut(&dyn Action) -> bool>>,
}

pub(crate) struct WindowsPlatform {
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<CosmicTextSystem>,
    inner: Rc<WindowsPlatformInner>,
    menu_handle: RefCell<Option<HMENU>>,
}

impl WindowsPlatform {
    pub(crate) fn new() -> Self {
        platform_init().expect("error init windows platform");
        let (sender, receiver) = flume::unbounded::<Runnable>();
        let dispatch_window_handle = <WindowsPlatformInner as WindowsWindowBase>::create(
            DISPATCH_WINDOW_CLASS,
            DISPATCH_WINDOW_STYLE,
            DISPATCH_WINDOW_EXSTYLE,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let inner = Rc::new(WindowsPlatformInner::new(dispatch_window_handle, receiver));
        unsafe {
            set_windowdata(
                dispatch_window_handle,
                WindowsWinodwDataWrapper(inner.clone()),
            );
        }
        let dispatcher = Arc::new(WindowsDispatcher::new(sender, dispatch_window_handle));

        WindowsPlatform {
            background_executor: BackgroundExecutor::new(dispatcher.clone()),
            foreground_executor: ForegroundExecutor::new(dispatcher),
            text_system: Arc::new(CosmicTextSystem::new()),
            inner,
            menu_handle: RefCell::new(None),
        }
    }
}

impl Drop for WindowsPlatform {
    fn drop(&mut self) {
        unsafe {
            OleUninitialize();
        }
    }
}

pub struct WindowsPlatformInner {
    pub dispatch_window_handle: HWND,
    pub main_receiver: flume::Receiver<Runnable>,
    windows_count: AtomicUsize,
    pub callbacks: RefCell<PlatformCallbacks>,
    pub menu_actions: RefCell<Vec<Box<dyn Action>>>,
}

impl WindowsPlatformInner {
    pub fn new(dispatch_window_handle: HWND, main_receiver: flume::Receiver<Runnable>) -> Self {
        WindowsPlatformInner {
            dispatch_window_handle,
            main_receiver,
            windows_count: AtomicUsize::new(0),
            callbacks: RefCell::new(PlatformCallbacks::default()),
            menu_actions: RefCell::new(Vec::new()),
        }
    }

    pub fn open_new_window(&self) {
        self.windows_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn close_one_window(&self) {
        if self.windows_count.load(Ordering::SeqCst) == 1 {
            unsafe {
                PostQuitMessage(0);
            }
        }
        self.windows_count.fetch_sub(1, Ordering::SeqCst);
    }
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

    fn run(&self, on_finish_launching: Box<dyn FnOnce()>) {
        on_finish_launching();
        unsafe {
            let mut msg = std::mem::zeroed();
            while GetMessageW(&mut msg, HWND::default(), 0, 0).as_bool() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        if let Some(ref mut func) = self.inner.callbacks.borrow_mut().quit {
            func();
        }
    }

    fn quit(&self) {
        unsafe {
            PostQuitMessage(0);
        }
    }

    //todo!(windows)
    fn restart(&self) {}

    //todo!(windows)
    fn activate(&self, ignoring_other_apps: bool) {}

    //todo!(windows)
    fn hide(&self) {}

    //todo!(windows)
    fn hide_other_apps(&self) {}

    //todo!(windows)
    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        WindowsDisplay::displays()
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(WindowsDisplay::new(id)))
    }

    //todo!(windows)
    fn active_window(&self) -> Option<AnyWindowHandle> {
        None
    }

    fn open_window(
        &self,
        _handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Box<dyn PlatformWindow> {
        let menu_handle = self.menu_handle.borrow().clone();
        let window = WindowsWindow::new(
            self.foreground_executor(),
            self.inner.dispatch_window_handle.into(),
            &options,
            menu_handle,
        );
        self.inner.open_new_window();

        Box::new(window)
    }

    fn open_url(&self, url: &str) {
        let url_string = url.to_string();
        println!("Open: {}", url_string);
        self.background_executor()
            .spawn(async move {
                open_target(url_string);
            })
            .detach();
    }

    fn on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>) {
        self.inner.callbacks.borrow_mut().open_urls = Some(callback);
    }

    fn prompt_for_paths(
        &self,
        options: PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        let (tx, rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                unsafe {
                    let dialog = show_openfile_dialog(options).expect("error show openfile dialog");
                    if let Ok(_) = dialog.Show(None) {
                        let Ok(items) = dialog.GetResults() else {
                            log_windows_error_with_message!("Error get result from dialog");
                            let _ = tx.send(None);
                            return;
                        };
                        let Ok(count) = items.GetCount() else {
                            log_windows_error_with_message!("Error get results count from dialog");
                            let _ = tx.send(None);
                            return;
                        };
                        let mut path_vec = Vec::new();
                        for index in 0..count {
                            let Ok(item) = items.GetItemAt(index) else {
                                log_windows_error_with_message!("Error get item dialog");
                                continue;
                            };
                            let Ok(item_string) = item.GetDisplayName(SIGDN_PARENTRELATIVEPARSING)
                            else {
                                log_windows_error_with_message!("Error parsing item name");
                                continue;
                            };
                            let Ok(path_string) = item_string.to_string() else {
                                log_windows_error_with_message!(
                                    "Error parsing item name from utf16 to string"
                                );
                                continue;
                            };
                            path_vec.push(PathBuf::from(path_string));
                        }
                        let _ = tx.send(Some(path_vec));
                    }
                }
            })
            .detach();

        rx
    }

    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        let directory = directory.to_owned();
        let (tx, rx) = oneshot::channel();
        self.foreground_executor()
            .spawn(async move {
                unsafe {
                    let dialog =
                        show_savefile_dialog(directory).expect("error open savefile dialog");
                    if let Ok(_) = dialog.Show(None) {
                        if let Ok(shell_item) = dialog.GetResult() {
                            if let Ok(file) = shell_item.GetDisplayName(SIGDN_PARENTRELATIVEPARSING)
                            {
                                let _ = tx.send(Some(PathBuf::from(file.to_string().unwrap())));
                                return;
                            }
                        }
                    }
                    let _ = tx.send(None);
                }
            })
            .detach();

        rx
    }

    fn reveal_path(&self, path: &Path) {
        let file_path = path.to_string_lossy().into_owned();
        self.background_executor()
            .spawn(async move {
                open_target(file_path);
            })
            .detach();
    }

    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().become_active = Some(callback);
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().resign_active = Some(callback);
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().quit = Some(callback);
    }

    fn on_reopen(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().reopen = Some(callback);
    }

    fn on_event(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        self.inner.callbacks.borrow_mut().event = Some(callback);
    }

    fn on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>) {
        self.inner.callbacks.borrow_mut().app_menu_action = Some(callback);
    }

    fn on_will_open_app_menu(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().will_open_app_menu = Some(callback);
    }

    fn on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>) {
        self.inner.callbacks.borrow_mut().validate_app_menu_command = Some(callback);
    }

    fn os_name(&self) -> &'static str {
        "Windows"
    }

    fn double_click_interval(&self) -> Duration {
        let millis = unsafe { GetDoubleClickTime() };
        Duration::from_millis(millis as _)
    }

    fn os_version(&self) -> Result<SemanticVersion> {
        let mut info = unsafe { std::mem::zeroed() };
        let status = unsafe { RtlGetVersion(&mut info) };
        if status == STATUS_SUCCESS {
            Ok(SemanticVersion {
                major: info.dwMajorVersion as _,
                minor: info.dwMinorVersion as _,
                patch: info.dwBuildNumber as _,
            })
        } else {
            Err(anyhow::anyhow!("{}", std::io::Error::last_os_error()))
        }
    }

    // todo!(windows)
    fn app_version(&self) -> Result<SemanticVersion> {
        Ok(SemanticVersion {
            major: 1,
            minor: 0,
            patch: 0,
        })
    }

    // todo!(windows)
    fn app_path(&self) -> Result<PathBuf> {
        unimplemented!()
    }

    //todo!(windows)
    fn set_menus(&self, menus: Vec<Menu>, keymap: &Keymap) {
        let mut actions_vec = Vec::new();
        let mut accelerator_vec = Vec::new();
        unsafe {
            let menu_bar_handle = CreateMenu().expect("unable to create menu");
            let actions_count = AtomicUsize::new(1);
            for menu in menus {
                let _ = generate_menu(
                    menu_bar_handle,
                    menu,
                    &actions_count,
                    &mut actions_vec,
                    keymap,
                    &mut accelerator_vec,
                );
            }
            let _ = self.menu_handle.borrow_mut().insert(menu_bar_handle);
            let _ = CreateAcceleratorTableW(&accelerator_vec).inspect_err(log_windows_error);
        }
        (*self.inner.menu_actions.borrow_mut()) = actions_vec;
    }

    fn local_timezone(&self) -> UtcOffset {
        let mut info = unsafe { std::mem::zeroed() };
        let ret = unsafe { GetTimeZoneInformation(&mut info) };
        if ret == TIME_ZONE_ID_INVALID {
            log_windows_error_with_message!("Unable to get local timezone");
            return UtcOffset::UTC;
        }
        // Windows treat offset as:
        // UTC = localtime + offset
        // so we add a minus here
        let hours = -info.Bias / 60;
        let minutes = -info.Bias % 60;

        UtcOffset::from_hms(hours as _, minutes as _, 0).unwrap()
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        unimplemented!()
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        unsafe {
            let handle = match style {
                CursorStyle::IBeam | CursorStyle::IBeamCursorForVerticalLayout => LoadImageW(
                    None,
                    IDC_IBEAM,
                    IMAGE_CURSOR,
                    0,
                    0,
                    LR_DEFAULTSIZE | LR_SHARED,
                ),
                CursorStyle::Crosshair => LoadImageW(
                    None,
                    IDC_CROSS,
                    IMAGE_CURSOR,
                    0,
                    0,
                    LR_DEFAULTSIZE | LR_SHARED,
                ),
                CursorStyle::PointingHand | CursorStyle::DragLink => LoadImageW(
                    None,
                    IDC_HAND,
                    IMAGE_CURSOR,
                    0,
                    0,
                    LR_DEFAULTSIZE | LR_SHARED,
                ),
                CursorStyle::ResizeLeft
                | CursorStyle::ResizeRight
                | CursorStyle::ResizeLeftRight => LoadImageW(
                    None,
                    IDC_SIZEWE,
                    IMAGE_CURSOR,
                    0,
                    0,
                    LR_DEFAULTSIZE | LR_SHARED,
                ),
                CursorStyle::ResizeUp | CursorStyle::ResizeDown | CursorStyle::ResizeUpDown => {
                    LoadImageW(
                        None,
                        IDC_SIZENS,
                        IMAGE_CURSOR,
                        0,
                        0,
                        LR_DEFAULTSIZE | LR_SHARED,
                    )
                }
                CursorStyle::OperationNotAllowed => {
                    LoadImageW(None, IDC_NO, IMAGE_CURSOR, 0, 0, LR_DEFAULTSIZE | LR_SHARED)
                }
                _ => LoadImageW(
                    None,
                    IDC_ARROW,
                    IMAGE_CURSOR,
                    0,
                    0,
                    LR_DEFAULTSIZE | LR_SHARED,
                ),
            };
            if handle.is_err() {
                log_windows_error_with_message!("Error loading cursor image");
                return;
            }
            let _ = SetCursor(HCURSOR(handle.unwrap().0));
        }
    }

    //todo!(windows)
    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    //todo!(windows)
    fn write_to_clipboard(&self, item: ClipboardItem) {
        unsafe {
            if OpenClipboard(self.inner.dispatch_window_handle)
                .inspect_err(log_windows_error)
                .is_err()
            {
                return;
            }
            if EmptyClipboard().inspect_err(log_windows_error).is_err() {
                return;
            }
            // MultiByteToWideChar(codepage, dwflags, lpmultibytestr, lpwidecharstr);
            let data_ptr = encode_wide(&item.text);
            let count = data_ptr.len() + 1;
            let global = GlobalAlloc(GMEM_MOVEABLE, count * 2).unwrap();
            let handle = GlobalLock(global);
            u_memcpy(handle as _, data_ptr.as_ptr() as _, count as _);
            let _ = GlobalUnlock(global);
            if SetClipboardData(CF_UNICODETEXT, HANDLE(global.0 as isize))
                .inspect_err(log_windows_error)
                .is_err()
            {
                return;
            }
            let _ = CloseClipboard();
        }
    }

    //todo!(windows)
    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        unsafe {
            let Ok(clipboard) = OleGetClipboard().inspect_err(log_windows_error) else {
                return None;
            };
            let config = FORMATETC {
                cfFormat: CF_UNICODETEXT as _,
                ptd: std::ptr::null_mut() as _,
                dwAspect: DVASPECT_CONTENT.0,
                lindex: -1,
                tymed: TYMED_HGLOBAL.0 as _,
            };
            let Ok(mut data) = clipboard
                .GetData(&config as _)
                .inspect_err(log_windows_error)
            else {
                return None;
            };
            let string_raw = GlobalLock(data.u.hGlobal) as *mut Vec<u16>;
            let string = String::from_utf16_lossy(&*string_raw);
            let _ = GlobalUnlock(data.u.hGlobal);
            ReleaseStgMedium(&mut data);

            Some(ClipboardItem {
                text: string,
                metadata: None,
            })
        }
    }

    // todo!(windows)
    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>> {
        unimplemented!()
    }

    // todo!(windows)
    fn read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        unimplemented!()
    }

    // todo!(windows)
    fn delete_credentials(&self, url: &str) -> Task<Result<()>> {
        unimplemented!()
    }

    // todo!(windows)
    fn window_appearance(&self) -> crate::WindowAppearance {
        crate::WindowAppearance::Light
    }
}

impl WindowsWindowBase for WindowsPlatformInner {
    unsafe fn handle_message(&self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            MAIN_DISPATCH => {
                if let Ok(runnable) = self.main_receiver.try_recv() {
                    runnable.run();
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WINDOW_CLOSE => {
                self.close_one_window();
                LRESULT(0)
            }
            MENU_ACTIONS => {
                if let Some(ref mut callback) = self.callbacks.borrow_mut().app_menu_action {
                    if let Some(action) = self.menu_actions.borrow().get(wparam.0) {
                        println!("Action index: {}", wparam.0);
                        let action = action.boxed_clone();
                        callback(&*action);
                    }
                }
                LRESULT(0)
            }
            _ => DefWindowProcW(self.dispatch_window_handle, message, wparam, lparam),
        }
    }
}

fn platform_init() -> anyhow::Result<()> {
    unsafe {
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE)
            .inspect_err(log_windows_error)?;
        OleInitialize(None).inspect_err(log_windows_error)?;
        Ok(())
    }
}

fn open_target(target: String) {
    unsafe {
        let operation = encode_wide("open");
        let file_path_vec = encode_wide(&target);
        let ret = ShellExecuteW(
            None,
            PCWSTR::from_raw(operation.as_ptr()),
            PCWSTR::from_raw(file_path_vec.as_ptr()),
            None,
            None,
            SW_SHOWDEFAULT,
        );
        if ret.0 <= 32 {
            log_windows_error_with_message!("Unable to open target");
            return;
        }
    }
}

fn show_openfile_dialog(options: PathPromptOptions) -> anyhow::Result<IFileOpenDialog> {
    unsafe {
        let dialog: IFileOpenDialog =
            CoCreateInstance(&FileOpenDialog, None, CLSCTX_ALL).inspect_err(log_windows_error)?;
        let mut config = FILEOPENDIALOGOPTIONS::default();
        if options.directories {
            config |= FOS_PICKFOLDERS;
        }
        if options.multiple {
            config |= FOS_ALLOWMULTISELECT;
        }
        let _ = dialog.SetOptions(config).inspect_err(log_windows_error)?;

        Ok(dialog)
    }
}

fn show_savefile_dialog(directory: PathBuf) -> anyhow::Result<IFileSaveDialog> {
    unsafe {
        let dialog: IFileSaveDialog =
            CoCreateInstance(&FileSaveDialog, None, CLSCTX_ALL).inspect_err(log_windows_error)?;
        let dir_str = directory.to_str().unwrap();
        println!("Target dir: {}", dir_str);
        let dir_vec = encode_wide(dir_str);
        let bind_context = CreateBindCtx(0).inspect_err(log_windows_error)?;
        let dir_shell_item: IShellItem =
            SHCreateItemFromParsingName(PCWSTR::from_raw(dir_vec.as_ptr()), &bind_context)
                .inspect_err(log_windows_error)?;
        let _ = dialog
            .SetFolder(&dir_shell_item)
            .inspect_err(log_windows_error);

        Ok(dialog)
    }
}

unsafe fn generate_menu(
    parent_handle: HMENU,
    menu: Menu,
    actions_count: &AtomicUsize,
    actions_vec: &mut Vec<Box<dyn Action>>,
    keymap: &Keymap,
    accelerator_vec: &mut Vec<ACCEL>,
) -> anyhow::Result<()> {
    let menu_handle = CreateMenu().unwrap();
    let menu_name = encode_wide(menu.name);
    AppendMenuW(
        parent_handle,
        MF_POPUP,
        menu_handle.0 as _,
        PCWSTR::from_raw(menu_name.as_ptr()),
    )
    .inspect_err(log_windows_error)?;
    if menu.items.is_empty() {
        return Ok(());
    }
    for menu_item in menu.items {
        match menu_item {
            crate::MenuItem::Separator => AppendMenuW(menu_handle, MF_SEPARATOR, 0, PCWSTR::null())
                .inspect_err(log_windows_error)?,
            crate::MenuItem::Submenu(submenu) => {
                generate_menu(
                    menu_handle,
                    submenu,
                    actions_count,
                    actions_vec,
                    keymap,
                    accelerator_vec,
                )?;
            }
            crate::MenuItem::Action {
                name,
                action,
                os_action,
            } => {
                let keystrokes = keymap
                    .bindings_for_action(action.as_ref())
                    .next()
                    .map(|binding| binding.keystrokes());
                // println!("Shortcut: {:#?}", keystrokes);

                let mut item_name = name.to_string();
                let action_index = actions_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if let Some(keystrokes) = keystrokes {
                    // TODO: deal with 2 keystrokes
                    if keystrokes.len() == 1 {
                        let keystroke = &keystrokes[0];
                        item_name.push('\t');
                        keystroke_to_menu_string(keystroke, &mut item_name);
                        let accel_table = keystroke_to_accel_table(keystroke, action_index as _);
                        accelerator_vec.push(accel_table);
                    } else {
                        // windows cant show multiple chortcuts on menu item
                        for keystroke in keystrokes.iter() {
                            keystroke_to_menu_string(keystroke, &mut item_name);
                            let accel_table =
                                keystroke_to_accel_table(keystroke, action_index as _);
                            accelerator_vec.push(accel_table);
                        }
                    }
                }
                let name_vec = encode_wide(&item_name);
                AppendMenuW(
                    menu_handle,
                    MF_STRING,
                    action_index,
                    PCWSTR::from_raw(name_vec.as_ptr()),
                )
                .inspect_err(log_windows_error)?;
                // println!("action [{}]: {:#?}", action_index, action);
                actions_vec.push(action);
            }
        }
    }

    Ok(())
}

fn keystroke_to_menu_string(keystroke: &Keystroke, menu_string: &mut String) {
    if keystroke.modifiers.control {
        let _ = write!(menu_string, "Ctrl+");
    }
    if keystroke.modifiers.shift {
        let _ = write!(menu_string, "Shift+");
    }
    if keystroke.modifiers.alt {
        let _ = write!(menu_string, "Alt+");
    }
    let _ = write!(menu_string, "{}", keystroke.key.to_uppercase());
}

fn keystroke_to_accel_table(keystroke: &Keystroke, action_index: u16) -> ACCEL {
    let mut table = ACCEL::default();
    if keystroke.modifiers.control {
        table.fVirt |= ACCEL_VIRT_FLAGS(ACCEL_FCONTROL);
    }
    if keystroke.modifiers.shift {
        table.fVirt |= ACCEL_VIRT_FLAGS(ACCEL_FSHIFT);
    }
    if keystroke.modifiers.alt {
        table.fVirt |= ACCEL_VIRT_FLAGS(ACCEL_FALT);
    }
    table.fVirt |= ACCEL_VIRT_FLAGS(ACCEL_FVIRTKEY);
    table.key = keycode_to_vkey(&keystroke.key).unwrap_or(VK_DELETE).0;
    table.cmd = action_index + 1;

    table
}

fn keycode_to_vkey(keycode: &str) -> Option<VIRTUAL_KEY> {
    let mut key = match keycode {
        "backspace" => Some(VK_BACK),
        "escape" => Some(VK_ESCAPE),
        "enter" => Some(VK_RETURN),
        "up" => Some(VK_UP),
        "down" => Some(VK_DOWN),
        "left" => Some(VK_LEFT),
        "right" => Some(VK_RIGHT),
        "pageup" => Some(VK_PRIOR),
        "pagedown" => Some(VK_NEXT),
        "home" => Some(VK_HOME),
        "end" => Some(VK_END),
        "delete" => Some(VK_DELETE),
        "f1" => Some(VK_F1),
        "f2" => Some(VK_F2),
        "f3" => Some(VK_F3),
        "f4" => Some(VK_F4),
        "f5" => Some(VK_F5),
        "f6" => Some(VK_F6),
        "f7" => Some(VK_F7),
        "f8" => Some(VK_F8),
        "f9" => Some(VK_F9),
        "f10" => Some(VK_F10),
        "f11" => Some(VK_F11),
        "f12" => Some(VK_F12),
        _ => None,
    };
    if key.is_none() {
        let Ok(this_char) = char::from_str(keycode) else {
            return None;
        };
        // TODO: is this correct?
        key = Some(VIRTUAL_KEY(this_char as u16));
        // println!("Char {} to vk {:?}", this_char, key);
    }

    key
}
