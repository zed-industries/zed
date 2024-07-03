use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Deref;
use std::os::fd::AsRawFd;
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Context;
use async_task::Runnable;
use calloop::channel::Channel;

use collections::HashMap;

use futures::channel::oneshot;
use mio::{Interest, Token, Waker};
use util::ResultExt;
use x11rb::connection::{Connection, RequestConnection};
use x11rb::cursor;
use x11rb::errors::ConnectionError;
use x11rb::protocol::xinput::ConnectionExt;
use x11rb::protocol::xkb::ConnectionExt as _;
use x11rb::protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt as _};
use x11rb::protocol::{randr, render, xinput, xkb, xproto, Event};
use x11rb::resource_manager::Database;
use x11rb::xcb_ffi::XCBConnection;
use xim::{x11rb::X11rbClient, Client};
use xim::{AttributeName, InputStyle};
use xkbc::x11::ffi::{XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION};
use xkbcommon::xkb as xkbc;

use crate::platform::linux::LinuxClient;
use crate::platform::{LinuxCommon, PlatformWindow};
use crate::{
    modifiers_from_xinput_info, point, px, AnyWindowHandle, Bounds, ClipboardItem, CursorStyle,
    DisplayId, Keystroke, Modifiers, ModifiersChangedEvent, Pixels, PlatformDisplay, PlatformInput,
    Point, QuitSignal, ScrollDelta, Size, TouchPhase, WindowParams, X11Window,
};

use super::{
    super::{get_xkb_compose_state, open_uri_internal, SCROLL_LINES},
    X11Display, X11WindowStatePtr, XcbAtoms,
};
use super::{button_of_key, modifiers_from_state, pressed_button_from_mask};
use super::{XimCallbackEvent, XimHandler};
use crate::platform::linux::is_within_click_distance;
use crate::platform::linux::platform::DOUBLE_CLICK_INTERVAL;
use crate::platform::linux::xdg_desktop_portal::{Event as XDPEvent, XDPEventSource};

pub(super) const XINPUT_MASTER_DEVICE: u16 = 1;

pub(crate) struct WindowRef {
    window: X11WindowStatePtr,
}

impl WindowRef {
    pub fn handle(&self) -> AnyWindowHandle {
        self.window.state.borrow().handle
    }
}

impl Deref for WindowRef {
    type Target = X11WindowStatePtr;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum EventHandlerError {
    XCBConnectionError(ConnectionError),
    XIMClientError(xim::ClientError),
}

impl std::error::Error for EventHandlerError {}

impl std::fmt::Display for EventHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventHandlerError::XCBConnectionError(err) => err.fmt(f),
            EventHandlerError::XIMClientError(err) => err.fmt(f),
        }
    }
}

impl From<ConnectionError> for EventHandlerError {
    fn from(err: ConnectionError) -> Self {
        EventHandlerError::XCBConnectionError(err)
    }
}

impl From<xim::ClientError> for EventHandlerError {
    fn from(err: xim::ClientError) -> Self {
        EventHandlerError::XIMClientError(err)
    }
}

pub struct X11ClientState {
    /// poll is in an Option so we can take it out in `run()` without
    /// mutating self.
    poll: Option<mio::Poll>,
    quit_signal_rx: oneshot::Receiver<()>,
    runnables: Channel<Runnable>,
    xdp_event_source: XDPEventSource,

    pub(crate) last_click: Instant,
    pub(crate) last_location: Point<Pixels>,
    pub(crate) current_count: usize,

    pub(crate) scale_factor: f32,
    pub(crate) xcb_connection: Rc<XCBConnection>,
    pub(crate) x_root_index: usize,
    pub(crate) _resource_database: Database,
    pub(crate) atoms: XcbAtoms,
    pub(crate) windows: HashMap<xproto::Window, WindowRef>,
    pub(crate) focused_window: Option<xproto::Window>,
    pub(crate) xkb: xkbc::State,
    pub(crate) ximc: Option<X11rbClient<Rc<XCBConnection>>>,
    pub(crate) xim_handler: Option<XimHandler>,
    pub modifiers: Modifiers,

    pub(crate) compose_state: Option<xkbc::compose::State>,
    pub(crate) pre_edit_text: Option<String>,
    pub(crate) composing: bool,
    pub(crate) cursor_handle: cursor::Handle,
    pub(crate) cursor_styles: HashMap<xproto::Window, CursorStyle>,
    pub(crate) cursor_cache: HashMap<CursorStyle, xproto::Cursor>,

    pub(crate) scroll_class_data: Vec<xinput::DeviceClassDataScroll>,
    pub(crate) scroll_x: Option<f32>,
    pub(crate) scroll_y: Option<f32>,

    pub(crate) common: LinuxCommon,
    pub(crate) clipboard: x11_clipboard::Clipboard,
    pub(crate) clipboard_item: Option<ClipboardItem>,
}

#[derive(Clone)]
pub struct X11ClientStatePtr(pub Weak<RefCell<X11ClientState>>);

impl X11ClientStatePtr {
    pub fn drop_window(&self, x_window: u32) {
        let client = X11Client(self.0.upgrade().expect("client already dropped"));
        let mut state = client.0.borrow_mut();

        if state.windows.remove(&x_window).is_none() {
            log::warn!(
                "failed to remove X window {} from client state, does not exist",
                x_window
            );
        }

        state.cursor_styles.remove(&x_window);

        if state.windows.is_empty() {
            state.common.quit_signal.quit();
        }
    }
}

struct ChannelQuitSignal {
    tx: Option<oneshot::Sender<()>>,
    waker: Option<Arc<Waker>>,
}

impl ChannelQuitSignal {
    fn new(waker: Option<Arc<Waker>>) -> (Self, oneshot::Receiver<()>) {
        let (tx, rx) = oneshot::channel::<()>();

        let quit_signal = ChannelQuitSignal {
            tx: Some(tx),
            waker,
        };

        (quit_signal, rx)
    }
}

impl QuitSignal for ChannelQuitSignal {
    fn quit(&mut self) {
        if let Some(tx) = self.tx.take() {
            tx.send(()).log_err();
            if let Some(waker) = self.waker.as_ref() {
                waker.wake().ok();
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct X11Client(Rc<RefCell<X11ClientState>>);

impl X11Client {
    pub(crate) fn new() -> Self {
        let mut poll = mio::Poll::new().unwrap();

        let waker = Arc::new(Waker::new(poll.registry(), WAKER_TOKEN).unwrap());

        let (quit_signal, quit_signal_rx) = ChannelQuitSignal::new(Some(waker.clone()));
        let (common, runnables) = LinuxCommon::new(Box::new(quit_signal), Some(waker.clone()));

        let (xcb_connection, x_root_index) = XCBConnection::connect(None).unwrap();
        xcb_connection
            .prefetch_extension_information(xkb::X11_EXTENSION_NAME)
            .unwrap();
        xcb_connection
            .prefetch_extension_information(randr::X11_EXTENSION_NAME)
            .unwrap();
        xcb_connection
            .prefetch_extension_information(render::X11_EXTENSION_NAME)
            .unwrap();
        xcb_connection
            .prefetch_extension_information(xinput::X11_EXTENSION_NAME)
            .unwrap();

        let xinput_version = xcb_connection
            .xinput_xi_query_version(2, 0)
            .unwrap()
            .reply()
            .unwrap();
        assert!(
            xinput_version.major_version >= 2,
            "XInput Extension v2 not supported."
        );

        let master_device_query = xcb_connection
            .xinput_xi_query_device(XINPUT_MASTER_DEVICE)
            .unwrap()
            .reply()
            .unwrap();
        let scroll_class_data = master_device_query
            .infos
            .iter()
            .find(|info| info.type_ == xinput::DeviceType::MASTER_POINTER)
            .unwrap()
            .classes
            .iter()
            .filter_map(|class| class.data.as_scroll())
            .map(|class| *class)
            .collect::<Vec<_>>();

        let atoms = XcbAtoms::new(&xcb_connection).unwrap();
        let xkb = xcb_connection
            .xkb_use_extension(XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION)
            .unwrap();

        let atoms = atoms.reply().unwrap();
        let xkb = xkb.reply().unwrap();
        let events = xkb::EventType::STATE_NOTIFY;
        xcb_connection
            .xkb_select_events(
                xkb::ID::USE_CORE_KBD.into(),
                0u8.into(),
                events,
                0u8.into(),
                0u8.into(),
                &xkb::SelectEventsAux::new(),
            )
            .unwrap();
        assert!(xkb.supported);

        let xkb_context = xkbc::Context::new(xkbc::CONTEXT_NO_FLAGS);
        let xkb_state = {
            let xkb_device_id = xkbc::x11::get_core_keyboard_device_id(&xcb_connection);
            let xkb_keymap = xkbc::x11::keymap_new_from_device(
                &xkb_context,
                &xcb_connection,
                xkb_device_id,
                xkbc::KEYMAP_COMPILE_NO_FLAGS,
            );
            xkbc::x11::state_new_from_device(&xkb_keymap, &xcb_connection, xkb_device_id)
        };
        let compose_state = get_xkb_compose_state(&xkb_context);
        let resource_database = x11rb::resource_manager::new_from_default(&xcb_connection).unwrap();

        let scale_factor = resource_database
            .get_value("Xft.dpi", "Xft.dpi")
            .ok()
            .flatten()
            .map(|dpi: f32| dpi / 96.0)
            .unwrap_or(1.0);

        let cursor_handle = cursor::Handle::new(&xcb_connection, x_root_index, &resource_database)
            .unwrap()
            .reply()
            .unwrap();

        let clipboard = x11_clipboard::Clipboard::new().unwrap();

        let xcb_connection = Rc::new(xcb_connection);

        let ximc = X11rbClient::init(Rc::clone(&xcb_connection), x_root_index, None).ok();
        let xim_handler = if ximc.is_some() {
            Some(XimHandler::new())
        } else {
            None
        };

        let xdp_event_source =
            XDPEventSource::new(&common.background_executor, Some(waker.clone()));

        X11Client(Rc::new(RefCell::new(X11ClientState {
            poll: Some(poll),
            runnables,

            xdp_event_source,
            quit_signal_rx,
            common,

            modifiers: Modifiers::default(),
            last_click: Instant::now(),
            last_location: Point::new(px(0.0), px(0.0)),
            current_count: 0,
            scale_factor,

            xcb_connection,
            x_root_index,
            _resource_database: resource_database,
            atoms,
            windows: HashMap::default(),
            focused_window: None,
            xkb: xkb_state,
            ximc,
            xim_handler,

            compose_state,
            pre_edit_text: None,
            composing: false,

            cursor_handle,
            cursor_styles: HashMap::default(),
            cursor_cache: HashMap::default(),

            scroll_class_data,
            scroll_x: None,
            scroll_y: None,

            clipboard,
            clipboard_item: None,
        })))
    }

    pub fn enable_ime(&self) {
        let mut state = self.0.borrow_mut();
        if state.ximc.is_none() {
            return;
        }

        let mut ximc = state.ximc.take().unwrap();
        let mut xim_handler = state.xim_handler.take().unwrap();
        let mut ic_attributes = ximc
            .build_ic_attributes()
            .push(
                AttributeName::InputStyle,
                InputStyle::PREEDIT_CALLBACKS
                    | InputStyle::STATUS_NOTHING
                    | InputStyle::PREEDIT_NONE,
            )
            .push(AttributeName::ClientWindow, xim_handler.window)
            .push(AttributeName::FocusWindow, xim_handler.window);

        let window_id = state.focused_window;
        drop(state);
        if let Some(window_id) = window_id {
            let window = self.get_window(window_id).unwrap();
            if let Some(area) = window.get_ime_area() {
                ic_attributes =
                    ic_attributes.nested_list(xim::AttributeName::PreeditAttributes, |b| {
                        b.push(
                            xim::AttributeName::SpotLocation,
                            xim::Point {
                                x: u32::from(area.origin.x + area.size.width) as i16,
                                y: u32::from(area.origin.y + area.size.height) as i16,
                            },
                        );
                    });
            }
        }
        ximc.create_ic(xim_handler.im_id, ic_attributes.build())
            .ok();
        state = self.0.borrow_mut();
        state.xim_handler = Some(xim_handler);
        state.ximc = Some(ximc);
    }

    pub fn disable_ime(&self) {
        let mut state = self.0.borrow_mut();
        state.composing = false;
        if let Some(mut ximc) = state.ximc.take() {
            let xim_handler = state.xim_handler.as_ref().unwrap();
            ximc.destroy_ic(xim_handler.im_id, xim_handler.ic_id).ok();
            state.ximc = Some(ximc);
        }
    }

    fn get_window(&self, win: xproto::Window) -> Option<X11WindowStatePtr> {
        let state = self.0.borrow();
        state
            .windows
            .get(&win)
            .filter(|window_reference| !window_reference.window.state.borrow().destroyed)
            .map(|window_reference| window_reference.window.clone())
    }

    fn read_x11_events(&self) -> (HashSet<u32>, Vec<Event>) {
        let mut events = Vec::new();
        let mut windows_to_refresh = HashSet::new();
        let mut state = self.0.borrow_mut();

        let mut last_key_release: Option<Event> = None;

        loop {
            match state.xcb_connection.poll_for_event() {
                Ok(Some(event)) => {
                    if let Event::Expose(expose_event) = event {
                        windows_to_refresh.insert(expose_event.window);
                    } else {
                        match event {
                            Event::KeyRelease(_) => {
                                last_key_release = Some(event);
                            }
                            Event::KeyPress(key_press) => {
                                if let Some(Event::KeyRelease(key_release)) =
                                    last_key_release.take()
                                {
                                    // We ignore that last KeyRelease if it's too close to this KeyPress,
                                    // suggesting that it's auto-generated by X11 as a key-repeat event.
                                    if key_release.detail != key_press.detail
                                        || key_press.time.wrapping_sub(key_release.time) > 20
                                    {
                                        events.push(Event::KeyRelease(key_release));
                                    }
                                }
                                events.push(Event::KeyPress(key_press));
                            }
                            _ => {
                                if let Some(release_event) = last_key_release.take() {
                                    events.push(release_event);
                                }
                                events.push(event);
                            }
                        }
                    }
                }
                Ok(None) => {
                    // Add any remaining stored KeyRelease event
                    if let Some(release_event) = last_key_release.take() {
                        events.push(release_event);
                    }
                    break;
                }
                Err(e) => {
                    log::warn!("error polling for X11 events: {e:?}");
                    break;
                }
            }
        }

        (windows_to_refresh, events)
    }

    fn process_x11_events(&self, events: Vec<Event>) {
        for event in events.into_iter() {
            let mut state = self.0.borrow_mut();
            if state.ximc.is_none() || state.xim_handler.is_none() {
                drop(state);
                self.handle_event(event);
                continue;
            }

            let mut ximc = state.ximc.take().unwrap();
            let mut xim_handler = state.xim_handler.take().unwrap();
            let xim_connected = xim_handler.connected;
            drop(state);

            // let xim_filtered = false;
            let xim_filtered = match ximc.filter_event(&event, &mut xim_handler) {
                Ok(handled) => handled,
                Err(err) => {
                    log::error!("XIMClientError: {}", err);
                    false
                }
            };
            let xim_callback_event = xim_handler.last_callback_event.take();

            let mut state = self.0.borrow_mut();
            state.ximc = Some(ximc);
            state.xim_handler = Some(xim_handler);

            if let Some(event) = xim_callback_event {
                drop(state);
                self.handle_xim_callback_event(event);
            } else {
                drop(state);
            }

            if xim_filtered {
                continue;
            }

            if xim_connected {
                self.xim_handle_event(event);
            } else {
                self.handle_event(event);
            }
        }
    }

    fn handle_event(&self, event: Event) -> Option<()> {
        match event {
            Event::ClientMessage(event) => {
                let window = self.get_window(event.window)?;
                let [atom, _arg1, arg2, arg3, _arg4] = event.data.as_data32();
                let mut state = self.0.borrow_mut();

                if atom == state.atoms.WM_DELETE_WINDOW {
                    // window "x" button clicked by user
                    if window.should_close() {
                        // Rest of the close logic is handled in drop_window()
                        window.close();
                    }
                } else if atom == state.atoms._NET_WM_SYNC_REQUEST {
                    window.state.borrow_mut().last_sync_counter =
                        Some(x11rb::protocol::sync::Int64 {
                            lo: arg2,
                            hi: arg3 as i32,
                        })
                }
            }
            Event::ConfigureNotify(event) => {
                let bounds = Bounds {
                    origin: Point {
                        x: event.x.into(),
                        y: event.y.into(),
                    },
                    size: Size {
                        width: event.width.into(),
                        height: event.height.into(),
                    },
                };
                let window = self.get_window(event.window)?;
                window.configure(bounds);
            }
            Event::PropertyNotify(event) => {
                let window = self.get_window(event.window)?;
                window.property_notify(event);
            }
            Event::Expose(event) => {
                let window = self.get_window(event.window)?;
                window.refresh();
            }
            Event::FocusIn(event) => {
                let window = self.get_window(event.event)?;
                window.set_focused(true);
                let mut state = self.0.borrow_mut();
                state.focused_window = Some(event.event);
                drop(state);
                self.enable_ime();
            }
            Event::FocusOut(event) => {
                let window = self.get_window(event.event)?;
                window.set_focused(false);
                let mut state = self.0.borrow_mut();
                state.focused_window = None;
                if let Some(compose_state) = state.compose_state.as_mut() {
                    compose_state.reset();
                }
                state.pre_edit_text.take();
                drop(state);
                self.disable_ime();
                window.handle_ime_delete();
            }
            Event::XkbStateNotify(event) => {
                let mut state = self.0.borrow_mut();
                state.xkb.update_mask(
                    event.base_mods.into(),
                    event.latched_mods.into(),
                    event.locked_mods.into(),
                    0,
                    0,
                    event.locked_group.into(),
                );

                let modifiers = Modifiers::from_xkb(&state.xkb);
                if state.modifiers == modifiers {
                    drop(state);
                } else {
                    let focused_window_id = state.focused_window?;
                    state.modifiers = modifiers;
                    drop(state);

                    let focused_window = self.get_window(focused_window_id)?;
                    focused_window.handle_input(PlatformInput::ModifiersChanged(
                        ModifiersChangedEvent { modifiers },
                    ));
                }
            }
            Event::KeyPress(event) => {
                let window = self.get_window(event.event)?;
                let mut state = self.0.borrow_mut();

                let modifiers = modifiers_from_state(event.state);
                state.modifiers = modifiers;

                let keystroke = {
                    let code = event.detail.into();
                    let mut keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                    state.xkb.update_key(code, xkbc::KeyDirection::Down);
                    let keysym = state.xkb.key_get_one_sym(code);
                    if keysym.is_modifier_key() {
                        return Some(());
                    }
                    if let Some(mut compose_state) = state.compose_state.take() {
                        compose_state.feed(keysym);
                        match compose_state.status() {
                            xkbc::Status::Composed => {
                                state.pre_edit_text.take();
                                keystroke.ime_key = compose_state.utf8();
                                if let Some(keysym) = compose_state.keysym() {
                                    keystroke.key = xkbc::keysym_get_name(keysym);
                                }
                            }
                            xkbc::Status::Composing => {
                                keystroke.ime_key = None;
                                state.pre_edit_text = compose_state
                                    .utf8()
                                    .or(crate::Keystroke::underlying_dead_key(keysym));
                                let pre_edit =
                                    state.pre_edit_text.clone().unwrap_or(String::default());
                                drop(state);
                                window.handle_ime_preedit(pre_edit);
                                state = self.0.borrow_mut();
                            }
                            xkbc::Status::Cancelled => {
                                let pre_edit = state.pre_edit_text.take();
                                drop(state);
                                if let Some(pre_edit) = pre_edit {
                                    window.handle_ime_commit(pre_edit);
                                }
                                if let Some(current_key) = Keystroke::underlying_dead_key(keysym) {
                                    window.handle_ime_preedit(current_key);
                                }
                                state = self.0.borrow_mut();
                                compose_state.feed(keysym);
                            }
                            _ => {}
                        }
                        state.compose_state = Some(compose_state);
                    }
                    keystroke
                };
                drop(state);
                window.handle_input(PlatformInput::KeyDown(crate::KeyDownEvent {
                    keystroke,
                    is_held: false,
                }));
            }
            Event::KeyRelease(event) => {
                let window = self.get_window(event.event)?;
                let mut state = self.0.borrow_mut();

                let modifiers = modifiers_from_state(event.state);
                state.modifiers = modifiers;

                let keystroke = {
                    let code = event.detail.into();
                    let keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                    state.xkb.update_key(code, xkbc::KeyDirection::Up);
                    let keysym = state.xkb.key_get_one_sym(code);
                    if keysym.is_modifier_key() {
                        return Some(());
                    }
                    keystroke
                };
                drop(state);
                window.handle_input(PlatformInput::KeyUp(crate::KeyUpEvent { keystroke }));
            }
            Event::XinputButtonPress(event) => {
                let window = self.get_window(event.event)?;
                let mut state = self.0.borrow_mut();

                let modifiers = modifiers_from_xinput_info(event.mods);
                state.modifiers = modifiers;

                let position = point(
                    px(event.event_x as f32 / u16::MAX as f32 / state.scale_factor),
                    px(event.event_y as f32 / u16::MAX as f32 / state.scale_factor),
                );

                if state.composing && state.ximc.is_some() {
                    drop(state);
                    self.disable_ime();
                    self.enable_ime();
                    window.handle_ime_unmark();
                    state = self.0.borrow_mut();
                } else if let Some(text) = state.pre_edit_text.take() {
                    if let Some(compose_state) = state.compose_state.as_mut() {
                        compose_state.reset();
                    }
                    drop(state);
                    window.handle_ime_commit(text);
                    state = self.0.borrow_mut();
                }
                if let Some(button) = button_of_key(event.detail.try_into().unwrap()) {
                    let click_elapsed = state.last_click.elapsed();

                    if click_elapsed < DOUBLE_CLICK_INTERVAL
                        && is_within_click_distance(state.last_location, position)
                    {
                        state.current_count += 1;
                    } else {
                        state.current_count = 1;
                    }

                    state.last_click = Instant::now();
                    state.last_location = position;
                    let current_count = state.current_count;

                    drop(state);
                    window.handle_input(PlatformInput::MouseDown(crate::MouseDownEvent {
                        button,
                        position,
                        modifiers,
                        click_count: current_count,
                        first_mouse: false,
                    }));
                } else {
                    log::warn!("Unknown button press: {event:?}");
                }
            }
            Event::XinputButtonRelease(event) => {
                let window = self.get_window(event.event)?;
                let mut state = self.0.borrow_mut();
                let modifiers = modifiers_from_xinput_info(event.mods);
                state.modifiers = modifiers;

                let position = point(
                    px(event.event_x as f32 / u16::MAX as f32 / state.scale_factor),
                    px(event.event_y as f32 / u16::MAX as f32 / state.scale_factor),
                );
                if let Some(button) = button_of_key(event.detail.try_into().unwrap()) {
                    let click_count = state.current_count;
                    drop(state);
                    window.handle_input(PlatformInput::MouseUp(crate::MouseUpEvent {
                        button,
                        position,
                        modifiers,
                        click_count,
                    }));
                }
            }
            Event::XinputMotion(event) => {
                let window = self.get_window(event.event)?;
                let mut state = self.0.borrow_mut();
                let pressed_button = pressed_button_from_mask(event.button_mask[0]);
                let position = point(
                    px(event.event_x as f32 / u16::MAX as f32 / state.scale_factor),
                    px(event.event_y as f32 / u16::MAX as f32 / state.scale_factor),
                );
                let modifiers = modifiers_from_xinput_info(event.mods);
                state.modifiers = modifiers;
                drop(state);

                let axisvalues = event
                    .axisvalues
                    .iter()
                    .map(|axisvalue| fp3232_to_f32(*axisvalue))
                    .collect::<Vec<_>>();

                if event.valuator_mask[0] & 3 != 0 {
                    window.handle_input(PlatformInput::MouseMove(crate::MouseMoveEvent {
                        position,
                        pressed_button,
                        modifiers,
                    }));
                }

                let mut valuator_idx = 0;
                let scroll_class_data = self.0.borrow().scroll_class_data.clone();
                for shift in 0..32 {
                    if (event.valuator_mask[0] >> shift) & 1 == 0 {
                        continue;
                    }

                    for scroll_class in &scroll_class_data {
                        if scroll_class.scroll_type == xinput::ScrollType::HORIZONTAL
                            && scroll_class.number == shift
                        {
                            let new_scroll = axisvalues[valuator_idx]
                                / fp3232_to_f32(scroll_class.increment)
                                * SCROLL_LINES as f32;
                            let old_scroll = self.0.borrow().scroll_x;
                            self.0.borrow_mut().scroll_x = Some(new_scroll);

                            if let Some(old_scroll) = old_scroll {
                                let delta_scroll = old_scroll - new_scroll;
                                window.handle_input(PlatformInput::ScrollWheel(
                                    crate::ScrollWheelEvent {
                                        position,
                                        delta: ScrollDelta::Lines(Point::new(delta_scroll, 0.0)),
                                        modifiers,
                                        touch_phase: TouchPhase::default(),
                                    },
                                ));
                            }
                        } else if scroll_class.scroll_type == xinput::ScrollType::VERTICAL
                            && scroll_class.number == shift
                        {
                            // the `increment` is the valuator delta equivalent to one positive unit of scrolling. Here that means SCROLL_LINES lines.
                            let new_scroll = axisvalues[valuator_idx]
                                / fp3232_to_f32(scroll_class.increment)
                                * SCROLL_LINES as f32;
                            let old_scroll = self.0.borrow().scroll_y;
                            self.0.borrow_mut().scroll_y = Some(new_scroll);

                            if let Some(old_scroll) = old_scroll {
                                let delta_scroll = old_scroll - new_scroll;
                                let (x, y) = if !modifiers.shift {
                                    (0.0, delta_scroll)
                                } else {
                                    (delta_scroll, 0.0)
                                };
                                window.handle_input(PlatformInput::ScrollWheel(
                                    crate::ScrollWheelEvent {
                                        position,
                                        delta: ScrollDelta::Lines(Point::new(x, y)),
                                        modifiers,
                                        touch_phase: TouchPhase::default(),
                                    },
                                ));
                            }
                        }
                    }

                    valuator_idx += 1;
                }
            }
            Event::XinputLeave(event) if event.mode == xinput::NotifyMode::NORMAL => {
                self.0.borrow_mut().scroll_x = None; // Set last scroll to `None` so that a large delta isn't created if scrolling is done outside the window (the valuator is global)
                self.0.borrow_mut().scroll_y = None;

                let window = self.get_window(event.event)?;
                let mut state = self.0.borrow_mut();
                let pressed_button = pressed_button_from_mask(event.buttons[0]);
                let position = point(
                    px(event.event_x as f32 / u16::MAX as f32 / state.scale_factor),
                    px(event.event_y as f32 / u16::MAX as f32 / state.scale_factor),
                );
                let modifiers = modifiers_from_xinput_info(event.mods);
                state.modifiers = modifiers;
                drop(state);

                window.handle_input(PlatformInput::MouseExited(crate::MouseExitEvent {
                    pressed_button,
                    position,
                    modifiers,
                }));
            }
            _ => {}
        };

        Some(())
    }

    fn handle_xim_callback_event(&self, event: XimCallbackEvent) {
        match event {
            XimCallbackEvent::XimXEvent(event) => {
                self.handle_event(event);
            }
            XimCallbackEvent::XimCommitEvent(window, text) => {
                self.xim_handle_commit(window, text);
            }
            XimCallbackEvent::XimPreeditEvent(window, text) => {
                self.xim_handle_preedit(window, text);
            }
        };
    }

    fn xim_handle_event(&self, event: Event) -> Option<()> {
        match event {
            Event::KeyPress(event) | Event::KeyRelease(event) => {
                let mut state = self.0.borrow_mut();
                let mut ximc = state.ximc.take().unwrap();
                let mut xim_handler = state.xim_handler.take().unwrap();
                drop(state);
                xim_handler.window = event.event;
                ximc.forward_event(
                    xim_handler.im_id,
                    xim_handler.ic_id,
                    xim::ForwardEventFlag::empty(),
                    &event,
                )
                .unwrap();
                let mut state = self.0.borrow_mut();
                state.ximc = Some(ximc);
                state.xim_handler = Some(xim_handler);
                drop(state);
            }
            event => {
                self.handle_event(event);
            }
        }
        Some(())
    }

    fn xim_handle_commit(&self, window: xproto::Window, text: String) -> Option<()> {
        let window = self.get_window(window).unwrap();
        let mut state = self.0.borrow_mut();
        state.composing = false;
        drop(state);

        window.handle_ime_commit(text);
        Some(())
    }

    fn xim_handle_preedit(&self, window: xproto::Window, text: String) -> Option<()> {
        let window = self.get_window(window).unwrap();
        window.handle_ime_preedit(text);

        let mut state = self.0.borrow_mut();
        let mut ximc = state.ximc.take().unwrap();
        let mut xim_handler = state.xim_handler.take().unwrap();
        state.composing = true;
        drop(state);

        if let Some(area) = window.get_ime_area() {
            let ic_attributes = ximc
                .build_ic_attributes()
                .push(
                    xim::AttributeName::InputStyle,
                    xim::InputStyle::PREEDIT_CALLBACKS
                        | xim::InputStyle::STATUS_NOTHING
                        | xim::InputStyle::PREEDIT_POSITION,
                )
                .push(xim::AttributeName::ClientWindow, xim_handler.window)
                .push(xim::AttributeName::FocusWindow, xim_handler.window)
                .nested_list(xim::AttributeName::PreeditAttributes, |b| {
                    b.push(
                        xim::AttributeName::SpotLocation,
                        xim::Point {
                            x: u32::from(area.origin.x + area.size.width) as i16,
                            y: u32::from(area.origin.y + area.size.height) as i16,
                        },
                    );
                })
                .build();
            ximc.set_ic_values(xim_handler.im_id, xim_handler.ic_id, ic_attributes)
                .ok();
        }
        let mut state = self.0.borrow_mut();
        state.ximc = Some(ximc);
        state.xim_handler = Some(xim_handler);
        drop(state);
        Some(())
    }
}

const XCB_CONNECTION_TOKEN: Token = Token(0);
const WAKER_TOKEN: Token = Token(1);

impl LinuxClient for X11Client {
    fn compositor_name(&self) -> &'static str {
        "X11"
    }
    fn with_common<R>(&self, f: impl FnOnce(&mut LinuxCommon) -> R) -> R {
        f(&mut self.0.borrow_mut().common)
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        let state = self.0.borrow();
        let setup = state.xcb_connection.setup();
        setup
            .roots
            .iter()
            .enumerate()
            .filter_map(|(root_id, _)| {
                Some(Rc::new(X11Display::new(
                    &state.xcb_connection,
                    state.scale_factor,
                    root_id,
                )?) as Rc<dyn PlatformDisplay>)
            })
            .collect()
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        let state = self.0.borrow();

        Some(Rc::new(
            X11Display::new(
                &state.xcb_connection,
                state.scale_factor,
                state.x_root_index,
            )
            .expect("There should always be a root index"),
        ))
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        let state = self.0.borrow();

        Some(Rc::new(X11Display::new(
            &state.xcb_connection,
            state.scale_factor,
            id.0 as usize,
        )?))
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        params: WindowParams,
    ) -> anyhow::Result<Box<dyn PlatformWindow>> {
        let mut state = self.0.borrow_mut();
        let x_window = state.xcb_connection.generate_id().unwrap();

        let window = X11Window::new(
            handle,
            X11ClientStatePtr(Rc::downgrade(&self.0)),
            state.common.foreground_executor.clone(),
            params,
            &state.xcb_connection,
            state.x_root_index,
            x_window,
            &state.atoms,
            state.scale_factor,
            state.common.appearance,
        )?;

        let window_ref = WindowRef {
            window: window.0.clone(),
        };

        state.windows.insert(x_window, window_ref);
        Ok(Box::new(window))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        let mut state = self.0.borrow_mut();
        let Some(focused_window) = state.focused_window else {
            return;
        };
        let current_style = state
            .cursor_styles
            .get(&focused_window)
            .unwrap_or(&CursorStyle::Arrow);
        if *current_style == style {
            return;
        }

        let cursor = match state.cursor_cache.get(&style) {
            Some(cursor) => *cursor,
            None => {
                let cursor = state
                    .cursor_handle
                    .load_cursor(&state.xcb_connection, &style.to_icon_name())
                    .expect("failed to load cursor");
                state.cursor_cache.insert(style, cursor);
                cursor
            }
        };

        state.cursor_styles.insert(focused_window, style);
        state
            .xcb_connection
            .change_window_attributes(
                focused_window,
                &ChangeWindowAttributesAux {
                    cursor: Some(cursor),
                    ..Default::default()
                },
            )
            .expect("failed to change window cursor");
    }

    fn open_uri(&self, uri: &str) {
        open_uri_internal(uri, None);
    }

    fn write_to_primary(&self, item: crate::ClipboardItem) {
        let state = self.0.borrow_mut();
        state
            .clipboard
            .store(
                state.clipboard.setter.atoms.primary,
                state.clipboard.setter.atoms.utf8_string,
                item.text().as_bytes(),
            )
            .ok();
    }

    fn write_to_clipboard(&self, item: crate::ClipboardItem) {
        let mut state = self.0.borrow_mut();
        state
            .clipboard
            .store(
                state.clipboard.setter.atoms.clipboard,
                state.clipboard.setter.atoms.utf8_string,
                item.text().as_bytes(),
            )
            .ok();
        state.clipboard_item.replace(item);
    }

    fn read_from_primary(&self) -> Option<crate::ClipboardItem> {
        let state = self.0.borrow_mut();
        state
            .clipboard
            .load(
                state.clipboard.getter.atoms.primary,
                state.clipboard.getter.atoms.utf8_string,
                state.clipboard.getter.atoms.property,
                Duration::from_secs(3),
            )
            .map(|text| crate::ClipboardItem {
                text: String::from_utf8(text).unwrap(),
                metadata: None,
            })
            .ok()
    }

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        let state = self.0.borrow_mut();
        // if the last copy was from this app, return our cached item
        // which has metadata attached.
        if state
            .clipboard
            .setter
            .connection
            .get_selection_owner(state.clipboard.setter.atoms.clipboard)
            .ok()
            .and_then(|r| r.reply().ok())
            .map(|reply| reply.owner == state.clipboard.setter.window)
            .unwrap_or(false)
        {
            return state.clipboard_item.clone();
        }
        state
            .clipboard
            .load(
                state.clipboard.getter.atoms.clipboard,
                state.clipboard.getter.atoms.utf8_string,
                state.clipboard.getter.atoms.property,
                Duration::from_secs(3),
            )
            .map(|text| crate::ClipboardItem {
                text: String::from_utf8(text).unwrap(),
                metadata: None,
            })
            .ok()
    }

    fn run(&self) {
        let mut poll = self
            .0
            .borrow_mut()
            .poll
            .take()
            .context("no poll set on X11Client. calling run more than once is not possible")
            .unwrap();

        let xcb_fd = self.0.borrow().xcb_connection.as_raw_fd();
        let mut xcb_source = mio::unix::SourceFd(&xcb_fd);
        poll.registry()
            .register(&mut xcb_source, XCB_CONNECTION_TOKEN, Interest::READABLE)
            .unwrap();

        let mut events = mio::Events::with_capacity(1024);
        let mut next_refresh_needed = Instant::now();

        'run_loop: loop {
            let poll_timeout = next_refresh_needed - Instant::now();
            // We rounding the poll_timeout down so `mio` doesn't round it up to the next higher milliseconds
            let poll_timeout = Duration::from_millis(poll_timeout.as_millis() as u64);

            if poll_timeout >= Duration::from_millis(1) {
                let _ = poll.poll(&mut events, Some(poll_timeout));
            };

            let mut state = self.0.borrow_mut();

            // Check if we need to quit
            if let Ok(Some(())) = state.quit_signal_rx.try_recv() {
                return;
            }

            // Redraw windows
            let now = Instant::now();
            if now > next_refresh_needed {
                // This will be pulled down to 16ms (or less) if a window is open
                let mut frame_length = Duration::from_millis(100);

                let mut windows = vec![];
                for (_, window_ref) in state.windows.iter() {
                    if !window_ref.window.state.borrow().destroyed {
                        frame_length = frame_length.min(window_ref.window.refresh_rate());
                        windows.push(window_ref.window.clone());
                    }
                }

                drop(state);

                for window in windows {
                    window.refresh();
                }

                state = self.0.borrow_mut();

                // In the case that we're looping a bit too fast, slow down
                next_refresh_needed = now.max(next_refresh_needed) + frame_length;
            }

            // X11 events
            drop(state);

            loop {
                let (x_windows, events) = self.read_x11_events();
                for x_window in x_windows {
                    if let Some(window) = self.get_window(x_window) {
                        window.refresh();
                    }
                }

                if events.len() == 0 {
                    break;
                }
                self.process_x11_events(events);

                // When X11 is sending us events faster than we can handle we'll
                // let the frame rate drop to 10fps to try and avoid getting too behind.
                if Instant::now() > next_refresh_needed + Duration::from_millis(80) {
                    continue 'run_loop;
                }
            }

            state = self.0.borrow_mut();

            // Runnables
            while let Ok(runnable) = state.runnables.try_recv() {
                drop(state);
                runnable.run();
                state = self.0.borrow_mut();

                if Instant::now() + Duration::from_millis(1) >= next_refresh_needed {
                    continue 'run_loop;
                }
            }

            // XDG events
            if let Ok(event) = state.xdp_event_source.try_recv() {
                match event {
                    XDPEvent::WindowAppearance(appearance) => {
                        let mut windows = state
                            .windows
                            .values()
                            .map(|window| window.window.clone())
                            .collect::<Vec<_>>();
                        drop(state);

                        self.with_common(|common| common.appearance = appearance);
                        for mut window in windows {
                            window.set_appearance(appearance);
                        }
                    }
                    XDPEvent::CursorTheme(_) | XDPEvent::CursorSize(_) => {
                        // noop, X11 manages this for us.
                    }
                };
            };
        }
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        let state = self.0.borrow();
        state.focused_window.and_then(|focused_window| {
            state
                .windows
                .get(&focused_window)
                .map(|window| window.handle())
        })
    }
}

fn fp3232_to_f32(value: xinput::Fp3232) -> f32 {
    value.integral as f32 + value.frac as f32 / u32::MAX as f32
}
