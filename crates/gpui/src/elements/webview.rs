//! WebView element for rendering HTML content using platform-native webviews.
//!
//! This element is gated behind the `webview` feature flag.
//!
//! On macOS, Windows, and Linux/X11, webviews are embedded as child views
//! inside the GPUI window using wry's `build_as_child()`.
//!
//! On Linux/Wayland, webviews open in separate GTK windows on a dedicated
//! thread because wry's `build_as_child()` does not support Wayland.

use crate::{
    App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    Pixels, Style, StyleRefinement, Styled, Window, px,
};
use refineable::Refineable;
#[cfg(feature = "webview")]
use std::collections::HashMap;
#[cfg(all(feature = "webview", any(target_os = "linux", target_os = "freebsd")))]
use std::sync::Mutex;

#[cfg(feature = "webview")]
static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

#[cfg(feature = "webview")]
static ACTIVE_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

// wry::WebView contains raw pointers (not Send/Sync), so we use thread-local
// storage. All GPUI paint calls happen on the main thread.
// Only macOS/Windows use this path; Linux uses GTK-thread storage instead.
#[cfg(all(
    feature = "webview",
    not(any(target_os = "linux", target_os = "freebsd"))
))]
thread_local! {
    static CHILD_WEBVIEWS: std::cell::RefCell<HashMap<usize, ChildWebView>> =
        std::cell::RefCell::new(HashMap::new());
}

#[cfg(all(
    feature = "webview",
    not(any(target_os = "linux", target_os = "freebsd"))
))]
struct ChildWebView {
    webview: wry::WebView,
    last_bounds: Bounds<Pixels>,
}

#[cfg(all(
    feature = "webview",
    feature = "gtk",
    any(target_os = "linux", target_os = "freebsd")
))]
use gtk::glib;
#[cfg(all(
    feature = "webview",
    feature = "gtk",
    any(target_os = "linux", target_os = "freebsd")
))]
use gtk::prelude::*;
#[cfg(all(
    feature = "webview",
    feature = "gtk",
    any(target_os = "linux", target_os = "freebsd")
))]
use wry::WebViewBuilderExtUnix;

#[cfg(all(
    feature = "webview",
    feature = "gtk",
    any(target_os = "linux", target_os = "freebsd")
))]
mod gtk_thread {
    use super::*;

    enum GtkMessage {
        Create { id: usize, content: WebViewContent },
        Close { id: usize },
    }

    static CREATED: std::sync::LazyLock<Mutex<std::collections::HashSet<usize>>> =
        std::sync::LazyLock::new(|| Mutex::new(std::collections::HashSet::new()));

    // Populated by delete_event so the GPUI render loop can remove stale entries.
    static CLOSED_BY_USER: std::sync::LazyLock<Mutex<Vec<usize>>> =
        std::sync::LazyLock::new(|| Mutex::new(Vec::new()));

    // gtk::Window and wry::WebView aren't Send, so we store them on the GTK thread.
    // The webview is kept alive as long as the window exists; dropping it destroys
    // the underlying WebKitGTK widget.
    thread_local! {
        static WINDOWS: std::cell::RefCell<HashMap<usize, gtk::Window>> =
            std::cell::RefCell::new(HashMap::new());
        static WEBVIEWS: std::cell::RefCell<HashMap<usize, wry::WebView>> =
            std::cell::RefCell::new(HashMap::new());
    }

    static GTK_SENDER: std::sync::LazyLock<Mutex<std::sync::mpsc::Sender<GtkMessage>>> =
        std::sync::LazyLock::new(|| {
            let (sender, receiver) = std::sync::mpsc::channel::<GtkMessage>();

            std::thread::spawn(move || {
                if let Err(error) = gtk::init() {
                    log::error!("Failed to initialize GTK: {}", error);
                    return;
                }

                let receiver = std::cell::RefCell::new(receiver);
                glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
                    while let Ok(message) = receiver.borrow().try_recv() {
                        match message {
                            GtkMessage::Create { id, content } => create_window(id, content),
                            GtkMessage::Close { id } => close_window(id),
                        }
                    }
                    glib::ControlFlow::Continue
                });

                gtk::main();
            });

            Mutex::new(sender)
        });

    fn create_window(id: usize, content: WebViewContent) {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_title("Zed WebView");
        window.set_default_size(900, 700);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.set_hexpand(true);
        container.set_vexpand(true);
        window.add(&container);
        window.show_all();

        let builder = match &content {
            WebViewContent::Html(html) => wry::WebViewBuilder::new().with_html(html),
            WebViewContent::Url(url) => wry::WebViewBuilder::new().with_url(url),
        };

        match builder.build_gtk(&container) {
            Ok(webview) => {
                log::info!("WebView #{} created (Wayland/GTK)", id);
                ACTIVE_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                WEBVIEWS.with(|webviews| {
                    webviews.borrow_mut().insert(id, webview);
                });
            }
            Err(error) => {
                log::error!("WebView #{}: failed: {}", id, error);
                window.close();
                return;
            }
        }

        let webview_id = id;
        window.connect_delete_event(move |_, _| {
            let was_tracked =
                WINDOWS.with(|windows| windows.borrow_mut().remove(&webview_id).is_some());
            if was_tracked {
                WEBVIEWS.with(|webviews| {
                    webviews.borrow_mut().remove(&webview_id);
                });
                ACTIVE_COUNT.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                CLOSED_BY_USER
                    .lock()
                    .unwrap_or_else(|poison| poison.into_inner())
                    .push(webview_id);
            }
            glib::Propagation::Proceed
        });

        WINDOWS.with(|windows| {
            windows.borrow_mut().insert(id, window);
        });
    }

    fn close_window(id: usize) {
        WINDOWS.with(|windows| {
            if let Some(window) = windows.borrow_mut().remove(&id) {
                WEBVIEWS.with(|webviews| {
                    webviews.borrow_mut().remove(&id);
                });
                ACTIVE_COUNT.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                window.close();
            }
        });
    }

    pub fn drain_closed() -> Vec<usize> {
        let mut closed = CLOSED_BY_USER
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        std::mem::take(&mut *closed)
    }

    pub fn remove_created(id: usize) {
        CREATED
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .remove(&id);
        let sender = GTK_SENDER
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if let Err(error) = sender.send(GtkMessage::Close { id }) {
            log::error!(
                "WebView #{}: failed to send close to GTK thread: {}",
                id,
                error
            );
        }
    }

    pub fn ensure_created(id: usize, content: &WebViewContent) {
        let mut created = CREATED.lock().unwrap_or_else(|poison| poison.into_inner());
        if created.contains(&id) {
            return;
        }
        created.insert(id);
        drop(created);

        let sender = GTK_SENDER
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if let Err(error) = sender.send(GtkMessage::Create {
            id,
            content: content.clone(),
        }) {
            log::error!("WebView #{}: failed to send to GTK thread: {}", id, error);
        }
    }
}

#[cfg(feature = "webview")]
fn is_wayland() -> bool {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        crate::guess_compositor() == "Wayland"
    }
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        false
    }
}

#[cfg(feature = "webview")]
fn to_wry_bounds(bounds: Bounds<Pixels>, scale_factor: f32) -> wry::Rect {
    // GPUI bounds are in logical pixels. Convert to physical pixels for
    // the native child window since wry positions relative to the parent
    // window's pixel coordinates.
    let x = (bounds.origin.x.0 * scale_factor) as i32;
    let y = (bounds.origin.y.0 * scale_factor) as i32;
    let width = (bounds.size.width.0 * scale_factor) as u32;
    let height = (bounds.size.height.0 * scale_factor) as u32;
    wry::Rect {
        position: wry::dpi::Position::Physical(wry::dpi::PhysicalPosition::new(x, y)),
        size: wry::dpi::Size::Physical(wry::dpi::PhysicalSize::new(width, height)),
    }
}

// On X11, wry's build_as_child internally creates GTK widgets that need
// their own event loop. We run this on a dedicated thread to avoid blocking
// GPUI's render loop. The main thread extracts raw window IDs and sends
// them to this thread.
#[cfg(all(feature = "webview", any(target_os = "linux", target_os = "freebsd")))]
mod x11_child_thread {
    use super::*;

    // wry only accepts Xlib handles but GPUI provides Xcb. The X11 window
    // ID is the same u32 in both, so we store it and present as Xlib.
    struct RawXlibHandle {
        window_id: std::ffi::c_ulong,
        screen: i32,
    }

    // Safety: the window ID and screen are plain integers copied from the
    // main thread. They reference an X11 window that outlives this struct.
    unsafe impl Send for RawXlibHandle {}
    unsafe impl Sync for RawXlibHandle {}

    impl raw_window_handle::HasWindowHandle for RawXlibHandle {
        fn window_handle(
            &self,
        ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
            let handle = raw_window_handle::XlibWindowHandle::new(self.window_id);
            Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(handle.into()) })
        }
    }

    impl raw_window_handle::HasDisplayHandle for RawXlibHandle {
        fn display_handle(
            &self,
        ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
            let handle = raw_window_handle::XlibDisplayHandle::new(None, self.screen);
            Ok(unsafe { raw_window_handle::DisplayHandle::borrow_raw(handle.into()) })
        }
    }

    enum ChildMessage {
        Create {
            id: usize,
            content: WebViewContent,
            window_id: std::ffi::c_ulong,
            screen: i32,
            bounds: wry::Rect,
        },
        SetBounds {
            id: usize,
            bounds: wry::Rect,
        },
        Close {
            id: usize,
        },
    }

    static CREATED: std::sync::LazyLock<Mutex<std::collections::HashSet<usize>>> =
        std::sync::LazyLock::new(|| Mutex::new(std::collections::HashSet::new()));

    // wry::WebView isn't Send, so we store them on the GTK thread.
    thread_local! {
        static WEBVIEWS: std::cell::RefCell<HashMap<usize, wry::WebView>> =
            std::cell::RefCell::new(HashMap::new());
    }

    static CHILD_SENDER: std::sync::LazyLock<Mutex<std::sync::mpsc::Sender<ChildMessage>>> =
        std::sync::LazyLock::new(|| {
            let (sender, receiver) = std::sync::mpsc::channel::<ChildMessage>();

            std::thread::spawn(move || {
                if let Err(error) = gtk::init() {
                    log::error!("X11 child thread: GTK init failed: {}", error);
                    return;
                }

                let receiver = std::cell::RefCell::new(receiver);
                glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
                    while let Ok(message) = receiver.borrow().try_recv() {
                        match message {
                            ChildMessage::Create {
                                id,
                                content,
                                window_id,
                                screen,
                                bounds,
                            } => {
                                create_child(id, content, window_id, screen, bounds);
                            }
                            ChildMessage::SetBounds { id, bounds } => {
                                WEBVIEWS.with(|webviews| {
                                    if let Some(webview) = webviews.borrow().get(&id) {
                                        if let Err(error) = webview.set_bounds(bounds) {
                                            log::error!(
                                                "WebView #{}: set_bounds failed: {}",
                                                id,
                                                error
                                            );
                                        }
                                    }
                                });
                            }
                            ChildMessage::Close { id } => {
                                WEBVIEWS.with(|webviews| {
                                    webviews.borrow_mut().remove(&id);
                                });
                            }
                        }
                    }
                    glib::ControlFlow::Continue
                });

                gtk::main();
            });

            Mutex::new(sender)
        });

    fn create_child(
        id: usize,
        content: WebViewContent,
        window_id: std::ffi::c_ulong,
        screen: i32,
        bounds: wry::Rect,
    ) {
        let handle = RawXlibHandle { window_id, screen };

        let builder = match &content {
            WebViewContent::Html(html) => wry::WebViewBuilder::new().with_html(html),
            WebViewContent::Url(url) => wry::WebViewBuilder::new().with_url(url),
        };

        match builder
            .with_bounds(bounds)
            .with_transparent(true)
            .build_as_child(&handle)
        {
            Ok(webview) => {
                log::info!("WebView #{} created (X11 child)", id);
                ACTIVE_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                WEBVIEWS.with(|webviews| {
                    webviews.borrow_mut().insert(id, webview);
                });
            }
            Err(error) => {
                log::error!("WebView #{}: build_as_child failed: {}", id, error);
            }
        }
    }

    pub fn request_create(id: usize, content: &WebViewContent, bounds: wry::Rect, window: &Window) {
        let mut created = CREATED.lock().unwrap_or_else(|poison| poison.into_inner());
        if created.contains(&id) {
            return;
        }
        created.insert(id);
        drop(created);

        let (window_id, screen) = match raw_window_handle::HasWindowHandle::window_handle(window) {
            Ok(handle) => match handle.as_raw() {
                raw_window_handle::RawWindowHandle::Xcb(xcb) => {
                    (xcb.window.get() as std::ffi::c_ulong, 0i32)
                }
                _ => {
                    log::error!("WebView #{}: expected Xcb window handle", id);
                    return;
                }
            },
            Err(error) => {
                log::error!("WebView #{}: failed to get window handle: {}", id, error);
                return;
            }
        };

        let sender = CHILD_SENDER
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if let Err(error) = sender.send(ChildMessage::Create {
            id,
            content: content.clone(),
            window_id,
            screen,
            bounds,
        }) {
            log::error!(
                "WebView #{}: failed to send to X11 child thread: {}",
                id,
                error
            );
        }
    }

    // Track last bounds per webview on the main thread to avoid sending
    // redundant SetBounds messages every paint frame.
    static LAST_BOUNDS: std::sync::LazyLock<Mutex<HashMap<usize, Bounds<Pixels>>>> =
        std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

    pub fn request_set_bounds(id: usize, bounds: Bounds<Pixels>, scale_factor: f32) {
        let mut last = LAST_BOUNDS
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if last.get(&id) == Some(&bounds) {
            return;
        }
        last.insert(id, bounds);
        drop(last);

        let sender = CHILD_SENDER
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if let Err(error) = sender.send(ChildMessage::SetBounds {
            id,
            bounds: super::to_wry_bounds(bounds, scale_factor),
        }) {
            log::error!("WebView #{}: failed to send set_bounds: {}", id, error);
        }
    }

    pub fn remove_created(id: usize) {
        CREATED
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .remove(&id);
        LAST_BOUNDS
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .remove(&id);
        let sender = CHILD_SENDER
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        if let Err(error) = sender.send(ChildMessage::Close { id }) {
            log::error!("WebView #{}: failed to send close: {}", id, error);
        }
    }
}

// On macOS/Windows, build_as_child runs on the main thread (no GTK needed).
#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
struct RawHandle {
    window: raw_window_handle::RawWindowHandle,
    display: raw_window_handle::RawDisplayHandle,
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
unsafe impl Send for RawHandle {}
#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
unsafe impl Sync for RawHandle {}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
impl raw_window_handle::HasWindowHandle for RawHandle {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        unsafe { Ok(raw_window_handle::WindowHandle::borrow_raw(self.window)) }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
impl raw_window_handle::HasDisplayHandle for RawHandle {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        unsafe { Ok(raw_window_handle::DisplayHandle::borrow_raw(self.display)) }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
#[cfg(feature = "webview")]
fn create_child_webview(
    id: usize,
    content: WebViewContent,
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    window: &mut Window,
    cx: &mut App,
) {
    let already_exists = CHILD_WEBVIEWS.with(|children| children.borrow().contains_key(&id));
    if already_exists {
        return;
    }

    use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
    let raw_window_handle = match window.window_handle() {
        Ok(handle) => handle.as_raw(),
        Err(error) => {
            log::error!("WebView #{}: failed to get window handle: {}", id, error);
            return;
        }
    };
    let display_handle = match window.display_handle() {
        Ok(handle) => handle.as_raw(),
        Err(error) => {
            log::error!("WebView #{}: failed to get display handle: {}", id, error);
            return;
        }
    };
    let handle = RawHandle {
        window: raw_window_handle,
        display: display_handle,
    };

    let mut cx = cx.to_async();
    cx.clone()
        .foreground_executor()
        .spawn(async move {
            let already_exists =
                CHILD_WEBVIEWS.with(|children| children.borrow().contains_key(&id));
            if already_exists {
                return;
            }

            let builder = match content {
                WebViewContent::Html(html) => wry::WebViewBuilder::new().with_html(html),
                WebViewContent::Url(url) => wry::WebViewBuilder::new().with_url(url),
            };

            // build_as_child may process platform messages synchronously (e.g.
            // WebView2 on Windows), so the RefCell must not be borrowed here.
            let result = builder
                .with_bounds(to_wry_bounds(bounds, scale_factor))
                .with_transparent(true)
                .build_as_child(&handle);

            match result {
                Ok(webview) => {
                    log::info!("WebView #{} created (child)", id);
                    ACTIVE_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    CHILD_WEBVIEWS.with(|children| {
                        children.borrow_mut().insert(
                            id,
                            ChildWebView {
                                webview,
                                last_bounds: bounds,
                            },
                        );
                    });

                    if let Err(error) = cx.update(|cx| cx.refresh_windows()) {
                        log::error!("WebView #{}: failed to refresh windows: {}", id, error);
                    }
                }
                Err(error) => {
                    log::error!("WebView #{}: build_as_child failed: {}", id, error);
                }
            }
        })
        .detach();
}

#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
#[cfg(feature = "webview")]
fn update_child_bounds(id: usize, bounds: Bounds<Pixels>, scale_factor: f32) {
    CHILD_WEBVIEWS.with(|children| {
        if let Some(child) = children.borrow_mut().get_mut(&id) {
            if child.last_bounds == bounds {
                return;
            }
            child.last_bounds = bounds;
            if let Err(error) = child
                .webview
                .set_bounds(to_wry_bounds(bounds, scale_factor))
            {
                log::error!("WebView #{}: set_bounds failed: {}", id, error);
            }
        }
    });
}

/// A WebView element for rendering HTML content.
///
/// Each webview is identified by a unique ID. The native webview is created
/// once and persists across GPUI render cycles.
pub struct WebView {
    id: usize,
    content: WebViewContent,
    style: StyleRefinement,
}

/// Content to display in a webview.
#[derive(Clone)]
pub enum WebViewContent {
    /// Raw HTML string.
    Html(String),
    /// URL to navigate to.
    Url(String),
}

impl WebView {
    /// Create a WebView element that renders the given HTML string.
    pub fn from_html(id: usize, html: impl Into<String>) -> Self {
        Self {
            id,
            content: WebViewContent::Html(html.into()),
            style: Default::default(),
        }
    }

    /// Create a WebView element that navigates to the given URL.
    pub fn from_url(id: usize, url: impl Into<String>) -> Self {
        Self {
            id,
            content: WebViewContent::Url(url.into()),
            style: Default::default(),
        }
    }

    /// Allocate a new unique webview ID.
    pub fn next_id() -> usize {
        #[cfg(feature = "webview")]
        {
            NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        }
        #[cfg(not(feature = "webview"))]
        {
            0
        }
    }

    /// Return the number of currently open webview windows.
    pub fn active_count() -> usize {
        #[cfg(feature = "webview")]
        {
            ACTIVE_COUNT.load(std::sync::atomic::Ordering::Relaxed)
        }
        #[cfg(not(feature = "webview"))]
        {
            0
        }
    }

    /// Returns IDs of webviews that were closed externally (e.g. by the window manager).
    /// Call this during render to clean up stale entries from your model.
    pub fn drain_closed() -> Vec<usize> {
        #[cfg(all(
            feature = "webview",
            feature = "gtk",
            any(target_os = "linux", target_os = "freebsd")
        ))]
        {
            gtk_thread::drain_closed()
        }
        #[cfg(not(all(
            feature = "webview",
            feature = "gtk",
            any(target_os = "linux", target_os = "freebsd")
        )))]
        {
            Vec::new()
        }
    }

    /// Destroy a webview by ID, freeing its resources.
    pub fn remove(id: usize) {
        #[cfg(all(
            feature = "webview",
            not(any(target_os = "linux", target_os = "freebsd"))
        ))]
        {
            let was_child =
                CHILD_WEBVIEWS.with(|children| children.borrow_mut().remove(&id).is_some());
            if was_child {
                ACTIVE_COUNT.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            }
        }

        #[cfg(all(
            feature = "webview",
            feature = "gtk",
            any(target_os = "linux", target_os = "freebsd")
        ))]
        {
            if is_wayland() {
                gtk_thread::remove_created(id);
            } else {
                x11_child_thread::remove_created(id);
                ACTIVE_COUNT.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            }
        }

        #[cfg(not(feature = "webview"))]
        drop(id);
    }

    /// Open a webview window with HTML content immediately.
    /// On Wayland this opens a separate GTK window.
    /// On macOS/Windows/X11 this is a no-op (use the element API instead).
    pub fn open_html(html: impl Into<String>) {
        let html = html.into();

        #[cfg(all(
            feature = "webview",
            feature = "gtk",
            any(target_os = "linux", target_os = "freebsd")
        ))]
        {
            let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            gtk_thread::ensure_created(id, &WebViewContent::Html(html));
        }

        #[cfg(not(all(
            feature = "webview",
            feature = "gtk",
            any(target_os = "linux", target_os = "freebsd")
        )))]
        drop(html);
    }

    /// Open a webview window navigating to a URL immediately.
    pub fn open_url(url: impl Into<String>) {
        let url = url.into();

        #[cfg(all(
            feature = "webview",
            feature = "gtk",
            any(target_os = "linux", target_os = "freebsd")
        ))]
        {
            let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            gtk_thread::ensure_created(id, &WebViewContent::Url(url));
        }

        #[cfg(not(all(
            feature = "webview",
            feature = "gtk",
            any(target_os = "linux", target_os = "freebsd")
        )))]
        drop(url);
    }
}

impl Element for WebView {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.refine(&self.style);
        if matches!(style.size.width, crate::geometry::Length::Auto) {
            style.size.width = crate::geometry::Length::Definite(px(400.0).into());
        }
        if matches!(style.size.height, crate::geometry::Length::Auto) {
            style.size.height = crate::geometry::Length::Definite(px(300.0).into());
        }
        let layout_id = window.request_layout(style, [], cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        #[cfg(feature = "webview")]
        {
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            let _ = &cx;

            if is_wayland() {
                // Wayland: separate GTK window (build_as_child not supported)
                #[cfg(all(feature = "gtk", any(target_os = "linux", target_os = "freebsd")))]
                gtk_thread::ensure_created(self.id, &self.content);
            } else {
                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                {
                    let scale_factor = window.scale_factor();
                    x11_child_thread::request_create(
                        self.id,
                        &self.content,
                        to_wry_bounds(bounds, scale_factor),
                        window,
                    );
                    x11_child_thread::request_set_bounds(self.id, bounds, scale_factor);
                }

                #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                {
                    let scale_factor = window.scale_factor();
                    let exists =
                        CHILD_WEBVIEWS.with(|children| children.borrow().contains_key(&self.id));
                    if exists {
                        update_child_bounds(self.id, bounds, scale_factor);
                    } else {
                        create_child_webview(
                            self.id,
                            self.content.clone(),
                            bounds,
                            scale_factor,
                            window,
                            cx,
                        );
                    }
                }
            }
        }
    }
}

impl IntoElement for WebView {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for WebView {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webview_from_html() {
        let webview = WebView::from_html(0, "<html><body>Hello</body></html>");
        assert!(matches!(webview.content, WebViewContent::Html(_)));
    }

    #[test]
    fn test_webview_from_url() {
        let webview = WebView::from_url(0, "https://example.com");
        assert!(matches!(webview.content, WebViewContent::Url(_)));
    }
}
