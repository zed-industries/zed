use std::default::Default;

use x11rb::protocol::{Event, xproto};
use xim::{AHashMap, AttributeName, Client, ClientError, ClientHandler, InputStyle};

pub enum XimCallbackEvent {
    XimXEvent(x11rb::protocol::Event),
    XimPreeditEvent(xproto::Window, String),
    XimCommitEvent(xproto::Window, String),
}

pub struct XimHandler {
    pub im_id: u16,
    pub ic_id: u16,
    pub connected: bool,
    pub window: xproto::Window,
    pub last_callback_event: Option<XimCallbackEvent>,
}

impl XimHandler {
    pub fn new() -> Self {
        Self {
            im_id: Default::default(),
            ic_id: Default::default(),
            connected: false,
            window: Default::default(),
            last_callback_event: None,
        }
    }
}

impl<C: Client<XEvent = xproto::KeyPressEvent>> ClientHandler<C> for XimHandler {
    fn handle_connect(&mut self, client: &mut C) -> Result<(), ClientError> {
        dbg!("XIM: handle_connect called");
        let result = client.open("C");
        dbg!("XIM: handle_connect result", &result);
        result
    }

    fn handle_open(&mut self, client: &mut C, input_method_id: u16) -> Result<(), ClientError> {
        dbg!("XIM: handle_open called", input_method_id);
        self.im_id = input_method_id;

        let result = client.get_im_values(input_method_id, &[AttributeName::QueryInputStyle]);
        dbg!("XIM: handle_open result", &result);
        result
    }

    fn handle_get_im_values(
        &mut self,
        client: &mut C,
        input_method_id: u16,
        _attributes: AHashMap<AttributeName, Vec<u8>>,
    ) -> Result<(), ClientError> {
        let ic_attributes = client
            .build_ic_attributes()
            .push(AttributeName::InputStyle, InputStyle::PREEDIT_CALLBACKS)
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
        dbg!("XIM: handle_create_ic called", input_context_id);
        self.connected = true;
        self.ic_id = input_context_id;
        dbg!("XIM: connection established", self.connected, self.ic_id);
        Ok(())
    }

    fn handle_commit(
        &mut self,
        _client: &mut C,
        _input_method_id: u16,
        _input_context_id: u16,
        text: &str,
    ) -> Result<(), ClientError> {
        dbg!("XIM: handle_commit called", text, self.window);
        self.last_callback_event = Some(XimCallbackEvent::XimCommitEvent(
            self.window,
            String::from(text),
        ));
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
        dbg!("XIM: handle_forward_event called", xev.response_type);
        match xev.response_type {
            x11rb::protocol::xproto::KEY_PRESS_EVENT => {
                dbg!("XIM: forwarding key press event");
                self.last_callback_event = Some(XimCallbackEvent::XimXEvent(Event::KeyPress(xev)));
            }
            x11rb::protocol::xproto::KEY_RELEASE_EVENT => {
                dbg!("XIM: forwarding key release event");
                self.last_callback_event =
                    Some(XimCallbackEvent::XimXEvent(Event::KeyRelease(xev)));
            }
            _ => {
                dbg!("XIM: ignoring event type", xev.response_type);
            }
        }
        Ok(())
    }

    fn handle_close(&mut self, client: &mut C, _input_method_id: u16) -> Result<(), ClientError> {
        dbg!("XIM: handle_close called");
        // self.connected = false;
        let result = client.disconnect();
        dbg!("XIM: disconnect result", &result);
        result
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
        dbg!(
            "XIM: handle_preedit_draw called",
            preedit_string,
            self.window,
            _caret,
            _chg_first,
            _chg_len
        );
        // XIMReverse: 1, XIMPrimary: 8, XIMTertiary: 32: selected text
        // XIMUnderline: 2, XIMSecondary: 16: underlined text
        // XIMHighlight: 4: normal text
        // XIMVisibleToForward: 64, XIMVisibleToBackward: 128, XIMVisibleCenter: 256: text align position
        // XIMPrimary, XIMHighlight, XIMSecondary, XIMTertiary are not specified,
        // but interchangeable as above
        // Currently there's no way to support these.
        self.last_callback_event = Some(XimCallbackEvent::XimPreeditEvent(
            self.window,
            String::from(preedit_string),
        ));
        Ok(())
    }
}
