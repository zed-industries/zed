#![cfg_attr(target_family = "wasm", no_main)]
//! Demonstrates the `gpui_wgpu` external compositor path: a backend-specific
//! renderer (here, a trivial procedural-texture generator standing in for e.g. a
//! wgpu-based 3D renderer) that GPUI composites into a rectangular region of its own
//! scene, at a controlled point in its own frame (see
//! `crates/gpui/src/external_compositor.rs` and
//! `crates/gpui_wgpu/src/wgpu_renderer.rs`).
//!
//! Not available on wasm: the canvas backend (`WgpuRenderer::new_from_canvas`) never
//! wires up an `ExternalCompositorRegistry`, so `Window::external_compositor_registry`
//! always returns `None` there; this example just shows a "no registry" status label
//! in that case instead of erroring out (the color set via
//! `ExternalCompositorElement::background` covers app-visible degradation on
//! backends without support, e.g. macOS/Metal in this phase — see that element's
//! docs).
//!
//! Run with `cargo run -p gpui --example external_compositor`. Set
//! `EXTERNAL_COMPOSITOR_EXIT_AFTER=<n>` (a frame count) to have the example close
//! itself and exit with status `0` after `n` rendered frames — used to smoke-test it
//! headlessly (e.g. in CI), without a human watching the window.

use gpui::{
    AlphaMode, App, Bounds, Context, ExternalSlotDescriptor, ExternalSlotFormat,
    ExternalSlotHandle, Render, SharedString, Window, WindowBounds, WindowOptions, div,
    external_compositor, prelude::*, px, rgb, rgba, size,
};
use gpui_platform::application;
#[cfg(not(target_family = "wasm"))]
use gpui_wgpu::{
    ExternalComposeOutput, WgpuCompositorBackendCtx, WgpuExternalCompositor,
    register_external_compositor, wgpu,
};
use std::sync::Arc;

const TEXTURE_SIZE: u32 = 256;

/// Reads `EXTERNAL_COMPOSITOR_EXIT_AFTER` once at startup.
fn exit_after_frames() -> Option<u64> {
    std::env::var("EXTERNAL_COMPOSITOR_EXIT_AFTER")
        .ok()
        .and_then(|value| value.parse().ok())
}

/// A minimal [`WgpuExternalCompositor`]: on its first `compose` call it creates a
/// `TEXTURE_SIZE`x`TEXTURE_SIZE` `Rgba8UnormSrgb` texture on the frame's `wgpu`
/// device; every `compose` call after that (including the first) it rewrites the
/// texture with a procedural gradient plus a diagonal stripe animated by
/// `ctx.frame_index`, proving the texture is actually updated live, frame over
/// frame, rather than composited once and left static.
#[cfg(not(target_family = "wasm"))]
struct DemoCompositor {
    texture: Option<(wgpu::Texture, Arc<wgpu::TextureView>)>,
}

#[cfg(not(target_family = "wasm"))]
impl DemoCompositor {
    fn new() -> Self {
        Self { texture: None }
    }

    /// A `TEXTURE_SIZE`x`TEXTURE_SIZE` RGBA8 gradient (red = x, green = y) with a
    /// diagonal stripe that sweeps across the texture as `frame_index` advances.
    fn pixels(frame_index: u64) -> Vec<u8> {
        let stripe = (frame_index % TEXTURE_SIZE as u64) as u32;
        let mut data = vec![0u8; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];
        for y in 0..TEXTURE_SIZE {
            for x in 0..TEXTURE_SIZE {
                let i = ((y * TEXTURE_SIZE + x) * 4) as usize;
                let on_stripe = (x + y) % TEXTURE_SIZE == stripe;
                data[i] = (x * 255 / (TEXTURE_SIZE - 1)) as u8;
                data[i + 1] = (y * 255 / (TEXTURE_SIZE - 1)) as u8;
                data[i + 2] = if on_stripe { 255 } else { 96 };
                data[i + 3] = 255;
            }
        }
        data
    }
}

#[cfg(not(target_family = "wasm"))]
impl WgpuExternalCompositor for DemoCompositor {
    fn compose(
        &mut self,
        _slot: ExternalSlotHandle,
        ctx: &mut WgpuCompositorBackendCtx<'_>,
    ) -> ExternalComposeOutput {
        let (texture, view) = self.texture.get_or_insert_with(|| {
            log::info!(
                "external_compositor example: creating {TEXTURE_SIZE}x{TEXTURE_SIZE} demo \
                 texture (swapchain target format {:?}, context generation {})",
                ctx.target_format,
                ctx.context_generation,
            );
            let texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("external_compositor_example_demo_texture"),
                size: wgpu::Extent3d {
                    width: TEXTURE_SIZE,
                    height: TEXTURE_SIZE,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let view = Arc::new(texture.create_view(&wgpu::TextureViewDescriptor::default()));
            (texture, view)
        });

        let pixels = Self::pixels(ctx.frame_index);
        ctx.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(TEXTURE_SIZE * 4),
                rows_per_image: Some(TEXTURE_SIZE),
            },
            wgpu::Extent3d {
                width: TEXTURE_SIZE,
                height: TEXTURE_SIZE,
                depth_or_array_layers: 1,
            },
        );

        ExternalComposeOutput::Ready { view: view.clone() }
    }

    fn on_context_recreated(&mut self, new_generation: u64) {
        // The device this texture lived on is gone: drop it. The example's
        // per-frame poll (see `ExternalCompositorDemo::render`) will notice the
        // handle went stale via `ExternalCompositorRegistry::is_valid`, clean it up,
        // and register a fresh `DemoCompositor` under `new_generation`; that fresh
        // instance will lazily recreate the texture on its own first `compose`.
        log::warn!(
            "external_compositor example: graphics context recreated (generation \
             {new_generation}); dropping demo texture"
        );
        self.texture = None;
    }
}

struct ExternalCompositorDemo {
    handle: Option<ExternalSlotHandle>,
    frames_rendered: u64,
    exit_after: Option<u64>,
    status: SharedString,
}

impl ExternalCompositorDemo {
    fn new() -> Self {
        Self {
            handle: None,
            frames_rendered: 0,
            exit_after: exit_after_frames(),
            status: "waiting for external compositor registry".into(),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn ensure_registered(&mut self, window: &mut Window) {
        let Some(registry) = window.external_compositor_registry() else {
            self.status = "this platform backend has no external compositor registry".into();
            return;
        };

        let needs_register = match self.handle {
            Some(handle) => !registry.borrow().is_valid(handle),
            None => true,
        };
        if !needs_register {
            return;
        }

        if let Some(old_handle) = self.handle.take() {
            // The previous handle went stale (a graphics context recreation
            // happened): this is `unregister`'s context-recreation cleanup case —
            // it frees the slot immediately, no frame-in-flight deferral, since a
            // stale slot is never composed.
            if let Err(error) = registry.borrow_mut().unregister(old_handle) {
                log::debug!("external_compositor example: stale slot cleanup: {error}");
            }
        }

        let generation = registry.borrow().current_context_generation();
        if let Some(specs) = window.gpu_specs() {
            log::info!(
                "external_compositor example: gpu = {} ({}), software_emulated = {}",
                specs.device_name,
                specs.driver_name,
                specs.is_software_emulated,
            );
        }
        log::info!(
            "external_compositor example: registering demo compositor under context \
             generation {generation}"
        );

        let descriptor = ExternalSlotDescriptor {
            format: ExternalSlotFormat::Rgba8UnormSrgb,
            alpha_mode: AlphaMode::Straight,
            width: TEXTURE_SIZE,
            height: TEXTURE_SIZE,
            sample_count: 1,
            context_generation: generation,
        };
        match register_external_compositor(&registry, descriptor, DemoCompositor::new()) {
            Ok(handle) => {
                self.handle = Some(handle);
                self.status = "compositor registered".into();
            }
            Err(error) => {
                self.status = format!("registration failed: {error}").into();
                log::error!("external_compositor example: {error}");
            }
        }
    }

    #[cfg(target_family = "wasm")]
    fn ensure_registered(&mut self, _window: &mut Window) {
        self.status = "external composition is not available on wasm".into();
    }
}

impl Render for ExternalCompositorDemo {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.frames_rendered += 1;
        self.ensure_registered(window);

        if let Some(exit_after) = self.exit_after
            && self.frames_rendered >= exit_after
        {
            log::info!(
                "external_compositor example: exiting after {} frames \
                 (EXTERNAL_COMPOSITOR_EXIT_AFTER={exit_after})",
                self.frames_rendered
            );
            std::process::exit(0);
        }

        // Keep redrawing every frame: the demo compositor's texture animates by
        // `WgpuCompositorBackendCtx::frame_index`, and this is also how the example
        // keeps polling `ExternalCompositorRegistry::is_valid` to notice context
        // recreation promptly.
        window.request_animation_frame();

        let content = if let Some(handle) = self.handle {
            div()
                .w(px(TEXTURE_SIZE as f32))
                .h(px(TEXTURE_SIZE as f32))
                .border_2()
                .border_color(gpui::white())
                .child(external_compositor(handle).background(rgba(0x1a1a1aff)))
        } else {
            div()
                .w(px(TEXTURE_SIZE as f32))
                .h(px(TEXTURE_SIZE as f32))
                .border_2()
                .border_color(gpui::white())
                .bg(gpui::black())
        };

        div()
            .flex()
            .flex_col()
            .gap_3()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .p_4()
            .text_color(gpui::white())
            .child(format!(
                "external compositor demo — frame {} — {}",
                self.frames_rendered, self.status
            ))
            .child(div().flex().justify_center().items_center().child(content))
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(420.0), px(420.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ExternalCompositorDemo::new()),
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("gpui", log::LevelFilter::Info)
        .filter_module("gpui_wgpu", log::LevelFilter::Info)
        .filter_module("external_compositor", log::LevelFilter::Info)
        .init();
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
