use super::FutureState;

#[derive(Default)]
pub struct State;

impl FutureState<'_> {
    pub(super) fn read_inter_task_stream(&mut self) {
        assert!(
            self.remaining_work(),
            "

    Rust task cannot sleep waiting only on Rust-originating events unless the
    `wit-bindgen` crate is compiled with the `inter-task-wakeup` feature
    enabled.

"
        );
    }

    pub(super) fn cancel_inter_task_stream_read(&mut self) {
        // nothing to do
    }
}

impl State {
    pub fn consume_waitable_event(&mut self, _waitable: u32, _code: u32) -> bool {
        false
    }
}

#[derive(Default)]
pub struct WakerState;

impl WakerState {
    pub fn wake(&self) {
        panic!(
            "

    Cannot support cross-component-model-task wakeup unlses the `wit-bindgen`
    crate is compiled with the `inter-task-wakeup` feature enabled.

"
        );
    }
}
