use xcb::{present, x, Xid};

use std::{
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

xcb::atoms_struct! {
    #[derive(Debug)]
    struct Atoms {
        wm_protocols    => b"WM_PROTOCOLS",
        wm_del_window   => b"WM_DELETE_WINDOW",
    }
}

struct Example {
    conn: xcb::Connection,
    window: x::Window,
    atoms: Atoms,
}

impl Example {
    fn build() -> xcb::Result<Self> {
        let (conn, screen_num) =
            xcb::Connection::connect_with_extensions(None, &[xcb::Extension::Present], &[])?;
        let setup = conn.get_setup();
        let screen = setup.roots().nth(screen_num as usize).unwrap();

        let window: x::Window = conn.generate_id();

        conn.send_request(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as u8,
            wid: window,
            parent: screen.root(),
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            border_width: 10,
            class: x::WindowClass::InputOutput,
            visual: screen.root_visual(),
            value_list: &[
                x::Cw::BackPixel(screen.white_pixel()),
                x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS),
            ],
        });

        conn.send_and_check_request(&x::MapWindow { window })?;

        // retrieving a few atoms
        let atoms = Atoms::intern_all(&conn)?;

        // activate the sending of close event through `x::Event::ClientMessage`
        // either the request must be checked as follow, or conn.flush() must be called before entering the loop
        conn.send_and_check_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window,
            property: atoms.wm_protocols,
            r#type: x::ATOM_ATOM,
            data: &[atoms.wm_del_window],
        })?;

        conn.flush()?;

        Ok(Self {
            conn,
            window,
            atoms,
        })
    }

    fn main_event_loop(&self) -> xcb::Result<()> {
        loop {
            let ev = self.conn.wait_for_event()?;
            println!("Received regular event {:#?}", ev);
            match ev {
                xcb::Event::X(x::Event::KeyPress(ev)) => {
                    if ev.detail() == 0x18 {
                        // Q (on qwerty)
                        break Ok(());
                    }
                }
                xcb::Event::X(x::Event::ClientMessage(ev)) => {
                    if let x::ClientMessageData::Data32([atom, ..]) = ev.data() {
                        if atom == self.atoms.wm_del_window.resource_id() {
                            // window "x" button clicked by user, we gracefully exit
                            break Ok(());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn special_event_loop(&self, rx: mpsc::Receiver<Msg>) -> xcb::Result<()> {
        let eid = self.conn.generate_id();

        self.conn.send_request(&present::SelectInput {
            eid,
            window: self.window,
            event_mask: present::EventMask::CONFIGURE_NOTIFY
                | present::EventMask::COMPLETE_NOTIFY
                | present::EventMask::IDLE_NOTIFY,
        });

        let special_event = self
            .conn
            .register_for_special_event(xcb::Extension::Present, eid);

        self.conn.flush()?;

        loop {
            let ev = self.conn.poll_for_special_event2(&special_event)?;
            if let Some(ev) = ev {
                println!("Received special event {:#?}", ev);
            }

            if let Ok(msg) = rx.recv_timeout(Duration::from_millis(100)) {
                match msg {
                    Msg::Quit => break,
                }
            }
        }

        self.conn.unregister_for_special_event(special_event);

        Ok(())
    }
}

enum Msg {
    Quit,
}

fn main() -> xcb::Result<()> {
    let example = Example::build()?;
    let example = Arc::new(example);

    let (tx, rx) = mpsc::channel();

    let special_event_thread = {
        let example = example.clone();
        thread::spawn(move || example.special_event_loop(rx))
    };
    example.main_event_loop()?;

    tx.send(Msg::Quit).unwrap();
    special_event_thread.join().unwrap()?;

    Ok(())
}
