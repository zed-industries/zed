#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct OnceLock<T>(std::sync::OnceLock<T>);

impl<T> OnceLock<T> {
    pub const fn new() -> Self {
        Self(std::sync::OnceLock::new())
    }

    pub fn get(&self) -> Option<&T> {
        self.0.get()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.0.get_mut()
    }

    pub fn set(&self, value: T) -> Result<(), T> {
        self.0.set(value)
    }

    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        self.0.get_or_init(f)
    }

    pub fn into_inner(self) -> Option<T> {
        self.0.into_inner()
    }

    pub fn take(&mut self) -> Option<T> {
        self.0.take()
    }
}

impl<T> From<T> for OnceLock<T> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
