#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IIterable<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IIterable<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IIterable<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IIterable<T> {}
impl<T: windows_core::RuntimeType + 'static> IIterable<T> {
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for &IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterable<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{faa585ea-6214-4217-afda-7f46de5869b3}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")") };
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterable<T> {
    type Vtable = IIterable_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IIterable_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IIterator<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IIterator<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IIterator<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IIterator<T> {}
impl<T: windows_core::RuntimeType + 'static> IIterator<T> {
    pub fn Current(&self) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
}
impl<T: windows_core::RuntimeType> Iterator for IIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.Current().ok();
        if result.is_some() {
            self.MoveNext().ok()?;
        }
        result
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterator<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{6a79e863-4300-459a-9966-cbb660963ee1}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")") };
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterator<T> {
    type Vtable = IIterator_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IIterator_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub HasCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::AbiType<T>, *mut u32) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IKeyValuePair<K, V>(windows_core::IUnknown, core::marker::PhantomData<K>, core::marker::PhantomData<V>)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> core::ops::Deref for IKeyValuePair<K, V> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IKeyValuePair<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IKeyValuePair<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IKeyValuePair<K, V> {
    pub fn Key(&self) -> windows_core::Result<K> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Key)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<V> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IKeyValuePair<K, V> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{02b51929-c1c4-4a7e-8940-0312b5c18500}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b";").push_other(V::SIGNATURE).push_slice(b")") };
}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::Interface for IKeyValuePair<K, V> {
    type Vtable = IKeyValuePair_Vtbl<K, V>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IKeyValuePair_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<K>) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<V>) -> windows_core::HRESULT,
    pub K: core::marker::PhantomData<K>,
    pub V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IMap<K, V>(windows_core::IUnknown, core::marker::PhantomData<K>, core::marker::PhantomData<V>)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> core::ops::Deref for IMap<K, V> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IMap<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IMap<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<IKeyValuePair<K, V>>> for IMap<K, V> {
    const QUERY: bool = true;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IMap<K, V> {
    pub fn Lookup<P0>(&self, key: P0) -> windows_core::Result<V>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<K, V>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P0, P1>(&self, key: P0, value: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
        P1: windows_core::Param<V>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), key.param().abi(), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), key.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for IMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for &IMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IMap<K, V> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{3c2925fe-8519-45c1-aa79-197b6718c1c1}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b";").push_other(V::SIGNATURE).push_slice(b")") };
}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::Interface for IMap<K, V> {
    type Vtable = IMap_Vtbl<K, V>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IMap_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>, *mut windows_core::AbiType<V>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub HasKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>, *mut bool) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>, windows_core::AbiType<V>, *mut bool) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub K: core::marker::PhantomData<K>,
    pub V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IMapChangedEventArgs<K>(windows_core::IUnknown, core::marker::PhantomData<K>)
where
    K: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static> core::ops::Deref for IMapChangedEventArgs<K> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<K: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IMapChangedEventArgs<K> {}
impl<K: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IMapChangedEventArgs<K> {}
impl<K: windows_core::RuntimeType + 'static> IMapChangedEventArgs<K> {
    pub fn CollectionChange(&self) -> windows_core::Result<CollectionChange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CollectionChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Key(&self) -> windows_core::Result<K> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Key)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IMapChangedEventArgs<K> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9939f4df-050a-4c0f-aa60-77075f9c4777}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b")") };
}
unsafe impl<K: windows_core::RuntimeType + 'static> windows_core::Interface for IMapChangedEventArgs<K> {
    type Vtable = IMapChangedEventArgs_Vtbl<K>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IMapChangedEventArgs_Vtbl<K>
where
    K: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub CollectionChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CollectionChange) -> windows_core::HRESULT,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<K>) -> windows_core::HRESULT,
    pub K: core::marker::PhantomData<K>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IMapView<K, V>(windows_core::IUnknown, core::marker::PhantomData<K>, core::marker::PhantomData<V>)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> core::ops::Deref for IMapView<K, V> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IMapView<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IMapView<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<IKeyValuePair<K, V>>> for IMapView<K, V> {
    const QUERY: bool = true;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IMapView<K, V> {
    pub fn Lookup<P0>(&self, key: P0) -> windows_core::Result<V>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Split(&self, first: &mut Option<IMapView<K, V>>, second: &mut Option<IMapView<K, V>>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Split)(windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for IMapView<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for &IMapView<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IMapView<K, V> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{e480ce40-a338-4ada-adcf-272272e48cb9}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b";").push_other(V::SIGNATURE).push_slice(b")") };
}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::Interface for IMapView<K, V> {
    type Vtable = IMapView_Vtbl<K, V>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IMapView_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>, *mut windows_core::AbiType<V>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub HasKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<K>, *mut bool) -> windows_core::HRESULT,
    pub Split: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub K: core::marker::PhantomData<K>,
    pub V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IObservableMap<K, V>(windows_core::IUnknown, core::marker::PhantomData<K>, core::marker::PhantomData<V>)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> core::ops::Deref for IObservableMap<K, V> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IObservableMap<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IObservableMap<K, V> {}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<IKeyValuePair<K, V>>> for IObservableMap<K, V> {
    const QUERY: bool = true;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IMap<K, V>> for IObservableMap<K, V> {
    const QUERY: bool = true;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IObservableMap<K, V> {
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<MapChangedEventHandler<K, V>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup<P0>(&self, key: P0) -> windows_core::Result<V>
    where
        P0: windows_core::Param<K>,
    {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
    {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<K, V>> {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P0, P1>(&self, key: P0, value: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
        P1: windows_core::Param<V>,
    {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), key.param().abi(), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<K>,
    {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), key.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<K, V>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for IObservableMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator for &IObservableMap<K, V> {
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IObservableMap<K, V> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{65df2bf5-bf39-41b5-aebc-5a9d865e472b}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b";").push_other(V::SIGNATURE).push_slice(b")") };
}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::Interface for IObservableMap<K, V> {
    type Vtable = IObservableMap_Vtbl<K, V>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IObservableMap_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub MapChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveMapChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::EventRegistrationToken) -> windows_core::HRESULT,
    pub K: core::marker::PhantomData<K>,
    pub V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IObservableVector<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IObservableVector<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IObservableVector<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IObservableVector<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>> for IObservableVector<T> {
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IVector<T>> for IObservableVector<T> {
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IObservableVector<T> {
    pub fn VectorChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<VectorChangedEventHandler<T>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VectorChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveVectorChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveVectorChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = &windows_core::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IVectorView<T>> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<T>,
    {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[<T as windows_core::Type<T>>::Default]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IVector<T>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for IObservableVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IObservableVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorIterator::new(windows_core::Interface::cast(self).ok())
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IObservableVector<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{5917eb53-50b4-4a0d-b309-65862b3f1dbc}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")") };
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IObservableVector<T> {
    type Vtable = IObservableVector_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IObservableVector_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub VectorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveVectorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::EventRegistrationToken) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
windows_core::imp::define_interface!(IPropertySet, IPropertySet_Vtbl, 0x8a43ed9f_f4e6_4421_acf9_1dab2986820c);
impl core::ops::Deref for IPropertySet {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPropertySet, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPropertySet, IIterable < IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > >, IMap < windows_core::HSTRING , windows_core::IInspectable >, IObservableMap < windows_core::HSTRING , windows_core::IInspectable >);
impl IPropertySet {
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P0>(&self, key: &windows_core::HSTRING, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<MapChangedEventHandler<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl IntoIterator for IPropertySet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &IPropertySet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl windows_core::RuntimeType for IPropertySet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPropertySet_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IVector<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IVector<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IVector<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IVector<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>> for IVector<T> {
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVector<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IVectorView<T>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P0>(&self, index: u32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertAt)(windows_core::Interface::as_raw(this), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAt)(windows_core::Interface::as_raw(this), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[<T as windows_core::Type<T>>::Default]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReplaceAll)(windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = &windows_core::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
pub struct VectorIterator<T: windows_core::RuntimeType + 'static> {
    vector: Option<IVector<T>>,
    current: u32,
}
impl<T: windows_core::RuntimeType> VectorIterator<T> {
    pub fn new(vector: Option<IVector<T>>) -> Self {
        Self { vector, current: 0 }
    }
}
impl<T: windows_core::RuntimeType> Iterator for VectorIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.vector.as_ref().and_then(|vector| vector.GetAt(self.current).ok()).and_then(|result| {
            self.current += 1;
            Some(result)
        })
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for IVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for &IVector<T> {
    type Item = T;
    type IntoIter = VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorIterator::new(Some(Clone::clone(self)))
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVector<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{913337e9-11a1-4345-a3a2-4e7f956e222d}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")") };
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVector<T> {
    type Vtable = IVector_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IVector_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<T>, *mut u32, *mut bool) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut windows_core::AbiType<T>, *mut u32) -> windows_core::HRESULT,
    pub ReplaceAll: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
windows_core::imp::define_interface!(IVectorChangedEventArgs, IVectorChangedEventArgs_Vtbl, 0x575933df_34fe_4480_af15_07691f3d5d9b);
impl core::ops::Deref for IVectorChangedEventArgs {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVectorChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl IVectorChangedEventArgs {
    pub fn CollectionChange(&self) -> windows_core::Result<CollectionChange> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CollectionChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Index(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Index)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for IVectorChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVectorChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CollectionChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CollectionChange) -> windows_core::HRESULT,
    pub Index: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IVectorView<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IVectorView<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IVectorView<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IVectorView<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>> for IVectorView<T> {
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVectorView<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = &windows_core::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
pub struct VectorViewIterator<T: windows_core::RuntimeType + 'static> {
    vector: Option<IVectorView<T>>,
    current: u32,
}
impl<T: windows_core::RuntimeType> VectorViewIterator<T> {
    pub fn new(vector: Option<IVectorView<T>>) -> Self {
        Self { vector, current: 0 }
    }
}
impl<T: windows_core::RuntimeType> Iterator for VectorViewIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.vector.as_ref().and_then(|vector| vector.GetAt(self.current).ok()).and_then(|result| {
            self.current += 1;
            Some(result)
        })
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for IVectorView<T> {
    type Item = T;
    type IntoIter = VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for &IVectorView<T> {
    type Item = T;
    type IntoIter = VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        VectorViewIterator::new(Some(Clone::clone(self)))
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVectorView<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")") };
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVectorView<T> {
    type Vtable = IVectorView_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
pub struct IVectorView_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::AbiType<T>) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::AbiType<T>, *mut u32, *mut bool) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut windows_core::AbiType<T>, *mut u32) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PropertySet(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PropertySet, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PropertySet, IIterable < IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > >, IMap < windows_core::HSTRING , windows_core::IInspectable >, IObservableMap < windows_core::HSTRING , windows_core::IInspectable >, IPropertySet);
impl PropertySet {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PropertySet, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P0>(&self, key: &windows_core::HSTRING, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<MapChangedEventHandler<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for PropertySet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPropertySet>();
}
unsafe impl windows_core::Interface for PropertySet {
    type Vtable = IPropertySet_Vtbl;
    const IID: windows_core::GUID = <IPropertySet as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PropertySet {
    const NAME: &'static str = "Windows.Foundation.Collections.PropertySet";
}
impl IntoIterator for PropertySet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &PropertySet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl Send for PropertySet {}
unsafe impl Sync for PropertySet {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StringMap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StringMap, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StringMap, IIterable < IKeyValuePair < windows_core::HSTRING , windows_core::HSTRING > >, IMap < windows_core::HSTRING , windows_core::HSTRING >, IObservableMap < windows_core::HSTRING , windows_core::HSTRING >);
impl StringMap {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StringMap, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert(&self, key: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<MapChangedEventHandler<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::HSTRING>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for StringMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMap<windows_core::HSTRING, windows_core::HSTRING>>();
}
unsafe impl windows_core::Interface for StringMap {
    type Vtable = IMap_Vtbl<windows_core::HSTRING, windows_core::HSTRING>;
    const IID: windows_core::GUID = <IMap<windows_core::HSTRING, windows_core::HSTRING> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StringMap {
    const NAME: &'static str = "Windows.Foundation.Collections.StringMap";
}
impl IntoIterator for StringMap {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &StringMap {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::HSTRING>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl Send for StringMap {}
unsafe impl Sync for StringMap {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ValueSet(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ValueSet, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ValueSet, IIterable < IKeyValuePair < windows_core::HSTRING , windows_core::IInspectable > >, IMap < windows_core::HSTRING , windows_core::IInspectable >, IObservableMap < windows_core::HSTRING , windows_core::IInspectable >, IPropertySet);
impl ValueSet {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ValueSet, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Insert<P0>(&self, key: &windows_core::HSTRING, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove(&self, key: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn MapChanged<P0>(&self, vhnd: P0) -> windows_core::Result<super::EventRegistrationToken>
    where
        P0: windows_core::Param<MapChangedEventHandler<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MapChanged)(windows_core::Interface::as_raw(this), vhnd.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveMapChanged(&self, token: super::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IObservableMap<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveMapChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for ValueSet {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPropertySet>();
}
unsafe impl windows_core::Interface for ValueSet {
    type Vtable = IPropertySet_Vtbl;
    const IID: windows_core::GUID = <IPropertySet as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ValueSet {
    const NAME: &'static str = "Windows.Foundation.Collections.ValueSet";
}
impl IntoIterator for ValueSet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &ValueSet {
    type Item = IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl Send for ValueSet {}
unsafe impl Sync for ValueSet {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CollectionChange(pub i32);
impl CollectionChange {
    pub const Reset: Self = Self(0i32);
    pub const ItemInserted: Self = Self(1i32);
    pub const ItemRemoved: Self = Self(2i32);
    pub const ItemChanged: Self = Self(3i32);
}
impl windows_core::TypeKind for CollectionChange {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CollectionChange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CollectionChange").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CollectionChange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Collections.CollectionChange;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MapChangedEventHandler<K, V>(windows_core::IUnknown, core::marker::PhantomData<K>, core::marker::PhantomData<V>)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> MapChangedEventHandler<K, V> {
    pub fn new<F: FnMut(Option<&IObservableMap<K, V>>, Option<&IMapChangedEventArgs<K>>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = MapChangedEventHandlerBox::<K, V, F> { vtable: &MapChangedEventHandlerBox::<K, V, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, event: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObservableMap<K, V>>,
        P1: windows_core::Param<IMapChangedEventArgs<K>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), event.param().abi()).ok() }
    }
}
#[repr(C)]
struct MapChangedEventHandlerBox<K, V, F: FnMut(Option<&IObservableMap<K, V>>, Option<&IMapChangedEventArgs<K>>) -> windows_core::Result<()> + Send + 'static>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    vtable: *const MapChangedEventHandler_Vtbl<K, V>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, F: FnMut(Option<&IObservableMap<K, V>>, Option<&IMapChangedEventArgs<K>>) -> windows_core::Result<()> + Send + 'static> MapChangedEventHandlerBox<K, V, F> {
    const VTABLE: MapChangedEventHandler_Vtbl<K, V> = MapChangedEventHandler_Vtbl::<K, V> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        K: core::marker::PhantomData::<K>,
        V: core::marker::PhantomData::<V>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <MapChangedEventHandler<K, V> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&event)).into()
    }
}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::Interface for MapChangedEventHandler<K, V> {
    type Vtable = MapChangedEventHandler_Vtbl<K, V>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeType for MapChangedEventHandler<K, V> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{179517f3-94ee-41f8-bddc-768a895544f3}").push_slice(b";").push_other(K::SIGNATURE).push_slice(b";").push_other(V::SIGNATURE).push_slice(b")") };
}
#[repr(C)]
pub struct MapChangedEventHandler_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub K: core::marker::PhantomData<K>,
    pub V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VectorChangedEventHandler<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> VectorChangedEventHandler<T> {
    pub fn new<F: FnMut(Option<&IObservableVector<T>>, Option<&IVectorChangedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = VectorChangedEventHandlerBox::<T, F> { vtable: &VectorChangedEventHandlerBox::<T, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, event: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IObservableVector<T>>,
        P1: windows_core::Param<IVectorChangedEventArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), event.param().abi()).ok() }
    }
}
#[repr(C)]
struct VectorChangedEventHandlerBox<T, F: FnMut(Option<&IObservableVector<T>>, Option<&IVectorChangedEventArgs>) -> windows_core::Result<()> + Send + 'static>
where
    T: windows_core::RuntimeType + 'static,
{
    vtable: *const VectorChangedEventHandler_Vtbl<T>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<T: windows_core::RuntimeType + 'static, F: FnMut(Option<&IObservableVector<T>>, Option<&IVectorChangedEventArgs>) -> windows_core::Result<()> + Send + 'static> VectorChangedEventHandlerBox<T, F> {
    const VTABLE: VectorChangedEventHandler_Vtbl<T> = VectorChangedEventHandler_Vtbl::<T> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        T: core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <VectorChangedEventHandler<T> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender), windows_core::from_raw_borrowed(&event)).into()
    }
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for VectorChangedEventHandler<T> {
    type Vtable = VectorChangedEventHandler_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for VectorChangedEventHandler<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = { windows_core::imp::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{0c051752-9fbf-4c70-aa0c-0e4c82d9a761}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")") };
}
#[repr(C)]
pub struct VectorChangedEventHandler_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub T: core::marker::PhantomData<T>,
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
