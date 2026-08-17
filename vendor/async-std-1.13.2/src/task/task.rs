use std::fmt;
use std::sync::Arc;

use crate::task::TaskId;

/// A handle to a task.
#[derive(Clone)]
pub struct Task {
    /// The task ID.
    id: TaskId,

    /// The optional task name.
    name: Option<Arc<String>>,
}

impl Task {
    /// Creates a new task handle.
    #[inline]
    pub(crate) fn new(name: Option<Arc<String>>) -> Task {
        Task {
            id: TaskId::generate(),
            name,
        }
    }

    /// Gets the task's unique identifier.
    #[inline]
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// Returns the name of this task.
    ///
    /// The name is configured by [`Builder::name`] before spawning.
    ///
    /// [`Builder::name`]: struct.Builder.html#method.name
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("id", &self.id())
            .field("name", &self.name())
            .finish()
    }
}
