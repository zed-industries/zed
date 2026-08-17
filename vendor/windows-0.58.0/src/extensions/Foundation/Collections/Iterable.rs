use crate::Foundation::Collections::{IIterable, IIterable_Impl, IIterator, IIterator_Impl};

#[windows_core::implement(IIterable<T>)]
struct StockIterable<T>
where
    T: windows_core::RuntimeType + 'static,
    T::Default: Clone,
{
    values: Vec<T::Default>,
}

impl<T> IIterable_Impl<T> for StockIterable_Impl<T>
where
    T: windows_core::RuntimeType,
    T::Default: Clone,
{
    fn First(&self) -> windows_core::Result<IIterator<T>> {
        use windows_core::IUnknownImpl;
        Ok(windows_core::ComObject::new(StockIterator { owner: self.to_object(), current: 0.into() }).into_interface())
    }
}

#[windows_core::implement(IIterator<T>)]
struct StockIterator<T>
where
    T: windows_core::RuntimeType + 'static,
    T::Default: Clone,
{
    owner: windows_core::ComObject<StockIterable<T>>,
    current: std::sync::atomic::AtomicUsize,
}

impl<T> IIterator_Impl<T> for StockIterator_Impl<T>
where
    T: windows_core::RuntimeType,
    T::Default: Clone,
{
    fn Current(&self) -> windows_core::Result<T> {
        let owner: &StockIterable<T> = &self.owner;
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);

        if self.owner.values.len() > current {
            T::from_default(&owner.values[current])
        } else {
            Err(windows_core::Error::from(windows_core::imp::E_BOUNDS))
        }
    }

    fn HasCurrent(&self) -> windows_core::Result<bool> {
        let owner: &StockIterable<T> = &self.owner;
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);

        Ok(owner.values.len() > current)
    }

    fn MoveNext(&self) -> windows_core::Result<bool> {
        let owner: &StockIterable<T> = &self.owner;
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);

        if current < owner.values.len() {
            self.current.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        Ok(owner.values.len() > current + 1)
    }

    fn GetMany(&self, values: &mut [T::Default]) -> windows_core::Result<u32> {
        let owner: &StockIterable<T> = &self.owner;
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);

        let actual = std::cmp::min(owner.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&owner.values[current..current + actual]);
        self.current.fetch_add(actual, std::sync::atomic::Ordering::Relaxed);
        Ok(actual as u32)
    }
}

impl<T> TryFrom<Vec<T::Default>> for IIterable<T>
where
    T: windows_core::RuntimeType,
    T::Default: Clone,
{
    type Error = windows_core::Error;
    fn try_from(values: Vec<T::Default>) -> windows_core::Result<Self> {
        // TODO: should provide a fallible try_into or more explicit allocator
        Ok(windows_core::ComObject::new(StockIterable { values }).into_interface())
    }
}
