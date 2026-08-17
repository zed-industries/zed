use super::*;
use crate::{IUnknown, IUnknown_Vtbl, Interface, GUID, HRESULT};
use core::ffi::c_void;
use core::mem::{transmute, transmute_copy};
use core::ptr::null_mut;

windows_link::link!("combase.dll" "system" fn CoCreateFreeThreadedMarshaler(punkouter: *mut c_void, ppunkmarshal: *mut *mut c_void) -> HRESULT);

pub unsafe fn marshaler(outer: IUnknown, result: *mut *mut c_void) -> HRESULT {
    unsafe {
        let mut marshaler_raw = null_mut();
        _ = CoCreateFreeThreadedMarshaler(null_mut(), &mut marshaler_raw);
        assert!(!marshaler_raw.is_null(), "allocation failed");
        let marshaler: IUnknown = transmute(marshaler_raw);

        _ = (marshaler.vtable().QueryInterface)(
            transmute_copy(&marshaler),
            &IMarshal::IID,
            &mut marshaler_raw,
        );

        debug_assert!(!marshaler_raw.is_null());
        let marshaler: IMarshal = transmute(marshaler_raw);

        let marshaler = Marshaler {
            vtable: &Marshaler::VTABLE,
            outer,
            marshaler,
            count: RefCount::new(1),
        };

        debug_assert!(!result.is_null());
        *result = transmute::<Box<_>, *mut c_void>(Box::new(marshaler));
        S_OK
    }
}

#[repr(C)]
struct Marshaler {
    vtable: *const IMarshal_Vtbl,
    outer: IUnknown,
    marshaler: IMarshal,
    count: RefCount,
}

impl Marshaler {
    const VTABLE: IMarshal_Vtbl = IMarshal_Vtbl {
        base__: IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        GetUnmarshalClass: Self::GetUnmarshalClass,
        GetMarshalSizeMax: Self::GetMarshalSizeMax,
        MarshalInterface: Self::MarshalInterface,
        UnmarshalInterface: Self::UnmarshalInterface,
        ReleaseMarshalData: Self::ReleaseMarshalData,
        DisconnectObject: Self::DisconnectObject,
    };

    unsafe extern "system" fn QueryInterface(
        this: *mut c_void,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;

            if iid.is_null() || interface.is_null() {
                return E_POINTER;
            }

            if *iid == IMarshal::IID {
                *interface = &mut (*this).vtable as *mut _ as _;
                (*this).count.add_ref();
                return S_OK;
            }

            ((*this).outer.vtable().QueryInterface)(transmute_copy(&(*this).outer), iid, interface)
        }
    }

    unsafe extern "system" fn AddRef(this: *mut c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;
            (*this).count.add_ref()
        }
    }

    unsafe extern "system" fn Release(this: *mut c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;
            let remaining = (*this).count.release();

            if remaining == 0 {
                let _ = Box::from_raw(this);
            }

            remaining
        }
    }

    unsafe extern "system" fn GetUnmarshalClass(
        this: *mut c_void,
        riid: *const GUID,
        pv: *const c_void,
        dwdestcontext: u32,
        pvdestcontext: *const c_void,
        mshlflags: u32,
        pcid: *mut GUID,
    ) -> HRESULT {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;

            ((*this).marshaler.vtable().GetUnmarshalClass)(
                transmute_copy(&(*this).marshaler),
                riid,
                pv,
                dwdestcontext,
                pvdestcontext,
                mshlflags,
                pcid,
            )
        }
    }

    unsafe extern "system" fn GetMarshalSizeMax(
        this: *mut c_void,
        riid: *const GUID,
        pv: *const c_void,
        dwdestcontext: u32,
        pvdestcontext: *const c_void,
        mshlflags: u32,
        psize: *mut u32,
    ) -> HRESULT {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;

            ((*this).marshaler.vtable().GetMarshalSizeMax)(
                transmute_copy(&(*this).marshaler),
                riid,
                pv,
                dwdestcontext,
                pvdestcontext,
                mshlflags,
                psize,
            )
        }
    }

    unsafe extern "system" fn MarshalInterface(
        this: *mut c_void,
        pstm: *mut c_void,
        riid: *const GUID,
        pv: *const c_void,
        dwdestcontext: u32,
        pvdestcontext: *const c_void,
        mshlflags: u32,
    ) -> HRESULT {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;

            ((*this).marshaler.vtable().MarshalInterface)(
                transmute_copy(&(*this).marshaler),
                pstm,
                riid,
                pv,
                dwdestcontext,
                pvdestcontext,
                mshlflags,
            )
        }
    }

    unsafe extern "system" fn UnmarshalInterface(
        this: *mut c_void,
        pstm: *mut c_void,
        riid: *const GUID,
        ppv: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;

            ((*this).marshaler.vtable().UnmarshalInterface)(
                transmute_copy(&(*this).marshaler),
                pstm,
                riid,
                ppv,
            )
        }
    }

    unsafe extern "system" fn ReleaseMarshalData(this: *mut c_void, pstm: *mut c_void) -> HRESULT {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;

            ((*this).marshaler.vtable().ReleaseMarshalData)(
                transmute_copy(&(*this).marshaler),
                pstm,
            )
        }
    }

    unsafe extern "system" fn DisconnectObject(this: *mut c_void, dwreserved: u32) -> HRESULT {
        unsafe {
            let this = this as *mut *mut c_void as *mut Self;

            ((*this).marshaler.vtable().DisconnectObject)(
                transmute_copy(&(*this).marshaler),
                dwreserved,
            )
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct IMarshal(IUnknown);

unsafe impl Interface for IMarshal {
    type Vtable = IMarshal_Vtbl;
    const IID: GUID = GUID::from_u128(0x00000003_0000_0000_c000_000000000046);
}

#[repr(C)]
pub struct IMarshal_Vtbl {
    base__: IUnknown_Vtbl,

    GetUnmarshalClass: unsafe extern "system" fn(
        *mut c_void,
        *const GUID,
        *const c_void,
        u32,
        *const c_void,
        u32,
        *mut GUID,
    ) -> HRESULT,

    GetMarshalSizeMax: unsafe extern "system" fn(
        *mut c_void,
        *const GUID,
        *const c_void,
        u32,
        *const c_void,
        u32,
        *mut u32,
    ) -> HRESULT,

    MarshalInterface: unsafe extern "system" fn(
        *mut c_void,
        *mut c_void,
        *const GUID,
        *const c_void,
        u32,
        *const c_void,
        u32,
    ) -> HRESULT,

    UnmarshalInterface: unsafe extern "system" fn(
        *mut c_void,
        *mut c_void,
        *const GUID,
        *mut *mut c_void,
    ) -> HRESULT,

    ReleaseMarshalData: unsafe extern "system" fn(*mut c_void, *mut c_void) -> HRESULT,
    DisconnectObject: unsafe extern "system" fn(*mut c_void, u32) -> HRESULT,
}
