use gpui::ModelContext;
use language_model::Role;

/// A message in a [`Thread`].
pub struct Message {
    pub role: Role,
    pub text: String,
}

/// A thread of conversation with the LLM.
pub struct Thread {
    pub messages: Vec<Message>,
}

impl Thread {
    pub fn new(cx: &mut ModelContext<Self>) -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}
