use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::time::{Duration, Instant};

use calloop::{EventLoop, LoopHandle};
use collections::HashMap;

use util::ResultExt;

use crate::platform::linux::LinuxClient;
use crate::platform::{LinuxCommon, PlatformWindow};
use crate::{
    px, AnyWindowHandle, Bounds, CursorStyle, DisplayId, Modifiers, ModifiersChangedEvent, Pixels,
    PlatformDisplay, PlatformInput, Point, ScrollDelta, Size, TouchPhase, WindowParams,
};

use calloop::{
    generic::{FdWrapper, Generic},
    RegistrationToken,
};

pub struct HeadlessClientState {
    pub(crate) loop_handle: LoopHandle<'static, HeadlessClient>,
    pub(crate) event_loop: Option<calloop::EventLoop<'static, HeadlessClient>>,
    pub(crate) common: LinuxCommon,
}

#[derive(Clone)]
pub(crate) struct HeadlessClient(Rc<RefCell<HeadlessClientState>>);

impl HeadlessClient {
    pub(crate) fn new() -> Self {
        let event_loop = EventLoop::try_new().unwrap();

        let (common, main_receiver) = LinuxCommon::new(event_loop.get_signal());

        let handle = event_loop.handle();

        handle.insert_source(main_receiver, |event, _, _: &mut HeadlessClient| {
            if let calloop::channel::Event::Msg(runnable) = event {
                runnable.run();
            }
        });

        HeadlessClient(Rc::new(RefCell::new(HeadlessClientState {
            event_loop: Some(event_loop),
            loop_handle: handle,
            common,
        })))
    }
}

impl LinuxClient for HeadlessClient {
    fn with_common<R>(&self, f: impl FnOnce(&mut LinuxCommon) -> R) -> R {
        f(&mut self.0.borrow_mut().common)
    }

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        vec![]
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        None
    }

    fn display(&self, id: DisplayId) -> Option<Rc<dyn PlatformDisplay>> {
        None
    }

    fn open_window(
        &self,
        _handle: AnyWindowHandle,
        params: WindowParams,
    ) -> Box<dyn PlatformWindow> {
        unimplemented!()
    }

    //todo(linux)
    fn set_cursor_style(&self, _style: CursorStyle) {}

    fn write_to_clipboard(&self, item: crate::ClipboardItem) {}

    fn read_from_clipboard(&self) -> Option<crate::ClipboardItem> {
        None
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
