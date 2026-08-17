#[non_exhaustive]
#[derive(Debug)]
pub enum LockstepError {
    ArgumentNotFound(String),
    InterfaceNotFound(String),
    MemberNotFound(String),
    PropertyNotFound(String),
}

impl std::error::Error for LockstepError {}

impl std::fmt::Display for LockstepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockstepError::ArgumentNotFound(name) => {
                write!(f, "Argument \"{name}\" not found.")
            }
            LockstepError::InterfaceNotFound(name) => {
                write!(f, "Interface \"{name}\" not found.")
            }
            LockstepError::MemberNotFound(name) => {
                write!(f, "Member \"{name}\" not found.")
            }
            LockstepError::PropertyNotFound(name) => {
                write!(f, "Property \"{name}\" not found.")
            }
        }
    }
}
