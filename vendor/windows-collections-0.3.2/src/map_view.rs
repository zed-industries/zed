use super::*;
use windows_core::*;

#[implement(IMapView<K, V>, IIterable<IKeyValuePair<K, V>>)]
struct StockMapView<K, V>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    map: std::collections::BTreeMap<K::Default, V::Default>,
}

impl<K, V> IIterable_Impl<IKeyValuePair<K, V>> for StockMapView_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn First(&self) -> Result<IIterator<IKeyValuePair<K, V>>> {
        Ok(ComObject::new(StockMapViewIterator::<K, V> {
            _owner: self.to_object(),
            current: std::sync::RwLock::new(self.map.iter()),
        })
        .into_interface())
    }
}

impl<K, V> IMapView_Impl<K, V> for StockMapView_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn Lookup(&self, key: Ref<K>) -> Result<V> {
        let value = self.map.get(&*key).ok_or_else(|| Error::from(E_BOUNDS))?;

        V::from_default(value)
    }

    fn Size(&self) -> Result<u32> {
        Ok(self.map.len().try_into()?)
    }

    fn HasKey(&self, key: Ref<K>) -> Result<bool> {
        Ok(self.map.contains_key(&*key))
    }

    fn Split(&self, first: OutRef<IMapView<K, V>>, second: OutRef<IMapView<K, V>>) -> Result<()> {
        _ = first.write(None);
        _ = second.write(None);
        Ok(())
    }
}

#[implement(IIterator<IKeyValuePair<K, V>>)]
struct StockMapViewIterator<'a, K, V>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    _owner: ComObject<StockMapView<K, V>>,
    current: std::sync::RwLock<std::collections::btree_map::Iter<'a, K::Default, V::Default>>,
}

impl<K, V> IIterator_Impl<IKeyValuePair<K, V>> for StockMapViewIterator_Impl<'_, K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn Current(&self) -> Result<IKeyValuePair<K, V>> {
        let mut current = self.current.read().unwrap().clone().peekable();

        if let Some((key, value)) = current.peek() {
            Ok(ComObject::new(StockKeyValuePair {
                key: (*key).clone(),
                value: (*value).clone(),
            })
            .into_interface())
        } else {
            Err(Error::from(E_BOUNDS))
        }
    }

    fn HasCurrent(&self) -> Result<bool> {
        let mut current = self.current.read().unwrap().clone().peekable();
        Ok(current.peek().is_some())
    }

    fn MoveNext(&self) -> Result<bool> {
        let mut current = self.current.write().unwrap();
        current.next();
        Ok(current.clone().peekable().peek().is_some())
    }

    fn GetMany(&self, pairs: &mut [Option<IKeyValuePair<K, V>>]) -> Result<u32> {
        let mut current = self.current.write().unwrap();
        let mut actual = 0;

        for pair in pairs {
            if let Some((key, value)) = current.next() {
                *pair = Some(
                    ComObject::new(StockKeyValuePair {
                        key: (*key).clone(),
                        value: (*value).clone(),
                    })
                    .into_interface(),
                );
                actual += 1;
            } else {
                break;
            }
        }

        Ok(actual)
    }
}

#[implement(IKeyValuePair<K, V>)]
struct StockKeyValuePair<K, V>
where
    K: RuntimeType + 'static,
    V: RuntimeType + 'static,
    K::Default: Clone,
    V::Default: Clone,
{
    key: K::Default,
    value: V::Default,
}

impl<K, V> IKeyValuePair_Impl<K, V> for StockKeyValuePair_Impl<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone,
    V::Default: Clone,
{
    fn Key(&self) -> Result<K> {
        K::from_default(&self.key)
    }

    fn Value(&self) -> Result<V> {
        V::from_default(&self.value)
    }
}

impl<K, V> From<std::collections::BTreeMap<K::Default, V::Default>> for IMapView<K, V>
where
    K: RuntimeType,
    V: RuntimeType,
    K::Default: Clone + Ord,
    V::Default: Clone,
{
    fn from(map: std::collections::BTreeMap<K::Default, V::Default>) -> Self {
        StockMapView { map }.into()
    }
}
