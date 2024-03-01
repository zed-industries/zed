use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    mem,
    num::NonZeroIsize,
    path::PathBuf,
    rc::{Rc, Weak},
    str::FromStr,
    sync::{
        atomic::{AtomicIsize, Ordering},
        Arc,
    },
};

use futures::channel::oneshot;
use parking_lot::Mutex;
use raw_window_handle as rwh;
use smallvec::SmallVec;
use util::ResultExt;
use windows::{
    core::{implement, IUnknown, Interface, IntoParam, Param, ReferenceType, PCWSTR},
    Win32::{
        Foundation::{
            ERROR_UNRECOVERABLE_STACK_OVERFLOW, HWND, LPARAM, LRESULT, POINTL, RECT, S_OK, WPARAM,
        },
        Graphics::Gdi::{
            GetMonitorInfoW, MonitorFromWindow, RedrawWindow, UpdateWindow, ValidateRect, HRGN,
            MONITORINFO, MONITOR_DEFAULTTONEAREST, RDW_INVALIDATE,
        },
        System::{
            Com::{IDataObject, DVASPECT_CONTENT, FORMATETC, TYMED_HGLOBAL},
            Memory::{GlobalLock, GlobalUnlock},
            Ole::{
                IDropTarget, IDropTarget_Impl, RegisterDragDrop, ReleaseStgMedium, RevokeDragDrop,
                CF_HDROP, DROPEFFECT, DROPEFFECT_LINK, DROPEFFECT_NONE,
            },
            SystemServices::MODIFIERKEYS_FLAGS,
        },
        UI::{
            Controls::{
                TaskDialogIndirect, TASKDIALOGCONFIG, TASKDIALOG_BUTTON, TASKDIALOG_FLAGS,
                TDF_USE_HICON_MAIN, TD_ERROR_ICON, TD_INFORMATION_ICON, TD_WARNING_ICON,
            },
            HiDpi::{GetDpiForMonitor, GetDpiForWindow, MDT_EFFECTIVE_DPI},
            Input::{
                Ime::{
                    ImmGetContext, ImmReleaseContext, ImmSetCompositionWindow, CFS_POINT,
                    COMPOSITIONFORM,
                },
                KeyboardAndMouse::GetMouseMovePointsEx,
            },
            Shell::{DragQueryFileA, DragQueryFileW, HDROP},
            WindowsAndMessaging::{
                CreateWindowExW, DefWindowProcW, GetClientRect, GetCursorPos, KillTimer,
                MessageBoxExW, PostMessageW, PostQuitMessage, RegisterClassExW, SetTimer,
                SetWindowTextW, ShowWindow, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, GWLP_HINSTANCE,
                HMENU, SIZE_MINIMIZED, SW_SHOW, TIMERPROC, WA_ACTIVE, WA_CLICKACTIVE, WA_INACTIVE,
                WINDOW_EX_STYLE, WINDOW_STYLE, WM_ACTIVATE, WM_CHAR, WM_CLOSE, WM_COMMAND,
                WM_DESTROY, WM_DROPFILES, WM_IME_STARTCOMPOSITION, WM_KEYDOWN, WM_KEYUP,
                WM_LBUTTONDBLCLK, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN,
                WM_MBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_PAINT, WM_RBUTTONDBLCLK,
                WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SIZE, WM_TIMER, WM_XBUTTONDBLCLK, WM_XBUTTONDOWN,
                WM_XBUTTONUP, WNDCLASSEXW, WS_EX_ACCEPTFILES, WS_MAXIMIZE, WS_OVERLAPPEDWINDOW,
                WS_POPUP, WS_VISIBLE,
            },
        },
    },
};

use crate::{
    available_monitors, encode_wide, get_module_handle, hiword, log_windows_error,
    log_windows_error_with_message, loword, parse_dropfiles, parse_keyboard_input,
    parse_mouse_button, parse_mouse_movement, parse_mouse_wheel, parse_system_key,
    platform::cross_platform::BladeRenderer, set_windowdata, Action, Bounds, DisplayId,
    ForegroundExecutor, Modifiers, Pixels, PlatformDisplay, PlatformInput, PlatformInputHandler,
    PlatformWindow, Point, Size, WindowKind, WindowOptions, WindowsWindowBase,
    WindowsWinodwDataWrapper, DRAGDROP_GET_COUNT, FILENAME_MAXLENGTH, MENU_ACTIONS, WINDOW_CLOSE,
    WINDOW_REFRESH_TIMER, WINODW_EXTRA_EXSTYLE, WINODW_REFRESH_INTERVAL, WINODW_STYLE,
};

use super::{display::WindowsDisplay, WINDOW_CLASS};

#[derive(Default)]
pub struct Callbacks {
    pub request_frame: Option<Box<dyn FnMut()>>,
    pub input: Option<Box<dyn FnMut(crate::PlatformInput) -> bool>>,
    pub active_status_change: Option<Box<dyn FnMut(bool)>>,
    pub resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    pub fullscreen: Option<Box<dyn FnMut(bool)>>,
    pub moved: Option<Box<dyn FnMut()>>,
    pub should_close: Option<Box<dyn FnMut() -> bool>>,
    pub close: Option<Box<dyn FnOnce()>>,
    pub appearance_changed: Option<Box<dyn FnMut()>>,
}

pub struct WindowsWindow {
    foreground_executor: ForegroundExecutor,
    inner: Rc<WindowsWindowinner>,
    display: Rc<dyn PlatformDisplay>,
    windows_dragdrop: IDropTarget,
}

impl WindowsWindow {
    pub fn new(
        foreground_executor: ForegroundExecutor,
        dispatch_window_handle: HWND,
        options: &WindowOptions,
        menu_handle: Option<HMENU>,
    ) -> Self {
        let mut monitor = available_monitors()
            .into_iter()
            .nth(0)
            .expect("no monitor detected!");
        let mut display = WindowsDisplay::new(DisplayId(0));
        if let Some(display_id) = options.display_id {
            monitor = available_monitors()
                .into_iter()
                .nth(display_id.0 as usize)
                .unwrap();
            // TODO: move window to target monitor
            display.display_id = display_id;
        }
        let scale_factor = monitor.scale_factor();

        let mut lpwindowname = None;
        if let Some(ref titlebar_opt) = options.titlebar {
            if let Some(ref title) = titlebar_opt.title {
                let title = encode_wide(title.as_ref());
                lpwindowname = Some(PCWSTR::from_raw(title.as_ptr()));
            }
        }
        let (style, exstyle, width, height) = parse_window_options(options);
        let raw_window_handle = <WindowsWindowinner as WindowsWindowBase>::create(
            WINDOW_CLASS,
            style,
            exstyle,
            Some(CW_USEDEFAULT),
            Some(CW_USEDEFAULT),
            Some(width.unwrap_or(CW_USEDEFAULT)),
            Some(height.unwrap_or(CW_USEDEFAULT)),
            menu_handle,
            lpwindowname,
        );
        let window_handle = RawWindow::new(raw_window_handle);
        let bounds = match options.bounds {
            crate::WindowBounds::Fullscreen | crate::WindowBounds::Maximized => Bounds {
                origin: Point::default(),
                size: window_handle.size(),
            },
            crate::WindowBounds::Fixed(bounds) => bounds.map(|p| p.0 as i32),
        };
        let gpu = Arc::new(unsafe {
            blade_graphics::Context::init_windowed(
                &window_handle,
                blade_graphics::ContextDesc {
                    validation: false,
                    capture: false,
                },
            )
            .unwrap()
        });
        let gpu_extent = blade_graphics::Extent {
            width: window_handle.size().width as _,
            height: window_handle.size().height as _,
            depth: 1,
        };
        let renderer = BladeRenderer::new(gpu, gpu_extent);
        let inner = WindowsWindowinner::new(
            dispatch_window_handle,
            options,
            scale_factor,
            window_handle,
            bounds,
            renderer,
        );
        unsafe {
            SetTimer(
                raw_window_handle,
                WINDOW_REFRESH_TIMER,
                WINODW_REFRESH_INTERVAL,
                TIMERPROC::None,
            )
        };
        let windows_dragdrop = unsafe {
            set_windowdata(raw_window_handle, WindowsWinodwDataWrapper(inner.clone()));
            let drop_target = WindowsDragDropTarget(inner.clone());
            let windows_dragdrop: IDropTarget = drop_target.into();
            RegisterDragDrop(raw_window_handle, &windows_dragdrop)
                .inspect_err(log_windows_error)
                .expect("Unable to register drawgrop op");
            windows_dragdrop
        };

        WindowsWindow {
            foreground_executor,
            display: Rc::new(display),
            inner,
            windows_dragdrop,
        }
    }
}

pub struct WindowsWindowinner {
    pub dispatch_window_handle: HWND,
    pub window_handle: RawWindow,
    bounds: RefCell<Bounds<i32>>,
    scale_factor: f32,
    pub callbacks: RefCell<Callbacks>,
    input_handler: RefCell<Option<PlatformInputHandler>>,
    pub renderer: RefCell<BladeRenderer>,
    pub modifiers: RefCell<Modifiers>,
    mouse_position: RefCell<Point<Pixels>>,
}

#[implement(IDropTarget)]
struct WindowsDragDropTarget(pub Rc<WindowsWindowinner>);

struct RawWindow {
    handle: HWND,
}

impl WindowsWindowinner {
    pub fn new(
        dispatch_window_handle: HWND,
        options: &WindowOptions,
        scale_factor: f32,
        window_handle: RawWindow,
        bounds: Bounds<i32>,
        renderer: BladeRenderer,
    ) -> Rc<Self> {
        window_handle.show();

        Rc::new(WindowsWindowinner {
            dispatch_window_handle,
            window_handle,
            callbacks: RefCell::new(Callbacks::default()),
            bounds: RefCell::new(bounds),
            input_handler: RefCell::new(None),
            scale_factor,
            renderer: RefCell::new(renderer),
            modifiers: RefCell::new(Modifiers::default()),
            mouse_position: RefCell::new(Point::default()),
        })
    }

    fn request_redraw(&self) {
        if let Some(ref mut func) = self.callbacks.borrow_mut().request_frame {
            func();
        }
    }

    fn destroy(&self) {
        self.renderer.borrow_mut().destroy();
        if let Some(func) = self.callbacks.borrow_mut().close.take() {
            func();
        }
    }

    fn update(&self) {
        unsafe {
            RedrawWindow(
                self.window_handle.hwnd(),
                None,
                HRGN::default(),
                RDW_INVALIDATE,
            );
        }
    }

    fn handle_input(&self, input: PlatformInput) {
        if let Some(ref mut func) = self.callbacks.borrow_mut().input {
            if func(input.clone()) {
                return;
            }
        }
        match input.clone() {
            PlatformInput::KeyDown(event) => {
                if let Some(mut input_handler) = self.input_handler.borrow_mut().as_mut() {
                    input_handler.replace_text_in_range(None, &event.keystroke.key);
                }
            }
            PlatformInput::KeyUp(_) => {}
            PlatformInput::ModifiersChanged(_) => {}
            PlatformInput::MouseDown(_) => {
                if let Some(ref mut input_handler) = self.callbacks.borrow_mut().input {
                    input_handler(input);
                }
            }
            PlatformInput::MouseUp(_) => {}
            PlatformInput::MouseMove(_) => {}
            PlatformInput::MouseExited(_) => {}
            PlatformInput::ScrollWheel(_) => {}
            PlatformInput::FileDrop(_) => {}
        }
    }

    fn resize(&self, width: u32, height: u32) {
        let mut resize_args = None;
        {
            let mut bounds_lock = self.bounds.borrow_mut();
            let bounds = Bounds {
                origin: bounds_lock.origin,
                size: Size {
                    width: width as _,
                    height: height as _,
                },
            };
            *bounds_lock = bounds;
            let window_size = self.window_handle.size();
            let gpu_size = blade_graphics::Extent {
                width,
                height,
                depth: 1,
            };
            let mut render = self.renderer.borrow_mut();
            if render.viewport_size() != gpu_size {
                render.update_drawable_size(crate::size(gpu_size.width as _, gpu_size.height as _));
                let content_size = Size {
                    width: render.viewport_size().width.into(),
                    height: render.viewport_size().height.into(),
                };
                resize_args = Some((content_size, self.scale_factor));
            }
        }

        if let Some((content_size, scale_factor)) = resize_args {
            if let Some(ref mut func) = self.callbacks.borrow_mut().resize {
                func(content_size, scale_factor)
            }
        }
    }

    fn set_focused(&self, focus: bool) {
        if let Some(ref mut func) = self.callbacks.borrow_mut().active_status_change {
            func(focus);
        }
    }
}

impl WindowsWindowBase for WindowsWindowinner {
    unsafe fn handle_message(&self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            WM_TIMER => {
                self.update();
                LRESULT(0)
            }
            WM_PAINT => {
                self.request_redraw();
                ValidateRect(self.window_handle.hwnd(), None);
                DefWindowProcW(self.window_handle.hwnd(), message, wparam, lparam)
            }
            WM_DESTROY => {
                self.destroy();
                let _ = PostMessageW(
                    self.dispatch_window_handle,
                    WINDOW_CLOSE,
                    WPARAM::default(),
                    LPARAM::default(),
                );
                if let Some(func) = self.callbacks.borrow_mut().close.take() {
                    func();
                }
                let _ = KillTimer(self.window_handle.hwnd(), WINDOW_REFRESH_TIMER);
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_COMMAND => {
                let action_index = loword!(wparam.0, u16) as usize;
                if action_index != 0 {
                    let _ = PostMessageW(
                        self.dispatch_window_handle,
                        MENU_ACTIONS,
                        WPARAM(action_index - 1),
                        LPARAM::default(),
                    )
                    .inspect_err(log_windows_error);
                }
                self.update();
                LRESULT(0)
            }
            WM_ACTIVATE => {
                if loword!(wparam.0, u16) as u32 & (WA_ACTIVE | WA_CLICKACTIVE) > 0 {
                    self.set_focused(true);
                } else if loword!(wparam.0, u16) as u32 & WA_INACTIVE > 0 {
                    self.set_focused(false);
                }
                LRESULT(0)
            }
            WM_KEYDOWN | WM_KEYUP => {
                let mut modifiers = self.modifiers.borrow().clone();
                if let Some(key) = parse_system_key(message, wparam, lparam, &mut modifiers) {
                    self.handle_input(key);
                    self.update();
                }
                (*self.modifiers.borrow_mut()) = modifiers;
                LRESULT(0)
            }
            WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_LBUTTONUP | WM_RBUTTONUP
            | WM_MBUTTONUP | WM_XBUTTONDOWN | WM_XBUTTONUP | WM_LBUTTONDBLCLK
            | WM_RBUTTONDBLCLK | WM_MBUTTONDBLCLK | WM_XBUTTONDBLCLK => {
                let modifiers = self.modifiers.borrow();
                let key = parse_mouse_button(message, wparam, lparam, &modifiers);
                self.handle_input(key);
                self.update();
                LRESULT(0)
            }
            WM_MOUSEMOVE => {
                let modifiers = self.modifiers.borrow().clone();
                let (new_pos, input) = parse_mouse_movement(wparam, lparam, modifiers);
                *self.mouse_position.borrow_mut() = new_pos;
                self.handle_input(input);
                LRESULT(0)
            }
            WM_CHAR => {
                let modifiers = self.modifiers.borrow();
                let keycode = parse_keyboard_input(wparam, lparam, &*modifiers);
                if let Some(key) = keycode {
                    self.handle_input(key);
                    self.update();
                }
                LRESULT(0)
            }
            WM_MOUSEWHEEL => {
                let modifiers = self.modifiers.borrow().clone();
                let input = parse_mouse_wheel(wparam, lparam, modifiers);
                self.handle_input(input);
                self.update();
                LRESULT(0)
            }
            WM_SIZE => {
                if wparam.0 as u32 == SIZE_MINIMIZED {
                    return DefWindowProcW(self.window_handle.hwnd(), message, wparam, lparam);
                }
                let width = loword!(lparam.0, u16) as u32;
                let height = hiword!(lparam.0, u16) as u32;
                self.resize(width, height);
                self.update();
                LRESULT(0)
            }
            WM_IME_STARTCOMPOSITION => {
                let ctx = ImmGetContext(None);
                let mut config = COMPOSITIONFORM::default();
                config.dwStyle = CFS_POINT;
                let mut cursor = std::mem::zeroed();
                if let Err(ref e) = GetCursorPos(&mut cursor) {
                    log_windows_error(e);
                    cursor.x = self.mouse_position.borrow().x.0 as _;
                    cursor.y = self.mouse_position.borrow().y.0 as _;
                }
                config.ptCurrentPos.x = cursor.x;
                config.ptCurrentPos.y = cursor.y;
                ImmSetCompositionWindow(ctx, &config as _);
                ImmReleaseContext(self.window_handle.hwnd(), ctx);
                self.update();
                println!(
                    "Set composition pos: ({}, {})",
                    config.ptCurrentPos.x, config.ptCurrentPos.y
                );
                LRESULT(0)
            }
            _ => DefWindowProcW(self.window_handle.hwnd(), message, wparam, lparam),
        }
    }
}

impl IDropTarget_Impl for WindowsDragDropTarget {
    fn DragEnter(
        &self,
        pdataobj: Option<&IDataObject>,
        grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        unsafe {
            let Some(idata_obj) = pdataobj else {
                log_windows_error_with_message!("no file detected while dragging");
                return Ok(());
            };
            let config = FORMATETC {
                cfFormat: CF_HDROP.0,
                ptd: std::ptr::null_mut() as _,
                dwAspect: DVASPECT_CONTENT.0,
                lindex: -1,
                tymed: TYMED_HGLOBAL.0 as _,
            };
            let mut paths = SmallVec::<[PathBuf; 2]>::new();
            if idata_obj.QueryGetData(&config as _) == S_OK {
                *pdweffect = DROPEFFECT_LINK;
                let Ok(mut idata) = idata_obj.GetData(&config as _) else {
                    return Ok(());
                };
                // idata_obj.query(iid, interface)
                if idata.u.hGlobal.is_invalid() {
                    return Ok(());
                }
                let hdrop = idata.u.hGlobal.0 as *mut HDROP;
                // let mut temp_buffer = [0u16; FILENAME_MAXLENGTH];
                let file_count = DragQueryFileW(*hdrop, DRAGDROP_GET_COUNT, None);
                for file_index in 0..file_count {
                    let mut buffer = [0u16; FILENAME_MAXLENGTH];
                    let filename_length = DragQueryFileW(*hdrop, file_index, None) as usize;
                    let ret = DragQueryFileW(*hdrop, file_index, Some(&mut buffer));
                    if ret == 0 {
                        log_windows_error_with_message!("unable to read file name");
                        continue;
                    }
                    if let Ok(file_name) = String::from_utf16(&buffer[0..filename_length]) {
                        if let Ok(path) = PathBuf::from_str(&file_name) {
                            paths.push(path);
                        }
                    }
                }
                ReleaseStgMedium(&mut idata);
            } else {
                *pdweffect = DROPEFFECT_NONE;
            }

            let input = PlatformInput::FileDrop(crate::FileDropEvent::Entered {
                position: Point {
                    x: Pixels(pt.x as _),
                    y: Pixels(pt.y as _),
                },
                paths: crate::ExternalPaths(paths),
            });
            self.0.handle_input(input);
        }
        Ok(())
    }

    fn DragOver(
        &self,
        grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        let input = PlatformInput::FileDrop(crate::FileDropEvent::Pending {
            position: Point {
                x: Pixels(pt.x as _),
                y: Pixels(pt.y as _),
            },
        });
        self.0.handle_input(input);

        Ok(())
    }

    fn DragLeave(&self) -> windows::core::Result<()> {
        let input = PlatformInput::FileDrop(crate::FileDropEvent::Exited);
        self.0.handle_input(input);

        Ok(())
    }

    fn Drop(
        &self,
        pdataobj: ::core::option::Option<&IDataObject>,
        grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        let input = PlatformInput::FileDrop(crate::FileDropEvent::Submit {
            position: Point {
                x: Pixels(pt.x as _),
                y: Pixels(pt.y as _),
            },
        });
        self.0.handle_input(input);

        Ok(())
    }
}

impl rwh::HasWindowHandle for WindowsWindow {
    fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
        Ok(unsafe { rwh::WindowHandle::borrow_raw(self.inner.window_handle.raw_wh()) })
    }
}

impl rwh::HasDisplayHandle for WindowsWindow {
    fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
        Ok(unsafe { rwh::DisplayHandle::borrow_raw(self.inner.window_handle.raw_dh()) })
    }
}

impl PlatformWindow for WindowsWindow {
    fn bounds(&self) -> crate::WindowBounds {
        let block = self.inner.bounds.borrow();
        crate::WindowBounds::Fixed(block.map(|v| crate::GlobalPixels(v as f32)))
    }

    fn content_size(&self) -> Size<Pixels> {
        let size = self.inner.renderer.borrow().viewport_size();
        Size {
            width: size.width.into(),
            height: size.height.into(),
        }
    }

    fn scale_factor(&self) -> f32 {
        self.inner.scale_factor
    }

    fn titlebar_height(&self) -> Pixels {
        todo!()
    }

    fn appearance(&self) -> crate::WindowAppearance {
        crate::WindowAppearance::Light
    }

    fn display(&self) -> Rc<dyn crate::PlatformDisplay> {
        Rc::clone(&self.display)
    }

    fn mouse_position(&self) -> crate::Point<Pixels> {
        self.inner.mouse_position.borrow().clone()
    }

    fn modifiers(&self) -> crate::Modifiers {
        self.inner.modifiers.borrow().clone()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_input_handler(&mut self, input_handler: crate::PlatformInputHandler) {
        (*self.inner.input_handler.borrow_mut()) = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<crate::PlatformInputHandler> {
        self.inner.input_handler.borrow_mut().take()
    }

    fn prompt(
        &self,
        level: crate::PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> futures::channel::oneshot::Receiver<usize> {
        let (done_tx, done_rx) = oneshot::channel();
        let mut config;
        unsafe {
            config = std::mem::zeroed::<TASKDIALOGCONFIG>();
            config.cbSize = std::mem::size_of::<TASKDIALOGCONFIG>() as _;
            config.hwndParent = self.inner.window_handle.hwnd();
            let title;
            let main_icon;
            match level {
                crate::PromptLevel::Info => {
                    title = windows::core::w!("Info");
                    main_icon = TD_INFORMATION_ICON;
                }
                crate::PromptLevel::Warning => {
                    title = windows::core::w!("Warning");
                    main_icon = TD_WARNING_ICON;
                }
                crate::PromptLevel::Critical => {
                    title = windows::core::w!("Critical");
                    main_icon = TD_ERROR_ICON;
                }
            };
            config.pszWindowTitle = title;
            config.Anonymous1.pszMainIcon = main_icon;
            let instruction = encode_wide(msg);
            config.pszMainInstruction = PCWSTR::from_raw(instruction.as_ptr());
            let hints_encoded;
            if let Some(hints) = detail {
                hints_encoded = encode_wide(hints);
                config.pszContent = PCWSTR::from_raw(hints_encoded.as_ptr());
            };
            let mut buttons = Vec::new();
            let mut btn_encoded = Vec::new();
            for (index, btn_string) in answers.iter().enumerate() {
                let encoded = encode_wide(btn_string);
                buttons.push(TASKDIALOG_BUTTON {
                    nButtonID: index as _,
                    pszButtonText: PCWSTR::from_raw(encoded.as_ptr()),
                });
                btn_encoded.push(encoded);
            }
            config.cButtons = buttons.len() as _;
            config.pButtons = buttons.as_ptr();
        }
        self.foreground_executor
            .spawn(async move {
                unsafe {
                    config.pfCallback = None;
                    let mut res = std::mem::zeroed();
                    let _ = TaskDialogIndirect(&config, Some(&mut res), None, None)
                        .inspect_err(log_windows_error);

                    let _ = done_tx.send(res as usize);
                }
            })
            .detach();

        done_rx
    }

    fn activate(&self) {}

    fn set_title(&mut self, title: &str) {
        self.inner.window_handle.set_title(title);
    }

    fn set_edited(&mut self, edited: bool) {}

    fn show_character_palette(&self) {
        todo!()
    }

    fn minimize(&self) {
        // TODO:
    }

    fn zoom(&self) {
        todo!()
    }

    fn toggle_full_screen(&self) {
        todo!()
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(crate::PlatformInput) -> bool>) {
        self.inner.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.inner.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_fullscreen(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().fullscreen = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.inner.callbacks.borrow_mut().should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.inner.callbacks.borrow_mut().close = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    fn is_topmost_for_position(&self, position: crate::Point<Pixels>) -> bool {
        todo!()
    }

    fn draw(&self, scene: &crate::Scene) {
        self.inner.renderer.borrow_mut().draw(scene);
    }

    fn sprite_atlas(&self) -> std::sync::Arc<dyn crate::PlatformAtlas> {
        self.inner.renderer.borrow().sprite_atlas().clone()
    }
}

impl RawWindow {
    pub fn new(handle: HWND) -> Self {
        RawWindow { handle }
    }

    pub fn hwnd(&self) -> HWND {
        self.handle
    }

    pub fn set_title(&self, title: &str) {
        let title_vec = encode_wide(title);
        unsafe {
            SetWindowTextW(self.hwnd(), PCWSTR::from_raw(title_vec.as_ptr())).log_err();
        }
    }

    pub fn set_data(&self, data: Rc<WindowsWindowinner>) {
        unsafe {
            set_windowdata(self.hwnd(), data);
        }
    }

    pub fn show(&self) {
        unsafe {
            // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
            // UpdateWindow(self.hwnd());
            ShowWindow(self.hwnd(), SW_SHOW);
            println!("Show window Error: {:?}", std::io::Error::last_os_error());
        }
    }

    pub fn size(&self) -> Size<i32> {
        let mut rect: RECT = unsafe { mem::zeroed() };
        unsafe { GetClientRect(self.hwnd(), &mut rect).log_err() };
        Size {
            width: (rect.right - rect.left) as i32,
            height: (rect.bottom - rect.top) as i32,
        }
    }

    pub fn scale_factor(&self) -> f32 {
        (self.get_dpi() as f32) / (96 as f32)
    }

    fn get_dpi(&self) -> u32 {
        unsafe {
            let res = GetDpiForWindow(self.hwnd());
            if res > 0 {
                return res;
            }
            // failed
            let monitor = { MonitorFromWindow(self.hwnd(), MONITOR_DEFAULTTONEAREST) };
            if monitor.is_invalid() {
                return 96;
            }

            let mut dpi_x = 0;
            let mut dpi_y = 0;
            if GetDpiForMonitor(monitor, MDT_EFFECTIVE_DPI, &mut dpi_x, &mut dpi_y).is_ok() {
                dpi_x
            } else {
                96
            }
        }
    }

    pub fn raw_wh(&self) -> rwh::RawWindowHandle {
        let mut window_handle = rwh::Win32WindowHandle::new(unsafe {
            // SAFETY: Handle will never be zero.
            std::num::NonZeroIsize::new_unchecked(self.hwnd().0)
        });
        let hinstance = unsafe {
            windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW(self.hwnd(), GWLP_HINSTANCE)
        };
        window_handle.hinstance = std::num::NonZeroIsize::new(hinstance);

        rwh::RawWindowHandle::Win32(window_handle)
    }

    pub fn raw_dh(&self) -> rwh::RawDisplayHandle {
        rwh::RawDisplayHandle::Windows(rwh::WindowsDisplayHandle::new())
    }
}

impl Drop for RawWindow {
    fn drop(&mut self) {
        unsafe {
            let _ = RevokeDragDrop(self.handle);
        }
    }
}

unsafe impl blade_rwh::HasRawWindowHandle for RawWindow {
    fn raw_window_handle(&self) -> blade_rwh::RawWindowHandle {
        let mut window_handle = blade_rwh::Win32WindowHandle::empty();
        window_handle.hwnd = self.hwnd().0 as *mut _;
        let hinstance = unsafe {
            windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW(self.hwnd(), GWLP_HINSTANCE)
        };
        window_handle.hinstance = hinstance as *mut _;

        blade_rwh::RawWindowHandle::Win32(window_handle)
    }
}

unsafe impl blade_rwh::HasRawDisplayHandle for RawWindow {
    fn raw_display_handle(&self) -> blade_rwh::RawDisplayHandle {
        blade_rwh::RawDisplayHandle::Windows(blade_rwh::WindowsDisplayHandle::empty())
    }
}

fn parse_window_options(
    options: &WindowOptions,
) -> (WINDOW_STYLE, WINDOW_EX_STYLE, Option<i32>, Option<i32>) {
    let mut style = WINODW_STYLE;
    // https://learn.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles
    let exstyle = WINDOW_EX_STYLE::default() | WINODW_EXTRA_EXSTYLE;
    let mut width = None;
    let mut height = None;
    if options.show {
        style |= WS_VISIBLE
    }
    if options.kind == WindowKind::PopUp {
        style |= WS_POPUP;
    }
    match options.bounds {
        crate::WindowBounds::Fullscreen => style &= !WS_OVERLAPPEDWINDOW,
        crate::WindowBounds::Maximized => style |= WS_MAXIMIZE,
        crate::WindowBounds::Maximized => {}
        crate::WindowBounds::Fixed(bounds) => {
            width = Some(bounds.size.width.0 as _);
            height = Some(bounds.size.height.0 as _);
        }
    }

    (style, exstyle, width, height)
}
