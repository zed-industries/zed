#[cfg(feature = "Foundation_Collections")]
pub trait IMediaEncodingProperties_Impl: Sized {
    fn Properties(&self) -> windows_core::Result<MediaPropertySet>;
    fn Type(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetSubtype(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Subtype(&self) -> windows_core::Result<windows_core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IMediaEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.IMediaEncodingProperties";
}
#[cfg(feature = "Foundation_Collections")]
impl IMediaEncodingProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>() -> IMediaEncodingProperties_Vtbl {
        unsafe extern "system" fn Properties<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMediaEncodingProperties_Impl::Properties(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMediaEncodingProperties_Impl::Type(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubtype<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMediaEncodingProperties_Impl::SetSubtype(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Subtype<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaEncodingProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMediaEncodingProperties_Impl::Subtype(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaEncodingProperties, OFFSET>(),
            Properties: Properties::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetSubtype: SetSubtype::<Identity, Impl, OFFSET>,
            Subtype: Subtype::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaEncodingProperties as windows_core::Interface>::IID
    }
}
