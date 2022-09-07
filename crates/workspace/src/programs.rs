// TODO: Need to put this basic structure in workspace, and make 'program handles'
// based off of the 'searchable item' pattern except with models. This way, the workspace's clients
// can register their models as programs with a specific identity and capable of notifying the workspace
// Programs are:
//  - Kept alive by the program manager, they need to emit an event to get dropped from it
//  - Can be interacted with directly, (closed, activated, etc.) by the program manager, bypassing
//    associated view(s)
//  - Have special rendering methods that the program manager requires them to implement to fill out
//    the status bar
//  - Can emit events for the program manager which:
//    - Add a jewel (notification, change, etc.)
//    - Drop the program
//    - ???
//  - Program Manager is kept in a global, listens for window drop so it can drop all it's program handles

use collections::HashMap;
use gpui::{AnyModelHandle, Entity, ModelHandle, View, ViewContext};

/// This struct is going to be the starting point for the 'program manager' feature that will
/// eventually be implemented to provide a collaborative way of engaging with identity-having
/// features like the terminal.
pub struct Dock {
    // TODO: Make this a hashset or something
    modals: HashMap<usize, AnyModelHandle>,
}

impl Dock {
    pub fn insert_or_replace<T: Entity, V: View>(
        window: usize,
        program: ModelHandle<T>,
        cx: &mut ViewContext<V>,
    ) -> Option<AnyModelHandle> {
        cx.update_global::<Dock, _, _>(|pm, _| pm.insert_or_replace_internal::<T>(window, program))
    }

    pub fn remove<T: Entity, V: View>(
        window: usize,
        cx: &mut ViewContext<V>,
    ) -> Option<ModelHandle<T>> {
        cx.update_global::<Dock, _, _>(|pm, _| pm.remove_internal::<T>(window))
    }

    pub fn new() -> Self {
        Self {
            modals: Default::default(),
        }
    }

    /// Inserts or replaces the model at the given location.
    fn insert_or_replace_internal<T: Entity>(
        &mut self,
        window: usize,
        program: ModelHandle<T>,
    ) -> Option<AnyModelHandle> {
        self.modals.insert(window, AnyModelHandle::from(program))
    }

    /// Remove the program associated with this window, if it's of the given type
    fn remove_internal<T: Entity>(&mut self, window: usize) -> Option<ModelHandle<T>> {
        let program = self.modals.remove(&window);
        if let Some(program) = program {
            if program.is::<T>() {
                // Guaranteed to be some, but leave it in the option
                // anyway for the API
                program.downcast()
            } else {
                // Model is of the incorrect type, put it back
                self.modals.insert(window, program);
                None
            }
        } else {
            None
        }
    }
}
