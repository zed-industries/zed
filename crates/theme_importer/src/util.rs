use anyhow::Result;

pub trait Traverse<T, U> {
    fn traverse(self, f: impl FnOnce(T) -> Result<U>) -> Result<Option<U>>;
}

impl<T, U> Traverse<T, U> for Option<T> {
    fn traverse(self, f: impl FnOnce(T) -> Result<U>) -> Result<Option<U>> {
        self.map_or(Ok(None), |value| f(value).map(Some))
    }
}
