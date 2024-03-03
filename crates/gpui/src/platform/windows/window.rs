#![deny(unsafe_op_in_unsafe_fn)]
// todo!("windows"): remove
#![allow(unused_variables)]

use std::{
    any::Any,
    cell::{Cell, RefCell},
    ffi::c_void,
    num::NonZeroIsize,
    rc::{Rc, Weak},
    sync::{Arc, Once},
};

use blade_graphics as gpu;
use futures::channel::oneshot::Receiver;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, GetWindowLongPtrW, LoadCursorW, PostQuitMessage,
            RegisterClassW, SetWindowLongPtrW, SetWindowTextW, ShowWindow, CREATESTRUCTW,
            CW_USEDEFAULT, GWLP_USERDATA, HMENU, IDC_ARROW, SW_MAXIMIZE, SW_SHOW, WINDOW_EX_STYLE,
            WINDOW_LONG_PTR_INDEX, WM_CLOSE, WM_DESTROY, WM_MOVE, WM_NCCREATE, WM_NCDESTROY,
            WM_PAINT, WM_SIZE, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

use crate::{
    platform::blade::BladeRenderer, AnyWindowHandle, Bounds, GlobalPixels, HiLoWord, Modifiers,
    Pixels, PlatformAtlas, PlatformDisplay, PlatformInput, PlatformInputHandler, PlatformWindow,
    Point, PromptLevel, Scene, Size, WindowAppearance, WindowBounds, WindowOptions, WindowsDisplay,
    WindowsPlatformInner,
};

struct WindowsWindowInner {
    hwnd: HWND,
    origin: Cell<Point<GlobalPixels>>,
    size: Cell<Size<GlobalPixels>>,
    mouse_position: Cell<Point<Pixels>>,
    input_handler: Cell<Option<PlatformInputHandler>>,
    renderer: RefCell<BladeRenderer>,
    callbacks: RefCell<Callbacks>,
    platform_inner: Rc<WindowsPlatformInner>,
    handle: AnyWindowHandle,
}

impl WindowsWindowInner {
    fn new(
        hwnd: HWND,
        cs: &CREATESTRUCTW,
        platform_inner: Rc<WindowsPlatformInner>,
        handle: AnyWindowHandle,
    ) -> Self {
        let origin = Cell::new(Point::new((cs.x as f64).into(), (cs.y as f64).into()));
        let size = Cell::new(Size {
            width: (cs.cx as f64).into(),
            height: (cs.cy as f64).into(),
        });
        let mouse_position = Cell::new(Point::default());
        let input_handler = Cell::new(None);
        struct RawWindow {
            hwnd: *mut c_void,
        }
        unsafe impl blade_rwh::HasRawWindowHandle for RawWindow {
            fn raw_window_handle(&self) -> blade_rwh::RawWindowHandle {
                let mut handle = blade_rwh::Win32WindowHandle::empty();
                handle.hwnd = self.hwnd;
                handle.into()
            }
        }
        unsafe impl blade_rwh::HasRawDisplayHandle for RawWindow {
            fn raw_display_handle(&self) -> blade_rwh::RawDisplayHandle {
                blade_rwh::WindowsDisplayHandle::empty().into()
            }
        }
        let raw = RawWindow { hwnd: hwnd.0 as _ };
        let gpu = Arc::new(
            unsafe {
                gpu::Context::init_windowed(
                    &raw,
                    gpu::ContextDesc {
                        validation: false,
                        capture: false,
                    },
                )
            }
            .unwrap(),
        );
        let extent = gpu::Extent {
            width: 1,
            height: 1,
            depth: 1,
        };
        let renderer = RefCell::new(BladeRenderer::new(gpu, extent));
        let callbacks = RefCell::new(Callbacks::default());
        Self {
            hwnd,
            origin,
            size,
            mouse_position,
            input_handler,
            renderer,
            callbacks,
            platform_inner,
            handle,
        }
    }

    fn handle_msg(&self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        log::debug!("msg: {msg}, wparam: {}, lparam: {}", wparam.0, lparam.0);
        match msg {
            WM_MOVE => {
                let x = lparam.loword() as f64;
                let y = lparam.hiword() as f64;
                self.origin.set(Point::new(x.into(), y.into()));
                let mut callbacks = self.callbacks.borrow_mut();
                if let Some(callback) = callbacks.moved.as_mut() {
                    callback()
                }
            }
            WM_SIZE => {
                // todo!("windows"): handle maximized or minimized
                let width = lparam.loword().max(1) as f64;
                let height = lparam.hiword().max(1) as f64;
                self.renderer
                    .borrow_mut()
                    .update_drawable_size(Size { width, height });
                let width = width.into();
                let height = height.into();
                self.size.set(Size { width, height });
                let mut callbacks = self.callbacks.borrow_mut();
                if let Some(callback) = callbacks.resize.as_mut() {
                    callback(
                        Size {
                            width: Pixels(width.0),
                            height: Pixels(height.0),
                        },
                        1.0,
                    )
                }
            }
            WM_PAINT => {
                let mut callbacks = self.callbacks.borrow_mut();
                if let Some(callback) = callbacks.request_frame.as_mut() {
                    callback()
                }
            }
            WM_CLOSE => {
                let mut callbacks: std::cell::RefMut<'_, Callbacks> = self.callbacks.borrow_mut();
                if let Some(callback) = callbacks.should_close.as_mut() {
                    if callback() {
                        return LRESULT(0);
                    }
                }
                drop(callbacks);
                return unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) };
            }
            WM_DESTROY => {
                let mut callbacks: std::cell::RefMut<'_, Callbacks> = self.callbacks.borrow_mut();
                if let Some(callback) = callbacks.close.take() {
                    callback()
                }
                let mut window_handles = self.platform_inner.window_handles.borrow_mut();
                window_handles.remove(&self.handle);
                if window_handles.is_empty() {
                    self.platform_inner
                        .foreground_executor
                        .spawn(async {
                            unsafe { PostQuitMessage(0) };
                        })
                        .detach();
                }
                return LRESULT(1);
            }
            _ => return unsafe { DefWindowProcW(self.hwnd, msg, wparam, lparam) },
        }
        LRESULT(0)
    }
}

#[derive(Default)]
struct Callbacks {
    request_frame: Option<Box<dyn FnMut()>>,
    input: Option<Box<dyn FnMut(crate::PlatformInput) -> bool>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    fullscreen: Option<Box<dyn FnMut(bool)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
}

pub(crate) struct WindowsWindow {
    inner: Rc<WindowsWindowInner>,
}

struct WindowCreateContext {
    inner: Option<Rc<WindowsWindowInner>>,
    platform_inner: Rc<WindowsPlatformInner>,
    handle: AnyWindowHandle,
}

impl WindowsWindow {
    pub(crate) fn new(
        platform_inner: Rc<WindowsPlatformInner>,
        handle: AnyWindowHandle,
        options: WindowOptions,
    ) -> Self {
        let dwexstyle = WINDOW_EX_STYLE::default();
        let classname = register_wnd_class();
        let windowname = HSTRING::from(
            options
                .titlebar
                .as_ref()
                .and_then(|titlebar| titlebar.title.as_ref())
                .map(|title| title.as_ref())
                .unwrap_or(""),
        );
        let dwstyle = WS_OVERLAPPEDWINDOW & !WS_VISIBLE;
        let mut x = CW_USEDEFAULT;
        let mut y = CW_USEDEFAULT;
        let mut nwidth = CW_USEDEFAULT;
        let mut nheight = CW_USEDEFAULT;
        match options.bounds {
            WindowBounds::Fullscreen => {}
            WindowBounds::Maximized => {}
            WindowBounds::Fixed(bounds) => {
                x = bounds.origin.x.0 as i32;
                y = bounds.origin.y.0 as i32;
                nwidth = bounds.size.width.0 as i32;
                nheight = bounds.size.height.0 as i32;
            }
        };
        let hwndparent = HWND::default();
        let hmenu = HMENU::default();
        let hinstance = HINSTANCE::default();
        let mut context = WindowCreateContext {
            inner: None,
            platform_inner: platform_inner.clone(),
            handle,
        };
        let lpparam = Some(&context as *const _ as *const _);
        unsafe {
            CreateWindowExW(
                dwexstyle,
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
        let wnd = Self {
            inner: context.inner.unwrap(),
        };
        platform_inner.window_handles.borrow_mut().insert(handle);
        match options.bounds {
            WindowBounds::Fullscreen => wnd.toggle_full_screen(),
            WindowBounds::Maximized => wnd.maximize(),
            WindowBounds::Fixed(_) => {}
        }
        unsafe { ShowWindow(wnd.inner.hwnd, SW_SHOW) };
        wnd
    }

    fn maximize(&self) {
        unsafe { ShowWindow(self.inner.hwnd, SW_MAXIMIZE) };
    }
}

impl HasWindowHandle for WindowsWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let raw = raw_window_handle::Win32WindowHandle::new(unsafe {
            NonZeroIsize::new_unchecked(self.inner.hwnd.0)
        })
        .into();
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(raw) })
    }
}

// todo!("windows")
impl HasDisplayHandle for WindowsWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        unimplemented!()
    }
}

impl PlatformWindow for WindowsWindow {
    fn bounds(&self) -> WindowBounds {
        WindowBounds::Fixed(Bounds {
            origin: self.inner.origin.get(),
            size: self.inner.size.get(),
        })
    }

    // todo!("windows")
    fn content_size(&self) -> Size<Pixels> {
        let size = self.inner.size.get();
        Size {
            width: size.width.0.into(),
            height: size.height.0.into(),
        }
    }

    // todo!("windows")
    fn scale_factor(&self) -> f32 {
        1.0
    }

    // todo!("windows")
    fn titlebar_height(&self) -> Pixels {
        20.0.into()
    }

    // todo!("windows")
    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Dark
    }

    // todo!("windows")
    fn display(&self) -> Rc<dyn PlatformDisplay> {
        Rc::new(WindowsDisplay::new())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.inner.mouse_position.get()
    }

    // todo!("windows")
    fn modifiers(&self) -> Modifiers {
        Modifiers::none()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    // todo!("windows")
    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.inner.input_handler.set(Some(input_handler));
    }

    // todo!("windows")
    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.inner.input_handler.take()
    }

    // todo!("windows")
    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> Receiver<usize> {
        unimplemented!()
    }

    // todo!("windows")
    fn activate(&self) {}

    // todo!("windows")
    fn set_title(&mut self, title: &str) {
        unsafe { SetWindowTextW(self.inner.hwnd, &HSTRING::from(title)) }
            .inspect_err(|e| log::error!("Set title failed: {e}"))
            .ok();
    }

    // todo!("windows")
    fn set_edited(&mut self, edited: bool) {}

    // todo!("windows")
    fn show_character_palette(&self) {}

    // todo!("windows")
    fn minimize(&self) {}

    // todo!("windows")
    fn zoom(&self) {}

    // todo!("windows")
    fn toggle_full_screen(&self) {}

    // todo!("windows")
    fn on_request_frame(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().request_frame = Some(callback);
    }

    // todo!("windows")
    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> bool>) {
        self.inner.callbacks.borrow_mut().input = Some(callback);
    }

    // todo!("windows")
    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    // todo!("windows")
    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.inner.callbacks.borrow_mut().resize = Some(callback);
    }

    // todo!("windows")
    fn on_fullscreen(&self, callback: Box<dyn FnMut(bool)>) {
        self.inner.callbacks.borrow_mut().fullscreen = Some(callback);
    }

    // todo!("windows")
    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().moved = Some(callback);
    }

    // todo!("windows")
    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.inner.callbacks.borrow_mut().should_close = Some(callback);
    }

    // todo!("windows")
    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.inner.callbacks.borrow_mut().close = Some(callback);
    }

    // todo!("windows")
    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.inner.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    // todo!("windows")
    fn is_topmost_for_position(&self, position: Point<Pixels>) -> bool {
        true
    }

    // todo!("windows")
    fn draw(&self, scene: &Scene) {
        self.inner.renderer.borrow_mut().draw(scene)
    }

    // todo!("windows")
    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.inner.renderer.borrow().sprite_atlas().clone()
    }
}

fn register_wnd_class() -> PCWSTR {
    const CLASS_NAME: PCWSTR = w!("Zed::Window");

    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() },
            lpszClassName: PCWSTR(CLASS_NAME.as_ptr()),
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
        let inner = Rc::new(WindowsWindowInner::new(
            hwnd,
            cs,
            ctx.platform_inner.clone(),
            ctx.handle,
        ));
        let weak = Box::new(Rc::downgrade(&inner));
        unsafe { set_window_long(hwnd, GWLP_USERDATA, Box::into_raw(weak) as isize) };
        ctx.inner = Some(inner);
        return LRESULT(1);
    }
    let ptr = unsafe { get_window_long(hwnd, GWLP_USERDATA) } as *mut Weak<WindowsWindowInner>;
    if ptr.is_null() {
        return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
    }
    let inner = unsafe { &*ptr };
    let r = if let Some(inner) = inner.upgrade() {
        inner.handle_msg(msg, wparam, lparam)
    } else {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    };
    if msg == WM_NCDESTROY {
        unsafe { set_window_long(hwnd, GWLP_USERDATA, 0) };
        unsafe { std::mem::drop(Box::from_raw(ptr)) };
    }
    r
}

unsafe fn get_window_long(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize {
    #[cfg(target_pointer_width = "64")]
    unsafe {
        GetWindowLongPtrW(hwnd, nindex)
    }
    #[cfg(target_pointer_width = "32")]
    unsafe {
        GetWindowLongW(hwnd, nindex) as isize
    }
}

unsafe fn set_window_long(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX, dwnewlong: isize) -> isize {
    #[cfg(target_pointer_width = "64")]
    unsafe {
        SetWindowLongPtrW(hwnd, nindex, dwnewlong)
    }
    #[cfg(target_pointer_width = "32")]
    unsafe {
        SetWindowLongW(hwnd, nindex, dwnewlong as i32) as isize
    }
}
