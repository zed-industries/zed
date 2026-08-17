/// Used to wrap a function result so callers can see whether the result was cached.
#[derive(Clone)]
pub struct Return<T> {
    pub was_cached: bool,
    pub value: T,
}

impl<T> Return<T> {
    pub fn new(value: T) -> Self {
        Self {
            was_cached: false,
            value,
        }
    }
}

impl<T> std::ops::Deref for Return<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for Return<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
