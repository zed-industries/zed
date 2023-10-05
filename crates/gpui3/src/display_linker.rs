use crate::{DisplayId, PlatformDisplayLinker, VideoTimestamp};
use collections::HashMap;
use parking_lot::Mutex;
use std::sync::Arc;

type FrameCallback = Box<dyn FnOnce(&VideoTimestamp, &VideoTimestamp) + Send>;

pub struct DisplayLinker {
    platform_linker: Arc<dyn PlatformDisplayLinker>,
    next_frame_callbacks: Arc<Mutex<HashMap<DisplayId, Vec<FrameCallback>>>>,
}

impl DisplayLinker {
    pub(crate) fn new(platform_linker: Arc<dyn PlatformDisplayLinker>) -> Self {
        Self {
            platform_linker,
            next_frame_callbacks: Default::default(),
        }
    }

    pub(crate) fn on_next_frame(
        &self,
        display_id: DisplayId,
        callback: impl FnOnce(&VideoTimestamp, &VideoTimestamp) + Send + 'static,
    ) {
        let next_frame_callbacks = self.next_frame_callbacks.clone();
        let callback = Box::new(callback);
        match self.next_frame_callbacks.lock().entry(display_id) {
            collections::hash_map::Entry::Occupied(mut entry) => {
                if entry.get().is_empty() {
                    self.platform_linker.start(display_id);
                }
                entry.get_mut().push(callback)
            }
            collections::hash_map::Entry::Vacant(entry) => {
                // let platform_linker = self.platform_linker.clone();
                self.platform_linker.set_output_callback(
                    display_id,
                    Box::new(move |current_time, output_time| {
                        for callback in next_frame_callbacks
                            .lock()
                            .get_mut(&display_id)
                            .unwrap()
                            .drain(..)
                        {
                            callback(current_time, output_time);
                        }
                        // platform_linker.stop(display_id);
                    }),
                );
                self.platform_linker.start(display_id);
                entry.insert(vec![callback]);
            }
        }
    }
}
