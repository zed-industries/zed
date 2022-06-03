// use std::Error;

use serde::{de::DeserializeOwned, Serialize};

/// Represents a handle to a constant or function in the Runtime.
/// Should be constructed by calling [`Runtime::handle_for`].
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Handle(String);

impl Handle {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

/// Represents an interface that can be implemented by a plugin.
pub trait Interface
where
    Self: Sized,
{
    /// Create an interface from a given runtime.
    /// All handles to be used by the interface should be registered and stored in `Self`.
    fn from_runtime<T: Runtime>(runtime: &mut T) -> Option<Self>;
}

pub trait Runtime
where
    Self: Sized,
{
    /// Represents a plugin to be loaded by the runtime,
    /// e.g. some source code + anything else needed to set up.
    type Plugin;

    /// The error type for this module.
    /// Ideally should implement the [`std::err::Error`] trait.
    type Error;

    /// Initializes a plugin, returning a [`Runtime`] that can be queried.
    /// Note that if you have any configuration,
    fn init(plugin: Self::Plugin) -> Result<Self, Self::Error>;

    /// Returns a top-level constant from the module.
    /// This can be used to extract configuration information from the module, for example.
    /// Before calling this function, get a handle into the runtime using [`handle_for`].
    fn constant<T: DeserializeOwned>(&mut self, handle: &Handle) -> Result<T, Self::Error>;

    /// Call a function defined in the module.
    fn call<A: Serialize, R: DeserializeOwned>(
        &mut self,
        handle: &Handle,
        arg: A,
    ) -> Result<R, Self::Error>;

    /// Registers a handle with the runtime.
    /// This is a mutable item if needed, but generally
    /// this should be an immutable operation.
    /// Returns whether the handle exists/was successfully registered.
    fn register_handle<T: AsRef<str>>(&mut self, name: T) -> bool;

    /// Returns the handle for a given name if the handle is defined.
    /// Will only return an error if there was an error while trying to register the handle.
    /// This function uses [`register_handle`], no need to implement this one.
    fn handle_for<T: AsRef<str>>(&mut self, name: T) -> Option<Handle> {
        if self.register_handle(&name) {
            Some(Handle(name.as_ref().to_string()))
        } else {
            None
        }
    }

    /// Creates the given interface from the current module.
    /// Returns [`Error`] if the provided plugin does not match the expected interface.
    /// Essentially wraps the [`Interface`] trait.
    fn as_interface<T: Interface>(&mut self) -> Option<T> {
        Interface::from_runtime(self)
    }
}
