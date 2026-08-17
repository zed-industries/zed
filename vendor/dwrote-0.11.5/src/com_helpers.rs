// This is only handy for implementing a single-interface-implementing IUnknown.

macro_rules! implement_iunknown {
    ($interface:ident, $typ:ident) => {
        IUnknownVtbl {
            QueryInterface: {
                #[allow(non_snake_case)]
                unsafe extern "system" fn QueryInterface(
                    unknown_this: *mut IUnknown,
                    riid: REFIID,
                    ppv_object: *mut *mut c_void,
                ) -> HRESULT {
                    use $crate::winapi::Interface;
                    let this = if $crate::winapi::shared::guiddef::IsEqualGUID(
                        &*riid,
                        &$interface::uuidof(),
                    ) {
                        mem::transmute(unknown_this)
                    } else if $crate::winapi::shared::guiddef::IsEqualGUID(
                        &*riid,
                        &IUnknown::uuidof(),
                    ) {
                        mem::transmute(unknown_this)
                    } else {
                        return $crate::winapi::shared::winerror::E_NOINTERFACE;
                    };

                    (*unknown_this).AddRef();
                    *ppv_object = this;
                    return S_OK;
                }
                QueryInterface
            },
            AddRef: {
                unsafe extern "system" fn AddRef(unknown_this: *mut IUnknown) -> ULONG {
                    let this = $typ::from_interface(unknown_this);
                    let count = this.refcount.fetch_add(1, atomic::Ordering::Relaxed) + 1;
                    count as ULONG
                }
                AddRef
            },
            Release: {
                unsafe extern "system" fn Release(unknown_this: *mut IUnknown) -> ULONG {
                    let this = $typ::from_interface(unknown_this);
                    let count = this.refcount.fetch_sub(1, atomic::Ordering::Release) - 1;
                    if count == 0 {
                        <$typ as Com<$interface>>::destroy(unknown_this as *mut $interface);
                    }
                    count as ULONG
                }
                Release
            },
        }
    };
    (static $interface:ident, $typ:ident) => {
        IUnknownVtbl {
            QueryInterface: {
                #[allow(non_snake_case)]
                unsafe extern "system" fn QueryInterface(
                    unknown_this: *mut IUnknown,
                    riid: REFIID,
                    ppvObject: *mut *mut $crate::winapi::ctypes::c_void,
                ) -> HRESULT {
                    use $crate::winapi::Interface;
                    let this = if $crate::winapi::shared::guiddef::IsEqualGUID(
                        &*riid,
                        &$interface::uuidof(),
                    ) {
                        mem::transmute(unknown_this)
                    } else if $crate::winapi::shared::guiddef::IsEqualGUID(
                        &*riid,
                        &IUnknown::uuidof(),
                    ) {
                        mem::transmute(unknown_this)
                    } else {
                        return $crate::winapi::shared::winerror::E_NOINTERFACE;
                    };

                    (*unknown_this).AddRef();
                    *ppvObject = this;
                    return S_OK;
                }
                QueryInterface
            },
            AddRef: {
                // FIXME(pcwalton): Uh? Maybe we should actually reference count?
                #[allow(non_snake_case)]
                unsafe extern "system" fn AddRef(_: *mut IUnknown) -> ULONG {
                    1
                }
                AddRef
            },
            Release: {
                #[allow(non_snake_case)]
                unsafe extern "system" fn Release(_: *mut IUnknown) -> ULONG {
                    1
                }
                Release
            },
        }
    };
}

#[repr(C)]
pub struct ComRepr<Type, Vtbl>(*const Vtbl, Type);

pub trait Com<Interface>
where
    Self: Sized,
{
    type Vtbl: 'static;

    fn vtbl() -> &'static Self::Vtbl;

    fn into_interface(self) -> *mut Interface {
        let com = Box::new(ComRepr(Self::vtbl(), self));
        Box::into_raw(com) as *mut Interface
    }

    unsafe fn from_interface<'a>(thing: *mut Interface) -> &'a mut Self {
        &mut (*(thing as *mut ComRepr<Self, Self::Vtbl>)).1
    }

    unsafe fn destroy(thing: *mut Interface) {
        let _ = Box::from_raw(thing as *mut ComRepr<Self, Self::Vtbl>);
    }
}
