#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//todo!(linux): remove
#![allow(unused_variables)]

use crate::{PlatformDispatcher, TaskLabel};
use async_task::Runnable;
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use std::{
    panic,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use xcb::x;

pub(crate) struct LinuxDispatcher {
    xcb_connection: Arc<xcb::Connection>,
    x_listener_window: x::Window,
    parker: Mutex<Parker>,
    timed_tasks: Mutex<Vec<(Instant, Runnable)>>,
    main_sender: flume::Sender<Runnable>,
    background_sender: flume::Sender<Runnable>,
    _background_thread: thread::JoinHandle<()>,
    main_thread_id: thread::ThreadId,
}

impl LinuxDispatcher {
    pub fn new(
        main_sender: flume::Sender<Runnable>,
        xcb_connection: &Arc<xcb::Connection>,
        x_root_index: i32,
    ) -> Self {
        let x_listener_window = xcb_connection.generate_id();
        let screen = xcb_connection
            .get_setup()
            .roots()
            .nth(x_root_index as usize)
            .unwrap();
        xcb_connection.send_request(&x::CreateWindow {
            depth: 0,
            wid: x_listener_window,
            parent: screen.root(),
            x: 0,
            y: 0,
            width: 1,
            height: 1,
            border_width: 0,
            class: x::WindowClass::InputOnly,
            visual: screen.root_visual(),
            value_list: &[],
        });

        let (background_sender, background_receiver) = flume::unbounded::<Runnable>();
        let background_thread = thread::spawn(move || {
            for runnable in background_receiver {
                let _ignore_panic = panic::catch_unwind(|| runnable.run());
            }
        });
        LinuxDispatcher {
            xcb_connection: Arc::clone(xcb_connection),
            x_listener_window,
            parker: Mutex::new(Parker::new()),
            timed_tasks: Mutex::new(Vec::new()),
            main_sender,
            background_sender,
            _background_thread: background_thread,
            main_thread_id: thread::current().id(),
        }
    }
}

impl Drop for LinuxDispatcher {
    fn drop(&mut self) {
        self.xcb_connection.send_request(&x::DestroyWindow {
            window: self.x_listener_window,
        });
    }
}

impl PlatformDispatcher for LinuxDispatcher {
    fn is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    fn dispatch(&self, runnable: Runnable, _: Option<TaskLabel>) {
        self.background_sender.send(runnable).unwrap();
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        self.main_sender.send(runnable).unwrap();
        // Send a message to the invisible window, forcing
        // the main loop to wake up and dispatch the runnable.
        self.xcb_connection.send_request(&x::SendEvent {
            propagate: false,
            destination: x::SendEventDest::Window(self.x_listener_window),
            event_mask: x::EventMask::NO_EVENT,
            event: &x::VisibilityNotifyEvent::new(
                self.x_listener_window,
                x::Visibility::Unobscured,
            ),
        });
        self.xcb_connection.flush().unwrap();
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        let moment = Instant::now() + duration;
        let mut timed_tasks = self.timed_tasks.lock();
        timed_tasks.push((moment, runnable));
        timed_tasks.sort_unstable_by(|&(ref a, _), &(ref b, _)| b.cmp(a));
    }

    fn tick(&self, background_only: bool) -> bool {
        let mut timed_tasks = self.timed_tasks.lock();
        let old_count = timed_tasks.len();
        while let Some(&(moment, _)) = timed_tasks.last() {
            if moment <= Instant::now() {
                let (_, runnable) = timed_tasks.pop().unwrap();
                runnable.run();
            } else {
                break;
            }
        }
        timed_tasks.len() != old_count
    }

    fn park(&self) {
        self.parker.lock().park()
    }

    fn unparker(&self) -> Unparker {
        self.parker.lock().unparker()
    }
}
