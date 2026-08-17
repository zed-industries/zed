#[cfg(feature = "Win32_System_Com")]
pub trait IFeed_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Xml(&self, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> windows_core::Result<windows_core::BSTR>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Rename(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Url(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetUrl(&self, feedurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn LocalId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Move(&self, newparentpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Parent(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn LastWriteTime(&self) -> windows_core::Result<f64>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Download(&self) -> windows_core::Result<()>;
    fn AsyncDownload(&self) -> windows_core::Result<()>;
    fn CancelAsyncDownload(&self) -> windows_core::Result<()>;
    fn SyncSetting(&self) -> windows_core::Result<FEEDS_SYNC_SETTING>;
    fn SetSyncSetting(&self, syncsetting: FEEDS_SYNC_SETTING) -> windows_core::Result<()>;
    fn Interval(&self) -> windows_core::Result<i32>;
    fn SetInterval(&self, minutes: i32) -> windows_core::Result<()>;
    fn LastDownloadTime(&self) -> windows_core::Result<f64>;
    fn LocalEnclosurePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Items(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetItem(&self, itemid: i32) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Link(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Image(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LastBuildDate(&self) -> windows_core::Result<f64>;
    fn PubDate(&self) -> windows_core::Result<f64>;
    fn Ttl(&self) -> windows_core::Result<i32>;
    fn Language(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Copyright(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MaxItemCount(&self) -> windows_core::Result<i32>;
    fn SetMaxItemCount(&self, count: i32) -> windows_core::Result<()>;
    fn DownloadEnclosuresAutomatically(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDownloadEnclosuresAutomatically(&self, downloadenclosuresautomatically: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DownloadStatus(&self) -> windows_core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&self) -> windows_core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn Merge(&self, feedxml: &windows_core::BSTR, feedurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DownloadUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsList(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MarkAllItemsRead(&self) -> windows_core::Result<()>;
    fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn UnreadItemCount(&self) -> windows_core::Result<i32>;
    fn ItemCount(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFeed {}
#[cfg(feature = "Win32_System_Com")]
impl IFeed_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeed_Vtbl
    where
        Identity: IFeed_Impl,
    {
        unsafe extern "system" fn Xml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Xml(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&sortproperty), core::mem::transmute_copy(&sortorder), core::mem::transmute_copy(&filterflags), core::mem::transmute_copy(&includeflags)) {
                Ok(ok__) => {
                    xml.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::Rename(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn Url<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Url(this) {
                Ok(ok__) => {
                    feedurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::SetUrl(this, core::mem::transmute(&feedurl)).into()
        }
        unsafe extern "system" fn LocalId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedguid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::LocalId(this) {
                Ok(ok__) => {
                    feedguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Path(this) {
                Ok(ok__) => {
                    path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newparentpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::Move(this, core::mem::transmute(&newparentpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Parent(this) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWriteTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastwrite: *mut f64) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::LastWriteTime(this) {
                Ok(ok__) => {
                    lastwrite.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Download<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::Download(this).into()
        }
        unsafe extern "system" fn AsyncDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::AsyncDownload(this).into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::CancelAsyncDownload(this).into()
        }
        unsafe extern "system" fn SyncSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncsetting: *mut FEEDS_SYNC_SETTING) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::SyncSetting(this) {
                Ok(ok__) => {
                    syncsetting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, syncsetting: FEEDS_SYNC_SETTING) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::SetSyncSetting(this, core::mem::transmute_copy(&syncsetting)).into()
        }
        unsafe extern "system" fn Interval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Interval(this) {
                Ok(ok__) => {
                    minutes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: i32) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::SetInterval(this, core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn LastDownloadTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastdownload: *mut f64) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::LastDownloadTime(this) {
                Ok(ok__) => {
                    lastdownload.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalEnclosurePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::LocalEnclosurePath(this) {
                Ok(ok__) => {
                    path.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Items(this) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: i32, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::GetItem(this, core::mem::transmute_copy(&itemid)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Title(this) {
                Ok(ok__) => {
                    title.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Description(this) {
                Ok(ok__) => {
                    description.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, homepage: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Link(this) {
                Ok(ok__) => {
                    homepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Image(this) {
                Ok(ok__) => {
                    imageurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBuildDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastbuilddate: *mut f64) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::LastBuildDate(this) {
                Ok(ok__) => {
                    lastbuilddate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastpopulatedate: *mut f64) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::PubDate(this) {
                Ok(ok__) => {
                    lastpopulatedate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ttl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ttl: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Ttl(this) {
                Ok(ok__) => {
                    ttl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Language(this) {
                Ok(ok__) => {
                    language.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copyright<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, copyright: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::Copyright(this) {
                Ok(ok__) => {
                    copyright.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::MaxItemCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::SetMaxItemCount(this, core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadenclosuresautomatically: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::DownloadEnclosuresAutomatically(this) {
                Ok(ok__) => {
                    downloadenclosuresautomatically.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadenclosuresautomatically: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::SetDownloadEnclosuresAutomatically(this, core::mem::transmute_copy(&downloadenclosuresautomatically)).into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::DownloadStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::LastDownloadError(this) {
                Ok(ok__) => {
                    error.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedxml: core::mem::MaybeUninit<windows_core::BSTR>, feedurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::Merge(this, core::mem::transmute(&feedxml), core::mem::transmute(&feedurl)).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::DownloadUrl(this) {
                Ok(ok__) => {
                    feedurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, islist: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::IsList(this) {
                Ok(ok__) => {
                    islist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarkAllItemsRead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed_Impl::MarkAllItemsRead(this).into()
        }
        unsafe extern "system" fn GetWatcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::GetWatcher(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&mask)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnreadItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::UnreadItemCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed_Impl::ItemCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Xml: Xml::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Rename: Rename::<Identity, OFFSET>,
            Url: Url::<Identity, OFFSET>,
            SetUrl: SetUrl::<Identity, OFFSET>,
            LocalId: LocalId::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            LastWriteTime: LastWriteTime::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, OFFSET>,
            SyncSetting: SyncSetting::<Identity, OFFSET>,
            SetSyncSetting: SetSyncSetting::<Identity, OFFSET>,
            Interval: Interval::<Identity, OFFSET>,
            SetInterval: SetInterval::<Identity, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, OFFSET>,
            LocalEnclosurePath: LocalEnclosurePath::<Identity, OFFSET>,
            Items: Items::<Identity, OFFSET>,
            GetItem: GetItem::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Link: Link::<Identity, OFFSET>,
            Image: Image::<Identity, OFFSET>,
            LastBuildDate: LastBuildDate::<Identity, OFFSET>,
            PubDate: PubDate::<Identity, OFFSET>,
            Ttl: Ttl::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            Copyright: Copyright::<Identity, OFFSET>,
            MaxItemCount: MaxItemCount::<Identity, OFFSET>,
            SetMaxItemCount: SetMaxItemCount::<Identity, OFFSET>,
            DownloadEnclosuresAutomatically: DownloadEnclosuresAutomatically::<Identity, OFFSET>,
            SetDownloadEnclosuresAutomatically: SetDownloadEnclosuresAutomatically::<Identity, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, OFFSET>,
            Merge: Merge::<Identity, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, OFFSET>,
            IsList: IsList::<Identity, OFFSET>,
            MarkAllItemsRead: MarkAllItemsRead::<Identity, OFFSET>,
            GetWatcher: GetWatcher::<Identity, OFFSET>,
            UnreadItemCount: UnreadItemCount::<Identity, OFFSET>,
            ItemCount: ItemCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeed as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeed2_Impl: Sized + IFeed_Impl {
    fn GetItemByEffectiveId(&self, itemeffectiveid: i32) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn LastItemDownloadTime(&self) -> windows_core::Result<f64>;
    fn Username(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Password(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCredentials(&self, username: &windows_core::BSTR, password: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClearCredentials(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFeed2 {}
#[cfg(feature = "Win32_System_Com")]
impl IFeed2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeed2_Vtbl
    where
        Identity: IFeed2_Impl,
    {
        unsafe extern "system" fn GetItemByEffectiveId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemeffectiveid: i32, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed2_Impl::GetItemByEffectiveId(this, core::mem::transmute_copy(&itemeffectiveid)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastItemDownloadTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastitemdownloadtime: *mut f64) -> windows_core::HRESULT
        where
            Identity: IFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed2_Impl::LastItemDownloadTime(this) {
                Ok(ok__) => {
                    lastitemdownloadtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed2_Impl::Username(this) {
                Ok(ok__) => {
                    username.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, password: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeed2_Impl::Password(this) {
                Ok(ok__) => {
                    password.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: core::mem::MaybeUninit<windows_core::BSTR>, password: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed2_Impl::SetCredentials(this, core::mem::transmute(&username), core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn ClearCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeed2_Impl::ClearCredentials(this).into()
        }
        Self {
            base__: IFeed_Vtbl::new::<Identity, OFFSET>(),
            GetItemByEffectiveId: GetItemByEffectiveId::<Identity, OFFSET>,
            LastItemDownloadTime: LastItemDownloadTime::<Identity, OFFSET>,
            Username: Username::<Identity, OFFSET>,
            Password: Password::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            ClearCredentials: ClearCredentials::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeed2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFeed as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedEnclosure_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Url(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn AsyncDownload(&self) -> windows_core::Result<()>;
    fn CancelAsyncDownload(&self) -> windows_core::Result<()>;
    fn DownloadStatus(&self) -> windows_core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&self) -> windows_core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn LocalPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Parent(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn DownloadUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DownloadMimeType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RemoveFile(&self) -> windows_core::Result<()>;
    fn SetFile(&self, downloadurl: &windows_core::BSTR, downloadfilepath: &windows_core::BSTR, downloadmimetype: &windows_core::BSTR, enclosurefilename: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFeedEnclosure {}
#[cfg(feature = "Win32_System_Com")]
impl IFeedEnclosure_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeedEnclosure_Vtbl
    where
        Identity: IFeedEnclosure_Impl,
    {
        unsafe extern "system" fn Url<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enclosureurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedEnclosure_Impl::Url(this) {
                Ok(ok__) => {
                    enclosureurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mimetype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedEnclosure_Impl::Type(this) {
                Ok(ok__) => {
                    mimetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedEnclosure_Impl::Length(this) {
                Ok(ok__) => {
                    length.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEnclosure_Impl::AsyncDownload(this).into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEnclosure_Impl::CancelAsyncDownload(this).into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedEnclosure_Impl::DownloadStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedEnclosure_Impl::LastDownloadError(this) {
                Ok(ok__) => {
                    error.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, localpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedEnclosure_Impl::LocalPath(this) {
                Ok(ok__) => {
                    localpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedEnclosure_Impl::Parent(this) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enclosureurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedEnclosure_Impl::DownloadUrl(this) {
                Ok(ok__) => {
                    enclosureurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMimeType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, mimetype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedEnclosure_Impl::DownloadMimeType(this) {
                Ok(ok__) => {
                    mimetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEnclosure_Impl::RemoveFile(this).into()
        }
        unsafe extern "system" fn SetFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, downloadurl: core::mem::MaybeUninit<windows_core::BSTR>, downloadfilepath: core::mem::MaybeUninit<windows_core::BSTR>, downloadmimetype: core::mem::MaybeUninit<windows_core::BSTR>, enclosurefilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEnclosure_Impl::SetFile(this, core::mem::transmute(&downloadurl), core::mem::transmute(&downloadfilepath), core::mem::transmute(&downloadmimetype), core::mem::transmute(&enclosurefilename)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Url: Url::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, OFFSET>,
            LocalPath: LocalPath::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, OFFSET>,
            DownloadMimeType: DownloadMimeType::<Identity, OFFSET>,
            RemoveFile: RemoveFile::<Identity, OFFSET>,
            SetFile: SetFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedEnclosure as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Error(&self) -> windows_core::Result<()>;
    fn FeedDeleted(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedRenamed(&self, path: &windows_core::BSTR, oldpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedUrlChanged(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedMoved(&self, path: &windows_core::BSTR, oldpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedDownloading(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedDownloadCompleted(&self, path: &windows_core::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> windows_core::Result<()>;
    fn FeedItemCountChanged(&self, path: &windows_core::BSTR, itemcounttype: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFeedEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IFeedEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeedEvents_Vtbl
    where
        Identity: IFeedEvents_Impl,
    {
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEvents_Impl::Error(this).into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEvents_Impl::FeedDeleted(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, oldpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEvents_Impl::FeedRenamed(this, core::mem::transmute(&path), core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEvents_Impl::FeedUrlChanged(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedMoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, oldpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEvents_Impl::FeedMoved(this, core::mem::transmute(&path), core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEvents_Impl::FeedDownloading(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> windows_core::HRESULT
        where
            Identity: IFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEvents_Impl::FeedDownloadCompleted(this, core::mem::transmute(&path), core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, itemcounttype: i32) -> windows_core::HRESULT
        where
            Identity: IFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedEvents_Impl::FeedItemCountChanged(this, core::mem::transmute(&path), core::mem::transmute_copy(&itemcounttype)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Error: Error::<Identity, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, OFFSET>,
            FeedMoved: FeedMoved::<Identity, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedFolder_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Feeds(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn Subfolders(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn CreateFeed(&self, feedname: &windows_core::BSTR, feedurl: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn CreateSubfolder(&self, foldername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn ExistsFeed(&self, feedname: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetFeed(&self, feedname: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn ExistsSubfolder(&self, foldername: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSubfolder(&self, foldername: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Rename(&self, foldername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Move(&self, newparentpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Parent(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn IsRoot(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TotalUnreadItemCount(&self) -> windows_core::Result<i32>;
    fn TotalItemCount(&self) -> windows_core::Result<i32>;
    fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFeedFolder {}
#[cfg(feature = "Win32_System_Com")]
impl IFeedFolder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeedFolder_Vtbl
    where
        Identity: IFeedFolder_Impl,
    {
        unsafe extern "system" fn Feeds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::Feeds(this) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subfolders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::Subfolders(this) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedname: core::mem::MaybeUninit<windows_core::BSTR>, feedurl: core::mem::MaybeUninit<windows_core::BSTR>, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::CreateFeed(this, core::mem::transmute(&feedname), core::mem::transmute(&feedurl)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSubfolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, foldername: core::mem::MaybeUninit<windows_core::BSTR>, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::CreateSubfolder(this, core::mem::transmute(&foldername)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedname: core::mem::MaybeUninit<windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::ExistsFeed(this, core::mem::transmute(&feedname)) {
                Ok(ok__) => {
                    exists.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedname: core::mem::MaybeUninit<windows_core::BSTR>, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::GetFeed(this, core::mem::transmute(&feedname)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsSubfolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, foldername: core::mem::MaybeUninit<windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::ExistsSubfolder(this, core::mem::transmute(&foldername)) {
                Ok(ok__) => {
                    exists.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubfolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, foldername: core::mem::MaybeUninit<windows_core::BSTR>, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::GetSubfolder(this, core::mem::transmute(&foldername)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolder_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, foldername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::Name(this) {
                Ok(ok__) => {
                    foldername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, foldername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolder_Impl::Rename(this, core::mem::transmute(&foldername)).into()
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, folderpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::Path(this) {
                Ok(ok__) => {
                    folderpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newparentpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolder_Impl::Move(this, core::mem::transmute(&newparentpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::Parent(this) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isroot: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::IsRoot(this) {
                Ok(ok__) => {
                    isroot.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalUnreadItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::TotalUnreadItemCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::TotalItemCount(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedFolder_Impl::GetWatcher(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&mask)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Feeds: Feeds::<Identity, OFFSET>,
            Subfolders: Subfolders::<Identity, OFFSET>,
            CreateFeed: CreateFeed::<Identity, OFFSET>,
            CreateSubfolder: CreateSubfolder::<Identity, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, OFFSET>,
            GetFeed: GetFeed::<Identity, OFFSET>,
            ExistsSubfolder: ExistsSubfolder::<Identity, OFFSET>,
            GetSubfolder: GetSubfolder::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Rename: Rename::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            IsRoot: IsRoot::<Identity, OFFSET>,
            TotalUnreadItemCount: TotalUnreadItemCount::<Identity, OFFSET>,
            TotalItemCount: TotalItemCount::<Identity, OFFSET>,
            GetWatcher: GetWatcher::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedFolder as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedFolderEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Error(&self) -> windows_core::Result<()>;
    fn FolderAdded(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FolderDeleted(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FolderRenamed(&self, path: &windows_core::BSTR, oldpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FolderMovedFrom(&self, path: &windows_core::BSTR, oldpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FolderMovedTo(&self, path: &windows_core::BSTR, oldpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FolderItemCountChanged(&self, path: &windows_core::BSTR, itemcounttype: i32) -> windows_core::Result<()>;
    fn FeedAdded(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedDeleted(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedRenamed(&self, path: &windows_core::BSTR, oldpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedUrlChanged(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedMovedFrom(&self, path: &windows_core::BSTR, oldpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedMovedTo(&self, path: &windows_core::BSTR, oldpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedDownloading(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FeedDownloadCompleted(&self, path: &windows_core::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> windows_core::Result<()>;
    fn FeedItemCountChanged(&self, path: &windows_core::BSTR, itemcounttype: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFeedFolderEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IFeedFolderEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeedFolderEvents_Vtbl
    where
        Identity: IFeedFolderEvents_Impl,
    {
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::Error(this).into()
        }
        unsafe extern "system" fn FolderAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FolderAdded(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FolderDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FolderDeleted(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FolderRenamed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, oldpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FolderRenamed(this, core::mem::transmute(&path), core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FolderMovedFrom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, oldpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FolderMovedFrom(this, core::mem::transmute(&path), core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FolderMovedTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, oldpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FolderMovedTo(this, core::mem::transmute(&path), core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FolderItemCountChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, itemcounttype: i32) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FolderItemCountChanged(this, core::mem::transmute(&path), core::mem::transmute_copy(&itemcounttype)).into()
        }
        unsafe extern "system" fn FeedAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FeedAdded(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FeedDeleted(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, oldpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FeedRenamed(this, core::mem::transmute(&path), core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FeedUrlChanged(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedMovedFrom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, oldpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FeedMovedFrom(this, core::mem::transmute(&path), core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FeedMovedTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, oldpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FeedMovedTo(this, core::mem::transmute(&path), core::mem::transmute(&oldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FeedDownloading(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FeedDownloadCompleted(this, core::mem::transmute(&path), core::mem::transmute_copy(&error)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, itemcounttype: i32) -> windows_core::HRESULT
        where
            Identity: IFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedFolderEvents_Impl::FeedItemCountChanged(this, core::mem::transmute(&path), core::mem::transmute_copy(&itemcounttype)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Error: Error::<Identity, OFFSET>,
            FolderAdded: FolderAdded::<Identity, OFFSET>,
            FolderDeleted: FolderDeleted::<Identity, OFFSET>,
            FolderRenamed: FolderRenamed::<Identity, OFFSET>,
            FolderMovedFrom: FolderMovedFrom::<Identity, OFFSET>,
            FolderMovedTo: FolderMovedTo::<Identity, OFFSET>,
            FolderItemCountChanged: FolderItemCountChanged::<Identity, OFFSET>,
            FeedAdded: FeedAdded::<Identity, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, OFFSET>,
            FeedMovedFrom: FeedMovedFrom::<Identity, OFFSET>,
            FeedMovedTo: FeedMovedTo::<Identity, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedFolderEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Xml(&self, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> windows_core::Result<windows_core::BSTR>;
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Link(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Guid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PubDate(&self) -> windows_core::Result<f64>;
    fn Comments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Author(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Enclosure(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn IsRead(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsRead(&self, isread: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn LocalId(&self) -> windows_core::Result<i32>;
    fn Parent(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn DownloadUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LastDownloadTime(&self) -> windows_core::Result<f64>;
    fn Modified(&self) -> windows_core::Result<f64>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFeedItem {}
#[cfg(feature = "Win32_System_Com")]
impl IFeedItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeedItem_Vtbl
    where
        Identity: IFeedItem_Impl,
    {
        unsafe extern "system" fn Xml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::Xml(this, core::mem::transmute_copy(&includeflags)) {
                Ok(ok__) => {
                    xml.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, title: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::Title(this) {
                Ok(ok__) => {
                    title.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, linkurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::Link(this) {
                Ok(ok__) => {
                    linkurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemguid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::Guid(this) {
                Ok(ok__) => {
                    itemguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::Description(this) {
                Ok(ok__) => {
                    description.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pubdate: *mut f64) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::PubDate(this) {
                Ok(ok__) => {
                    pubdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, comments: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::Comments(this) {
                Ok(ok__) => {
                    comments.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, author: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::Author(this) {
                Ok(ok__) => {
                    author.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enclosure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::Enclosure(this) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isread: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::IsRead(this) {
                Ok(ok__) => {
                    isread.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isread: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedItem_Impl::SetIsRead(this, core::mem::transmute_copy(&isread)).into()
        }
        unsafe extern "system" fn LocalId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::LocalId(this) {
                Ok(ok__) => {
                    itemid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::Parent(this) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedItem_Impl::Delete(this).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::DownloadUrl(this) {
                Ok(ok__) => {
                    itemurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastdownload: *mut f64) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::LastDownloadTime(this) {
                Ok(ok__) => {
                    lastdownload.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, modified: *mut f64) -> windows_core::HRESULT
        where
            Identity: IFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem_Impl::Modified(this) {
                Ok(ok__) => {
                    modified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Xml: Xml::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            Link: Link::<Identity, OFFSET>,
            Guid: Guid::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            PubDate: PubDate::<Identity, OFFSET>,
            Comments: Comments::<Identity, OFFSET>,
            Author: Author::<Identity, OFFSET>,
            Enclosure: Enclosure::<Identity, OFFSET>,
            IsRead: IsRead::<Identity, OFFSET>,
            SetIsRead: SetIsRead::<Identity, OFFSET>,
            LocalId: LocalId::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedItem2_Impl: Sized + IFeedItem_Impl {
    fn EffectiveId(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFeedItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl IFeedItem2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeedItem2_Vtbl
    where
        Identity: IFeedItem2_Impl,
    {
        unsafe extern "system" fn EffectiveId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectiveid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeedItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedItem2_Impl::EffectiveId(this) {
                Ok(ok__) => {
                    effectiveid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IFeedItem_Vtbl::new::<Identity, OFFSET>(), EffectiveId: EffectiveId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedItem2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFeedItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFeedsEnum_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: i32) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl windows_core::RuntimeName for IFeedsEnum {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFeedsEnum_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeedsEnum_Vtbl
    where
        Identity: IFeedsEnum_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeedsEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsEnum_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedsEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsEnum_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvar: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedsEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsEnum_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    enumvar.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedsEnum as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFeedsManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RootFolder(&self) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn IsSubscribed(&self, feedurl: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExistsFeed(&self, feedpath: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetFeed(&self, feedpath: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetFeedByUrl(&self, feedurl: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn ExistsFolder(&self, folderpath: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetFolder(&self, folderpath: &windows_core::BSTR) -> windows_core::Result<super::super::System::Com::IDispatch>;
    fn DeleteFeed(&self, feedpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeleteFolder(&self, folderpath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn BackgroundSync(&self, action: FEEDS_BACKGROUNDSYNC_ACTION) -> windows_core::Result<()>;
    fn BackgroundSyncStatus(&self) -> windows_core::Result<FEEDS_BACKGROUNDSYNC_STATUS>;
    fn DefaultInterval(&self) -> windows_core::Result<i32>;
    fn SetDefaultInterval(&self, minutes: i32) -> windows_core::Result<()>;
    fn AsyncSyncAll(&self) -> windows_core::Result<()>;
    fn Normalize(&self, feedxmlin: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn ItemCountLimit(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IFeedsManager {}
#[cfg(feature = "Win32_System_Com")]
impl IFeedsManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IFeedsManager_Vtbl
    where
        Identity: IFeedsManager_Impl,
    {
        unsafe extern "system" fn RootFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::RootFolder(this) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubscribed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedurl: core::mem::MaybeUninit<windows_core::BSTR>, subscribed: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::IsSubscribed(this, core::mem::transmute(&feedurl)) {
                Ok(ok__) => {
                    subscribed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedpath: core::mem::MaybeUninit<windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::ExistsFeed(this, core::mem::transmute(&feedpath)) {
                Ok(ok__) => {
                    exists.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedpath: core::mem::MaybeUninit<windows_core::BSTR>, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::GetFeed(this, core::mem::transmute(&feedpath)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeedByUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedurl: core::mem::MaybeUninit<windows_core::BSTR>, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::GetFeedByUrl(this, core::mem::transmute(&feedurl)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, folderpath: core::mem::MaybeUninit<windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::ExistsFolder(this, core::mem::transmute(&folderpath)) {
                Ok(ok__) => {
                    exists.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, folderpath: core::mem::MaybeUninit<windows_core::BSTR>, disp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::GetFolder(this, core::mem::transmute(&folderpath)) {
                Ok(ok__) => {
                    disp.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedsManager_Impl::DeleteFeed(this, core::mem::transmute(&feedpath)).into()
        }
        unsafe extern "system" fn DeleteFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, folderpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedsManager_Impl::DeleteFolder(this, core::mem::transmute(&folderpath)).into()
        }
        unsafe extern "system" fn BackgroundSync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: FEEDS_BACKGROUNDSYNC_ACTION) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedsManager_Impl::BackgroundSync(this, core::mem::transmute_copy(&action)).into()
        }
        unsafe extern "system" fn BackgroundSyncStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::BackgroundSyncStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::DefaultInterval(this) {
                Ok(ok__) => {
                    minutes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, minutes: i32) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedsManager_Impl::SetDefaultInterval(this, core::mem::transmute_copy(&minutes)).into()
        }
        unsafe extern "system" fn AsyncSyncAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFeedsManager_Impl::AsyncSyncAll(this).into()
        }
        unsafe extern "system" fn Normalize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedxmlin: core::mem::MaybeUninit<windows_core::BSTR>, feedxmlout: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::Normalize(this, core::mem::transmute(&feedxmlin)) {
                Ok(ok__) => {
                    feedxmlout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCountLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemcountlimit: *mut i32) -> windows_core::HRESULT
        where
            Identity: IFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFeedsManager_Impl::ItemCountLimit(this) {
                Ok(ok__) => {
                    itemcountlimit.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RootFolder: RootFolder::<Identity, OFFSET>,
            IsSubscribed: IsSubscribed::<Identity, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, OFFSET>,
            GetFeed: GetFeed::<Identity, OFFSET>,
            GetFeedByUrl: GetFeedByUrl::<Identity, OFFSET>,
            ExistsFolder: ExistsFolder::<Identity, OFFSET>,
            GetFolder: GetFolder::<Identity, OFFSET>,
            DeleteFeed: DeleteFeed::<Identity, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, OFFSET>,
            BackgroundSync: BackgroundSync::<Identity, OFFSET>,
            BackgroundSyncStatus: BackgroundSyncStatus::<Identity, OFFSET>,
            DefaultInterval: DefaultInterval::<Identity, OFFSET>,
            SetDefaultInterval: SetDefaultInterval::<Identity, OFFSET>,
            AsyncSyncAll: AsyncSyncAll::<Identity, OFFSET>,
            Normalize: Normalize::<Identity, OFFSET>,
            ItemCountLimit: ItemCountLimit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFeedsManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IWMPAudioRenderConfig_Impl: Sized {
    fn audioOutputDevice(&self, pbstroutputdevice: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetaudioOutputDevice(&self, bstroutputdevice: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPAudioRenderConfig {}
impl IWMPAudioRenderConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPAudioRenderConfig_Vtbl
    where
        Identity: IWMPAudioRenderConfig_Impl,
    {
        unsafe extern "system" fn audioOutputDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstroutputdevice: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPAudioRenderConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPAudioRenderConfig_Impl::audioOutputDevice(this, core::mem::transmute_copy(&pbstroutputdevice)).into()
        }
        unsafe extern "system" fn SetaudioOutputDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstroutputdevice: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPAudioRenderConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPAudioRenderConfig_Impl::SetaudioOutputDevice(this, core::mem::transmute(&bstroutputdevice)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            audioOutputDevice: audioOutputDevice::<Identity, OFFSET>,
            SetaudioOutputDevice: SetaudioOutputDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPAudioRenderConfig as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCdrom_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn driveSpecifier(&self, pbstrdrive: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn playlist(&self) -> windows_core::Result<IWMPPlaylist>;
    fn eject(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPCdrom {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCdrom_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPCdrom_Vtbl
    where
        Identity: IWMPCdrom_Impl,
    {
        unsafe extern "system" fn driveSpecifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdrive: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPCdrom_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdrom_Impl::driveSpecifier(this, core::mem::transmute_copy(&pbstrdrive)).into()
        }
        unsafe extern "system" fn playlist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplaylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdrom_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCdrom_Impl::playlist(this) {
                Ok(ok__) => {
                    ppplaylist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn eject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdrom_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdrom_Impl::eject(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            driveSpecifier: driveSpecifier::<Identity, OFFSET>,
            playlist: playlist::<Identity, OFFSET>,
            eject: eject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCdrom as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCdromBurn_Impl: Sized {
    fn isAvailable(&self, bstritem: &windows_core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getItemInfo(&self, bstritem: &windows_core::BSTR, pbstrval: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn label(&self, pbstrlabel: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Setlabel(&self, bstrlabel: &windows_core::BSTR) -> windows_core::Result<()>;
    fn burnFormat(&self, pwmpbf: *mut WMPBurnFormat) -> windows_core::Result<()>;
    fn SetburnFormat(&self, wmpbf: WMPBurnFormat) -> windows_core::Result<()>;
    fn burnPlaylist(&self) -> windows_core::Result<IWMPPlaylist>;
    fn SetburnPlaylist(&self, pplaylist: Option<&IWMPPlaylist>) -> windows_core::Result<()>;
    fn refreshStatus(&self) -> windows_core::Result<()>;
    fn burnState(&self, pwmpbs: *mut WMPBurnState) -> windows_core::Result<()>;
    fn burnProgress(&self, plprogress: *mut i32) -> windows_core::Result<()>;
    fn startBurn(&self) -> windows_core::Result<()>;
    fn stopBurn(&self) -> windows_core::Result<()>;
    fn erase(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPCdromBurn {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCdromBurn_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPCdromBurn_Vtbl
    where
        Identity: IWMPCdromBurn_Impl,
    {
        unsafe extern "system" fn isAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritem: core::mem::MaybeUninit<windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::isAvailable(this, core::mem::transmute(&bstritem), core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritem: core::mem::MaybeUninit<windows_core::BSTR>, pbstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::getItemInfo(this, core::mem::transmute(&bstritem), core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn label<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::label(this, core::mem::transmute_copy(&pbstrlabel)).into()
        }
        unsafe extern "system" fn Setlabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrlabel: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::Setlabel(this, core::mem::transmute(&bstrlabel)).into()
        }
        unsafe extern "system" fn burnFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpbf: *mut WMPBurnFormat) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::burnFormat(this, core::mem::transmute_copy(&pwmpbf)).into()
        }
        unsafe extern "system" fn SetburnFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmpbf: WMPBurnFormat) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::SetburnFormat(this, core::mem::transmute_copy(&wmpbf)).into()
        }
        unsafe extern "system" fn burnPlaylist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplaylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCdromBurn_Impl::burnPlaylist(this) {
                Ok(ok__) => {
                    ppplaylist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetburnPlaylist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplaylist: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::SetburnPlaylist(this, windows_core::from_raw_borrowed(&pplaylist)).into()
        }
        unsafe extern "system" fn refreshStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::refreshStatus(this).into()
        }
        unsafe extern "system" fn burnState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpbs: *mut WMPBurnState) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::burnState(this, core::mem::transmute_copy(&pwmpbs)).into()
        }
        unsafe extern "system" fn burnProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprogress: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::burnProgress(this, core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startBurn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::startBurn(this).into()
        }
        unsafe extern "system" fn stopBurn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::stopBurn(this).into()
        }
        unsafe extern "system" fn erase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdromBurn_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromBurn_Impl::erase(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            isAvailable: isAvailable::<Identity, OFFSET>,
            getItemInfo: getItemInfo::<Identity, OFFSET>,
            label: label::<Identity, OFFSET>,
            Setlabel: Setlabel::<Identity, OFFSET>,
            burnFormat: burnFormat::<Identity, OFFSET>,
            SetburnFormat: SetburnFormat::<Identity, OFFSET>,
            burnPlaylist: burnPlaylist::<Identity, OFFSET>,
            SetburnPlaylist: SetburnPlaylist::<Identity, OFFSET>,
            refreshStatus: refreshStatus::<Identity, OFFSET>,
            burnState: burnState::<Identity, OFFSET>,
            burnProgress: burnProgress::<Identity, OFFSET>,
            startBurn: startBurn::<Identity, OFFSET>,
            stopBurn: stopBurn::<Identity, OFFSET>,
            erase: erase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCdromBurn as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCdromCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn item(&self, lindex: i32) -> windows_core::Result<IWMPCdrom>;
    fn getByDriveSpecifier(&self, bstrdrivespecifier: &windows_core::BSTR) -> windows_core::Result<IWMPCdrom>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPCdromCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCdromCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPCdromCollection_Vtbl
    where
        Identity: IWMPCdromCollection_Impl,
    {
        unsafe extern "system" fn count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPCdromCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromCollection_Impl::count(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdromCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCdromCollection_Impl::item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByDriveSpecifier<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdrivespecifier: core::mem::MaybeUninit<windows_core::BSTR>, ppcdrom: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdromCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCdromCollection_Impl::getByDriveSpecifier(this, core::mem::transmute(&bstrdrivespecifier)) {
                Ok(ok__) => {
                    ppcdrom.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            count: count::<Identity, OFFSET>,
            item: item::<Identity, OFFSET>,
            getByDriveSpecifier: getByDriveSpecifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCdromCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IWMPCdromRip_Impl: Sized {
    fn ripState(&self, pwmprs: *mut WMPRipState) -> windows_core::Result<()>;
    fn ripProgress(&self, plprogress: *mut i32) -> windows_core::Result<()>;
    fn startRip(&self) -> windows_core::Result<()>;
    fn stopRip(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPCdromRip {}
impl IWMPCdromRip_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPCdromRip_Vtbl
    where
        Identity: IWMPCdromRip_Impl,
    {
        unsafe extern "system" fn ripState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmprs: *mut WMPRipState) -> windows_core::HRESULT
        where
            Identity: IWMPCdromRip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromRip_Impl::ripState(this, core::mem::transmute_copy(&pwmprs)).into()
        }
        unsafe extern "system" fn ripProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprogress: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPCdromRip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromRip_Impl::ripProgress(this, core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startRip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdromRip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromRip_Impl::startRip(this).into()
        }
        unsafe extern "system" fn stopRip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCdromRip_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCdromRip_Impl::stopRip(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ripState: ripState::<Identity, OFFSET>,
            ripProgress: ripProgress::<Identity, OFFSET>,
            startRip: startRip::<Identity, OFFSET>,
            stopRip: stopRip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCdromRip as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPClosedCaption_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SAMIStyle(&self, pbstrsamistyle: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSAMIStyle(&self, bstrsamistyle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SAMILang(&self, pbstrsamilang: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSAMILang(&self, bstrsamilang: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SAMIFileName(&self, pbstrsamifilename: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSAMIFileName(&self, bstrsamifilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn captioningId(&self, pbstrcaptioningid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetcaptioningId(&self, bstrcaptioningid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPClosedCaption {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPClosedCaption_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPClosedCaption_Vtbl
    where
        Identity: IWMPClosedCaption_Impl,
    {
        unsafe extern "system" fn SAMIStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsamistyle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption_Impl::SAMIStyle(this, core::mem::transmute_copy(&pbstrsamistyle)).into()
        }
        unsafe extern "system" fn SetSAMIStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsamistyle: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption_Impl::SetSAMIStyle(this, core::mem::transmute(&bstrsamistyle)).into()
        }
        unsafe extern "system" fn SAMILang<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsamilang: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption_Impl::SAMILang(this, core::mem::transmute_copy(&pbstrsamilang)).into()
        }
        unsafe extern "system" fn SetSAMILang<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsamilang: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption_Impl::SetSAMILang(this, core::mem::transmute(&bstrsamilang)).into()
        }
        unsafe extern "system" fn SAMIFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsamifilename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption_Impl::SAMIFileName(this, core::mem::transmute_copy(&pbstrsamifilename)).into()
        }
        unsafe extern "system" fn SetSAMIFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsamifilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption_Impl::SetSAMIFileName(this, core::mem::transmute(&bstrsamifilename)).into()
        }
        unsafe extern "system" fn captioningId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcaptioningid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption_Impl::captioningId(this, core::mem::transmute_copy(&pbstrcaptioningid)).into()
        }
        unsafe extern "system" fn SetcaptioningId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaptioningid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption_Impl::SetcaptioningId(this, core::mem::transmute(&bstrcaptioningid)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SAMIStyle: SAMIStyle::<Identity, OFFSET>,
            SetSAMIStyle: SetSAMIStyle::<Identity, OFFSET>,
            SAMILang: SAMILang::<Identity, OFFSET>,
            SetSAMILang: SetSAMILang::<Identity, OFFSET>,
            SAMIFileName: SAMIFileName::<Identity, OFFSET>,
            SetSAMIFileName: SetSAMIFileName::<Identity, OFFSET>,
            captioningId: captioningId::<Identity, OFFSET>,
            SetcaptioningId: SetcaptioningId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPClosedCaption as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPClosedCaption2_Impl: Sized + IWMPClosedCaption_Impl {
    fn SAMILangCount(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn getSAMILangName(&self, nindex: i32, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn getSAMILangID(&self, nindex: i32, pllangid: *mut i32) -> windows_core::Result<()>;
    fn SAMIStyleCount(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn getSAMIStyleName(&self, nindex: i32, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPClosedCaption2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPClosedCaption2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPClosedCaption2_Vtbl
    where
        Identity: IWMPClosedCaption2_Impl,
    {
        unsafe extern "system" fn SAMILangCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption2_Impl::SAMILangCount(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getSAMILangName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption2_Impl::getSAMILangName(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn getSAMILangID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, pllangid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption2_Impl::getSAMILangID(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn SAMIStyleCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption2_Impl::SAMIStyleCount(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getSAMIStyleName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: i32, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPClosedCaption2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPClosedCaption2_Impl::getSAMIStyleName(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pbstrname)).into()
        }
        Self {
            base__: IWMPClosedCaption_Vtbl::new::<Identity, OFFSET>(),
            SAMILangCount: SAMILangCount::<Identity, OFFSET>,
            getSAMILangName: getSAMILangName::<Identity, OFFSET>,
            getSAMILangID: getSAMILangID::<Identity, OFFSET>,
            SAMIStyleCount: SAMIStyleCount::<Identity, OFFSET>,
            getSAMIStyleName: getSAMIStyleName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPClosedCaption2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPClosedCaption as windows_core::Interface>::IID
    }
}
pub trait IWMPContentContainer_Impl: Sized {
    fn GetID(&self) -> windows_core::Result<u32>;
    fn GetPrice(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetContentCount(&self) -> windows_core::Result<u32>;
    fn GetContentPrice(&self, idxcontent: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetContentID(&self, idxcontent: u32) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IWMPContentContainer {}
impl IWMPContentContainer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPContentContainer_Vtbl
    where
        Identity: IWMPContentContainer_Impl,
    {
        unsafe extern "system" fn GetID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontentid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentContainer_Impl::GetID(this) {
                Ok(ok__) => {
                    pcontentid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprice: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentContainer_Impl::GetPrice(this) {
                Ok(ok__) => {
                    pbstrprice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentContainer_Impl::GetType(this) {
                Ok(ok__) => {
                    pbstrtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccontent: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentContainer_Impl::GetContentCount(this) {
                Ok(ok__) => {
                    pccontent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentPrice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, idxcontent: u32, pbstrprice: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentContainer_Impl::GetContentPrice(this, core::mem::transmute_copy(&idxcontent)) {
                Ok(ok__) => {
                    pbstrprice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, idxcontent: u32, pcontentid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentContainer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentContainer_Impl::GetContentID(this, core::mem::transmute_copy(&idxcontent)) {
                Ok(ok__) => {
                    pcontentid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, OFFSET>,
            GetPrice: GetPrice::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetContentCount: GetContentCount::<Identity, OFFSET>,
            GetContentPrice: GetContentPrice::<Identity, OFFSET>,
            GetContentID: GetContentID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPContentContainer as windows_core::Interface>::IID
    }
}
pub trait IWMPContentContainerList_Impl: Sized {
    fn GetTransactionType(&self) -> windows_core::Result<WMPTransactionType>;
    fn GetContainerCount(&self) -> windows_core::Result<u32>;
    fn GetContainer(&self, idxcontainer: u32) -> windows_core::Result<IWMPContentContainer>;
}
impl windows_core::RuntimeName for IWMPContentContainerList {}
impl IWMPContentContainerList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPContentContainerList_Vtbl
    where
        Identity: IWMPContentContainerList_Impl,
    {
        unsafe extern "system" fn GetTransactionType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmptt: *mut WMPTransactionType) -> windows_core::HRESULT
        where
            Identity: IWMPContentContainerList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentContainerList_Impl::GetTransactionType(this) {
                Ok(ok__) => {
                    pwmptt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainerCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccontainer: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentContainerList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentContainerList_Impl::GetContainerCount(this) {
                Ok(ok__) => {
                    pccontainer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, idxcontainer: u32, ppcontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPContentContainerList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentContainerList_Impl::GetContainer(this, core::mem::transmute_copy(&idxcontainer)) {
                Ok(ok__) => {
                    ppcontent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTransactionType: GetTransactionType::<Identity, OFFSET>,
            GetContainerCount: GetContainerCount::<Identity, OFFSET>,
            GetContainer: GetContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPContentContainerList as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPContentPartner_Impl: Sized {
    fn SetCallback(&self, pcallback: Option<&IWMPContentPartnerCallback>) -> windows_core::Result<()>;
    fn Notify(&self, r#type: WMPPartnerNotification, pcontext: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetItemInfo(&self, bstrinfoname: &windows_core::BSTR, pcontext: *const windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn GetContentPartnerInfo(&self, bstrinfoname: &windows_core::BSTR) -> windows_core::Result<windows_core::VARIANT>;
    fn GetCommands(&self, location: &windows_core::BSTR, plocationcontext: *const windows_core::VARIANT, itemlocation: &windows_core::BSTR, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> windows_core::Result<()>;
    fn InvokeCommand(&self, dwcommandid: u32, location: &windows_core::BSTR, plocationcontext: *const windows_core::VARIANT, itemlocation: &windows_core::BSTR, citemids: u32, rgitemids: *const u32) -> windows_core::Result<()>;
    fn CanBuySilent(&self, pinfo: Option<&IWMPContentContainerList>, pbstrtotalprice: *mut windows_core::BSTR, psilentok: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Buy(&self, pinfo: Option<&IWMPContentContainerList>, cookie: u32) -> windows_core::Result<()>;
    fn GetStreamingURL(&self, st: WMPStreamingType, pstreamcontext: *const windows_core::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn Download(&self, pinfo: Option<&IWMPContentContainerList>, cookie: u32) -> windows_core::Result<()>;
    fn DownloadTrackComplete(&self, hrresult: windows_core::HRESULT, contentid: u32, downloadtrackparam: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RefreshLicense(&self, dwcookie: u32, flocal: super::super::Foundation::VARIANT_BOOL, bstrurl: &windows_core::BSTR, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: &windows_core::BSTR, preasoncontext: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetCatalogURL(&self, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut windows_core::BSTR, pexpirationdate: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn GetTemplate(&self, task: WMPTaskType, location: &windows_core::BSTR, pcontext: *const windows_core::VARIANT, clicklocation: &windows_core::BSTR, pclickcontext: *const windows_core::VARIANT, bstrfilter: &windows_core::BSTR, bstrviewparams: &windows_core::BSTR, pbstrtemplateurl: *mut windows_core::BSTR, ptemplatesize: *mut WMPTemplateSize) -> windows_core::Result<()>;
    fn UpdateDevice(&self, bstrdevicename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetListContents(&self, location: &windows_core::BSTR, pcontext: *const windows_core::VARIANT, bstrlisttype: &windows_core::BSTR, bstrparams: &windows_core::BSTR, dwlistcookie: u32) -> windows_core::Result<()>;
    fn Login(&self, userinfo: &super::super::System::Com::BLOB, pwdinfo: &super::super::System::Com::BLOB, fusedcachedcreds: super::super::Foundation::VARIANT_BOOL, foktocache: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Authenticate(&self, userinfo: &super::super::System::Com::BLOB, pwdinfo: &super::super::System::Com::BLOB) -> windows_core::Result<()>;
    fn Logout(&self) -> windows_core::Result<()>;
    fn SendMessage(&self, bstrmsg: &windows_core::BSTR, bstrparam: &windows_core::BSTR) -> windows_core::Result<()>;
    fn StationEvent(&self, bstrstationeventtype: &windows_core::BSTR, stationid: u32, playlistindex: u32, trackid: u32, trackdata: &windows_core::BSTR, dwsecondsplayed: u32) -> windows_core::Result<()>;
    fn CompareContainerListPrices(&self, plistbase: Option<&IWMPContentContainerList>, plistcompare: Option<&IWMPContentContainerList>) -> windows_core::Result<i32>;
    fn VerifyPermission(&self, bstrpermission: &windows_core::BSTR, pcontext: *const windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPContentPartner {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPContentPartner_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPContentPartner_Vtbl
    where
        Identity: IWMPContentPartner_Impl,
    {
        unsafe extern "system" fn SetCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::SetCallback(this, windows_core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn Notify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: WMPPartnerNotification, pcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::Notify(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn GetItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinfoname: core::mem::MaybeUninit<windows_core::BSTR>, pcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>, pdata: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentPartner_Impl::GetItemInfo(this, core::mem::transmute(&bstrinfoname), core::mem::transmute_copy(&pcontext)) {
                Ok(ok__) => {
                    pdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentPartnerInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinfoname: core::mem::MaybeUninit<windows_core::BSTR>, pdata: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentPartner_Impl::GetContentPartnerInfo(this, core::mem::transmute(&bstrinfoname)) {
                Ok(ok__) => {
                    pdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCommands<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, location: core::mem::MaybeUninit<windows_core::BSTR>, plocationcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>, itemlocation: core::mem::MaybeUninit<windows_core::BSTR>, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::GetCommands(this, core::mem::transmute(&location), core::mem::transmute_copy(&plocationcontext), core::mem::transmute(&itemlocation), core::mem::transmute_copy(&citemids), core::mem::transmute_copy(&prgitemids), core::mem::transmute_copy(&pcitemids), core::mem::transmute_copy(&pprgitems)).into()
        }
        unsafe extern "system" fn InvokeCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcommandid: u32, location: core::mem::MaybeUninit<windows_core::BSTR>, plocationcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>, itemlocation: core::mem::MaybeUninit<windows_core::BSTR>, citemids: u32, rgitemids: *const u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::InvokeCommand(this, core::mem::transmute_copy(&dwcommandid), core::mem::transmute(&location), core::mem::transmute_copy(&plocationcontext), core::mem::transmute(&itemlocation), core::mem::transmute_copy(&citemids), core::mem::transmute_copy(&rgitemids)).into()
        }
        unsafe extern "system" fn CanBuySilent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut core::ffi::c_void, pbstrtotalprice: *mut core::mem::MaybeUninit<windows_core::BSTR>, psilentok: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::CanBuySilent(this, windows_core::from_raw_borrowed(&pinfo), core::mem::transmute_copy(&pbstrtotalprice), core::mem::transmute_copy(&psilentok)).into()
        }
        unsafe extern "system" fn Buy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::Buy(this, windows_core::from_raw_borrowed(&pinfo), core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn GetStreamingURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, st: WMPStreamingType, pstreamcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>, pbstrurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentPartner_Impl::GetStreamingURL(this, core::mem::transmute_copy(&st), core::mem::transmute_copy(&pstreamcontext)) {
                Ok(ok__) => {
                    pbstrurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Download<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::Download(this, windows_core::from_raw_borrowed(&pinfo), core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn DownloadTrackComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrresult: windows_core::HRESULT, contentid: u32, downloadtrackparam: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::DownloadTrackComplete(this, core::mem::transmute_copy(&hrresult), core::mem::transmute_copy(&contentid), core::mem::transmute(&downloadtrackparam)).into()
        }
        unsafe extern "system" fn RefreshLicense<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32, flocal: super::super::Foundation::VARIANT_BOOL, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: core::mem::MaybeUninit<windows_core::BSTR>, preasoncontext: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::RefreshLicense(this, core::mem::transmute_copy(&dwcookie), core::mem::transmute_copy(&flocal), core::mem::transmute(&bstrurl), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&contentid), core::mem::transmute(&bstrrefreshreason), core::mem::transmute_copy(&preasoncontext)).into()
        }
        unsafe extern "system" fn GetCatalogURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut core::mem::MaybeUninit<windows_core::BSTR>, pexpirationdate: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::GetCatalogURL(this, core::mem::transmute_copy(&dwcatalogversion), core::mem::transmute_copy(&dwcatalogschemaversion), core::mem::transmute_copy(&cataloglcid), core::mem::transmute_copy(&pdwnewcatalogversion), core::mem::transmute_copy(&pbstrcatalogurl), core::mem::transmute_copy(&pexpirationdate)).into()
        }
        unsafe extern "system" fn GetTemplate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, task: WMPTaskType, location: core::mem::MaybeUninit<windows_core::BSTR>, pcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>, clicklocation: core::mem::MaybeUninit<windows_core::BSTR>, pclickcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrfilter: core::mem::MaybeUninit<windows_core::BSTR>, bstrviewparams: core::mem::MaybeUninit<windows_core::BSTR>, pbstrtemplateurl: *mut core::mem::MaybeUninit<windows_core::BSTR>, ptemplatesize: *mut WMPTemplateSize) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::GetTemplate(this, core::mem::transmute_copy(&task), core::mem::transmute(&location), core::mem::transmute_copy(&pcontext), core::mem::transmute(&clicklocation), core::mem::transmute_copy(&pclickcontext), core::mem::transmute(&bstrfilter), core::mem::transmute(&bstrviewparams), core::mem::transmute_copy(&pbstrtemplateurl), core::mem::transmute_copy(&ptemplatesize)).into()
        }
        unsafe extern "system" fn UpdateDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdevicename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::UpdateDevice(this, core::mem::transmute(&bstrdevicename)).into()
        }
        unsafe extern "system" fn GetListContents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, location: core::mem::MaybeUninit<windows_core::BSTR>, pcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>, bstrlisttype: core::mem::MaybeUninit<windows_core::BSTR>, bstrparams: core::mem::MaybeUninit<windows_core::BSTR>, dwlistcookie: u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::GetListContents(this, core::mem::transmute(&location), core::mem::transmute_copy(&pcontext), core::mem::transmute(&bstrlisttype), core::mem::transmute(&bstrparams), core::mem::transmute_copy(&dwlistcookie)).into()
        }
        unsafe extern "system" fn Login<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: super::super::Foundation::VARIANT_BOOL, foktocache: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::Login(this, core::mem::transmute(&userinfo), core::mem::transmute(&pwdinfo), core::mem::transmute_copy(&fusedcachedcreds), core::mem::transmute_copy(&foktocache)).into()
        }
        unsafe extern "system" fn Authenticate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::Authenticate(this, core::mem::transmute(&userinfo), core::mem::transmute(&pwdinfo)).into()
        }
        unsafe extern "system" fn Logout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::Logout(this).into()
        }
        unsafe extern "system" fn SendMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmsg: core::mem::MaybeUninit<windows_core::BSTR>, bstrparam: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::SendMessage(this, core::mem::transmute(&bstrmsg), core::mem::transmute(&bstrparam)).into()
        }
        unsafe extern "system" fn StationEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrstationeventtype: core::mem::MaybeUninit<windows_core::BSTR>, stationid: u32, playlistindex: u32, trackid: u32, trackdata: core::mem::MaybeUninit<windows_core::BSTR>, dwsecondsplayed: u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::StationEvent(this, core::mem::transmute(&bstrstationeventtype), core::mem::transmute_copy(&stationid), core::mem::transmute_copy(&playlistindex), core::mem::transmute_copy(&trackid), core::mem::transmute(&trackdata), core::mem::transmute_copy(&dwsecondsplayed)).into()
        }
        unsafe extern "system" fn CompareContainerListPrices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plistbase: *mut core::ffi::c_void, plistcompare: *mut core::ffi::c_void, presult: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPContentPartner_Impl::CompareContainerListPrices(this, windows_core::from_raw_borrowed(&plistbase), windows_core::from_raw_borrowed(&plistcompare)) {
                Ok(ok__) => {
                    presult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyPermission<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpermission: core::mem::MaybeUninit<windows_core::BSTR>, pcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartner_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartner_Impl::VerifyPermission(this, core::mem::transmute(&bstrpermission), core::mem::transmute_copy(&pcontext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCallback: SetCallback::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            GetItemInfo: GetItemInfo::<Identity, OFFSET>,
            GetContentPartnerInfo: GetContentPartnerInfo::<Identity, OFFSET>,
            GetCommands: GetCommands::<Identity, OFFSET>,
            InvokeCommand: InvokeCommand::<Identity, OFFSET>,
            CanBuySilent: CanBuySilent::<Identity, OFFSET>,
            Buy: Buy::<Identity, OFFSET>,
            GetStreamingURL: GetStreamingURL::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            DownloadTrackComplete: DownloadTrackComplete::<Identity, OFFSET>,
            RefreshLicense: RefreshLicense::<Identity, OFFSET>,
            GetCatalogURL: GetCatalogURL::<Identity, OFFSET>,
            GetTemplate: GetTemplate::<Identity, OFFSET>,
            UpdateDevice: UpdateDevice::<Identity, OFFSET>,
            GetListContents: GetListContents::<Identity, OFFSET>,
            Login: Login::<Identity, OFFSET>,
            Authenticate: Authenticate::<Identity, OFFSET>,
            Logout: Logout::<Identity, OFFSET>,
            SendMessage: SendMessage::<Identity, OFFSET>,
            StationEvent: StationEvent::<Identity, OFFSET>,
            CompareContainerListPrices: CompareContainerListPrices::<Identity, OFFSET>,
            VerifyPermission: VerifyPermission::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPContentPartner as windows_core::Interface>::IID
    }
}
pub trait IWMPContentPartnerCallback_Impl: Sized {
    fn Notify(&self, r#type: WMPCallbackNotification, pcontext: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn BuyComplete(&self, hrresult: windows_core::HRESULT, dwbuycookie: u32) -> windows_core::Result<()>;
    fn DownloadTrack(&self, cookie: u32, bstrtrackurl: &windows_core::BSTR, dwservicetrackid: u32, bstrdownloadparams: &windows_core::BSTR, hrdownload: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetCatalogVersion(&self, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> windows_core::Result<()>;
    fn UpdateDeviceComplete(&self, bstrdevicename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangeView(&self, bstrtype: &windows_core::BSTR, bstrid: &windows_core::BSTR, bstrfilter: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddListContents(&self, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> windows_core::Result<()>;
    fn ListContentsComplete(&self, dwlistcookie: u32, hrsuccess: windows_core::HRESULT) -> windows_core::Result<()>;
    fn SendMessageComplete(&self, bstrmsg: &windows_core::BSTR, bstrparam: &windows_core::BSTR, bstrresult: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetContentIDsInLibrary(&self, pccontentids: *mut u32, pprgids: *mut *mut u32) -> windows_core::Result<()>;
    fn RefreshLicenseComplete(&self, dwcookie: u32, contentid: u32, hrrefresh: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ShowPopup(&self, lindex: i32, bstrparameters: &windows_core::BSTR) -> windows_core::Result<()>;
    fn VerifyPermissionComplete(&self, bstrpermission: &windows_core::BSTR, pcontext: *const windows_core::VARIANT, hrpermission: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPContentPartnerCallback {}
impl IWMPContentPartnerCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPContentPartnerCallback_Vtbl
    where
        Identity: IWMPContentPartnerCallback_Impl,
    {
        unsafe extern "system" fn Notify<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: WMPCallbackNotification, pcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::Notify(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pcontext)).into()
        }
        unsafe extern "system" fn BuyComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrresult: windows_core::HRESULT, dwbuycookie: u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::BuyComplete(this, core::mem::transmute_copy(&hrresult), core::mem::transmute_copy(&dwbuycookie)).into()
        }
        unsafe extern "system" fn DownloadTrack<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: u32, bstrtrackurl: core::mem::MaybeUninit<windows_core::BSTR>, dwservicetrackid: u32, bstrdownloadparams: core::mem::MaybeUninit<windows_core::BSTR>, hrdownload: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::DownloadTrack(this, core::mem::transmute_copy(&cookie), core::mem::transmute(&bstrtrackurl), core::mem::transmute_copy(&dwservicetrackid), core::mem::transmute(&bstrdownloadparams), core::mem::transmute_copy(&hrdownload)).into()
        }
        unsafe extern "system" fn GetCatalogVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::GetCatalogVersion(this, core::mem::transmute_copy(&pdwversion), core::mem::transmute_copy(&pdwschemaversion), core::mem::transmute_copy(&plcid)).into()
        }
        unsafe extern "system" fn UpdateDeviceComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdevicename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::UpdateDeviceComplete(this, core::mem::transmute(&bstrdevicename)).into()
        }
        unsafe extern "system" fn ChangeView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtype: core::mem::MaybeUninit<windows_core::BSTR>, bstrid: core::mem::MaybeUninit<windows_core::BSTR>, bstrfilter: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::ChangeView(this, core::mem::transmute(&bstrtype), core::mem::transmute(&bstrid), core::mem::transmute(&bstrfilter)).into()
        }
        unsafe extern "system" fn AddListContents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::AddListContents(this, core::mem::transmute_copy(&dwlistcookie), core::mem::transmute_copy(&citems), core::mem::transmute_copy(&prgitems)).into()
        }
        unsafe extern "system" fn ListContentsComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwlistcookie: u32, hrsuccess: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::ListContentsComplete(this, core::mem::transmute_copy(&dwlistcookie), core::mem::transmute_copy(&hrsuccess)).into()
        }
        unsafe extern "system" fn SendMessageComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmsg: core::mem::MaybeUninit<windows_core::BSTR>, bstrparam: core::mem::MaybeUninit<windows_core::BSTR>, bstrresult: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::SendMessageComplete(this, core::mem::transmute(&bstrmsg), core::mem::transmute(&bstrparam), core::mem::transmute(&bstrresult)).into()
        }
        unsafe extern "system" fn GetContentIDsInLibrary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccontentids: *mut u32, pprgids: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::GetContentIDsInLibrary(this, core::mem::transmute_copy(&pccontentids), core::mem::transmute_copy(&pprgids)).into()
        }
        unsafe extern "system" fn RefreshLicenseComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32, contentid: u32, hrrefresh: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::RefreshLicenseComplete(this, core::mem::transmute_copy(&dwcookie), core::mem::transmute_copy(&contentid), core::mem::transmute_copy(&hrrefresh)).into()
        }
        unsafe extern "system" fn ShowPopup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, bstrparameters: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::ShowPopup(this, core::mem::transmute_copy(&lindex), core::mem::transmute(&bstrparameters)).into()
        }
        unsafe extern "system" fn VerifyPermissionComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpermission: core::mem::MaybeUninit<windows_core::BSTR>, pcontext: *const core::mem::MaybeUninit<windows_core::VARIANT>, hrpermission: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWMPContentPartnerCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPContentPartnerCallback_Impl::VerifyPermissionComplete(this, core::mem::transmute(&bstrpermission), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&hrpermission)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Notify: Notify::<Identity, OFFSET>,
            BuyComplete: BuyComplete::<Identity, OFFSET>,
            DownloadTrack: DownloadTrack::<Identity, OFFSET>,
            GetCatalogVersion: GetCatalogVersion::<Identity, OFFSET>,
            UpdateDeviceComplete: UpdateDeviceComplete::<Identity, OFFSET>,
            ChangeView: ChangeView::<Identity, OFFSET>,
            AddListContents: AddListContents::<Identity, OFFSET>,
            ListContentsComplete: ListContentsComplete::<Identity, OFFSET>,
            SendMessageComplete: SendMessageComplete::<Identity, OFFSET>,
            GetContentIDsInLibrary: GetContentIDsInLibrary::<Identity, OFFSET>,
            RefreshLicenseComplete: RefreshLicenseComplete::<Identity, OFFSET>,
            ShowPopup: ShowPopup::<Identity, OFFSET>,
            VerifyPermissionComplete: VerifyPermissionComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPContentPartnerCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPControls_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_isAvailable(&self, bstritem: &windows_core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn play(&self) -> windows_core::Result<()>;
    fn stop(&self) -> windows_core::Result<()>;
    fn pause(&self) -> windows_core::Result<()>;
    fn fastForward(&self) -> windows_core::Result<()>;
    fn fastReverse(&self) -> windows_core::Result<()>;
    fn currentPosition(&self, pdcurrentposition: *mut f64) -> windows_core::Result<()>;
    fn SetcurrentPosition(&self, dcurrentposition: f64) -> windows_core::Result<()>;
    fn currentPositionString(&self, pbstrcurrentposition: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn next(&self) -> windows_core::Result<()>;
    fn previous(&self) -> windows_core::Result<()>;
    fn currentItem(&self) -> windows_core::Result<IWMPMedia>;
    fn SetcurrentItem(&self, piwmpmedia: Option<&IWMPMedia>) -> windows_core::Result<()>;
    fn currentMarker(&self, plmarker: *mut i32) -> windows_core::Result<()>;
    fn SetcurrentMarker(&self, lmarker: i32) -> windows_core::Result<()>;
    fn playItem(&self, piwmpmedia: Option<&IWMPMedia>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPControls {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPControls_Vtbl
    where
        Identity: IWMPControls_Impl,
    {
        unsafe extern "system" fn get_isAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritem: core::mem::MaybeUninit<windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::get_isAvailable(this, core::mem::transmute(&bstritem), core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn play<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::play(this).into()
        }
        unsafe extern "system" fn stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::stop(this).into()
        }
        unsafe extern "system" fn pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::pause(this).into()
        }
        unsafe extern "system" fn fastForward<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::fastForward(this).into()
        }
        unsafe extern "system" fn fastReverse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::fastReverse(this).into()
        }
        unsafe extern "system" fn currentPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdcurrentposition: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::currentPosition(this, core::mem::transmute_copy(&pdcurrentposition)).into()
        }
        unsafe extern "system" fn SetcurrentPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dcurrentposition: f64) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::SetcurrentPosition(this, core::mem::transmute_copy(&dcurrentposition)).into()
        }
        unsafe extern "system" fn currentPositionString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcurrentposition: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::currentPositionString(this, core::mem::transmute_copy(&pbstrcurrentposition)).into()
        }
        unsafe extern "system" fn next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::next(this).into()
        }
        unsafe extern "system" fn previous<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::previous(this).into()
        }
        unsafe extern "system" fn currentItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwmpmedia: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPControls_Impl::currentItem(this) {
                Ok(ok__) => {
                    ppiwmpmedia.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpmedia: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::SetcurrentItem(this, windows_core::from_raw_borrowed(&piwmpmedia)).into()
        }
        unsafe extern "system" fn currentMarker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmarker: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::currentMarker(this, core::mem::transmute_copy(&plmarker)).into()
        }
        unsafe extern "system" fn SetcurrentMarker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmarker: i32) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::SetcurrentMarker(this, core::mem::transmute_copy(&lmarker)).into()
        }
        unsafe extern "system" fn playItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpmedia: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPControls_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls_Impl::playItem(this, windows_core::from_raw_borrowed(&piwmpmedia)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_isAvailable: get_isAvailable::<Identity, OFFSET>,
            play: play::<Identity, OFFSET>,
            stop: stop::<Identity, OFFSET>,
            pause: pause::<Identity, OFFSET>,
            fastForward: fastForward::<Identity, OFFSET>,
            fastReverse: fastReverse::<Identity, OFFSET>,
            currentPosition: currentPosition::<Identity, OFFSET>,
            SetcurrentPosition: SetcurrentPosition::<Identity, OFFSET>,
            currentPositionString: currentPositionString::<Identity, OFFSET>,
            next: next::<Identity, OFFSET>,
            previous: previous::<Identity, OFFSET>,
            currentItem: currentItem::<Identity, OFFSET>,
            SetcurrentItem: SetcurrentItem::<Identity, OFFSET>,
            currentMarker: currentMarker::<Identity, OFFSET>,
            SetcurrentMarker: SetcurrentMarker::<Identity, OFFSET>,
            playItem: playItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPControls as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPControls2_Impl: Sized + IWMPControls_Impl {
    fn step(&self, lstep: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPControls2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPControls2_Vtbl
    where
        Identity: IWMPControls2_Impl,
    {
        unsafe extern "system" fn step<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstep: i32) -> windows_core::HRESULT
        where
            Identity: IWMPControls2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls2_Impl::step(this, core::mem::transmute_copy(&lstep)).into()
        }
        Self { base__: IWMPControls_Vtbl::new::<Identity, OFFSET>(), step: step::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPControls2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPControls as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPControls3_Impl: Sized + IWMPControls2_Impl {
    fn audioLanguageCount(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn getAudioLanguageID(&self, lindex: i32, pllangid: *mut i32) -> windows_core::Result<()>;
    fn getAudioLanguageDescription(&self, lindex: i32, pbstrlangdesc: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn currentAudioLanguage(&self, pllangid: *mut i32) -> windows_core::Result<()>;
    fn SetcurrentAudioLanguage(&self, llangid: i32) -> windows_core::Result<()>;
    fn currentAudioLanguageIndex(&self, plindex: *mut i32) -> windows_core::Result<()>;
    fn SetcurrentAudioLanguageIndex(&self, lindex: i32) -> windows_core::Result<()>;
    fn getLanguageName(&self, llangid: i32, pbstrlangname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn currentPositionTimecode(&self, bstrtimecode: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetcurrentPositionTimecode(&self, bstrtimecode: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPControls3 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPControls3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPControls3_Vtbl
    where
        Identity: IWMPControls3_Impl,
    {
        unsafe extern "system" fn audioLanguageCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPControls3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls3_Impl::audioLanguageCount(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getAudioLanguageID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pllangid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPControls3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls3_Impl::getAudioLanguageID(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn getAudioLanguageDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstrlangdesc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPControls3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls3_Impl::getAudioLanguageDescription(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pbstrlangdesc)).into()
        }
        unsafe extern "system" fn currentAudioLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllangid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPControls3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls3_Impl::currentAudioLanguage(this, core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn SetcurrentAudioLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, llangid: i32) -> windows_core::HRESULT
        where
            Identity: IWMPControls3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls3_Impl::SetcurrentAudioLanguage(this, core::mem::transmute_copy(&llangid)).into()
        }
        unsafe extern "system" fn currentAudioLanguageIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plindex: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPControls3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls3_Impl::currentAudioLanguageIndex(this, core::mem::transmute_copy(&plindex)).into()
        }
        unsafe extern "system" fn SetcurrentAudioLanguageIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT
        where
            Identity: IWMPControls3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls3_Impl::SetcurrentAudioLanguageIndex(this, core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn getLanguageName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, llangid: i32, pbstrlangname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPControls3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls3_Impl::getLanguageName(this, core::mem::transmute_copy(&llangid), core::mem::transmute_copy(&pbstrlangname)).into()
        }
        unsafe extern "system" fn currentPositionTimecode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtimecode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPControls3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls3_Impl::currentPositionTimecode(this, core::mem::transmute_copy(&bstrtimecode)).into()
        }
        unsafe extern "system" fn SetcurrentPositionTimecode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtimecode: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPControls3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPControls3_Impl::SetcurrentPositionTimecode(this, core::mem::transmute(&bstrtimecode)).into()
        }
        Self {
            base__: IWMPControls2_Vtbl::new::<Identity, OFFSET>(),
            audioLanguageCount: audioLanguageCount::<Identity, OFFSET>,
            getAudioLanguageID: getAudioLanguageID::<Identity, OFFSET>,
            getAudioLanguageDescription: getAudioLanguageDescription::<Identity, OFFSET>,
            currentAudioLanguage: currentAudioLanguage::<Identity, OFFSET>,
            SetcurrentAudioLanguage: SetcurrentAudioLanguage::<Identity, OFFSET>,
            currentAudioLanguageIndex: currentAudioLanguageIndex::<Identity, OFFSET>,
            SetcurrentAudioLanguageIndex: SetcurrentAudioLanguageIndex::<Identity, OFFSET>,
            getLanguageName: getLanguageName::<Identity, OFFSET>,
            currentPositionTimecode: currentPositionTimecode::<Identity, OFFSET>,
            SetcurrentPositionTimecode: SetcurrentPositionTimecode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPControls3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPControls as windows_core::Interface>::IID || iid == &<IWMPControls2 as windows_core::Interface>::IID
    }
}
pub trait IWMPConvert_Impl: Sized {
    fn ConvertFile(&self, bstrinputfile: &windows_core::BSTR, bstrdestinationfolder: &windows_core::BSTR, pbstroutputfile: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetErrorURL(&self, pbstrurl: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPConvert {}
impl IWMPConvert_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPConvert_Vtbl
    where
        Identity: IWMPConvert_Impl,
    {
        unsafe extern "system" fn ConvertFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinputfile: core::mem::MaybeUninit<windows_core::BSTR>, bstrdestinationfolder: core::mem::MaybeUninit<windows_core::BSTR>, pbstroutputfile: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPConvert_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPConvert_Impl::ConvertFile(this, core::mem::transmute(&bstrinputfile), core::mem::transmute(&bstrdestinationfolder), core::mem::transmute_copy(&pbstroutputfile)).into()
        }
        unsafe extern "system" fn GetErrorURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPConvert_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPConvert_Impl::GetErrorURL(this, core::mem::transmute_copy(&pbstrurl)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConvertFile: ConvertFile::<Identity, OFFSET>,
            GetErrorURL: GetErrorURL::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPConvert as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCore_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn close(&self) -> windows_core::Result<()>;
    fn URL(&self, pbstrurl: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetURL(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn openState(&self, pwmpos: *mut WMPOpenState) -> windows_core::Result<()>;
    fn playState(&self, pwmpps: *mut WMPPlayState) -> windows_core::Result<()>;
    fn controls(&self) -> windows_core::Result<IWMPControls>;
    fn settings(&self) -> windows_core::Result<IWMPSettings>;
    fn currentMedia(&self) -> windows_core::Result<IWMPMedia>;
    fn SetcurrentMedia(&self, pmedia: Option<&IWMPMedia>) -> windows_core::Result<()>;
    fn mediaCollection(&self) -> windows_core::Result<IWMPMediaCollection>;
    fn playlistCollection(&self) -> windows_core::Result<IWMPPlaylistCollection>;
    fn versionInfo(&self, pbstrversioninfo: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn launchURL(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn network(&self) -> windows_core::Result<IWMPNetwork>;
    fn currentPlaylist(&self) -> windows_core::Result<IWMPPlaylist>;
    fn SetcurrentPlaylist(&self, ppl: Option<&IWMPPlaylist>) -> windows_core::Result<()>;
    fn cdromCollection(&self) -> windows_core::Result<IWMPCdromCollection>;
    fn closedCaption(&self) -> windows_core::Result<IWMPClosedCaption>;
    fn isOnline(&self, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn error(&self) -> windows_core::Result<IWMPError>;
    fn status(&self, pbstrstatus: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPCore {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPCore_Vtbl
    where
        Identity: IWMPCore_Impl,
    {
        unsafe extern "system" fn close<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::close(this).into()
        }
        unsafe extern "system" fn URL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::URL(this, core::mem::transmute_copy(&pbstrurl)).into()
        }
        unsafe extern "system" fn SetURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::SetURL(this, core::mem::transmute(&bstrurl)).into()
        }
        unsafe extern "system" fn openState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpos: *mut WMPOpenState) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::openState(this, core::mem::transmute_copy(&pwmpos)).into()
        }
        unsafe extern "system" fn playState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpps: *mut WMPPlayState) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::playState(this, core::mem::transmute_copy(&pwmpps)).into()
        }
        unsafe extern "system" fn controls<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore_Impl::controls(this) {
                Ok(ok__) => {
                    ppcontrol.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn settings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsettings: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore_Impl::settings(this) {
                Ok(ok__) => {
                    ppsettings.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentMedia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmedia: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore_Impl::currentMedia(this) {
                Ok(ok__) => {
                    ppmedia.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentMedia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::SetcurrentMedia(this, windows_core::from_raw_borrowed(&pmedia)).into()
        }
        unsafe extern "system" fn mediaCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediacollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore_Impl::mediaCollection(this) {
                Ok(ok__) => {
                    ppmediacollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn playlistCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplaylistcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore_Impl::playlistCollection(this) {
                Ok(ok__) => {
                    ppplaylistcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn versionInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrversioninfo: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::versionInfo(this, core::mem::transmute_copy(&pbstrversioninfo)).into()
        }
        unsafe extern "system" fn launchURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::launchURL(this, core::mem::transmute(&bstrurl)).into()
        }
        unsafe extern "system" fn network<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppqni: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore_Impl::network(this) {
                Ok(ok__) => {
                    ppqni.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn currentPlaylist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore_Impl::currentPlaylist(this) {
                Ok(ok__) => {
                    pppl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetcurrentPlaylist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppl: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::SetcurrentPlaylist(this, windows_core::from_raw_borrowed(&ppl)).into()
        }
        unsafe extern "system" fn cdromCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcdromcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore_Impl::cdromCollection(this) {
                Ok(ok__) => {
                    ppcdromcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn closedCaption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclosedcaption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore_Impl::closedCaption(this) {
                Ok(ok__) => {
                    ppclosedcaption.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isOnline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::isOnline(this, core::mem::transmute_copy(&pfonline)).into()
        }
        unsafe extern "system" fn error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore_Impl::error(this) {
                Ok(ok__) => {
                    pperror.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn status<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrstatus: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPCore_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPCore_Impl::status(this, core::mem::transmute_copy(&pbstrstatus)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            close: close::<Identity, OFFSET>,
            URL: URL::<Identity, OFFSET>,
            SetURL: SetURL::<Identity, OFFSET>,
            openState: openState::<Identity, OFFSET>,
            playState: playState::<Identity, OFFSET>,
            controls: controls::<Identity, OFFSET>,
            settings: settings::<Identity, OFFSET>,
            currentMedia: currentMedia::<Identity, OFFSET>,
            SetcurrentMedia: SetcurrentMedia::<Identity, OFFSET>,
            mediaCollection: mediaCollection::<Identity, OFFSET>,
            playlistCollection: playlistCollection::<Identity, OFFSET>,
            versionInfo: versionInfo::<Identity, OFFSET>,
            launchURL: launchURL::<Identity, OFFSET>,
            network: network::<Identity, OFFSET>,
            currentPlaylist: currentPlaylist::<Identity, OFFSET>,
            SetcurrentPlaylist: SetcurrentPlaylist::<Identity, OFFSET>,
            cdromCollection: cdromCollection::<Identity, OFFSET>,
            closedCaption: closedCaption::<Identity, OFFSET>,
            isOnline: isOnline::<Identity, OFFSET>,
            error: error::<Identity, OFFSET>,
            status: status::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCore as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCore2_Impl: Sized + IWMPCore_Impl {
    fn dvd(&self) -> windows_core::Result<IWMPDVD>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPCore2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPCore2_Vtbl
    where
        Identity: IWMPCore2_Impl,
    {
        unsafe extern "system" fn dvd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdvd: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore2_Impl::dvd(this) {
                Ok(ok__) => {
                    ppdvd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IWMPCore_Vtbl::new::<Identity, OFFSET>(), dvd: dvd::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCore2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPCore3_Impl: Sized + IWMPCore2_Impl {
    fn newPlaylist(&self, bstrname: &windows_core::BSTR, bstrurl: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn newMedia(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<IWMPMedia>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPCore3 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPCore3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPCore3_Vtbl
    where
        Identity: IWMPCore3_Impl,
    {
        unsafe extern "system" fn newPlaylist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, ppplaylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore3_Impl::newPlaylist(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrurl)) {
                Ok(ok__) => {
                    ppplaylist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn newMedia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, ppmedia: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPCore3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPCore3_Impl::newMedia(this, core::mem::transmute(&bstrurl)) {
                Ok(ok__) => {
                    ppmedia.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IWMPCore2_Vtbl::new::<Identity, OFFSET>(), newPlaylist: newPlaylist::<Identity, OFFSET>, newMedia: newMedia::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPCore3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID || iid == &<IWMPCore2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDVD_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_isAvailable(&self, bstritem: &windows_core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn domain(&self, strdomain: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn topMenu(&self) -> windows_core::Result<()>;
    fn titleMenu(&self) -> windows_core::Result<()>;
    fn back(&self) -> windows_core::Result<()>;
    fn resume(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPDVD {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDVD_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPDVD_Vtbl
    where
        Identity: IWMPDVD_Impl,
    {
        unsafe extern "system" fn get_isAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritem: core::mem::MaybeUninit<windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPDVD_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDVD_Impl::get_isAvailable(this, core::mem::transmute(&bstritem), core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn domain<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdomain: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPDVD_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDVD_Impl::domain(this, core::mem::transmute_copy(&strdomain)).into()
        }
        unsafe extern "system" fn topMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDVD_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDVD_Impl::topMenu(this).into()
        }
        unsafe extern "system" fn titleMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDVD_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDVD_Impl::titleMenu(this).into()
        }
        unsafe extern "system" fn back<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDVD_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDVD_Impl::back(this).into()
        }
        unsafe extern "system" fn resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDVD_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDVD_Impl::resume(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_isAvailable: get_isAvailable::<Identity, OFFSET>,
            domain: domain::<Identity, OFFSET>,
            topMenu: topMenu::<Identity, OFFSET>,
            titleMenu: titleMenu::<Identity, OFFSET>,
            back: back::<Identity, OFFSET>,
            resume: resume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPDVD as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn id(&self, plid: *mut i32) -> windows_core::Result<()>;
    fn count(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn item(&self, litem: i32) -> windows_core::Result<IWMPDownloadItem2>;
    fn startDownload(&self, bstrsourceurl: &windows_core::BSTR, bstrtype: &windows_core::BSTR) -> windows_core::Result<IWMPDownloadItem2>;
    fn removeItem(&self, litem: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPDownloadCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPDownloadCollection_Vtbl
    where
        Identity: IWMPDownloadCollection_Impl,
    {
        unsafe extern "system" fn id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadCollection_Impl::id(this, core::mem::transmute_copy(&plid)).into()
        }
        unsafe extern "system" fn count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadCollection_Impl::count(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, litem: i32, ppdownload: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPDownloadCollection_Impl::item(this, core::mem::transmute_copy(&litem)) {
                Ok(ok__) => {
                    ppdownload.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsourceurl: core::mem::MaybeUninit<windows_core::BSTR>, bstrtype: core::mem::MaybeUninit<windows_core::BSTR>, ppdownload: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPDownloadCollection_Impl::startDownload(this, core::mem::transmute(&bstrsourceurl), core::mem::transmute(&bstrtype)) {
                Ok(ok__) => {
                    ppdownload.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, litem: i32) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadCollection_Impl::removeItem(this, core::mem::transmute_copy(&litem)).into()
        }
        unsafe extern "system" fn Clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadCollection_Impl::Clear(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            id: id::<Identity, OFFSET>,
            count: count::<Identity, OFFSET>,
            item: item::<Identity, OFFSET>,
            startDownload: startDownload::<Identity, OFFSET>,
            removeItem: removeItem::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPDownloadCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn sourceURL(&self, pbstrurl: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn size(&self, plsize: *mut i32) -> windows_core::Result<()>;
    fn r#type(&self, pbstrtype: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn progress(&self, plprogress: *mut i32) -> windows_core::Result<()>;
    fn downloadState(&self, pwmpsdls: *mut WMPSubscriptionDownloadState) -> windows_core::Result<()>;
    fn pause(&self) -> windows_core::Result<()>;
    fn resume(&self) -> windows_core::Result<()>;
    fn cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPDownloadItem {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPDownloadItem_Vtbl
    where
        Identity: IWMPDownloadItem_Impl,
    {
        unsafe extern "system" fn sourceURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadItem_Impl::sourceURL(this, core::mem::transmute_copy(&pbstrurl)).into()
        }
        unsafe extern "system" fn size<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsize: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadItem_Impl::size(this, core::mem::transmute_copy(&plsize)).into()
        }
        unsafe extern "system" fn r#type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadItem_Impl::r#type(this, core::mem::transmute_copy(&pbstrtype)).into()
        }
        unsafe extern "system" fn progress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprogress: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadItem_Impl::progress(this, core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn downloadState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpsdls: *mut WMPSubscriptionDownloadState) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadItem_Impl::downloadState(this, core::mem::transmute_copy(&pwmpsdls)).into()
        }
        unsafe extern "system" fn pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadItem_Impl::pause(this).into()
        }
        unsafe extern "system" fn resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadItem_Impl::resume(this).into()
        }
        unsafe extern "system" fn cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadItem_Impl::cancel(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            sourceURL: sourceURL::<Identity, OFFSET>,
            size: size::<Identity, OFFSET>,
            r#type: r#type::<Identity, OFFSET>,
            progress: progress::<Identity, OFFSET>,
            downloadState: downloadState::<Identity, OFFSET>,
            pause: pause::<Identity, OFFSET>,
            resume: resume::<Identity, OFFSET>,
            cancel: cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPDownloadItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadItem2_Impl: Sized + IWMPDownloadItem_Impl {
    fn getItemInfo(&self, bstritemname: &windows_core::BSTR, pbstrval: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPDownloadItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadItem2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPDownloadItem2_Vtbl
    where
        Identity: IWMPDownloadItem2_Impl,
    {
        unsafe extern "system" fn getItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPDownloadItem2_Impl::getItemInfo(this, core::mem::transmute(&bstritemname), core::mem::transmute_copy(&pbstrval)).into()
        }
        Self { base__: IWMPDownloadItem_Vtbl::new::<Identity, OFFSET>(), getItemInfo: getItemInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPDownloadItem2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPDownloadItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPDownloadManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn getDownloadCollection(&self, lcollectionid: i32) -> windows_core::Result<IWMPDownloadCollection>;
    fn createDownloadCollection(&self) -> windows_core::Result<IWMPDownloadCollection>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPDownloadManager {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPDownloadManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPDownloadManager_Vtbl
    where
        Identity: IWMPDownloadManager_Impl,
    {
        unsafe extern "system" fn getDownloadCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcollectionid: i32, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPDownloadManager_Impl::getDownloadCollection(this, core::mem::transmute_copy(&lcollectionid)) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDownloadCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPDownloadManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPDownloadManager_Impl::createDownloadCollection(this) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            getDownloadCollection: getDownloadCollection::<Identity, OFFSET>,
            createDownloadCollection: createDownloadCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPDownloadManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IWMPEffects_Impl: Sized {
    fn Render(&self, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn MediaInfo(&self, lchannelcount: i32, lsamplerate: i32, bstrtitle: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCapabilities(&self, pdwcapabilities: *mut u32) -> windows_core::Result<()>;
    fn GetTitle(&self, bstrtitle: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPresetTitle(&self, npreset: i32, bstrpresettitle: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPresetCount(&self, pnpresetcount: *mut i32) -> windows_core::Result<()>;
    fn SetCurrentPreset(&self, npreset: i32) -> windows_core::Result<()>;
    fn GetCurrentPreset(&self, pnpreset: *mut i32) -> windows_core::Result<()>;
    fn DisplayPropertyPage(&self, hwndowner: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn GoFullscreen(&self, ffullscreen: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn RenderFullScreen(&self, plevels: *mut TimedLevel) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::RuntimeName for IWMPEffects {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IWMPEffects_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPEffects_Vtbl
    where
        Identity: IWMPEffects_Impl,
    {
        unsafe extern "system" fn Render<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::Render(this, core::mem::transmute_copy(&plevels), core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&prc)).into()
        }
        unsafe extern "system" fn MediaInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lchannelcount: i32, lsamplerate: i32, bstrtitle: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::MediaInfo(this, core::mem::transmute_copy(&lchannelcount), core::mem::transmute_copy(&lsamplerate), core::mem::transmute(&bstrtitle)).into()
        }
        unsafe extern "system" fn GetCapabilities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcapabilities: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::GetCapabilities(this, core::mem::transmute_copy(&pdwcapabilities)).into()
        }
        unsafe extern "system" fn GetTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::GetTitle(this, core::mem::transmute_copy(&bstrtitle)).into()
        }
        unsafe extern "system" fn GetPresetTitle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, npreset: i32, bstrpresettitle: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::GetPresetTitle(this, core::mem::transmute_copy(&npreset), core::mem::transmute_copy(&bstrpresettitle)).into()
        }
        unsafe extern "system" fn GetPresetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnpresetcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::GetPresetCount(this, core::mem::transmute_copy(&pnpresetcount)).into()
        }
        unsafe extern "system" fn SetCurrentPreset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, npreset: i32) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::SetCurrentPreset(this, core::mem::transmute_copy(&npreset)).into()
        }
        unsafe extern "system" fn GetCurrentPreset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnpreset: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::GetCurrentPreset(this, core::mem::transmute_copy(&pnpreset)).into()
        }
        unsafe extern "system" fn DisplayPropertyPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndowner: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::DisplayPropertyPage(this, core::mem::transmute_copy(&hwndowner)).into()
        }
        unsafe extern "system" fn GoFullscreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::GoFullscreen(this, core::mem::transmute_copy(&ffullscreen)).into()
        }
        unsafe extern "system" fn RenderFullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevels: *mut TimedLevel) -> windows_core::HRESULT
        where
            Identity: IWMPEffects_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects_Impl::RenderFullScreen(this, core::mem::transmute_copy(&plevels)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Render: Render::<Identity, OFFSET>,
            MediaInfo: MediaInfo::<Identity, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            GetTitle: GetTitle::<Identity, OFFSET>,
            GetPresetTitle: GetPresetTitle::<Identity, OFFSET>,
            GetPresetCount: GetPresetCount::<Identity, OFFSET>,
            SetCurrentPreset: SetCurrentPreset::<Identity, OFFSET>,
            GetCurrentPreset: GetCurrentPreset::<Identity, OFFSET>,
            DisplayPropertyPage: DisplayPropertyPage::<Identity, OFFSET>,
            GoFullscreen: GoFullscreen::<Identity, OFFSET>,
            RenderFullScreen: RenderFullScreen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPEffects as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IWMPEffects2_Impl: Sized + IWMPEffects_Impl {
    fn SetCore(&self, pplayer: Option<&IWMPCore>) -> windows_core::Result<()>;
    fn Create(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn Destroy(&self) -> windows_core::Result<()>;
    fn NotifyNewMedia(&self, pmedia: Option<&IWMPMedia>) -> windows_core::Result<()>;
    fn OnWindowMessage(&self, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> windows_core::Result<()>;
    fn RenderWindowed(&self, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IWMPEffects2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl IWMPEffects2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPEffects2_Vtbl
    where
        Identity: IWMPEffects2_Impl,
    {
        unsafe extern "system" fn SetCore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplayer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPEffects2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects2_Impl::SetCore(this, windows_core::from_raw_borrowed(&pplayer)).into()
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IWMPEffects2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects2_Impl::Create(this, core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn Destroy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPEffects2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects2_Impl::Destroy(this).into()
        }
        unsafe extern "system" fn NotifyNewMedia<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPEffects2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects2_Impl::NotifyNewMedia(this, windows_core::from_raw_borrowed(&pmedia)).into()
        }
        unsafe extern "system" fn OnWindowMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> windows_core::HRESULT
        where
            Identity: IWMPEffects2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects2_Impl::OnWindowMessage(this, core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam), core::mem::transmute_copy(&plresultparam)).into()
        }
        unsafe extern "system" fn RenderWindowed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPEffects2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEffects2_Impl::RenderWindowed(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&frequiredrender)).into()
        }
        Self {
            base__: IWMPEffects_Vtbl::new::<Identity, OFFSET>(),
            SetCore: SetCore::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Destroy: Destroy::<Identity, OFFSET>,
            NotifyNewMedia: NotifyNewMedia::<Identity, OFFSET>,
            OnWindowMessage: OnWindowMessage::<Identity, OFFSET>,
            RenderWindowed: RenderWindowed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPEffects2 as windows_core::Interface>::IID || iid == &<IWMPEffects as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPError_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn clearErrorQueue(&self) -> windows_core::Result<()>;
    fn errorCount(&self, plnumerrors: *mut i32) -> windows_core::Result<()>;
    fn get_item(&self, dwindex: i32) -> windows_core::Result<IWMPErrorItem>;
    fn webHelp(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPError {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPError_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPError_Vtbl
    where
        Identity: IWMPError_Impl,
    {
        unsafe extern "system" fn clearErrorQueue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPError_Impl::clearErrorQueue(this).into()
        }
        unsafe extern "system" fn errorCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plnumerrors: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPError_Impl::errorCount(this, core::mem::transmute_copy(&plnumerrors)).into()
        }
        unsafe extern "system" fn get_item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: i32, pperroritem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPError_Impl::get_item(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    pperroritem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn webHelp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPError_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPError_Impl::webHelp(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            clearErrorQueue: clearErrorQueue::<Identity, OFFSET>,
            errorCount: errorCount::<Identity, OFFSET>,
            get_item: get_item::<Identity, OFFSET>,
            webHelp: webHelp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPError as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPErrorItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn errorCode(&self, phr: *mut i32) -> windows_core::Result<()>;
    fn errorDescription(&self, pbstrdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn errorContext(&self, pvarcontext: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn remedy(&self, plremedy: *mut i32) -> windows_core::Result<()>;
    fn customUrl(&self, pbstrcustomurl: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPErrorItem {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPErrorItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPErrorItem_Vtbl
    where
        Identity: IWMPErrorItem_Impl,
    {
        unsafe extern "system" fn errorCode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPErrorItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPErrorItem_Impl::errorCode(this, core::mem::transmute_copy(&phr)).into()
        }
        unsafe extern "system" fn errorDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPErrorItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPErrorItem_Impl::errorDescription(this, core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn errorContext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcontext: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPErrorItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPErrorItem_Impl::errorContext(this, core::mem::transmute_copy(&pvarcontext)).into()
        }
        unsafe extern "system" fn remedy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plremedy: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPErrorItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPErrorItem_Impl::remedy(this, core::mem::transmute_copy(&plremedy)).into()
        }
        unsafe extern "system" fn customUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcustomurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPErrorItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPErrorItem_Impl::customUrl(this, core::mem::transmute_copy(&pbstrcustomurl)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            errorCode: errorCode::<Identity, OFFSET>,
            errorDescription: errorDescription::<Identity, OFFSET>,
            errorContext: errorContext::<Identity, OFFSET>,
            remedy: remedy::<Identity, OFFSET>,
            customUrl: customUrl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPErrorItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPErrorItem2_Impl: Sized + IWMPErrorItem_Impl {
    fn condition(&self, plcondition: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPErrorItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPErrorItem2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPErrorItem2_Vtbl
    where
        Identity: IWMPErrorItem2_Impl,
    {
        unsafe extern "system" fn condition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcondition: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPErrorItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPErrorItem2_Impl::condition(this, core::mem::transmute_copy(&plcondition)).into()
        }
        Self { base__: IWMPErrorItem_Vtbl::new::<Identity, OFFSET>(), condition: condition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPErrorItem2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPErrorItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPEvents_Impl: Sized {
    fn OpenStateChange(&self, newstate: i32);
    fn PlayStateChange(&self, newstate: i32);
    fn AudioLanguageChange(&self, langid: i32);
    fn StatusChange(&self);
    fn ScriptCommand(&self, sctype: &windows_core::BSTR, param: &windows_core::BSTR);
    fn NewStream(&self);
    fn Disconnect(&self, result: i32);
    fn Buffering(&self, start: super::super::Foundation::VARIANT_BOOL);
    fn Error(&self);
    fn Warning(&self, warningtype: i32, param: i32, description: &windows_core::BSTR);
    fn EndOfStream(&self, result: i32);
    fn PositionChange(&self, oldposition: f64, newposition: f64);
    fn MarkerHit(&self, markernum: i32);
    fn DurationUnitChange(&self, newdurationunit: i32);
    fn CdromMediaChange(&self, cdromnum: i32);
    fn PlaylistChange(&self, playlist: Option<&super::super::System::Com::IDispatch>, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistChange(&self, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistItemAvailable(&self, bstritemname: &windows_core::BSTR);
    fn MediaChange(&self, item: Option<&super::super::System::Com::IDispatch>);
    fn CurrentMediaItemAvailable(&self, bstritemname: &windows_core::BSTR);
    fn CurrentItemChange(&self, pdispmedia: Option<&super::super::System::Com::IDispatch>);
    fn MediaCollectionChange(&self);
    fn MediaCollectionAttributeStringAdded(&self, bstrattribname: &windows_core::BSTR, bstrattribval: &windows_core::BSTR);
    fn MediaCollectionAttributeStringRemoved(&self, bstrattribname: &windows_core::BSTR, bstrattribval: &windows_core::BSTR);
    fn MediaCollectionAttributeStringChanged(&self, bstrattribname: &windows_core::BSTR, bstroldattribval: &windows_core::BSTR, bstrnewattribval: &windows_core::BSTR);
    fn PlaylistCollectionChange(&self);
    fn PlaylistCollectionPlaylistAdded(&self, bstrplaylistname: &windows_core::BSTR);
    fn PlaylistCollectionPlaylistRemoved(&self, bstrplaylistname: &windows_core::BSTR);
    fn PlaylistCollectionPlaylistSetAsDeleted(&self, bstrplaylistname: &windows_core::BSTR, varfisdeleted: super::super::Foundation::VARIANT_BOOL);
    fn ModeChange(&self, modename: &windows_core::BSTR, newvalue: super::super::Foundation::VARIANT_BOOL);
    fn MediaError(&self, pmediaobject: Option<&super::super::System::Com::IDispatch>);
    fn OpenPlaylistSwitch(&self, pitem: Option<&super::super::System::Com::IDispatch>);
    fn DomainChange(&self, strdomain: &windows_core::BSTR);
    fn SwitchedToPlayerApplication(&self);
    fn SwitchedToControl(&self);
    fn PlayerDockedStateChange(&self);
    fn PlayerReconnect(&self);
    fn Click(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn DoubleClick(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn KeyDown(&self, nkeycode: i16, nshiftstate: i16);
    fn KeyPress(&self, nkeyascii: i16);
    fn KeyUp(&self, nkeycode: i16, nshiftstate: i16);
    fn MouseDown(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn MouseMove(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn MouseUp(&self, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPEvents {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPEvents_Vtbl
    where
        Identity: IWMPEvents_Impl,
    {
        unsafe extern "system" fn OpenStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::OpenStateChange(this, core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn PlayStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::PlayStateChange(this, core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn AudioLanguageChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::AudioLanguageChange(this, core::mem::transmute_copy(&langid))
        }
        unsafe extern "system" fn StatusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::StatusChange(this)
        }
        unsafe extern "system" fn ScriptCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sctype: core::mem::MaybeUninit<windows_core::BSTR>, param: core::mem::MaybeUninit<windows_core::BSTR>)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::ScriptCommand(this, core::mem::transmute(&sctype), core::mem::transmute(&param))
        }
        unsafe extern "system" fn NewStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::NewStream(this)
        }
        unsafe extern "system" fn Disconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::Disconnect(this, core::mem::transmute_copy(&result))
        }
        unsafe extern "system" fn Buffering<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: super::super::Foundation::VARIANT_BOOL)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::Buffering(this, core::mem::transmute_copy(&start))
        }
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::Error(this)
        }
        unsafe extern "system" fn Warning<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, warningtype: i32, param: i32, description: core::mem::MaybeUninit<windows_core::BSTR>)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::Warning(this, core::mem::transmute_copy(&warningtype), core::mem::transmute_copy(&param), core::mem::transmute(&description))
        }
        unsafe extern "system" fn EndOfStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::EndOfStream(this, core::mem::transmute_copy(&result))
        }
        unsafe extern "system" fn PositionChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldposition: f64, newposition: f64)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::PositionChange(this, core::mem::transmute_copy(&oldposition), core::mem::transmute_copy(&newposition))
        }
        unsafe extern "system" fn MarkerHit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, markernum: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::MarkerHit(this, core::mem::transmute_copy(&markernum))
        }
        unsafe extern "system" fn DurationUnitChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newdurationunit: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::DurationUnitChange(this, core::mem::transmute_copy(&newdurationunit))
        }
        unsafe extern "system" fn CdromMediaChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdromnum: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::CdromMediaChange(this, core::mem::transmute_copy(&cdromnum))
        }
        unsafe extern "system" fn PlaylistChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, playlist: *mut core::ffi::c_void, change: WMPPlaylistChangeEventType)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::PlaylistChange(this, windows_core::from_raw_borrowed(&playlist), core::mem::transmute_copy(&change))
        }
        unsafe extern "system" fn CurrentPlaylistChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, change: WMPPlaylistChangeEventType)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::CurrentPlaylistChange(this, core::mem::transmute_copy(&change))
        }
        unsafe extern "system" fn CurrentPlaylistItemAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::CurrentPlaylistItemAvailable(this, core::mem::transmute(&bstritemname))
        }
        unsafe extern "system" fn MediaChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::MediaChange(this, windows_core::from_raw_borrowed(&item))
        }
        unsafe extern "system" fn CurrentMediaItemAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::CurrentMediaItemAvailable(this, core::mem::transmute(&bstritemname))
        }
        unsafe extern "system" fn CurrentItemChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdispmedia: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::CurrentItemChange(this, windows_core::from_raw_borrowed(&pdispmedia))
        }
        unsafe extern "system" fn MediaCollectionChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::MediaCollectionChange(this)
        }
        unsafe extern "system" fn MediaCollectionAttributeStringAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribname: core::mem::MaybeUninit<windows_core::BSTR>, bstrattribval: core::mem::MaybeUninit<windows_core::BSTR>)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::MediaCollectionAttributeStringAdded(this, core::mem::transmute(&bstrattribname), core::mem::transmute(&bstrattribval))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringRemoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribname: core::mem::MaybeUninit<windows_core::BSTR>, bstrattribval: core::mem::MaybeUninit<windows_core::BSTR>)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::MediaCollectionAttributeStringRemoved(this, core::mem::transmute(&bstrattribname), core::mem::transmute(&bstrattribval))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribname: core::mem::MaybeUninit<windows_core::BSTR>, bstroldattribval: core::mem::MaybeUninit<windows_core::BSTR>, bstrnewattribval: core::mem::MaybeUninit<windows_core::BSTR>)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::MediaCollectionAttributeStringChanged(this, core::mem::transmute(&bstrattribname), core::mem::transmute(&bstroldattribval), core::mem::transmute(&bstrnewattribval))
        }
        unsafe extern "system" fn PlaylistCollectionChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::PlaylistCollectionChange(this)
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrplaylistname: core::mem::MaybeUninit<windows_core::BSTR>)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::PlaylistCollectionPlaylistAdded(this, core::mem::transmute(&bstrplaylistname))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistRemoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrplaylistname: core::mem::MaybeUninit<windows_core::BSTR>)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::PlaylistCollectionPlaylistRemoved(this, core::mem::transmute(&bstrplaylistname))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistSetAsDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrplaylistname: core::mem::MaybeUninit<windows_core::BSTR>, varfisdeleted: super::super::Foundation::VARIANT_BOOL)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::PlaylistCollectionPlaylistSetAsDeleted(this, core::mem::transmute(&bstrplaylistname), core::mem::transmute_copy(&varfisdeleted))
        }
        unsafe extern "system" fn ModeChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, modename: core::mem::MaybeUninit<windows_core::BSTR>, newvalue: super::super::Foundation::VARIANT_BOOL)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::ModeChange(this, core::mem::transmute(&modename), core::mem::transmute_copy(&newvalue))
        }
        unsafe extern "system" fn MediaError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediaobject: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::MediaError(this, windows_core::from_raw_borrowed(&pmediaobject))
        }
        unsafe extern "system" fn OpenPlaylistSwitch<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::OpenPlaylistSwitch(this, windows_core::from_raw_borrowed(&pitem))
        }
        unsafe extern "system" fn DomainChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdomain: core::mem::MaybeUninit<windows_core::BSTR>)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::DomainChange(this, core::mem::transmute(&strdomain))
        }
        unsafe extern "system" fn SwitchedToPlayerApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::SwitchedToPlayerApplication(this)
        }
        unsafe extern "system" fn SwitchedToControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::SwitchedToControl(this)
        }
        unsafe extern "system" fn PlayerDockedStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::PlayerDockedStateChange(this)
        }
        unsafe extern "system" fn PlayerReconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::PlayerReconnect(this)
        }
        unsafe extern "system" fn Click<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::Click(this, core::mem::transmute_copy(&nbutton), core::mem::transmute_copy(&nshiftstate), core::mem::transmute_copy(&fx), core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn DoubleClick<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::DoubleClick(this, core::mem::transmute_copy(&nbutton), core::mem::transmute_copy(&nshiftstate), core::mem::transmute_copy(&fx), core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn KeyDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nkeycode: i16, nshiftstate: i16)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::KeyDown(this, core::mem::transmute_copy(&nkeycode), core::mem::transmute_copy(&nshiftstate))
        }
        unsafe extern "system" fn KeyPress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nkeyascii: i16)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::KeyPress(this, core::mem::transmute_copy(&nkeyascii))
        }
        unsafe extern "system" fn KeyUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nkeycode: i16, nshiftstate: i16)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::KeyUp(this, core::mem::transmute_copy(&nkeycode), core::mem::transmute_copy(&nshiftstate))
        }
        unsafe extern "system" fn MouseDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::MouseDown(this, core::mem::transmute_copy(&nbutton), core::mem::transmute_copy(&nshiftstate), core::mem::transmute_copy(&fx), core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn MouseMove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::MouseMove(this, core::mem::transmute_copy(&nbutton), core::mem::transmute_copy(&nshiftstate), core::mem::transmute_copy(&fx), core::mem::transmute_copy(&fy))
        }
        unsafe extern "system" fn MouseUp<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32)
        where
            Identity: IWMPEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents_Impl::MouseUp(this, core::mem::transmute_copy(&nbutton), core::mem::transmute_copy(&nshiftstate), core::mem::transmute_copy(&fx), core::mem::transmute_copy(&fy))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenStateChange: OpenStateChange::<Identity, OFFSET>,
            PlayStateChange: PlayStateChange::<Identity, OFFSET>,
            AudioLanguageChange: AudioLanguageChange::<Identity, OFFSET>,
            StatusChange: StatusChange::<Identity, OFFSET>,
            ScriptCommand: ScriptCommand::<Identity, OFFSET>,
            NewStream: NewStream::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            Buffering: Buffering::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
            Warning: Warning::<Identity, OFFSET>,
            EndOfStream: EndOfStream::<Identity, OFFSET>,
            PositionChange: PositionChange::<Identity, OFFSET>,
            MarkerHit: MarkerHit::<Identity, OFFSET>,
            DurationUnitChange: DurationUnitChange::<Identity, OFFSET>,
            CdromMediaChange: CdromMediaChange::<Identity, OFFSET>,
            PlaylistChange: PlaylistChange::<Identity, OFFSET>,
            CurrentPlaylistChange: CurrentPlaylistChange::<Identity, OFFSET>,
            CurrentPlaylistItemAvailable: CurrentPlaylistItemAvailable::<Identity, OFFSET>,
            MediaChange: MediaChange::<Identity, OFFSET>,
            CurrentMediaItemAvailable: CurrentMediaItemAvailable::<Identity, OFFSET>,
            CurrentItemChange: CurrentItemChange::<Identity, OFFSET>,
            MediaCollectionChange: MediaCollectionChange::<Identity, OFFSET>,
            MediaCollectionAttributeStringAdded: MediaCollectionAttributeStringAdded::<Identity, OFFSET>,
            MediaCollectionAttributeStringRemoved: MediaCollectionAttributeStringRemoved::<Identity, OFFSET>,
            MediaCollectionAttributeStringChanged: MediaCollectionAttributeStringChanged::<Identity, OFFSET>,
            PlaylistCollectionChange: PlaylistCollectionChange::<Identity, OFFSET>,
            PlaylistCollectionPlaylistAdded: PlaylistCollectionPlaylistAdded::<Identity, OFFSET>,
            PlaylistCollectionPlaylistRemoved: PlaylistCollectionPlaylistRemoved::<Identity, OFFSET>,
            PlaylistCollectionPlaylistSetAsDeleted: PlaylistCollectionPlaylistSetAsDeleted::<Identity, OFFSET>,
            ModeChange: ModeChange::<Identity, OFFSET>,
            MediaError: MediaError::<Identity, OFFSET>,
            OpenPlaylistSwitch: OpenPlaylistSwitch::<Identity, OFFSET>,
            DomainChange: DomainChange::<Identity, OFFSET>,
            SwitchedToPlayerApplication: SwitchedToPlayerApplication::<Identity, OFFSET>,
            SwitchedToControl: SwitchedToControl::<Identity, OFFSET>,
            PlayerDockedStateChange: PlayerDockedStateChange::<Identity, OFFSET>,
            PlayerReconnect: PlayerReconnect::<Identity, OFFSET>,
            Click: Click::<Identity, OFFSET>,
            DoubleClick: DoubleClick::<Identity, OFFSET>,
            KeyDown: KeyDown::<Identity, OFFSET>,
            KeyPress: KeyPress::<Identity, OFFSET>,
            KeyUp: KeyUp::<Identity, OFFSET>,
            MouseDown: MouseDown::<Identity, OFFSET>,
            MouseMove: MouseMove::<Identity, OFFSET>,
            MouseUp: MouseUp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPEvents2_Impl: Sized + IWMPEvents_Impl {
    fn DeviceConnect(&self, pdevice: Option<&IWMPSyncDevice>);
    fn DeviceDisconnect(&self, pdevice: Option<&IWMPSyncDevice>);
    fn DeviceStatusChange(&self, pdevice: Option<&IWMPSyncDevice>, newstatus: WMPDeviceStatus);
    fn DeviceSyncStateChange(&self, pdevice: Option<&IWMPSyncDevice>, newstate: WMPSyncState);
    fn DeviceSyncError(&self, pdevice: Option<&IWMPSyncDevice>, pmedia: Option<&super::super::System::Com::IDispatch>);
    fn CreatePartnershipComplete(&self, pdevice: Option<&IWMPSyncDevice>, hrresult: windows_core::HRESULT);
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPEvents2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPEvents2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPEvents2_Vtbl
    where
        Identity: IWMPEvents2_Impl,
    {
        unsafe extern "system" fn DeviceConnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents2_Impl::DeviceConnect(this, windows_core::from_raw_borrowed(&pdevice))
        }
        unsafe extern "system" fn DeviceDisconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents2_Impl::DeviceDisconnect(this, windows_core::from_raw_borrowed(&pdevice))
        }
        unsafe extern "system" fn DeviceStatusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, newstatus: WMPDeviceStatus)
        where
            Identity: IWMPEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents2_Impl::DeviceStatusChange(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&newstatus))
        }
        unsafe extern "system" fn DeviceSyncStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, newstate: WMPSyncState)
        where
            Identity: IWMPEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents2_Impl::DeviceSyncStateChange(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&newstate))
        }
        unsafe extern "system" fn DeviceSyncError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents2_Impl::DeviceSyncError(this, windows_core::from_raw_borrowed(&pdevice), windows_core::from_raw_borrowed(&pmedia))
        }
        unsafe extern "system" fn CreatePartnershipComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hrresult: windows_core::HRESULT)
        where
            Identity: IWMPEvents2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents2_Impl::CreatePartnershipComplete(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&hrresult))
        }
        Self {
            base__: IWMPEvents_Vtbl::new::<Identity, OFFSET>(),
            DeviceConnect: DeviceConnect::<Identity, OFFSET>,
            DeviceDisconnect: DeviceDisconnect::<Identity, OFFSET>,
            DeviceStatusChange: DeviceStatusChange::<Identity, OFFSET>,
            DeviceSyncStateChange: DeviceSyncStateChange::<Identity, OFFSET>,
            DeviceSyncError: DeviceSyncError::<Identity, OFFSET>,
            CreatePartnershipComplete: CreatePartnershipComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPEvents2 as windows_core::Interface>::IID || iid == &<IWMPEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPEvents3_Impl: Sized + IWMPEvents2_Impl {
    fn CdromRipStateChange(&self, pcdromrip: Option<&IWMPCdromRip>, wmprs: WMPRipState);
    fn CdromRipMediaError(&self, pcdromrip: Option<&IWMPCdromRip>, pmedia: Option<&super::super::System::Com::IDispatch>);
    fn CdromBurnStateChange(&self, pcdromburn: Option<&IWMPCdromBurn>, wmpbs: WMPBurnState);
    fn CdromBurnMediaError(&self, pcdromburn: Option<&IWMPCdromBurn>, pmedia: Option<&super::super::System::Com::IDispatch>);
    fn CdromBurnError(&self, pcdromburn: Option<&IWMPCdromBurn>, hrerror: windows_core::HRESULT);
    fn LibraryConnect(&self, plibrary: Option<&IWMPLibrary>);
    fn LibraryDisconnect(&self, plibrary: Option<&IWMPLibrary>);
    fn FolderScanStateChange(&self, wmpfss: WMPFolderScanState);
    fn StringCollectionChange(&self, pdispstringcollection: Option<&super::super::System::Com::IDispatch>, change: WMPStringCollectionChangeEventType, lcollectionindex: i32);
    fn MediaCollectionMediaAdded(&self, pdispmedia: Option<&super::super::System::Com::IDispatch>);
    fn MediaCollectionMediaRemoved(&self, pdispmedia: Option<&super::super::System::Com::IDispatch>);
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPEvents3 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPEvents3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPEvents3_Vtbl
    where
        Identity: IWMPEvents3_Impl,
    {
        unsafe extern "system" fn CdromRipStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdromrip: *mut core::ffi::c_void, wmprs: WMPRipState)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::CdromRipStateChange(this, windows_core::from_raw_borrowed(&pcdromrip), core::mem::transmute_copy(&wmprs))
        }
        unsafe extern "system" fn CdromRipMediaError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdromrip: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::CdromRipMediaError(this, windows_core::from_raw_borrowed(&pcdromrip), windows_core::from_raw_borrowed(&pmedia))
        }
        unsafe extern "system" fn CdromBurnStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdromburn: *mut core::ffi::c_void, wmpbs: WMPBurnState)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::CdromBurnStateChange(this, windows_core::from_raw_borrowed(&pcdromburn), core::mem::transmute_copy(&wmpbs))
        }
        unsafe extern "system" fn CdromBurnMediaError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdromburn: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::CdromBurnMediaError(this, windows_core::from_raw_borrowed(&pcdromburn), windows_core::from_raw_borrowed(&pmedia))
        }
        unsafe extern "system" fn CdromBurnError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdromburn: *mut core::ffi::c_void, hrerror: windows_core::HRESULT)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::CdromBurnError(this, windows_core::from_raw_borrowed(&pcdromburn), core::mem::transmute_copy(&hrerror))
        }
        unsafe extern "system" fn LibraryConnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plibrary: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::LibraryConnect(this, windows_core::from_raw_borrowed(&plibrary))
        }
        unsafe extern "system" fn LibraryDisconnect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plibrary: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::LibraryDisconnect(this, windows_core::from_raw_borrowed(&plibrary))
        }
        unsafe extern "system" fn FolderScanStateChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmpfss: WMPFolderScanState)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::FolderScanStateChange(this, core::mem::transmute_copy(&wmpfss))
        }
        unsafe extern "system" fn StringCollectionChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdispstringcollection: *mut core::ffi::c_void, change: WMPStringCollectionChangeEventType, lcollectionindex: i32)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::StringCollectionChange(this, windows_core::from_raw_borrowed(&pdispstringcollection), core::mem::transmute_copy(&change), core::mem::transmute_copy(&lcollectionindex))
        }
        unsafe extern "system" fn MediaCollectionMediaAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdispmedia: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::MediaCollectionMediaAdded(this, windows_core::from_raw_borrowed(&pdispmedia))
        }
        unsafe extern "system" fn MediaCollectionMediaRemoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdispmedia: *mut core::ffi::c_void)
        where
            Identity: IWMPEvents3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents3_Impl::MediaCollectionMediaRemoved(this, windows_core::from_raw_borrowed(&pdispmedia))
        }
        Self {
            base__: IWMPEvents2_Vtbl::new::<Identity, OFFSET>(),
            CdromRipStateChange: CdromRipStateChange::<Identity, OFFSET>,
            CdromRipMediaError: CdromRipMediaError::<Identity, OFFSET>,
            CdromBurnStateChange: CdromBurnStateChange::<Identity, OFFSET>,
            CdromBurnMediaError: CdromBurnMediaError::<Identity, OFFSET>,
            CdromBurnError: CdromBurnError::<Identity, OFFSET>,
            LibraryConnect: LibraryConnect::<Identity, OFFSET>,
            LibraryDisconnect: LibraryDisconnect::<Identity, OFFSET>,
            FolderScanStateChange: FolderScanStateChange::<Identity, OFFSET>,
            StringCollectionChange: StringCollectionChange::<Identity, OFFSET>,
            MediaCollectionMediaAdded: MediaCollectionMediaAdded::<Identity, OFFSET>,
            MediaCollectionMediaRemoved: MediaCollectionMediaRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPEvents3 as windows_core::Interface>::IID || iid == &<IWMPEvents as windows_core::Interface>::IID || iid == &<IWMPEvents2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPEvents4_Impl: Sized + IWMPEvents3_Impl {
    fn DeviceEstimation(&self, pdevice: Option<&IWMPSyncDevice>, hrresult: windows_core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64);
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPEvents4 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPEvents4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPEvents4_Vtbl
    where
        Identity: IWMPEvents4_Impl,
    {
        unsafe extern "system" fn DeviceEstimation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, hrresult: windows_core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64)
        where
            Identity: IWMPEvents4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPEvents4_Impl::DeviceEstimation(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&hrresult), core::mem::transmute_copy(&qwestimatedusedspace), core::mem::transmute_copy(&qwestimatedspace))
        }
        Self { base__: IWMPEvents3_Vtbl::new::<Identity, OFFSET>(), DeviceEstimation: DeviceEstimation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPEvents4 as windows_core::Interface>::IID || iid == &<IWMPEvents as windows_core::Interface>::IID || iid == &<IWMPEvents2 as windows_core::Interface>::IID || iid == &<IWMPEvents3 as windows_core::Interface>::IID
    }
}
pub trait IWMPFolderMonitorServices_Impl: Sized {
    fn count(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn item(&self, lindex: i32, pbstrfolder: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn add(&self, bstrfolder: &windows_core::BSTR) -> windows_core::Result<()>;
    fn remove(&self, lindex: i32) -> windows_core::Result<()>;
    fn scanState(&self, pwmpfss: *mut WMPFolderScanState) -> windows_core::Result<()>;
    fn currentFolder(&self, pbstrfolder: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn scannedFilesCount(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn addedFilesCount(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn updateProgress(&self, plprogress: *mut i32) -> windows_core::Result<()>;
    fn startScan(&self) -> windows_core::Result<()>;
    fn stopScan(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPFolderMonitorServices {}
impl IWMPFolderMonitorServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPFolderMonitorServices_Vtbl
    where
        Identity: IWMPFolderMonitorServices_Impl,
    {
        unsafe extern "system" fn count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::count(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstrfolder: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::item(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pbstrfolder)).into()
        }
        unsafe extern "system" fn add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfolder: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::add(this, core::mem::transmute(&bstrfolder)).into()
        }
        unsafe extern "system" fn remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::remove(this, core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn scanState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpfss: *mut WMPFolderScanState) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::scanState(this, core::mem::transmute_copy(&pwmpfss)).into()
        }
        unsafe extern "system" fn currentFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfolder: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::currentFolder(this, core::mem::transmute_copy(&pbstrfolder)).into()
        }
        unsafe extern "system" fn scannedFilesCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::scannedFilesCount(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn addedFilesCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::addedFilesCount(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn updateProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprogress: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::updateProgress(this, core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn startScan<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::startScan(this).into()
        }
        unsafe extern "system" fn stopScan<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPFolderMonitorServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPFolderMonitorServices_Impl::stopScan(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            count: count::<Identity, OFFSET>,
            item: item::<Identity, OFFSET>,
            add: add::<Identity, OFFSET>,
            remove: remove::<Identity, OFFSET>,
            scanState: scanState::<Identity, OFFSET>,
            currentFolder: currentFolder::<Identity, OFFSET>,
            scannedFilesCount: scannedFilesCount::<Identity, OFFSET>,
            addedFilesCount: addedFilesCount::<Identity, OFFSET>,
            updateProgress: updateProgress::<Identity, OFFSET>,
            startScan: startScan::<Identity, OFFSET>,
            stopScan: stopScan::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPFolderMonitorServices as windows_core::Interface>::IID
    }
}
pub trait IWMPGraphCreation_Impl: Sized {
    fn GraphCreationPreRender(&self, pfiltergraph: Option<&windows_core::IUnknown>, preserved: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GraphCreationPostRender(&self, pfiltergraph: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetGraphCreationFlags(&self, pdwflags: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPGraphCreation {}
impl IWMPGraphCreation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPGraphCreation_Vtbl
    where
        Identity: IWMPGraphCreation_Impl,
    {
        unsafe extern "system" fn GraphCreationPreRender<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiltergraph: *mut core::ffi::c_void, preserved: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPGraphCreation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPGraphCreation_Impl::GraphCreationPreRender(this, windows_core::from_raw_borrowed(&pfiltergraph), windows_core::from_raw_borrowed(&preserved)).into()
        }
        unsafe extern "system" fn GraphCreationPostRender<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfiltergraph: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPGraphCreation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPGraphCreation_Impl::GraphCreationPostRender(this, windows_core::from_raw_borrowed(&pfiltergraph)).into()
        }
        unsafe extern "system" fn GetGraphCreationFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPGraphCreation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPGraphCreation_Impl::GetGraphCreationFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GraphCreationPreRender: GraphCreationPreRender::<Identity, OFFSET>,
            GraphCreationPostRender: GraphCreationPostRender::<Identity, OFFSET>,
            GetGraphCreationFlags: GetGraphCreationFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPGraphCreation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPLibrary_Impl: Sized {
    fn name(&self, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn r#type(&self, pwmplt: *mut WMPLibraryType) -> windows_core::Result<()>;
    fn mediaCollection(&self) -> windows_core::Result<IWMPMediaCollection>;
    fn isIdentical(&self, piwmplibrary: Option<&IWMPLibrary>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPLibrary {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPLibrary_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPLibrary_Vtbl
    where
        Identity: IWMPLibrary_Impl,
    {
        unsafe extern "system" fn name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPLibrary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPLibrary_Impl::name(this, core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn r#type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmplt: *mut WMPLibraryType) -> windows_core::HRESULT
        where
            Identity: IWMPLibrary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPLibrary_Impl::r#type(this, core::mem::transmute_copy(&pwmplt)).into()
        }
        unsafe extern "system" fn mediaCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwmpmediacollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPLibrary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPLibrary_Impl::mediaCollection(this) {
                Ok(ok__) => {
                    ppiwmpmediacollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isIdentical<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmplibrary: *mut core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPLibrary_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPLibrary_Impl::isIdentical(this, windows_core::from_raw_borrowed(&piwmplibrary), core::mem::transmute_copy(&pvbool)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            name: name::<Identity, OFFSET>,
            r#type: r#type::<Identity, OFFSET>,
            mediaCollection: mediaCollection::<Identity, OFFSET>,
            isIdentical: isIdentical::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPLibrary as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPLibrary2_Impl: Sized + IWMPLibrary_Impl {
    fn getItemInfo(&self, bstritemname: &windows_core::BSTR, pbstrval: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPLibrary2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPLibrary2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPLibrary2_Vtbl
    where
        Identity: IWMPLibrary2_Impl,
    {
        unsafe extern "system" fn getItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPLibrary2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPLibrary2_Impl::getItemInfo(this, core::mem::transmute(&bstritemname), core::mem::transmute_copy(&pbstrval)).into()
        }
        Self { base__: IWMPLibrary_Vtbl::new::<Identity, OFFSET>(), getItemInfo: getItemInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPLibrary2 as windows_core::Interface>::IID || iid == &<IWMPLibrary as windows_core::Interface>::IID
    }
}
pub trait IWMPLibraryServices_Impl: Sized {
    fn getCountByType(&self, wmplt: WMPLibraryType, plcount: *mut i32) -> windows_core::Result<()>;
    fn getLibraryByType(&self, wmplt: WMPLibraryType, lindex: i32) -> windows_core::Result<IWMPLibrary>;
}
impl windows_core::RuntimeName for IWMPLibraryServices {}
impl IWMPLibraryServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPLibraryServices_Vtbl
    where
        Identity: IWMPLibraryServices_Impl,
    {
        unsafe extern "system" fn getCountByType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmplt: WMPLibraryType, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPLibraryServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPLibraryServices_Impl::getCountByType(this, core::mem::transmute_copy(&wmplt), core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getLibraryByType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wmplt: WMPLibraryType, lindex: i32, ppiwmplibrary: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPLibraryServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPLibraryServices_Impl::getLibraryByType(this, core::mem::transmute_copy(&wmplt), core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    ppiwmplibrary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getCountByType: getCountByType::<Identity, OFFSET>,
            getLibraryByType: getLibraryByType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPLibraryServices as windows_core::Interface>::IID
    }
}
pub trait IWMPLibrarySharingServices_Impl: Sized {
    fn isLibraryShared(&self, pvbshared: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn isLibrarySharingEnabled(&self, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn showLibrarySharing(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPLibrarySharingServices {}
impl IWMPLibrarySharingServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPLibrarySharingServices_Vtbl
    where
        Identity: IWMPLibrarySharingServices_Impl,
    {
        unsafe extern "system" fn isLibraryShared<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbshared: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPLibrarySharingServices_Impl::isLibraryShared(this, core::mem::transmute_copy(&pvbshared)).into()
        }
        unsafe extern "system" fn isLibrarySharingEnabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPLibrarySharingServices_Impl::isLibrarySharingEnabled(this, core::mem::transmute_copy(&pvbenabled)).into()
        }
        unsafe extern "system" fn showLibrarySharing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPLibrarySharingServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPLibrarySharingServices_Impl::showLibrarySharing(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            isLibraryShared: isLibraryShared::<Identity, OFFSET>,
            isLibrarySharingEnabled: isLibrarySharingEnabled::<Identity, OFFSET>,
            showLibrarySharing: showLibrarySharing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPLibrarySharingServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMedia_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_isIdentical(&self, piwmpmedia: Option<&IWMPMedia>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn sourceURL(&self, pbstrsourceurl: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn name(&self, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Setname(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn imageSourceWidth(&self, pwidth: *mut i32) -> windows_core::Result<()>;
    fn imageSourceHeight(&self, pheight: *mut i32) -> windows_core::Result<()>;
    fn markerCount(&self, pmarkercount: *mut i32) -> windows_core::Result<()>;
    fn getMarkerTime(&self, markernum: i32, pmarkertime: *mut f64) -> windows_core::Result<()>;
    fn getMarkerName(&self, markernum: i32, pbstrmarkername: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn duration(&self, pduration: *mut f64) -> windows_core::Result<()>;
    fn durationString(&self, pbstrduration: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn attributeCount(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn getAttributeName(&self, lindex: i32, pbstritemname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn getItemInfo(&self, bstritemname: &windows_core::BSTR, pbstrval: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn setItemInfo(&self, bstritemname: &windows_core::BSTR, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getItemInfoByAtom(&self, latom: i32, pbstrval: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn isMemberOf(&self, pplaylist: Option<&IWMPPlaylist>, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn isReadOnlyItem(&self, bstritemname: &windows_core::BSTR, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPMedia {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPMedia_Vtbl
    where
        Identity: IWMPMedia_Impl,
    {
        unsafe extern "system" fn get_isIdentical<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpmedia: *mut core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::get_isIdentical(this, windows_core::from_raw_borrowed(&piwmpmedia), core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn sourceURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsourceurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::sourceURL(this, core::mem::transmute_copy(&pbstrsourceurl)).into()
        }
        unsafe extern "system" fn name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::name(this, core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn Setname<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::Setname(this, core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn imageSourceWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::imageSourceWidth(this, core::mem::transmute_copy(&pwidth)).into()
        }
        unsafe extern "system" fn imageSourceHeight<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pheight: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::imageSourceHeight(this, core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn markerCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmarkercount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::markerCount(this, core::mem::transmute_copy(&pmarkercount)).into()
        }
        unsafe extern "system" fn getMarkerTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, markernum: i32, pmarkertime: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::getMarkerTime(this, core::mem::transmute_copy(&markernum), core::mem::transmute_copy(&pmarkertime)).into()
        }
        unsafe extern "system" fn getMarkerName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, markernum: i32, pbstrmarkername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::getMarkerName(this, core::mem::transmute_copy(&markernum), core::mem::transmute_copy(&pbstrmarkername)).into()
        }
        unsafe extern "system" fn duration<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pduration: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::duration(this, core::mem::transmute_copy(&pduration)).into()
        }
        unsafe extern "system" fn durationString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrduration: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::durationString(this, core::mem::transmute_copy(&pbstrduration)).into()
        }
        unsafe extern "system" fn attributeCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::attributeCount(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getAttributeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstritemname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::getAttributeName(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pbstritemname)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::getItemInfo(this, core::mem::transmute(&bstritemname), core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn setItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::setItemInfo(this, core::mem::transmute(&bstritemname), core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn getItemInfoByAtom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, latom: i32, pbstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::getItemInfoByAtom(this, core::mem::transmute_copy(&latom), core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn isMemberOf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplaylist: *mut core::ffi::c_void, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::isMemberOf(this, windows_core::from_raw_borrowed(&pplaylist), core::mem::transmute_copy(&pvarfismemberof)).into()
        }
        unsafe extern "system" fn isReadOnlyItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPMedia_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia_Impl::isReadOnlyItem(this, core::mem::transmute(&bstritemname), core::mem::transmute_copy(&pvarfisreadonly)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_isIdentical: get_isIdentical::<Identity, OFFSET>,
            sourceURL: sourceURL::<Identity, OFFSET>,
            name: name::<Identity, OFFSET>,
            Setname: Setname::<Identity, OFFSET>,
            imageSourceWidth: imageSourceWidth::<Identity, OFFSET>,
            imageSourceHeight: imageSourceHeight::<Identity, OFFSET>,
            markerCount: markerCount::<Identity, OFFSET>,
            getMarkerTime: getMarkerTime::<Identity, OFFSET>,
            getMarkerName: getMarkerName::<Identity, OFFSET>,
            duration: duration::<Identity, OFFSET>,
            durationString: durationString::<Identity, OFFSET>,
            attributeCount: attributeCount::<Identity, OFFSET>,
            getAttributeName: getAttributeName::<Identity, OFFSET>,
            getItemInfo: getItemInfo::<Identity, OFFSET>,
            setItemInfo: setItemInfo::<Identity, OFFSET>,
            getItemInfoByAtom: getItemInfoByAtom::<Identity, OFFSET>,
            isMemberOf: isMemberOf::<Identity, OFFSET>,
            isReadOnlyItem: isReadOnlyItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMedia as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMedia2_Impl: Sized + IWMPMedia_Impl {
    fn error(&self) -> windows_core::Result<IWMPErrorItem>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPMedia2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPMedia2_Vtbl
    where
        Identity: IWMPMedia2_Impl,
    {
        unsafe extern "system" fn error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwmperroritem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMedia2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMedia2_Impl::error(this) {
                Ok(ok__) => {
                    ppiwmperroritem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IWMPMedia_Vtbl::new::<Identity, OFFSET>(), error: error::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMedia2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPMedia as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMedia3_Impl: Sized + IWMPMedia2_Impl {
    fn getAttributeCountByType(&self, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR, plcount: *mut i32) -> windows_core::Result<()>;
    fn getItemInfoByType(&self, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR, lindex: i32, pvarvalue: *mut windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPMedia3 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMedia3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPMedia3_Vtbl
    where
        Identity: IWMPMedia3_Impl,
    {
        unsafe extern "system" fn getAttributeCountByType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtype: core::mem::MaybeUninit<windows_core::BSTR>, bstrlanguage: core::mem::MaybeUninit<windows_core::BSTR>, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPMedia3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia3_Impl::getAttributeCountByType(this, core::mem::transmute(&bstrtype), core::mem::transmute(&bstrlanguage), core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getItemInfoByType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtype: core::mem::MaybeUninit<windows_core::BSTR>, bstrlanguage: core::mem::MaybeUninit<windows_core::BSTR>, lindex: i32, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPMedia3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMedia3_Impl::getItemInfoByType(this, core::mem::transmute(&bstrtype), core::mem::transmute(&bstrlanguage), core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base__: IWMPMedia2_Vtbl::new::<Identity, OFFSET>(),
            getAttributeCountByType: getAttributeCountByType::<Identity, OFFSET>,
            getItemInfoByType: getItemInfoByType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMedia3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPMedia as windows_core::Interface>::IID || iid == &<IWMPMedia2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMediaCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn add(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<IWMPMedia>;
    fn getAll(&self) -> windows_core::Result<IWMPPlaylist>;
    fn getByName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn getByGenre(&self, bstrgenre: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn getByAuthor(&self, bstrauthor: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn getByAlbum(&self, bstralbum: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn getByAttribute(&self, bstrattribute: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn remove(&self, pitem: Option<&IWMPMedia>, varfdeletefile: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getAttributeStringCollection(&self, bstrattribute: &windows_core::BSTR, bstrmediatype: &windows_core::BSTR) -> windows_core::Result<IWMPStringCollection>;
    fn getMediaAtom(&self, bstritemname: &windows_core::BSTR, platom: *mut i32) -> windows_core::Result<()>;
    fn setDeleted(&self, pitem: Option<&IWMPMedia>, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn isDeleted(&self, pitem: Option<&IWMPMedia>, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPMediaCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMediaCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPMediaCollection_Vtbl
    where
        Identity: IWMPMediaCollection_Impl,
    {
        unsafe extern "system" fn add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection_Impl::add(this, core::mem::transmute(&bstrurl)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection_Impl::getAll(this) {
                Ok(ok__) => {
                    ppmediaitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection_Impl::getByName(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    ppmediaitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByGenre<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgenre: core::mem::MaybeUninit<windows_core::BSTR>, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection_Impl::getByGenre(this, core::mem::transmute(&bstrgenre)) {
                Ok(ok__) => {
                    ppmediaitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAuthor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrauthor: core::mem::MaybeUninit<windows_core::BSTR>, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection_Impl::getByAuthor(this, core::mem::transmute(&bstrauthor)) {
                Ok(ok__) => {
                    ppmediaitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAlbum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstralbum: core::mem::MaybeUninit<windows_core::BSTR>, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection_Impl::getByAlbum(this, core::mem::transmute(&bstralbum)) {
                Ok(ok__) => {
                    ppmediaitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAttribute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribute: core::mem::MaybeUninit<windows_core::BSTR>, bstrvalue: core::mem::MaybeUninit<windows_core::BSTR>, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection_Impl::getByAttribute(this, core::mem::transmute(&bstrattribute), core::mem::transmute(&bstrvalue)) {
                Ok(ok__) => {
                    ppmediaitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, varfdeletefile: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMediaCollection_Impl::remove(this, windows_core::from_raw_borrowed(&pitem), core::mem::transmute_copy(&varfdeletefile)).into()
        }
        unsafe extern "system" fn getAttributeStringCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribute: core::mem::MaybeUninit<windows_core::BSTR>, bstrmediatype: core::mem::MaybeUninit<windows_core::BSTR>, ppstringcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection_Impl::getAttributeStringCollection(this, core::mem::transmute(&bstrattribute), core::mem::transmute(&bstrmediatype)) {
                Ok(ok__) => {
                    ppstringcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getMediaAtom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, platom: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMediaCollection_Impl::getMediaAtom(this, core::mem::transmute(&bstritemname), core::mem::transmute_copy(&platom)).into()
        }
        unsafe extern "system" fn setDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMediaCollection_Impl::setDeleted(this, windows_core::from_raw_borrowed(&pitem), core::mem::transmute_copy(&varfisdeleted)).into()
        }
        unsafe extern "system" fn isDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMediaCollection_Impl::isDeleted(this, windows_core::from_raw_borrowed(&pitem), core::mem::transmute_copy(&pvarfisdeleted)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            add: add::<Identity, OFFSET>,
            getAll: getAll::<Identity, OFFSET>,
            getByName: getByName::<Identity, OFFSET>,
            getByGenre: getByGenre::<Identity, OFFSET>,
            getByAuthor: getByAuthor::<Identity, OFFSET>,
            getByAlbum: getByAlbum::<Identity, OFFSET>,
            getByAttribute: getByAttribute::<Identity, OFFSET>,
            remove: remove::<Identity, OFFSET>,
            getAttributeStringCollection: getAttributeStringCollection::<Identity, OFFSET>,
            getMediaAtom: getMediaAtom::<Identity, OFFSET>,
            setDeleted: setDeleted::<Identity, OFFSET>,
            isDeleted: isDeleted::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMediaCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMediaCollection2_Impl: Sized + IWMPMediaCollection_Impl {
    fn createQuery(&self) -> windows_core::Result<IWMPQuery>;
    fn getPlaylistByQuery(&self, pquery: Option<&IWMPQuery>, bstrmediatype: &windows_core::BSTR, bstrsortattribute: &windows_core::BSTR, fsortascending: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IWMPPlaylist>;
    fn getStringCollectionByQuery(&self, bstrattribute: &windows_core::BSTR, pquery: Option<&IWMPQuery>, bstrmediatype: &windows_core::BSTR, bstrsortattribute: &windows_core::BSTR, fsortascending: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IWMPStringCollection>;
    fn getByAttributeAndMediaType(&self, bstrattribute: &windows_core::BSTR, bstrvalue: &windows_core::BSTR, bstrmediatype: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPMediaCollection2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMediaCollection2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPMediaCollection2_Vtbl
    where
        Identity: IWMPMediaCollection2_Impl,
    {
        unsafe extern "system" fn createQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppquery: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection2_Impl::createQuery(this) {
                Ok(ok__) => {
                    ppquery.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPlaylistByQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquery: *mut core::ffi::c_void, bstrmediatype: core::mem::MaybeUninit<windows_core::BSTR>, bstrsortattribute: core::mem::MaybeUninit<windows_core::BSTR>, fsortascending: super::super::Foundation::VARIANT_BOOL, ppplaylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection2_Impl::getPlaylistByQuery(this, windows_core::from_raw_borrowed(&pquery), core::mem::transmute(&bstrmediatype), core::mem::transmute(&bstrsortattribute), core::mem::transmute_copy(&fsortascending)) {
                Ok(ok__) => {
                    ppplaylist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getStringCollectionByQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribute: core::mem::MaybeUninit<windows_core::BSTR>, pquery: *mut core::ffi::c_void, bstrmediatype: core::mem::MaybeUninit<windows_core::BSTR>, bstrsortattribute: core::mem::MaybeUninit<windows_core::BSTR>, fsortascending: super::super::Foundation::VARIANT_BOOL, ppstringcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection2_Impl::getStringCollectionByQuery(this, core::mem::transmute(&bstrattribute), windows_core::from_raw_borrowed(&pquery), core::mem::transmute(&bstrmediatype), core::mem::transmute(&bstrsortattribute), core::mem::transmute_copy(&fsortascending)) {
                Ok(ok__) => {
                    ppstringcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByAttributeAndMediaType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribute: core::mem::MaybeUninit<windows_core::BSTR>, bstrvalue: core::mem::MaybeUninit<windows_core::BSTR>, bstrmediatype: core::mem::MaybeUninit<windows_core::BSTR>, ppmediaitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPMediaCollection2_Impl::getByAttributeAndMediaType(this, core::mem::transmute(&bstrattribute), core::mem::transmute(&bstrvalue), core::mem::transmute(&bstrmediatype)) {
                Ok(ok__) => {
                    ppmediaitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMPMediaCollection_Vtbl::new::<Identity, OFFSET>(),
            createQuery: createQuery::<Identity, OFFSET>,
            getPlaylistByQuery: getPlaylistByQuery::<Identity, OFFSET>,
            getStringCollectionByQuery: getStringCollectionByQuery::<Identity, OFFSET>,
            getByAttributeAndMediaType: getByAttributeAndMediaType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMediaCollection2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPMediaCollection as windows_core::Interface>::IID
    }
}
pub trait IWMPMediaPluginRegistrar_Impl: Sized {
    fn WMPRegisterPlayerPlugin(&self, pwszfriendlyname: &windows_core::PCWSTR, pwszdescription: &windows_core::PCWSTR, pwszuninstallstring: &windows_core::PCWSTR, dwpriority: u32, guidplugintype: &windows_core::GUID, clsid: &windows_core::GUID, cmediatypes: u32, pmediatypes: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn WMPUnRegisterPlayerPlugin(&self, guidplugintype: &windows_core::GUID, clsid: &windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPMediaPluginRegistrar {}
impl IWMPMediaPluginRegistrar_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPMediaPluginRegistrar_Vtbl
    where
        Identity: IWMPMediaPluginRegistrar_Impl,
    {
        unsafe extern "system" fn WMPRegisterPlayerPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfriendlyname: windows_core::PCWSTR, pwszdescription: windows_core::PCWSTR, pwszuninstallstring: windows_core::PCWSTR, dwpriority: u32, guidplugintype: windows_core::GUID, clsid: windows_core::GUID, cmediatypes: u32, pmediatypes: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPMediaPluginRegistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMediaPluginRegistrar_Impl::WMPRegisterPlayerPlugin(this, core::mem::transmute(&pwszfriendlyname), core::mem::transmute(&pwszdescription), core::mem::transmute(&pwszuninstallstring), core::mem::transmute_copy(&dwpriority), core::mem::transmute(&guidplugintype), core::mem::transmute(&clsid), core::mem::transmute_copy(&cmediatypes), core::mem::transmute_copy(&pmediatypes)).into()
        }
        unsafe extern "system" fn WMPUnRegisterPlayerPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidplugintype: windows_core::GUID, clsid: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWMPMediaPluginRegistrar_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMediaPluginRegistrar_Impl::WMPUnRegisterPlayerPlugin(this, core::mem::transmute(&guidplugintype), core::mem::transmute(&clsid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WMPRegisterPlayerPlugin: WMPRegisterPlayerPlugin::<Identity, OFFSET>,
            WMPUnRegisterPlayerPlugin: WMPUnRegisterPlayerPlugin::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMediaPluginRegistrar as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMetadataPicture_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn mimeType(&self, pbstrmimetype: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn pictureType(&self, pbstrpicturetype: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn description(&self, pbstrdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn URL(&self, pbstrurl: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPMetadataPicture {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMetadataPicture_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPMetadataPicture_Vtbl
    where
        Identity: IWMPMetadataPicture_Impl,
    {
        unsafe extern "system" fn mimeType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmimetype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMetadataPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMetadataPicture_Impl::mimeType(this, core::mem::transmute_copy(&pbstrmimetype)).into()
        }
        unsafe extern "system" fn pictureType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpicturetype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMetadataPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMetadataPicture_Impl::pictureType(this, core::mem::transmute_copy(&pbstrpicturetype)).into()
        }
        unsafe extern "system" fn description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMetadataPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMetadataPicture_Impl::description(this, core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn URL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMetadataPicture_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMetadataPicture_Impl::URL(this, core::mem::transmute_copy(&pbstrurl)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            mimeType: mimeType::<Identity, OFFSET>,
            pictureType: pictureType::<Identity, OFFSET>,
            description: description::<Identity, OFFSET>,
            URL: URL::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMetadataPicture as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPMetadataText_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn description(&self, pbstrdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn text(&self, pbstrtext: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPMetadataText {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPMetadataText_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPMetadataText_Vtbl
    where
        Identity: IWMPMetadataText_Impl,
    {
        unsafe extern "system" fn description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMetadataText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMetadataText_Impl::description(this, core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn text<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPMetadataText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPMetadataText_Impl::text(this, core::mem::transmute_copy(&pbstrtext)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            description: description::<Identity, OFFSET>,
            text: text::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPMetadataText as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPNetwork_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn bandWidth(&self, plbandwidth: *mut i32) -> windows_core::Result<()>;
    fn recoveredPackets(&self, plrecoveredpackets: *mut i32) -> windows_core::Result<()>;
    fn sourceProtocol(&self, pbstrsourceprotocol: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn receivedPackets(&self, plreceivedpackets: *mut i32) -> windows_core::Result<()>;
    fn lostPackets(&self, pllostpackets: *mut i32) -> windows_core::Result<()>;
    fn receptionQuality(&self, plreceptionquality: *mut i32) -> windows_core::Result<()>;
    fn bufferingCount(&self, plbufferingcount: *mut i32) -> windows_core::Result<()>;
    fn bufferingProgress(&self, plbufferingprogress: *mut i32) -> windows_core::Result<()>;
    fn bufferingTime(&self, plbufferingtime: *mut i32) -> windows_core::Result<()>;
    fn SetbufferingTime(&self, lbufferingtime: i32) -> windows_core::Result<()>;
    fn frameRate(&self, plframerate: *mut i32) -> windows_core::Result<()>;
    fn maxBitRate(&self, plbitrate: *mut i32) -> windows_core::Result<()>;
    fn bitRate(&self, plbitrate: *mut i32) -> windows_core::Result<()>;
    fn getProxySettings(&self, bstrprotocol: &windows_core::BSTR, plproxysetting: *mut i32) -> windows_core::Result<()>;
    fn setProxySettings(&self, bstrprotocol: &windows_core::BSTR, lproxysetting: i32) -> windows_core::Result<()>;
    fn getProxyName(&self, bstrprotocol: &windows_core::BSTR, pbstrproxyname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn setProxyName(&self, bstrprotocol: &windows_core::BSTR, bstrproxyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getProxyPort(&self, bstrprotocol: &windows_core::BSTR, lproxyport: *mut i32) -> windows_core::Result<()>;
    fn setProxyPort(&self, bstrprotocol: &windows_core::BSTR, lproxyport: i32) -> windows_core::Result<()>;
    fn getProxyExceptionList(&self, bstrprotocol: &windows_core::BSTR, pbstrexceptionlist: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn setProxyExceptionList(&self, bstrprotocol: &windows_core::BSTR, pbstrexceptionlist: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getProxyBypassForLocal(&self, bstrprotocol: &windows_core::BSTR, pfbypassforlocal: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn setProxyBypassForLocal(&self, bstrprotocol: &windows_core::BSTR, fbypassforlocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn maxBandwidth(&self, lmaxbandwidth: *mut i32) -> windows_core::Result<()>;
    fn SetmaxBandwidth(&self, lmaxbandwidth: i32) -> windows_core::Result<()>;
    fn downloadProgress(&self, pldownloadprogress: *mut i32) -> windows_core::Result<()>;
    fn encodedFrameRate(&self, plframerate: *mut i32) -> windows_core::Result<()>;
    fn framesSkipped(&self, plframes: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPNetwork {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPNetwork_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPNetwork_Vtbl
    where
        Identity: IWMPNetwork_Impl,
    {
        unsafe extern "system" fn bandWidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbandwidth: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::bandWidth(this, core::mem::transmute_copy(&plbandwidth)).into()
        }
        unsafe extern "system" fn recoveredPackets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plrecoveredpackets: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::recoveredPackets(this, core::mem::transmute_copy(&plrecoveredpackets)).into()
        }
        unsafe extern "system" fn sourceProtocol<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsourceprotocol: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::sourceProtocol(this, core::mem::transmute_copy(&pbstrsourceprotocol)).into()
        }
        unsafe extern "system" fn receivedPackets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plreceivedpackets: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::receivedPackets(this, core::mem::transmute_copy(&plreceivedpackets)).into()
        }
        unsafe extern "system" fn lostPackets<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllostpackets: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::lostPackets(this, core::mem::transmute_copy(&pllostpackets)).into()
        }
        unsafe extern "system" fn receptionQuality<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plreceptionquality: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::receptionQuality(this, core::mem::transmute_copy(&plreceptionquality)).into()
        }
        unsafe extern "system" fn bufferingCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbufferingcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::bufferingCount(this, core::mem::transmute_copy(&plbufferingcount)).into()
        }
        unsafe extern "system" fn bufferingProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbufferingprogress: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::bufferingProgress(this, core::mem::transmute_copy(&plbufferingprogress)).into()
        }
        unsafe extern "system" fn bufferingTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbufferingtime: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::bufferingTime(this, core::mem::transmute_copy(&plbufferingtime)).into()
        }
        unsafe extern "system" fn SetbufferingTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbufferingtime: i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::SetbufferingTime(this, core::mem::transmute_copy(&lbufferingtime)).into()
        }
        unsafe extern "system" fn frameRate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plframerate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::frameRate(this, core::mem::transmute_copy(&plframerate)).into()
        }
        unsafe extern "system" fn maxBitRate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbitrate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::maxBitRate(this, core::mem::transmute_copy(&plbitrate)).into()
        }
        unsafe extern "system" fn bitRate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbitrate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::bitRate(this, core::mem::transmute_copy(&plbitrate)).into()
        }
        unsafe extern "system" fn getProxySettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, plproxysetting: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::getProxySettings(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&plproxysetting)).into()
        }
        unsafe extern "system" fn setProxySettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, lproxysetting: i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::setProxySettings(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&lproxysetting)).into()
        }
        unsafe extern "system" fn getProxyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, pbstrproxyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::getProxyName(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&pbstrproxyname)).into()
        }
        unsafe extern "system" fn setProxyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, bstrproxyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::setProxyName(this, core::mem::transmute(&bstrprotocol), core::mem::transmute(&bstrproxyname)).into()
        }
        unsafe extern "system" fn getProxyPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, lproxyport: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::getProxyPort(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&lproxyport)).into()
        }
        unsafe extern "system" fn setProxyPort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, lproxyport: i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::setProxyPort(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&lproxyport)).into()
        }
        unsafe extern "system" fn getProxyExceptionList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, pbstrexceptionlist: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::getProxyExceptionList(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&pbstrexceptionlist)).into()
        }
        unsafe extern "system" fn setProxyExceptionList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, pbstrexceptionlist: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::setProxyExceptionList(this, core::mem::transmute(&bstrprotocol), core::mem::transmute(&pbstrexceptionlist)).into()
        }
        unsafe extern "system" fn getProxyBypassForLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, pfbypassforlocal: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::getProxyBypassForLocal(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&pfbypassforlocal)).into()
        }
        unsafe extern "system" fn setProxyBypassForLocal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprotocol: core::mem::MaybeUninit<windows_core::BSTR>, fbypassforlocal: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::setProxyBypassForLocal(this, core::mem::transmute(&bstrprotocol), core::mem::transmute_copy(&fbypassforlocal)).into()
        }
        unsafe extern "system" fn maxBandwidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxbandwidth: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::maxBandwidth(this, core::mem::transmute_copy(&lmaxbandwidth)).into()
        }
        unsafe extern "system" fn SetmaxBandwidth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lmaxbandwidth: i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::SetmaxBandwidth(this, core::mem::transmute_copy(&lmaxbandwidth)).into()
        }
        unsafe extern "system" fn downloadProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldownloadprogress: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::downloadProgress(this, core::mem::transmute_copy(&pldownloadprogress)).into()
        }
        unsafe extern "system" fn encodedFrameRate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plframerate: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::encodedFrameRate(this, core::mem::transmute_copy(&plframerate)).into()
        }
        unsafe extern "system" fn framesSkipped<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plframes: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNetwork_Impl::framesSkipped(this, core::mem::transmute_copy(&plframes)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            bandWidth: bandWidth::<Identity, OFFSET>,
            recoveredPackets: recoveredPackets::<Identity, OFFSET>,
            sourceProtocol: sourceProtocol::<Identity, OFFSET>,
            receivedPackets: receivedPackets::<Identity, OFFSET>,
            lostPackets: lostPackets::<Identity, OFFSET>,
            receptionQuality: receptionQuality::<Identity, OFFSET>,
            bufferingCount: bufferingCount::<Identity, OFFSET>,
            bufferingProgress: bufferingProgress::<Identity, OFFSET>,
            bufferingTime: bufferingTime::<Identity, OFFSET>,
            SetbufferingTime: SetbufferingTime::<Identity, OFFSET>,
            frameRate: frameRate::<Identity, OFFSET>,
            maxBitRate: maxBitRate::<Identity, OFFSET>,
            bitRate: bitRate::<Identity, OFFSET>,
            getProxySettings: getProxySettings::<Identity, OFFSET>,
            setProxySettings: setProxySettings::<Identity, OFFSET>,
            getProxyName: getProxyName::<Identity, OFFSET>,
            setProxyName: setProxyName::<Identity, OFFSET>,
            getProxyPort: getProxyPort::<Identity, OFFSET>,
            setProxyPort: setProxyPort::<Identity, OFFSET>,
            getProxyExceptionList: getProxyExceptionList::<Identity, OFFSET>,
            setProxyExceptionList: setProxyExceptionList::<Identity, OFFSET>,
            getProxyBypassForLocal: getProxyBypassForLocal::<Identity, OFFSET>,
            setProxyBypassForLocal: setProxyBypassForLocal::<Identity, OFFSET>,
            maxBandwidth: maxBandwidth::<Identity, OFFSET>,
            SetmaxBandwidth: SetmaxBandwidth::<Identity, OFFSET>,
            downloadProgress: downloadProgress::<Identity, OFFSET>,
            encodedFrameRate: encodedFrameRate::<Identity, OFFSET>,
            framesSkipped: framesSkipped::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPNetwork as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IWMPNodeRealEstate_Impl: Sized {
    fn GetDesiredSize(&self, psize: *mut super::super::Foundation::SIZE) -> windows_core::Result<()>;
    fn SetRects(&self, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetRects(&self, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn SetWindowless(&self, fwindowless: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetWindowless(&self, pfwindowless: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetFullScreen(&self, ffullscreen: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetFullScreen(&self, pffullscreen: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPNodeRealEstate {}
impl IWMPNodeRealEstate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPNodeRealEstate_Vtbl
    where
        Identity: IWMPNodeRealEstate_Impl,
    {
        unsafe extern "system" fn GetDesiredSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: IWMPNodeRealEstate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeRealEstate_Impl::GetDesiredSize(this, core::mem::transmute_copy(&psize)).into()
        }
        unsafe extern "system" fn SetRects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IWMPNodeRealEstate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeRealEstate_Impl::SetRects(this, core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&pdest), core::mem::transmute_copy(&pclip)).into()
        }
        unsafe extern "system" fn GetRects<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IWMPNodeRealEstate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeRealEstate_Impl::GetRects(this, core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&pdest), core::mem::transmute_copy(&pclip)).into()
        }
        unsafe extern "system" fn SetWindowless<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fwindowless: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPNodeRealEstate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeRealEstate_Impl::SetWindowless(this, core::mem::transmute_copy(&fwindowless)).into()
        }
        unsafe extern "system" fn GetWindowless<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfwindowless: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPNodeRealEstate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeRealEstate_Impl::GetWindowless(this, core::mem::transmute_copy(&pfwindowless)).into()
        }
        unsafe extern "system" fn SetFullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPNodeRealEstate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeRealEstate_Impl::SetFullScreen(this, core::mem::transmute_copy(&ffullscreen)).into()
        }
        unsafe extern "system" fn GetFullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPNodeRealEstate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeRealEstate_Impl::GetFullScreen(this, core::mem::transmute_copy(&pffullscreen)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesiredSize: GetDesiredSize::<Identity, OFFSET>,
            SetRects: SetRects::<Identity, OFFSET>,
            GetRects: GetRects::<Identity, OFFSET>,
            SetWindowless: SetWindowless::<Identity, OFFSET>,
            GetWindowless: GetWindowless::<Identity, OFFSET>,
            SetFullScreen: SetFullScreen::<Identity, OFFSET>,
            GetFullScreen: GetFullScreen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPNodeRealEstate as windows_core::Interface>::IID
    }
}
pub trait IWMPNodeRealEstateHost_Impl: Sized {
    fn OnDesiredSizeChange(&self, psize: *mut super::super::Foundation::SIZE) -> windows_core::Result<()>;
    fn OnFullScreenTransition(&self, ffullscreen: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPNodeRealEstateHost {}
impl IWMPNodeRealEstateHost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPNodeRealEstateHost_Vtbl
    where
        Identity: IWMPNodeRealEstateHost_Impl,
    {
        unsafe extern "system" fn OnDesiredSizeChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> windows_core::HRESULT
        where
            Identity: IWMPNodeRealEstateHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeRealEstateHost_Impl::OnDesiredSizeChange(this, core::mem::transmute_copy(&psize)).into()
        }
        unsafe extern "system" fn OnFullScreenTransition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPNodeRealEstateHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeRealEstateHost_Impl::OnFullScreenTransition(this, core::mem::transmute_copy(&ffullscreen)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDesiredSizeChange: OnDesiredSizeChange::<Identity, OFFSET>,
            OnFullScreenTransition: OnFullScreenTransition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPNodeRealEstateHost as windows_core::Interface>::IID
    }
}
pub trait IWMPNodeWindowed_Impl: Sized {
    fn SetOwnerWindow(&self, hwnd: isize) -> windows_core::Result<()>;
    fn GetOwnerWindow(&self, phwnd: *mut isize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPNodeWindowed {}
impl IWMPNodeWindowed_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPNodeWindowed_Vtbl
    where
        Identity: IWMPNodeWindowed_Impl,
    {
        unsafe extern "system" fn SetOwnerWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: isize) -> windows_core::HRESULT
        where
            Identity: IWMPNodeWindowed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeWindowed_Impl::SetOwnerWindow(this, core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn GetOwnerWindow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut isize) -> windows_core::HRESULT
        where
            Identity: IWMPNodeWindowed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeWindowed_Impl::GetOwnerWindow(this, core::mem::transmute_copy(&phwnd)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOwnerWindow: SetOwnerWindow::<Identity, OFFSET>,
            GetOwnerWindow: GetOwnerWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPNodeWindowed as windows_core::Interface>::IID
    }
}
pub trait IWMPNodeWindowedHost_Impl: Sized {
    fn OnWindowMessageFromRenderer(&self, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPNodeWindowedHost {}
impl IWMPNodeWindowedHost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPNodeWindowedHost_Vtbl
    where
        Identity: IWMPNodeWindowedHost_Impl,
    {
        unsafe extern "system" fn OnWindowMessageFromRenderer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPNodeWindowedHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeWindowedHost_Impl::OnWindowMessageFromRenderer(this, core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam), core::mem::transmute_copy(&plret), core::mem::transmute_copy(&pfhandled)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnWindowMessageFromRenderer: OnWindowMessageFromRenderer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPNodeWindowedHost as windows_core::Interface>::IID
    }
}
pub trait IWMPNodeWindowless_Impl: Sized + IWMPWindowMessageSink_Impl {
    fn OnDraw(&self, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPNodeWindowless {}
impl IWMPNodeWindowless_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPNodeWindowless_Vtbl
    where
        Identity: IWMPNodeWindowless_Impl,
    {
        unsafe extern "system" fn OnDraw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IWMPNodeWindowless_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeWindowless_Impl::OnDraw(this, core::mem::transmute_copy(&hdc), core::mem::transmute_copy(&prcdraw)).into()
        }
        Self { base__: IWMPWindowMessageSink_Vtbl::new::<Identity, OFFSET>(), OnDraw: OnDraw::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPNodeWindowless as windows_core::Interface>::IID || iid == &<IWMPWindowMessageSink as windows_core::Interface>::IID
    }
}
pub trait IWMPNodeWindowlessHost_Impl: Sized {
    fn InvalidateRect(&self, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPNodeWindowlessHost {}
impl IWMPNodeWindowlessHost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPNodeWindowlessHost_Vtbl
    where
        Identity: IWMPNodeWindowlessHost_Impl,
    {
        unsafe extern "system" fn InvalidateRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPNodeWindowlessHost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPNodeWindowlessHost_Impl::InvalidateRect(this, core::mem::transmute_copy(&prc), core::mem::transmute_copy(&ferase)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InvalidateRect: InvalidateRect::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPNodeWindowlessHost as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayer_Impl: Sized + IWMPCore_Impl {
    fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Setenabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetfullScreen(&self, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetenableContextMenu(&self, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetuiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn uiMode(&self, pbstrmode: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPPlayer {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlayer_Vtbl
    where
        Identity: IWMPPlayer_Impl,
    {
        unsafe extern "system" fn enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer_Impl::enabled(this, core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer_Impl::Setenabled(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer_Impl::fullScreen(this, core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer_Impl::SetfullScreen(this, core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer_Impl::enableContextMenu(this, core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer_Impl::SetenableContextMenu(this, core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer_Impl::SetuiMode(this, core::mem::transmute(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer_Impl::uiMode(this, core::mem::transmute_copy(&pbstrmode)).into()
        }
        Self {
            base__: IWMPCore_Vtbl::new::<Identity, OFFSET>(),
            enabled: enabled::<Identity, OFFSET>,
            Setenabled: Setenabled::<Identity, OFFSET>,
            fullScreen: fullScreen::<Identity, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, OFFSET>,
            SetuiMode: SetuiMode::<Identity, OFFSET>,
            uiMode: uiMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayer2_Impl: Sized + IWMPCore_Impl {
    fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Setenabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetfullScreen(&self, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetenableContextMenu(&self, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetuiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn uiMode(&self, pbstrmode: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn stretchToFit(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetstretchToFit(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn windowlessVideo(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetwindowlessVideo(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPPlayer2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlayer2_Vtbl
    where
        Identity: IWMPPlayer2_Impl,
    {
        unsafe extern "system" fn enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::enabled(this, core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::Setenabled(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::fullScreen(this, core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::SetfullScreen(this, core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::enableContextMenu(this, core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::SetenableContextMenu(this, core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::SetuiMode(this, core::mem::transmute(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::uiMode(this, core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::stretchToFit(this, core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::SetstretchToFit(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::windowlessVideo(this, core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer2_Impl::SetwindowlessVideo(this, core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base__: IWMPCore_Vtbl::new::<Identity, OFFSET>(),
            enabled: enabled::<Identity, OFFSET>,
            Setenabled: Setenabled::<Identity, OFFSET>,
            fullScreen: fullScreen::<Identity, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, OFFSET>,
            SetuiMode: SetuiMode::<Identity, OFFSET>,
            uiMode: uiMode::<Identity, OFFSET>,
            stretchToFit: stretchToFit::<Identity, OFFSET>,
            SetstretchToFit: SetstretchToFit::<Identity, OFFSET>,
            windowlessVideo: windowlessVideo::<Identity, OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayer2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayer3_Impl: Sized + IWMPCore2_Impl {
    fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Setenabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetfullScreen(&self, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetenableContextMenu(&self, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetuiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn uiMode(&self, pbstrmode: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn stretchToFit(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetstretchToFit(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn windowlessVideo(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetwindowlessVideo(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPPlayer3 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlayer3_Vtbl
    where
        Identity: IWMPPlayer3_Impl,
    {
        unsafe extern "system" fn enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::enabled(this, core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::Setenabled(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::fullScreen(this, core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::SetfullScreen(this, core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::enableContextMenu(this, core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::SetenableContextMenu(this, core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::SetuiMode(this, core::mem::transmute(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::uiMode(this, core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::stretchToFit(this, core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::SetstretchToFit(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::windowlessVideo(this, core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer3_Impl::SetwindowlessVideo(this, core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base__: IWMPCore2_Vtbl::new::<Identity, OFFSET>(),
            enabled: enabled::<Identity, OFFSET>,
            Setenabled: Setenabled::<Identity, OFFSET>,
            fullScreen: fullScreen::<Identity, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, OFFSET>,
            SetuiMode: SetuiMode::<Identity, OFFSET>,
            uiMode: uiMode::<Identity, OFFSET>,
            stretchToFit: stretchToFit::<Identity, OFFSET>,
            SetstretchToFit: SetstretchToFit::<Identity, OFFSET>,
            windowlessVideo: windowlessVideo::<Identity, OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayer3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID || iid == &<IWMPCore2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayer4_Impl: Sized + IWMPCore3_Impl {
    fn enabled(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Setenabled(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn fullScreen(&self, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetfullScreen(&self, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn enableContextMenu(&self, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetenableContextMenu(&self, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetuiMode(&self, bstrmode: &windows_core::BSTR) -> windows_core::Result<()>;
    fn uiMode(&self, pbstrmode: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn stretchToFit(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetstretchToFit(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn windowlessVideo(&self, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetwindowlessVideo(&self, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn isRemote(&self, pvarfisremote: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn playerApplication(&self) -> windows_core::Result<IWMPPlayerApplication>;
    fn openPlayer(&self, bstrurl: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPPlayer4 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayer4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlayer4_Vtbl
    where
        Identity: IWMPPlayer4_Impl,
    {
        unsafe extern "system" fn enabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::enabled(this, core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn Setenabled<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::Setenabled(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn fullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::fullScreen(this, core::mem::transmute_copy(&pbfullscreen)).into()
        }
        unsafe extern "system" fn SetfullScreen<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::SetfullScreen(this, core::mem::transmute_copy(&bfullscreen)).into()
        }
        unsafe extern "system" fn enableContextMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::enableContextMenu(this, core::mem::transmute_copy(&pbenablecontextmenu)).into()
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::SetenableContextMenu(this, core::mem::transmute_copy(&benablecontextmenu)).into()
        }
        unsafe extern "system" fn SetuiMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::SetuiMode(this, core::mem::transmute(&bstrmode)).into()
        }
        unsafe extern "system" fn uiMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmode: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::uiMode(this, core::mem::transmute_copy(&pbstrmode)).into()
        }
        unsafe extern "system" fn stretchToFit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::stretchToFit(this, core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetstretchToFit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::SetstretchToFit(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn windowlessVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::windowlessVideo(this, core::mem::transmute_copy(&pbenabled)).into()
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::SetwindowlessVideo(this, core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn isRemote<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarfisremote: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::isRemote(this, core::mem::transmute_copy(&pvarfisremote)).into()
        }
        unsafe extern "system" fn playerApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiwmpplayerapplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPPlayer4_Impl::playerApplication(this) {
                Ok(ok__) => {
                    ppiwmpplayerapplication.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn openPlayer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayer4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayer4_Impl::openPlayer(this, core::mem::transmute(&bstrurl)).into()
        }
        Self {
            base__: IWMPCore3_Vtbl::new::<Identity, OFFSET>(),
            enabled: enabled::<Identity, OFFSET>,
            Setenabled: Setenabled::<Identity, OFFSET>,
            fullScreen: fullScreen::<Identity, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, OFFSET>,
            SetuiMode: SetuiMode::<Identity, OFFSET>,
            uiMode: uiMode::<Identity, OFFSET>,
            stretchToFit: stretchToFit::<Identity, OFFSET>,
            SetstretchToFit: SetstretchToFit::<Identity, OFFSET>,
            windowlessVideo: windowlessVideo::<Identity, OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Identity, OFFSET>,
            isRemote: isRemote::<Identity, OFFSET>,
            playerApplication: playerApplication::<Identity, OFFSET>,
            openPlayer: openPlayer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayer4 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPCore as windows_core::Interface>::IID || iid == &<IWMPCore2 as windows_core::Interface>::IID || iid == &<IWMPCore3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlayerApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn switchToPlayerApplication(&self) -> windows_core::Result<()>;
    fn switchToControl(&self) -> windows_core::Result<()>;
    fn playerDocked(&self, pbplayerdocked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn hasDisplay(&self, pbhasdisplay: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPPlayerApplication {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlayerApplication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlayerApplication_Vtbl
    where
        Identity: IWMPPlayerApplication_Impl,
    {
        unsafe extern "system" fn switchToPlayerApplication<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlayerApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayerApplication_Impl::switchToPlayerApplication(this).into()
        }
        unsafe extern "system" fn switchToControl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlayerApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayerApplication_Impl::switchToControl(this).into()
        }
        unsafe extern "system" fn playerDocked<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbplayerdocked: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayerApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayerApplication_Impl::playerDocked(this, core::mem::transmute_copy(&pbplayerdocked)).into()
        }
        unsafe extern "system" fn hasDisplay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbhasdisplay: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlayerApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayerApplication_Impl::hasDisplay(this, core::mem::transmute_copy(&pbhasdisplay)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            switchToPlayerApplication: switchToPlayerApplication::<Identity, OFFSET>,
            switchToControl: switchToControl::<Identity, OFFSET>,
            playerDocked: playerDocked::<Identity, OFFSET>,
            hasDisplay: hasDisplay::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayerApplication as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IWMPPlayerServices_Impl: Sized {
    fn activateUIPlugin(&self, bstrplugin: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setTaskPane(&self, bstrtaskpane: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setTaskPaneURL(&self, bstrtaskpane: &windows_core::BSTR, bstrurl: &windows_core::BSTR, bstrfriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPPlayerServices {}
impl IWMPPlayerServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlayerServices_Vtbl
    where
        Identity: IWMPPlayerServices_Impl,
    {
        unsafe extern "system" fn activateUIPlugin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrplugin: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayerServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayerServices_Impl::activateUIPlugin(this, core::mem::transmute(&bstrplugin)).into()
        }
        unsafe extern "system" fn setTaskPane<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtaskpane: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayerServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayerServices_Impl::setTaskPane(this, core::mem::transmute(&bstrtaskpane)).into()
        }
        unsafe extern "system" fn setTaskPaneURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtaskpane: core::mem::MaybeUninit<windows_core::BSTR>, bstrurl: core::mem::MaybeUninit<windows_core::BSTR>, bstrfriendlyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayerServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayerServices_Impl::setTaskPaneURL(this, core::mem::transmute(&bstrtaskpane), core::mem::transmute(&bstrurl), core::mem::transmute(&bstrfriendlyname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            activateUIPlugin: activateUIPlugin::<Identity, OFFSET>,
            setTaskPane: setTaskPane::<Identity, OFFSET>,
            setTaskPaneURL: setTaskPaneURL::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayerServices as windows_core::Interface>::IID
    }
}
pub trait IWMPPlayerServices2_Impl: Sized + IWMPPlayerServices_Impl {
    fn setBackgroundProcessingPriority(&self, bstrpriority: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPPlayerServices2 {}
impl IWMPPlayerServices2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlayerServices2_Vtbl
    where
        Identity: IWMPPlayerServices2_Impl,
    {
        unsafe extern "system" fn setBackgroundProcessingPriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpriority: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlayerServices2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlayerServices2_Impl::setBackgroundProcessingPriority(this, core::mem::transmute(&bstrpriority)).into()
        }
        Self { base__: IWMPPlayerServices_Vtbl::new::<Identity, OFFSET>(), setBackgroundProcessingPriority: setBackgroundProcessingPriority::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlayerServices2 as windows_core::Interface>::IID || iid == &<IWMPPlayerServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlaylist_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn name(&self, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Setname(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn attributeCount(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn get_attributeName(&self, lindex: i32, pbstrattributename: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn get_item(&self, lindex: i32) -> windows_core::Result<IWMPMedia>;
    fn getItemInfo(&self, bstrname: &windows_core::BSTR, pbstrval: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn setItemInfo(&self, bstrname: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_isIdentical(&self, piwmpplaylist: Option<&IWMPPlaylist>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn clear(&self) -> windows_core::Result<()>;
    fn insertItem(&self, lindex: i32, piwmpmedia: Option<&IWMPMedia>) -> windows_core::Result<()>;
    fn appendItem(&self, piwmpmedia: Option<&IWMPMedia>) -> windows_core::Result<()>;
    fn removeItem(&self, piwmpmedia: Option<&IWMPMedia>) -> windows_core::Result<()>;
    fn moveItem(&self, lindexold: i32, lindexnew: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPPlaylist {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylist_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlaylist_Vtbl
    where
        Identity: IWMPPlaylist_Impl,
    {
        unsafe extern "system" fn count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::count(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::name(this, core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn Setname<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::Setname(this, core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn attributeCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::attributeCount(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn get_attributeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstrattributename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::get_attributeName(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pbstrattributename)).into()
        }
        unsafe extern "system" fn get_item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppiwmpmedia: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPPlaylist_Impl::get_item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    ppiwmpmedia.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::getItemInfo(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn setItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, bstrvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::setItemInfo(this, core::mem::transmute(&bstrname), core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn get_isIdentical<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpplaylist: *mut core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::get_isIdentical(this, windows_core::from_raw_borrowed(&piwmpplaylist), core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn clear<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::clear(this).into()
        }
        unsafe extern "system" fn insertItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, piwmpmedia: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::insertItem(this, core::mem::transmute_copy(&lindex), windows_core::from_raw_borrowed(&piwmpmedia)).into()
        }
        unsafe extern "system" fn appendItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpmedia: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::appendItem(this, windows_core::from_raw_borrowed(&piwmpmedia)).into()
        }
        unsafe extern "system" fn removeItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpmedia: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::removeItem(this, windows_core::from_raw_borrowed(&piwmpmedia)).into()
        }
        unsafe extern "system" fn moveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindexold: i32, lindexnew: i32) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylist_Impl::moveItem(this, core::mem::transmute_copy(&lindexold), core::mem::transmute_copy(&lindexnew)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            count: count::<Identity, OFFSET>,
            name: name::<Identity, OFFSET>,
            Setname: Setname::<Identity, OFFSET>,
            attributeCount: attributeCount::<Identity, OFFSET>,
            get_attributeName: get_attributeName::<Identity, OFFSET>,
            get_item: get_item::<Identity, OFFSET>,
            getItemInfo: getItemInfo::<Identity, OFFSET>,
            setItemInfo: setItemInfo::<Identity, OFFSET>,
            get_isIdentical: get_isIdentical::<Identity, OFFSET>,
            clear: clear::<Identity, OFFSET>,
            insertItem: insertItem::<Identity, OFFSET>,
            appendItem: appendItem::<Identity, OFFSET>,
            removeItem: removeItem::<Identity, OFFSET>,
            moveItem: moveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlaylist as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlaylistArray_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn item(&self, lindex: i32) -> windows_core::Result<IWMPPlaylist>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPPlaylistArray {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylistArray_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlaylistArray_Vtbl
    where
        Identity: IWMPPlaylistArray_Impl,
    {
        unsafe extern "system" fn count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylistArray_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylistArray_Impl::count(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylistArray_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPPlaylistArray_Impl::item(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), count: count::<Identity, OFFSET>, item: item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlaylistArray as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPPlaylistCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn newPlaylist(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylist>;
    fn getAll(&self) -> windows_core::Result<IWMPPlaylistArray>;
    fn getByName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IWMPPlaylistArray>;
    fn remove(&self, pitem: Option<&IWMPPlaylist>) -> windows_core::Result<()>;
    fn setDeleted(&self, pitem: Option<&IWMPPlaylist>, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn isDeleted(&self, pitem: Option<&IWMPPlaylist>, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn importPlaylist(&self, pitem: Option<&IWMPPlaylist>) -> windows_core::Result<IWMPPlaylist>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPPlaylistCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPPlaylistCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlaylistCollection_Vtbl
    where
        Identity: IWMPPlaylistCollection_Impl,
    {
        unsafe extern "system" fn newPlaylist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylistCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPPlaylistCollection_Impl::newPlaylist(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplaylistarray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylistCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPPlaylistCollection_Impl::getAll(this) {
                Ok(ok__) => {
                    ppplaylistarray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getByName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppplaylistarray: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylistCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPPlaylistCollection_Impl::getByName(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    ppplaylistarray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylistCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylistCollection_Impl::remove(this, windows_core::from_raw_borrowed(&pitem)).into()
        }
        unsafe extern "system" fn setDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylistCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylistCollection_Impl::setDeleted(this, windows_core::from_raw_borrowed(&pitem), core::mem::transmute_copy(&varfisdeleted)).into()
        }
        unsafe extern "system" fn isDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylistCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlaylistCollection_Impl::isDeleted(this, windows_core::from_raw_borrowed(&pitem), core::mem::transmute_copy(&pvarfisdeleted)).into()
        }
        unsafe extern "system" fn importPlaylist<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitem: *mut core::ffi::c_void, ppimporteditem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlaylistCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPPlaylistCollection_Impl::importPlaylist(this, windows_core::from_raw_borrowed(&pitem)) {
                Ok(ok__) => {
                    ppimporteditem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            newPlaylist: newPlaylist::<Identity, OFFSET>,
            getAll: getAll::<Identity, OFFSET>,
            getByName: getByName::<Identity, OFFSET>,
            remove: remove::<Identity, OFFSET>,
            setDeleted: setDeleted::<Identity, OFFSET>,
            isDeleted: isDeleted::<Identity, OFFSET>,
            importPlaylist: importPlaylist::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlaylistCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IWMPPlugin_Impl: Sized {
    fn Init(&self, dwplaybackcontext: usize) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn GetID(&self, pguid: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetCaps(&self, pdwflags: *mut u32) -> windows_core::Result<()>;
    fn AdviseWMPServices(&self, pwmpservices: Option<&IWMPServices>) -> windows_core::Result<()>;
    fn UnAdviseWMPServices(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPPlugin {}
impl IWMPPlugin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPlugin_Vtbl
    where
        Identity: IWMPPlugin_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwplaybackcontext: usize) -> windows_core::HRESULT
        where
            Identity: IWMPPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlugin_Impl::Init(this, core::mem::transmute_copy(&dwplaybackcontext)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlugin_Impl::Shutdown(this).into()
        }
        unsafe extern "system" fn GetID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IWMPPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlugin_Impl::GetID(this, core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWMPPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlugin_Impl::GetCaps(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn AdviseWMPServices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpservices: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlugin_Impl::AdviseWMPServices(this, windows_core::from_raw_borrowed(&pwmpservices)).into()
        }
        unsafe extern "system" fn UnAdviseWMPServices<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPlugin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPlugin_Impl::UnAdviseWMPServices(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            GetID: GetID::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            AdviseWMPServices: AdviseWMPServices::<Identity, OFFSET>,
            UnAdviseWMPServices: UnAdviseWMPServices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPlugin as windows_core::Interface>::IID
    }
}
pub trait IWMPPluginEnable_Impl: Sized {
    fn SetEnable(&self, fenable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetEnable(&self, pfenable: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPPluginEnable {}
impl IWMPPluginEnable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPluginEnable_Vtbl
    where
        Identity: IWMPPluginEnable_Impl,
    {
        unsafe extern "system" fn SetEnable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPluginEnable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPluginEnable_Impl::SetEnable(this, core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetEnable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPPluginEnable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPluginEnable_Impl::GetEnable(this, core::mem::transmute_copy(&pfenable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetEnable: SetEnable::<Identity, OFFSET>,
            GetEnable: GetEnable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPluginEnable as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWMPPluginUI_Impl: Sized {
    fn SetCore(&self, pcore: Option<&IWMPCore>) -> windows_core::Result<()>;
    fn Create(&self, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn Destroy(&self) -> windows_core::Result<()>;
    fn DisplayPropertyPage(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn GetProperty(&self, pwszname: &windows_core::PCWSTR, pvarproperty: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn SetProperty(&self, pwszname: &windows_core::PCWSTR, pvarproperty: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn TranslateAccelerator(&self, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IWMPPluginUI {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl IWMPPluginUI_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPPluginUI_Vtbl
    where
        Identity: IWMPPluginUI_Impl,
    {
        unsafe extern "system" fn SetCore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcore: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPluginUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPluginUI_Impl::SetCore(this, windows_core::from_raw_borrowed(&pcore)).into()
        }
        unsafe extern "system" fn Create<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IWMPPluginUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPluginUI_Impl::Create(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&phwndwindow)).into()
        }
        unsafe extern "system" fn Destroy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPPluginUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPluginUI_Impl::Destroy(this).into()
        }
        unsafe extern "system" fn DisplayPropertyPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IWMPPluginUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPluginUI_Impl::DisplayPropertyPage(this, core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pvarproperty: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPPluginUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPluginUI_Impl::GetProperty(this, core::mem::transmute(&pwszname), core::mem::transmute_copy(&pvarproperty)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, pvarproperty: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPPluginUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPluginUI_Impl::SetProperty(this, core::mem::transmute(&pwszname), core::mem::transmute_copy(&pvarproperty)).into()
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> windows_core::HRESULT
        where
            Identity: IWMPPluginUI_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPPluginUI_Impl::TranslateAccelerator(this, core::mem::transmute_copy(&lpmsg)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCore: SetCore::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Destroy: Destroy::<Identity, OFFSET>,
            DisplayPropertyPage: DisplayPropertyPage::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPPluginUI as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPQuery_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn addCondition(&self, bstrattribute: &windows_core::BSTR, bstroperator: &windows_core::BSTR, bstrvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn beginNextGroup(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPQuery {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPQuery_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPQuery_Vtbl
    where
        Identity: IWMPQuery_Impl,
    {
        unsafe extern "system" fn addCondition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrattribute: core::mem::MaybeUninit<windows_core::BSTR>, bstroperator: core::mem::MaybeUninit<windows_core::BSTR>, bstrvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPQuery_Impl::addCondition(this, core::mem::transmute(&bstrattribute), core::mem::transmute(&bstroperator), core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn beginNextGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPQuery_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPQuery_Impl::beginNextGroup(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            addCondition: addCondition::<Identity, OFFSET>,
            beginNextGroup: beginNextGroup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPQuery as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPRemoteMediaServices_Impl: Sized {
    fn GetServiceType(&self, pbstrtype: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetApplicationName(&self, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetScriptableObject(&self, pbstrname: *mut windows_core::BSTR, ppdispatch: *mut Option<super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
    fn GetCustomUIMode(&self, pbstrfile: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPRemoteMediaServices {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPRemoteMediaServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPRemoteMediaServices_Vtbl
    where
        Identity: IWMPRemoteMediaServices_Impl,
    {
        unsafe extern "system" fn GetServiceType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPRemoteMediaServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPRemoteMediaServices_Impl::GetServiceType(this, core::mem::transmute_copy(&pbstrtype)).into()
        }
        unsafe extern "system" fn GetApplicationName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPRemoteMediaServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPRemoteMediaServices_Impl::GetApplicationName(this, core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn GetScriptableObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>, ppdispatch: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPRemoteMediaServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPRemoteMediaServices_Impl::GetScriptableObject(this, core::mem::transmute_copy(&pbstrname), core::mem::transmute_copy(&ppdispatch)).into()
        }
        unsafe extern "system" fn GetCustomUIMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfile: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPRemoteMediaServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPRemoteMediaServices_Impl::GetCustomUIMode(this, core::mem::transmute_copy(&pbstrfile)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetServiceType: GetServiceType::<Identity, OFFSET>,
            GetApplicationName: GetApplicationName::<Identity, OFFSET>,
            GetScriptableObject: GetScriptableObject::<Identity, OFFSET>,
            GetCustomUIMode: GetCustomUIMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPRemoteMediaServices as windows_core::Interface>::IID
    }
}
pub trait IWMPRenderConfig_Impl: Sized {
    fn SetinProcOnly(&self, finproc: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn inProcOnly(&self, pfinproc: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPRenderConfig {}
impl IWMPRenderConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPRenderConfig_Vtbl
    where
        Identity: IWMPRenderConfig_Impl,
    {
        unsafe extern "system" fn SetinProcOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, finproc: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPRenderConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPRenderConfig_Impl::SetinProcOnly(this, core::mem::transmute_copy(&finproc)).into()
        }
        unsafe extern "system" fn inProcOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfinproc: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPRenderConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPRenderConfig_Impl::inProcOnly(this, core::mem::transmute_copy(&pfinproc)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetinProcOnly: SetinProcOnly::<Identity, OFFSET>,
            inProcOnly: inProcOnly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPRenderConfig as windows_core::Interface>::IID
    }
}
pub trait IWMPServices_Impl: Sized {
    fn GetStreamTime(&self, prt: *mut i64) -> windows_core::Result<()>;
    fn GetStreamState(&self, pstate: *mut WMPServices_StreamState) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPServices {}
impl IWMPServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPServices_Vtbl
    where
        Identity: IWMPServices_Impl,
    {
        unsafe extern "system" fn GetStreamTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prt: *mut i64) -> windows_core::HRESULT
        where
            Identity: IWMPServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPServices_Impl::GetStreamTime(this, core::mem::transmute_copy(&prt)).into()
        }
        unsafe extern "system" fn GetStreamState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut WMPServices_StreamState) -> windows_core::HRESULT
        where
            Identity: IWMPServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPServices_Impl::GetStreamState(this, core::mem::transmute_copy(&pstate)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreamTime: GetStreamTime::<Identity, OFFSET>,
            GetStreamState: GetStreamState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPSettings_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_isAvailable(&self, bstritem: &windows_core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn autoStart(&self, pfautostart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetautoStart(&self, fautostart: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn baseURL(&self, pbstrbaseurl: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetbaseURL(&self, bstrbaseurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn defaultFrame(&self, pbstrdefaultframe: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetdefaultFrame(&self, bstrdefaultframe: &windows_core::BSTR) -> windows_core::Result<()>;
    fn invokeURLs(&self, pfinvokeurls: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetinvokeURLs(&self, finvokeurls: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn mute(&self, pfmute: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Setmute(&self, fmute: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn playCount(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn SetplayCount(&self, lcount: i32) -> windows_core::Result<()>;
    fn rate(&self, pdrate: *mut f64) -> windows_core::Result<()>;
    fn Setrate(&self, drate: f64) -> windows_core::Result<()>;
    fn balance(&self, plbalance: *mut i32) -> windows_core::Result<()>;
    fn Setbalance(&self, lbalance: i32) -> windows_core::Result<()>;
    fn volume(&self, plvolume: *mut i32) -> windows_core::Result<()>;
    fn Setvolume(&self, lvolume: i32) -> windows_core::Result<()>;
    fn getMode(&self, bstrmode: &windows_core::BSTR, pvarfmode: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn setMode(&self, bstrmode: &windows_core::BSTR, varfmode: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn enableErrorDialogs(&self, pfenableerrordialogs: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetenableErrorDialogs(&self, fenableerrordialogs: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPSettings {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPSettings_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPSettings_Vtbl
    where
        Identity: IWMPSettings_Impl,
    {
        unsafe extern "system" fn get_isAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritem: core::mem::MaybeUninit<windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::get_isAvailable(this, core::mem::transmute(&bstritem), core::mem::transmute_copy(&pisavailable)).into()
        }
        unsafe extern "system" fn autoStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfautostart: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::autoStart(this, core::mem::transmute_copy(&pfautostart)).into()
        }
        unsafe extern "system" fn SetautoStart<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fautostart: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::SetautoStart(this, core::mem::transmute_copy(&fautostart)).into()
        }
        unsafe extern "system" fn baseURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbaseurl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::baseURL(this, core::mem::transmute_copy(&pbstrbaseurl)).into()
        }
        unsafe extern "system" fn SetbaseURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrbaseurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::SetbaseURL(this, core::mem::transmute(&bstrbaseurl)).into()
        }
        unsafe extern "system" fn defaultFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdefaultframe: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::defaultFrame(this, core::mem::transmute_copy(&pbstrdefaultframe)).into()
        }
        unsafe extern "system" fn SetdefaultFrame<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdefaultframe: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::SetdefaultFrame(this, core::mem::transmute(&bstrdefaultframe)).into()
        }
        unsafe extern "system" fn invokeURLs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfinvokeurls: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::invokeURLs(this, core::mem::transmute_copy(&pfinvokeurls)).into()
        }
        unsafe extern "system" fn SetinvokeURLs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, finvokeurls: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::SetinvokeURLs(this, core::mem::transmute_copy(&finvokeurls)).into()
        }
        unsafe extern "system" fn mute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmute: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::mute(this, core::mem::transmute_copy(&pfmute)).into()
        }
        unsafe extern "system" fn Setmute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmute: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::Setmute(this, core::mem::transmute_copy(&fmute)).into()
        }
        unsafe extern "system" fn playCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::playCount(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn SetplayCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcount: i32) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::SetplayCount(this, core::mem::transmute_copy(&lcount)).into()
        }
        unsafe extern "system" fn rate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdrate: *mut f64) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::rate(this, core::mem::transmute_copy(&pdrate)).into()
        }
        unsafe extern "system" fn Setrate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, drate: f64) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::Setrate(this, core::mem::transmute_copy(&drate)).into()
        }
        unsafe extern "system" fn balance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plbalance: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::balance(this, core::mem::transmute_copy(&plbalance)).into()
        }
        unsafe extern "system" fn Setbalance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbalance: i32) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::Setbalance(this, core::mem::transmute_copy(&lbalance)).into()
        }
        unsafe extern "system" fn volume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plvolume: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::volume(this, core::mem::transmute_copy(&plvolume)).into()
        }
        unsafe extern "system" fn Setvolume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lvolume: i32) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::Setvolume(this, core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn getMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: core::mem::MaybeUninit<windows_core::BSTR>, pvarfmode: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::getMode(this, core::mem::transmute(&bstrmode), core::mem::transmute_copy(&pvarfmode)).into()
        }
        unsafe extern "system" fn setMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmode: core::mem::MaybeUninit<windows_core::BSTR>, varfmode: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::setMode(this, core::mem::transmute(&bstrmode), core::mem::transmute_copy(&varfmode)).into()
        }
        unsafe extern "system" fn enableErrorDialogs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenableerrordialogs: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::enableErrorDialogs(this, core::mem::transmute_copy(&pfenableerrordialogs)).into()
        }
        unsafe extern "system" fn SetenableErrorDialogs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenableerrordialogs: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings_Impl::SetenableErrorDialogs(this, core::mem::transmute_copy(&fenableerrordialogs)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_isAvailable: get_isAvailable::<Identity, OFFSET>,
            autoStart: autoStart::<Identity, OFFSET>,
            SetautoStart: SetautoStart::<Identity, OFFSET>,
            baseURL: baseURL::<Identity, OFFSET>,
            SetbaseURL: SetbaseURL::<Identity, OFFSET>,
            defaultFrame: defaultFrame::<Identity, OFFSET>,
            SetdefaultFrame: SetdefaultFrame::<Identity, OFFSET>,
            invokeURLs: invokeURLs::<Identity, OFFSET>,
            SetinvokeURLs: SetinvokeURLs::<Identity, OFFSET>,
            mute: mute::<Identity, OFFSET>,
            Setmute: Setmute::<Identity, OFFSET>,
            playCount: playCount::<Identity, OFFSET>,
            SetplayCount: SetplayCount::<Identity, OFFSET>,
            rate: rate::<Identity, OFFSET>,
            Setrate: Setrate::<Identity, OFFSET>,
            balance: balance::<Identity, OFFSET>,
            Setbalance: Setbalance::<Identity, OFFSET>,
            volume: volume::<Identity, OFFSET>,
            Setvolume: Setvolume::<Identity, OFFSET>,
            getMode: getMode::<Identity, OFFSET>,
            setMode: setMode::<Identity, OFFSET>,
            enableErrorDialogs: enableErrorDialogs::<Identity, OFFSET>,
            SetenableErrorDialogs: SetenableErrorDialogs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSettings as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPSettings2_Impl: Sized + IWMPSettings_Impl {
    fn defaultAudioLanguage(&self, pllangid: *mut i32) -> windows_core::Result<()>;
    fn mediaAccessRights(&self, pbstrrights: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn requestMediaAccessRights(&self, bstrdesiredaccess: &windows_core::BSTR, pvbaccepted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPSettings2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPSettings2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPSettings2_Vtbl
    where
        Identity: IWMPSettings2_Impl,
    {
        unsafe extern "system" fn defaultAudioLanguage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllangid: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPSettings2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings2_Impl::defaultAudioLanguage(this, core::mem::transmute_copy(&pllangid)).into()
        }
        unsafe extern "system" fn mediaAccessRights<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrights: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSettings2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings2_Impl::mediaAccessRights(this, core::mem::transmute_copy(&pbstrrights)).into()
        }
        unsafe extern "system" fn requestMediaAccessRights<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdesiredaccess: core::mem::MaybeUninit<windows_core::BSTR>, pvbaccepted: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSettings2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSettings2_Impl::requestMediaAccessRights(this, core::mem::transmute(&bstrdesiredaccess), core::mem::transmute_copy(&pvbaccepted)).into()
        }
        Self {
            base__: IWMPSettings_Vtbl::new::<Identity, OFFSET>(),
            defaultAudioLanguage: defaultAudioLanguage::<Identity, OFFSET>,
            mediaAccessRights: mediaAccessRights::<Identity, OFFSET>,
            requestMediaAccessRights: requestMediaAccessRights::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSettings2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPSettings as windows_core::Interface>::IID
    }
}
pub trait IWMPSkinManager_Impl: Sized {
    fn SetVisualStyle(&self, bstrpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPSkinManager {}
impl IWMPSkinManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPSkinManager_Vtbl
    where
        Identity: IWMPSkinManager_Impl,
    {
        unsafe extern "system" fn SetVisualStyle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSkinManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSkinManager_Impl::SetVisualStyle(this, core::mem::transmute(&bstrpath)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetVisualStyle: SetVisualStyle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSkinManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPStringCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn count(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn item(&self, lindex: i32, pbstrstring: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPStringCollection {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPStringCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPStringCollection_Vtbl
    where
        Identity: IWMPStringCollection_Impl,
    {
        unsafe extern "system" fn count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPStringCollection_Impl::count(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstrstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPStringCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPStringCollection_Impl::item(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&pbstrstring)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), count: count::<Identity, OFFSET>, item: item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPStringCollection as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPStringCollection2_Impl: Sized + IWMPStringCollection_Impl {
    fn isIdentical(&self, piwmpstringcollection2: Option<&IWMPStringCollection2>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn getItemInfo(&self, lcollectionindex: i32, bstritemname: &windows_core::BSTR, pbstrvalue: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn getAttributeCountByType(&self, lcollectionindex: i32, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR, plcount: *mut i32) -> windows_core::Result<()>;
    fn getItemInfoByType(&self, lcollectionindex: i32, bstrtype: &windows_core::BSTR, bstrlanguage: &windows_core::BSTR, lattributeindex: i32, pvarvalue: *mut windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPStringCollection2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPStringCollection2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPStringCollection2_Vtbl
    where
        Identity: IWMPStringCollection2_Impl,
    {
        unsafe extern "system" fn isIdentical<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piwmpstringcollection2: *mut core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPStringCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPStringCollection2_Impl::isIdentical(this, windows_core::from_raw_borrowed(&piwmpstringcollection2), core::mem::transmute_copy(&pvbool)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcollectionindex: i32, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPStringCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPStringCollection2_Impl::getItemInfo(this, core::mem::transmute_copy(&lcollectionindex), core::mem::transmute(&bstritemname), core::mem::transmute_copy(&pbstrvalue)).into()
        }
        unsafe extern "system" fn getAttributeCountByType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcollectionindex: i32, bstrtype: core::mem::MaybeUninit<windows_core::BSTR>, bstrlanguage: core::mem::MaybeUninit<windows_core::BSTR>, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPStringCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPStringCollection2_Impl::getAttributeCountByType(this, core::mem::transmute_copy(&lcollectionindex), core::mem::transmute(&bstrtype), core::mem::transmute(&bstrlanguage), core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getItemInfoByType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcollectionindex: i32, bstrtype: core::mem::MaybeUninit<windows_core::BSTR>, bstrlanguage: core::mem::MaybeUninit<windows_core::BSTR>, lattributeindex: i32, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: IWMPStringCollection2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPStringCollection2_Impl::getItemInfoByType(this, core::mem::transmute_copy(&lcollectionindex), core::mem::transmute(&bstrtype), core::mem::transmute(&bstrlanguage), core::mem::transmute_copy(&lattributeindex), core::mem::transmute_copy(&pvarvalue)).into()
        }
        Self {
            base__: IWMPStringCollection_Vtbl::new::<Identity, OFFSET>(),
            isIdentical: isIdentical::<Identity, OFFSET>,
            getItemInfo: getItemInfo::<Identity, OFFSET>,
            getAttributeCountByType: getAttributeCountByType::<Identity, OFFSET>,
            getItemInfoByType: getItemInfoByType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPStringCollection2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWMPStringCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPSubscriptionService_Impl: Sized {
    fn allowPlay(&self, hwnd: super::super::Foundation::HWND, pmedia: Option<&IWMPMedia>, pfallowplay: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn allowCDBurn(&self, hwnd: super::super::Foundation::HWND, pplaylist: Option<&IWMPPlaylist>, pfallowburn: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn allowPDATransfer(&self, hwnd: super::super::Foundation::HWND, pplaylist: Option<&IWMPPlaylist>, pfallowtransfer: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn startBackgroundProcessing(&self, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPSubscriptionService {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPSubscriptionService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPSubscriptionService_Vtbl
    where
        Identity: IWMPSubscriptionService_Impl,
    {
        unsafe extern "system" fn allowPlay<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, pmedia: *mut core::ffi::c_void, pfallowplay: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSubscriptionService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSubscriptionService_Impl::allowPlay(this, core::mem::transmute_copy(&hwnd), windows_core::from_raw_borrowed(&pmedia), core::mem::transmute_copy(&pfallowplay)).into()
        }
        unsafe extern "system" fn allowCDBurn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: *mut core::ffi::c_void, pfallowburn: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSubscriptionService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSubscriptionService_Impl::allowCDBurn(this, core::mem::transmute_copy(&hwnd), windows_core::from_raw_borrowed(&pplaylist), core::mem::transmute_copy(&pfallowburn)).into()
        }
        unsafe extern "system" fn allowPDATransfer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: *mut core::ffi::c_void, pfallowtransfer: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSubscriptionService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSubscriptionService_Impl::allowPDATransfer(this, core::mem::transmute_copy(&hwnd), windows_core::from_raw_borrowed(&pplaylist), core::mem::transmute_copy(&pfallowtransfer)).into()
        }
        unsafe extern "system" fn startBackgroundProcessing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IWMPSubscriptionService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSubscriptionService_Impl::startBackgroundProcessing(this, core::mem::transmute_copy(&hwnd)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            allowPlay: allowPlay::<Identity, OFFSET>,
            allowCDBurn: allowCDBurn::<Identity, OFFSET>,
            allowPDATransfer: allowPDATransfer::<Identity, OFFSET>,
            startBackgroundProcessing: startBackgroundProcessing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSubscriptionService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPSubscriptionService2_Impl: Sized + IWMPSubscriptionService_Impl {
    fn stopBackgroundProcessing(&self) -> windows_core::Result<()>;
    fn serviceEvent(&self, event: WMPSubscriptionServiceEvent) -> windows_core::Result<()>;
    fn deviceAvailable(&self, bstrdevicename: &windows_core::BSTR, pcb: Option<&IWMPSubscriptionServiceCallback>) -> windows_core::Result<()>;
    fn prepareForSync(&self, bstrfilename: &windows_core::BSTR, bstrdevicename: &windows_core::BSTR, pcb: Option<&IWMPSubscriptionServiceCallback>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPSubscriptionService2 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPSubscriptionService2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPSubscriptionService2_Vtbl
    where
        Identity: IWMPSubscriptionService2_Impl,
    {
        unsafe extern "system" fn stopBackgroundProcessing<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPSubscriptionService2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSubscriptionService2_Impl::stopBackgroundProcessing(this).into()
        }
        unsafe extern "system" fn serviceEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: WMPSubscriptionServiceEvent) -> windows_core::HRESULT
        where
            Identity: IWMPSubscriptionService2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSubscriptionService2_Impl::serviceEvent(this, core::mem::transmute_copy(&event)).into()
        }
        unsafe extern "system" fn deviceAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdevicename: core::mem::MaybeUninit<windows_core::BSTR>, pcb: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPSubscriptionService2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSubscriptionService2_Impl::deviceAvailable(this, core::mem::transmute(&bstrdevicename), windows_core::from_raw_borrowed(&pcb)).into()
        }
        unsafe extern "system" fn prepareForSync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfilename: core::mem::MaybeUninit<windows_core::BSTR>, bstrdevicename: core::mem::MaybeUninit<windows_core::BSTR>, pcb: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPSubscriptionService2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSubscriptionService2_Impl::prepareForSync(this, core::mem::transmute(&bstrfilename), core::mem::transmute(&bstrdevicename), windows_core::from_raw_borrowed(&pcb)).into()
        }
        Self {
            base__: IWMPSubscriptionService_Vtbl::new::<Identity, OFFSET>(),
            stopBackgroundProcessing: stopBackgroundProcessing::<Identity, OFFSET>,
            serviceEvent: serviceEvent::<Identity, OFFSET>,
            deviceAvailable: deviceAvailable::<Identity, OFFSET>,
            prepareForSync: prepareForSync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSubscriptionService2 as windows_core::Interface>::IID || iid == &<IWMPSubscriptionService as windows_core::Interface>::IID
    }
}
pub trait IWMPSubscriptionServiceCallback_Impl: Sized {
    fn onComplete(&self, hrresult: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPSubscriptionServiceCallback {}
impl IWMPSubscriptionServiceCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPSubscriptionServiceCallback_Vtbl
    where
        Identity: IWMPSubscriptionServiceCallback_Impl,
    {
        unsafe extern "system" fn onComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrresult: windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: IWMPSubscriptionServiceCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSubscriptionServiceCallback_Impl::onComplete(this, core::mem::transmute_copy(&hrresult)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), onComplete: onComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSubscriptionServiceCallback as windows_core::Interface>::IID
    }
}
pub trait IWMPSyncDevice_Impl: Sized {
    fn friendlyName(&self, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetfriendlyName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn deviceName(&self, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn deviceId(&self, pbstrdeviceid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn partnershipIndex(&self, plindex: *mut i32) -> windows_core::Result<()>;
    fn connected(&self, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn status(&self, pwmpds: *mut WMPDeviceStatus) -> windows_core::Result<()>;
    fn syncState(&self, pwmpss: *mut WMPSyncState) -> windows_core::Result<()>;
    fn progress(&self, plprogress: *mut i32) -> windows_core::Result<()>;
    fn getItemInfo(&self, bstritemname: &windows_core::BSTR, pbstrval: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn createPartnership(&self, vbshowui: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn deletePartnership(&self) -> windows_core::Result<()>;
    fn start(&self) -> windows_core::Result<()>;
    fn stop(&self) -> windows_core::Result<()>;
    fn showSettings(&self) -> windows_core::Result<()>;
    fn isIdentical(&self, pdevice: Option<&IWMPSyncDevice>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPSyncDevice {}
impl IWMPSyncDevice_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPSyncDevice_Vtbl
    where
        Identity: IWMPSyncDevice_Impl,
    {
        unsafe extern "system" fn friendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::friendlyName(this, core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn SetfriendlyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::SetfriendlyName(this, core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn deviceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::deviceName(this, core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn deviceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdeviceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::deviceId(this, core::mem::transmute_copy(&pbstrdeviceid)).into()
        }
        unsafe extern "system" fn partnershipIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plindex: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::partnershipIndex(this, core::mem::transmute_copy(&plindex)).into()
        }
        unsafe extern "system" fn connected<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::connected(this, core::mem::transmute_copy(&pvbconnected)).into()
        }
        unsafe extern "system" fn status<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpds: *mut WMPDeviceStatus) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::status(this, core::mem::transmute_copy(&pwmpds)).into()
        }
        unsafe extern "system" fn syncState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwmpss: *mut WMPSyncState) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::syncState(this, core::mem::transmute_copy(&pwmpss)).into()
        }
        unsafe extern "system" fn progress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plprogress: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::progress(this, core::mem::transmute_copy(&plprogress)).into()
        }
        unsafe extern "system" fn getItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, pbstrval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::getItemInfo(this, core::mem::transmute(&bstritemname), core::mem::transmute_copy(&pbstrval)).into()
        }
        unsafe extern "system" fn createPartnership<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vbshowui: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::createPartnership(this, core::mem::transmute_copy(&vbshowui)).into()
        }
        unsafe extern "system" fn deletePartnership<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::deletePartnership(this).into()
        }
        unsafe extern "system" fn start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::start(this).into()
        }
        unsafe extern "system" fn stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::stop(this).into()
        }
        unsafe extern "system" fn showSettings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::showSettings(this).into()
        }
        unsafe extern "system" fn isIdentical<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice_Impl::isIdentical(this, windows_core::from_raw_borrowed(&pdevice), core::mem::transmute_copy(&pvbool)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            friendlyName: friendlyName::<Identity, OFFSET>,
            SetfriendlyName: SetfriendlyName::<Identity, OFFSET>,
            deviceName: deviceName::<Identity, OFFSET>,
            deviceId: deviceId::<Identity, OFFSET>,
            partnershipIndex: partnershipIndex::<Identity, OFFSET>,
            connected: connected::<Identity, OFFSET>,
            status: status::<Identity, OFFSET>,
            syncState: syncState::<Identity, OFFSET>,
            progress: progress::<Identity, OFFSET>,
            getItemInfo: getItemInfo::<Identity, OFFSET>,
            createPartnership: createPartnership::<Identity, OFFSET>,
            deletePartnership: deletePartnership::<Identity, OFFSET>,
            start: start::<Identity, OFFSET>,
            stop: stop::<Identity, OFFSET>,
            showSettings: showSettings::<Identity, OFFSET>,
            isIdentical: isIdentical::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSyncDevice as windows_core::Interface>::IID
    }
}
pub trait IWMPSyncDevice2_Impl: Sized + IWMPSyncDevice_Impl {
    fn setItemInfo(&self, bstritemname: &windows_core::BSTR, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPSyncDevice2 {}
impl IWMPSyncDevice2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPSyncDevice2_Vtbl
    where
        Identity: IWMPSyncDevice2_Impl,
    {
        unsafe extern "system" fn setItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstritemname: core::mem::MaybeUninit<windows_core::BSTR>, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice2_Impl::setItemInfo(this, core::mem::transmute(&bstritemname), core::mem::transmute(&bstrval)).into()
        }
        Self { base__: IWMPSyncDevice_Vtbl::new::<Identity, OFFSET>(), setItemInfo: setItemInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSyncDevice2 as windows_core::Interface>::IID || iid == &<IWMPSyncDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPSyncDevice3_Impl: Sized + IWMPSyncDevice2_Impl {
    fn estimateSyncSize(&self, pnonruleplaylist: Option<&IWMPPlaylist>, prulesplaylist: Option<&IWMPPlaylist>) -> windows_core::Result<()>;
    fn cancelEstimation(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IWMPSyncDevice3 {}
#[cfg(feature = "Win32_System_Com")]
impl IWMPSyncDevice3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPSyncDevice3_Vtbl
    where
        Identity: IWMPSyncDevice3_Impl,
    {
        unsafe extern "system" fn estimateSyncSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnonruleplaylist: *mut core::ffi::c_void, prulesplaylist: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice3_Impl::estimateSyncSize(this, windows_core::from_raw_borrowed(&pnonruleplaylist), windows_core::from_raw_borrowed(&prulesplaylist)).into()
        }
        unsafe extern "system" fn cancelEstimation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPSyncDevice3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncDevice3_Impl::cancelEstimation(this).into()
        }
        Self {
            base__: IWMPSyncDevice2_Vtbl::new::<Identity, OFFSET>(),
            estimateSyncSize: estimateSyncSize::<Identity, OFFSET>,
            cancelEstimation: cancelEstimation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSyncDevice3 as windows_core::Interface>::IID || iid == &<IWMPSyncDevice as windows_core::Interface>::IID || iid == &<IWMPSyncDevice2 as windows_core::Interface>::IID
    }
}
pub trait IWMPSyncServices_Impl: Sized {
    fn deviceCount(&self, plcount: *mut i32) -> windows_core::Result<()>;
    fn getDevice(&self, lindex: i32) -> windows_core::Result<IWMPSyncDevice>;
}
impl windows_core::RuntimeName for IWMPSyncServices {}
impl IWMPSyncServices_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPSyncServices_Vtbl
    where
        Identity: IWMPSyncServices_Impl,
    {
        unsafe extern "system" fn deviceCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IWMPSyncServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPSyncServices_Impl::deviceCount(this, core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn getDevice<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPSyncServices_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWMPSyncServices_Impl::getDevice(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    ppdevice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            deviceCount: deviceCount::<Identity, OFFSET>,
            getDevice: getDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPSyncServices as windows_core::Interface>::IID
    }
}
pub trait IWMPTranscodePolicy_Impl: Sized {
    fn allowTranscode(&self, pvballow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPTranscodePolicy {}
impl IWMPTranscodePolicy_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPTranscodePolicy_Vtbl
    where
        Identity: IWMPTranscodePolicy_Impl,
    {
        unsafe extern "system" fn allowTranscode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvballow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPTranscodePolicy_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPTranscodePolicy_Impl::allowTranscode(this, core::mem::transmute_copy(&pvballow)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), allowTranscode: allowTranscode::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPTranscodePolicy as windows_core::Interface>::IID
    }
}
pub trait IWMPUserEventSink_Impl: Sized {
    fn NotifyUserEvent(&self, eventcode: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPUserEventSink {}
impl IWMPUserEventSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPUserEventSink_Vtbl
    where
        Identity: IWMPUserEventSink_Impl,
    {
        unsafe extern "system" fn NotifyUserEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcode: i32) -> windows_core::HRESULT
        where
            Identity: IWMPUserEventSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPUserEventSink_Impl::NotifyUserEvent(this, core::mem::transmute_copy(&eventcode)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NotifyUserEvent: NotifyUserEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPUserEventSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IWMPVideoRenderConfig_Impl: Sized {
    fn SetpresenterActivate(&self, pactivate: Option<&super::MediaFoundation::IMFActivate>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl windows_core::RuntimeName for IWMPVideoRenderConfig {}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl IWMPVideoRenderConfig_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPVideoRenderConfig_Vtbl
    where
        Identity: IWMPVideoRenderConfig_Impl,
    {
        unsafe extern "system" fn SetpresenterActivate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactivate: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWMPVideoRenderConfig_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPVideoRenderConfig_Impl::SetpresenterActivate(this, windows_core::from_raw_borrowed(&pactivate)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetpresenterActivate: SetpresenterActivate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPVideoRenderConfig as windows_core::Interface>::IID
    }
}
pub trait IWMPWindowMessageSink_Impl: Sized {
    fn OnWindowMessage(&self, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWMPWindowMessageSink {}
impl IWMPWindowMessageSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWMPWindowMessageSink_Vtbl
    where
        Identity: IWMPWindowMessageSink_Impl,
    {
        unsafe extern "system" fn OnWindowMessage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWMPWindowMessageSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWMPWindowMessageSink_Impl::OnWindowMessage(this, core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam), core::mem::transmute_copy(&plret), core::mem::transmute_copy(&pfhandled)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnWindowMessage: OnWindowMessage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWMPWindowMessageSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXFeed_Impl: Sized {
    fn Xml(&self, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> windows_core::Result<super::super::System::Com::IStream>;
    fn Name(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Rename(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Url(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetUrl(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LocalId(&self) -> windows_core::Result<windows_core::GUID>;
    fn Path(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Move(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Parent(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn LastWriteTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Download(&self) -> windows_core::Result<()>;
    fn AsyncDownload(&self) -> windows_core::Result<()>;
    fn CancelAsyncDownload(&self) -> windows_core::Result<()>;
    fn SyncSetting(&self) -> windows_core::Result<FEEDS_SYNC_SETTING>;
    fn SetSyncSetting(&self, fss: FEEDS_SYNC_SETTING) -> windows_core::Result<()>;
    fn Interval(&self) -> windows_core::Result<u32>;
    fn SetInterval(&self, uiinterval: u32) -> windows_core::Result<()>;
    fn LastDownloadTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn LocalEnclosurePath(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Items(&self) -> windows_core::Result<IXFeedsEnum>;
    fn GetItem(&self, uiid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn MarkAllItemsRead(&self) -> windows_core::Result<()>;
    fn MaxItemCount(&self) -> windows_core::Result<u32>;
    fn SetMaxItemCount(&self, uimaxitemcount: u32) -> windows_core::Result<()>;
    fn DownloadEnclosuresAutomatically(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetDownloadEnclosuresAutomatically(&self, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn DownloadStatus(&self) -> windows_core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&self) -> windows_core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn Merge(&self, pstream: Option<&super::super::System::Com::IStream>, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DownloadUrl(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Title(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Link(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Image(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn LastBuildDate(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn PubDate(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Ttl(&self) -> windows_core::Result<u32>;
    fn Language(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Copyright(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsList(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn UnreadItemCount(&self) -> windows_core::Result<u32>;
    fn ItemCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXFeed {}
#[cfg(feature = "Win32_System_Com")]
impl IXFeed_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXFeed_Vtbl
    where
        Identity: IXFeed_Impl,
    {
        unsafe extern "system" fn Xml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, pps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Xml(this, core::mem::transmute_copy(&uiitemcount), core::mem::transmute_copy(&sortproperty), core::mem::transmute_copy(&sortorder), core::mem::transmute_copy(&filterflags), core::mem::transmute_copy(&includeflags)) {
                Ok(ok__) => {
                    pps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Name(this) {
                Ok(ok__) => {
                    ppszname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::Rename(this, core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn Url<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Url(this) {
                Ok(ok__) => {
                    ppszurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::SetUrl(this, core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn LocalId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::LocalId(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Path(this) {
                Ok(ok__) => {
                    ppszpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::Move(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::Parent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn LastWriteTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstlastwritetime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::LastWriteTime(this) {
                Ok(ok__) => {
                    pstlastwritetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Download<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::Download(this).into()
        }
        unsafe extern "system" fn AsyncDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::AsyncDownload(this).into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::CancelAsyncDownload(this).into()
        }
        unsafe extern "system" fn SyncSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfss: *mut FEEDS_SYNC_SETTING) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::SyncSetting(this) {
                Ok(ok__) => {
                    pfss.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncSetting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fss: FEEDS_SYNC_SETTING) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::SetSyncSetting(this, core::mem::transmute_copy(&fss)).into()
        }
        unsafe extern "system" fn Interval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiinterval: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Interval(this) {
                Ok(ok__) => {
                    puiinterval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiinterval: u32) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::SetInterval(this, core::mem::transmute_copy(&uiinterval)).into()
        }
        unsafe extern "system" fn LastDownloadTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::LastDownloadTime(this) {
                Ok(ok__) => {
                    pstlastdownloadtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalEnclosurePath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::LocalEnclosurePath(this) {
                Ok(ok__) => {
                    ppszpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Items<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Items(this) {
                Ok(ok__) => {
                    ppfe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::GetItem(this, core::mem::transmute_copy(&uiid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn MarkAllItemsRead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::MarkAllItemsRead(this).into()
        }
        unsafe extern "system" fn MaxItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puimaxitemcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::MaxItemCount(this) {
                Ok(ok__) => {
                    puimaxitemcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uimaxitemcount: u32) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::SetMaxItemCount(this, core::mem::transmute_copy(&uimaxitemcount)).into()
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdownloadenclosuresautomatically: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::DownloadEnclosuresAutomatically(this) {
                Ok(ok__) => {
                    pbdownloadenclosuresautomatically.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::SetDownloadEnclosuresAutomatically(this, core::mem::transmute_copy(&bdownloadenclosuresautomatically)).into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::DownloadStatus(this) {
                Ok(ok__) => {
                    pfds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::LastDownloadError(this) {
                Ok(ok__) => {
                    pfde.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Merge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::Merge(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::DownloadUrl(this) {
                Ok(ok__) => {
                    ppszurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsztitle: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Title(this) {
                Ok(ok__) => {
                    ppsztitle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Description(this) {
                Ok(ok__) => {
                    ppszdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszhomepage: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Link(this) {
                Ok(ok__) => {
                    ppszhomepage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszimageurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Image(this) {
                Ok(ok__) => {
                    ppszimageurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBuildDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstlastbuilddate: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::LastBuildDate(this) {
                Ok(ok__) => {
                    pstlastbuilddate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::PubDate(this) {
                Ok(ok__) => {
                    pstpubdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ttl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puittl: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Ttl(this) {
                Ok(ok__) => {
                    puittl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszlanguage: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Language(this) {
                Ok(ok__) => {
                    ppszlanguage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Copyright<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcopyright: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::Copyright(this) {
                Ok(ok__) => {
                    ppszcopyright.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbislist: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::IsList(this) {
                Ok(ok__) => {
                    pbislist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed_Impl::GetWatcher(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&mask), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn UnreadItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiunreaditemcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::UnreadItemCount(this) {
                Ok(ok__) => {
                    puiunreaditemcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiitemcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeed_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed_Impl::ItemCount(this) {
                Ok(ok__) => {
                    puiitemcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Xml: Xml::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Rename: Rename::<Identity, OFFSET>,
            Url: Url::<Identity, OFFSET>,
            SetUrl: SetUrl::<Identity, OFFSET>,
            LocalId: LocalId::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            LastWriteTime: LastWriteTime::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, OFFSET>,
            SyncSetting: SyncSetting::<Identity, OFFSET>,
            SetSyncSetting: SetSyncSetting::<Identity, OFFSET>,
            Interval: Interval::<Identity, OFFSET>,
            SetInterval: SetInterval::<Identity, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, OFFSET>,
            LocalEnclosurePath: LocalEnclosurePath::<Identity, OFFSET>,
            Items: Items::<Identity, OFFSET>,
            GetItem: GetItem::<Identity, OFFSET>,
            MarkAllItemsRead: MarkAllItemsRead::<Identity, OFFSET>,
            MaxItemCount: MaxItemCount::<Identity, OFFSET>,
            SetMaxItemCount: SetMaxItemCount::<Identity, OFFSET>,
            DownloadEnclosuresAutomatically: DownloadEnclosuresAutomatically::<Identity, OFFSET>,
            SetDownloadEnclosuresAutomatically: SetDownloadEnclosuresAutomatically::<Identity, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, OFFSET>,
            Merge: Merge::<Identity, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Link: Link::<Identity, OFFSET>,
            Image: Image::<Identity, OFFSET>,
            LastBuildDate: LastBuildDate::<Identity, OFFSET>,
            PubDate: PubDate::<Identity, OFFSET>,
            Ttl: Ttl::<Identity, OFFSET>,
            Language: Language::<Identity, OFFSET>,
            Copyright: Copyright::<Identity, OFFSET>,
            IsList: IsList::<Identity, OFFSET>,
            GetWatcher: GetWatcher::<Identity, OFFSET>,
            UnreadItemCount: UnreadItemCount::<Identity, OFFSET>,
            ItemCount: ItemCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXFeed as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXFeed2_Impl: Sized + IXFeed_Impl {
    fn GetItemByEffectiveId(&self, uieffectiveid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn LastItemDownloadTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Username(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Password(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetCredentials(&self, pszusername: &windows_core::PCWSTR, pszpassword: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ClearCredentials(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXFeed2 {}
#[cfg(feature = "Win32_System_Com")]
impl IXFeed2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXFeed2_Vtbl
    where
        Identity: IXFeed2_Impl,
    {
        unsafe extern "system" fn GetItemByEffectiveId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uieffectiveid: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed2_Impl::GetItemByEffectiveId(this, core::mem::transmute_copy(&uieffectiveid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn LastItemDownloadTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstlastitemdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed2_Impl::LastItemDownloadTime(this) {
                Ok(ok__) => {
                    pstlastitemdownloadtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Username<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszusername: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed2_Impl::Username(this) {
                Ok(ok__) => {
                    ppszusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpassword: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeed2_Impl::Password(this) {
                Ok(ok__) => {
                    ppszpassword.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszusername: windows_core::PCWSTR, pszpassword: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed2_Impl::SetCredentials(this, core::mem::transmute(&pszusername), core::mem::transmute(&pszpassword)).into()
        }
        unsafe extern "system" fn ClearCredentials<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeed2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeed2_Impl::ClearCredentials(this).into()
        }
        Self {
            base__: IXFeed_Vtbl::new::<Identity, OFFSET>(),
            GetItemByEffectiveId: GetItemByEffectiveId::<Identity, OFFSET>,
            LastItemDownloadTime: LastItemDownloadTime::<Identity, OFFSET>,
            Username: Username::<Identity, OFFSET>,
            Password: Password::<Identity, OFFSET>,
            SetCredentials: SetCredentials::<Identity, OFFSET>,
            ClearCredentials: ClearCredentials::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXFeed2 as windows_core::Interface>::IID || iid == &<IXFeed as windows_core::Interface>::IID
    }
}
pub trait IXFeedEnclosure_Impl: Sized {
    fn Url(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Type(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Length(&self) -> windows_core::Result<u32>;
    fn AsyncDownload(&self) -> windows_core::Result<()>;
    fn CancelAsyncDownload(&self) -> windows_core::Result<()>;
    fn DownloadStatus(&self) -> windows_core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(&self) -> windows_core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn LocalPath(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Parent(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DownloadUrl(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn DownloadMimeType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn RemoveFile(&self) -> windows_core::Result<()>;
    fn SetFile(&self, pszdownloadurl: &windows_core::PCWSTR, pszdownloadfilepath: &windows_core::PCWSTR, pszdownloadmimetype: &windows_core::PCWSTR, pszenclosurefilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXFeedEnclosure {}
impl IXFeedEnclosure_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXFeedEnclosure_Vtbl
    where
        Identity: IXFeedEnclosure_Impl,
    {
        unsafe extern "system" fn Url<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedEnclosure_Impl::Url(this) {
                Ok(ok__) => {
                    ppszurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszmimetype: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedEnclosure_Impl::Type(this) {
                Ok(ok__) => {
                    ppszmimetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puilength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedEnclosure_Impl::Length(this) {
                Ok(ok__) => {
                    puilength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEnclosure_Impl::AsyncDownload(this).into()
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEnclosure_Impl::CancelAsyncDownload(this).into()
        }
        unsafe extern "system" fn DownloadStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedEnclosure_Impl::DownloadStatus(this) {
                Ok(ok__) => {
                    pfds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadError<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedEnclosure_Impl::LastDownloadError(this) {
                Ok(ok__) => {
                    pfde.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedEnclosure_Impl::LocalPath(this) {
                Ok(ok__) => {
                    ppszpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEnclosure_Impl::Parent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedEnclosure_Impl::DownloadUrl(this) {
                Ok(ok__) => {
                    ppszurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadMimeType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszmimetype: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedEnclosure_Impl::DownloadMimeType(this) {
                Ok(ok__) => {
                    ppszmimetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEnclosure_Impl::RemoveFile(this).into()
        }
        unsafe extern "system" fn SetFile<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdownloadurl: windows_core::PCWSTR, pszdownloadfilepath: windows_core::PCWSTR, pszdownloadmimetype: windows_core::PCWSTR, pszenclosurefilename: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEnclosure_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEnclosure_Impl::SetFile(this, core::mem::transmute(&pszdownloadurl), core::mem::transmute(&pszdownloadfilepath), core::mem::transmute(&pszdownloadmimetype), core::mem::transmute(&pszenclosurefilename)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Url: Url::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, OFFSET>,
            LocalPath: LocalPath::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, OFFSET>,
            DownloadMimeType: DownloadMimeType::<Identity, OFFSET>,
            RemoveFile: RemoveFile::<Identity, OFFSET>,
            SetFile: SetFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXFeedEnclosure as windows_core::Interface>::IID
    }
}
pub trait IXFeedEvents_Impl: Sized {
    fn Error(&self) -> windows_core::Result<()>;
    fn FeedDeleted(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedRenamed(&self, pszpath: &windows_core::PCWSTR, pszoldpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedUrlChanged(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedMoved(&self, pszpath: &windows_core::PCWSTR, pszoldpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedDownloading(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedDownloadCompleted(&self, pszpath: &windows_core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> windows_core::Result<()>;
    fn FeedItemCountChanged(&self, pszpath: &windows_core::PCWSTR, feicfflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXFeedEvents {}
impl IXFeedEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXFeedEvents_Vtbl
    where
        Identity: IXFeedEvents_Impl,
    {
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEvents_Impl::Error(this).into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEvents_Impl::FeedDeleted(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pszoldpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEvents_Impl::FeedRenamed(this, core::mem::transmute(&pszpath), core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEvents_Impl::FeedUrlChanged(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedMoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pszoldpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEvents_Impl::FeedMoved(this, core::mem::transmute(&pszpath), core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEvents_Impl::FeedDownloading(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> windows_core::HRESULT
        where
            Identity: IXFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEvents_Impl::FeedDownloadCompleted(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&fde)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, feicfflags: i32) -> windows_core::HRESULT
        where
            Identity: IXFeedEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedEvents_Impl::FeedItemCountChanged(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&feicfflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Error: Error::<Identity, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, OFFSET>,
            FeedMoved: FeedMoved::<Identity, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXFeedEvents as windows_core::Interface>::IID
    }
}
pub trait IXFeedFolder_Impl: Sized {
    fn Feeds(&self) -> windows_core::Result<IXFeedsEnum>;
    fn Subfolders(&self) -> windows_core::Result<IXFeedsEnum>;
    fn CreateFeed(&self, pszname: &windows_core::PCWSTR, pszurl: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateSubfolder(&self, pszname: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ExistsFeed(&self, pszname: &windows_core::PCWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ExistsSubfolder(&self, pszname: &windows_core::PCWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetFeed(&self, pszname: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSubfolder(&self, pszname: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Rename(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Move(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Parent(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsRoot(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetWatcher(&self, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn TotalUnreadItemCount(&self) -> windows_core::Result<u32>;
    fn TotalItemCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IXFeedFolder {}
impl IXFeedFolder_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXFeedFolder_Vtbl
    where
        Identity: IXFeedFolder_Impl,
    {
        unsafe extern "system" fn Feeds<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedFolder_Impl::Feeds(this) {
                Ok(ok__) => {
                    ppfe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subfolders<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedFolder_Impl::Subfolders(this) {
                Ok(ok__) => {
                    ppfe.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pszurl: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::CreateFeed(this, core::mem::transmute(&pszname), core::mem::transmute(&pszurl), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateSubfolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::CreateSubfolder(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ExistsFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::ExistsFeed(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&pbfeedexists)).into()
        }
        unsafe extern "system" fn ExistsSubfolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::ExistsSubfolder(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&pbsubfolderexists)).into()
        }
        unsafe extern "system" fn GetFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::GetFeed(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetSubfolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::GetSubfolder(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedFolder_Impl::Name(this) {
                Ok(ok__) => {
                    ppszname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rename<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::Rename(this, core::mem::transmute(&pszname)).into()
        }
        unsafe extern "system" fn Path<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedFolder_Impl::Path(this) {
                Ok(ok__) => {
                    ppszpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::Move(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::Parent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsRoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisrootfeedfolder: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedFolder_Impl::IsRoot(this) {
                Ok(ok__) => {
                    pbisrootfeedfolder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatcher<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolder_Impl::GetWatcher(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&mask), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn TotalUnreadItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puitotalunreaditemcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedFolder_Impl::TotalUnreadItemCount(this) {
                Ok(ok__) => {
                    puitotalunreaditemcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalItemCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puitotalitemcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeedFolder_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedFolder_Impl::TotalItemCount(this) {
                Ok(ok__) => {
                    puitotalitemcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Feeds: Feeds::<Identity, OFFSET>,
            Subfolders: Subfolders::<Identity, OFFSET>,
            CreateFeed: CreateFeed::<Identity, OFFSET>,
            CreateSubfolder: CreateSubfolder::<Identity, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, OFFSET>,
            ExistsSubfolder: ExistsSubfolder::<Identity, OFFSET>,
            GetFeed: GetFeed::<Identity, OFFSET>,
            GetSubfolder: GetSubfolder::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Rename: Rename::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            IsRoot: IsRoot::<Identity, OFFSET>,
            GetWatcher: GetWatcher::<Identity, OFFSET>,
            TotalUnreadItemCount: TotalUnreadItemCount::<Identity, OFFSET>,
            TotalItemCount: TotalItemCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXFeedFolder as windows_core::Interface>::IID
    }
}
pub trait IXFeedFolderEvents_Impl: Sized {
    fn Error(&self) -> windows_core::Result<()>;
    fn FolderAdded(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FolderDeleted(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FolderRenamed(&self, pszpath: &windows_core::PCWSTR, pszoldpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FolderMovedFrom(&self, pszpath: &windows_core::PCWSTR, pszoldpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FolderMovedTo(&self, pszpath: &windows_core::PCWSTR, pszoldpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FolderItemCountChanged(&self, pszpath: &windows_core::PCWSTR, feicfflags: i32) -> windows_core::Result<()>;
    fn FeedAdded(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedDeleted(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedRenamed(&self, pszpath: &windows_core::PCWSTR, pszoldpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedUrlChanged(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedMovedFrom(&self, pszpath: &windows_core::PCWSTR, pszoldpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedMovedTo(&self, pszpath: &windows_core::PCWSTR, pszoldpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedDownloading(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FeedDownloadCompleted(&self, pszpath: &windows_core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> windows_core::Result<()>;
    fn FeedItemCountChanged(&self, pszpath: &windows_core::PCWSTR, feicfflags: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXFeedFolderEvents {}
impl IXFeedFolderEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXFeedFolderEvents_Vtbl
    where
        Identity: IXFeedFolderEvents_Impl,
    {
        unsafe extern "system" fn Error<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::Error(this).into()
        }
        unsafe extern "system" fn FolderAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FolderAdded(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FolderDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FolderDeleted(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FolderRenamed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pszoldpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FolderRenamed(this, core::mem::transmute(&pszpath), core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderMovedFrom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pszoldpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FolderMovedFrom(this, core::mem::transmute(&pszpath), core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderMovedTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pszoldpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FolderMovedTo(this, core::mem::transmute(&pszpath), core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FolderItemCountChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, feicfflags: i32) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FolderItemCountChanged(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&feicfflags)).into()
        }
        unsafe extern "system" fn FeedAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FeedAdded(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDeleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FeedDeleted(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedRenamed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pszoldpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FeedRenamed(this, core::mem::transmute(&pszpath), core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FeedUrlChanged(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedMovedFrom<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pszoldpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FeedMovedFrom(this, core::mem::transmute(&pszpath), core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedMovedTo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pszoldpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FeedMovedTo(this, core::mem::transmute(&pszpath), core::mem::transmute(&pszoldpath)).into()
        }
        unsafe extern "system" fn FeedDownloading<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FeedDownloading(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FeedDownloadCompleted(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&fde)).into()
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, feicfflags: i32) -> windows_core::HRESULT
        where
            Identity: IXFeedFolderEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedFolderEvents_Impl::FeedItemCountChanged(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&feicfflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Error: Error::<Identity, OFFSET>,
            FolderAdded: FolderAdded::<Identity, OFFSET>,
            FolderDeleted: FolderDeleted::<Identity, OFFSET>,
            FolderRenamed: FolderRenamed::<Identity, OFFSET>,
            FolderMovedFrom: FolderMovedFrom::<Identity, OFFSET>,
            FolderMovedTo: FolderMovedTo::<Identity, OFFSET>,
            FolderItemCountChanged: FolderItemCountChanged::<Identity, OFFSET>,
            FeedAdded: FeedAdded::<Identity, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, OFFSET>,
            FeedMovedFrom: FeedMovedFrom::<Identity, OFFSET>,
            FeedMovedTo: FeedMovedTo::<Identity, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXFeedFolderEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXFeedItem_Impl: Sized {
    fn Xml(&self, fxif: FEEDS_XML_INCLUDE_FLAGS) -> windows_core::Result<super::super::System::Com::IStream>;
    fn Title(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Link(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Guid(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn PubDate(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Comments(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Author(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Enclosure(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsRead(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsRead(&self, bisread: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn LocalId(&self) -> windows_core::Result<u32>;
    fn Parent(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn DownloadUrl(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn LastDownloadTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Modified(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXFeedItem {}
#[cfg(feature = "Win32_System_Com")]
impl IXFeedItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXFeedItem_Vtbl
    where
        Identity: IXFeedItem_Impl,
    {
        unsafe extern "system" fn Xml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fxif: FEEDS_XML_INCLUDE_FLAGS, pps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::Xml(this, core::mem::transmute_copy(&fxif)) {
                Ok(ok__) => {
                    pps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsztitle: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::Title(this) {
                Ok(ok__) => {
                    ppsztitle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::Link(this) {
                Ok(ok__) => {
                    ppszurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Guid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszguid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::Guid(this) {
                Ok(ok__) => {
                    ppszguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszdescription: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::Description(this) {
                Ok(ok__) => {
                    ppszdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PubDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::PubDate(this) {
                Ok(ok__) => {
                    pstpubdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comments<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::Comments(this) {
                Ok(ok__) => {
                    ppszurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszauthor: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::Author(this) {
                Ok(ok__) => {
                    ppszauthor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enclosure<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedItem_Impl::Enclosure(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsRead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbisread: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::IsRead(this) {
                Ok(ok__) => {
                    pbisread.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bisread: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedItem_Impl::SetIsRead(this, core::mem::transmute_copy(&bisread)).into()
        }
        unsafe extern "system" fn LocalId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::LocalId(this) {
                Ok(ok__) => {
                    puiid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedItem_Impl::Parent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedItem_Impl::Delete(this).into()
        }
        unsafe extern "system" fn DownloadUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::DownloadUrl(this) {
                Ok(ok__) => {
                    ppszurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastDownloadTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::LastDownloadTime(this) {
                Ok(ok__) => {
                    pstlastdownloadtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstmodifiedtime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT
        where
            Identity: IXFeedItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem_Impl::Modified(this) {
                Ok(ok__) => {
                    pstmodifiedtime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Xml: Xml::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            Link: Link::<Identity, OFFSET>,
            Guid: Guid::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            PubDate: PubDate::<Identity, OFFSET>,
            Comments: Comments::<Identity, OFFSET>,
            Author: Author::<Identity, OFFSET>,
            Enclosure: Enclosure::<Identity, OFFSET>,
            IsRead: IsRead::<Identity, OFFSET>,
            SetIsRead: SetIsRead::<Identity, OFFSET>,
            LocalId: LocalId::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXFeedItem as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXFeedItem2_Impl: Sized + IXFeedItem_Impl {
    fn EffectiveId(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXFeedItem2 {}
#[cfg(feature = "Win32_System_Com")]
impl IXFeedItem2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXFeedItem2_Vtbl
    where
        Identity: IXFeedItem2_Impl,
    {
        unsafe extern "system" fn EffectiveId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puieffectiveid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeedItem2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedItem2_Impl::EffectiveId(this) {
                Ok(ok__) => {
                    puieffectiveid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IXFeedItem_Vtbl::new::<Identity, OFFSET>(), EffectiveId: EffectiveId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXFeedItem2 as windows_core::Interface>::IID || iid == &<IXFeedItem as windows_core::Interface>::IID
    }
}
pub trait IXFeedsEnum_Impl: Sized {
    fn Count(&self) -> windows_core::Result<u32>;
    fn Item(&self, uiindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXFeedsEnum {}
impl IXFeedsEnum_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXFeedsEnum_Vtbl
    where
        Identity: IXFeedsEnum_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puicount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeedsEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedsEnum_Impl::Count(this) {
                Ok(ok__) => {
                    puicount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiindex: u32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedsEnum_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedsEnum_Impl::Item(this, core::mem::transmute_copy(&uiindex), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Count: Count::<Identity, OFFSET>, Item: Item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXFeedsEnum as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXFeedsManager_Impl: Sized {
    fn RootFolder(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsSubscribed(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn ExistsFeed(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFeed(&self, pszpath: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetFeedByUrl(&self, pszurl: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ExistsFolder(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFolder(&self, pszpath: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DeleteFeed(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeleteFolder(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn BackgroundSync(&self, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> windows_core::Result<()>;
    fn BackgroundSyncStatus(&self) -> windows_core::Result<FEEDS_BACKGROUNDSYNC_STATUS>;
    fn DefaultInterval(&self) -> windows_core::Result<u32>;
    fn SetDefaultInterval(&self, uiinterval: u32) -> windows_core::Result<()>;
    fn AsyncSyncAll(&self) -> windows_core::Result<()>;
    fn Normalize(&self, pstreamin: Option<&super::super::System::Com::IStream>) -> windows_core::Result<super::super::System::Com::IStream>;
    fn ItemCountLimit(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IXFeedsManager {}
#[cfg(feature = "Win32_System_Com")]
impl IXFeedsManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXFeedsManager_Vtbl
    where
        Identity: IXFeedsManager_Impl,
    {
        unsafe extern "system" fn RootFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedsManager_Impl::RootFolder(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn IsSubscribed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pbsubscribed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedsManager_Impl::IsSubscribed(this, core::mem::transmute(&pszurl)) {
                Ok(ok__) => {
                    pbsubscribed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExistsFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pbfeedexists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedsManager_Impl::ExistsFeed(this, core::mem::transmute(&pszpath)) {
                Ok(ok__) => {
                    pbfeedexists.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedsManager_Impl::GetFeed(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetFeedByUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedsManager_Impl::GetFeedByUrl(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ExistsFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, pbfolderexists: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedsManager_Impl::ExistsFolder(this, core::mem::transmute(&pszpath)) {
                Ok(ok__) => {
                    pbfolderexists.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedsManager_Impl::GetFolder(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn DeleteFeed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedsManager_Impl::DeleteFeed(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn DeleteFolder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedsManager_Impl::DeleteFolder(this, core::mem::transmute(&pszpath)).into()
        }
        unsafe extern "system" fn BackgroundSync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedsManager_Impl::BackgroundSync(this, core::mem::transmute_copy(&fbsa)).into()
        }
        unsafe extern "system" fn BackgroundSyncStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfbss: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedsManager_Impl::BackgroundSyncStatus(this) {
                Ok(ok__) => {
                    pfbss.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiinterval: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedsManager_Impl::DefaultInterval(this) {
                Ok(ok__) => {
                    puiinterval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultInterval<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, uiinterval: u32) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedsManager_Impl::SetDefaultInterval(this, core::mem::transmute_copy(&uiinterval)).into()
        }
        unsafe extern "system" fn AsyncSyncAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXFeedsManager_Impl::AsyncSyncAll(this).into()
        }
        unsafe extern "system" fn Normalize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstreamin: *mut core::ffi::c_void, ppstreamout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedsManager_Impl::Normalize(this, windows_core::from_raw_borrowed(&pstreamin)) {
                Ok(ok__) => {
                    ppstreamout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCountLimit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiitemcountlimit: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXFeedsManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXFeedsManager_Impl::ItemCountLimit(this) {
                Ok(ok__) => {
                    puiitemcountlimit.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RootFolder: RootFolder::<Identity, OFFSET>,
            IsSubscribed: IsSubscribed::<Identity, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, OFFSET>,
            GetFeed: GetFeed::<Identity, OFFSET>,
            GetFeedByUrl: GetFeedByUrl::<Identity, OFFSET>,
            ExistsFolder: ExistsFolder::<Identity, OFFSET>,
            GetFolder: GetFolder::<Identity, OFFSET>,
            DeleteFeed: DeleteFeed::<Identity, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, OFFSET>,
            BackgroundSync: BackgroundSync::<Identity, OFFSET>,
            BackgroundSyncStatus: BackgroundSyncStatus::<Identity, OFFSET>,
            DefaultInterval: DefaultInterval::<Identity, OFFSET>,
            SetDefaultInterval: SetDefaultInterval::<Identity, OFFSET>,
            AsyncSyncAll: AsyncSyncAll::<Identity, OFFSET>,
            Normalize: Normalize::<Identity, OFFSET>,
            ItemCountLimit: ItemCountLimit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXFeedsManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait _WMPOCXEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for _WMPOCXEvents {}
#[cfg(feature = "Win32_System_Com")]
impl _WMPOCXEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> _WMPOCXEvents_Vtbl
    where
        Identity: _WMPOCXEvents_Impl,
    {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_WMPOCXEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
