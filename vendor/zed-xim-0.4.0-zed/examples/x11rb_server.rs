use x11rb::connection::Connection;
use xim::{
    x11rb::X11rbServer, Server, ServerError, ServerHandler, UserInputContext, XimConnections,
};
use xim_parser::InputStyle;

#[derive(Default)]
struct Handler {}

impl Handler {}

impl<S: Server<XEvent = x11rb::protocol::xproto::KeyPressEvent>> ServerHandler<S> for Handler {
    type InputContextData = ();
    type InputStyleArray = [InputStyle; 4];

    fn new_ic_data(
        &mut self,
        _server: &mut S,
        _style: InputStyle,
    ) -> Result<Self::InputContextData, ServerError> {
        Ok(())
    }

    fn input_styles(&self) -> Self::InputStyleArray {
        [
            InputStyle::PREEDIT_CALLBACKS | InputStyle::STATUS_NOTHING,
            // over-spot
            InputStyle::PREEDIT_NOTHING | InputStyle::STATUS_NOTHING,
            InputStyle::PREEDIT_POSITION | InputStyle::STATUS_NOTHING,
            InputStyle::PREEDIT_POSITION | InputStyle::STATUS_NONE,
        ]
    }

    fn filter_events(&self) -> u32 {
        1
    }

    fn handle_connect(&mut self, _server: &mut S) -> Result<(), ServerError> {
        log::info!("Connected!");
        Ok(())
    }

    fn handle_create_ic(
        &mut self,
        server: &mut S,
        user_ic: &mut UserInputContext<Self::InputContextData>,
    ) -> Result<(), ServerError> {
        server.set_event_mask(&user_ic.ic, 1, 0)
    }

    fn handle_forward_event(
        &mut self,
        server: &mut S,
        user_ic: &mut UserInputContext<Self::InputContextData>,
        xev: &S::XEvent,
    ) -> Result<bool, ServerError> {
        // Enter
        if xev.detail == 36 {
            server.preedit_draw(&mut user_ic.ic, "")?;
            server.commit(&user_ic.ic, "가나다")?;
        } else {
            server.preedit_draw(&mut user_ic.ic, "가나다")?;
        }
        Ok(true)
    }

    fn handle_destroy_ic(
        &mut self,
        _server: &mut S,
        _user_ic: UserInputContext<Self::InputContextData>,
    ) -> Result<(), ServerError> {
        Ok(())
    }

    fn handle_reset_ic(
        &mut self,
        _server: &mut S,
        _user_ic: &mut UserInputContext<Self::InputContextData>,
    ) -> Result<String, ServerError> {
        Ok(String::new())
    }

    fn handle_set_ic_values(
        &mut self,
        _server: &mut S,
        _user_ic: &mut UserInputContext<Self::InputContextData>,
    ) -> Result<(), ServerError> {
        Ok(())
    }

    fn handle_set_focus(
        &mut self,
        _server: &mut S,
        _user_ic: &mut UserInputContext<Self::InputContextData>,
    ) -> Result<(), ServerError> {
        Ok(())
    }

    fn handle_unset_focus(
        &mut self,
        _server: &mut S,
        _user_ic: &mut UserInputContext<Self::InputContextData>,
    ) -> Result<(), ServerError> {
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_custom_env("XIM_RS_LOG");

    let (conn, screen_num) = x11rb::rust_connection::RustConnection::connect(None)?;
    let mut server = X11rbServer::init(&conn, screen_num, "test_server", xim::ALL_LOCALES)?;
    let mut connections = XimConnections::new();
    let mut handler = Handler::default();

    loop {
        let e = conn.wait_for_event()?;
        server.filter_event(&e, &mut connections, &mut handler)?;
    }
}
