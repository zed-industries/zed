//! Host side of the "GPUI embedded in GPUI" spike. See `DESIGN.md` for the architecture and
//! `wit/plugin.wit` for the wire protocol. This crate compiles a `wasm32-wasip2` guest
//! component that renders a GPUI UI, and replays its retained display lists inside a native
//! GPUI application.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result};
use gpui::{AppContext as _, Context, Entity, Pixels, PlatformTextSystem, Size, Task, WeakEntity, px};
use gpui_embedded_shared::{
    AckSender, HandlerResponse, ResponseSender, SharedMessage, SharedProjection, SharedSpec,
};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

pub(crate) mod bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "plugin",
    });
}

use bindings::{Plugin, PluginImports};

mod plugin_element;
mod shared_entities;

pub use gpui_embedded_shared::{
    CallReceipt, HandleShared, HandleSharedAsync, Methods, RawCallReceipt, SendReceipt,
    SharedEntitySource, SharedRef,
};
pub use plugin_element::PluginViewState;

/// A typed host-side handle to a guest-homed shared entity.
pub struct HostRemote<S: SharedSpec> {
    name: String,
    replica: Entity<SharedProjection<S::Snapshot>>,
    host: WeakEntity<PluginHost>,
    /// Present only for remotes materialized from a [`SharedRef`]; named remotes are
    /// bootstrap mounts and are never auto-released.
    _guard: Option<Rc<HostReleaseGuard>>,
}

impl<S: SharedSpec> Clone for HostRemote<S> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            replica: self.replica.clone(),
            host: self.host.clone(),
            _guard: self._guard.clone(),
        }
    }
}

/// Dropping the last `HostRemote` for a ref-derived projection queues a release; the
/// host can't call into the guest from `Drop` (no context, and the instance may be
/// mid-call), so the queue is drained on the next `apply_effects` or [`PluginHost::pump`].
struct HostReleaseGuard {
    name: String,
    queue: Rc<RefCell<Vec<String>>>,
}

impl Drop for HostReleaseGuard {
    fn drop(&mut self) {
        self.queue.borrow_mut().push(self.name.clone());
    }
}

impl<S: SharedSpec> HostRemote<S> {
    /// The local replica entity, for `cx.observe` and reads.
    pub fn replica(&self) -> &Entity<SharedProjection<S::Snapshot>> {
        &self.replica
    }

    /// Send a typed message to the guest home. Await the receipt for read-your-writes.
    pub fn send<M: SharedMessage<Spec = S>>(&self, message: M, cx: &mut gpui::App) -> SendReceipt {
        let (ack_sender, receipt) = SendReceipt::channel();
        match gpui_embedded_shared::encode(&message) {
            Ok(payload) => {
                self.host
                    .update(cx, |host, cx| {
                        host.send_to_guest(
                            &self.name,
                            M::METHOD,
                            payload,
                            Some(ack_sender),
                            None,
                            cx,
                        );
                    })
                    .ok();
            }
            Err(error) => log::error!(
                "gpui_embedded: failed to encode {}::{}: {error:#}",
                S::TYPE_NAME,
                M::METHOD
            ),
        }
        receipt
    }

    /// Derive a weaker capability to the same entity, keeping only the listed methods
    /// (intersected with this ref's own table — attenuation is monotonic).
    pub fn attenuate(&self, keep: &[&str], cx: &mut gpui::App) -> CallReceipt<SharedRef<S>> {
        match gpui_embedded_shared::encode(&keep) {
            Ok(payload) => {
                self.call_raw(gpui_embedded_shared::ATTENUATE_METHOD, payload, cx)
            }
            Err(error) => {
                log::error!("gpui_embedded: failed to encode attenuation: {error:#}");
                CallReceipt::dropped()
            }
        }
    }

    /// The dynamic escape hatch: send an arbitrary method and payload.
    pub fn send_raw(&self, method: &str, payload: Vec<u8>, cx: &mut gpui::App) -> SendReceipt {
        let (ack_sender, receipt) = SendReceipt::channel();
        self.host
            .update(cx, |host, cx| {
                host.send_to_guest(&self.name, method, payload, Some(ack_sender), None, cx);
            })
            .ok();
        receipt
    }

    /// The dynamic call escape hatch: call an arbitrary method and decode the response.
    pub fn call_raw<R: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        payload: Vec<u8>,
        cx: &mut gpui::App,
    ) -> CallReceipt<R> {
        let (response_sender, receipt) = CallReceipt::channel();
        self.host
            .update(cx, |host, cx| {
                host.send_to_guest(&self.name, method, payload, None, Some(response_sender), cx);
            })
            .ok();
        receipt
    }

    /// Call a method by name with pre-encoded payload, resolving with the raw response
    /// bytes. This is the forwarding primitive: a caretaker pipes a caller's request
    /// through to the entity it guards without knowing the method's types.
    pub fn forward(&self, method: &str, payload: Vec<u8>, cx: &mut gpui::App) -> RawCallReceipt {
        let (response_sender, receipt) = RawCallReceipt::channel();
        self.host
            .update(cx, |host, cx| {
                host.send_to_guest(&self.name, method, payload, None, Some(response_sender), cx);
            })
            .ok();
        receipt
    }

    /// Call a typed method on the guest home, resolving with its return value after the
    /// replica reflects the mutation.
    pub fn call<M: SharedMessage<Spec = S>>(
        &self,
        message: M,
        cx: &mut gpui::App,
    ) -> CallReceipt<M::Response> {
        let (response_sender, receipt) = CallReceipt::channel();
        match gpui_embedded_shared::encode(&message) {
            Ok(payload) => {
                self.host
                    .update(cx, |host, cx| {
                        host.send_to_guest(
                            &self.name,
                            M::METHOD,
                            payload,
                            None,
                            Some(response_sender),
                            cx,
                        );
                    })
                    .ok();
            }
            Err(error) => log::error!(
                "gpui_embedded: failed to encode {}::{}: {error:#}",
                S::TYPE_NAME,
                M::METHOD
            ),
        }
        receipt
    }
}

/// Effects drained from the guest after each call into it. The host acts on these once the
/// guest call has returned, never re-entering wasm from within a host import (see DESIGN.md
/// invariant 3).
#[derive(Default)]
pub struct PendingEffects {
    pub scene_updates: Vec<(u32, bindings::DisplayList)>,
    pub tick_delay_ms: Option<u32>,
    pub cursor_style: Option<gpui::CursorStyle>,
    pub shared_messages: Vec<bindings::SharedMessage>,
    pub shared_announcements: Vec<bindings::SharedEntityAnnouncement>,
    pub shared_snapshots: Vec<bindings::SharedSnapshot>,
    pub shared_responses: Vec<bindings::SharedResponse>,
}

/// Alias used for the value returned from the `PluginInstance` methods after they drain the
/// pending effects.
pub type Effects = PendingEffects;

/// The data carried on the wasmtime `Store`. Host imports only mutate `pending`; the host
/// drains it after each guest call returns.
struct HostState {
    wasi: WasiCtx,
    table: ResourceTable,
    text_system: Arc<dyn PlatformTextSystem>,
    pending: PendingEffects,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

impl wasmtime::component::HasData for HostState {
    type Data<'a> = &'a mut HostState;
}

impl HostState {
    fn font(&self, family: impl Into<gpui::SharedString>, weight: f32, italic: bool) -> gpui::Font {
        gpui::Font {
            family: family.into(),
            features: gpui::FontFeatures::default(),
            fallbacks: None,
            weight: gpui::FontWeight(weight),
            style: if italic {
                gpui::FontStyle::Italic
            } else {
                gpui::FontStyle::Normal
            },
        }
    }
}

impl PluginImports for HostState {
    fn resolve_font(&mut self, font: bindings::FontDescriptor) -> u32 {
        let requested = self.font(font.family.clone(), font.weight, font.italic);
        match self.text_system.font_id(&requested) {
            Ok(id) => return id.0 as u32,
            Err(error) => {
                log::warn!(
                    "gpui_embedded: failed to resolve font {:?}: {error:#}; falling back",
                    font.family
                );
            }
        }

        for fallback in [".SystemUIFont", "Helvetica"] {
            let candidate = self.font(fallback, font.weight, font.italic);
            match self.text_system.font_id(&candidate) {
                Ok(id) => return id.0 as u32,
                Err(error) => {
                    log::warn!("gpui_embedded: fallback font {fallback:?} unavailable: {error:#}");
                }
            }
        }

        log::error!("gpui_embedded: no fallback font available; using font id 0");
        0
    }

    fn font_metrics_for(&mut self, font_id: u32) -> bindings::FontMetrics {
        let metrics = self.text_system.font_metrics(gpui::FontId(font_id as usize));
        bindings::FontMetrics {
            units_per_em: metrics.units_per_em,
            ascent: metrics.ascent,
            descent: metrics.descent,
            line_gap: metrics.line_gap,
            underline_position: metrics.underline_position,
            underline_thickness: metrics.underline_thickness,
            cap_height: metrics.cap_height,
            x_height: metrics.x_height,
            bounding_box: bounds_from_f32(metrics.bounding_box),
        }
    }

    fn layout_line(
        &mut self,
        text: String,
        font_size: f32,
        runs: Vec<bindings::FontRun>,
    ) -> bindings::LineLayout {
        let runs: Vec<gpui::FontRun> = runs
            .into_iter()
            .map(|run| gpui::FontRun {
                len: run.len as usize,
                font_id: gpui::FontId(run.font_id as usize),
            })
            .collect();
        let layout = self.text_system.layout_line(&text, px(font_size), &runs);
        convert_line_layout(&layout)
    }

    fn advance(&mut self, font_id: u32, glyph_id: u32) -> bindings::Extent {
        match self
            .text_system
            .advance(gpui::FontId(font_id as usize), gpui::GlyphId(glyph_id))
        {
            Ok(advance) => bindings::Extent {
                width: advance.width,
                height: advance.height,
            },
            Err(error) => {
                log::warn!("gpui_embedded: advance failed for glyph {glyph_id}: {error:#}");
                bindings::Extent {
                    width: 0.,
                    height: 0.,
                }
            }
        }
    }

    fn typographic_bounds(&mut self, font_id: u32, glyph_id: u32) -> bindings::Bounds {
        match self
            .text_system
            .typographic_bounds(gpui::FontId(font_id as usize), gpui::GlyphId(glyph_id))
        {
            Ok(bounds) => bounds_from_f32(bounds),
            Err(error) => {
                log::warn!(
                    "gpui_embedded: typographic_bounds failed for glyph {glyph_id}: {error:#}"
                );
                bounds_from_f32(gpui::Bounds::default())
            }
        }
    }

    fn glyph_for_char(&mut self, font_id: u32, ch: char) -> Option<u32> {
        self.text_system
            .glyph_for_char(gpui::FontId(font_id as usize), ch)
            .map(|glyph| glyph.0)
    }

    fn glyph_raster_bounds(&mut self, params: bindings::GlyphParams) -> bindings::DeviceBounds {
        let request = gpui::RenderGlyphParams {
            font_id: gpui::FontId(params.font_id as usize),
            glyph_id: gpui::GlyphId(params.glyph_id),
            font_size: px(params.font_size),
            subpixel_variant: gpui::Point {
                x: params.subpixel_variant_x,
                y: params.subpixel_variant_y,
            },
            scale_factor: params.scale_factor,
            is_emoji: params.is_emoji,
            subpixel_rendering: false,
            dilation: 0,
        };
        match self.text_system.glyph_raster_bounds(&request) {
            Ok(bounds) => bindings::DeviceBounds {
                origin_x: bounds.origin.x.0,
                origin_y: bounds.origin.y.0,
                width: bounds.size.width.0,
                height: bounds.size.height.0,
            },
            Err(error) => {
                log::warn!(
                    "gpui_embedded: glyph_raster_bounds failed for glyph {}: {error:#}",
                    params.glyph_id
                );
                bindings::DeviceBounds {
                    origin_x: 0,
                    origin_y: 0,
                    width: 0,
                    height: 0,
                }
            }
        }
    }

    fn request_tick(&mut self, delay_ms: u32) {
        self.pending.tick_delay_ms = Some(match self.pending.tick_delay_ms {
            Some(existing) => existing.min(delay_ms),
            None => delay_ms,
        });
    }

    fn update_scene(&mut self, view_id: u32, list: bindings::DisplayList) {
        self.pending.scene_updates.push((view_id, list));
    }

    fn send_shared_message(&mut self, message: bindings::SharedMessage) {
        self.pending.shared_messages.push(message);
    }

    fn announce_shared_entity(&mut self, announcement: bindings::SharedEntityAnnouncement) {
        self.pending.shared_announcements.push(announcement);
    }

    fn publish_shared_snapshot(&mut self, snapshot: bindings::SharedSnapshot) {
        self.pending.shared_snapshots.push(snapshot);
    }

    fn send_shared_response(&mut self, response: bindings::SharedResponse) {
        self.pending.shared_responses.push(response);
    }

    fn set_cursor_style(&mut self, style: bindings::CursorStyle) {
        self.pending.cursor_style = Some(cursor_style_from_wire(style));
    }
}

fn bounds_from_f32(bounds: gpui::Bounds<f32>) -> bindings::Bounds {
    bindings::Bounds {
        origin: bindings::Point {
            x: bounds.origin.x,
            y: bounds.origin.y,
        },
        size: bindings::Extent {
            width: bounds.size.width,
            height: bounds.size.height,
        },
    }
}

fn convert_line_layout(layout: &gpui::LineLayout) -> bindings::LineLayout {
    bindings::LineLayout {
        font_size: f32::from(layout.font_size),
        width: f32::from(layout.width),
        ascent: f32::from(layout.ascent),
        descent: f32::from(layout.descent),
        len: layout.len as u32,
        runs: layout
            .runs
            .iter()
            .map(|run| bindings::ShapedRun {
                font_id: run.font_id.0 as u32,
                glyphs: run
                    .glyphs
                    .iter()
                    .map(|glyph| bindings::ShapedGlyph {
                        id: glyph.id.0,
                        position: bindings::Point {
                            x: f32::from(glyph.position.x),
                            y: f32::from(glyph.position.y),
                        },
                        index: glyph.index as u32,
                        is_emoji: glyph.is_emoji,
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn cursor_style_from_wire(style: bindings::CursorStyle) -> gpui::CursorStyle {
    match style {
        bindings::CursorStyle::Arrow => gpui::CursorStyle::Arrow,
        bindings::CursorStyle::Ibeam => gpui::CursorStyle::IBeam,
        bindings::CursorStyle::Crosshair => gpui::CursorStyle::Crosshair,
        bindings::CursorStyle::ClosedHand => gpui::CursorStyle::ClosedHand,
        bindings::CursorStyle::OpenHand => gpui::CursorStyle::OpenHand,
        bindings::CursorStyle::PointingHand => gpui::CursorStyle::PointingHand,
        bindings::CursorStyle::ResizeLeftRight => gpui::CursorStyle::ResizeLeftRight,
        bindings::CursorStyle::ResizeUpDown => gpui::CursorStyle::ResizeUpDown,
        bindings::CursorStyle::OperationNotAllowed => gpui::CursorStyle::OperationNotAllowed,
    }
}

/// A synchronous wasmtime store plus its instantiated bindings. Each method calls a guest
/// export and then drains and returns the effects the guest queued during that call.
pub struct PluginInstance {
    store: Store<HostState>,
    bindings: Plugin,
}

impl PluginInstance {
    pub fn new(component_path: &Path, text_system: Arc<dyn PlatformTextSystem>) -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config).context("creating wasmtime engine")?;

        let component = Component::from_file(&engine, component_path)
            .with_context(|| format!("loading component {}", component_path.display()))?;

        let mut linker = Linker::new(&engine);
        wasmtime_wasi::p2::add_to_linker_sync(&mut linker).context("adding wasi to linker")?;
        Plugin::add_to_linker::<_, HostState>(&mut linker, |state| state)
            .context("adding plugin host imports to linker")?;

        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build();
        let state = HostState {
            wasi,
            table: ResourceTable::new(),
            text_system,
            pending: PendingEffects::default(),
        };
        let mut store = Store::new(&engine, state);
        let bindings = Plugin::instantiate(&mut store, &component, &linker)
            .context("instantiating plugin component")?;

        Ok(Self { store, bindings })
    }

    fn take_effects(&mut self) -> Effects {
        std::mem::take(&mut self.store.data_mut().pending)
    }

    pub fn init(&mut self) -> Result<Effects> {
        self.bindings.call_init_plugin(&mut self.store)?;
        Ok(self.take_effects())
    }

    pub fn create_view(&mut self, view_id: u32, size: Size<Pixels>, scale: f32) -> Result<Effects> {
        let extent = extent_from_size(size);
        self.bindings
            .call_create_view(&mut self.store, view_id, extent, scale)?;
        Ok(self.take_effects())
    }

    pub fn resize_view(&mut self, view_id: u32, size: Size<Pixels>, scale: f32) -> Result<Effects> {
        let extent = extent_from_size(size);
        self.bindings
            .call_resize_view(&mut self.store, view_id, extent, scale)?;
        Ok(self.take_effects())
    }

    pub fn handle_mouse(&mut self, view_id: u32, event: bindings::MouseEvent) -> Result<Effects> {
        self.bindings
            .call_handle_mouse(&mut self.store, view_id, event)?;
        Ok(self.take_effects())
    }

    pub fn handle_key(&mut self, view_id: u32, event: bindings::KeyEvent) -> Result<Effects> {
        self.bindings
            .call_handle_key(&mut self.store, view_id, &event)?;
        Ok(self.take_effects())
    }

    pub fn tick(&mut self) -> Result<Effects> {
        self.bindings.call_tick(&mut self.store)?;
        Ok(self.take_effects())
    }

    pub fn announce_shared_entity(
        &mut self,
        announcement: &bindings::SharedEntityAnnouncement,
    ) -> Result<Effects> {
        self.bindings
            .call_shared_entity_announced(&mut self.store, announcement)?;
        Ok(self.take_effects())
    }

    pub fn deliver_shared_snapshot(
        &mut self,
        snapshot: &bindings::SharedSnapshot,
    ) -> Result<Effects> {
        self.bindings
            .call_deliver_shared_snapshot(&mut self.store, snapshot)?;
        Ok(self.take_effects())
    }

    pub fn deliver_shared_message(&mut self, message: &bindings::SharedMessage) -> Result<Effects> {
        self.bindings
            .call_deliver_shared_message(&mut self.store, message)?;
        Ok(self.take_effects())
    }

    pub fn deliver_shared_response(
        &mut self,
        response: &bindings::SharedResponse,
    ) -> Result<Effects> {
        self.bindings
            .call_deliver_shared_response(&mut self.store, response)?;
        Ok(self.take_effects())
    }
}

fn extent_from_size(size: Size<Pixels>) -> bindings::Extent {
    bindings::Extent {
        width: f32::from(size.width),
        height: f32::from(size.height),
    }
}

/// A GPUI entity that owns the wasmtime store and mediates between the host application and
/// the guest. All calls into the guest happen from here, on the foreground thread.
/// Images shipped by the guest, cached per instance and shared by all of its views.
pub type PluginImages = Rc<RefCell<HashMap<u64, Arc<gpui::RenderImage>>>>;

pub struct PluginHost {
    instance: Rc<RefCell<PluginInstance>>,
    views: HashMap<u32, Entity<PluginViewState>>,
    images: PluginImages,
    shared: shared_entities::HostShared,
    remotes_by_name: HashMap<String, gpui::AnyEntity>,
    /// Names whose `HostReleaseGuard` dropped; drained into `$release` sends.
    pending_releases: Rc<RefCell<Vec<String>>>,
    release_guards: HashMap<String, std::rc::Weak<HostReleaseGuard>>,
    scheduled_tick: Option<Task<()>>,
}

impl PluginHost {
    pub fn new(instance: PluginInstance) -> Self {
        Self {
            instance: Rc::new(RefCell::new(instance)),
            views: HashMap::new(),
            images: PluginImages::default(),
            shared: shared_entities::HostShared::default(),
            remotes_by_name: HashMap::new(),
            pending_releases: Rc::default(),
            release_guards: HashMap::new(),
            scheduled_tick: None,
        }
    }

    /// Share a host entity with the guest under a well-known name. The entity becomes the
    /// *home* of the shared state: guest messages dispatch to the handlers registered in
    /// `register`, and every `cx.notify` on it publishes a fresh snapshot to guest
    /// projections. Call after [`PluginHost::init`].
    pub fn share<S, T>(
        &mut self,
        entity: &Entity<T>,
        name: impl Into<String>,
        register: impl FnOnce(&mut Methods<S, T>),
        cx: &mut Context<Self>,
    ) where
        S: SharedSpec,
        T: SharedEntitySource<S>,
    {
        let name = name.into();
        let mut methods = Methods::new(entity.downgrade());
        register(&mut methods);

        let snapshot_fn: Rc<dyn Fn(&gpui::App) -> Result<Vec<u8>>> = {
            let entity = entity.downgrade();
            Rc::new(move |cx| {
                let entity = entity.upgrade().context("shared entity dropped")?;
                gpui_embedded_shared::encode(&entity.read(cx).snapshot(cx))
            })
        };

        let entity_id = self.shared.insert_placeholder();
        let observation = cx.observe(entity, move |host, _, cx| {
            host.publish_home(entity_id, cx);
        });
        self.shared.fill_placeholder(
            entity_id,
            shared_entities::HostSharedEntity::new(
                name.clone(),
                S::TYPE_NAME,
                methods,
                snapshot_fn,
                true,
                None,
                observation,
            ),
        );

        let announcement = bindings::SharedEntityAnnouncement {
            entity_id,
            type_name: S::TYPE_NAME.to_string(),
            name,
        };
        let result = self
            .instance
            .borrow_mut()
            .announce_shared_entity(&announcement);
        match result {
            Ok(effects) => self.apply_effects(effects, cx),
            Err(error) => log::error!(
                "gpui_embedded: announcing shared entity {:?} failed: {error:#}",
                announcement.name
            ),
        }
        self.publish_home(entity_id, cx);
    }

    /// Attach to a guest-homed shared entity by name. The replica fills in when the guest's
    /// announcement and first snapshot arrive; sends queue (in order) until then.
    pub fn remote<S: SharedSpec>(
        &mut self,
        name: impl Into<String>,
        cx: &mut Context<Self>,
    ) -> HostRemote<S> {
        let name = name.into();
        let replica = cx.new(|_| SharedProjection::<S::Snapshot> { state: None });
        let apply_snapshot: Rc<dyn Fn(&[u8], &mut gpui::App) -> Result<()>> = {
            let replica = replica.downgrade();
            Rc::new(move |bytes, cx| {
                let snapshot: S::Snapshot =
                    gpui_embedded_shared::decode(bytes).context("decoding shared snapshot")?;
                replica.update(cx, |projection, cx| {
                    projection.state = Some(snapshot);
                    cx.notify();
                })
            })
        };
        self.shared
            .insert_projection::<S>(name.clone(), apply_snapshot);
        if let Some(announcement) = self.shared.unclaimed_announcements.remove(&name) {
            self.bind_projection(announcement, cx);
        }
        HostRemote {
            name,
            replica,
            host: cx.weak_entity(),
            _guard: None,
        }
    }

    /// Share a host entity anonymously, returning a capability reference to embed in
    /// snapshot or message payloads. The home holds a strong handle to the entity until the
    /// reference is released; snapshots start flowing when a guest projection subscribes.
    pub fn share_anonymous<S, T>(
        &mut self,
        entity: &Entity<T>,
        register: impl FnOnce(&mut Methods<S, T>),
        cx: &mut Context<Self>,
    ) -> SharedRef<S>
    where
        S: SharedSpec,
        T: SharedEntitySource<S>,
    {
        let mut methods = Methods::new(entity.downgrade());
        register(&mut methods);

        let snapshot_fn: Rc<dyn Fn(&gpui::App) -> Result<Vec<u8>>> = {
            let entity = entity.downgrade();
            Rc::new(move |cx| {
                let entity = entity.upgrade().context("shared entity dropped")?;
                gpui_embedded_shared::encode(&entity.read(cx).snapshot(cx))
            })
        };

        let entity_id = self.shared.insert_placeholder();
        let observation = cx.observe(entity, move |host, _, cx| {
            host.publish_home(entity_id, cx);
        });
        self.shared.fill_placeholder(
            entity_id,
            shared_entities::HostSharedEntity::new(
                format!("#{entity_id}"),
                S::TYPE_NAME,
                methods,
                snapshot_fn,
                false,
                Some(entity.clone().into_any()),
                observation,
            ),
        );
        SharedRef::from_raw(entity_id)
    }

    /// Attach to a guest-homed shared entity through a capability reference received in a
    /// payload. Subscribes immediately; the subscribe's ack delivers the initial snapshot.
    pub fn remote_from_ref<S: SharedSpec>(
        &mut self,
        reference: SharedRef<S>,
        cx: &mut Context<Self>,
    ) -> HostRemote<S> {
        let entity_id = reference.entity_id();
        let name = format!("#{entity_id}");
        if let Some(guard) = self
            .release_guards
            .get(&name)
            .and_then(std::rc::Weak::upgrade)
        {
            let replica = self
                .remotes_by_name
                .get(&name)
                .cloned()
                .and_then(|any| any.downcast::<SharedProjection<S::Snapshot>>().ok());
            match replica {
                Some(replica) => {
                    return HostRemote {
                        name,
                        replica,
                        host: cx.weak_entity(),
                        _guard: Some(guard),
                    };
                }
                None => log::error!(
                    "gpui_embedded: ref {entity_id} materialized twice with different specs"
                ),
            }
        }
        // A dead guard whose release hasn't been drained yet means the projection state
        // is stale; releasing now and resubscribing below keeps the guest home consistent.
        if self.shared.projections_by_name.contains_key(&name) {
            self.pending_releases
                .borrow_mut()
                .retain(|pending| pending != &name);
            self.release_projection(&name, cx);
        }
        {
            let replica = cx.new(|_| SharedProjection::<S::Snapshot> { state: None });
            let apply_snapshot: Rc<dyn Fn(&[u8], &mut gpui::App) -> Result<()>> = {
                let replica = replica.downgrade();
                Rc::new(move |bytes, cx| {
                    let snapshot: S::Snapshot = gpui_embedded_shared::decode(bytes)
                        .context("decoding shared snapshot")?;
                    replica.update(cx, |projection, cx| {
                        projection.state = Some(snapshot);
                        cx.notify();
                    })
                })
            };
            self.shared
                .insert_projection_bound::<S>(name.clone(), apply_snapshot, entity_id);
            self.remotes_by_name
                .insert(name.clone(), replica.clone().into_any());
            self.send_to_guest(
                &name,
                gpui_embedded_shared::SUBSCRIBE_METHOD,
                Vec::new(),
                None,
                None,
                cx,
            );
            let guard = Rc::new(HostReleaseGuard {
                name: name.clone(),
                queue: self.pending_releases.clone(),
            });
            self.release_guards
                .insert(name.clone(), Rc::downgrade(&guard));
            HostRemote {
                name,
                replica,
                host: cx.weak_entity(),
                _guard: Some(guard),
            }
        }
    }

    /// Send `$release` for a ref-derived projection and forget it locally. The guest home
    /// drops its strong handle; snapshots stop flowing.
    fn release_projection(&mut self, name: &str, cx: &mut Context<Self>) {
        self.send_to_guest(
            name,
            gpui_embedded_shared::RELEASE_METHOD,
            Vec::new(),
            None,
            None,
            cx,
        );
        if let Some(projection) = self.shared.projections_by_name.remove(name)
            && let Some(entity_id) = projection.entity_id
        {
            self.shared.projection_names_by_id.remove(&entity_id);
        }
        self.remotes_by_name.remove(name);
        self.release_guards.remove(name);
    }

    fn drain_pending_releases(&mut self, cx: &mut Context<Self>) {
        loop {
            let names = std::mem::take(&mut *self.pending_releases.borrow_mut());
            if names.is_empty() {
                break;
            }
            for name in names {
                self.release_projection(&name, cx);
            }
        }
    }

    /// Flush deferred work (queued capability releases) and give the guest a turn. Hosts
    /// with quiescent plugins (no pending tick) can call this to make drops observable.
    pub fn pump(&mut self, cx: &mut Context<Self>) {
        self.drain_pending_releases(cx);
        self.tick(cx);
    }

    fn publish_home(&mut self, entity_id: u64, cx: &mut Context<Self>) {
        let Some(home) = self.shared.home_mut(entity_id) else {
            return;
        };
        let facets = home.facets.clone();
        if home.subscribed {
            home.published_ack = home.applied_sequence;
            let acked_sequence = home.applied_sequence;
            let snapshot_fn = home.snapshot_fn.clone();
            match snapshot_fn(cx) {
                Ok(payload) => {
                    let result = self.instance.borrow_mut().deliver_shared_snapshot(
                        &bindings::SharedSnapshot {
                            entity_id,
                            acked_sequence,
                            payload,
                        },
                    );
                    match result {
                        Ok(effects) => self.apply_effects(effects, cx),
                        Err(error) => log::error!(
                            "gpui_embedded: delivering shared snapshot failed: {error:#}"
                        ),
                    }
                }
                Err(error) => {
                    log::error!("gpui_embedded: failed to encode shared snapshot: {error:#}")
                }
            }
        }
        // Attenuated facets alias the same entity state, so a change fans out to all.
        for facet in facets {
            self.publish_home(facet, cx);
        }
    }

    fn bind_projection(
        &mut self,
        announcement: bindings::SharedEntityAnnouncement,
        cx: &mut Context<Self>,
    ) {
        if let Some(pending_sends) = self.shared.bind_projection(&announcement) {
            for send in pending_sends {
                self.deliver_message_to_guest(announcement.entity_id, send, cx);
            }
        }
    }

    fn deliver_message_to_guest(
        &mut self,
        entity_id: u64,
        send: shared_entities::PendingSend,
        cx: &mut Context<Self>,
    ) {
        let result = self
            .instance
            .borrow_mut()
            .deliver_shared_message(&bindings::SharedMessage {
                entity_id,
                sequence: send.sequence,
                request_id: send.request_id,
                method: send.method,
                payload: send.payload,
            });
        match result {
            Ok(effects) => self.apply_effects(effects, cx),
            Err(error) => log::error!("gpui_embedded: delivering shared message failed: {error:#}"),
        }
    }

    fn send_to_guest(
        &mut self,
        name: &str,
        method: &str,
        payload: Vec<u8>,
        ack: Option<AckSender>,
        response: Option<ResponseSender>,
        cx: &mut Context<Self>,
    ) {
        let request_id = response.map(|sender| {
            self.shared.next_request_id += 1;
            let request_id = self.shared.next_request_id;
            self.shared.pending_responses.insert(request_id, sender);
            request_id
        });
        let Some(projection) = self.shared.projections_by_name.get_mut(name) else {
            log::warn!("gpui_embedded: send to unknown shared entity {name:?}");
            return;
        };
        projection.next_sequence += 1;
        let sequence = projection.next_sequence;
        if let Some(ack) = ack {
            projection.pending_acks.push((sequence, ack));
        }
        let entity_id = projection.entity_id;
        let send = shared_entities::PendingSend {
            sequence,
            request_id,
            method: method.to_string(),
            payload,
        };
        match entity_id {
            Some(entity_id) => self.deliver_message_to_guest(entity_id, send, cx),
            None => projection.pending_sends.push(send),
        }
    }

    pub fn init(&mut self, cx: &mut Context<Self>) {
        let result = self.instance.borrow_mut().init();
        match result {
            Ok(effects) => self.apply_effects(effects, cx),
            Err(error) => log::error!("gpui_embedded: plugin init failed: {error:#}"),
        }
    }

    pub fn create_view(
        &mut self,
        view_id: u32,
        size: Size<Pixels>,
        scale: f32,
        cx: &mut Context<Self>,
    ) -> Entity<PluginViewState> {
        let host = cx.weak_entity();
        let images = self.images.clone();
        let view = cx.new(|cx| PluginViewState::new(view_id, size, host, images, cx));
        self.views.insert(view_id, view.clone());

        let result = self.instance.borrow_mut().create_view(view_id, size, scale);
        match result {
            Ok(effects) => self.apply_effects(effects, cx),
            Err(error) => log::error!("gpui_embedded: create_view({view_id}) failed: {error:#}"),
        }
        view
    }

    pub fn resize_view(
        &mut self,
        view_id: u32,
        size: Size<Pixels>,
        scale: f32,
        cx: &mut Context<Self>,
    ) {
        let result = self.instance.borrow_mut().resize_view(view_id, size, scale);
        match result {
            Ok(effects) => self.apply_effects(effects, cx),
            Err(error) => log::error!("gpui_embedded: resize_view({view_id}) failed: {error:#}"),
        }
    }

    pub fn handle_mouse(
        &mut self,
        view_id: u32,
        event: bindings::MouseEvent,
        cx: &mut Context<Self>,
    ) {
        let result = self.instance.borrow_mut().handle_mouse(view_id, event);
        match result {
            Ok(effects) => self.apply_effects(effects, cx),
            Err(error) => log::error!("gpui_embedded: handle_mouse({view_id}) failed: {error:#}"),
        }
    }

    pub fn handle_key(&mut self, view_id: u32, event: bindings::KeyEvent, cx: &mut Context<Self>) {
        let result = self.instance.borrow_mut().handle_key(view_id, event);
        match result {
            Ok(effects) => self.apply_effects(effects, cx),
            Err(error) => log::error!("gpui_embedded: handle_key({view_id}) failed: {error:#}"),
        }
    }

    fn tick(&mut self, cx: &mut Context<Self>) {
        let result = self.instance.borrow_mut().tick();
        match result {
            Ok(effects) => self.apply_effects(effects, cx),
            Err(error) => log::error!("gpui_embedded: tick failed: {error:#}"),
        }
    }

    fn apply_effects(&mut self, effects: Effects, cx: &mut Context<Self>) {
        self.drain_pending_releases(cx);

        for announcement in effects.shared_announcements {
            self.bind_projection(announcement, cx);
        }

        // Snapshots strictly before responses: a call's receipt must only resolve once the
        // local replica already reflects it.
        for snapshot in effects.shared_snapshots {
            self.apply_guest_snapshot(snapshot, cx);
        }

        for response in effects.shared_responses {
            let Some(sender) = self.shared.pending_responses.remove(&response.request_id) else {
                log::warn!(
                    "gpui_embedded: response for unknown request {}",
                    response.request_id
                );
                continue;
            };
            sender.send(response.outcome).ok();
        }

        for message in effects.shared_messages {
            let response = self.shared.dispatch(
                message.entity_id,
                message.sequence,
                &message.method,
                &message.payload,
                cx,
            );
            let entity_id = message.entity_id;
            let outcome = match response {
                Ok(HandlerResponse::Ready(result)) => result.map_err(|error| {
                    log::error!("gpui_embedded: shared message failed: {error:#}");
                    format!("{error:#}")
                }),
                Ok(HandlerResponse::Pending(task)) => {
                    // The handler's work outlives this delivery; ack and response happen
                    // when its task resolves, preserving the same publish-before-respond
                    // order as the synchronous path below.
                    let request_id = message.request_id;
                    cx.spawn(async move |host, cx| {
                        let outcome = task.await.map_err(|error| {
                            log::error!("gpui_embedded: shared message failed: {error:#}");
                            format!("{error:#}")
                        });
                        host.update(cx, |host, cx| {
                            let needs_ack = host
                                .shared
                                .home_mut(entity_id)
                                .is_some_and(|home| home.published_ack < home.applied_sequence);
                            if needs_ack {
                                host.publish_home(entity_id, cx);
                            }
                            if let Some(request_id) = request_id {
                                host.deliver_response_to_guest(
                                    bindings::SharedResponse {
                                        request_id,
                                        outcome,
                                    },
                                    cx,
                                );
                            }
                        })
                        .ok();
                    })
                    .detach();
                    continue;
                }
                Err(error) => {
                    log::error!("gpui_embedded: shared message failed: {error:#}");
                    Err(format!("{error:#}"))
                }
            };
            // Both follow-ups are deferred so they run after the handler's notify effects
            // flush: first the acking snapshot (deduped via published_ack), then the
            // response, preserving snapshot-before-response ordering on the guest.
            let host = cx.weak_entity();
            cx.defer(move |cx| {
                host.update(cx, |host, cx| {
                    let needs_ack = host
                        .shared
                        .home_mut(entity_id)
                        .is_some_and(|home| home.published_ack < home.applied_sequence);
                    if needs_ack {
                        host.publish_home(entity_id, cx);
                    }
                })
                .ok();
            });
            if let Some(request_id) = message.request_id {
                let host = cx.weak_entity();
                cx.defer(move |cx| {
                    host.update(cx, |host, cx| {
                        host.deliver_response_to_guest(
                            bindings::SharedResponse {
                                request_id,
                                outcome,
                            },
                            cx,
                        );
                    })
                    .ok();
                });
            }
        }

        for (view_id, list) in effects.scene_updates {
            self.ingest_images(&list);
            if let Some(view) = self.views.get(&view_id) {
                view.update(cx, |view, cx| {
                    view.set_display_list(list);
                    cx.notify();
                });
            } else {
                log::warn!("gpui_embedded: update-scene for unknown view {view_id}");
            }
        }

        if let Some(cursor) = effects.cursor_style {
            for view in self.views.values() {
                view.update(cx, |view, cx| {
                    view.set_cursor(cursor);
                    cx.notify();
                });
            }
        }

        if let Some(delay) = effects.tick_delay_ms {
            self.scheduled_tick = Some(cx.spawn(async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(delay as u64))
                    .await;
                this.update(cx, |this, cx| this.tick(cx)).ok();
            }));
        }
    }

    fn deliver_response_to_guest(
        &mut self,
        response: bindings::SharedResponse,
        cx: &mut Context<Self>,
    ) {
        let result = self
            .instance
            .borrow_mut()
            .deliver_shared_response(&response);
        match result {
            Ok(effects) => self.apply_effects(effects, cx),
            Err(error) => {
                log::error!("gpui_embedded: delivering shared response failed: {error:#}")
            }
        }
    }

    fn apply_guest_snapshot(
        &mut self,
        snapshot: bindings::SharedSnapshot,
        cx: &mut Context<Self>,
    ) {
        let Some(name) = self
            .shared
            .projection_names_by_id
            .get(&snapshot.entity_id)
            .cloned()
        else {
            log::warn!(
                "gpui_embedded: snapshot for unknown shared entity {}",
                snapshot.entity_id
            );
            return;
        };
        let Some(projection) = self.shared.projections_by_name.get_mut(&name) else {
            return;
        };
        let apply_snapshot = projection.apply_snapshot.clone();
        if let Err(error) = apply_snapshot(&snapshot.payload, cx) {
            log::error!("gpui_embedded: failed to apply shared snapshot: {error:#}");
            return;
        }
        // Resolve receipts only after the replica reflects the acked writes.
        let Some(projection) = self.shared.projections_by_name.get_mut(&name) else {
            return;
        };
        let mut acked = Vec::new();
        projection.pending_acks.retain_mut(|(sequence, sender)| {
            if *sequence <= snapshot.acked_sequence {
                let (drained_tx, _drained_rx) = futures::channel::oneshot::channel();
                acked.push(std::mem::replace(sender, drained_tx));
                false
            } else {
                true
            }
        });
        for sender in acked {
            sender.send(()).ok();
        }
    }

    /// Decode freshly shipped image payloads into `RenderImage`s. Bytes are premultiplied
    /// BGRA straight from the guest's atlas pipeline, so no conversion is needed: the host's
    /// atlas upload will read back exactly these bytes.
    fn ingest_images(&mut self, list: &bindings::DisplayList) {
        for payload in &list.new_images {
            let expected_len = payload.width as usize * payload.height as usize * 4;
            if payload.bytes.len() != expected_len {
                log::error!(
                    "gpui_embedded: image payload {} has {} bytes, expected {expected_len}",
                    payload.id,
                    payload.bytes.len()
                );
                continue;
            }
            let Some(buffer) = image::RgbaImage::from_raw(
                payload.width,
                payload.height,
                payload.bytes.clone(),
            ) else {
                log::error!("gpui_embedded: image payload {} is malformed", payload.id);
                continue;
            };
            let render_image = Arc::new(gpui::RenderImage::new(smallvec::smallvec![
                image::Frame::new(buffer)
            ]));
            self.images.borrow_mut().insert(payload.id, render_image);
        }
    }
}
