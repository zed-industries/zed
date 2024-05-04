#![deny(unsafe_op_in_unsafe_fn)]

use std::{
    cell::RefCell,
    num::NonZeroIsize,
    path::PathBuf,
    rc::{Rc, Weak},
    str::FromStr,
    sync::{atomic::AtomicUsize, Arc, Once},
    time::{Duration, Instant},
};

use ::util::ResultExt;
use anyhow::Context;
use async_task::Runnable;
use blade_graphics as gpu;
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use parking_lot::RwLock;
use raw_window_handle as rwh;
use smallvec::SmallVec;
use std::result::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{Com::*, LibraryLoader::*, Ole::*, SystemServices::*},
        UI::{Controls::*, HiDpi::*, Input::KeyboardAndMouse::*, Shell::*, WindowsAndMessaging::*},
    },
};

use crate::platform::blade::{BladeRenderer, BladeSurfaceConfig};
use crate::*;

pub(crate) struct WindowsWindowState {
    pub(crate) hwnd: HWND,
    pub(crate) executor: ForegroundExecutor,
    pub(crate) origin: Point<DevicePixels>,
    pub(crate) physical_size: Size<DevicePixels>,
    pub(crate) scale_factor: f32,
    pub(crate) input_handler: Option<PlatformInputHandler>,
    pub(crate) renderer: BladeRenderer,
    pub(crate) callbacks: Callbacks,
    pub(crate) handle: AnyWindowHandle,
    pub(crate) hide_title_bar: bool,
    pub(crate) display: WindowsDisplay,
    pub(crate) click_state: ClickState,
    pub(crate) mouse_wheel_settings: MouseWheelSettings,
    pub(crate) fullscreen: Option<StyleAndBounds>,
    pub(crate) current_cursor: HCURSOR,
    main_receiver: flume::Receiver<Runnable>,
}

impl WindowsWindowState {
    fn new(
        hwnd: HWND,
        cs: &CREATESTRUCTW,
        handle: AnyWindowHandle,
        hide_title_bar: bool,
        display: WindowsDisplay,
        transparent: bool,
        executor: ForegroundExecutor,
        main_receiver: flume::Receiver<Runnable>,
        mouse_wheel_settings: MouseWheelSettings,
        current_cursor: HCURSOR,
    ) -> Rc<RefCell<Self>> {
        let monitor_dpi = unsafe { GetDpiForWindow(hwnd) } as f32;
        let origin = point(cs.x.into(), cs.y.into());
        let physical_size = size(cs.cx.into(), cs.cy.into());
        let scale_factor = monitor_dpi / USER_DEFAULT_SCREEN_DPI as f32;
        let input_handler = None;
        struct RawWindow {
            hwnd: isize,
        }
        impl rwh::HasWindowHandle for RawWindow {
            fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
                Ok(unsafe {
                    let hwnd = NonZeroIsize::new_unchecked(self.hwnd);
                    let mut handle = rwh::Win32WindowHandle::new(hwnd);
                    let hinstance = get_window_long(HWND(self.hwnd), GWLP_HINSTANCE);
                    handle.hinstance = NonZeroIsize::new(hinstance);
                    rwh::WindowHandle::borrow_raw(handle.into())
                })
            }
        }
        impl rwh::HasDisplayHandle for RawWindow {
            fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
                let handle = rwh::WindowsDisplayHandle::new();
                Ok(unsafe { rwh::DisplayHandle::borrow_raw(handle.into()) })
            }
        }

        let raw = RawWindow { hwnd: hwnd.0 };
        let gpu = Arc::new(
            unsafe {
                gpu::Context::init_windowed(
                    &raw,
                    gpu::ContextDesc {
                        validation: false,
                        capture: false,
                        overlay: false,
                    },
                )
            }
            .unwrap(),
        );
        let config = BladeSurfaceConfig {
            size: gpu::Extent::default(),
            transparent,
        };
        let renderer = BladeRenderer::new(gpu, config);
        let callbacks = Callbacks::default();
        let click_state = ClickState::new();
        let fullscreen = None;

        Rc::new(RefCell::new(Self {
            hwnd,
            executor,
            origin,
            physical_size,
            scale_factor,
            input_handler,
            renderer,
            callbacks,
            handle,
            hide_title_bar,
            display,
            click_state,
            mouse_wheel_settings,
            fullscreen,
            current_cursor,
            main_receiver,
        }))
    }

    pub(crate) fn is_maximized(&self) -> bool {
        !self.is_fullscreen() && unsafe { IsZoomed(self.hwnd) }.as_bool()
    }

    fn is_minimized(&self) -> bool {
        unsafe { IsIconic(self.hwnd) }.as_bool()
    }

    pub(crate) fn is_fullscreen(&self) -> bool {
        // let fullscreen = self.fullscreen.take();
        // let is_fullscreen = fullscreen.is_some();
        // self.fullscreen.insert(fullscreen);
        // is_fullscreen
        self.fullscreen.is_some()
    }

    pub fn bounds(&self) -> Bounds<DevicePixels> {
        Bounds {
            origin: self.origin,
            size: self.physical_size,
        }
    }

    /// get the logical size of the app's drawable area.
    ///
    /// Currently, GPUI uses logical size of the app to handle mouse interactions (such as
    /// whether the mouse collides with other elements of GPUI).
    pub fn content_size(&self) -> Size<Pixels> {
        logical_size(self.physical_size, self.scale_factor)
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    pub(crate) fn title_bar_padding(&self) -> Pixels {
        // using USER_DEFAULT_SCREEN_DPI because GPUI handles the scale with the scale factor
        let padding = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, USER_DEFAULT_SCREEN_DPI) };
        px(padding as f32)
    }

    pub(crate) fn title_bar_top_offset(&self) -> Pixels {
        if self.is_maximized() {
            self.title_bar_padding() * 2
        } else {
            px(0.)
        }
    }

    pub(crate) fn title_bar_height(&self) -> Pixels {
        // todo(windows) this is hard set to match the ui title bar
        //               in the future the ui title bar component will report the size
        px(32.) + self.title_bar_top_offset()
    }

    pub(crate) fn caption_button_width(&self) -> Pixels {
        // todo(windows) this is hard set to match the ui title bar
        //               in the future the ui title bar component will report the size
        px(36.)
    }

    pub(crate) fn get_titlebar_rect(&self) -> anyhow::Result<RECT> {
        let height = self.title_bar_height();
        let mut rect = RECT::default();
        unsafe { GetClientRect(self.hwnd, &mut rect) }?;
        rect.bottom = rect.top + ((height.0 * self.scale_factor).round() as i32);
        Ok(rect)
    }

    pub(crate) fn run_foreground_tasks(&self) {
        for runnable in self.main_receiver.drain() {
            runnable.run();
        }
    }

    fn handle_drag_drop(&mut self, input: PlatformInput) {
        let Some(ref mut func) = self.callbacks.input else {
            return;
        };
        func(input);
    }
}

#[derive(Default)]
pub(crate) struct Callbacks {
    pub(crate) request_frame: Option<Box<dyn FnMut()>>,
    pub(crate) input: Option<Box<dyn FnMut(crate::PlatformInput) -> DispatchEventResult>>,
    pub(crate) active_status_change: Option<Box<dyn FnMut(bool)>>,
    pub(crate) resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    pub(crate) fullscreen: Option<Box<dyn FnMut(bool)>>,
    pub(crate) moved: Option<Box<dyn FnMut()>>,
    pub(crate) should_close: Option<Box<dyn FnMut() -> bool>>,
    pub(crate) close: Option<Box<dyn FnOnce()>>,
    pub(crate) appearance_changed: Option<Box<dyn FnMut()>>,
}

pub(crate) struct WindowsWindow {
    state: Rc<RefCell<WindowsWindowState>>,
    drag_drop_handler: IDropTarget,
}

struct WindowCreateContext {
    inner: Option<Rc<RefCell<WindowsWindowState>>>,
    handle: AnyWindowHandle,
    hide_title_bar: bool,
    display: WindowsDisplay,
    transparent: bool,
    executor: ForegroundExecutor,
    main_receiver: flume::Receiver<Runnable>,
    mouse_wheel_settings: MouseWheelSettings,
    current_cursor: HCURSOR,
}

impl WindowsWindow {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        options: WindowParams,
        icon: HICON,
        executor: ForegroundExecutor,
        main_receiver: flume::Receiver<Runnable>,
        mouse_wheel_settings: MouseWheelSettings,
        current_cursor: HCURSOR,
    ) -> Self {
        let classname = register_wnd_class(icon);
        let hide_title_bar = options
            .titlebar
            .as_ref()
            .map(|titlebar| titlebar.appears_transparent)
            .unwrap_or(false);
        let windowname = HSTRING::from(
            options
                .titlebar
                .as_ref()
                .and_then(|titlebar| titlebar.title.as_ref())
                .map(|title| title.as_ref())
                .unwrap_or(""),
        );
        let dwstyle = WS_THICKFRAME | WS_SYSMENU | WS_MAXIMIZEBOX | WS_MINIMIZEBOX;
        let x = options.bounds.origin.x.0;
        let y = options.bounds.origin.y.0;
        let nwidth = options.bounds.size.width.0;
        let nheight = options.bounds.size.height.0;
        let hwndparent = HWND::default();
        let hmenu = HMENU::default();
        let hinstance = get_module_handle();
        let mut context = WindowCreateContext {
            inner: None,
            handle,
            hide_title_bar,
            // todo(windows) move window to target monitor
            // options.display_id
            display: WindowsDisplay::primary_monitor().unwrap(),
            transparent: options.window_background != WindowBackgroundAppearance::Opaque,
            executor,
            main_receiver,
            mouse_wheel_settings,
            current_cursor,
        };
        let lpparam = Some(&context as *const _ as *const _);
        let raw_hwnd = unsafe {
            CreateWindowExW(
                WS_EX_APPWINDOW,
                classname,
                &windowname,
                dwstyle,
                x,
                y,
                nwidth,
                nheight,
                hwndparent,
                hmenu,
                hinstance,
                lpparam,
            )
        };
        let drag_drop_handler = {
            let inner = context.inner.as_ref().unwrap();
            let handler = WindowsDragDropHandler(Rc::clone(inner));
            let drag_drop_handler: IDropTarget = handler.into();
            unsafe {
                RegisterDragDrop(raw_hwnd, &drag_drop_handler)
                    .expect("unable to register drag-drop event")
            };
            drag_drop_handler
        };
        let wnd = Self {
            state: context.inner.unwrap(),
            drag_drop_handler,
        };

        unsafe { ShowWindow(raw_hwnd, SW_SHOW) };

        wnd
    }
}

impl rwh::HasWindowHandle for WindowsWindow {
    fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
        let raw = rwh::Win32WindowHandle::new(unsafe {
            NonZeroIsize::new_unchecked(self.state.borrow().hwnd.0)
        })
        .into();
        Ok(unsafe { rwh::WindowHandle::borrow_raw(raw) })
    }
}

// todo(windows)
impl rwh::HasDisplayHandle for WindowsWindow {
    fn display_handle(&self) -> Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
        unimplemented!()
    }
}

impl Drop for WindowsWindow {
    fn drop(&mut self) {
        unsafe {
            let mut lock = self.state.as_ref().borrow_mut();
            let _ = RevokeDragDrop(lock.hwnd);
            lock.renderer.destroy();
        }
    }
}

impl PlatformWindow for WindowsWindow {
    fn bounds(&self) -> Bounds<DevicePixels> {
        self.state.as_ref().borrow().bounds()
    }

    fn is_maximized(&self) -> bool {
        self.state.as_ref().borrow().is_maximized()
    }

    fn is_minimized(&self) -> bool {
        self.state.as_ref().borrow().is_minimized()
    }

    /// get the logical size of the app's drawable area.
    ///
    /// Currently, GPUI uses logical size of the app to handle mouse interactions (such as
    /// whether the mouse collides with other elements of GPUI).
    fn content_size(&self) -> Size<Pixels> {
        self.state.as_ref().borrow().content_size()
    }

    fn scale_factor(&self) -> f32 {
        self.state.as_ref().borrow().scale_factor()
    }

    // todo(windows)
    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
    }

    fn display(&self) -> Rc<dyn PlatformDisplay> {
        let display = self.state.as_ref().borrow().display.clone();
        Rc::new(display)
    }

    fn mouse_position(&self) -> Point<Pixels> {
        let lock = self.state.as_ref().borrow();
        let handle = lock.hwnd;
        let scale_factor = lock.scale_factor();
        drop(lock);
        let point = unsafe {
            let mut point: POINT = std::mem::zeroed();
            GetCursorPos(&mut point)
                .context("unable to get cursor position")
                .log_err();
            ScreenToClient(handle, &mut point);
            point
        };
        logical_point(point.x as f32, point.y as f32, scale_factor)
    }

    // todo(windows)
    fn modifiers(&self) -> Modifiers {
        Modifiers::none()
    }

    // todo(windows)
    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        let _ = self
            .state
            .as_ref()
            .borrow_mut()
            .input_handler
            .insert(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.state.as_ref().borrow_mut().input_handler.take()
    }

    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> Option<Receiver<usize>> {
        let (done_tx, done_rx) = oneshot::channel();
        let msg = msg.to_string();
        let detail_string = match detail {
            Some(info) => Some(info.to_string()),
            None => None,
        };
        let answers = answers.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let lock = self.state.as_ref().borrow();
        let handle = lock.hwnd;
        let excutor = lock.executor.clone();
        drop(lock);
        excutor
            .spawn(async move {
                unsafe {
                    let mut config;
                    config = std::mem::zeroed::<TASKDIALOGCONFIG>();
                    config.cbSize = std::mem::size_of::<TASKDIALOGCONFIG>() as _;
                    config.hwndParent = handle;
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
                        crate::PromptLevel::Critical | crate::PromptLevel::Destructive => {
                            title = windows::core::w!("Critical");
                            main_icon = TD_ERROR_ICON;
                        }
                    };
                    config.pszWindowTitle = title;
                    config.Anonymous1.pszMainIcon = main_icon;
                    let instruction = msg.encode_utf16().chain(Some(0)).collect_vec();
                    config.pszMainInstruction = PCWSTR::from_raw(instruction.as_ptr());
                    let hints_encoded;
                    if let Some(ref hints) = detail_string {
                        hints_encoded = hints.encode_utf16().chain(Some(0)).collect_vec();
                        config.pszContent = PCWSTR::from_raw(hints_encoded.as_ptr());
                    };
                    let mut buttons = Vec::new();
                    let mut btn_encoded = Vec::new();
                    for (index, btn_string) in answers.iter().enumerate() {
                        let encoded = btn_string.encode_utf16().chain(Some(0)).collect_vec();
                        buttons.push(TASKDIALOG_BUTTON {
                            nButtonID: index as _,
                            pszButtonText: PCWSTR::from_raw(encoded.as_ptr()),
                        });
                        btn_encoded.push(encoded);
                    }
                    config.cButtons = buttons.len() as _;
                    config.pButtons = buttons.as_ptr();

                    config.pfCallback = None;
                    let mut res = std::mem::zeroed();
                    let _ = TaskDialogIndirect(&config, Some(&mut res), None, None)
                        .inspect_err(|e| log::error!("unable to create task dialog: {}", e));

                    let _ = done_tx.send(res as usize);
                }
            })
            .detach();

        Some(done_rx)
    }

    fn activate(&self) {
        let handle = self.state.as_ref().borrow().hwnd;
        unsafe { SetActiveWindow(handle) };
        unsafe { SetFocus(handle) };
        unsafe { SetForegroundWindow(handle) };
    }

    fn is_active(&self) -> bool {
        self.state.as_ref().borrow().hwnd == unsafe { GetActiveWindow() }
    }

    // todo(windows)
    fn set_title(&mut self, title: &str) {
        unsafe { SetWindowTextW(self.state.as_ref().borrow().hwnd, &HSTRING::from(title)) }
            .inspect_err(|e| log::error!("Set title failed: {e}"))
            .ok();
    }

    fn set_app_id(&mut self, _app_id: &str) {}

    fn set_background_appearance(&mut self, background_appearance: WindowBackgroundAppearance) {
        self.inner
            .renderer
            .borrow_mut()
            .update_transparency(background_appearance != WindowBackgroundAppearance::Opaque);
    }

    // todo(windows)
    fn set_edited(&mut self, _edited: bool) {}

    // todo(windows)
    fn show_character_palette(&self) {}

    fn minimize(&self) {
        unsafe { ShowWindowAsync(self.state.as_ref().borrow().hwnd, SW_MINIMIZE) };
    }

    fn zoom(&self) {
        unsafe { ShowWindowAsync(self.state.as_ref().borrow().hwnd, SW_MAXIMIZE) };
    }

    fn toggle_fullscreen(&self) {
        let executor = self.state.borrow().executor.clone();
        let window_state = self.state.clone();
        executor
            .spawn(async move {
                let mut lock = window_state.as_ref().borrow_mut();
                let StyleAndBounds {
                    style,
                    x,
                    y,
                    cx,
                    cy,
                } = if let Some(state) = lock.fullscreen.take() {
                    state
                } else {
                    let style = WINDOW_STYLE(unsafe { get_window_long(lock.hwnd, GWL_STYLE) } as _);
                    let mut rc = RECT::default();
                    unsafe { GetWindowRect(lock.hwnd, &mut rc) }.log_err();
                    let _ = lock.fullscreen.insert(StyleAndBounds {
                        style,
                        x: rc.left,
                        y: rc.top,
                        cx: rc.right - rc.left,
                        cy: rc.bottom - rc.top,
                    });
                    let style = style
                        & !(WS_THICKFRAME
                            | WS_SYSMENU
                            | WS_MAXIMIZEBOX
                            | WS_MINIMIZEBOX
                            | WS_CAPTION);
                    let bounds = lock.display.bounds();
                    StyleAndBounds {
                        style,
                        x: bounds.left().0,
                        y: bounds.top().0,
                        cx: bounds.size.width.0,
                        cy: bounds.size.height.0,
                    }
                };
                unsafe { set_window_long(lock.hwnd, GWL_STYLE, style.0 as isize) };
                unsafe {
                    SetWindowPos(
                        lock.hwnd,
                        HWND::default(),
                        x,
                        y,
                        cx,
                        cy,
                        SWP_FRAMECHANGED | SWP_NOACTIVATE | SWP_NOZORDER,
                    )
                }
                .log_err();
            })
            .detach();
    }

    fn is_fullscreen(&self) -> bool {
        self.state.as_ref().borrow().is_fullscreen()
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.state.as_ref().borrow_mut().callbacks.request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.state.as_ref().borrow_mut().callbacks.input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.state
            .as_ref()
            .borrow_mut()
            .callbacks
            .active_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.state.as_ref().borrow_mut().callbacks.resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.state.as_ref().borrow_mut().callbacks.moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.state.as_ref().borrow_mut().callbacks.should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.state.as_ref().borrow_mut().callbacks.close = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.inner
            .as_ref()
            .borrow_mut()
            .callbacks
            .appearance_changed = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        self.state.as_ref().borrow_mut().renderer.draw(scene)
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.state.as_ref().borrow().renderer.sprite_atlas().clone()
    }

    fn get_raw_handle(&self) -> HWND {
        self.state.as_ref().borrow().hwnd
    }
}

#[implement(IDropTarget)]
struct WindowsDragDropHandler(pub Rc<RefCell<WindowsWindowState>>);

#[allow(non_snake_case)]
impl IDropTarget_Impl for WindowsDragDropHandler {
    fn DragEnter(
        &self,
        pdataobj: Option<&IDataObject>,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        unsafe {
            let Some(idata_obj) = pdataobj else {
                log::info!("no dragging file or directory detected");
                return Ok(());
            };
            let config = FORMATETC {
                cfFormat: CF_HDROP.0,
                ptd: std::ptr::null_mut() as _,
                dwAspect: DVASPECT_CONTENT.0,
                lindex: -1,
                tymed: TYMED_HGLOBAL.0 as _,
            };
            if idata_obj.QueryGetData(&config as _) == S_OK {
                *pdweffect = DROPEFFECT_LINK;
                let Some(mut idata) = idata_obj.GetData(&config as _).log_err() else {
                    return Ok(());
                };
                if idata.u.hGlobal.is_invalid() {
                    return Ok(());
                }
                let hdrop = idata.u.hGlobal.0 as *mut HDROP;
                let mut paths = SmallVec::<[PathBuf; 2]>::new();
                let file_count = DragQueryFileW(*hdrop, DRAGDROP_GET_FILES_COUNT, None);
                for file_index in 0..file_count {
                    let filename_length = DragQueryFileW(*hdrop, file_index, None) as usize;
                    let mut buffer = vec![0u16; filename_length + 1];
                    let ret = DragQueryFileW(*hdrop, file_index, Some(buffer.as_mut_slice()));
                    if ret == 0 {
                        log::error!("unable to read file name");
                        continue;
                    }
                    if let Some(file_name) =
                        String::from_utf16(&buffer[0..filename_length]).log_err()
                    {
                        if let Some(path) = PathBuf::from_str(&file_name).log_err() {
                            paths.push(path);
                        }
                    }
                }
                ReleaseStgMedium(&mut idata);
                let mut cursor_position = POINT { x: pt.x, y: pt.y };
                ScreenToClient(self.0.hwnd, &mut cursor_position);
                let scale_factor = self.0.scale_factor.get();
                let input = PlatformInput::FileDrop(FileDropEvent::Entered {
                    position: logical_point(
                        cursor_position.x as f32,
                        cursor_position.y as f32,
                        scale_factor,
                    ),
                    paths: ExternalPaths(paths),
                });
                self.0.as_ref().borrow_mut().handle_drag_drop(input);
            } else {
                *pdweffect = DROPEFFECT_NONE;
            }
        }
        Ok(())
    }

    fn DragOver(
        &self,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        _pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        let mut cursor_position = POINT { x: pt.x, y: pt.y };
        unsafe {
            ScreenToClient(self.0.hwnd, &mut cursor_position);
        }
        let scale_factor = self.0.scale_factor.get();
        let input = PlatformInput::FileDrop(FileDropEvent::Pending {
            position: logical_point(
                cursor_position.x as f32,
                cursor_position.y as f32,
                scale_factor,
            ),
        });
        self.0.as_ref().borrow_mut().handle_drag_drop(input);

        Ok(())
    }

    fn DragLeave(&self) -> windows::core::Result<()> {
        let input = PlatformInput::FileDrop(FileDropEvent::Exited);
        self.0.as_ref().borrow_mut().handle_drag_drop(input);

        Ok(())
    }

    fn Drop(
        &self,
        _pdataobj: Option<&IDataObject>,
        _grfkeystate: MODIFIERKEYS_FLAGS,
        pt: &POINTL,
        _pdweffect: *mut DROPEFFECT,
    ) -> windows::core::Result<()> {
        let mut cursor_position = POINT { x: pt.x, y: pt.y };
        unsafe {
            ScreenToClient(self.0.hwnd, &mut cursor_position);
        }
        let scale_factor = self.0.scale_factor.get();
        let input = PlatformInput::FileDrop(FileDropEvent::Submit {
            position: logical_point(
                cursor_position.x as f32,
                cursor_position.y as f32,
                scale_factor,
            ),
        });
        self.0.as_ref().borrow_mut().handle_drag_drop(input);

        Ok(())
    }
}

pub(crate) struct StyleAndBounds {
    style: WINDOW_STYLE,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
}

#[derive(Debug)]
pub(crate) struct ClickState {
    button: MouseButton,
    last_click: Instant,
    last_position: Point<DevicePixels>,
    pub(crate) current_count: usize,
}

impl ClickState {
    pub fn new() -> Self {
        ClickState {
            button: MouseButton::Left,
            last_click: Instant::now(),
            last_position: Point::default(),
            current_count: 0,
        }
    }

    /// update self and return the needed click count
    pub fn update(&mut self, button: MouseButton, new_position: Point<DevicePixels>) -> usize {
        if self.button == button && self.is_double_click(new_position) {
            self.current_count += 1;
        } else {
            self.current_count = 1;
        }
        self.last_click = Instant::now();
        self.last_position = new_position;
        self.button = button;

        self.current_count
    }

    #[inline]
    fn is_double_click(&self, new_position: Point<DevicePixels>) -> bool {
        let diff = self.last_position - new_position;

        self.last_click.elapsed() < DOUBLE_CLICK_INTERVAL
            && diff.x.0.abs() <= DOUBLE_CLICK_SPATIAL_TOLERANCE
            && diff.y.0.abs() <= DOUBLE_CLICK_SPATIAL_TOLERANCE
    }
}

fn register_wnd_class(icon_handle: HICON) -> PCWSTR {
    const CLASS_NAME: PCWSTR = w!("Zed::Window");

    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hIcon: icon_handle,
            lpszClassName: PCWSTR(CLASS_NAME.as_ptr()),
            style: CS_HREDRAW | CS_VREDRAW,
            hInstance: get_module_handle().into(),
            ..Default::default()
        };
        unsafe { RegisterClassW(&wc) };
    });

    CLASS_NAME
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_NCCREATE {
        let cs = lparam.0 as *const CREATESTRUCTW;
        let cs = unsafe { &*cs };
        let ctx = cs.lpCreateParams as *mut WindowCreateContext;
        let ctx = unsafe { &mut *ctx };
        let inner = WindowsWindowState::new(
            hwnd,
            cs,
            ctx.handle,
            ctx.hide_title_bar,
            ctx.display.clone(),
            ctx.transparent,
            ctx.executor.clone(),
            ctx.main_receiver.clone(),
            ctx.mouse_wheel_settings,
            ctx.current_cursor,
        );
        let weak = Box::new(Rc::downgrade(&inner));
        unsafe { set_window_long(hwnd, GWLP_USERDATA, Box::into_raw(weak) as isize) };
        ctx.inner = Some(inner);
        return LRESULT(1);
    }
    let ptr =
        unsafe { get_window_long(hwnd, GWLP_USERDATA) } as *mut Weak<RefCell<WindowsWindowState>>;
    if ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    }
    let inner = unsafe { &*ptr };
    {
        let indent_count = INDENT_COUNT.load(std::sync::atomic::Ordering::Relaxed);
        let mut pre_indent = "".to_owned();
        for _ in 0..indent_count {
            pre_indent.push_str(INDENT_STRING);
        }
        println!("{}Handling MSG: {}", pre_indent, msg);
    }
    INDENT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let r = if let Some(state) = inner.upgrade() {
        handle_msg(hwnd, msg, wparam, lparam, state)
    } else {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    };
    INDENT_COUNT.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    {
        let indent_count = INDENT_COUNT.load(std::sync::atomic::Ordering::Relaxed);
        let mut pre_indent = "".to_owned();
        for _ in 0..indent_count {
            pre_indent.push_str(INDENT_STRING);
        }
        println!("{}Finished handling: {}", pre_indent, msg);
    }
    if msg == WM_NCDESTROY {
        unsafe { set_window_long(hwnd, GWLP_USERDATA, 0) };
        unsafe { drop(Box::from_raw(ptr)) };
    }
    r
}

static INDENT_COUNT: AtomicUsize = AtomicUsize::new(0);
const INDENT_STRING: &str = "   ";

pub(crate) fn try_get_window_inner(hwnd: HWND) -> Option<Arc<RwLock<WindowsWindowState>>> {
    if hwnd == HWND(0) {
        return None;
    }

    let ptr = unsafe { get_window_long(hwnd, GWLP_USERDATA) }
        as *mut std::sync::Weak<RwLock<WindowsWindowState>>;
    if !ptr.is_null() {
        let inner = unsafe { &*ptr };
        inner.upgrade()
    } else {
        None
    }
}

fn get_module_handle() -> HMODULE {
    unsafe {
        let mut h_module = std::mem::zeroed();
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            windows::core::w!("ZedModule"),
            &mut h_module,
        )
        .expect("Unable to get module handle"); // this should never fail

        h_module
    }
}

// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew
const DRAGDROP_GET_FILES_COUNT: u32 = 0xFFFFFFFF;
// https://learn.microsoft.com/en-us/windows/win32/controls/ttm-setdelaytime?redirectedfrom=MSDN
const DOUBLE_CLICK_INTERVAL: Duration = Duration::from_millis(500);
// https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics
const DOUBLE_CLICK_SPATIAL_TOLERANCE: i32 = 4;

#[cfg(test)]
mod tests {
    use super::ClickState;
    use crate::{point, DevicePixels, MouseButton};
    use std::time::Duration;

    #[test]
    fn test_double_click_interval() {
        let mut state = ClickState::new();
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(0), DevicePixels(0))),
            1
        );
        assert_eq!(
            state.update(MouseButton::Right, point(DevicePixels(0), DevicePixels(0))),
            1
        );
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(0), DevicePixels(0))),
            1
        );
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(0), DevicePixels(0))),
            2
        );
        state.last_click -= Duration::from_millis(700);
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(0), DevicePixels(0))),
            1
        );
    }

    #[test]
    fn test_double_click_spatial_tolerance() {
        let mut state = ClickState::new();
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(-3), DevicePixels(0))),
            1
        );
        assert_eq!(
            state.update(MouseButton::Left, point(DevicePixels(0), DevicePixels(3))),
            2
        );
        assert_eq!(
            state.update(MouseButton::Right, point(DevicePixels(3), DevicePixels(2))),
            1
        );
        assert_eq!(
            state.update(MouseButton::Right, point(DevicePixels(10), DevicePixels(0))),
            1
        );
    }
}
