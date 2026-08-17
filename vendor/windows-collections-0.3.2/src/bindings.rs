#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IIterable<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IIterable<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IIterable<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterable<T> {
    type Vtable = IIterable_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterable<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({faa585ea-6214-4217-afda-7f46de5869b3}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> IIterable<T> {
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterable<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterable";
}
pub trait IIterable_Impl<T>: windows_core::IUnknownImpl
where
    T: windows_core::RuntimeType + 'static,
{
    fn First(&self) -> windows_core::Result<IIterator<T>>;
}
impl<T: windows_core::RuntimeType + 'static> IIterable_Vtbl<T> {
    pub const fn new<Identity: IIterable_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn First<
            T: windows_core::RuntimeType + 'static,
            Identity: IIterable_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterable_Impl::First(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IIterable<T>, OFFSET>(),
            First: First::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIterable<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIterable_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
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
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IIterator<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IIterator<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IIterator<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterator<T> {
    type Vtable = IIterator_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterator<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({6a79e863-4300-459a-9966-cbb660963ee1}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> IIterator<T> {
    pub fn Current(&self) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetMany(
        &self,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(
                windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                core::mem::transmute_copy(&items),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterator<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterator";
}
pub trait IIterator_Impl<T>: windows_core::IUnknownImpl
where
    T: windows_core::RuntimeType + 'static,
{
    fn Current(&self) -> windows_core::Result<T>;
    fn HasCurrent(&self) -> windows_core::Result<bool>;
    fn MoveNext(&self) -> windows_core::Result<bool>;
    fn GetMany(
        &self,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32>;
}
impl<T: windows_core::RuntimeType + 'static> IIterator_Vtbl<T> {
    pub const fn new<Identity: IIterator_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Current<
            T: windows_core::RuntimeType + 'static,
            Identity: IIterator_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterator_Impl::Current(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasCurrent<
            T: windows_core::RuntimeType + 'static,
            Identity: IIterator_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterator_Impl::HasCurrent(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveNext<
            T: windows_core::RuntimeType + 'static,
            Identity: IIterator_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterator_Impl::MoveNext(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMany<
            T: windows_core::RuntimeType + 'static,
            Identity: IIterator_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            items_array_size: u32,
            items: *mut T,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterator_Impl::GetMany(
                    this,
                    core::slice::from_raw_parts_mut(
                        core::mem::transmute_copy(&items),
                        items_array_size as usize,
                    ),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IIterator<T>, OFFSET>(),
            Current: Current::<T, Identity, OFFSET>,
            HasCurrent: HasCurrent::<T, Identity, OFFSET>,
            MoveNext: MoveNext::<T, Identity, OFFSET>,
            GetMany: GetMany::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIterator<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIterator_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub HasCurrent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MoveNext:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut T,
        *mut u32,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
impl<T: windows_core::RuntimeType> Iterator for IIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.HasCurrent().unwrap_or(false) {
            self.Current().ok()
        } else {
            None
        };
        if result.is_some() {
            self.MoveNext().ok()?;
        }
        result
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IKeyValuePair<K, V>(
    windows_core::IUnknown,
    core::marker::PhantomData<K>,
    core::marker::PhantomData<V>,
)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IUnknown> for IKeyValuePair<K, V>
{
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IInspectable> for IKeyValuePair<K, V>
{
}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::Interface for IKeyValuePair<K, V>
{
    type Vtable = IKeyValuePair_Vtbl<K, V>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::RuntimeType for IKeyValuePair<K, V>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500}")
        .push_slice(b";")
        .push_other(K::SIGNATURE)
        .push_slice(b";")
        .push_other(V::SIGNATURE)
        .push_slice(b")");
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    IKeyValuePair<K, V>
{
    pub fn Key(&self) -> windows_core::Result<K> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Key)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<V> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::RuntimeName for IKeyValuePair<K, V>
{
    const NAME: &'static str = "Windows.Foundation.Collections.IKeyValuePair";
}
pub trait IKeyValuePair_Impl<K, V>: windows_core::IUnknownImpl
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn Key(&self) -> windows_core::Result<K>;
    fn Value(&self) -> windows_core::Result<V>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    IKeyValuePair_Vtbl<K, V>
{
    pub const fn new<Identity: IKeyValuePair_Impl<K, V>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Key<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IKeyValuePair_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::AbiType<K>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyValuePair_Impl::Key(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IKeyValuePair_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::AbiType<V>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyValuePair_Impl::Value(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IKeyValuePair<K, V>, OFFSET>(),
            Key: Key::<K, V, Identity, OFFSET>,
            Value: Value::<K, V, Identity, OFFSET>,
            K: core::marker::PhantomData::<K>,
            V: core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKeyValuePair<K, V> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyValuePair_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Key: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<K>,
    ) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<V>,
    ) -> windows_core::HRESULT,
    K: core::marker::PhantomData<K>,
    V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IMap<K, V>(
    windows_core::IUnknown,
    core::marker::PhantomData<K>,
    core::marker::PhantomData<V>,
)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IUnknown> for IMap<K, V>
{
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IInspectable> for IMap<K, V>
{
}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::Interface for IMap<K, V>
{
    type Vtable = IMap_Vtbl<K, V>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::RuntimeType for IMap<K, V>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1}")
        .push_slice(b";")
        .push_other(K::SIGNATURE)
        .push_slice(b";")
        .push_other(V::SIGNATURE)
        .push_slice(b")");
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<IIterable<IKeyValuePair<K, V>>> for IMap<K, V>
{
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
            (windows_core::Interface::vtable(this).Lookup)(
                windows_core::Interface::as_raw(this),
                key.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(
                windows_core::Interface::as_raw(this),
                key.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IMapView<K, V>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(this).Insert)(
                windows_core::Interface::as_raw(this),
                key.param().abi(),
                value.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Remove<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Remove)(
                windows_core::Interface::as_raw(this),
                key.param().abi(),
            )
            .ok()
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator
    for IMap<K, V>
{
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator
    for &IMap<K, V>
{
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::RuntimeName for IMap<K, V>
{
    const NAME: &'static str = "Windows.Foundation.Collections.IMap";
}
pub trait IMap_Impl<K, V>: IIterable_Impl<IKeyValuePair<K, V>>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn Lookup(&self, key: windows_core::Ref<K>) -> windows_core::Result<V>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn HasKey(&self, key: windows_core::Ref<K>) -> windows_core::Result<bool>;
    fn GetView(&self) -> windows_core::Result<IMapView<K, V>>;
    fn Insert(
        &self,
        key: windows_core::Ref<K>,
        value: windows_core::Ref<V>,
    ) -> windows_core::Result<bool>;
    fn Remove(&self, key: windows_core::Ref<K>) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    IMap_Vtbl<K, V>
{
    pub const fn new<Identity: IMap_Impl<K, V>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lookup<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMap_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            key: windows_core::AbiType<K>,
            result__: *mut windows_core::AbiType<V>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMap_Impl::Lookup(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMap_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMap_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasKey<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMap_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            key: windows_core::AbiType<K>,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMap_Impl::HasKey(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetView<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMap_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMap_Impl::GetView(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Insert<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMap_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            key: windows_core::AbiType<K>,
            value: windows_core::AbiType<V>,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMap_Impl::Insert(
                    this,
                    core::mem::transmute_copy(&key),
                    core::mem::transmute_copy(&value),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMap_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            key: windows_core::AbiType<K>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMap_Impl::Remove(this, core::mem::transmute_copy(&key)).into()
            }
        }
        unsafe extern "system" fn Clear<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMap_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMap_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMap<K, V>, OFFSET>(),
            Lookup: Lookup::<K, V, Identity, OFFSET>,
            Size: Size::<K, V, Identity, OFFSET>,
            HasKey: HasKey::<K, V, Identity, OFFSET>,
            GetView: GetView::<K, V, Identity, OFFSET>,
            Insert: Insert::<K, V, Identity, OFFSET>,
            Remove: Remove::<K, V, Identity, OFFSET>,
            Clear: Clear::<K, V, Identity, OFFSET>,
            K: core::marker::PhantomData::<K>,
            V: core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMap<K, V> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMap_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<K>,
        *mut windows_core::AbiType<V>,
    ) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub HasKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<K>,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<K>,
        windows_core::AbiType<V>,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<K>,
    ) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    K: core::marker::PhantomData<K>,
    V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IMapView<K, V>(
    windows_core::IUnknown,
    core::marker::PhantomData<K>,
    core::marker::PhantomData<V>,
)
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static;
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IUnknown> for IMapView<K, V>
{
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IInspectable> for IMapView<K, V>
{
}
unsafe impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::Interface for IMapView<K, V>
{
    type Vtable = IMapView_Vtbl<K, V>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::RuntimeType for IMapView<K, V>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({e480ce40-a338-4ada-adcf-272272e48cb9}")
        .push_slice(b";")
        .push_other(K::SIGNATURE)
        .push_slice(b";")
        .push_other(V::SIGNATURE)
        .push_slice(b")");
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<IIterable<IKeyValuePair<K, V>>> for IMapView<K, V>
{
    const QUERY: bool = true;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    IMapView<K, V>
{
    pub fn Lookup<P0>(&self, key: P0) -> windows_core::Result<V>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(
                windows_core::Interface::as_raw(this),
                key.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<K>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(
                windows_core::Interface::as_raw(this),
                key.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Split(
        &self,
        first: &mut Option<IMapView<K, V>>,
        second: &mut Option<IMapView<K, V>>,
    ) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Split)(
                windows_core::Interface::as_raw(this),
                first as *mut _ as _,
                second as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<IKeyValuePair<K, V>>> {
        let this = &windows_core::Interface::cast::<IIterable<IKeyValuePair<K, V>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator
    for IMapView<K, V>
{
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IntoIterator
    for &IMapView<K, V>
{
    type Item = IKeyValuePair<K, V>;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::RuntimeName for IMapView<K, V>
{
    const NAME: &'static str = "Windows.Foundation.Collections.IMapView";
}
pub trait IMapView_Impl<K, V>: IIterable_Impl<IKeyValuePair<K, V>>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn Lookup(&self, key: windows_core::Ref<K>) -> windows_core::Result<V>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn HasKey(&self, key: windows_core::Ref<K>) -> windows_core::Result<bool>;
    fn Split(
        &self,
        first: windows_core::OutRef<IMapView<K, V>>,
        second: windows_core::OutRef<IMapView<K, V>>,
    ) -> windows_core::Result<()>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    IMapView_Vtbl<K, V>
{
    pub const fn new<Identity: IMapView_Impl<K, V>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lookup<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMapView_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            key: windows_core::AbiType<K>,
            result__: *mut windows_core::AbiType<V>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMapView_Impl::Lookup(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMapView_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMapView_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasKey<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMapView_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            key: windows_core::AbiType<K>,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMapView_Impl::HasKey(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Split<
            K: windows_core::RuntimeType + 'static,
            V: windows_core::RuntimeType + 'static,
            Identity: IMapView_Impl<K, V>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            first: *mut *mut core::ffi::c_void,
            second: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMapView_Impl::Split(
                    this,
                    core::mem::transmute_copy(&first),
                    core::mem::transmute_copy(&second),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMapView<K, V>, OFFSET>(),
            Lookup: Lookup::<K, V, Identity, OFFSET>,
            Size: Size::<K, V, Identity, OFFSET>,
            HasKey: HasKey::<K, V, Identity, OFFSET>,
            Split: Split::<K, V, Identity, OFFSET>,
            K: core::marker::PhantomData::<K>,
            V: core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMapView<K, V> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMapView_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Lookup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<K>,
        *mut windows_core::AbiType<V>,
    ) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub HasKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<K>,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub Split: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    K: core::marker::PhantomData<K>,
    V: core::marker::PhantomData<V>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IVector<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IVector<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IVector<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVector<T> {
    type Vtable = IVector_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVector<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>>
    for IVector<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVector<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(
                windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<IVectorView<T>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
                index,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetAt)(
                windows_core::Interface::as_raw(this),
                index,
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).InsertAt)(
                windows_core::Interface::as_raw(this),
                index,
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAt)(
                windows_core::Interface::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Append)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(
                windows_core::Interface::as_raw(this),
                startindex,
                items.len().try_into().unwrap(),
                core::mem::transmute_copy(&items),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReplaceAll(
        &self,
        items: &[<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).ReplaceAll)(
                windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = &windows_core::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for IVector<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IVector<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVector";
}
pub trait IVector_Impl<T>: IIterable_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> windows_core::Result<T>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn GetView(&self) -> windows_core::Result<IVectorView<T>>;
    fn IndexOf(&self, value: windows_core::Ref<T>, index: &mut u32) -> windows_core::Result<bool>;
    fn SetAt(&self, index: u32, value: windows_core::Ref<T>) -> windows_core::Result<()>;
    fn InsertAt(&self, index: u32, value: windows_core::Ref<T>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn Append(&self, value: windows_core::Ref<T>) -> windows_core::Result<()>;
    fn RemoveAtEnd(&self) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn GetMany(
        &self,
        startIndex: u32,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32>;
    fn ReplaceAll(
        &self,
        items: &[<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<()>;
}
impl<T: windows_core::RuntimeType + 'static> IVector_Vtbl<T> {
    pub const fn new<Identity: IVector_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAt<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            result__: *mut windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVector_Impl::GetAt(this, index) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVector_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetView<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVector_Impl::GetView(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IndexOf<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_core::AbiType<T>,
            index: *mut u32,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVector_Impl::IndexOf(
                    this,
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&index),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAt<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            value: windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::SetAt(this, index, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn InsertAt<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            value: windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::InsertAt(this, index, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::RemoveAt(this, index).into()
            }
        }
        unsafe extern "system" fn Append<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::Append(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveAtEnd<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::RemoveAtEnd(this).into()
            }
        }
        unsafe extern "system" fn Clear<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn GetMany<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startindex: u32,
            items_array_size: u32,
            items: *mut T,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVector_Impl::GetMany(
                    this,
                    startindex,
                    core::slice::from_raw_parts_mut(
                        core::mem::transmute_copy(&items),
                        items_array_size as usize,
                    ),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReplaceAll<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            items_array_size: u32,
            items: *const T,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::ReplaceAll(
                    this,
                    core::slice::from_raw_parts(
                        core::mem::transmute_copy(&items),
                        items_array_size as usize,
                    ),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVector<T>, OFFSET>(),
            GetAt: GetAt::<T, Identity, OFFSET>,
            Size: Size::<T, Identity, OFFSET>,
            GetView: GetView::<T, Identity, OFFSET>,
            IndexOf: IndexOf::<T, Identity, OFFSET>,
            SetAt: SetAt::<T, Identity, OFFSET>,
            InsertAt: InsertAt::<T, Identity, OFFSET>,
            RemoveAt: RemoveAt::<T, Identity, OFFSET>,
            Append: Append::<T, Identity, OFFSET>,
            RemoveAtEnd: RemoveAtEnd::<T, Identity, OFFSET>,
            Clear: Clear::<T, Identity, OFFSET>,
            GetMany: GetMany::<T, Identity, OFFSET>,
            ReplaceAll: ReplaceAll::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVector<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVector_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<T>,
        *mut u32,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut T,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub ReplaceAll:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const T) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IVectorView<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IVectorView<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IVectorView<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVectorView<T> {
    type Vtable = IVectorView_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVectorView<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>>
    for IVectorView<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVectorView<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(
                windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
                index,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(
                windows_core::Interface::as_raw(this),
                startindex,
                items.len().try_into().unwrap(),
                core::mem::transmute_copy(&items),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = &windows_core::Interface::cast::<IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for IVectorView<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IVectorView<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVectorView<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorView";
}
pub trait IVectorView_Impl<T>: IIterable_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> windows_core::Result<T>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn IndexOf(&self, value: windows_core::Ref<T>, index: &mut u32) -> windows_core::Result<bool>;
    fn GetMany(
        &self,
        startIndex: u32,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32>;
}
impl<T: windows_core::RuntimeType + 'static> IVectorView_Vtbl<T> {
    pub const fn new<Identity: IVectorView_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAt<
            T: windows_core::RuntimeType + 'static,
            Identity: IVectorView_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            result__: *mut windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVectorView_Impl::GetAt(this, index) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<
            T: windows_core::RuntimeType + 'static,
            Identity: IVectorView_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVectorView_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IndexOf<
            T: windows_core::RuntimeType + 'static,
            Identity: IVectorView_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_core::AbiType<T>,
            index: *mut u32,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVectorView_Impl::IndexOf(
                    this,
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&index),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMany<
            T: windows_core::RuntimeType + 'static,
            Identity: IVectorView_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startindex: u32,
            items_array_size: u32,
            items: *mut T,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVectorView_Impl::GetMany(
                    this,
                    startindex,
                    core::slice::from_raw_parts_mut(
                        core::mem::transmute_copy(&items),
                        items_array_size as usize,
                    ),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVectorView<T>, OFFSET>(),
            GetAt: GetAt::<T, Identity, OFFSET>,
            Size: Size::<T, Identity, OFFSET>,
            IndexOf: IndexOf::<T, Identity, OFFSET>,
            GetMany: GetMany::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVectorView<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVectorView_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<T>,
        *mut u32,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut T,
        *mut u32,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
