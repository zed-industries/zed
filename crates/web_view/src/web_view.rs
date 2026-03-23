mod element;
pub mod toolbar;

use gpui::{
    Action, App, ClipboardItem, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, Render, SharedString, Styled, Window, actions,
};
use schemars::JsonSchema;
use ui::prelude::*;
use util::ResultExt;
use workspace::Workspace;
use workspace::item::{Item, ItemEvent};

#[cfg(target_os = "linux")]
use gdkx11::{
    X11Display,
    glib::{Cast, ObjectType},
};
#[cfg(not(target_arch = "wasm32"))]
use std::cell::RefCell;
#[cfg(not(target_arch = "wasm32"))]
use std::rc::Rc;
#[cfg(not(target_arch = "wasm32"))]
use wry::WebView as WryWebView;
#[cfg(target_os = "linux")]
use wry::raw_window_handle::{
    HandleError, HasWindowHandle, RawWindowHandle, WindowHandle, XlibWindowHandle,
};
#[cfg(target_os = "linux")]
use x11_dl::xlib::{Display, Window as XWindow, Xlib};

actions!(
    web_view,
    [
        /// Open a web view tab
        Open,
        /// Reload the current page
        Reload,
        /// Navigate back
        GoBack,
        /// Navigate forward
        GoForward,
        /// Focus the URL bar
        FocusUrlBar,
        /// Copy the current URL
        CopyUrl,
    ]
);

#[derive(Clone, serde::Deserialize, PartialEq, JsonSchema, Action)]
#[action(namespace = web_view)]
pub struct OpenUrl {
    pub url: String,
}

pub fn init(cx: &mut App) {
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = gtk::init() {
            log::error!("Failed to initialize GTK: {}", e);
        }
    }

    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        WebView::register(workspace, window, cx);
    })
    .detach();
}

pub struct WebView {
    url: SharedString,
    title: Option<SharedString>,
    is_loading: bool,
    can_go_back: bool,
    can_go_forward: bool,
    request_focus_url_bar: bool,
    focus_handle: FocusHandle,
    error: Option<SharedString>,
    visible: bool,
    #[cfg(not(target_arch = "wasm32"))]
    wry_webview: Option<WryWebView>,
    #[cfg(not(target_arch = "wasm32"))]
    callback_state: Option<Rc<RefCell<WebViewCallbackState>>>,
    #[cfg(target_os = "linux")]
    x11_parent_window: Option<XWindow>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct WebViewCallbackState {
    pending_url: Option<String>,
    is_loading: bool,
}

#[cfg(target_os = "linux")]
struct LinuxX11WindowHandle {
    window: core::ffi::c_ulong,
    visual_id: core::ffi::c_ulong,
}

#[cfg(target_os = "linux")]
impl LinuxX11WindowHandle {
    fn from_window(window: &Window) -> Result<Self, String> {
        let raw_window_handle = HasWindowHandle::window_handle(window)
            .map_err(|error| format!("Failed to access window handle: {error}"))?
            .as_raw();

        match raw_window_handle {
            RawWindowHandle::Xcb(handle) => Ok(Self {
                window: handle.window.get().into(),
                visual_id: handle
                    .visual_id
                    .map(|visual_id| visual_id.get().into())
                    .unwrap_or_default(),
            }),
            RawWindowHandle::Xlib(handle) => Ok(Self {
                window: handle.window,
                visual_id: handle.visual_id,
            }),
            RawWindowHandle::Wayland(_) => {
                Err("Web views are not yet supported on Wayland. Please use X11.".to_string())
            }
            _ => Err("Web views are not supported for this Linux window backend.".to_string()),
        }
    }
}

#[cfg(target_os = "linux")]
impl HasWindowHandle for LinuxX11WindowHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let mut handle = XlibWindowHandle::new(self.window);
        handle.visual_id = self.visual_id;

        Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::Xlib(handle)) })
    }
}

impl WebView {
    fn register(workspace: &mut Workspace, _window: &mut Window, _cx: &mut Context<Workspace>) {
        workspace.register_action(|workspace, action: &OpenUrl, window, cx| {
            let web_view = cx.new(|cx| WebView::new(action.url.clone(), window, cx));
            workspace.add_item_to_active_pane(Box::new(web_view), None, true, window, cx);
        });

        workspace.register_action(|workspace, _: &Open, window, cx| {
            let web_view = cx.new(|cx| WebView::new("https://zed.dev".to_string(), window, cx));
            workspace.add_item_to_active_pane(Box::new(web_view), None, true, window, cx);
        });

        workspace.register_action(|workspace, _: &Reload, _window, cx| {
            if let Some(web_view) = Self::active_web_view(workspace, cx) {
                web_view.update(cx, |view, cx| view.reload_page(cx));
            }
        });

        workspace.register_action(|workspace, _: &GoBack, _window, cx| {
            if let Some(web_view) = Self::active_web_view(workspace, cx) {
                web_view.read(cx).go_back();
            }
        });

        workspace.register_action(|workspace, _: &GoForward, _window, cx| {
            if let Some(web_view) = Self::active_web_view(workspace, cx) {
                web_view.read(cx).go_forward();
            }
        });

        workspace.register_action(|workspace, _: &FocusUrlBar, _window, cx| {
            if let Some(web_view) = Self::active_web_view(workspace, cx) {
                web_view.update(cx, |view, cx| view.request_focus_url_bar(cx));
            }
        });

        workspace.register_action(|workspace, _: &CopyUrl, _window, cx| {
            if let Some(web_view) = Self::active_web_view(workspace, cx) {
                cx.write_to_clipboard(ClipboardItem::new_string(
                    web_view.read(cx).url().to_string(),
                ));
            }
        });
    }

    fn active_web_view(
        workspace: &Workspace,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<WebView>> {
        workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<WebView>(cx))
    }

    fn new(url: String, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            url: url.into(),
            title: None,
            is_loading: true,
            can_go_back: false,
            can_go_forward: false,
            request_focus_url_bar: false,
            focus_handle,
            error: None,
            visible: false,
            #[cfg(not(target_arch = "wasm32"))]
            wry_webview: None,
            #[cfg(not(target_arch = "wasm32"))]
            callback_state: None,
            #[cfg(target_os = "linux")]
            x11_parent_window: None,
        }
    }

    pub fn url(&self) -> &SharedString {
        &self.url
    }

    pub fn title(&self) -> Option<&SharedString> {
        self.title.as_ref()
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn can_go_back(&self) -> bool {
        self.can_go_back
    }

    pub fn can_go_forward(&self) -> bool {
        self.can_go_forward
    }

    pub fn take_focus_url_bar_request(&mut self) -> bool {
        std::mem::take(&mut self.request_focus_url_bar)
    }

    fn request_focus_url_bar(&mut self, cx: &mut Context<Self>) {
        self.request_focus_url_bar = true;
        cx.notify();
    }

    pub fn navigate(&mut self, url: String, cx: &mut Context<Self>) {
        self.url = url.into();
        self.is_loading = true;
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(webview) = &self.wry_webview {
            webview.load_url(self.url.as_ref()).log_err();
        }
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    pub fn reload_page(&mut self, cx: &mut Context<Self>) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(webview) = &self.wry_webview {
            webview.reload().log_err();
        }
        cx.notify();
    }

    pub fn go_back(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(webview) = &self.wry_webview {
            webview.evaluate_script("history.back()").log_err();
        }
    }

    pub fn go_forward(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(webview) = &self.wry_webview {
            webview.evaluate_script("history.forward()").log_err();
        }
    }

    fn set_visible(&mut self, visible: bool) {
        if self.visible == visible {
            return;
        }
        self.visible = visible;
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(webview) = &self.wry_webview {
            webview.set_visible(visible).log_err();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn ensure_webview(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.wry_webview.is_some() || self.error.is_some() {
            return;
        }

        let callback_state = Rc::new(RefCell::new(WebViewCallbackState {
            pending_url: None,
            is_loading: true,
        }));

        let navigation_state = callback_state.clone();
        let page_load_state = callback_state.clone();

        let webview_builder = wry::WebViewBuilder::new()
            .with_url(self.url.as_ref())
            .with_transparent(true)
            .with_visible(false)
            .with_navigation_handler(move |url| {
                if let Ok(mut state) = navigation_state.try_borrow_mut() {
                    state.pending_url = Some(url);
                    state.is_loading = true;
                }
                true
            })
            .with_on_page_load_handler(move |event, url| {
                use wry::PageLoadEvent;
                if let Ok(mut state) = page_load_state.try_borrow_mut() {
                    match event {
                        PageLoadEvent::Started => {
                            state.is_loading = true;
                            state.pending_url = Some(url);
                        }
                        PageLoadEvent::Finished => {
                            state.is_loading = false;
                            state.pending_url = Some(url);
                        }
                    }
                }
            })
            .with_new_window_req_handler(|url, _features| {
                open_url_in_system_browser(&url);
                wry::NewWindowResponse::Deny
            });

        self.callback_state = Some(callback_state);

        #[cfg(target_os = "linux")]
        let webview_result = match LinuxX11WindowHandle::from_window(window) {
            Ok(x11_window_handle) => {
                self.x11_parent_window = Some(x11_window_handle.window);
                webview_builder.build_as_child(&x11_window_handle)
            }
            Err(message) => {
                self.error = Some(message.into());
                cx.notify();
                return;
            }
        };

        #[cfg(not(target_os = "linux"))]
        let webview_result = webview_builder.build_as_child(window);

        match webview_result {
            Ok(webview) => {
                self.wry_webview = Some(webview);
            }
            Err(error) => {
                log::error!("Failed to create web view: {}", error);
                self.error = Some(format!("Failed to create web view: {error}").into());
                cx.notify();
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn sync_callback_state(&mut self, cx: &mut Context<Self>) {
        let Some(callback_state) = &self.callback_state else {
            return;
        };

        let Ok(mut state) = callback_state.try_borrow_mut() else {
            return;
        };

        let mut changed = false;

        if let Some(url) = state.pending_url.take() {
            if self.url.as_ref() != url {
                self.url = url.into();
                changed = true;
            }
        }

        if self.is_loading != state.is_loading {
            self.is_loading = state.is_loading;
            changed = true;
        }

        if changed {
            cx.emit(ItemEvent::UpdateTab);
            cx.notify();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn update_title_from_url(&mut self) {
        if let Ok(parsed) = url::Url::parse(self.url.as_ref()) {
            if let Some(host) = parsed.host_str() {
                let title: SharedString = host.to_string().into();
                if self.title.as_ref() != Some(&title) {
                    self.title = Some(title);
                }
            }
        }
    }

    pub fn paint_webview(
        &mut self,
        bounds: gpui::Bounds<gpui::Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.ensure_webview(window, cx);
            self.sync_callback_state(cx);

            if !self.is_loading {
                self.update_title_from_url();
                self.can_go_back = true;
            }

            if let Some(webview) = &self.wry_webview {
                let device_bounds = bounds.to_device_pixels(window.scale_factor());
                let rect = wry::Rect {
                    position: wry::dpi::PhysicalPosition::new(
                        device_bounds.origin.x.0,
                        device_bounds.origin.y.0,
                    )
                    .into(),
                    size: wry::dpi::PhysicalSize::new(
                        device_bounds.size.width.0.max(0) as u32,
                        device_bounds.size.height.0.max(0) as u32,
                    )
                    .into(),
                };
                webview.set_bounds(rect).log_err();
                #[cfg(target_os = "linux")]
                self.lower_x11_children();
            }
            self.set_visible(true);
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (bounds, window, cx);
        }
    }

    pub fn focus_webview(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(webview) = &self.wry_webview {
            webview.focus().log_err();
        }
    }

    #[cfg(target_os = "linux")]
    pub fn lower_x11_children(&self) {
        let Some(parent_xid) = self.x11_parent_window else {
            return;
        };

        let Some(gdk_display) = gtk::gdk::Display::default() else {
            return;
        };

        let gx11_display: &X11Display = match gdk_display.downcast_ref() {
            Some(d) => d,
            None => return,
        };

        let x11_display =
            unsafe { gdkx11::ffi::gdk_x11_display_get_xdisplay(gx11_display.as_ptr()) };

        if x11_display.is_null() {
            return;
        }

        let xlib = match Xlib::open() {
            Ok(xlib) => xlib,
            Err(_) => return,
        };

        unsafe {
            let mut root = 0;
            let mut parent = 0;
            let mut children: *mut XWindow = std::ptr::null_mut();
            let mut nchildren = 0;

            let status = (xlib.XQueryTree)(
                x11_display as *mut Display,
                parent_xid,
                &mut root,
                &mut parent,
                &mut children,
                &mut nchildren,
            );

            if status != 0 && !children.is_null() && nchildren > 0 {
                let children_slice = std::slice::from_raw_parts(children, nchildren as usize);
                for &child in children_slice {
                    (xlib.XLowerWindow)(x11_display as *mut Display, child);
                }
                (xlib.XFree)(children as *mut _);
            }

            (xlib.XFlush)(x11_display as *mut Display);
        }
    }
}

impl EventEmitter<ItemEvent> for WebView {}

impl Focusable for WebView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for WebView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().clone();
        if let Some(error) = &self.error {
            return div()
                .track_focus(&self.focus_handle)
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(cx.theme().colors().editor_background)
                .child(Label::new(error.clone()).color(Color::Error));
        }

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .child(element::WebViewElement::new(view))
    }
}

impl Item for WebView {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        self.title.clone().unwrap_or_else(|| self.url.clone())
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::ToolWeb))
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event)
    }

    fn deactivated(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.set_visible(false);
        #[cfg(target_os = "linux")]
        while gtk::events_pending() {
            gtk::main_iteration_do(false);
        }
    }

    fn workspace_deactivated(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.set_visible(false);
        #[cfg(target_os = "linux")]
        while gtk::events_pending() {
            gtk::main_iteration_do(false);
        }
    }
}

impl Drop for WebView {
    fn drop(&mut self) {
        self.set_visible(false);
        #[cfg(all(target_os = "linux", not(target_arch = "wasm32")))]
        {
            drop(self.wry_webview.take());
            while gtk::events_pending() {
                gtk::main_iteration_do(false);
            }
        }
    }
}

/// Open a URL as a new tab in the workspace.
pub fn open_url(
    url: &str,
    workspace: Entity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<WebView> {
    let url = url.to_string();
    let web_view = cx.new(|cx| WebView::new(url, window, cx));
    workspace.update(cx, |workspace, cx| {
        workspace.add_item_to_active_pane(Box::new(web_view.clone()), None, true, window, cx);
    });
    web_view
}

#[cfg(not(target_arch = "wasm32"))]
fn open_url_in_system_browser(url: &str) {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .log_err();
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .log_err();
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()
            .log_err();
    }
}
