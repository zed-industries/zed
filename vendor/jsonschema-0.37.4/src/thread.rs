#[cfg(target_family = "wasm")]
#[doc(hidden)]
pub trait ThreadBound {}

#[cfg(target_family = "wasm")]
impl<T: ?Sized> ThreadBound for T {}

#[cfg(not(target_family = "wasm"))]
#[doc(hidden)]
pub trait ThreadBound: Send + Sync {}

#[cfg(not(target_family = "wasm"))]
impl<T: ?Sized + Send + Sync> ThreadBound for T {}

#[cfg(target_family = "wasm")]
pub(crate) type Shared<T> = std::rc::Rc<T>;
#[cfg(not(target_family = "wasm"))]
pub(crate) type Shared<T> = std::sync::Arc<T>;

#[cfg(target_family = "wasm")]
pub(crate) type SharedWeak<T> = std::rc::Weak<T>;
#[cfg(not(target_family = "wasm"))]
pub(crate) type SharedWeak<T> = std::sync::Weak<T>;
