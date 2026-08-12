#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn main() {
    eprintln!("The native_webview example is only available on macOS and Windows.");
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
mod platform {
    use std::rc::Rc;
    #[cfg(target_os = "windows")]
    use std::sync::mpsc;

    #[cfg(target_os = "macos")]
    use cocoa::{
        appkit::NSView,
        base::{YES, id, nil},
        foundation::{NSPoint, NSRect, NSSize, NSString},
    };
    use gpui::{
        App, Bounds, Context, Div, Element, ElementId, Entity, GlobalElementId, IntoElement,
        LayoutId, MouseButton, Pixels, SharedString, Stateful, Style, Window, WindowBounds,
        WindowComposition, WindowCompositionSurface, WindowOptions, deferred, div, prelude::*, px,
        relative, rgb, size,
    };
    use gpui_platform::application;
    #[cfg(target_os = "macos")]
    use objc::{class, msg_send, sel, sel_impl};
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};

    #[cfg(target_os = "windows")]
    use gpui::{
        DispatchPhase, Modifiers, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
        NavigationDirection, ScrollDelta, ScrollWheelEvent,
    };
    #[cfg(target_os = "windows")]
    use webview2_com::{
        CoTaskMemPWSTR, CreateCoreWebView2CompositionControllerCompletedHandler,
        CreateCoreWebView2EnvironmentCompletedHandler, Microsoft::Web::WebView2::Win32::*,
    };
    #[cfg(target_os = "windows")]
    use windows::{
        Win32::Foundation::{E_ABORT, E_POINTER, HWND, POINT, RECT},
        Win32::UI::{
            Input::KeyboardAndMouse::{GetFocus, SetFocus},
            WindowsAndMessaging::{XBUTTON1, XBUTTON2},
        },
        core::Interface,
    };

    #[cfg(target_os = "macos")]
    #[link(name = "WebKit", kind = "framework")]
    unsafe extern "C" {}

    const PAGE: &str = include_str!("native_webview.html");

    enum NativeWebViewRoot {
        Ready(Entity<NativeWebViewExample>),
        Error(SharedString),
    }

    impl Render for NativeWebViewRoot {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            match self {
                NativeWebViewRoot::Ready(example) => div().size_full().child(example.clone()),
                NativeWebViewRoot::Error(error) => div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(rgb(0x18191b))
                    .text_color(rgb(0xff6b6b))
                    .child(error.clone()),
            }
        }
    }

    struct NativeWebView {
        #[cfg(target_os = "macos")]
        gpui_view: id,
        surface: WindowCompositionSurface,
        #[cfg(target_os = "macos")]
        view: id,
        #[cfg(target_os = "windows")]
        hwnd: HWND,
        #[cfg(target_os = "windows")]
        controller: ICoreWebView2CompositionController,
        #[cfg(target_os = "windows")]
        webview_controller: ICoreWebView2Controller,
        #[cfg(target_os = "windows")]
        #[allow(dead_code)]
        webview: ICoreWebView2,
    }

    impl NativeWebView {
        #[cfg(target_os = "macos")]
        fn new(window: &Window, composition: &WindowComposition<'_>) -> anyhow::Result<Self> {
            let window_handle = HasWindowHandle::window_handle(window).map_err(|error| {
                anyhow::anyhow!("failed to get AppKit window handle: {error:?}")
            })?;
            let gpui_view = match window_handle.as_raw() {
                RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr() as id,
                _ => anyhow::bail!("native_webview requires an AppKit window"),
            };
            let surface = composition.create_native_surface()?;
            let surface_handle = surface.platform_surface()?.platform_handle()?;
            let parent = surface_handle
                .downcast::<usize>()
                .map(|handle| *handle as id)
                .map_err(|_| anyhow::anyhow!("native surface did not provide an AppKit view"))?;

            unsafe {
                let configuration: id = msg_send![class!(WKWebViewConfiguration), new];
                let view: id = msg_send![class!(WKWebView), alloc];
                let view: id = msg_send![
                    view,
                    initWithFrame: NSRect::new(
                        NSPoint::new(0., 0.),
                        NSSize::new(0., 0.),
                    )
                    configuration: configuration
                ];
                if view.is_null() {
                    let _: () = msg_send![configuration, release];
                    anyhow::bail!("failed to create WKWebView");
                }

                let _: () = msg_send![view, setWantsLayer: YES];
                view.setAutoresizingMask_(
                    cocoa::appkit::NSViewWidthSizable | cocoa::appkit::NSViewHeightSizable,
                );
                let layer: id = msg_send![view, layer];
                let border_color: id = msg_send![
                    class!(NSColor),
                    colorWithSRGBRed: 63. / 255.
                    green: 64. / 255.
                    blue: 67. / 255.
                    alpha: 1.
                ];
                let border_color: id = msg_send![border_color, CGColor];
                let _: () = msg_send![layer, setMasksToBounds: YES];
                let _: () = msg_send![layer, setCornerRadius: 8.];
                let _: () = msg_send![layer, setBorderWidth: 1.];
                let _: () = msg_send![layer, setBorderColor: border_color];

                #[allow(
                    clippy::disallowed_methods,
                    reason = "the owned NSString is explicitly released after loading"
                )]
                let html = NSString::alloc(nil).init_str(PAGE);
                let _: id = msg_send![view, loadHTMLString: html baseURL: nil];
                parent.addSubview_(view);

                let _: () = msg_send![html, release];
                let _: () = msg_send![configuration, release];

                Ok(Self {
                    gpui_view,
                    surface,
                    view,
                })
            }
        }

        #[cfg(target_os = "windows")]
        fn new(window: &Window, composition: &WindowComposition<'_>) -> anyhow::Result<Self> {
            let window_handle = HasWindowHandle::window_handle(window)
                .map_err(|error| anyhow::anyhow!("failed to get Win32 window handle: {error:?}"))?;
            let hwnd = match window_handle.as_raw() {
                RawWindowHandle::Win32(handle) => HWND(handle.hwnd.get() as _),
                _ => anyhow::bail!("native_webview requires a Win32 window"),
            };
            let surface = composition.create_native_surface()?;
            let visual = surface.platform_surface()?.platform_handle()?;
            let visual = visual.downcast::<windows::core::IUnknown>().map_err(|_| {
                anyhow::anyhow!("native surface did not provide a DirectComposition visual")
            })?;

            let (environment_sender, environment_receiver) = mpsc::channel();
            CreateCoreWebView2EnvironmentCompletedHandler::wait_for_async_operation(
                Box::new(|handler| unsafe {
                    CreateCoreWebView2Environment(&handler)
                        .map_err(webview2_com::Error::WindowsError)
                }),
                Box::new(move |error, environment| {
                    let result = error.and_then(|_| {
                        environment.ok_or_else(|| windows::core::Error::from(E_POINTER))
                    });
                    environment_sender
                        .send(result)
                        .map_err(|_| windows::core::Error::from(E_ABORT))?;
                    Ok(())
                }),
            )?;
            let environment = webview2_com::wait_with_pump(environment_receiver)??;
            let environment = environment.cast::<ICoreWebView2Environment3>()?;
            let (controller_sender, controller_receiver) = mpsc::channel();
            CreateCoreWebView2CompositionControllerCompletedHandler::wait_for_async_operation(
                Box::new(move |handler| unsafe {
                    environment
                        .CreateCoreWebView2CompositionController(hwnd, &handler)
                        .map_err(webview2_com::Error::WindowsError)
                }),
                Box::new(move |error, controller| {
                    let result = error.and_then(|_| {
                        controller.ok_or_else(|| windows::core::Error::from(E_POINTER))
                    });
                    controller_sender
                        .send(result)
                        .map_err(|_| windows::core::Error::from(E_ABORT))?;
                    Ok(())
                }),
            )?;
            let controller = webview2_com::wait_with_pump(controller_receiver)??;
            unsafe { controller.SetRootVisualTarget(&*visual)? };
            let webview_controller = controller.cast::<ICoreWebView2Controller>()?;
            unsafe { webview_controller.SetIsVisible(true)? };
            let webview = unsafe { webview_controller.CoreWebView2()? };
            let html = CoTaskMemPWSTR::from(PAGE);
            unsafe { webview.NavigateToString(*html.as_ref().as_pcwstr())? };

            Ok(Self {
                surface,
                hwnd,
                controller,
                webview_controller,
                webview,
            })
        }

        fn set_bounds(&self, bounds: Bounds<Pixels>, scale_factor: f32) -> anyhow::Result<()> {
            self.surface
                .platform_surface()?
                .set_bounds(bounds.to_device_pixels(scale_factor))?;
            #[cfg(target_os = "windows")]
            unsafe {
                let device_bounds = bounds.to_device_pixels(scale_factor);
                self.webview_controller.SetBounds(RECT {
                    left: 0,
                    top: 0,
                    right: device_bounds.size.width.0,
                    bottom: device_bounds.size.height.0,
                })?;
                self.webview_controller.SetIsVisible(true)?;
            }
            Ok(())
        }

        #[cfg(target_os = "macos")]
        fn focus_parent(&self) {
            unsafe {
                let window: id = msg_send![self.view, window];
                if !window.is_null() {
                    let _: bool = msg_send![window, makeFirstResponder: self.gpui_view];
                }
            }
        }

        #[cfg(target_os = "windows")]
        fn focus_parent(&self) {
            unsafe {
                if let Err(error) = SetFocus(Some(self.hwnd))
                    && GetFocus() != self.hwnd
                {
                    log::error!("failed to return keyboard focus to the GPUI window: {error}");
                }
            }
        }

        #[cfg(target_os = "windows")]
        fn navigation_mouse_data(button: MouseButton) -> u32 {
            match button {
                MouseButton::Navigate(NavigationDirection::Back) => u32::from(XBUTTON1),
                MouseButton::Navigate(NavigationDirection::Forward) => u32::from(XBUTTON2),
                _ => 0,
            }
        }

        #[cfg(target_os = "windows")]
        fn send_mouse_input(
            &self,
            event_kind: COREWEBVIEW2_MOUSE_EVENT_KIND,
            button: Option<MouseButton>,
            modifiers: Modifiers,
            mouse_data: u32,
            position: gpui::Point<Pixels>,
            bounds: Bounds<Pixels>,
            scale_factor: f32,
        ) {
            if !bounds.contains(&position) {
                return;
            }
            let mut virtual_keys = COREWEBVIEW2_MOUSE_EVENT_VIRTUAL_KEYS_NONE;
            if modifiers.control {
                virtual_keys |= COREWEBVIEW2_MOUSE_EVENT_VIRTUAL_KEYS_CONTROL;
            }
            if modifiers.shift {
                virtual_keys |= COREWEBVIEW2_MOUSE_EVENT_VIRTUAL_KEYS_SHIFT;
            }
            if let Some(button) = button {
                virtual_keys |= match button {
                    MouseButton::Left => COREWEBVIEW2_MOUSE_EVENT_VIRTUAL_KEYS_LEFT_BUTTON,
                    MouseButton::Right => COREWEBVIEW2_MOUSE_EVENT_VIRTUAL_KEYS_RIGHT_BUTTON,
                    MouseButton::Middle => COREWEBVIEW2_MOUSE_EVENT_VIRTUAL_KEYS_MIDDLE_BUTTON,
                    MouseButton::Navigate(NavigationDirection::Back) => {
                        COREWEBVIEW2_MOUSE_EVENT_VIRTUAL_KEYS_X_BUTTON1
                    }
                    MouseButton::Navigate(NavigationDirection::Forward) => {
                        COREWEBVIEW2_MOUSE_EVENT_VIRTUAL_KEYS_X_BUTTON2
                    }
                };
            }
            let point = POINT {
                x: (f32::from(position.x - bounds.origin.x) * scale_factor) as i32,
                y: (f32::from(position.y - bounds.origin.y) * scale_factor) as i32,
            };
            unsafe {
                if let Err(error) =
                    self.controller
                        .SendMouseInput(event_kind, virtual_keys, mouse_data, point)
                {
                    log::error!("failed to send mouse input to WebView2: {error}");
                }
            }
        }
    }

    impl Drop for NativeWebView {
        #[cfg(target_os = "macos")]
        fn drop(&mut self) {
            unsafe {
                NSView::removeFromSuperview(self.view);
                let _: () = msg_send![self.view, release];
            }
        }

        #[cfg(target_os = "windows")]
        fn drop(&mut self) {
            unsafe {
                if let Err(error) = self.webview_controller.Close() {
                    log::error!("failed to close WebView2: {error}");
                }
            }
        }
    }

    struct NativeWebViewElement {
        webview: Rc<NativeWebView>,
    }

    impl IntoElement for NativeWebViewElement {
        type Element = Self;

        fn into_element(self) -> Self::Element {
            self
        }
    }

    impl Element for NativeWebViewElement {
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
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            window: &mut Window,
            cx: &mut App,
        ) -> (LayoutId, Self::RequestLayoutState) {
            let mut style = Style::default();
            style.size.width = relative(1.).into();
            style.size.height = relative(1.).into();
            (window.request_layout(style, [], cx), ())
        }

        fn prepaint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            window: &mut Window,
            _cx: &mut App,
        ) -> Self::PrepaintState {
            if let Err(error) = self.webview.set_bounds(bounds, window.scale_factor()) {
                log::error!("failed to update native WebView surface bounds: {error:#}");
            }
        }

        fn paint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            _bounds: Bounds<Pixels>,
            _request_layout: &mut Self::RequestLayoutState,
            _prepaint: &mut Self::PrepaintState,
            _window: &mut Window,
            _cx: &mut App,
        ) {
            #[cfg(target_os = "windows")]
            {
                let bounds = _bounds;
                let window = _window;
                let scale_factor = window.scale_factor();
                let webview = self.webview.clone();
                window.on_mouse_event({
                    let webview = webview.clone();
                    move |event: &MouseDownEvent, phase, _, _| {
                        if phase == DispatchPhase::Bubble {
                            let event_kind = match event.button {
                                MouseButton::Left => COREWEBVIEW2_MOUSE_EVENT_KIND_LEFT_BUTTON_DOWN,
                                MouseButton::Right => {
                                    COREWEBVIEW2_MOUSE_EVENT_KIND_RIGHT_BUTTON_DOWN
                                }
                                MouseButton::Middle => {
                                    COREWEBVIEW2_MOUSE_EVENT_KIND_MIDDLE_BUTTON_DOWN
                                }
                                MouseButton::Navigate(NavigationDirection::Back) => {
                                    COREWEBVIEW2_MOUSE_EVENT_KIND_X_BUTTON_DOWN
                                }
                                MouseButton::Navigate(NavigationDirection::Forward) => {
                                    COREWEBVIEW2_MOUSE_EVENT_KIND_X_BUTTON_DOWN
                                }
                            };
                            webview.send_mouse_input(
                                event_kind,
                                Some(event.button),
                                event.modifiers,
                                NativeWebView::navigation_mouse_data(event.button),
                                event.position,
                                bounds,
                                scale_factor,
                            );
                        }
                    }
                });
                window.on_mouse_event({
                    let webview = webview.clone();
                    move |event: &MouseUpEvent, phase, _, _| {
                        if phase == DispatchPhase::Bubble {
                            let event_kind = match event.button {
                                MouseButton::Left => COREWEBVIEW2_MOUSE_EVENT_KIND_LEFT_BUTTON_UP,
                                MouseButton::Right => COREWEBVIEW2_MOUSE_EVENT_KIND_RIGHT_BUTTON_UP,
                                MouseButton::Middle => {
                                    COREWEBVIEW2_MOUSE_EVENT_KIND_MIDDLE_BUTTON_UP
                                }
                                MouseButton::Navigate(NavigationDirection::Back) => {
                                    COREWEBVIEW2_MOUSE_EVENT_KIND_X_BUTTON_UP
                                }
                                MouseButton::Navigate(NavigationDirection::Forward) => {
                                    COREWEBVIEW2_MOUSE_EVENT_KIND_X_BUTTON_UP
                                }
                            };
                            webview.send_mouse_input(
                                event_kind,
                                Some(event.button),
                                event.modifiers,
                                NativeWebView::navigation_mouse_data(event.button),
                                event.position,
                                bounds,
                                scale_factor,
                            );
                        }
                    }
                });
                window.on_mouse_event({
                    let webview = webview.clone();
                    move |event: &MouseMoveEvent, phase, _, _| {
                        if phase == DispatchPhase::Bubble {
                            webview.send_mouse_input(
                                COREWEBVIEW2_MOUSE_EVENT_KIND_MOVE,
                                event.pressed_button,
                                event.modifiers,
                                0,
                                event.position,
                                bounds,
                                scale_factor,
                            );
                        }
                    }
                });
                window.on_mouse_event(move |event: &ScrollWheelEvent, phase, _, _| {
                    if phase == DispatchPhase::Bubble {
                        let amount = match event.delta {
                            ScrollDelta::Pixels(delta) => f32::from(delta.y),
                            ScrollDelta::Lines(delta) => delta.y * 120.0,
                        };
                        webview.send_mouse_input(
                            COREWEBVIEW2_MOUSE_EVENT_KIND_WHEEL,
                            None,
                            event.modifiers,
                            amount as i32 as u32,
                            event.position,
                            bounds,
                            scale_factor,
                        );
                    }
                });
            }
        }
    }

    struct NativeWebViewExample {
        webview: Rc<NativeWebView>,
        about_active: bool,
        dialog_open: bool,
        menu_open: bool,
        popover_open: bool,
    }

    fn button(id: &'static str, label: &'static str) -> Stateful<Div> {
        div()
            .id(id)
            .px_4()
            .py_2()
            .rounded_md()
            .border_1()
            .border_color(rgb(0x3f4043))
            .bg(rgb(0x1f2127))
            .text_color(rgb(0xbfbdb6))
            .text_sm()
            .cursor_pointer()
            .hover(|style| style.bg(rgb(0x2d2f34)).border_color(rgb(0x3e4043)))
            .child(label)
    }

    fn tab(id: &'static str, label: &'static str, active: bool) -> Stateful<Div> {
        div()
            .id(id)
            .px_3()
            .py_2()
            .border_b_2()
            .border_color(if active { rgb(0x5ac1fe) } else { rgb(0x313337) })
            .text_color(if active { rgb(0xbfbdb6) } else { rgb(0x8a8986) })
            .text_sm()
            .cursor_pointer()
            .hover(|style| style.text_color(rgb(0xbfbdb6)))
            .child(label)
    }

    fn menu_item(id: &'static str, label: &'static str) -> Stateful<Div> {
        div()
            .id(id)
            .px_2()
            .py_1()
            .rounded_md()
            .text_xs()
            .text_color(rgb(0xbfbdb6))
            .cursor_pointer()
            .hover(|style| style.bg(rgb(0x2d2f34)))
            .child(label)
    }

    fn layer_row(index: &'static str, label: &'static str, color: gpui::Rgba) -> Div {
        div()
            .flex()
            .items_center()
            .gap_2()
            .py_2()
            .border_b_1()
            .border_color(rgb(0x3f4043))
            .child(div().w_1().h_5().rounded_sm().bg(color))
            .child(div().text_color(rgb(0x696a6a)).w(px(18.)).child(index))
            .child(div().text_color(rgb(0xbfbdb6)).child(label))
    }

    impl Render for NativeWebViewExample {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("native-webview-example")
                .relative()
                .flex()
                .gap_6()
                .size_full()
                .p_7()
                .bg(rgb(0x313337))
                .text_color(rgb(0xbfbdb6))
                // While a deferred overlay is visible, the transparent GPUI
                // NSView captures the whole window. This handler therefore
                // also dismisses clicks geometrically over the WKWebView.
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.webview.focus_parent();
                        this.popover_open = false;
                        this.dialog_open = false;
                        this.menu_open = false;
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .justify_between()
                        .w(px(180.))
                        .child(
                            div()
                                .child(
                                    div()
                                        .mt(px(-2.))
                                        .text_lg()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .text_color(rgb(0xfeb454))
                                        .child("GPUI NATIVE WEBVIEW"),
                                )
                                .child(div().mt_2().text_sm().text_color(rgb(0x8a8986)).child(
                                    "Native composition.\nThree surfaces, one visual stack.",
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .text_xs()
                                .child(layer_row("03", "GPUI overlay", rgb(0xfeb454)))
                                .child(layer_row("02", "WKWebView", rgb(0x5ac1fe)))
                                .child(layer_row("01", "GPUI base", rgb(0x8a8986))),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_w_0()
                        .h_full()
                        .gap_4()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0x8a8986))
                                                .child("COMPOSITION TARGET"),
                                        )
                                        .child(
                                            div().mt_1().text_lg().child("WebView overlay proof"),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .gap_2()
                                        .child(
                                            button("toggle-popover", "Show popover").on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    cx.stop_propagation();
                                                    this.webview.focus_parent();
                                                    this.popover_open = !this.popover_open;
                                                    this.menu_open = false;
                                                    cx.notify();
                                                }),
                                            ),
                                        )
                                        .child(button("open-dialog", "Open dialog").on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                cx.stop_propagation();
                                                this.webview.focus_parent();
                                                this.dialog_open = true;
                                                this.menu_open = false;
                                                cx.notify();
                                            }),
                                        )),
                                ),
                        )
                        .child(
                            div()
                                .relative()
                                .flex()
                                .items_center()
                                .justify_between()
                                .border_b_1()
                                .border_color(rgb(0x3f4043))
                                .child(
                                    div()
                                        .flex()
                                        .child(
                                            tab("webview-tab", "WebView", !self.about_active)
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _, _, cx| {
                                                        cx.stop_propagation();
                                                        this.about_active = false;
                                                        this.popover_open = false;
                                                        this.dialog_open = false;
                                                        this.menu_open = false;
                                                        cx.notify();
                                                    }),
                                                ),
                                        )
                                        .child(
                                            tab("about-tab", "About", self.about_active)
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _, _, cx| {
                                                        cx.stop_propagation();
                                                        this.webview.focus_parent();
                                                        this.about_active = true;
                                                        this.popover_open = false;
                                                        this.dialog_open = false;
                                                        this.menu_open = false;
                                                        cx.notify();
                                                    }),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .id("popup-menu-trigger")
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .w(px(28.))
                                        .h(px(28.))
                                        .rounded_md()
                                        .text_base()
                                        .text_color(rgb(0x8a8986))
                                        .cursor_pointer()
                                        .hover(|style| {
                                            style.bg(rgb(0x2d2f34)).text_color(rgb(0xbfbdb6))
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                cx.stop_propagation();
                                                this.webview.focus_parent();
                                                this.menu_open = !this.menu_open;
                                                cx.notify();
                                            }),
                                        )
                                        .child("…"),
                                )
                                .when(self.menu_open, |tab_bar| {
                                    tab_bar.child(
                                        deferred(
                                            div()
                                                .absolute()
                                                .top(px(34.))
                                                .right_0()
                                                .w(px(180.))
                                                .p_1()
                                                .rounded_lg()
                                                .border_1()
                                                .border_color(rgb(0x3f4043))
                                                .bg(rgb(0x1f2127))
                                                .shadow_xl()
                                                .child(menu_item(
                                                    "popup-menu-reload",
                                                    "Reload WebView",
                                                ))
                                                .child(menu_item(
                                                    "popup-menu-inspect",
                                                    "Inspect native surface",
                                                ))
                                                .child(div().my_1().h(px(1.)).bg(rgb(0x3f4043)))
                                                .child(menu_item(
                                                    "popup-menu-about",
                                                    "About this example",
                                                ))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _, _, cx| {
                                                        cx.stop_propagation();
                                                        this.menu_open = false;
                                                        cx.notify();
                                                    }),
                                                ),
                                        )
                                        .priority(2),
                                    )
                                }),
                        )
                        .child(
                            div()
                                .relative()
                                .flex_1()
                                .min_h_0()
                                .child(NativeWebViewElement {
                                    webview: self.webview.clone(),
                                })
                                .when(self.about_active, |content| {
                                    content.child(
                                        deferred(
                                            div()
                                                .absolute()
                                                .inset_0()
                                                .flex()
                                                .flex_col()
                                                .justify_center()
                                                .p_10()
                                                .rounded_lg()
                                                .bg(rgb(0x0d1016))
                                                .text_color(rgb(0xbfbdb6))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x5ac1fe))
                                                        .child("GPUI OVERLAY CONTENT"),
                                                )
                                                .child(
                                                    div()
                                                        .mt_3()
                                                        .text_3xl()
                                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                                        .child("A regular rendered view"),
                                                )
                                                .child(
                                                    div()
                                                        .mt_4()
                                                        .max_w(px(520.))
                                                        .text_color(rgb(0x8a8986))
                                                        .line_height(relative(1.6))
                                                        .child(
                                                            "This tab is rendered by GPUI above \
                                                             the native WebView. It verifies that \
                                                             non-popup content can replace and \
                                                             fully occlude a native surface.",
                                                        ),
                                                ),
                                        )
                                        .priority(1),
                                    )
                                }),
                        ),
                )
                .when(self.popover_open, |root| {
                    root.child(
                        deferred(
                            div()
                                .absolute()
                                .top(px(92.))
                                .right(px(42.))
                                .w(px(300.))
                                .p_4()
                                .rounded_lg()
                                .shadow_xl()
                                .border_1()
                                .border_color(rgb(0x3f4043))
                                .bg(rgb(0x1f2127))
                                .text_color(rgb(0xbfbdb6))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0xfeb454))
                                        .child("SURFACE 03"),
                                )
                                .child(
                                    div()
                                        .mt_2()
                                        .font_weight(gpui::FontWeight::SEMIBOLD)
                                        .child("Deferred GPUI popover"),
                                )
                                .child(div().mt_2().text_sm().text_color(rgb(0x8a8986)).child(
                                    "Painted after the native WebView without changing its \
                                         AppKit z-order.",
                                )),
                        )
                        .priority(3),
                    )
                })
                .when(self.dialog_open, |root| {
                    root.child(
                        deferred(
                            div()
                                .absolute()
                                .inset_0()
                                .flex()
                                .items_center()
                                .justify_center()
                                .bg(rgb(0x0d1016).opacity(0.72))
                                .child(
                                    div()
                                        .w(px(500.))
                                        .p_7()
                                        .rounded_xl()
                                        .shadow_xl()
                                        .border_1()
                                        .border_color(rgb(0x3f4043))
                                        .bg(rgb(0x1f2127))
                                        .text_color(rgb(0xbfbdb6))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0xfeb454))
                                                        .child("SURFACE 03 / GPUI OVERLAY"),
                                                )
                                                .child(
                                                    div()
                                                        .px_2()
                                                        .py_1()
                                                        .rounded_md()
                                                        .bg(rgb(0x2d2f34))
                                                        .text_xs()
                                                        .text_color(rgb(0x8a8986))
                                                        .child("LIVE"),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .mt_5()
                                                .text_2xl()
                                                .font_weight(gpui::FontWeight::SEMIBOLD)
                                                .child(
                                                    "The native layer stays exactly where it is.",
                                                ),
                                        )
                                        .child(div().mt_3().text_color(rgb(0x8a8986)).child(
                                            "GPUI splits its scene before deferred draws. \
                                                     AppKit places WKWebView between the base and \
                                                     this transparent overlay surface.",
                                        ))
                                        .child(div().mt_5().h(px(1.)).w_full().bg(rgb(0x3f4043)))
                                        .child(div().mt_5().flex().child(
                                            button("close-dialog", "Close").on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(|this, _, _, cx| {
                                                    cx.stop_propagation();
                                                    this.webview.focus_parent();
                                                    this.dialog_open = false;
                                                    cx.notify();
                                                }),
                                            ),
                                        )),
                                ),
                        )
                        .priority(4),
                    )
                })
        }
    }

    pub fn run() {
        application().run(|cx: &mut App| {
            let bounds = Bounds::centered(None, size(px(900.), px(640.)), cx);
            let result = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window: &mut Window, cx: &mut App| {
                    let webview = window.enable_window_composition().and_then(|composition| {
                        NativeWebView::new(window, &composition).map(Rc::new)
                    });
                    match webview {
                        Ok(webview) => {
                            let example = cx.new(|_| NativeWebViewExample {
                                webview,
                                about_active: false,
                                dialog_open: false,
                                menu_open: false,
                                popover_open: false,
                            });
                            cx.new(|_| NativeWebViewRoot::Ready(example))
                        }
                        Err(error) => cx.new(|_| {
                            NativeWebViewRoot::Error(
                                format!("Failed to initialize native WebView: {error:#}").into(),
                            )
                        }),
                    }
                },
            );
            if let Err(error) = result {
                eprintln!("failed to open native WebView example: {error:#}");
                cx.quit();
                return;
            }
            cx.activate(true);
        });
    }

    #[cfg(all(test, target_os = "windows"))]
    mod tests {
        use super::*;

        #[test]
        fn navigation_mouse_buttons_include_their_xbutton_identity() {
            assert_eq!(
                NativeWebView::navigation_mouse_data(MouseButton::Navigate(
                    NavigationDirection::Back
                )),
                u32::from(XBUTTON1)
            );
            assert_eq!(
                NativeWebView::navigation_mouse_data(MouseButton::Navigate(
                    NavigationDirection::Forward
                )),
                u32::from(XBUTTON2)
            );
            assert_eq!(NativeWebView::navigation_mouse_data(MouseButton::Left), 0);
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn main() {
    platform::run();
}
