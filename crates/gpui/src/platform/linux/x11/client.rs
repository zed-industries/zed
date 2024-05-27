use std::cell::RefCell;
use std::ffi::OsString;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use calloop::generic::{FdWrapper, Generic};
use calloop::{channel, EventLoop, LoopHandle, RegistrationToken};

use collections::HashMap;
use copypasta::x11_clipboard::{Clipboard, Primary, X11ClipboardContext};
use copypasta::ClipboardProvider;
use parking_lot::Mutex;

use util::ResultExt;
use x11rb::connection::{Connection, RequestConnection};
use x11rb::cursor;
use x11rb::errors::ConnectionError;
use x11rb::protocol::randr::ConnectionExt as _;
use x11rb::protocol::xinput::{ConnectionExt, ScrollClass};
use x11rb::protocol::xkb::ConnectionExt as _;
use x11rb::protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt as _};
use x11rb::protocol::{randr, render, xinput, xkb, xproto, Event};
use x11rb::resource_manager::Database;
use x11rb::xcb_ffi::XCBConnection;
use xim::{x11rb::X11rbClient, Client};
use xim::{AHashMap, AttributeName, ClientHandler, InputStyle};
use xkbc::x11::ffi::{XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION};
use xkbcommon::xkb as xkbc;

use crate::platform::linux::LinuxClient;
use crate::platform::{LinuxCommon, PlatformWindow, WaylandClientState};
use crate::{
    modifiers_from_xinput_info, point, px, AnyWindowHandle, Bounds, CursorStyle, DisplayId,
    ForegroundExecutor, Keystroke, Modifiers, ModifiersChangedEvent, Pixels, PlatformDisplay,
    PlatformInput, Point, ScrollDelta, Size, TouchPhase, WindowAppearance, WindowParams, X11Window,
};

use super::{
    super::{open_uri_internal, SCROLL_LINES},
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
    refresh_event_token: RegistrationToken,
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
    pub(crate) loop_handle: LoopHandle<'static, X11Client>,
    pub(crate) event_loop: Option<calloop::EventLoop<'static, X11Client>>,

    pub(crate) last_click: Instant,
    pub(crate) last_location: Point<Pixels>,
    pub(crate) current_count: usize,

    pub(crate) scale_factor: f32,

    pub(crate) xcb_connection: Rc<XCBConnection>,
    pub(crate) x_root_index: usize,
    pub(crate) resource_database: Database,
    pub(crate) atoms: XcbAtoms,
    pub(crate) windows: HashMap<xproto::Window, WindowRef>,
    pub(crate) focused_window: Option<xproto::Window>,
    pub(crate) xkb: xkbc::State,
    pub(crate) ximc: Option<X11rbClient<Rc<XCBConnection>>>,
    pub(crate) xim_handler: Option<XimHandler>,

    pub(crate) compose_state: xkbc::compose::State,
    pub(crate) pre_edit_text: Option<String>,
    pub(crate) composing: bool,
    pub(crate) cursor_handle: cursor::Handle,
    pub(crate) cursor_styles: HashMap<xproto::Window, CursorStyle>,
    pub(crate) cursor_cache: HashMap<CursorStyle, xproto::Cursor>,

    pub(crate) scroll_class_data: Vec<xinput::DeviceClassDataScroll>,
    pub(crate) scroll_x: Option<f32>,
    pub(crate) scroll_y: Option<f32>,

    pub(crate) common: LinuxCommon,
    pub(crate) clipboard: X11ClipboardContext<Clipboard>,
    pub(crate) primary: X11ClipboardContext<Primary>,
}

#[derive(Clone)]
pub struct X11ClientStatePtr(pub Weak<RefCell<X11ClientState>>);

impl X11ClientStatePtr {
    pub fn drop_window(&self, x_window: u32) {
        let client = X11Client(self.0.upgrade().expect("client already dropped"));
        let mut state = client.0.borrow_mut();

        if let Some(window_ref) = state.windows.remove(&x_window) {
            state.loop_handle.remove(window_ref.refresh_event_token);
        }

        state.cursor_styles.remove(&x_window);

        if state.windows.is_empty() {
            state.common.signal.stop();
        }
    }
}

#[derive(Clone)]
pub(crate) struct X11Client(Rc<RefCell<X11ClientState>>);

impl X11Client {
    pub(crate) fn new() -> Self {
        let event_loop = EventLoop::try_new().unwrap();

        let (common, main_receiver) = LinuxCommon::new(event_loop.get_signal());

        let handle = event_loop.handle();

        handle.insert_source(main_receiver, |event, _, _: &mut X11Client| {
            if let calloop::channel::Event::Msg(runnable) = event {
                runnable.run();
            }
        });

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
        let compose_state = {
            let locale = std::env::var_os("LC_CTYPE").unwrap_or(OsString::from("C"));
            let table = xkbc::compose::Table::new_from_locale(
                &xkb_context,
                &locale,
                xkbc::compose::COMPILE_NO_FLAGS,
            )
            .log_err()
            .unwrap();
            xkbc::compose::State::new(&table, xkbc::compose::STATE_NO_FLAGS)
        };

        let screen = xcb_connection.setup().roots.get(x_root_index).unwrap();

        // Values from `Database::GET_RESOURCE_DATABASE`
        let resource_manager = xcb_connection
            .get_property(
                false,
                screen.root,
                xproto::AtomEnum::RESOURCE_MANAGER,
                xproto::AtomEnum::STRING,
                0,
                100_000_000,
            )
            .unwrap();
        let resource_manager = resource_manager.reply().unwrap();

        // todo(linux): read hostname
        let resource_database = Database::new_from_default(&resource_manager, "HOSTNAME".into());

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

        let clipboard = X11ClipboardContext::<Clipboard>::new().unwrap();
        let primary = X11ClipboardContext::<Primary>::new().unwrap();

        let xcb_connection = Rc::new(xcb_connection);

        let (xim_tx, xim_rx) = channel::channel::<XimCallbackEvent>();

        let ximc = X11rbClient::init(Rc::clone(&xcb_connection), x_root_index, None).ok();
        let xim_handler = if ximc.is_some() {
            Some(XimHandler::new(xim_tx))
        } else {
            None
        };

        // Safety: Safe if xcb::Connection always returns a valid fd
        let fd = unsafe { FdWrapper::new(Rc::clone(&xcb_connection)) };

        handle
            .insert_source(
                Generic::new_with_error::<EventHandlerError>(
                    fd,
                    calloop::Interest::READ,
                    calloop::Mode::Level,
                ),
                {
                    let xcb_connection = xcb_connection.clone();
                    move |_readiness, _, client| {
                        while let Some(event) = xcb_connection.poll_for_event()? {
                            let mut state = client.0.borrow_mut();
                            if state.ximc.is_none() || state.xim_handler.is_none() {
                                drop(state);
                                client.handle_event(event);
                                continue;
                            }
                            let mut ximc = state.ximc.take().unwrap();
                            let mut xim_handler = state.xim_handler.take().unwrap();
                            let xim_connected = xim_handler.connected;
                            drop(state);
                            let xim_filtered = match ximc.filter_event(&event, &mut xim_handler) {
                                Ok(handled) => handled,
                                Err(err) => {
                                    log::error!("XIMClientError: {}", err);
                                    false
                                }
                            };
                            let mut state = client.0.borrow_mut();
                            state.ximc = Some(ximc);
                            state.xim_handler = Some(xim_handler);
                            drop(state);
                            if xim_filtered {
                                continue;
                            }
                            if xim_connected {
                                client.xim_handle_event(event);
                            } else {
                                client.handle_event(event);
                            }
                        }
                        Ok(calloop::PostAction::Continue)
                    }
                },
            )
            .expect("Failed to initialize x11 event source");
        handle
            .insert_source(xim_rx, {
                move |chan_event, _, client| match chan_event {
                    channel::Event::Msg(xim_event) => {
                        match (xim_event) {
                            XimCallbackEvent::XimXEvent(event) => {
                                client.handle_event(event);
                            }
                            XimCallbackEvent::XimCommitEvent(window, text) => {
                                client.xim_handle_commit(window, text);
                            }
                            XimCallbackEvent::XimPreeditEvent(window, text) => {
                                client.xim_handle_preedit(window, text);
                            }
                        };
                    }
                    channel::Event::Closed => {
                        log::error!("XIM Event Sender dropped")
                    }
                }
            })
            .expect("Failed to initialize XIM event source");
        handle.insert_source(XDPEventSource::new(&common.background_executor), {
            move |event, _, client| match event {
                XDPEvent::WindowAppearance(appearance) => {
                    client.with_common(|common| common.appearance = appearance);
                    for (_, window) in &mut client.0.borrow_mut().windows {
                        window.window.set_appearance(appearance);
                    }
                }
            }
        });

        X11Client(Rc::new(RefCell::new(X11ClientState {
            event_loop: Some(event_loop),
            loop_handle: handle,
            common,
            last_click: Instant::now(),
            last_location: Point::new(px(0.0), px(0.0)),
            current_count: 0,
            scale_factor,

            xcb_connection,
            x_root_index,
            resource_database,
            atoms,
            windows: HashMap::default(),
            focused_window: None,
            xkb: xkb_state,
            ximc,
            xim_handler,

            compose_state: compose_state,
            pre_edit_text: None,
            composing: false,

            cursor_handle,
            cursor_styles: HashMap::default(),
            cursor_cache: HashMap::default(),

            scroll_class_data,
            scroll_x: None,
            scroll_y: None,

            clipboard,
            primary,
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
        ximc.create_ic(xim_handler.im_id, ic_attributes.build());
        state = self.0.borrow_mut();
        state.xim_handler = Some(xim_handler);
        state.ximc = Some(ximc);
    }

    pub fn disable_ime(&self) {
        let mut state = self.0.borrow_mut();
        state.composing = false;
        if let Some(mut ximc) = state.ximc.take() {
            let xim_handler = state.xim_handler.as_ref().unwrap();
            ximc.destroy_ic(xim_handler.im_id, xim_handler.ic_id);
            state.ximc = Some(ximc);
        }
    }

    fn get_window(&self, win: xproto::Window) -> Option<X11WindowStatePtr> {
        let state = self.0.borrow();
        state
            .windows
            .get(&win)
            .map(|window_reference| window_reference.window.clone())
    }

    fn handle_event(&self, event: Event) -> Option<()> {
        match event {
            Event::ClientMessage(event) => {
                let window = self.get_window(event.window)?;
                let [atom, ..] = event.data.as_data32();
                let mut state = self.0.borrow_mut();

                if atom == state.atoms.WM_DELETE_WINDOW {
                    // window "x" button clicked by user
                    if window.should_close() {
                        let window_ref = state.windows.remove(&event.window)?;
                        state.loop_handle.remove(window_ref.refresh_event_token);
                        // Rest of the close logic is handled in drop_window()
                    }
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
                state.compose_state.reset();
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
                let focused_window_id = state.focused_window?;
                drop(state);

                let focused_window = self.get_window(focused_window_id)?;
                focused_window.handle_input(PlatformInput::ModifiersChanged(
                    ModifiersChangedEvent { modifiers },
                ));
            }
            Event::KeyPress(event) => {
                let window = self.get_window(event.event)?;
                let mut state = self.0.borrow_mut();

                let modifiers = modifiers_from_state(event.state);
                let keystroke = {
                    let code = event.detail.into();
                    let mut keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                    state.xkb.update_key(code, xkbc::KeyDirection::Down);
                    let keysym = state.xkb.key_get_one_sym(code);
                    if keysym.is_modifier_key() {
                        return Some(());
                    }
                    state.compose_state.feed(keysym);
                    match state.compose_state.status() {
                        xkbc::Status::Composed => {
                            state.pre_edit_text.take();
                            keystroke.ime_key = state.compose_state.utf8();
                            keystroke.key =
                                xkbc::keysym_get_name(state.compose_state.keysym().unwrap());
                        }
                        xkbc::Status::Composing => {
                            state.pre_edit_text = state
                                .compose_state
                                .utf8()
                                .or(crate::Keystroke::underlying_dead_key(keysym));
                            let pre_edit = state.pre_edit_text.clone().unwrap_or(String::default());
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
                            state.compose_state.feed(keysym);
                        }
                        _ => {}
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
                    state.compose_state.reset();
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
                let state = self.0.borrow();
                let modifiers = modifiers_from_xinput_info(event.mods);
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
                let state = self.0.borrow();
                let pressed_button = pressed_button_from_mask(event.button_mask[0]);
                let position = point(
                    px(event.event_x as f32 / u16::MAX as f32 / state.scale_factor),
                    px(event.event_y as f32 / u16::MAX as f32 / state.scale_factor),
                );
                drop(state);
                let modifiers = modifiers_from_xinput_info(event.mods);

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
                                window.handle_input(PlatformInput::ScrollWheel(
                                    crate::ScrollWheelEvent {
                                        position,
                                        delta: ScrollDelta::Lines(Point::new(0.0, delta_scroll)),
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
            Event::XinputLeave(event) => {
                self.0.borrow_mut().scroll_x = None; // Set last scroll to `None` so that a large delta isn't created if scrolling is done outside the window (the valuator is global)
                self.0.borrow_mut().scroll_y = None;

                let window = self.get_window(event.event)?;
                let state = self.0.borrow();
                let pressed_button = pressed_button_from_mask(event.buttons[0]);
                let position = point(
                    px(event.event_x as f32 / u16::MAX as f32 / state.scale_factor),
                    px(event.event_y as f32 / u16::MAX as f32 / state.scale_factor),
                );
                let modifiers = modifiers_from_xinput_info(event.mods);
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
            ximc.set_ic_values(xim_handler.im_id, xim_handler.ic_id, ic_attributes);
        }
        let mut state = self.0.borrow_mut();
        state.ximc = Some(ximc);
        state.xim_handler = Some(xim_handler);
        drop(state);
        Some(())
    }
}

impl LinuxClient for X11Client {
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
                Some(Rc::new(X11Display::new(&state.xcb_connection, root_id)?)
                    as Rc<dyn PlatformDisplay>)
            })
            .collect()
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        let state = self.0.borrow();

        Some(Rc::new(
            X11Display::new(&state.xcb_connection, state.x_root_index)
                .expect("There should always be a root index"),
        ))
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        let state = self.0.borrow();

        Some(Rc::new(X11Display::new(
            &state.xcb_connection,
            id.0 as usize,
        )?))
    }

    fn open_window(
        &self,
        _handle: AnyWindowHandle,
        params: WindowParams,
    ) -> Box<dyn PlatformWindow> {
        let mut state = self.0.borrow_mut();
        let x_window = state.xcb_connection.generate_id().unwrap();

        let window = X11Window::new(
            X11ClientStatePtr(Rc::downgrade(&self.0)),
            state.common.foreground_executor.clone(),
            params,
            &state.xcb_connection,
            state.x_root_index,
            x_window,
            &state.atoms,
            state.scale_factor,
            state.common.appearance,
        );

        let screen_resources = state
            .xcb_connection
            .randr_get_screen_resources(x_window)
            .unwrap()
            .reply()
            .expect("Could not find available screens");

        let mode = screen_resources
            .crtcs
            .iter()
            .find_map(|crtc| {
                let crtc_info = state
                    .xcb_connection
                    .randr_get_crtc_info(*crtc, x11rb::CURRENT_TIME)
                    .ok()?
                    .reply()
                    .ok()?;

                screen_resources
                    .modes
                    .iter()
                    .find(|m| m.id == crtc_info.mode)
            })
            .expect("Unable to find screen refresh rate");

        let refresh_event_token = state
            .loop_handle
            .insert_source(calloop::timer::Timer::immediate(), {
                let refresh_duration = mode_refresh_rate(mode);
                move |mut instant, (), client| {
                    let state = client.0.borrow_mut();
                    state
                        .xcb_connection
                        .send_event(
                            false,
                            x_window,
                            xproto::EventMask::EXPOSURE,
                            xproto::ExposeEvent {
                                response_type: xproto::EXPOSE_EVENT,
                                sequence: 0,
                                window: x_window,
                                x: 0,
                                y: 0,
                                width: 0,
                                height: 0,
                                count: 1,
                            },
                        )
                        .unwrap();
                    let _ = state.xcb_connection.flush().unwrap();
                    // Take into account that some frames have been skipped
                    let now = Instant::now();
                    while instant < now {
                        instant += refresh_duration;
                    }
                    calloop::timer::TimeoutAction::ToInstant(instant)
                }
            })
            .expect("Failed to initialize refresh timer");

        let window_ref = WindowRef {
            window: window.0.clone(),
            refresh_event_token,
        };

        state.windows.insert(x_window, window_ref);
        Box::new(window)
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
        self.0.borrow_mut().primary.set_contents(item.text);
    }

    fn write_to_clipboard(&self, item: crate::ClipboardItem) {
        self.0.borrow_mut().clipboard.set_contents(item.text);
    }

    fn read_from_primary(&self) -> Option<crate::ClipboardItem> {
        self.0
            .borrow_mut()
            .primary
            .get_contents()
            .ok()
            .map(|text| crate::ClipboardItem {
                text,
                metadata: None,
            })
    }

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        self.0
            .borrow_mut()
            .clipboard
            .get_contents()
            .ok()
            .map(|text| crate::ClipboardItem {
                text,
                metadata: None,
            })
    }

    fn run(&self) {
        let mut event_loop = self
            .0
            .borrow_mut()
            .event_loop
            .take()
            .expect("App is already running");

        event_loop.run(None, &mut self.clone(), |_| {}).log_err();
    }
}

// Adatpted from:
// https://docs.rs/winit/0.29.11/src/winit/platform_impl/linux/x11/monitor.rs.html#103-111
pub fn mode_refresh_rate(mode: &randr::ModeInfo) -> Duration {
    let millihertz = mode.dot_clock as u64 * 1_000 / (mode.htotal as u64 * mode.vtotal as u64);
    let micros = 1_000_000_000 / millihertz;
    log::info!("Refreshing at {} micros", micros);
    Duration::from_micros(micros)
}

fn fp3232_to_f32(value: xinput::Fp3232) -> f32 {
    value.integral as f32 + value.frac as f32 / u32::MAX as f32
}
