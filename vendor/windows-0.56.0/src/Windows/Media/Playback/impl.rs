#[cfg(feature = "deprecated")]
pub trait IMediaEnginePlaybackSource_Impl: Sized {
    fn CurrentItem(&self) -> windows_core::Result<MediaPlaybackItem>;
    fn SetPlaybackSource(&self, source: Option<&IMediaPlaybackSource>) -> windows_core::Result<()>;
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for IMediaEnginePlaybackSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaEnginePlaybackSource";
}
#[cfg(feature = "deprecated")]
impl IMediaEnginePlaybackSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: isize>() -> IMediaEnginePlaybackSource_Vtbl {
        unsafe extern "system" fn CurrentItem<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IMediaEnginePlaybackSource_Impl::CurrentItem(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaybackSource<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaEnginePlaybackSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, source: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IMediaEnginePlaybackSource_Impl::SetPlaybackSource(this, windows_core::from_raw_borrowed(&source)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaEnginePlaybackSource, OFFSET>(),
            CurrentItem: CurrentItem::<Identity, Impl, OFFSET>,
            SetPlaybackSource: SetPlaybackSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaEnginePlaybackSource as windows_core::Interface>::IID
    }
}
pub trait IMediaPlaybackSource_Impl: Sized {}
impl windows_core::RuntimeName for IMediaPlaybackSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSource";
}
impl IMediaPlaybackSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaPlaybackSource_Impl, const OFFSET: isize>() -> IMediaPlaybackSource_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaPlaybackSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaPlaybackSource as windows_core::Interface>::IID
    }
}
