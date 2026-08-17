use super::*;
use windows_core::*;

#[implement(IVectorView<T>, IIterable<T>)]
struct StockVectorView<T>
where
    T: RuntimeType + 'static,
    T::Default: Clone + PartialEq,
{
    values: Vec<T::Default>,
}

impl<T> IIterable_Impl<T> for StockVectorView_Impl<T>
where
    T: RuntimeType,
    T::Default: Clone + PartialEq,
{
    fn First(&self) -> Result<IIterator<T>> {
        Ok(ComObject::new(StockVectorViewIterator {
            owner: self.to_object(),
            current: 0.into(),
        })
        .into_interface())
    }
}

impl<T> IVectorView_Impl<T> for StockVectorView_Impl<T>
where
    T: RuntimeType,
    T::Default: Clone + PartialEq,
{
    fn GetAt(&self, index: u32) -> Result<T> {
        let item = self
            .values
            .get(index as usize)
            .ok_or_else(|| Error::from(E_BOUNDS))?;

        T::from_default(item)
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.values.len().try_into()?)
    }

    fn IndexOf(&self, value: Ref<T>, result: &mut u32) -> Result<bool> {
        match self.values.iter().position(|element| element == &*value) {
            Some(index) => {
                *result = index as u32;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn GetMany(&self, current: u32, values: &mut [T::Default]) -> Result<u32> {
        let current = current as usize;

        if current >= self.values.len() {
            return Ok(0);
        }

        let actual = std::cmp::min(self.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&self.values[current..current + actual]);
        Ok(actual as u32)
    }
}

#[implement(IIterator<T>)]
struct StockVectorViewIterator<T>
where
    T: RuntimeType + 'static,
    T::Default: Clone + PartialEq,
{
    owner: ComObject<StockVectorView<T>>,
    current: std::sync::atomic::AtomicUsize,
}

impl<T> IIterator_Impl<T> for StockVectorViewIterator_Impl<T>
where
    T: RuntimeType,
    T::Default: Clone + PartialEq,
{
    fn Current(&self) -> Result<T> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);

        if let Some(item) = self.owner.values.get(current) {
            T::from_default(item)
        } else {
            Err(Error::from(E_BOUNDS))
        }
    }

    fn HasCurrent(&self) -> Result<bool> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        Ok(self.owner.values.len() > current)
    }

    fn MoveNext(&self) -> Result<bool> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);

        if current < self.owner.values.len() {
            self.current
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        Ok(self.owner.values.len() > current + 1)
    }

    fn GetMany(&self, values: &mut [T::Default]) -> Result<u32> {
        let current = self.current.load(std::sync::atomic::Ordering::Relaxed);
        let actual = std::cmp::min(self.owner.values.len() - current, values.len());
        let (values, _) = values.split_at_mut(actual);
        values.clone_from_slice(&self.owner.values[current..current + actual]);

        self.current
            .fetch_add(actual, std::sync::atomic::Ordering::Relaxed);

        Ok(actual as u32)
    }
}

impl<T> From<Vec<T::Default>> for IVectorView<T>
where
    T: RuntimeType,
    T::Default: Clone + PartialEq,
{
    fn from(values: Vec<T::Default>) -> Self {
        ComObject::new(StockVectorView { values }).into_interface()
    }
}
