pub trait IIterable_Impl<T>: Sized
where
    T: windows_core::RuntimeType + 'static,
{
    fn First(&self) -> windows_core::Result<IIterator<T>>;
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterable<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterable";
}
impl<T: windows_core::RuntimeType + 'static> IIterable_Vtbl<T> {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIterable_Vtbl<T>
    where
        Identity: IIterable_Impl<T>,
    {
        unsafe extern "system" fn First<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IIterable_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIterable_Impl::First(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IIterator_Impl<T>: Sized
where
    T: windows_core::RuntimeType + 'static,
{
    fn Current(&self) -> windows_core::Result<T>;
    fn HasCurrent(&self) -> windows_core::Result<bool>;
    fn MoveNext(&self) -> windows_core::Result<bool>;
    fn GetMany(&self, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32>;
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterator<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterator";
}
impl<T: windows_core::RuntimeType + 'static> IIterator_Vtbl<T> {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIterator_Vtbl<T>
    where
        Identity: IIterator_Impl<T>,
    {
        unsafe extern "system" fn Current<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<T>) -> windows_core::HRESULT
        where
            Identity: IIterator_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIterator_Impl::Current(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCurrent<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IIterator_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIterator_Impl::HasCurrent(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveNext<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IIterator_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIterator_Impl::MoveNext(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMany<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, items_array_size: u32, items: *mut windows_core::AbiType<T>, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IIterator_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIterator_Impl::GetMany(this, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&items), items_array_size as usize)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IKeyValuePair_Impl<K, V>: Sized
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn Key(&self) -> windows_core::Result<K>;
    fn Value(&self) -> windows_core::Result<V>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IKeyValuePair<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IKeyValuePair";
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IKeyValuePair_Vtbl<K, V> {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IKeyValuePair_Vtbl<K, V>
    where
        Identity: IKeyValuePair_Impl<K, V>,
    {
        unsafe extern "system" fn Key<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<K>) -> windows_core::HRESULT
        where
            Identity: IKeyValuePair_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKeyValuePair_Impl::Key(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<V>) -> windows_core::HRESULT
        where
            Identity: IKeyValuePair_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IKeyValuePair_Impl::Value(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IMap_Impl<K, V>: Sized + IIterable_Impl<IKeyValuePair<K, V>>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as windows_core::Type<K>>::Default) -> windows_core::Result<V>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn HasKey(&self, key: &<K as windows_core::Type<K>>::Default) -> windows_core::Result<bool>;
    fn GetView(&self) -> windows_core::Result<IMapView<K, V>>;
    fn Insert(&self, key: &<K as windows_core::Type<K>>::Default, value: &<V as windows_core::Type<V>>::Default) -> windows_core::Result<bool>;
    fn Remove(&self, key: &<K as windows_core::Type<K>>::Default) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IMap<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMap";
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IMap_Vtbl<K, V> {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMap_Vtbl<K, V>
    where
        Identity: IMap_Impl<K, V>,
    {
        unsafe extern "system" fn Lookup<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>, result__: *mut windows_core::AbiType<V>) -> windows_core::HRESULT
        where
            Identity: IMap_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMap_Impl::Lookup(this, core::mem::transmute(&key)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMap_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMap_Impl::Size(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKey<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IMap_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMap_Impl::HasKey(this, core::mem::transmute(&key)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMap_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMap_Impl::GetView(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Insert<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>, value: windows_core::AbiType<V>, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IMap_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMap_Impl::Insert(this, core::mem::transmute(&key), core::mem::transmute(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>) -> windows_core::HRESULT
        where
            Identity: IMap_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMap_Impl::Remove(this, core::mem::transmute(&key)).into()
        }
        unsafe extern "system" fn Clear<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMap_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMap_Impl::Clear(this).into()
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
pub trait IMapChangedEventArgs_Impl<K>: Sized
where
    K: windows_core::RuntimeType + 'static,
{
    fn CollectionChange(&self) -> windows_core::Result<CollectionChange>;
    fn Key(&self) -> windows_core::Result<K>;
}
impl<K: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IMapChangedEventArgs<K> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMapChangedEventArgs";
}
impl<K: windows_core::RuntimeType + 'static> IMapChangedEventArgs_Vtbl<K> {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMapChangedEventArgs_Vtbl<K>
    where
        Identity: IMapChangedEventArgs_Impl<K>,
    {
        unsafe extern "system" fn CollectionChange<K: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CollectionChange) -> windows_core::HRESULT
        where
            Identity: IMapChangedEventArgs_Impl<K>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMapChangedEventArgs_Impl::CollectionChange(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Key<K: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<K>) -> windows_core::HRESULT
        where
            Identity: IMapChangedEventArgs_Impl<K>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMapChangedEventArgs_Impl::Key(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMapChangedEventArgs<K>, OFFSET>(),
            CollectionChange: CollectionChange::<K, Identity, OFFSET>,
            Key: Key::<K, Identity, OFFSET>,
            K: core::marker::PhantomData::<K>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMapChangedEventArgs<K> as windows_core::Interface>::IID
    }
}
pub trait IMapView_Impl<K, V>: Sized + IIterable_Impl<IKeyValuePair<K, V>>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn Lookup(&self, key: &<K as windows_core::Type<K>>::Default) -> windows_core::Result<V>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn HasKey(&self, key: &<K as windows_core::Type<K>>::Default) -> windows_core::Result<bool>;
    fn Split(&self, first: &mut Option<IMapView<K, V>>, second: &mut Option<IMapView<K, V>>) -> windows_core::Result<()>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IMapView<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IMapView";
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IMapView_Vtbl<K, V> {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMapView_Vtbl<K, V>
    where
        Identity: IMapView_Impl<K, V>,
    {
        unsafe extern "system" fn Lookup<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>, result__: *mut windows_core::AbiType<V>) -> windows_core::HRESULT
        where
            Identity: IMapView_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMapView_Impl::Lookup(this, core::mem::transmute(&key)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMapView_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMapView_Impl::Size(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKey<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: windows_core::AbiType<K>, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IMapView_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMapView_Impl::HasKey(this, core::mem::transmute(&key)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Split<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, first: *mut *mut core::ffi::c_void, second: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMapView_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMapView_Impl::Split(this, core::mem::transmute_copy(&first), core::mem::transmute_copy(&second)).into()
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
pub trait IObservableMap_Impl<K, V>: Sized + IIterable_Impl<IKeyValuePair<K, V>> + IMap_Impl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    fn MapChanged(&self, vhnd: Option<&MapChangedEventHandler<K, V>>) -> windows_core::Result<super::EventRegistrationToken>;
    fn RemoveMapChanged(&self, token: &super::EventRegistrationToken) -> windows_core::Result<()>;
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IObservableMap<K, V> {
    const NAME: &'static str = "Windows.Foundation.Collections.IObservableMap";
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static> IObservableMap_Vtbl<K, V> {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObservableMap_Vtbl<K, V>
    where
        Identity: IObservableMap_Impl<K, V>,
    {
        unsafe extern "system" fn MapChanged<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vhnd: *mut core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IObservableMap_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObservableMap_Impl::MapChanged(this, windows_core::from_raw_borrowed(&vhnd)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMapChanged<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IObservableMap_Impl<K, V>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObservableMap_Impl::RemoveMapChanged(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IObservableMap<K, V>, OFFSET>(),
            MapChanged: MapChanged::<K, V, Identity, OFFSET>,
            RemoveMapChanged: RemoveMapChanged::<K, V, Identity, OFFSET>,
            K: core::marker::PhantomData::<K>,
            V: core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObservableMap<K, V> as windows_core::Interface>::IID
    }
}
pub trait IObservableVector_Impl<T>: Sized + IIterable_Impl<T> + IVector_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn VectorChanged(&self, vhnd: Option<&VectorChangedEventHandler<T>>) -> windows_core::Result<super::EventRegistrationToken>;
    fn RemoveVectorChanged(&self, token: &super::EventRegistrationToken) -> windows_core::Result<()>;
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IObservableVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IObservableVector";
}
impl<T: windows_core::RuntimeType + 'static> IObservableVector_Vtbl<T> {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObservableVector_Vtbl<T>
    where
        Identity: IObservableVector_Impl<T>,
    {
        unsafe extern "system" fn VectorChanged<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vhnd: *mut core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IObservableVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObservableVector_Impl::VectorChanged(this, windows_core::from_raw_borrowed(&vhnd)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVectorChanged<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: super::EventRegistrationToken) -> windows_core::HRESULT
        where
            Identity: IObservableVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObservableVector_Impl::RemoveVectorChanged(this, core::mem::transmute(&token)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IObservableVector<T>, OFFSET>(),
            VectorChanged: VectorChanged::<T, Identity, OFFSET>,
            RemoveVectorChanged: RemoveVectorChanged::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObservableVector<T> as windows_core::Interface>::IID
    }
}
pub trait IPropertySet_Impl: Sized + IIterable_Impl<IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>> + IMap_Impl<windows_core::HSTRING, windows_core::IInspectable> + IObservableMap_Impl<windows_core::HSTRING, windows_core::IInspectable> {}
impl windows_core::RuntimeName for IPropertySet {
    const NAME: &'static str = "Windows.Foundation.Collections.IPropertySet";
}
impl IPropertySet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPropertySet_Vtbl
    where
        Identity: IPropertySet_Impl,
    {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertySet, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySet as windows_core::Interface>::IID
    }
}
pub trait IVector_Impl<T>: Sized + IIterable_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> windows_core::Result<T>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn GetView(&self) -> windows_core::Result<IVectorView<T>>;
    fn IndexOf(&self, value: &<T as windows_core::Type<T>>::Default, index: &mut u32) -> windows_core::Result<bool>;
    fn SetAt(&self, index: u32, value: &<T as windows_core::Type<T>>::Default) -> windows_core::Result<()>;
    fn InsertAt(&self, index: u32, value: &<T as windows_core::Type<T>>::Default) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn Append(&self, value: &<T as windows_core::Type<T>>::Default) -> windows_core::Result<()>;
    fn RemoveAtEnd(&self) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn GetMany(&self, startindex: u32, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32>;
    fn ReplaceAll(&self, items: &[<T as windows_core::Type<T>>::Default]) -> windows_core::Result<()>;
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVector";
}
impl<T: windows_core::RuntimeType + 'static> IVector_Vtbl<T> {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVector_Vtbl<T>
    where
        Identity: IVector_Impl<T>,
    {
        unsafe extern "system" fn GetAt<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, result__: *mut windows_core::AbiType<T>) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVector_Impl::GetAt(this, index) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVector_Impl::Size(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVector_Impl::GetView(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::AbiType<T>, index: *mut u32, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVector_Impl::IndexOf(this, core::mem::transmute(&value), core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAt<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: windows_core::AbiType<T>) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::SetAt(this, index, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn InsertAt<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, value: windows_core::AbiType<T>) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::InsertAt(this, index, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAt<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::RemoveAt(this, index).into()
        }
        unsafe extern "system" fn Append<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::AbiType<T>) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::Append(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::RemoveAtEnd(this).into()
        }
        unsafe extern "system" fn Clear<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::Clear(this).into()
        }
        unsafe extern "system" fn GetMany<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut windows_core::AbiType<T>, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVector_Impl::GetMany(this, startindex, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&items), items_array_size as usize)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceAll<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, items_array_size: u32, items: *const windows_core::AbiType<T>) -> windows_core::HRESULT
        where
            Identity: IVector_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IVector_Impl::ReplaceAll(this, core::slice::from_raw_parts(core::mem::transmute_copy(&items), items_array_size as usize)).into()
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
pub trait IVectorChangedEventArgs_Impl: Sized {
    fn CollectionChange(&self) -> windows_core::Result<CollectionChange>;
    fn Index(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IVectorChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorChangedEventArgs";
}
impl IVectorChangedEventArgs_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVectorChangedEventArgs_Vtbl
    where
        Identity: IVectorChangedEventArgs_Impl,
    {
        unsafe extern "system" fn CollectionChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CollectionChange) -> windows_core::HRESULT
        where
            Identity: IVectorChangedEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorChangedEventArgs_Impl::CollectionChange(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Index<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVectorChangedEventArgs_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorChangedEventArgs_Impl::Index(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVectorChangedEventArgs, OFFSET>(),
            CollectionChange: CollectionChange::<Identity, OFFSET>,
            Index: Index::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVectorChangedEventArgs as windows_core::Interface>::IID
    }
}
pub trait IVectorView_Impl<T>: Sized + IIterable_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> windows_core::Result<T>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn IndexOf(&self, value: &<T as windows_core::Type<T>>::Default, index: &mut u32) -> windows_core::Result<bool>;
    fn GetMany(&self, startindex: u32, items: &mut [<T as windows_core::Type<T>>::Default]) -> windows_core::Result<u32>;
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVectorView<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorView";
}
impl<T: windows_core::RuntimeType + 'static> IVectorView_Vtbl<T> {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IVectorView_Vtbl<T>
    where
        Identity: IVectorView_Impl<T>,
    {
        unsafe extern "system" fn GetAt<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, result__: *mut windows_core::AbiType<T>) -> windows_core::HRESULT
        where
            Identity: IVectorView_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorView_Impl::GetAt(this, index) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVectorView_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorView_Impl::Size(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndexOf<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::AbiType<T>, index: *mut u32, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IVectorView_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorView_Impl::IndexOf(this, core::mem::transmute(&value), core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMany<T: windows_core::RuntimeType + 'static, Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, startindex: u32, items_array_size: u32, items: *mut windows_core::AbiType<T>, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IVectorView_Impl<T>,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IVectorView_Impl::GetMany(this, startindex, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&items), items_array_size as usize)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
