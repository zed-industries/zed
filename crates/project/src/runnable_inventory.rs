use runnable::{Runnable, TaskHandle};
use slotmap::{DefaultKey, SlotMap};

#[derive(Default)]
pub(crate) struct Inventory {
    available_runnables: Vec<Box<dyn Runnable>>,
    runnables_underway: SlotMap<DefaultKey, TaskHandle>,
}

struct InventorySlot {}
impl Inventory {
    pub(crate) fn cancel(&mut self) {}
}
