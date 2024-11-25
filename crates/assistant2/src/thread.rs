use gpui::{ModelContext, Task};
use language_model::Role;

/// A message in a [`Thread`].
pub struct Message {
    pub role: Role,
    pub text: String,
}

/// A thread of conversation with the LLM.
pub struct Thread {
    pub messages: Vec<Message>,
    pub pending_completion_tasks: Vec<Task<()>>,
}

impl Thread {
    pub fn new(_cx: &mut ModelContext<Self>) -> Self {
        Self {
            messages: Vec::new(),
            pending_completion_tasks: Vec::new(),
        }
    }
}
