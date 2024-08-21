#![deny(unsafe_op_in_unsafe_fn)]

use std::{
    cell::RefCell,
    num::NonZeroIsize,
    path::PathBuf,
    rc::{Rc, Weak},
    str::FromStr,
    sync::{Arc, Once},
    time::{Duration, Instant},
};

use ::util::ResultExt;
use anyhow::{Context, Result};
use futures::channel::oneshot::{self, Receiver};
use itertools::Itertools;
use raw_window_handle as rwh;
use smallvec::SmallVec;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::*,
        System::{Com::*, LibraryLoader::*, Ole::*, SystemServices::*},
        UI::{Controls::*, HiDpi::*, Input::KeyboardAndMouse::*, Shell::*, WindowsAndMessaging::*},
    },
};

use crate::platform::blade::BladeRenderer;
use crate::*;

pub(crate) struct WindowsWindow(pub Rc<WindowsWindowStatePtr>);

pub struct WindowsWindowState {
    pub origin: Point<Pixels>,
    pub logical_size: Size<Pixels>,
    pub fullscreen_restore_bounds: Bounds<Pixels>,
    pub border_offset: WindowBorderOffset,
    pub scale_factor: f32,

    pub callbacks: Callbacks,
    pub input_handler: Option<PlatformInputHandler>,
    pub system_key_handled: bool,

    pub renderer: BladeRenderer,

    pub click_state: ClickState,
    pub system_settings: WindowsSystemSettings,
    pub current_cursor: HCURSOR,
    pub nc_button_pressed: Option<u32>,

    pub display: WindowsDisplay,
    fullscreen: Option<StyleAndBounds>,
    hwnd: HWND,
}

pub(crate) struct WindowsWindowStatePtr {
    hwnd: HWND,
    pub(crate) state: RefCell<WindowsWindowState>,
    pub(crate) handle: AnyWindowHandle,
    pub(crate) hide_title_bar: bool,
    pub(crate) is_movable: bool,
    pub(crate) executor: ForegroundExecutor,
    pub(crate) windows_version: WindowsVersion,
    pub(crate) validation_number: usize,
}

impl WindowsWindowState {
    fn new(
        hwnd: HWND,
        transparent: bool,
        cs: &CREATESTRUCTW,
        current_cursor: HCURSOR,
        display: WindowsDisplay,
    ) -> Result<Self> {
        let scale_factor = {
            let monitor_dpi = unsafe { GetDpiForWindow(hwnd) } as f32;
            monitor_dpi / USER_DEFAULT_SCREEN_DPI as f32
        };
        let origin = logical_point(cs.x as f32, cs.y as f32, scale_factor);
        let logical_size = {
            let physical_size = size(DevicePixels(cs.cx), DevicePixels(cs.cy));
            physical_size.to_pixels(scale_factor)
        };
        let fullscreen_restore_bounds = Bounds {
            origin,
            size: logical_size,
        };
        let border_offset = WindowBorderOffset::default();
        let renderer = windows_renderer::windows_renderer(hwnd, transparent)?;
        let callbacks = Callbacks::default();
        let input_handler = None;
        let system_key_handled = false;
        let click_state = ClickState::new();
        let system_settings = WindowsSystemSettings::new();
        let nc_button_pressed = None;
        let fullscreen = None;

        Ok(Self {
            origin,
            logical_size,
            fullscreen_restore_bounds,
            border_offset,
            scale_factor,
            callbacks,
            input_handler,
            system_key_handled,
            renderer,
            click_state,
            system_settings,
            current_cursor,
            nc_button_pressed,
            display,
            fullscreen,
            hwnd,
        })
    }

    #[inline]
    pub(crate) fn is_fullscreen(&self) -> bool {
        self.fullscreen.is_some()
    }

    pub(crate) fn is_maximized(&self) -> bool {
        !self.is_fullscreen() && unsafe { IsZoomed(self.hwnd) }.as_bool()
    }

    fn bounds(&self) -> Bounds<Pixels> {
        Bounds {
            origin: self.origin,
            size: self.logical_size,
        }
    }

    // Calculate the bounds used for saving and whether the window is maximized.
    fn calculate_window_bounds(&self) -> (Bounds<Pixels>, bool) {
        let placement = unsafe {
            let mut placement = WINDOWPLACEMENT {
                length: std::mem::size_of::<WINDOWPLACEMENT>() as u32,
                ..Default::default()
            };
            GetWindowPlacement(self.hwnd, &mut placement).log_err();
            placement
        };
        (
            calculate_client_rect(
                placement.rcNormalPosition,
                self.border_offset,
                self.scale_factor,
            ),
            placement.showCmd == SW_SHOWMAXIMIZED.0 as u32,
        )
    }

    fn window_bounds(&self) -> WindowBounds {
        let (bounds, maximized) = self.calculate_window_bounds();

        if self.is_fullscreen() {
            WindowBounds::Fullscreen(self.fullscreen_restore_bounds)
        } else if maximized {
            WindowBounds::Maximized(bounds)
        } else {
            WindowBounds::Windowed(bounds)
        }
    }

    /// get the logical size of the app's drawable area.
    ///
    /// Currently, GPUI uses logical size of the app to handle mouse interactions (such as
    /// whether the mouse collides with other elements of GPUI).
    fn content_size(&self) -> Size<Pixels> {
        self.logical_size
    }

    fn title_bar_padding(&self) -> Pixels {
        // using USER_DEFAULT_SCREEN_DPI because GPUI handles the scale with the scale factor
        let padding = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, USER_DEFAULT_SCREEN_DPI) };
        px(padding as f32)
    }

    fn title_bar_top_offset(&self) -> Pixels {
        if self.is_maximized() {
            self.title_bar_padding() * 2
        } else {
            px(0.)
        }
    }

    fn title_bar_height(&self) -> Pixels {
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
}

impl WindowsWindowStatePtr {
    fn new(context: &WindowCreateContext, hwnd: HWND, cs: &CREATESTRUCTW) -> Result<Rc<Self>> {
        let state = RefCell::new(WindowsWindowState::new(
            hwnd,
            context.transparent,
            cs,
            context.current_cursor,
            context.display,
        )?);

        Ok(Rc::new(Self {
            state,
            hwnd,
            handle: context.handle,
            hide_title_bar: context.hide_title_bar,
            is_movable: context.is_movable,
            executor: context.executor.clone(),
            windows_version: context.windows_version,
            validation_number: context.validation_number,
        }))
    }
}

#[derive(Default)]
pub(crate) struct Callbacks {
    pub(crate) request_frame: Option<Box<dyn FnMut()>>,
    pub(crate) input: Option<Box<dyn FnMut(crate::PlatformInput) -> DispatchEventResult>>,
    pub(crate) active_status_change: Option<Box<dyn FnMut(bool)>>,
    pub(crate) resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    pub(crate) moved: Option<Box<dyn FnMut()>>,
    pub(crate) should_close: Option<Box<dyn FnMut() -> bool>>,
    pub(crate) close: Option<Box<dyn FnOnce()>>,
    pub(crate) appearance_changed: Option<Box<dyn FnMut()>>,
}

struct WindowCreateContext {
    inner: Option<Result<Rc<WindowsWindowStatePtr>>>,
    handle: AnyWindowHandle,
    hide_title_bar: bool,
    display: WindowsDisplay,
    transparent: bool,
    is_movable: bool,
    executor: ForegroundExecutor,
    current_cursor: HCURSOR,
    windows_version: WindowsVersion,
    validation_number: usize,
}

impl WindowsWindow {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        params: WindowParams,
        icon: HICON,
        executor: ForegroundExecutor,
        current_cursor: HCURSOR,
        windows_version: WindowsVersion,
        validation_number: usize,
    ) -> Result<Self> {
        let classname = register_wnd_class(icon);
        let hide_title_bar = params
            .titlebar
            .as_ref()
            .map(|titlebar| titlebar.appears_transparent)
            .unwrap_or(true);
        let windowname = HSTRING::from(
            params
                .titlebar
                .as_ref()
                .and_then(|titlebar| titlebar.title.as_ref())
                .map(|title| title.as_ref())
                .unwrap_or(""),
        );
        let (dwexstyle, dwstyle) = if params.kind == WindowKind::PopUp {
            (WS_EX_TOOLWINDOW, WINDOW_STYLE(0x0))
        } else {
            (
                WS_EX_APPWINDOW,
                WS_THICKFRAME | WS_SYSMENU | WS_MAXIMIZEBOX | WS_MINIMIZEBOX,
            )
        };
        let hinstance = get_module_handle();
        let display = if let Some(display_id) = params.display_id {
            // if we obtain a display_id, then this ID must be valid.
            WindowsDisplay::new(display_id).unwrap()
        } else {
            WindowsDisplay::primary_monitor().unwrap()
        };
        let mut context = WindowCreateContext {
            inner: None,
            handle,
            hide_title_bar,
            display,
            transparent: true,
            is_movable: params.is_movable,
            executor,
            current_cursor,
            windows_version,
            validation_number,
        };
        let lpparam = Some(&context as *const _ as *const _);
        let creation_result = unsafe {
            CreateWindowExW(
                dwexstyle,
                classname,
                &windowname,
                dwstyle,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                hinstance,
                lpparam,
            )
        };
        // We should call `?` on state_ptr first, then call `?` on raw_hwnd.
        // Or, we will lose the error info reported by `WindowsWindowState::new`
        let state_ptr = context.inner.take().unwrap()?;
        let raw_hwnd = creation_result?;
        register_drag_drop(state_ptr.clone())?;

        unsafe {
            let mut placement = WINDOWPLACEMENT {
                length: std::mem::size_of::<WINDOWPLACEMENT>() as u32,
                ..Default::default()
            };
            GetWindowPlacement(raw_hwnd, &mut placement)?;
            // the bounds may be not inside the display
            let bounds = if display.check_given_bounds(params.bounds) {
                params.bounds
            } else {
                display.default_bounds()
            };
            let mut lock = state_ptr.state.borrow_mut();
            let bounds = bounds.to_device_pixels(lock.scale_factor);
            lock.border_offset.udpate(raw_hwnd)?;
            placement.rcNormalPosition = calcualte_window_rect(bounds, lock.border_offset);
            drop(lock);
            SetWindowPlacement(raw_hwnd, &placement)?;
        }
        unsafe { ShowWindow(raw_hwnd, SW_SHOW).ok()? };

        Ok(Self(state_ptr))
    }
}

impl rwh::HasWindowHandle for WindowsWindow {
    fn window_handle(&self) -> std::result::Result<rwh::WindowHandle<'_>, rwh::HandleError> {
        let raw = rwh::Win32WindowHandle::new(unsafe {
            NonZeroIsize::new_unchecked(self.0.hwnd.0 as isize)
        })
        .into();
        Ok(unsafe { rwh::WindowHandle::borrow_raw(raw) })
    }
}

// todo(windows)
impl rwh::HasDisplayHandle for WindowsWindow {
    fn display_handle(&self) -> std::result::Result<rwh::DisplayHandle<'_>, rwh::HandleError> {
        unimplemented!()
    }
}

impl Drop for WindowsWindow {
    fn drop(&mut self) {
        self.0.state.borrow_mut().renderer.destroy();
        // clone this `Rc` to prevent early release of the pointer
        let this = self.0.clone();
        self.0
            .executor
            .spawn(async move {
                let handle = this.hwnd;
                unsafe {
                    RevokeDragDrop(handle).log_err();
                    DestroyWindow(handle).log_err();
                }
            })
            .detach();
    }
}

impl PlatformWindow for WindowsWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        self.0.state.borrow().bounds()
    }

    fn is_maximized(&self) -> bool {
        self.0.state.borrow().is_maximized()
    }

    fn window_bounds(&self) -> WindowBounds {
        self.0.state.borrow().window_bounds()
    }

    /// get the logical size of the app's drawable area.
    ///
    /// Currently, GPUI uses logical size of the app to handle mouse interactions (such as
    /// whether the mouse collides with other elements of GPUI).
    fn content_size(&self) -> Size<Pixels> {
        self.0.state.borrow().content_size()
    }

    fn scale_factor(&self) -> f32 {
        self.0.state.borrow().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        system_appearance().log_err().unwrap_or_default()
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(Rc::new(self.0.state.borrow().display))
    }

    fn mouse_position(&self) -> Point<Pixels> {
        let scale_factor = self.scale_factor();
        let point = unsafe {
            let mut point: POINT = std::mem::zeroed();
            GetCursorPos(&mut point)
                .context("unable to get cursor position")
                .log_err();
            ScreenToClient(self.0.hwnd, &mut point).ok().log_err();
            point
        };
        logical_point(point.x as f32, point.y as f32, scale_factor)
    }

    fn modifiers(&self) -> Modifiers {
        current_modifiers()
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.0.state.borrow_mut().input_handler = Some(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.0.state.borrow_mut().input_handler.take()
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
        let handle = self.0.hwnd;
        self.0
            .executor
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
                        crate::PromptLevel::Critical => {
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
        let hwnd = self.0.hwnd;
        unsafe { SetActiveWindow(hwnd).log_err() };
        unsafe { SetFocus(hwnd).log_err() };
        // todo(windows)
        // crate `windows 0.56` reports true as Err
        unsafe { SetForegroundWindow(hwnd).as_bool() };
    }

    fn is_active(&self) -> bool {
        self.0.hwnd == unsafe { GetActiveWindow() }
    }

    // is_hovered is unused on Windows. See WindowContext::is_window_hovered.
    fn is_hovered(&self) -> bool {
        false
    }

    fn set_title(&mut self, title: &str) {
        unsafe { SetWindowTextW(self.0.hwnd, &HSTRING::from(title)) }
            .inspect_err(|e| log::error!("Set title failed: {e}"))
            .ok();
    }

    fn set_background_appearance(&self, background_appearance: WindowBackgroundAppearance) {
        self.0
            .state
            .borrow_mut()
            .renderer
            .update_transparency(background_appearance != WindowBackgroundAppearance::Opaque);
    }

    fn minimize(&self) {
        unsafe { ShowWindowAsync(self.0.hwnd, SW_MINIMIZE).ok().log_err() };
    }

    fn zoom(&self) {
        unsafe { ShowWindowAsync(self.0.hwnd, SW_MAXIMIZE).ok().log_err() };
    }

    fn toggle_fullscreen(&self) {
        let state_ptr = self.0.clone();
        self.0
            .executor
            .spawn(async move {
                let mut lock = state_ptr.state.borrow_mut();
                let StyleAndBounds {
                    style,
                    x,
                    y,
                    cx,
                    cy,
                } = if let Some(state) = lock.fullscreen.take() {
                    state
                } else {
                    let (window_bounds, _) = lock.calculate_window_bounds();
                    lock.fullscreen_restore_bounds = window_bounds;
                    let style =
                        WINDOW_STYLE(unsafe { get_window_long(state_ptr.hwnd, GWL_STYLE) } as _);
                    let mut rc = RECT::default();
                    unsafe { GetWindowRect(state_ptr.hwnd, &mut rc) }.log_err();
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
                    let physical_bounds = lock.display.physical_bounds();
                    StyleAndBounds {
                        style,
                        x: physical_bounds.left().0,
                        y: physical_bounds.top().0,
                        cx: physical_bounds.size.width.0,
                        cy: physical_bounds.size.height.0,
                    }
                };
                drop(lock);
                unsafe { set_window_long(state_ptr.hwnd, GWL_STYLE, style.0 as isize) };
                unsafe {
                    SetWindowPos(
                        state_ptr.hwnd,
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
        self.0.state.borrow().is_fullscreen()
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.0.state.borrow_mut().callbacks.request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.0.state.borrow_mut().callbacks.input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.state.borrow_mut().callbacks.active_status_change = Some(callback);
    }

    fn on_hover_status_change(&self, _: Box<dyn FnMut(bool)>) {}

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.state.borrow_mut().callbacks.resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.state.borrow_mut().callbacks.moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.0.state.borrow_mut().callbacks.should_close = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.0.state.borrow_mut().callbacks.close = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.0.state.borrow_mut().callbacks.appearance_changed = Some(callback);
    }

    fn draw(&self, scene: &Scene, on_complete: Option<oneshot::Sender<()>>) {
        self.0.state.borrow_mut().renderer.draw(scene, on_complete)
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.0.state.borrow().renderer.sprite_atlas().clone()
    }

    fn get_raw_handle(&self) -> HWND {
        self.0.hwnd
    }

    fn gpu_specs(&self) -> Option<GPUSpecs> {
        Some(self.0.state.borrow().renderer.gpu_specs())
    }

    fn fps(&self) -> Option<f32> {
        None
    }
}

#[implement(IDropTarget)]
struct WindowsDragDropHandler(pub Rc<WindowsWindowStatePtr>);

impl WindowsDragDropHandler {
    fn handle_drag_drop(&self, input: PlatformInput) {
        let mut lock = self.0.state.borrow_mut();
        if let Some(mut func) = lock.callbacks.input.take() {
            drop(lock);
            func(input);
            self.0.state.borrow_mut().callbacks.input = Some(func);
        }
    }
}

#[allow(non_snake_case)]
impl IDropTarget_Impl for WindowsDragDropHandler_Impl {
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
                ScreenToClient(self.0.hwnd, &mut cursor_position)
                    .ok()
                    .log_err();
                let scale_factor = self.0.state.borrow().scale_factor;
                let input = PlatformInput::FileDrop(FileDropEvent::Entered {
                    position: logical_point(
                        cursor_position.x as f32,
                        cursor_position.y as f32,
                        scale_factor,
                    ),
                    paths: ExternalPaths(paths),
                });
                self.handle_drag_drop(input);
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
            ScreenToClient(self.0.hwnd, &mut cursor_position)
                .ok()
                .log_err();
        }
        let scale_factor = self.0.state.borrow().scale_factor;
        let input = PlatformInput::FileDrop(FileDropEvent::Pending {
            position: logical_point(
                cursor_position.x as f32,
                cursor_position.y as f32,
                scale_factor,
            ),
        });
        self.handle_drag_drop(input);

        Ok(())
    }

    fn DragLeave(&self) -> windows::core::Result<()> {
        let input = PlatformInput::FileDrop(FileDropEvent::Exited);
        self.handle_drag_drop(input);

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
            ScreenToClient(self.0.hwnd, &mut cursor_position)
                .ok()
                .log_err();
        }
        let scale_factor = self.0.state.borrow().scale_factor;
        let input = PlatformInput::FileDrop(FileDropEvent::Submit {
            position: logical_point(
                cursor_position.x as f32,
                cursor_position.y as f32,
                scale_factor,
            ),
        });
        self.handle_drag_drop(input);

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct ClickState {
    button: MouseButton,
    last_click: Instant,
    last_position: Point<DevicePixels>,
    double_click_spatial_tolerance_width: i32,
    double_click_spatial_tolerance_height: i32,
    double_click_interval: Duration,
    pub(crate) current_count: usize,
}

impl ClickState {
    pub fn new() -> Self {
        let double_click_spatial_tolerance_width = unsafe { GetSystemMetrics(SM_CXDOUBLECLK) };
        let double_click_spatial_tolerance_height = unsafe { GetSystemMetrics(SM_CYDOUBLECLK) };
        let double_click_interval = Duration::from_millis(unsafe { GetDoubleClickTime() } as u64);

        ClickState {
            button: MouseButton::Left,
            last_click: Instant::now(),
            last_position: Point::default(),
            double_click_spatial_tolerance_width,
            double_click_spatial_tolerance_height,
            double_click_interval,
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

    pub fn system_update(&mut self) {
        self.double_click_spatial_tolerance_width = unsafe { GetSystemMetrics(SM_CXDOUBLECLK) };
        self.double_click_spatial_tolerance_height = unsafe { GetSystemMetrics(SM_CYDOUBLECLK) };
        self.double_click_interval = Duration::from_millis(unsafe { GetDoubleClickTime() } as u64);
    }

    #[inline]
    fn is_double_click(&self, new_position: Point<DevicePixels>) -> bool {
        let diff = self.last_position - new_position;

        self.last_click.elapsed() < self.double_click_interval
            && diff.x.0.abs() <= self.double_click_spatial_tolerance_width
            && diff.y.0.abs() <= self.double_click_spatial_tolerance_height
    }
}

struct StyleAndBounds {
    style: WINDOW_STYLE,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
}

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct WindowBorderOffset {
    width_offset: i32,
    height_offset: i32,
}

impl WindowBorderOffset {
    pub(crate) fn udpate(&mut self, hwnd: HWND) -> anyhow::Result<()> {
        let window_rect = unsafe {
            let mut rect = std::mem::zeroed();
            GetWindowRect(hwnd, &mut rect)?;
            rect
        };
        let client_rect = unsafe {
            let mut rect = std::mem::zeroed();
            GetClientRect(hwnd, &mut rect)?;
            rect
        };
        self.width_offset =
            (window_rect.right - window_rect.left) - (client_rect.right - client_rect.left);
        self.height_offset =
            (window_rect.bottom - window_rect.top) - (client_rect.bottom - client_rect.top);
        Ok(())
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
        let creation_result = WindowsWindowStatePtr::new(ctx, hwnd, cs);
        if creation_result.is_err() {
            ctx.inner = Some(creation_result);
            return LRESULT(0);
        }
        let weak = Box::new(Rc::downgrade(creation_result.as_ref().unwrap()));
        unsafe { set_window_long(hwnd, GWLP_USERDATA, Box::into_raw(weak) as isize) };
        ctx.inner = Some(creation_result);
        return LRESULT(1);
    }
    let ptr = unsafe { get_window_long(hwnd, GWLP_USERDATA) } as *mut Weak<WindowsWindowStatePtr>;
    if ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    }
    let inner = unsafe { &*ptr };
    let r = if let Some(state) = inner.upgrade() {
        handle_msg(hwnd, msg, wparam, lparam, state)
    } else {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    };
    if msg == WM_NCDESTROY {
        unsafe { set_window_long(hwnd, GWLP_USERDATA, 0) };
        unsafe { drop(Box::from_raw(ptr)) };
    }
    r
}

pub(crate) fn try_get_window_inner(hwnd: HWND) -> Option<Rc<WindowsWindowStatePtr>> {
    if hwnd.is_invalid() {
        return None;
    }

    let ptr = unsafe { get_window_long(hwnd, GWLP_USERDATA) } as *mut Weak<WindowsWindowStatePtr>;
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

fn register_drag_drop(state_ptr: Rc<WindowsWindowStatePtr>) -> Result<()> {
    let window_handle = state_ptr.hwnd;
    let handler = WindowsDragDropHandler(state_ptr);
    // The lifetime of `IDropTarget` is handled by Windows, it won't release until
    // we call `RevokeDragDrop`.
    // So, it's safe to drop it here.
    let drag_drop_handler: IDropTarget = handler.into();
    unsafe {
        RegisterDragDrop(window_handle, &drag_drop_handler)
            .context("unable to register drag-drop event")?;
    }
    Ok(())
}

fn calcualte_window_rect(bounds: Bounds<DevicePixels>, border_offset: WindowBorderOffset) -> RECT {
    // NOTE:
    // The reason that not using `AdjustWindowRectEx()` here is
    // that the size reported by this function is incorrect.
    // You can test it, and there are similar discussions online.
    // See: https://stackoverflow.com/questions/12423584/how-to-set-exact-client-size-for-overlapped-window-winapi
    //
    // So we manually calculate these values here.
    let mut rect = RECT {
        left: bounds.left().0,
        top: bounds.top().0,
        right: bounds.right().0,
        bottom: bounds.bottom().0,
    };
    let left_offset = border_offset.width_offset / 2;
    let top_offset = border_offset.height_offset / 2;
    let right_offset = border_offset.width_offset - left_offset;
    let bottom_offet = border_offset.height_offset - top_offset;
    rect.left -= left_offset;
    rect.top -= top_offset;
    rect.right += right_offset;
    rect.bottom += bottom_offet;
    rect
}

fn calculate_client_rect(
    rect: RECT,
    border_offset: WindowBorderOffset,
    scale_factor: f32,
) -> Bounds<Pixels> {
    let left_offset = border_offset.width_offset / 2;
    let top_offset = border_offset.height_offset / 2;
    let right_offset = border_offset.width_offset - left_offset;
    let bottom_offet = border_offset.height_offset - top_offset;
    let left = rect.left + left_offset;
    let top = rect.top + top_offset;
    let right = rect.right - right_offset;
    let bottom = rect.bottom - bottom_offet;
    let physical_size = size(DevicePixels(right - left), DevicePixels(bottom - top));
    Bounds {
        origin: logical_point(left as f32, top as f32, scale_factor),
        size: physical_size.to_pixels(scale_factor),
    }
}

// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew
const DRAGDROP_GET_FILES_COUNT: u32 = 0xFFFFFFFF;

mod windows_renderer {
    use std::{num::NonZeroIsize, sync::Arc};

    use blade_graphics as gpu;
    use raw_window_handle as rwh;
    use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::GWLP_HINSTANCE};

    use crate::{
        get_window_long,
        platform::blade::{BladeRenderer, BladeSurfaceConfig},
    };

    pub(super) fn windows_renderer(hwnd: HWND, transparent: bool) -> anyhow::Result<BladeRenderer> {
        let raw = RawWindow { hwnd };
        let gpu: Arc<gpu::Context> = Arc::new(
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
            .map_err(|e| anyhow::anyhow!("{:?}", e))?,
        );
        let config = BladeSurfaceConfig {
            size: gpu::Extent::default(),
            transparent,
        };

        Ok(BladeRenderer::new(gpu, config))
    }

    struct RawWindow {
        hwnd: HWND,
    }

    impl rwh::HasWindowHandle for RawWindow {
        fn window_handle(&self) -> Result<rwh::WindowHandle<'_>, rwh::HandleError> {
            Ok(unsafe {
                let hwnd = NonZeroIsize::new_unchecked(self.hwnd.0 as isize);
                let mut handle = rwh::Win32WindowHandle::new(hwnd);
                let hinstance = get_window_long(self.hwnd, GWLP_HINSTANCE);
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
}

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
