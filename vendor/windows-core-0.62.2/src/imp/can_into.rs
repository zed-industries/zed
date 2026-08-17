pub trait CanInto<T>: Sized {
    const QUERY: bool = false;
}

impl<T> CanInto<T> for T where T: Clone {}
