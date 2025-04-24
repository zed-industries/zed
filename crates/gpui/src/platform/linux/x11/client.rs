use core::str;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashSet},
    ops::Deref,
    path::PathBuf,
    rc::{Rc, Weak},
    time::{Duration, Instant},
};

use anyhow::Context as _;
use calloop::{
    EventLoop, LoopHandle, RegistrationToken,
    generic::{FdWrapper, Generic},
};
use collections::HashMap;
use futures::channel::oneshot;
use http_client::Url;
use smallvec::SmallVec;
use util::ResultExt;

use x11rb::{
    connection::{Connection, RequestConnection},
    cursor,
    errors::ConnectionError,
    protocol::randr::ConnectionExt as _,
    protocol::xinput::ConnectionExt,
    protocol::xkb::ConnectionExt as _,
    protocol::xproto::{
        AtomEnum, ChangeWindowAttributesAux, ClientMessageData, ClientMessageEvent,
        ConnectionExt as _, EventMask, KeyPressEvent,
    },
    protocol::{Event, randr, render, xinput, xkb, xproto},
    resource_manager::Database,
    wrapper::ConnectionExt as _,
    xcb_ffi::XCBConnection,
};
use xim::{AttributeName, Client, InputStyle, x11rb::X11rbClient};
use xkbc::x11::ffi::{XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION};
use xkbcommon::xkb::{self as xkbc, LayoutIndex, ModMask, STATE_LAYOUT_EFFECTIVE};

use super::{
    ButtonOrScroll, ScrollDirection, button_or_scroll_from_event_detail, get_valuator_axis_index,
    modifiers_from_state, pressed_button_from_mask,
};
use super::{X11Display, X11WindowStatePtr, XcbAtoms};
use super::{XimCallbackEvent, XimHandler};

use crate::platform::{
    LinuxCommon, PlatformWindow,
    blade::BladeContext,
    linux::{
        LinuxClient, get_xkb_compose_state, is_within_click_distance, open_uri_internal,
        platform::{DOUBLE_CLICK_INTERVAL, SCROLL_LINES},
        reveal_path_internal,
        x11::clipboard2::Clipboard,
        xdg_desktop_portal::{Event as XDPEvent, XDPEventSource},
    },
    scap_screen_capture::scap_screen_sources,
};
use crate::{
    AnyWindowHandle, Bounds, ClipboardItem, CursorStyle, DisplayId, FileDropEvent, Keystroke,
    LinuxKeyboardLayout, Modifiers, ModifiersChangedEvent, MouseButton, Pixels, Platform,
    PlatformDisplay, PlatformInput, PlatformKeyboardLayout, Point, RequestFrameOptions,
    ScaledPixels, ScreenCaptureSource, ScrollDelta, Size, TouchPhase, WindowParams, X11Window,
    modifiers_from_xinput_info, point, px,
};

/// Value for DeviceId parameters which selects all devices.
pub(crate) const XINPUT_ALL_DEVICES: xinput::DeviceId = 0;

/// Value for DeviceId parameters which selects all device groups. Events that
/// occur within the group are emitted by the group itself.
///
/// In XInput 2's interface, these are referred to as "master devices", but that
/// terminology is both archaic and unclear.
pub(crate) const XINPUT_ALL_DEVICE_GROUPS: xinput::DeviceId = 1;

pub(crate) struct WindowRef {
    window: X11WindowStatePtr,
    refresh_event_token: RegistrationToken,
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

#[derive(Debug, Default, Clone)]
struct XKBStateNotiy {
    depressed_layout: LayoutIndex,
    latched_layout: LayoutIndex,
    locked_layout: LayoutIndex,
}

#[derive(Debug, Default)]
pub struct Xdnd {
    other_window: xproto::Window,
    drag_type: u32,
    retrieved: bool,
    position: Point<Pixels>,
}

#[derive(Debug)]
struct PointerDeviceState {
    horizontal: ScrollAxisState,
    vertical: ScrollAxisState,
}

#[derive(Debug, Default)]
struct ScrollAxisState {
    /// Valuator number for looking up this axis's scroll value.
    valuator_number: Option<u16>,
    /// Conversion factor from scroll units to lines.
    multiplier: f32,
    /// Last scroll value for calculating scroll delta.
    ///
    /// This gets set to `None` whenever it might be invalid - when devices change or when window focus changes.
    /// The logic errs on the side of invalidating this, since the consequence is just skipping the delta of one scroll event.
    /// The consequence of not invalidating it can be large invalid deltas, which are much more user visible.
    scroll_value: Option<f32>,
}

pub struct X11ClientState {
    pub(crate) loop_handle: LoopHandle<'static, X11Client>,
    pub(crate) event_loop: Option<calloop::EventLoop<'static, X11Client>>,

    pub(crate) last_click: Instant,
    pub(crate) last_mouse_button: Option<MouseButton>,
    pub(crate) last_location: Point<Pixels>,
    pub(crate) current_count: usize,

    gpu_context: BladeContext,

    pub(crate) scale_factor: f32,

    xkb_context: xkbc::Context,
    pub(crate) xcb_connection: Rc<XCBConnection>,
    xkb_device_id: i32,
    client_side_decorations_supported: bool,
    pub(crate) x_root_index: usize,
    pub(crate) _resource_database: Database,
    pub(crate) atoms: XcbAtoms,
    pub(crate) windows: HashMap<xproto::Window, WindowRef>,
    pub(crate) mouse_focused_window: Option<xproto::Window>,
    pub(crate) keyboard_focused_window: Option<xproto::Window>,
    pub(crate) xkb: xkbc::State,
    previous_xkb_state: XKBStateNotiy,
    pub(crate) ximc: Option<X11rbClient<Rc<XCBConnection>>>,
    pub(crate) xim_handler: Option<XimHandler>,
    pub modifiers: Modifiers,
    // TODO: Can the other updates to `modifiers` be removed so that this is unnecessary?
    pub last_modifiers_changed_event: Modifiers,

    pub(crate) compose_state: Option<xkbc::compose::State>,
    pub(crate) pre_edit_text: Option<String>,
    pub(crate) composing: bool,
    pub(crate) pre_key_char_down: Option<Keystroke>,
    pub(crate) cursor_handle: cursor::Handle,
    pub(crate) cursor_styles: HashMap<xproto::Window, CursorStyle>,
    pub(crate) cursor_cache: HashMap<CursorStyle, xproto::Cursor>,

    pointer_device_states: BTreeMap<xinput::DeviceId, PointerDeviceState>,

    pub(crate) common: LinuxCommon,
    pub(crate) clipboard: Clipboard,
    pub(crate) clipboard_item: Option<ClipboardItem>,
    pub(crate) xdnd_state: Xdnd,
}

#[derive(Clone)]
pub struct X11ClientStatePtr(pub Weak<RefCell<X11ClientState>>);

impl X11ClientStatePtr {
    fn get_client(&self) -> X11Client {
        X11Client(self.0.upgrade().expect("client already dropped"))
    }

    pub fn drop_window(&self, x_window: u32) {
        let client = self.get_client();
        let mut state = client.0.borrow_mut();

        if let Some(window_ref) = state.windows.remove(&x_window) {
            state.loop_handle.remove(window_ref.refresh_event_token);
        }
        if state.mouse_focused_window == Some(x_window) {
            state.mouse_focused_window = None;
        }
        if state.keyboard_focused_window == Some(x_window) {
            state.keyboard_focused_window = None;
        }
        state.cursor_styles.remove(&x_window);

        if state.windows.is_empty() {
            state.common.signal.stop();
        }
    }

    pub fn update_ime_position(&self, bounds: Bounds<ScaledPixels>) {
        let client = self.get_client();
        let mut state = client.0.borrow_mut();
        if state.composing || state.ximc.is_none() {
            return;
        }

        let mut ximc = state.ximc.take().unwrap();
        let xim_handler = state.xim_handler.take().unwrap();
        let ic_attributes = ximc
            .build_ic_attributes()
            .push(
                xim::AttributeName::InputStyle,
                xim::InputStyle::PREEDIT_CALLBACKS,
            )
            .push(xim::AttributeName::ClientWindow, xim_handler.window)
            .push(xim::AttributeName::FocusWindow, xim_handler.window)
            .nested_list(xim::AttributeName::PreeditAttributes, |b| {
                b.push(
                    xim::AttributeName::SpotLocation,
                    xim::Point {
                        x: u32::from(bounds.origin.x + bounds.size.width) as i16,
                        y: u32::from(bounds.origin.y + bounds.size.height) as i16,
                    },
                );
            })
            .build();
        let _ = ximc
            .set_ic_values(xim_handler.im_id, xim_handler.ic_id, ic_attributes)
            .log_err();
        state.ximc = Some(ximc);
        state.xim_handler = Some(xim_handler);
    }
}

#[derive(Clone)]
pub(crate) struct X11Client(Rc<RefCell<X11ClientState>>);

impl X11Client {
    pub(crate) fn new() -> Self {
        let event_loop = EventLoop::try_new().unwrap();

        let (common, main_receiver) = LinuxCommon::new(event_loop.get_signal());

        let handle = event_loop.handle();

        handle
            .insert_source(main_receiver, {
                let handle = handle.clone();
                move |event, _, _: &mut X11Client| {
                    if let calloop::channel::Event::Msg(runnable) = event {
                        // Insert the runnables as idle callbacks, so we make sure that user-input and X11
                        // events have higher priority and runnables are only worked off after the event
                        // callbacks.
                        handle.insert_idle(|_| {
                            runnable.run();
                        });
                    }
                }
            })
            .unwrap();

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

        // Announce to X server that XInput up to 2.1 is supported. To increase this to 2.2 and
        // beyond, support for touch events would need to be added.
        let xinput_version = xcb_connection
            .xinput_xi_query_version(2, 1)
            .unwrap()
            .reply()
            .unwrap();
        // XInput 1.x is not supported.
        assert!(
            xinput_version.major_version >= 2,
            "XInput version >= 2 required."
        );

        let pointer_device_states =
            get_new_pointer_device_states(&xcb_connection, &BTreeMap::new());

        let atoms = XcbAtoms::new(&xcb_connection).unwrap().reply().unwrap();

        let root = xcb_connection.setup().roots[0].root;
        let compositor_present = check_compositor_present(&xcb_connection, root);
        let gtk_frame_extents_supported =
            check_gtk_frame_extents_supported(&xcb_connection, &atoms, root);
        let client_side_decorations_supported = compositor_present && gtk_frame_extents_supported;
        log::info!(
            "x11: compositor present: {}, gtk_frame_extents_supported: {}",
            compositor_present,
            gtk_frame_extents_supported
        );

        let xkb = xcb_connection
            .xkb_use_extension(XKB_X11_MIN_MAJOR_XKB_VERSION, XKB_X11_MIN_MINOR_XKB_VERSION)
            .unwrap()
            .reply()
            .unwrap();

        let events = xkb::EventType::STATE_NOTIFY
            | xkb::EventType::MAP_NOTIFY
            | xkb::EventType::NEW_KEYBOARD_NOTIFY;
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
        let xkb_device_id = xkbc::x11::get_core_keyboard_device_id(&xcb_connection);
        let xkb_state = {
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

        let gpu_context = BladeContext::new().expect("Unable to init GPU context");

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

        let clipboard = Clipboard::new().unwrap();

        let xcb_connection = Rc::new(xcb_connection);

        let ximc = X11rbClient::init(Rc::clone(&xcb_connection), x_root_index, None).ok();
        let xim_handler = if ximc.is_some() {
            Some(XimHandler::new())
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
                        client.process_x11_events(&xcb_connection)?;
                        Ok(calloop::PostAction::Continue)
                    }
                },
            )
            .expect("Failed to initialize x11 event source");

        handle
            .insert_source(XDPEventSource::new(&common.background_executor), {
                move |event, _, client| match event {
                    XDPEvent::WindowAppearance(appearance) => {
                        client.with_common(|common| common.appearance = appearance);
                        for (_, window) in &mut client.0.borrow_mut().windows {
                            window.window.set_appearance(appearance);
                        }
                    }
                    XDPEvent::CursorTheme(_) | XDPEvent::CursorSize(_) => {
                        // noop, X11 manages this for us.
                    }
                }
            })
            .unwrap();

        X11Client(Rc::new(RefCell::new(X11ClientState {
            modifiers: Modifiers::default(),
            last_modifiers_changed_event: Modifiers::default(),
            event_loop: Some(event_loop),
            loop_handle: handle,
            common,
            last_click: Instant::now(),
            last_mouse_button: None,
            last_location: Point::new(px(0.0), px(0.0)),
            current_count: 0,
            gpu_context,
            scale_factor,

            xkb_context,
            xcb_connection,
            xkb_device_id,
            client_side_decorations_supported,
            x_root_index,
            _resource_database: resource_database,
            atoms,
            windows: HashMap::default(),
            mouse_focused_window: None,
            keyboard_focused_window: None,
            xkb: xkb_state,
            previous_xkb_state: XKBStateNotiy::default(),
            ximc,
            xim_handler,

            compose_state,
            pre_edit_text: None,
            pre_key_char_down: None,
            composing: false,

            cursor_handle,
            cursor_styles: HashMap::default(),
            cursor_cache: HashMap::default(),

            pointer_device_states,

            clipboard,
            clipboard_item: None,
            xdnd_state: Xdnd::default(),
        })))
    }

    pub fn process_x11_events(
        &self,
        xcb_connection: &XCBConnection,
    ) -> Result<(), EventHandlerError> {
        loop {
            let mut events = Vec::new();
            let mut windows_to_refresh = HashSet::new();

            let mut last_key_release = None;
            let mut last_key_press: Option<KeyPressEvent> = None;

            loop {
                match xcb_connection.poll_for_event() {
                    Ok(Some(event)) => {
                        match event {
                            Event::Expose(expose_event) => {
                                windows_to_refresh.insert(expose_event.window);
                            }
                            Event::KeyRelease(_) => {
                                last_key_release = Some(event);
                            }
                            Event::KeyPress(key_press) => {
                                if let Some(last_press) = last_key_press.as_ref() {
                                    if last_press.detail == key_press.detail {
                                        continue;
                                    }
                                }

                                if let Some(Event::KeyRelease(key_release)) =
                                    last_key_release.take()
                                {
                                    // We ignore that last KeyRelease if it's too close to this KeyPress,
                                    // suggesting that it's auto-generated by X11 as a key-repeat event.
                                    if key_release.detail != key_press.detail
                                        || key_press.time.saturating_sub(key_release.time) > 20
                                    {
                                        events.push(Event::KeyRelease(key_release));
                                    }
                                }
                                events.push(Event::KeyPress(key_press));
                                last_key_press = Some(key_press);
                            }
                            _ => {
                                if let Some(release_event) = last_key_release.take() {
                                    events.push(release_event);
                                }
                                events.push(event);
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

            if events.is_empty() && windows_to_refresh.is_empty() {
                break;
            }

            for window in windows_to_refresh.into_iter() {
                if let Some(window) = self.get_window(window) {
                    window.refresh(RequestFrameOptions {
                        require_presentation: true,
                    });
                }
            }

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
                drop(state);

                if let Some(event) = xim_callback_event {
                    self.handle_xim_callback_event(event);
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
        Ok(())
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
            .push(AttributeName::InputStyle, InputStyle::PREEDIT_CALLBACKS)
            .push(AttributeName::ClientWindow, xim_handler.window)
            .push(AttributeName::FocusWindow, xim_handler.window);

        let window_id = state.keyboard_focused_window;
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

    pub fn reset_ime(&self) {
        let mut state = self.0.borrow_mut();
        state.composing = false;
        if let Some(mut ximc) = state.ximc.take() {
            let xim_handler = state.xim_handler.as_ref().unwrap();
            ximc.reset_ic(xim_handler.im_id, xim_handler.ic_id).ok();
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

    fn handle_event(&self, event: Event) -> Option<()> {
        match event {
            Event::ClientMessage(event) => {
                let window = self.get_window(event.window)?;
                let [atom, arg1, arg2, arg3, arg4] = event.data.as_data32();
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

                if event.type_ == state.atoms.XdndEnter {
                    state.xdnd_state.other_window = atom;
                    if (arg1 & 0x1) == 0x1 {
                        state.xdnd_state.drag_type = xdnd_get_supported_atom(
                            &state.xcb_connection,
                            &state.atoms,
                            state.xdnd_state.other_window,
                        );
                    } else {
                        if let Some(atom) = [arg2, arg3, arg4]
                            .into_iter()
                            .find(|atom| xdnd_is_atom_supported(*atom, &state.atoms))
                        {
                            state.xdnd_state.drag_type = atom;
                        }
                    }
                } else if event.type_ == state.atoms.XdndLeave {
                    let position = state.xdnd_state.position;
                    drop(state);
                    window
                        .handle_input(PlatformInput::FileDrop(FileDropEvent::Pending { position }));
                    window.handle_input(PlatformInput::FileDrop(FileDropEvent::Exited {}));
                    self.0.borrow_mut().xdnd_state = Xdnd::default();
                } else if event.type_ == state.atoms.XdndPosition {
                    if let Ok(pos) = state
                        .xcb_connection
                        .query_pointer(event.window)
                        .unwrap()
                        .reply()
                    {
                        state.xdnd_state.position =
                            Point::new(Pixels(pos.win_x as f32), Pixels(pos.win_y as f32));
                    }
                    if !state.xdnd_state.retrieved {
                        state
                            .xcb_connection
                            .convert_selection(
                                event.window,
                                state.atoms.XdndSelection,
                                state.xdnd_state.drag_type,
                                state.atoms.XDND_DATA,
                                arg3,
                            )
                            .unwrap();
                    }
                    xdnd_send_status(
                        &state.xcb_connection,
                        &state.atoms,
                        event.window,
                        state.xdnd_state.other_window,
                        arg4,
                    );
                    let position = state.xdnd_state.position;
                    drop(state);
                    window
                        .handle_input(PlatformInput::FileDrop(FileDropEvent::Pending { position }));
                } else if event.type_ == state.atoms.XdndDrop {
                    xdnd_send_finished(
                        &state.xcb_connection,
                        &state.atoms,
                        event.window,
                        state.xdnd_state.other_window,
                    );
                    let position = state.xdnd_state.position;
                    drop(state);
                    window
                        .handle_input(PlatformInput::FileDrop(FileDropEvent::Submit { position }));
                    self.0.borrow_mut().xdnd_state = Xdnd::default();
                }
            }
            Event::SelectionNotify(event) => {
                let window = self.get_window(event.requestor)?;
                let mut state = self.0.borrow_mut();
                let property = state.xcb_connection.get_property(
                    false,
                    event.requestor,
                    state.atoms.XDND_DATA,
                    AtomEnum::ANY,
                    0,
                    1024,
                );
                if property.as_ref().log_err().is_none() {
                    return Some(());
                }
                if let Ok(reply) = property.unwrap().reply() {
                    match str::from_utf8(&reply.value) {
                        Ok(file_list) => {
                            let paths: SmallVec<[_; 2]> = file_list
                                .lines()
                                .filter_map(|path| Url::parse(path).log_err())
                                .filter_map(|url| url.to_file_path().log_err())
                                .collect();
                            let input = PlatformInput::FileDrop(FileDropEvent::Entered {
                                position: state.xdnd_state.position,
                                paths: crate::ExternalPaths(paths),
                            });
                            drop(state);
                            window.handle_input(input);
                            self.0.borrow_mut().xdnd_state.retrieved = true;
                        }
                        Err(_) => {}
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
                window.configure(bounds).unwrap();
            }
            Event::PropertyNotify(event) => {
                let window = self.get_window(event.window)?;
                window.property_notify(event).unwrap();
            }
            Event::FocusIn(event) => {
                let window = self.get_window(event.event)?;
                window.set_active(true);
                let mut state = self.0.borrow_mut();
                state.keyboard_focused_window = Some(event.event);
                if let Some(handler) = state.xim_handler.as_mut() {
                    handler.window = event.event;
                }
                drop(state);
                self.enable_ime();
            }
            Event::FocusOut(event) => {
                let window = self.get_window(event.event)?;
                window.set_active(false);
                let mut state = self.0.borrow_mut();
                state.keyboard_focused_window = None;
                if let Some(compose_state) = state.compose_state.as_mut() {
                    compose_state.reset();
                }
                state.pre_edit_text.take();
                drop(state);
                self.reset_ime();
                window.handle_ime_delete();
            }
            Event::XkbNewKeyboardNotify(_) | Event::MapNotify(_) => {
                let mut state = self.0.borrow_mut();
                let xkb_state = {
                    let xkb_keymap = xkbc::x11::keymap_new_from_device(
                        &state.xkb_context,
                        &state.xcb_connection,
                        state.xkb_device_id,
                        xkbc::KEYMAP_COMPILE_NO_FLAGS,
                    );
                    xkbc::x11::state_new_from_device(
                        &xkb_keymap,
                        &state.xcb_connection,
                        state.xkb_device_id,
                    )
                };
                state.xkb = xkb_state;
            }
            Event::XkbStateNotify(event) => {
                let mut state = self.0.borrow_mut();
                let old_layout = state.xkb.serialize_layout(STATE_LAYOUT_EFFECTIVE);
                let new_layout = u32::from(event.group);
                state.xkb.update_mask(
                    event.base_mods.into(),
                    event.latched_mods.into(),
                    event.locked_mods.into(),
                    event.base_group as u32,
                    event.latched_group as u32,
                    event.locked_group.into(),
                );
                state.previous_xkb_state = XKBStateNotiy {
                    depressed_layout: event.base_group as u32,
                    latched_layout: event.latched_group as u32,
                    locked_layout: event.locked_group.into(),
                };

                if new_layout != old_layout {
                    if let Some(mut callback) = state.common.callbacks.keyboard_layout_change.take()
                    {
                        drop(state);
                        callback();
                        state = self.0.borrow_mut();
                        state.common.callbacks.keyboard_layout_change = Some(callback);
                    }
                }

                let modifiers = Modifiers::from_xkb(&state.xkb);
                if state.last_modifiers_changed_event == modifiers {
                    drop(state);
                } else {
                    let focused_window_id = state.keyboard_focused_window?;
                    state.modifiers = modifiers;
                    state.last_modifiers_changed_event = modifiers;
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
                state.pre_key_char_down.take();
                let keystroke = {
                    let code = event.detail.into();
                    let xkb_state = state.previous_xkb_state.clone();
                    state.xkb.update_mask(
                        event.state.bits() as ModMask,
                        0,
                        0,
                        xkb_state.depressed_layout,
                        xkb_state.latched_layout,
                        xkb_state.locked_layout,
                    );
                    let mut keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
                    let keysym = state.xkb.key_get_one_sym(code);
                    if keysym.is_modifier_key() {
                        return Some(());
                    }
                    if let Some(mut compose_state) = state.compose_state.take() {
                        compose_state.feed(keysym);
                        match compose_state.status() {
                            xkbc::Status::Composed => {
                                state.pre_edit_text.take();
                                keystroke.key_char = compose_state.utf8();
                                if let Some(keysym) = compose_state.keysym() {
                                    keystroke.key = xkbc::keysym_get_name(keysym);
                                }
                            }
                            xkbc::Status::Composing => {
                                keystroke.key_char = None;
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
                    let xkb_state = state.previous_xkb_state.clone();
                    state.xkb.update_mask(
                        event.state.bits() as ModMask,
                        0,
                        0,
                        xkb_state.depressed_layout,
                        xkb_state.latched_layout,
                        xkb_state.locked_layout,
                    );
                    let keystroke = crate::Keystroke::from_xkb(&state.xkb, modifiers, code);
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
                    self.reset_ime();
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
                match button_or_scroll_from_event_detail(event.detail) {
                    Some(ButtonOrScroll::Button(button)) => {
                        let click_elapsed = state.last_click.elapsed();
                        if click_elapsed < DOUBLE_CLICK_INTERVAL
                            && state
                                .last_mouse_button
                                .is_some_and(|prev_button| prev_button == button)
                            && is_within_click_distance(state.last_location, position)
                        {
                            state.current_count += 1;
                        } else {
                            state.current_count = 1;
                        }

                        state.last_click = Instant::now();
                        state.last_mouse_button = Some(button);
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
                    }
                    Some(ButtonOrScroll::Scroll(direction)) => {
                        drop(state);
                        // Emulated scroll button presses are sent simultaneously with smooth scrolling XinputMotion events.
                        // Since handling those events does the scrolling, they are skipped here.
                        if !event
                            .flags
                            .contains(xinput::PointerEventFlags::POINTER_EMULATED)
                        {
                            let scroll_delta = match direction {
                                ScrollDirection::Up => Point::new(0.0, SCROLL_LINES),
                                ScrollDirection::Down => Point::new(0.0, -SCROLL_LINES),
                                ScrollDirection::Left => Point::new(SCROLL_LINES, 0.0),
                                ScrollDirection::Right => Point::new(-SCROLL_LINES, 0.0),
                            };
                            window.handle_input(PlatformInput::ScrollWheel(
                                make_scroll_wheel_event(position, scroll_delta, modifiers),
                            ));
                        }
                    }
                    None => {
                        log::error!("Unknown x11 button: {}", event.detail);
                    }
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
                match button_or_scroll_from_event_detail(event.detail) {
                    Some(ButtonOrScroll::Button(button)) => {
                        let click_count = state.current_count;
                        drop(state);
                        window.handle_input(PlatformInput::MouseUp(crate::MouseUpEvent {
                            button,
                            position,
                            modifiers,
                            click_count,
                        }));
                    }
                    Some(ButtonOrScroll::Scroll(_)) => {}
                    None => {}
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

                if event.valuator_mask[0] & 3 != 0 {
                    window.handle_input(PlatformInput::MouseMove(crate::MouseMoveEvent {
                        position,
                        pressed_button,
                        modifiers,
                    }));
                }

                state = self.0.borrow_mut();
                if let Some(mut pointer) = state.pointer_device_states.get_mut(&event.sourceid) {
                    let scroll_delta = get_scroll_delta_and_update_state(&mut pointer, &event);
                    drop(state);
                    if let Some(scroll_delta) = scroll_delta {
                        window.handle_input(PlatformInput::ScrollWheel(make_scroll_wheel_event(
                            position,
                            scroll_delta,
                            modifiers,
                        )));
                    }
                }
            }
            Event::XinputEnter(event) if event.mode == xinput::NotifyMode::NORMAL => {
                let window = self.get_window(event.event)?;
                window.set_hovered(true);
                let mut state = self.0.borrow_mut();
                state.mouse_focused_window = Some(event.event);
            }
            Event::XinputLeave(event) if event.mode == xinput::NotifyMode::NORMAL => {
                let mut state = self.0.borrow_mut();

                // Set last scroll values to `None` so that a large delta isn't created if scrolling is done outside the window (the valuator is global)
                reset_all_pointer_device_scroll_positions(&mut state.pointer_device_states);
                state.mouse_focused_window = None;
                let pressed_button = pressed_button_from_mask(event.buttons[0]);
                let position = point(
                    px(event.event_x as f32 / u16::MAX as f32 / state.scale_factor),
                    px(event.event_y as f32 / u16::MAX as f32 / state.scale_factor),
                );
                let modifiers = modifiers_from_xinput_info(event.mods);
                state.modifiers = modifiers;
                drop(state);

                let window = self.get_window(event.event)?;
                window.handle_input(PlatformInput::MouseExited(crate::MouseExitEvent {
                    pressed_button,
                    position,
                    modifiers,
                }));
                window.set_hovered(false);
            }
            Event::XinputHierarchy(event) => {
                let mut state = self.0.borrow_mut();
                // Temporarily use `state.pointer_device_states` to only store pointers that still have valid scroll values.
                // Any change to a device invalidates its scroll values.
                for info in event.infos {
                    if is_pointer_device(info.type_) {
                        state.pointer_device_states.remove(&info.deviceid);
                    }
                }
                state.pointer_device_states = get_new_pointer_device_states(
                    &state.xcb_connection,
                    &state.pointer_device_states,
                );
            }
            Event::XinputDeviceChanged(event) => {
                let mut state = self.0.borrow_mut();
                if let Some(mut pointer) = state.pointer_device_states.get_mut(&event.sourceid) {
                    reset_pointer_device_scroll_positions(&mut pointer);
                }
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
                state.pre_key_char_down = Some(Keystroke::from_xkb(
                    &state.xkb,
                    state.modifiers,
                    event.detail.into(),
                ));
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
        let keystroke = state.pre_key_char_down.take();
        state.composing = false;
        drop(state);
        if let Some(mut keystroke) = keystroke {
            keystroke.key_char = Some(text.clone());
            window.handle_input(PlatformInput::KeyDown(crate::KeyDownEvent {
                keystroke,
                is_held: false,
            }));
        }

        Some(())
    }

    fn xim_handle_preedit(&self, window: xproto::Window, text: String) -> Option<()> {
        let window = self.get_window(window).unwrap();

        let mut state = self.0.borrow_mut();
        let mut ximc = state.ximc.take().unwrap();
        let mut xim_handler = state.xim_handler.take().unwrap();
        state.composing = !text.is_empty();
        drop(state);
        window.handle_ime_preedit(text);

        if let Some(area) = window.get_ime_area() {
            let ic_attributes = ximc
                .build_ic_attributes()
                .push(
                    xim::AttributeName::InputStyle,
                    xim::InputStyle::PREEDIT_CALLBACKS,
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

impl LinuxClient for X11Client {
    fn compositor_name(&self) -> &'static str {
        "X11"
    }

    fn with_common<R>(&self, f: impl FnOnce(&mut LinuxCommon) -> R) -> R {
        f(&mut self.0.borrow_mut().common)
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        let state = self.0.borrow();
        let layout_idx = state.xkb.serialize_layout(STATE_LAYOUT_EFFECTIVE);
        Box::new(LinuxKeyboardLayout::new(
            state
                .xkb
                .get_keymap()
                .layout_get_name(layout_idx)
                .to_string(),
        ))
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        let state = self.0.borrow();
        let setup = state.xcb_connection.setup();
        setup
            .roots
            .iter()
            .enumerate()
            .filter_map(|(root_id, _)| {
                Some(Rc::new(
                    X11Display::new(&state.xcb_connection, state.scale_factor, root_id).ok()?,
                ) as Rc<dyn PlatformDisplay>)
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

        Some(Rc::new(
            X11Display::new(&state.xcb_connection, state.scale_factor, id.0 as usize).ok()?,
        ))
    }

    fn is_screen_capture_supported(&self) -> bool {
        true
    }

    fn screen_capture_sources(
        &self,
    ) -> oneshot::Receiver<anyhow::Result<Vec<Box<dyn ScreenCaptureSource>>>> {
        scap_screen_sources(&self.0.borrow().common.foreground_executor)
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
            &state.gpu_context,
            params,
            &state.xcb_connection,
            state.client_side_decorations_supported,
            state.x_root_index,
            x_window,
            &state.atoms,
            state.scale_factor,
            state.common.appearance,
        )?;
        state
            .xcb_connection
            .change_property32(
                xproto::PropMode::REPLACE,
                x_window,
                state.atoms.XdndAware,
                state.atoms.XA_ATOM,
                &[5],
            )
            .unwrap();

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
                    let xcb_connection = {
                        let state = client.0.borrow_mut();
                        let xcb_connection = state.xcb_connection.clone();
                        if let Some(window) = state.windows.get(&x_window) {
                            let window = window.window.clone();
                            drop(state);
                            window.refresh(Default::default());
                        }
                        xcb_connection
                    };
                    client.process_x11_events(&xcb_connection).log_err();

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
        Ok(Box::new(window))
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        let mut state = self.0.borrow_mut();
        let Some(focused_window) = state.mouse_focused_window else {
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
                let Some(cursor) = (match style {
                    CursorStyle::None => create_invisible_cursor(&state.xcb_connection).log_err(),
                    _ => state
                        .cursor_handle
                        .load_cursor(&state.xcb_connection, &style.to_icon_name())
                        .log_err(),
                }) else {
                    return;
                };

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
            .anyhow()
            .and_then(|cookie| cookie.check().anyhow())
            .context("setting cursor style")
            .log_err();
    }

    fn open_uri(&self, uri: &str) {
        #[cfg(any(feature = "wayland", feature = "x11"))]
        open_uri_internal(self.background_executor(), uri, None);
    }

    fn reveal_path(&self, path: PathBuf) {
        #[cfg(any(feature = "x11", feature = "wayland"))]
        reveal_path_internal(self.background_executor(), path, None);
    }

    fn write_to_primary(&self, item: crate::ClipboardItem) {
        let state = self.0.borrow_mut();
        state
            .clipboard
            .set_text(
                std::borrow::Cow::Owned(item.text().unwrap_or_default()),
                super::clipboard2::LinuxClipboardKind::Primary,
                super::clipboard2::WaitConfig::None,
            )
            .ok();
    }

    fn write_to_clipboard(&self, item: crate::ClipboardItem) {
        let mut state = self.0.borrow_mut();
        state
            .clipboard
            .set_text(
                std::borrow::Cow::Owned(item.text().unwrap_or_default()),
                super::clipboard2::LinuxClipboardKind::Clipboard,
                super::clipboard2::WaitConfig::None,
            )
            .ok();
        state.clipboard_item.replace(item);
    }

    fn read_from_primary(&self) -> Option<crate::ClipboardItem> {
        let state = self.0.borrow_mut();
        if let Ok(image) = state
            .clipboard
            .get_image(super::clipboard2::LinuxClipboardKind::Primary)
        {
            return Some(crate::ClipboardItem::new_image(&image));
        }
        return state
            .clipboard
            .get_text(super::clipboard2::LinuxClipboardKind::Primary)
            .map(|text| crate::ClipboardItem::new_string(text))
            .ok();
    }

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        let state = self.0.borrow_mut();
        // todo! support
        // if the last copy was from this app, return our cached item
        // which has metadata attached.
        // if state
        //     .clipboard
        //     .clipboard
        //     .setter
        //     .connection
        //     .get_selection_owner(state.clipboard.setter.atoms.clipboard)
        //     .ok()
        //     .and_then(|r| r.reply().ok())
        //     .map(|reply| reply.owner == state.clipboard.setter.window)
        //     .unwrap_or(false)
        // {
        //     return state.clipboard_item.clone();
        // }
        if let Ok(image) = state
            .clipboard
            .get_image(super::clipboard2::LinuxClipboardKind::Clipboard)
        {
            return Some(crate::ClipboardItem::new_image(&image));
        }
        return state
            .clipboard
            .get_text(super::clipboard2::LinuxClipboardKind::Primary)
            .map(|text| crate::ClipboardItem::new_string(text))
            .ok();
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

    fn active_window(&self) -> Option<AnyWindowHandle> {
        let state = self.0.borrow();
        state.keyboard_focused_window.and_then(|focused_window| {
            state
                .windows
                .get(&focused_window)
                .map(|window| window.handle())
        })
    }

    fn window_stack(&self) -> Option<Vec<AnyWindowHandle>> {
        let state = self.0.borrow();
        let root = state.xcb_connection.setup().roots[state.x_root_index].root;

        let reply = state
            .xcb_connection
            .get_property(
                false,
                root,
                state.atoms._NET_CLIENT_LIST_STACKING,
                xproto::AtomEnum::WINDOW,
                0,
                u32::MAX,
            )
            .ok()?
            .reply()
            .ok()?;

        let window_ids = reply
            .value
            .chunks_exact(4)
            .map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<xproto::Window>>();

        let mut handles = Vec::new();

        // We need to reverse, since _NET_CLIENT_LIST_STACKING has
        // a back-to-front order.
        // See: https://specifications.freedesktop.org/wm-spec/1.3/ar01s03.html
        for window_ref in window_ids
            .iter()
            .rev()
            .filter_map(|&win| state.windows.get(&win))
        {
            if !window_ref.window.state.borrow().destroyed {
                handles.push(window_ref.handle());
            }
        }

        Some(handles)
    }
}

// Adapted from:
// https://docs.rs/winit/0.29.11/src/winit/platform_impl/linux/x11/monitor.rs.html#103-111
pub fn mode_refresh_rate(mode: &randr::ModeInfo) -> Duration {
    if mode.dot_clock == 0 || mode.htotal == 0 || mode.vtotal == 0 {
        return Duration::from_millis(16);
    }

    let millihertz = mode.dot_clock as u64 * 1_000 / (mode.htotal as u64 * mode.vtotal as u64);
    let micros = 1_000_000_000 / millihertz;
    log::info!("Refreshing at {} micros", micros);
    Duration::from_micros(micros)
}

fn fp3232_to_f32(value: xinput::Fp3232) -> f32 {
    value.integral as f32 + value.frac as f32 / u32::MAX as f32
}

fn check_compositor_present(xcb_connection: &XCBConnection, root: u32) -> bool {
    // Method 1: Check for _NET_WM_CM_S{root}
    let atom_name = format!("_NET_WM_CM_S{}", root);
    let atom = xcb_connection
        .intern_atom(false, atom_name.as_bytes())
        .unwrap()
        .reply()
        .map(|reply| reply.atom)
        .unwrap_or(0);

    let method1 = if atom != 0 {
        xcb_connection
            .get_selection_owner(atom)
            .unwrap()
            .reply()
            .map(|reply| reply.owner != 0)
            .unwrap_or(false)
    } else {
        false
    };

    // Method 2: Check for _NET_WM_CM_OWNER
    let atom_name = "_NET_WM_CM_OWNER";
    let atom = xcb_connection
        .intern_atom(false, atom_name.as_bytes())
        .unwrap()
        .reply()
        .map(|reply| reply.atom)
        .unwrap_or(0);

    let method2 = if atom != 0 {
        xcb_connection
            .get_property(false, root, atom, xproto::AtomEnum::WINDOW, 0, 1)
            .unwrap()
            .reply()
            .map(|reply| reply.value_len > 0)
            .unwrap_or(false)
    } else {
        false
    };

    // Method 3: Check for _NET_SUPPORTING_WM_CHECK
    let atom_name = "_NET_SUPPORTING_WM_CHECK";
    let atom = xcb_connection
        .intern_atom(false, atom_name.as_bytes())
        .unwrap()
        .reply()
        .map(|reply| reply.atom)
        .unwrap_or(0);

    let method3 = if atom != 0 {
        xcb_connection
            .get_property(false, root, atom, xproto::AtomEnum::WINDOW, 0, 1)
            .unwrap()
            .reply()
            .map(|reply| reply.value_len > 0)
            .unwrap_or(false)
    } else {
        false
    };

    // TODO: Remove this
    log::info!(
        "Compositor detection: _NET_WM_CM_S?={}, _NET_WM_CM_OWNER={}, _NET_SUPPORTING_WM_CHECK={}",
        method1,
        method2,
        method3
    );

    method1 || method2 || method3
}

fn check_gtk_frame_extents_supported(
    xcb_connection: &XCBConnection,
    atoms: &XcbAtoms,
    root: xproto::Window,
) -> bool {
    let supported_atoms = xcb_connection
        .get_property(
            false,
            root,
            atoms._NET_SUPPORTED,
            xproto::AtomEnum::ATOM,
            0,
            1024,
        )
        .unwrap()
        .reply()
        .map(|reply| {
            // Convert Vec<u8> to Vec<u32>
            reply
                .value
                .chunks_exact(4)
                .map(|chunk| u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect::<Vec<u32>>()
        })
        .unwrap_or_default();

    supported_atoms.contains(&atoms._GTK_FRAME_EXTENTS)
}

fn xdnd_is_atom_supported(atom: u32, atoms: &XcbAtoms) -> bool {
    return atom == atoms.TEXT
        || atom == atoms.STRING
        || atom == atoms.UTF8_STRING
        || atom == atoms.TEXT_PLAIN
        || atom == atoms.TEXT_PLAIN_UTF8
        || atom == atoms.TextUriList;
}

fn xdnd_get_supported_atom(
    xcb_connection: &XCBConnection,
    supported_atoms: &XcbAtoms,
    target: xproto::Window,
) -> u32 {
    let property = xcb_connection
        .get_property(
            false,
            target,
            supported_atoms.XdndTypeList,
            AtomEnum::ANY,
            0,
            1024,
        )
        .unwrap();
    if let Ok(reply) = property.reply() {
        if let Some(atoms) = reply.value32() {
            for atom in atoms {
                if xdnd_is_atom_supported(atom, &supported_atoms) {
                    return atom;
                }
            }
        }
    }
    return 0;
}

fn xdnd_send_finished(
    xcb_connection: &XCBConnection,
    atoms: &XcbAtoms,
    source: xproto::Window,
    target: xproto::Window,
) {
    let message = ClientMessageEvent {
        format: 32,
        window: target,
        type_: atoms.XdndFinished,
        data: ClientMessageData::from([source, 1, atoms.XdndActionCopy, 0, 0]),
        sequence: 0,
        response_type: xproto::CLIENT_MESSAGE_EVENT,
    };
    xcb_connection
        .send_event(false, target, EventMask::default(), message)
        .unwrap();
}

fn xdnd_send_status(
    xcb_connection: &XCBConnection,
    atoms: &XcbAtoms,
    source: xproto::Window,
    target: xproto::Window,
    action: u32,
) {
    let message = ClientMessageEvent {
        format: 32,
        window: target,
        type_: atoms.XdndStatus,
        data: ClientMessageData::from([source, 1, 0, 0, action]),
        sequence: 0,
        response_type: xproto::CLIENT_MESSAGE_EVENT,
    };
    xcb_connection
        .send_event(false, target, EventMask::default(), message)
        .unwrap();
}

/// Recomputes `pointer_device_states` by querying all pointer devices.
/// When a device is present in `scroll_values_to_preserve`, its value for `ScrollAxisState.scroll_value` is used.
fn get_new_pointer_device_states(
    xcb_connection: &XCBConnection,
    scroll_values_to_preserve: &BTreeMap<xinput::DeviceId, PointerDeviceState>,
) -> BTreeMap<xinput::DeviceId, PointerDeviceState> {
    let devices_query_result = xcb_connection
        .xinput_xi_query_device(XINPUT_ALL_DEVICES)
        .unwrap()
        .reply()
        .unwrap();

    let mut pointer_device_states = BTreeMap::new();
    pointer_device_states.extend(
        devices_query_result
            .infos
            .iter()
            .filter(|info| is_pointer_device(info.type_))
            .filter_map(|info| {
                let scroll_data = info
                    .classes
                    .iter()
                    .filter_map(|class| class.data.as_scroll())
                    .map(|class| *class)
                    .rev()
                    .collect::<Vec<_>>();
                let old_state = scroll_values_to_preserve.get(&info.deviceid);
                let old_horizontal = old_state.map(|state| &state.horizontal);
                let old_vertical = old_state.map(|state| &state.vertical);
                let horizontal = scroll_data
                    .iter()
                    .find(|data| data.scroll_type == xinput::ScrollType::HORIZONTAL)
                    .map(|data| scroll_data_to_axis_state(data, old_horizontal));
                let vertical = scroll_data
                    .iter()
                    .find(|data| data.scroll_type == xinput::ScrollType::VERTICAL)
                    .map(|data| scroll_data_to_axis_state(data, old_vertical));
                if horizontal.is_none() && vertical.is_none() {
                    None
                } else {
                    Some((
                        info.deviceid,
                        PointerDeviceState {
                            horizontal: horizontal.unwrap_or_else(Default::default),
                            vertical: vertical.unwrap_or_else(Default::default),
                        },
                    ))
                }
            }),
    );
    if pointer_device_states.is_empty() {
        log::error!("Found no xinput mouse pointers.");
    }
    return pointer_device_states;
}

/// Returns true if the device is a pointer device. Does not include pointer device groups.
fn is_pointer_device(type_: xinput::DeviceType) -> bool {
    type_ == xinput::DeviceType::SLAVE_POINTER
}

fn scroll_data_to_axis_state(
    data: &xinput::DeviceClassDataScroll,
    old_axis_state_with_valid_scroll_value: Option<&ScrollAxisState>,
) -> ScrollAxisState {
    ScrollAxisState {
        valuator_number: Some(data.number),
        multiplier: SCROLL_LINES / fp3232_to_f32(data.increment),
        scroll_value: old_axis_state_with_valid_scroll_value.and_then(|state| state.scroll_value),
    }
}

fn reset_all_pointer_device_scroll_positions(
    pointer_device_states: &mut BTreeMap<xinput::DeviceId, PointerDeviceState>,
) {
    pointer_device_states
        .iter_mut()
        .for_each(|(_, device_state)| reset_pointer_device_scroll_positions(device_state));
}

fn reset_pointer_device_scroll_positions(pointer: &mut PointerDeviceState) {
    pointer.horizontal.scroll_value = None;
    pointer.vertical.scroll_value = None;
}

/// Returns the scroll delta for a smooth scrolling motion event, or `None` if no scroll data is present.
fn get_scroll_delta_and_update_state(
    pointer: &mut PointerDeviceState,
    event: &xinput::MotionEvent,
) -> Option<Point<f32>> {
    let delta_x = get_axis_scroll_delta_and_update_state(event, &mut pointer.horizontal);
    let delta_y = get_axis_scroll_delta_and_update_state(event, &mut pointer.vertical);
    if delta_x.is_some() || delta_y.is_some() {
        Some(Point::new(delta_x.unwrap_or(0.0), delta_y.unwrap_or(0.0)))
    } else {
        None
    }
}

fn get_axis_scroll_delta_and_update_state(
    event: &xinput::MotionEvent,
    axis: &mut ScrollAxisState,
) -> Option<f32> {
    let axis_index = get_valuator_axis_index(&event.valuator_mask, axis.valuator_number?)?;
    if let Some(axis_value) = event.axisvalues.get(axis_index) {
        let new_scroll = fp3232_to_f32(*axis_value);
        let delta_scroll = axis
            .scroll_value
            .map(|old_scroll| (old_scroll - new_scroll) * axis.multiplier);
        axis.scroll_value = Some(new_scroll);
        delta_scroll
    } else {
        log::error!("Encountered invalid XInput valuator_mask, scrolling may not work properly.");
        None
    }
}

fn make_scroll_wheel_event(
    position: Point<Pixels>,
    scroll_delta: Point<f32>,
    modifiers: Modifiers,
) -> crate::ScrollWheelEvent {
    // When shift is held down, vertical scrolling turns into horizontal scrolling.
    let delta = if modifiers.shift {
        Point {
            x: scroll_delta.y,
            y: 0.0,
        }
    } else {
        scroll_delta
    };
    crate::ScrollWheelEvent {
        position,
        delta: ScrollDelta::Lines(delta),
        modifiers,
        touch_phase: TouchPhase::default(),
    }
}

fn create_invisible_cursor(
    connection: &XCBConnection,
) -> anyhow::Result<crate::platform::linux::x11::client::xproto::Cursor> {
    let empty_pixmap = connection.generate_id()?;
    let root = connection.setup().roots[0].root;
    connection.create_pixmap(1, empty_pixmap, root, 1, 1)?;

    let cursor = connection.generate_id()?;
    connection.create_cursor(cursor, empty_pixmap, empty_pixmap, 0, 0, 0, 0, 0, 0, 0, 0)?;

    connection.free_pixmap(empty_pixmap)?;

    connection.flush()?;
    Ok(cursor)
}
