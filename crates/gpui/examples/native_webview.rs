#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_os = "freebsd"
)))]
fn main() {
    eprintln!("The native_webview example is only available on macOS, Windows, and Linux.");
}

#[cfg(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_os = "freebsd"
))]
mod platform {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    use std::cell::RefCell;
    use std::rc::Rc;
    #[cfg(target_os = "windows")]
    use std::sync::mpsc;
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    use std::time::Instant;

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
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    use raw_window_handle::HasWindowHandle;
    use raw_window_handle::RawWindowHandle;
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    use raw_window_handle::{HasDisplayHandle, RawDisplayHandle};

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

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    const PAGE: &str = include_str!("native_webview.html");

    #[cfg(target_os = "macos")]
    const NATIVE_LAYER_LABEL: &str = "WKWebView";
    #[cfg(target_os = "windows")]
    const NATIVE_LAYER_LABEL: &str = "WebView2";
    // WebKitGTK cannot render into another client's Wayland surface, so Linux
    // demonstrates the native layer with content from an independent wgpu
    // device instead of a browser engine.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    const NATIVE_LAYER_LABEL: &str = "Native wgpu surface";

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    mod copy {
        pub const TITLE: &str = "GPUI NATIVE WEBVIEW";
        pub const HEADING: &str = "WebView overlay proof";
        pub const NATIVE_TAB: &str = "WebView";
        pub const RELOAD: &str = "Reload WebView";
        pub const ABOUT: &str = "This tab is rendered by GPUI above the native WebView. It \
                                 verifies that non-popup content can replace and fully occlude \
                                 a native surface.";
        pub const POPOVER: &str = "Painted after the native WebView without changing its AppKit \
                                   z-order.";
        pub const DIALOG: &str = "GPUI splits its scene before deferred draws. AppKit places \
                                  WKWebView between the base and this transparent overlay \
                                  surface.";
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    mod copy {
        pub const TITLE: &str = "GPUI NATIVE SURFACE";
        pub const HEADING: &str = "Native surface overlay proof";
        pub const NATIVE_TAB: &str = "Native";
        pub const RELOAD: &str = "Reload native surface";
        pub const ABOUT: &str = "This tab is rendered by GPUI above the native surface. It \
                                 verifies that non-popup content can replace and fully occlude \
                                 a native surface.";
        pub const POPOVER: &str = "Painted after the native surface without changing its \
                                   Wayland or X11 stacking.";
        pub const DIALOG: &str = "GPUI splits its scene before deferred draws. Wayland stacks \
                                  the native subsurface between the base and this overlay; X11 \
                                  cuts the overlay out of the native child window.";
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    const NATIVE_SHADER: &str = r#"
struct Globals {
    size: vec2<f32>,
    time: f32,
    _padding: f32,
};

@group(0) @binding(0) var<uniform> globals: Globals;

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let uv = vec2<f32>(f32((index << 1u) & 2u), f32(index & 2u));
    return vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
}

// 5x7 glyphs, one bit per pixel in row-major order: ' ABCDEFGINOPRSTUVWY'
var<private> FONT: array<vec2<u32>, 19> = array<vec2<u32>, 19>(
    vec2<u32>(0u, 0u),
    vec2<u32>(1663026734u, 4u),
    vec2<u32>(3809986095u, 3u),
    vec2<u32>(2718991918u, 3u),
    vec2<u32>(3810051631u, 3u),
    vec2<u32>(3256321087u, 7u),
    vec2<u32>(1108837439u, 0u),
    vec2<u32>(2736686638u, 7u),
    vec2<u32>(2286030990u, 3u),
    vec2<u32>(1662834289u, 4u),
    vec2<u32>(2736309806u, 3u),
    vec2<u32>(1108854319u, 0u),
    vec2<u32>(1381484079u, 4u),
    vec2<u32>(3775333438u, 3u),
    vec2<u32>(138547359u, 1u),
    vec2<u32>(2736309809u, 3u),
    vec2<u32>(353945137u, 1u),
    vec2<u32>(2874852913u, 2u),
    vec2<u32>(138553905u, 1u),
);
// NATIVE SURFACE / DRAWN BY ITS OWN WGPU DEVICE / NOT BY GPUI
var<private> TEXT: array<u32, 53> = array<u32, 53>(
    9u, 1u, 14u, 8u, 16u, 5u, 0u, 13u, 15u, 12u, 6u, 1u, 3u, 5u, 4u, 12u,
    1u, 17u, 9u, 0u, 2u, 18u, 0u, 8u, 14u, 13u, 0u, 10u, 17u, 9u, 0u, 17u,
    7u, 11u, 15u, 0u, 4u, 5u, 16u, 8u, 3u, 5u, 9u, 10u, 14u, 0u, 2u, 18u,
    0u, 7u, 11u, 15u, 8u,
);
const LINE_STARTS = vec3<u32>(0u, 14u, 42u);
const LINE_LENGTHS = vec3<u32>(14u, 28u, 11u);

fn glyph_pixel(glyph: u32, column: u32, row: u32) -> f32 {
    let bits = FONT[glyph];
    let bit = row * 5u + column;
    var word = bits.x;
    var shift = bit;
    if (bit >= 32u) {
        word = bits.y;
        shift = bit - 32u;
    }
    return f32((word >> shift) & 1u);
}

fn text_line(position: vec2<f32>, start: u32, length: u32, top: f32, scale: f32) -> f32 {
    let width = f32(length) * 6.0 * scale - scale;
    let left = floor((globals.size.x - width) * 0.5);
    let local = (position - vec2<f32>(left, top)) / scale;
    if (local.x < 0.0 || local.y < 0.0 || local.y >= 7.0 || local.x >= f32(length) * 6.0) {
        return 0.0;
    }
    let cell = u32(local.x / 6.0);
    let column = u32(local.x) - cell * 6u;
    if (column >= 5u) {
        return 0.0;
    }
    return glyph_pixel(TEXT[start + cell], column, u32(local.y));
}

@fragment
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let size = max(globals.size, vec2<f32>(1.0));
    let x = position.x / size.x;
    var intensity = 0.0;
    for (var index = 0; index < 5; index++) {
        let offset = f32(index);
        let amplitude = size.y * (0.08 + 0.04 * offset);
        let frequency = 6.2831853 * (1.0 + 0.25 * offset);
        let phase = globals.time * (0.6 + 0.15 * offset) + offset * 0.9;
        let curve = size.y * 0.5 + amplitude * sin(x * frequency + phase);
        let slope = amplitude * frequency / size.x * cos(x * frequency + phase);
        let distance = abs(position.y - curve) / sqrt(1.0 + slope * slope);
        let line = 1.0 - smoothstep(0.5, 1.5, distance);
        intensity = max(intensity, line * (1.0 - 0.16 * offset));
    }
    var color = mix(vec3<f32>(0.04), vec3<f32>(0.92), intensity);

    let title_fit = size.x * 0.6 / (f32(LINE_LENGTHS.x) * 6.0);
    let body_fit = size.x * 0.75 / (f32(max(LINE_LENGTHS.y, LINE_LENGTHS.z)) * 6.0);
    let title_scale = clamp(floor(min(size.y / 110.0, title_fit)), 1.0, 8.0);
    let body_scale = clamp(floor(min(max(title_scale * 0.5, 2.0), body_fit)), 1.0, 4.0);
    let widest = max(
        f32(LINE_LENGTHS.x) * 6.0 * title_scale,
        max(f32(LINE_LENGTHS.y), f32(LINE_LENGTHS.z)) * 6.0 * body_scale,
    );
    let text_height = 7.0 * title_scale + 19.0 * body_scale;
    let padding = 6.0 * body_scale;
    let top = floor((size.y - text_height) * 0.5);
    let panel_min = vec2<f32>((size.x - widest) * 0.5, top) - padding;
    let panel_max = vec2<f32>((size.x + widest) * 0.5, top + text_height) + padding;
    if (all(position.xy >= panel_min) && all(position.xy <= panel_max)) {
        color = mix(color, vec3<f32>(0.04), 0.9);
    }
    var text = text_line(position.xy, LINE_STARTS.x, LINE_LENGTHS.x, top, title_scale);
    let body_top = top + 7.0 * title_scale + 5.0 * body_scale;
    text = max(text, 0.6 * text_line(position.xy, LINE_STARTS.y, LINE_LENGTHS.y, body_top, body_scale));
    text = max(
        text,
        0.6 * text_line(position.xy, LINE_STARTS.z, LINE_LENGTHS.z, body_top + 10.0 * body_scale, body_scale),
    );
    color = mix(color, vec3<f32>(0.95), text);
    return vec4<f32>(color, 1.0);
}
"#;

    /// Content for the native surface slot, rendered by a wgpu device that
    /// GPUI does not own, standing in for an embedded browser or video player.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    struct NativeSurfaceRenderer {
        // Declared first so the wgpu surface is released before the native
        // surface it presents to.
        surface: wgpu::Surface<'static>,
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
        pipeline: wgpu::RenderPipeline,
        globals: wgpu::Buffer,
        bind_group: wgpu::BindGroup,
        started: Instant,
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    #[derive(Debug)]
    struct DisplayHandle(RawDisplayHandle);

    // SAFETY: the handle refers to the GPUI window's display connection, which
    // outlives the example's wgpu instance.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    unsafe impl Send for DisplayHandle {}
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    unsafe impl Sync for DisplayHandle {}

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    impl HasDisplayHandle for DisplayHandle {
        fn display_handle(
            &self,
        ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
            // SAFETY: see the `Send` implementation above.
            Ok(unsafe { raw_window_handle::DisplayHandle::borrow_raw(self.0) })
        }
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    impl NativeSurfaceRenderer {
        fn new(display: RawDisplayHandle, window: RawWindowHandle) -> anyhow::Result<Self> {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::VULKAN | wgpu::Backends::GL,
                flags: wgpu::InstanceFlags::default(),
                backend_options: wgpu::BackendOptions::default(),
                memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
                display: Some(Box::new(DisplayHandle(display))),
            });
            // SAFETY: the native surface outlives this renderer, which is
            // dropped before the composition surface in `NativeWebView`.
            let surface = unsafe {
                instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: Some(display),
                    raw_window_handle: window,
                })?
            };
            let adapter = gpui::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))?;
            let (device, queue) =
                gpui::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))?;
            let capabilities = surface.get_capabilities(&adapter);
            let format = capabilities
                .formats
                .iter()
                .copied()
                .find(|format| !format.is_srgb())
                .or_else(|| capabilities.formats.first().copied())
                .ok_or_else(|| anyhow::anyhow!("native surface has no supported formats"))?;
            // Presenting must not wait for the compositor, since it runs on
            // GPUI's foreground thread.
            let present_mode = [wgpu::PresentMode::Mailbox, wgpu::PresentMode::Immediate]
                .into_iter()
                .find(|mode| capabilities.present_modes.contains(mode))
                .unwrap_or(wgpu::PresentMode::Fifo);
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width: 1,
                height: 1,
                present_mode,
                desired_maximum_frame_latency: 2,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: Vec::new(),
            };

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("native_surface_shader"),
                source: wgpu::ShaderSource::Wgsl(NATIVE_SHADER.into()),
            });
            let globals = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("native_surface_globals"),
                size: 16,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("native_surface_globals"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("native_surface_globals"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: globals.as_entire_binding(),
                }],
            });
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("native_surface"),
                bind_group_layouts: &[Some(&bind_group_layout)],
                immediate_size: 0,
            });
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("native_surface"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(format.into())],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            });

            Ok(Self {
                surface,
                device,
                queue,
                config,
                pipeline,
                globals,
                bind_group,
                started: Instant::now(),
            })
        }

        fn render(&mut self, size: gpui::Size<gpui::DevicePixels>) {
            let width = size.width.0.max(1) as u32;
            let height = size.height.0.max(1) as u32;
            if (width, height) != (self.config.width, self.config.height) {
                self.config.width = width;
                self.config.height = height;
                self.surface.configure(&self.device, &self.config);
            }
            let frame = match self.surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(frame)
                | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
                wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                    self.surface.configure(&self.device, &self.config);
                    return;
                }
                _ => return,
            };
            let mut globals = [0u8; 16];
            for (index, value) in [
                width as f32,
                height as f32,
                self.started.elapsed().as_secs_f32(),
                0.,
            ]
            .into_iter()
            .enumerate()
            {
                globals[index * 4..index * 4 + 4].copy_from_slice(&value.to_ne_bytes());
            }
            self.queue.write_buffer(&self.globals, 0, &globals);
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("native_surface"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &self.bind_group, &[]);
                pass.draw(0..3, 0..1);
            }
            self.queue.submit([encoder.finish()]);
            frame.present();
        }
    }

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
        // Dropped before `surface`, whose native surface it presents to.
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        renderer: RefCell<NativeSurfaceRenderer>,
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

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        fn new(window: &Window, composition: &WindowComposition<'_>) -> anyhow::Result<Self> {
            let display = HasDisplayHandle::display_handle(window)
                .map_err(|error| anyhow::anyhow!("failed to get display handle: {error:?}"))?
                .as_raw();
            let surface = composition.create_native_surface()?;
            let native_window = surface
                .platform_surface()?
                .platform_handle()?
                .downcast::<RawWindowHandle>()
                .map_err(|_| anyhow::anyhow!("native surface did not provide a window handle"))?;
            let renderer = NativeSurfaceRenderer::new(display, *native_window)?;
            Ok(Self {
                renderer: RefCell::new(renderer),
                surface,
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

        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        fn focus_parent(&self) {}

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

    #[cfg(any(target_os = "macos", target_os = "windows"))]
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
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            {
                self.webview
                    .renderer
                    .borrow_mut()
                    .render(bounds.to_device_pixels(window.scale_factor()).size);
                window.request_animation_frame();
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
                                        .child(copy::TITLE),
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
                                .child(layer_row("02", NATIVE_LAYER_LABEL, rgb(0x5ac1fe)))
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
                                        .child(div().mt_1().text_lg().child(copy::HEADING)),
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
                                            tab(
                                                "webview-tab",
                                                copy::NATIVE_TAB,
                                                !self.about_active,
                                            )
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
                                                .child(menu_item("popup-menu-reload", copy::RELOAD))
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
                                                        .child(copy::ABOUT),
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
                                .child(
                                    div()
                                        .mt_2()
                                        .text_sm()
                                        .text_color(rgb(0x8a8986))
                                        .child(copy::POPOVER),
                                ),
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
                                        .child(
                                            div()
                                                .mt_3()
                                                .text_color(rgb(0x8a8986))
                                                .child(copy::DIALOG),
                                        )
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

#[cfg(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_os = "freebsd"
))]
fn main() {
    platform::run();
}
