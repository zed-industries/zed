use std::{cell::Cell, default::Default, rc::Rc};

use calloop::channel;

use x11rb::{
    connection::{Connection, RequestConnection},
    cookie::{CookieWithFds, VoidCookie},
    protocol::{xproto, Event},
    xcb_ffi::XCBConnection,
};
use xim::{
    x11rb::HasConnection, AHashMap, AttributeName, Client, ClientError, ClientHandler, InputStyle,
};

pub enum XimCallbackEvent {
    XimXEvent(x11rb::protocol::Event),
    XimPreeditEvent(xproto::Window, String),
    XimCommitEvent(xproto::Window, String),
}

pub struct XimHandler {
    pub im_id: u16,
    pub ic_id: u16,
    // pub xim_tx: channel::Sender<XimCallbackEvent>,
    pub connected: bool,
    pub window: xproto::Window,
    pub last_callback_event: Option<XimCallbackEvent>,
}

impl XimHandler {
    pub fn new() -> Self {
        Self {
            im_id: Default::default(),
            ic_id: Default::default(),
            // xim_tx,
            connected: false,
            window: Default::default(),
            last_callback_event: None,
        }
    }
}

impl<C: Client<XEvent = xproto::KeyPressEvent>> ClientHandler<C> for XimHandler {
    fn handle_connect(&mut self, client: &mut C) -> Result<(), ClientError> {
        client.open("C")
    }

    fn handle_open(&mut self, client: &mut C, input_method_id: u16) -> Result<(), ClientError> {
        self.im_id = input_method_id;

        client.get_im_values(input_method_id, &[AttributeName::QueryInputStyle])
    }

    fn handle_get_im_values(
        &mut self,
        client: &mut C,
        input_method_id: u16,
        _attributes: AHashMap<AttributeName, Vec<u8>>,
    ) -> Result<(), ClientError> {
        let ic_attributes = client
            .build_ic_attributes()
            .push(
                AttributeName::InputStyle,
                InputStyle::PREEDIT_CALLBACKS
                    | InputStyle::STATUS_NOTHING
                    | InputStyle::PREEDIT_NONE,
            )
            .push(AttributeName::ClientWindow, self.window)
            .push(AttributeName::FocusWindow, self.window)
            .build();
        client.create_ic(input_method_id, ic_attributes)
    }

    fn handle_create_ic(
        &mut self,
        _client: &mut C,
        _input_method_id: u16,
        input_context_id: u16,
    ) -> Result<(), ClientError> {
        self.connected = true;
        self.ic_id = input_context_id;
        Ok(())
    }

    fn handle_commit(
        &mut self,
        _client: &mut C,
        _input_method_id: u16,
        _input_context_id: u16,
        text: &str,
    ) -> Result<(), ClientError> {
        self.last_callback_event
            .replace(XimCallbackEvent::XimCommitEvent(
                self.window,
                String::from(text),
            ));
        // .ok();
        Ok(())
    }

    fn handle_forward_event(
        &mut self,
        _client: &mut C,
        _input_method_id: u16,
        _input_context_id: u16,
        _flag: xim::ForwardEventFlag,
        xev: C::XEvent,
    ) -> Result<(), ClientError> {
        match xev.response_type {
            x11rb::protocol::xproto::KEY_PRESS_EVENT => {
                // println!(
                //     "XimHandler. handle_forward_event(KeyPress). sequence: {}",
                //     xev.sequence
                // );
                self.last_callback_event
                    .replace(XimCallbackEvent::XimXEvent(Event::KeyPress(xev)));
            }
            x11rb::protocol::xproto::KEY_RELEASE_EVENT => {
                // println!(
                //     "XimHandler. handle_forward_event(KeyRelease), sequence: {}",
                //     xev.sequence
                // );
                self.last_callback_event
                    .replace(XimCallbackEvent::XimXEvent(Event::KeyRelease(xev)));
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_close(&mut self, client: &mut C, _input_method_id: u16) -> Result<(), ClientError> {
        client.disconnect()
    }

    fn handle_destroy_ic(
        &mut self,
        client: &mut C,
        input_method_id: u16,
        _input_context_id: u16,
    ) -> Result<(), ClientError> {
        client.close(input_method_id)
    }

    fn handle_preedit_draw(
        &mut self,
        _client: &mut C,
        _input_method_id: u16,
        _input_context_id: u16,
        _caret: i32,
        _chg_first: i32,
        _chg_len: i32,
        _status: xim::PreeditDrawStatus,
        preedit_string: &str,
        _feedbacks: Vec<xim::Feedback>,
    ) -> Result<(), ClientError> {
        // XIMReverse: 1, XIMPrimary: 8, XIMTertiary: 32: selected text
        // XIMUnderline: 2, XIMSecondary: 16: underlined text
        // XIMHighlight: 4: normal text
        // XIMVisibleToForward: 64, XIMVisibleToBackward: 128, XIMVisibleCenter: 256: text align position
        // XIMPrimary, XIMHighlight, XIMSecondary, XIMTertiary are not specified,
        // but interchangeable as above
        // Currently there's no way to support these.
        self.last_callback_event
            .replace(XimCallbackEvent::XimPreeditEvent(
                self.window,
                String::from(preedit_string),
            ));
        // self.xim_tx
        //     .send()
        //     .ok();
        Ok(())
    }
}

#[derive(Clone)]
pub struct XimXCBConnection(Rc<XCBConnection>, Rc<Cell<bool>>);

impl XimXCBConnection {
    pub fn new(connection: Rc<XCBConnection>, can_flush: Rc<Cell<bool>>) -> Self {
        Self(connection, can_flush)
    }

    pub fn set_can_flush(&self, can_flush: bool) {
        self.1.set(can_flush);
    }
}

impl HasConnection for XimXCBConnection {
    type Connection = Self;

    fn conn(&self) -> &Self::Connection {
        self
    }
}

impl Connection for XimXCBConnection {
    fn wait_for_raw_event_with_sequence(
        &self,
    ) -> Result<x11rb::connection::RawEventAndSeqNumber<Self::Buf>, x11rb::errors::ConnectionError>
    {
        self.0.wait_for_raw_event_with_sequence()
    }

    fn poll_for_raw_event_with_sequence(
        &self,
    ) -> Result<
        Option<x11rb::connection::RawEventAndSeqNumber<Self::Buf>>,
        x11rb::errors::ConnectionError,
    > {
        self.0.poll_for_raw_event_with_sequence()
    }

    fn flush(&self) -> Result<(), x11rb::errors::ConnectionError> {
        if self.1.get() {
            // println!("*real* flush");
            self.0.flush()
        } else {
            // println!("fake flush");
            Ok(())
        }
    }

    fn setup(&self) -> &xproto::Setup {
        self.0.setup()
    }

    fn generate_id(&self) -> Result<u32, x11rb::errors::ReplyOrIdError> {
        self.0.generate_id()
    }
}

impl RequestConnection for XimXCBConnection {
    type Buf = <XCBConnection as RequestConnection>::Buf;

    fn send_request_with_reply<R>(
        &self,
        bufs: &[std::io::IoSlice<'_>],
        fds: Vec<x11rb::utils::RawFdContainer>,
    ) -> Result<x11rb::cookie::Cookie<'_, Self, R>, x11rb::errors::ConnectionError>
    where
        R: x11rb::x11_utils::TryParse,
    {
        self.0
            .send_request_with_reply::<R>(bufs, fds)
            .map(|cookie| x11rb::cookie::Cookie::new(self, cookie.sequence_number()))
    }

    fn send_request_with_reply_with_fds<R>(
        &self,
        bufs: &[std::io::IoSlice<'_>],
        fds: Vec<x11rb::utils::RawFdContainer>,
    ) -> Result<CookieWithFds<'_, Self, R>, x11rb::errors::ConnectionError>
    where
        R: x11rb::x11_utils::TryParseFd,
    {
        self.0
            .send_request_with_reply_with_fds::<R>(bufs, fds)
            .map(|cookie| CookieWithFds::new(self, cookie.sequence_number()))
    }

    fn send_request_without_reply(
        &self,
        bufs: &[std::io::IoSlice<'_>],
        fds: Vec<x11rb::utils::RawFdContainer>,
    ) -> Result<VoidCookie<'_, Self>, x11rb::errors::ConnectionError> {
        self.0
            .send_request_without_reply(bufs, fds)
            .map(|cookie| VoidCookie::new(self, cookie.sequence_number()))
    }

    fn discard_reply(
        &self,
        sequence: x11rb::connection::SequenceNumber,
        kind: x11rb::connection::RequestKind,
        mode: x11rb::connection::DiscardMode,
    ) {
        self.0.discard_reply(sequence, kind, mode)
    }

    fn prefetch_extension_information(
        &self,
        extension_name: &'static str,
    ) -> Result<(), x11rb::errors::ConnectionError> {
        self.0.prefetch_extension_information(extension_name)
    }

    fn extension_information(
        &self,
        extension_name: &'static str,
    ) -> Result<Option<x11rb::x11_utils::ExtensionInformation>, x11rb::errors::ConnectionError>
    {
        self.0.extension_information(extension_name)
    }

    fn wait_for_reply_or_raw_error(
        &self,
        sequence: x11rb::connection::SequenceNumber,
    ) -> Result<x11rb::connection::ReplyOrError<Self::Buf>, x11rb::errors::ConnectionError> {
        self.0.wait_for_reply_or_raw_error(sequence)
    }

    fn wait_for_reply(
        &self,
        sequence: x11rb::connection::SequenceNumber,
    ) -> Result<Option<Self::Buf>, x11rb::errors::ConnectionError> {
        self.0.wait_for_reply(sequence)
    }

    fn wait_for_reply_with_fds_raw(
        &self,
        sequence: x11rb::connection::SequenceNumber,
    ) -> Result<
        x11rb::connection::ReplyOrError<x11rb::connection::BufWithFds<Self::Buf>, Self::Buf>,
        x11rb::errors::ConnectionError,
    > {
        self.0.wait_for_reply_with_fds_raw(sequence)
    }

    fn check_for_raw_error(
        &self,
        sequence: x11rb::connection::SequenceNumber,
    ) -> Result<Option<Self::Buf>, x11rb::errors::ConnectionError> {
        self.0.check_for_raw_error(sequence)
    }

    fn prefetch_maximum_request_bytes(&self) {
        self.0.prefetch_maximum_request_bytes()
    }

    fn maximum_request_bytes(&self) -> usize {
        self.0.maximum_request_bytes()
    }

    fn parse_error(
        &self,
        error: &[u8],
    ) -> Result<x11rb::x11_utils::X11Error, x11rb::errors::ParseError> {
        self.0.parse_error(error)
    }

    fn parse_event(&self, event: &[u8]) -> Result<Event, x11rb::errors::ParseError> {
        self.0.parse_event(event)
    }
}
