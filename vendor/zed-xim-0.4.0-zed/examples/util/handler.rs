use std::convert::TryInto;

use xim::{AHashMap, Client, ClientError, ClientHandler};
use xim_parser::{AttributeName, InputStyle, Point};

#[derive(Default)]
pub struct ExampleHandler {
    pub im_id: u16,
    pub ic_id: u16,
    pub connected: bool,
    pub window: u32,
}

#[cfg(all(feature = "x11rb-client", not(feature = "xlib-client")))]
trait ClientAlias = Client<XEvent = x11rb::protocol::xproto::KeyPressEvent>;

#[cfg(all(feature = "xlib-client", not(feature = "x11rb-client")))]
trait ClientAlias = Client<XEvent = x11_dl::xlib::XKeyPressedEvent>;

#[cfg(all(feature = "xlib-client", feature = "x11rb-client"))]
trait ClientAlias = Client;

impl<C: ClientAlias> ClientHandler<C> for ExampleHandler {
    fn handle_connect(&mut self, client: &mut C) -> Result<(), ClientError> {
        log::trace!("Connected");
        client.open("en_US")
    }

    fn handle_open(&mut self, client: &mut C, input_method_id: u16) -> Result<(), ClientError> {
        log::trace!("Opened");
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
                InputStyle::PREEDIT_CALLBACKS | InputStyle::STATUS_NOTHING,
            )
            .push(AttributeName::ClientWindow, self.window)
            .push(AttributeName::FocusWindow, self.window)
            .nested_list(AttributeName::PreeditAttributes, |b| {
                b.push(AttributeName::SpotLocation, Point { x: 0, y: 0 });
            })
            .build();
        client.create_ic(input_method_id, ic_attributes)
    }

    fn handle_create_ic(
        &mut self,
        _client: &mut C,
        input_method_id: u16,
        input_context_id: u16,
    ) -> Result<(), ClientError> {
        self.connected = true;
        self.ic_id = input_context_id;
        log::info!("IC created {}, {}", input_method_id, input_context_id);
        Ok(())
    }

    fn handle_forward_event(
        &mut self,
        _client: &mut C,
        _input_method_id: u16,
        _input_context_id: u16,
        flag: xim::ForwardEventFlag,
        xev: C::XEvent,
    ) -> Result<(), ClientError> {
        #[cfg(all(feature = "x11rb-client", not(feature = "xlib-client")))]
        let keycode = xev.detail;
        #[cfg(all(feature = "xlib-client", not(feature = "x11rb-client")))]
        let keycode = xev.keycode;

        // When both feature are enabled, emit flag only.
        #[cfg(all(feature = "xlib-client", feature = "x11rb-client"))]
        log::info!("Handle forward event {:?}", flag);

        #[cfg(not(all(feature = "xlib-client", feature = "x11rb-client")))]
        log::info!("Handle forward event {:?}, {}", flag, keycode);
        Ok(())
    }

    fn handle_commit(
        &mut self,
        _client: &mut C,
        _input_method_id: u16,
        _input_context_id: u16,
        text: &str,
    ) -> Result<(), ClientError> {
        log::info!("Commited {}", text);
        Ok(())
    }

    fn handle_disconnect(&mut self) {
        log::info!("disconnected");
    }

    fn handle_close(&mut self, client: &mut C, _input_method_id: u16) -> Result<(), ClientError> {
        log::info!("closed");
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

    fn handle_set_event_mask(
        &mut self,
        _client: &mut C,
        input_method_id: u16,
        input_context_id: u16,
        forward_event_mask: u32,
        synchronous_event_mask: u32,
    ) -> Result<(), ClientError> {
        log::info!(
            "Set event mask {}, {}, {}, {}",
            input_method_id,
            input_context_id,
            forward_event_mask,
            synchronous_event_mask
        );
        Ok(())
    }

    fn handle_preedit_start(
        &mut self,
        _client: &mut C,
        input_method_id: u16,
        input_context_id: u16,
    ) -> Result<(), ClientError> {
        log::info!("Preedit start {}, {}", input_method_id, input_context_id);
        Ok(())
    }

    fn handle_preedit_done(
        &mut self,
        _client: &mut C,
        input_method_id: u16,
        input_context_id: u16,
    ) -> Result<(), ClientError> {
        log::info!("Preedit done {}, {}", input_method_id, input_context_id);
        Ok(())
    }

    fn handle_preedit_draw(
        &mut self,
        _client: &mut C,
        _input_method_id: u16,
        _input_context_id: u16,
        caret: i32,
        _chg_first: i32,
        _chg_len: i32,
        _status: xim::PreeditDrawStatus,
        preedit_string: &str,
        feedbacks: Vec<xim::Feedback>,
    ) -> Result<(), ClientError> {
        let mut caret_string = String::new();

        let mut chars = preedit_string.chars();

        caret_string.extend(chars.by_ref().take(caret.try_into().unwrap_or_default()));
        caret_string.push('|');
        caret_string.extend(chars);

        log::info!("Preedit {}({:?})", caret_string, feedbacks);

        Ok(())
    }
}
