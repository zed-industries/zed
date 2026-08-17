use std::ops::Deref;

use super::Signature;

/// A child signature of a container signature.
#[derive(Debug, Clone)]
pub enum Child {
    /// A static child signature.
    Static { child: &'static Signature },
    /// A dynamic child signature.
    Dynamic { child: Box<Signature> },
}

impl Child {
    /// The underlying child `Signature`.
    pub const fn signature(&self) -> &Signature {
        match self {
            Child::Static { child } => child,
            Child::Dynamic { child } => child,
        }
    }

    /// The length of the child signature in string form.
    pub const fn string_len(&self) -> usize {
        self.signature().string_len()
    }
}

impl Deref for Child {
    type Target = Signature;

    fn deref(&self) -> &Self::Target {
        self.signature()
    }
}

impl From<Box<Signature>> for Child {
    fn from(child: Box<Signature>) -> Self {
        Child::Dynamic { child }
    }
}

impl From<Signature> for Child {
    fn from(child: Signature) -> Self {
        Child::Dynamic {
            child: Box::new(child),
        }
    }
}

impl From<&'static Signature> for Child {
    fn from(child: &'static Signature) -> Self {
        Child::Static { child }
    }
}
