use gpui::SharedString;

/// Some context attached to a message in a thread.
#[derive(Debug, Clone)]
pub struct Context {
    pub name: SharedString,
    pub kind: ContextKind,
    pub text: SharedString,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextKind {
    File,
}
