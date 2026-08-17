pub trait DataSource_Impl: Sized {
    fn getDataMember(&self, bstrdm: *const u16, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn getDataMemberName(&self, lindex: i32) -> windows_core::Result<*mut u16>;
    fn getDataMemberCount(&self) -> windows_core::Result<i32>;
    fn addDataSourceListener(&self, pdsl: Option<&DataSourceListener>) -> windows_core::Result<()>;
    fn removeDataSourceListener(&self, pdsl: Option<&DataSourceListener>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for DataSource {}
impl DataSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> DataSource_Vtbl
    where
        Identity: DataSource_Impl,
    {
        unsafe extern "system" fn getDataMember<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdm: *const u16, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: DataSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match DataSource_Impl::getDataMember(this, core::mem::transmute_copy(&bstrdm), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDataMemberName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, pbstrdm: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: DataSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match DataSource_Impl::getDataMemberName(this, core::mem::transmute_copy(&lindex)) {
                Ok(ok__) => {
                    pbstrdm.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDataMemberCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: DataSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match DataSource_Impl::getDataMemberCount(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addDataSourceListener<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsl: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: DataSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DataSource_Impl::addDataSourceListener(this, windows_core::from_raw_borrowed(&pdsl)).into()
        }
        unsafe extern "system" fn removeDataSourceListener<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdsl: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: DataSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DataSource_Impl::removeDataSourceListener(this, windows_core::from_raw_borrowed(&pdsl)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getDataMember: getDataMember::<Identity, OFFSET>,
            getDataMemberName: getDataMemberName::<Identity, OFFSET>,
            getDataMemberCount: getDataMemberCount::<Identity, OFFSET>,
            addDataSourceListener: addDataSourceListener::<Identity, OFFSET>,
            removeDataSourceListener: removeDataSourceListener::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DataSource as windows_core::Interface>::IID
    }
}
pub trait DataSourceListener_Impl: Sized {
    fn dataMemberChanged(&self, bstrdm: *const u16) -> windows_core::Result<()>;
    fn dataMemberAdded(&self, bstrdm: *const u16) -> windows_core::Result<()>;
    fn dataMemberRemoved(&self, bstrdm: *const u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for DataSourceListener {}
impl DataSourceListener_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> DataSourceListener_Vtbl
    where
        Identity: DataSourceListener_Impl,
    {
        unsafe extern "system" fn dataMemberChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdm: *const u16) -> windows_core::HRESULT
        where
            Identity: DataSourceListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DataSourceListener_Impl::dataMemberChanged(this, core::mem::transmute_copy(&bstrdm)).into()
        }
        unsafe extern "system" fn dataMemberAdded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdm: *const u16) -> windows_core::HRESULT
        where
            Identity: DataSourceListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DataSourceListener_Impl::dataMemberAdded(this, core::mem::transmute_copy(&bstrdm)).into()
        }
        unsafe extern "system" fn dataMemberRemoved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdm: *const u16) -> windows_core::HRESULT
        where
            Identity: DataSourceListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DataSourceListener_Impl::dataMemberRemoved(this, core::mem::transmute_copy(&bstrdm)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            dataMemberChanged: dataMemberChanged::<Identity, OFFSET>,
            dataMemberAdded: dataMemberAdded::<Identity, OFFSET>,
            dataMemberRemoved: dataMemberRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DataSourceListener as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DataSourceObject_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for DataSourceObject {}
#[cfg(feature = "Win32_System_Com")]
impl DataSourceObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> DataSourceObject_Vtbl
    where
        Identity: DataSourceObject_Impl,
    {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DataSourceObject as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessor_Impl: Sized {
    fn AddRefAccessor(&self, haccessor: HACCESSOR, pcrefcount: *mut u32) -> windows_core::Result<()>;
    fn CreateAccessor(&self, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut HACCESSOR, rgstatus: *mut u32) -> windows_core::Result<()>;
    fn GetBindings(&self, haccessor: HACCESSOR, pdwaccessorflags: *mut u32, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> windows_core::Result<()>;
    fn ReleaseAccessor(&self, haccessor: HACCESSOR, pcrefcount: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IAccessor {}
#[cfg(feature = "Win32_System_Com")]
impl IAccessor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAccessor_Vtbl
    where
        Identity: IAccessor_Impl,
    {
        unsafe extern "system" fn AddRefAccessor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, pcrefcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccessor_Impl::AddRefAccessor(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pcrefcount)).into()
        }
        unsafe extern "system" fn CreateAccessor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut HACCESSOR, rgstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccessor_Impl::CreateAccessor(this, core::mem::transmute_copy(&dwaccessorflags), core::mem::transmute_copy(&cbindings), core::mem::transmute_copy(&rgbindings), core::mem::transmute_copy(&cbrowsize), core::mem::transmute_copy(&phaccessor), core::mem::transmute_copy(&rgstatus)).into()
        }
        unsafe extern "system" fn GetBindings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, pdwaccessorflags: *mut u32, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> windows_core::HRESULT
        where
            Identity: IAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccessor_Impl::GetBindings(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdwaccessorflags), core::mem::transmute_copy(&pcbindings), core::mem::transmute_copy(&prgbindings)).into()
        }
        unsafe extern "system" fn ReleaseAccessor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, pcrefcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAccessor_Impl::ReleaseAccessor(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pcrefcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefAccessor: AddRefAccessor::<Identity, OFFSET>,
            CreateAccessor: CreateAccessor::<Identity, OFFSET>,
            GetBindings: GetBindings::<Identity, OFFSET>,
            ReleaseAccessor: ReleaseAccessor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IAlterIndex_Impl: Sized {
    fn AlterIndex(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, pnewindexid: *const super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IAlterIndex {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IAlterIndex_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAlterIndex_Vtbl
    where
        Identity: IAlterIndex_Impl,
    {
        unsafe extern "system" fn AlterIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, pnewindexid: *const super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: IAlterIndex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlterIndex_Impl::AlterIndex(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pindexid), core::mem::transmute_copy(&pnewindexid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AlterIndex: AlterIndex::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAlterIndex as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IAlterTable_Impl: Sized {
    fn AlterColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *const DBCOLUMNDESC) -> windows_core::Result<()>;
    fn AlterTable(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pnewtableid: *const super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IAlterTable {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IAlterTable_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IAlterTable_Vtbl
    where
        Identity: IAlterTable_Impl,
    {
        unsafe extern "system" fn AlterColumn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *const DBCOLUMNDESC) -> windows_core::HRESULT
        where
            Identity: IAlterTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlterTable_Impl::AlterColumn(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pcolumnid), core::mem::transmute_copy(&dwcolumndescflags), core::mem::transmute_copy(&pcolumndesc)).into()
        }
        unsafe extern "system" fn AlterTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pnewtableid: *const super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: IAlterTable_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlterTable_Impl::AlterTable(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pnewtableid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AlterColumn: AlterColumn::<Identity, OFFSET>,
            AlterTable: AlterTable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAlterTable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBindResource_Impl: Sized {
    fn Bind(&self, punkouter: Option<&windows_core::IUnknown>, pwszurl: &windows_core::PCWSTR, dwbindurlflags: u32, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, pauthenticate: Option<&super::Com::IAuthenticate>, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IBindResource {}
#[cfg(feature = "Win32_System_Com")]
impl IBindResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBindResource_Vtbl
    where
        Identity: IBindResource_Impl,
    {
        unsafe extern "system" fn Bind<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwbindurlflags: u32, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, pauthenticate: *mut core::ffi::c_void, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IBindResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBindResource_Impl::Bind(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwbindurlflags), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&pauthenticate), core::mem::transmute_copy(&pimplsession), core::mem::transmute_copy(&pdwbindstatus), core::mem::transmute_copy(&ppunk)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Bind: Bind::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindResource as windows_core::Interface>::IID
    }
}
pub trait IChapteredRowset_Impl: Sized {
    fn AddRefChapter(&self, hchapter: usize, pcrefcount: *mut u32) -> windows_core::Result<()>;
    fn ReleaseChapter(&self, hchapter: usize, pcrefcount: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IChapteredRowset {}
impl IChapteredRowset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IChapteredRowset_Vtbl
    where
        Identity: IChapteredRowset_Impl,
    {
        unsafe extern "system" fn AddRefChapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, pcrefcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IChapteredRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChapteredRowset_Impl::AddRefChapter(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&pcrefcount)).into()
        }
        unsafe extern "system" fn ReleaseChapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, pcrefcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IChapteredRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IChapteredRowset_Impl::ReleaseChapter(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&pcrefcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefChapter: AddRefChapter::<Identity, OFFSET>,
            ReleaseChapter: ReleaseChapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChapteredRowset as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IColumnMapper_Impl: Sized {
    fn GetPropInfoFromName(&self, wcspropname: &windows_core::PCWSTR, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> windows_core::Result<()>;
    fn GetPropInfoFromId(&self, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> windows_core::Result<()>;
    fn EnumPropInfo(&self, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> windows_core::Result<()>;
    fn IsMapUpToDate(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IColumnMapper {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IColumnMapper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IColumnMapper_Vtbl
    where
        Identity: IColumnMapper_Impl,
    {
        unsafe extern "system" fn GetPropInfoFromName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wcspropname: windows_core::PCWSTR, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IColumnMapper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IColumnMapper_Impl::GetPropInfoFromName(this, core::mem::transmute(&wcspropname), core::mem::transmute_copy(&pppropid), core::mem::transmute_copy(&pproptype), core::mem::transmute_copy(&puiwidth)).into()
        }
        unsafe extern "system" fn GetPropInfoFromId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IColumnMapper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IColumnMapper_Impl::GetPropInfoFromId(this, core::mem::transmute_copy(&ppropid), core::mem::transmute_copy(&pwcsname), core::mem::transmute_copy(&pproptype), core::mem::transmute_copy(&puiwidth)).into()
        }
        unsafe extern "system" fn EnumPropInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> windows_core::HRESULT
        where
            Identity: IColumnMapper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IColumnMapper_Impl::EnumPropInfo(this, core::mem::transmute_copy(&ientry), core::mem::transmute_copy(&pwcsname), core::mem::transmute_copy(&pppropid), core::mem::transmute_copy(&pproptype), core::mem::transmute_copy(&puiwidth)).into()
        }
        unsafe extern "system" fn IsMapUpToDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IColumnMapper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IColumnMapper_Impl::IsMapUpToDate(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropInfoFromName: GetPropInfoFromName::<Identity, OFFSET>,
            GetPropInfoFromId: GetPropInfoFromId::<Identity, OFFSET>,
            EnumPropInfo: EnumPropInfo::<Identity, OFFSET>,
            IsMapUpToDate: IsMapUpToDate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnMapper as windows_core::Interface>::IID
    }
}
pub trait IColumnMapperCreator_Impl: Sized {
    fn GetColumnMapper(&self, wcsmachinename: &windows_core::PCWSTR, wcscatalogname: &windows_core::PCWSTR) -> windows_core::Result<IColumnMapper>;
}
impl windows_core::RuntimeName for IColumnMapperCreator {}
impl IColumnMapperCreator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IColumnMapperCreator_Vtbl
    where
        Identity: IColumnMapperCreator_Impl,
    {
        unsafe extern "system" fn GetColumnMapper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wcsmachinename: windows_core::PCWSTR, wcscatalogname: windows_core::PCWSTR, ppcolumnmapper: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IColumnMapperCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IColumnMapperCreator_Impl::GetColumnMapper(this, core::mem::transmute(&wcsmachinename), core::mem::transmute(&wcscatalogname)) {
                Ok(ok__) => {
                    ppcolumnmapper.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetColumnMapper: GetColumnMapper::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnMapperCreator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IColumnsInfo_Impl: Sized {
    fn GetColumnInfo(&self, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> windows_core::Result<()>;
    fn MapColumnIDs(&self, ccolumnids: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgcolumns: *mut usize) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IColumnsInfo {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IColumnsInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IColumnsInfo_Vtbl
    where
        Identity: IColumnsInfo_Impl,
    {
        unsafe extern "system" fn GetColumnInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IColumnsInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IColumnsInfo_Impl::GetColumnInfo(this, core::mem::transmute_copy(&pccolumns), core::mem::transmute_copy(&prginfo), core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        unsafe extern "system" fn MapColumnIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumnids: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgcolumns: *mut usize) -> windows_core::HRESULT
        where
            Identity: IColumnsInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IColumnsInfo_Impl::MapColumnIDs(this, core::mem::transmute_copy(&ccolumnids), core::mem::transmute_copy(&rgcolumnids), core::mem::transmute_copy(&rgcolumns)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColumnInfo: GetColumnInfo::<Identity, OFFSET>,
            MapColumnIDs: MapColumnIDs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnsInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IColumnsInfo2_Impl: Sized + IColumnsInfo_Impl {
    fn GetRestrictedColumnInfo(&self, ccolumnidmasks: usize, rgcolumnidmasks: *const super::super::Storage::IndexServer::DBID, dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IColumnsInfo2 {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IColumnsInfo2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IColumnsInfo2_Vtbl
    where
        Identity: IColumnsInfo2_Impl,
    {
        unsafe extern "system" fn GetRestrictedColumnInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumnidmasks: usize, rgcolumnidmasks: *const super::super::Storage::IndexServer::DBID, dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IColumnsInfo2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IColumnsInfo2_Impl::GetRestrictedColumnInfo(this, core::mem::transmute_copy(&ccolumnidmasks), core::mem::transmute_copy(&rgcolumnidmasks), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pccolumns), core::mem::transmute_copy(&prgcolumnids), core::mem::transmute_copy(&prgcolumninfo), core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        Self { base__: IColumnsInfo_Vtbl::new::<Identity, OFFSET>(), GetRestrictedColumnInfo: GetRestrictedColumnInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnsInfo2 as windows_core::Interface>::IID || iid == &<IColumnsInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IColumnsRowset_Impl: Sized {
    fn GetAvailableColumns(&self, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
    fn GetColumnsRowset(&self, punkouter: Option<&windows_core::IUnknown>, coptcolumns: usize, rgoptcolumns: *const super::super::Storage::IndexServer::DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IColumnsRowset {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IColumnsRowset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IColumnsRowset_Vtbl
    where
        Identity: IColumnsRowset_Impl,
    {
        unsafe extern "system" fn GetAvailableColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: IColumnsRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IColumnsRowset_Impl::GetAvailableColumns(this, core::mem::transmute_copy(&pcoptcolumns), core::mem::transmute_copy(&prgoptcolumns)).into()
        }
        unsafe extern "system" fn GetColumnsRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, coptcolumns: usize, rgoptcolumns: *const super::super::Storage::IndexServer::DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IColumnsRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IColumnsRowset_Impl::GetColumnsRowset(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&coptcolumns), core::mem::transmute_copy(&rgoptcolumns), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&ppcolrowset)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableColumns: GetAvailableColumns::<Identity, OFFSET>,
            GetColumnsRowset: GetColumnsRowset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IColumnsRowset as windows_core::Interface>::IID
    }
}
pub trait ICommand_Impl: Sized {
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Execute(&self, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut isize, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetDBSession(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for ICommand {}
impl ICommand_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommand_Vtbl
    where
        Identity: ICommand_Impl,
    {
        unsafe extern "system" fn Cancel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommand_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommand_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Execute<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut isize, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommand_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommand_Impl::Execute(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&pcrowsaffected), core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn GetDBSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommand_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommand_Impl::GetDBSession(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, OFFSET>,
            Execute: Execute::<Identity, OFFSET>,
            GetDBSession: GetDBSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommand as windows_core::Interface>::IID
    }
}
pub trait ICommandCost_Impl: Sized {
    fn GetAccumulatedCost(&self, pwszrowsetname: &windows_core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> windows_core::Result<()>;
    fn GetCostEstimate(&self, pwszrowsetname: &windows_core::PCWSTR, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> windows_core::Result<()>;
    fn GetCostGoals(&self, pwszrowsetname: &windows_core::PCWSTR, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> windows_core::Result<()>;
    fn GetCostLimits(&self, pwszrowsetname: &windows_core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> windows_core::Result<()>;
    fn SetCostGoals(&self, pwszrowsetname: &windows_core::PCWSTR, ccostgoals: u32, rgcostgoals: *const DBCOST) -> windows_core::Result<()>;
    fn SetCostLimits(&self, pwszrowsetname: &windows_core::PCWSTR, ccostlimits: u32, prgcostlimits: *const DBCOST, dwexecutionflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICommandCost {}
impl ICommandCost_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommandCost_Vtbl
    where
        Identity: ICommandCost_Impl,
    {
        unsafe extern "system" fn GetAccumulatedCost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszrowsetname: windows_core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> windows_core::HRESULT
        where
            Identity: ICommandCost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandCost_Impl::GetAccumulatedCost(this, core::mem::transmute(&pwszrowsetname), core::mem::transmute_copy(&pccostlimits), core::mem::transmute_copy(&prgcostlimits)).into()
        }
        unsafe extern "system" fn GetCostEstimate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszrowsetname: windows_core::PCWSTR, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> windows_core::HRESULT
        where
            Identity: ICommandCost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandCost_Impl::GetCostEstimate(this, core::mem::transmute(&pwszrowsetname), core::mem::transmute_copy(&pccostestimates), core::mem::transmute_copy(&prgcostestimates)).into()
        }
        unsafe extern "system" fn GetCostGoals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszrowsetname: windows_core::PCWSTR, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> windows_core::HRESULT
        where
            Identity: ICommandCost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandCost_Impl::GetCostGoals(this, core::mem::transmute(&pwszrowsetname), core::mem::transmute_copy(&pccostgoals), core::mem::transmute_copy(&prgcostgoals)).into()
        }
        unsafe extern "system" fn GetCostLimits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszrowsetname: windows_core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> windows_core::HRESULT
        where
            Identity: ICommandCost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandCost_Impl::GetCostLimits(this, core::mem::transmute(&pwszrowsetname), core::mem::transmute_copy(&pccostlimits), core::mem::transmute_copy(&prgcostlimits)).into()
        }
        unsafe extern "system" fn SetCostGoals<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszrowsetname: windows_core::PCWSTR, ccostgoals: u32, rgcostgoals: *const DBCOST) -> windows_core::HRESULT
        where
            Identity: ICommandCost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandCost_Impl::SetCostGoals(this, core::mem::transmute(&pwszrowsetname), core::mem::transmute_copy(&ccostgoals), core::mem::transmute_copy(&rgcostgoals)).into()
        }
        unsafe extern "system" fn SetCostLimits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszrowsetname: windows_core::PCWSTR, ccostlimits: u32, prgcostlimits: *const DBCOST, dwexecutionflags: u32) -> windows_core::HRESULT
        where
            Identity: ICommandCost_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandCost_Impl::SetCostLimits(this, core::mem::transmute(&pwszrowsetname), core::mem::transmute_copy(&ccostlimits), core::mem::transmute_copy(&prgcostlimits), core::mem::transmute_copy(&dwexecutionflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAccumulatedCost: GetAccumulatedCost::<Identity, OFFSET>,
            GetCostEstimate: GetCostEstimate::<Identity, OFFSET>,
            GetCostGoals: GetCostGoals::<Identity, OFFSET>,
            GetCostLimits: GetCostLimits::<Identity, OFFSET>,
            SetCostGoals: SetCostGoals::<Identity, OFFSET>,
            SetCostLimits: SetCostLimits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandCost as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait ICommandPersist_Impl: Sized {
    fn DeleteCommand(&self, pcommandid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
    fn GetCurrentCommand(&self) -> windows_core::Result<*mut super::super::Storage::IndexServer::DBID>;
    fn LoadCommand(&self, pcommandid: *const super::super::Storage::IndexServer::DBID, dwflags: u32) -> windows_core::Result<()>;
    fn SaveCommand(&self, pcommandid: *const super::super::Storage::IndexServer::DBID, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for ICommandPersist {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ICommandPersist_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommandPersist_Vtbl
    where
        Identity: ICommandPersist_Impl,
    {
        unsafe extern "system" fn DeleteCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommandid: *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: ICommandPersist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandPersist_Impl::DeleteCommand(this, core::mem::transmute_copy(&pcommandid)).into()
        }
        unsafe extern "system" fn GetCurrentCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcommandid: *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: ICommandPersist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICommandPersist_Impl::GetCurrentCommand(this) {
                Ok(ok__) => {
                    ppcommandid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommandid: *const super::super::Storage::IndexServer::DBID, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ICommandPersist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandPersist_Impl::LoadCommand(this, core::mem::transmute_copy(&pcommandid), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SaveCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcommandid: *const super::super::Storage::IndexServer::DBID, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: ICommandPersist_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandPersist_Impl::SaveCommand(this, core::mem::transmute_copy(&pcommandid), core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeleteCommand: DeleteCommand::<Identity, OFFSET>,
            GetCurrentCommand: GetCurrentCommand::<Identity, OFFSET>,
            LoadCommand: LoadCommand::<Identity, OFFSET>,
            SaveCommand: SaveCommand::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandPersist as windows_core::Interface>::IID
    }
}
pub trait ICommandPrepare_Impl: Sized {
    fn Prepare(&self, cexpectedruns: u32) -> windows_core::Result<()>;
    fn Unprepare(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICommandPrepare {}
impl ICommandPrepare_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommandPrepare_Vtbl
    where
        Identity: ICommandPrepare_Impl,
    {
        unsafe extern "system" fn Prepare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cexpectedruns: u32) -> windows_core::HRESULT
        where
            Identity: ICommandPrepare_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandPrepare_Impl::Prepare(this, core::mem::transmute_copy(&cexpectedruns)).into()
        }
        unsafe extern "system" fn Unprepare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommandPrepare_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandPrepare_Impl::Unprepare(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Prepare: Prepare::<Identity, OFFSET>, Unprepare: Unprepare::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandPrepare as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait ICommandProperties_Impl: Sized {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
    fn SetProperties(&self, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for ICommandProperties {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ICommandProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommandProperties_Vtbl
    where
        Identity: ICommandProperties_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: ICommandProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandProperties_Impl::GetProperties(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn SetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> windows_core::HRESULT
        where
            Identity: ICommandProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandProperties_Impl::SetProperties(this, core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            SetProperties: SetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandProperties as windows_core::Interface>::IID
    }
}
pub trait ICommandStream_Impl: Sized {
    fn GetCommandStream(&self, piid: *mut windows_core::GUID, pguiddialect: *mut windows_core::GUID, ppcommandstream: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetCommandStream(&self, riid: *const windows_core::GUID, rguiddialect: *const windows_core::GUID, pcommandstream: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICommandStream {}
impl ICommandStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommandStream_Vtbl
    where
        Identity: ICommandStream_Impl,
    {
        unsafe extern "system" fn GetCommandStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID, pguiddialect: *mut windows_core::GUID, ppcommandstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommandStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandStream_Impl::GetCommandStream(this, core::mem::transmute_copy(&piid), core::mem::transmute_copy(&pguiddialect), core::mem::transmute_copy(&ppcommandstream)).into()
        }
        unsafe extern "system" fn SetCommandStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, rguiddialect: *const windows_core::GUID, pcommandstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommandStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandStream_Impl::SetCommandStream(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&rguiddialect), windows_core::from_raw_borrowed(&pcommandstream)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCommandStream: GetCommandStream::<Identity, OFFSET>,
            SetCommandStream: SetCommandStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandStream as windows_core::Interface>::IID
    }
}
pub trait ICommandText_Impl: Sized + ICommand_Impl {
    fn GetCommandText(&self, pguiddialect: *mut windows_core::GUID, ppwszcommand: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetCommandText(&self, rguiddialect: *const windows_core::GUID, pwszcommand: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICommandText {}
impl ICommandText_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommandText_Vtbl
    where
        Identity: ICommandText_Impl,
    {
        unsafe extern "system" fn GetCommandText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguiddialect: *mut windows_core::GUID, ppwszcommand: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICommandText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandText_Impl::GetCommandText(this, core::mem::transmute_copy(&pguiddialect), core::mem::transmute_copy(&ppwszcommand)).into()
        }
        unsafe extern "system" fn SetCommandText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguiddialect: *const windows_core::GUID, pwszcommand: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ICommandText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandText_Impl::SetCommandText(this, core::mem::transmute_copy(&rguiddialect), core::mem::transmute(&pwszcommand)).into()
        }
        Self {
            base__: ICommand_Vtbl::new::<Identity, OFFSET>(),
            GetCommandText: GetCommandText::<Identity, OFFSET>,
            SetCommandText: SetCommandText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandText as windows_core::Interface>::IID || iid == &<ICommand as windows_core::Interface>::IID
    }
}
pub trait ICommandValidate_Impl: Sized {
    fn ValidateCompletely(&self) -> windows_core::Result<()>;
    fn ValidateSyntax(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICommandValidate {}
impl ICommandValidate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommandValidate_Vtbl
    where
        Identity: ICommandValidate_Impl,
    {
        unsafe extern "system" fn ValidateCompletely<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommandValidate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandValidate_Impl::ValidateCompletely(this).into()
        }
        unsafe extern "system" fn ValidateSyntax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICommandValidate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandValidate_Impl::ValidateSyntax(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ValidateCompletely: ValidateCompletely::<Identity, OFFSET>,
            ValidateSyntax: ValidateSyntax::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandValidate as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICommandWithParameters_Impl: Sized {
    fn GetParameterInfo(&self, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> windows_core::Result<()>;
    fn MapParameterNames(&self, cparamnames: usize, rgparamnames: *const windows_core::PCWSTR, rgparamordinals: *mut isize) -> windows_core::Result<()>;
    fn SetParameterInfo(&self, cparams: usize, rgparamordinals: *const usize, rgparambindinfo: *const DBPARAMBINDINFO) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICommandWithParameters {}
#[cfg(feature = "Win32_System_Com")]
impl ICommandWithParameters_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICommandWithParameters_Vtbl
    where
        Identity: ICommandWithParameters_Impl,
    {
        unsafe extern "system" fn GetParameterInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: ICommandWithParameters_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandWithParameters_Impl::GetParameterInfo(this, core::mem::transmute_copy(&pcparams), core::mem::transmute_copy(&prgparaminfo), core::mem::transmute_copy(&ppnamesbuffer)).into()
        }
        unsafe extern "system" fn MapParameterNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cparamnames: usize, rgparamnames: *const windows_core::PCWSTR, rgparamordinals: *mut isize) -> windows_core::HRESULT
        where
            Identity: ICommandWithParameters_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandWithParameters_Impl::MapParameterNames(this, core::mem::transmute_copy(&cparamnames), core::mem::transmute_copy(&rgparamnames), core::mem::transmute_copy(&rgparamordinals)).into()
        }
        unsafe extern "system" fn SetParameterInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cparams: usize, rgparamordinals: *const usize, rgparambindinfo: *const DBPARAMBINDINFO) -> windows_core::HRESULT
        where
            Identity: ICommandWithParameters_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICommandWithParameters_Impl::SetParameterInfo(this, core::mem::transmute_copy(&cparams), core::mem::transmute_copy(&rgparamordinals), core::mem::transmute_copy(&rgparambindinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameterInfo: GetParameterInfo::<Identity, OFFSET>,
            MapParameterNames: MapParameterNames::<Identity, OFFSET>,
            SetParameterInfo: SetParameterInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommandWithParameters as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
pub trait ICondition_Impl: Sized + super::Com::IPersistStream_Impl {
    fn GetConditionType(&self) -> windows_core::Result<Common::CONDITION_TYPE>;
    fn GetSubConditions(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetComparisonInfo(&self, ppszpropertyname: *mut windows_core::PWSTR, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetValueType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetValueNormalization(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetInputTerms(&self, pppropertyterm: *mut Option<IRichChunk>, ppoperationterm: *mut Option<IRichChunk>, ppvalueterm: *mut Option<IRichChunk>) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<ICondition>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
impl windows_core::RuntimeName for ICondition {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
impl ICondition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICondition_Vtbl
    where
        Identity: ICondition_Impl,
    {
        unsafe extern "system" fn GetConditionType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnodetype: *mut Common::CONDITION_TYPE) -> windows_core::HRESULT
        where
            Identity: ICondition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICondition_Impl::GetConditionType(this) {
                Ok(ok__) => {
                    pnodetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubConditions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICondition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICondition_Impl::GetSubConditions(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetComparisonInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpropertyname: *mut windows_core::PWSTR, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: ICondition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICondition_Impl::GetComparisonInfo(this, core::mem::transmute_copy(&ppszpropertyname), core::mem::transmute_copy(&pcop), core::mem::transmute_copy(&ppropvar)).into()
        }
        unsafe extern "system" fn GetValueType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszvaluetypename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICondition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICondition_Impl::GetValueType(this) {
                Ok(ok__) => {
                    ppszvaluetypename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValueNormalization<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsznormalization: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICondition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICondition_Impl::GetValueNormalization(this) {
                Ok(ok__) => {
                    ppsznormalization.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputTerms<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertyterm: *mut *mut core::ffi::c_void, ppoperationterm: *mut *mut core::ffi::c_void, ppvalueterm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICondition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICondition_Impl::GetInputTerms(this, core::mem::transmute_copy(&pppropertyterm), core::mem::transmute_copy(&ppoperationterm), core::mem::transmute_copy(&ppvalueterm)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICondition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICondition_Impl::Clone(this) {
                Ok(ok__) => {
                    ppc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IPersistStream_Vtbl::new::<Identity, OFFSET>(),
            GetConditionType: GetConditionType::<Identity, OFFSET>,
            GetSubConditions: GetSubConditions::<Identity, OFFSET>,
            GetComparisonInfo: GetComparisonInfo::<Identity, OFFSET>,
            GetValueType: GetValueType::<Identity, OFFSET>,
            GetValueNormalization: GetValueNormalization::<Identity, OFFSET>,
            GetInputTerms: GetInputTerms::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICondition as windows_core::Interface>::IID || iid == &<super::Com::IPersist as windows_core::Interface>::IID || iid == &<super::Com::IPersistStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ICondition2_Impl: Sized + ICondition_Impl {
    fn GetLocale(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetLeafConditionInfo(&self, ppropkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for ICondition2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ICondition2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICondition2_Vtbl
    where
        Identity: ICondition2_Impl,
    {
        unsafe extern "system" fn GetLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszlocalename: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ICondition2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICondition2_Impl::GetLocale(this) {
                Ok(ok__) => {
                    ppszlocalename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLeafConditionInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: ICondition2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICondition2_Impl::GetLeafConditionInfo(this, core::mem::transmute_copy(&ppropkey), core::mem::transmute_copy(&pcop), core::mem::transmute_copy(&ppropvar)).into()
        }
        Self {
            base__: ICondition_Vtbl::new::<Identity, OFFSET>(),
            GetLocale: GetLocale::<Identity, OFFSET>,
            GetLeafConditionInfo: GetLeafConditionInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICondition2 as windows_core::Interface>::IID || iid == &<super::Com::IPersist as windows_core::Interface>::IID || iid == &<super::Com::IPersistStream as windows_core::Interface>::IID || iid == &<ICondition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
pub trait IConditionFactory_Impl: Sized {
    fn MakeNot(&self, pcsub: Option<&ICondition>, fsimplify: super::super::Foundation::BOOL) -> windows_core::Result<ICondition>;
    fn MakeAndOr(&self, ct: Common::CONDITION_TYPE, peusubs: Option<&super::Com::IEnumUnknown>, fsimplify: super::super::Foundation::BOOL) -> windows_core::Result<ICondition>;
    fn MakeLeaf(&self, pszpropertyname: &windows_core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: &windows_core::PCWSTR, ppropvar: *const windows_core::PROPVARIANT, ppropertynameterm: Option<&IRichChunk>, poperationterm: Option<&IRichChunk>, pvalueterm: Option<&IRichChunk>, fexpand: super::super::Foundation::BOOL) -> windows_core::Result<ICondition>;
    fn Resolve(&self, pc: Option<&ICondition>, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<ICondition>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
impl windows_core::RuntimeName for IConditionFactory {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
impl IConditionFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConditionFactory_Vtbl
    where
        Identity: IConditionFactory_Impl,
    {
        unsafe extern "system" fn MakeNot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcsub: *mut core::ffi::c_void, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConditionFactory_Impl::MakeNot(this, windows_core::from_raw_borrowed(&pcsub), core::mem::transmute_copy(&fsimplify)) {
                Ok(ok__) => {
                    ppcresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeAndOr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: Common::CONDITION_TYPE, peusubs: *mut core::ffi::c_void, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConditionFactory_Impl::MakeAndOr(this, core::mem::transmute_copy(&ct), windows_core::from_raw_borrowed(&peusubs), core::mem::transmute_copy(&fsimplify)) {
                Ok(ok__) => {
                    ppcresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeLeaf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: windows_core::PCWSTR, ppropvar: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, ppropertynameterm: *mut core::ffi::c_void, poperationterm: *mut core::ffi::c_void, pvalueterm: *mut core::ffi::c_void, fexpand: super::super::Foundation::BOOL, ppcresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConditionFactory_Impl::MakeLeaf(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&cop), core::mem::transmute(&pszvaluetype), core::mem::transmute_copy(&ppropvar), windows_core::from_raw_borrowed(&ppropertynameterm), windows_core::from_raw_borrowed(&poperationterm), windows_core::from_raw_borrowed(&pvalueterm), core::mem::transmute_copy(&fexpand)) {
                Ok(ok__) => {
                    ppcresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resolve<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pc: *mut core::ffi::c_void, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, ppcresolved: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConditionFactory_Impl::Resolve(this, windows_core::from_raw_borrowed(&pc), core::mem::transmute_copy(&sqro), core::mem::transmute_copy(&pstreferencetime)) {
                Ok(ok__) => {
                    ppcresolved.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MakeNot: MakeNot::<Identity, OFFSET>,
            MakeAndOr: MakeAndOr::<Identity, OFFSET>,
            MakeLeaf: MakeLeaf::<Identity, OFFSET>,
            Resolve: Resolve::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConditionFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IConditionFactory2_Impl: Sized + IConditionFactory_Impl {
    fn CreateTrueFalse(&self, fval: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateNegation(&self, pcsub: Option<&ICondition>, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateCompoundFromObjectArray(&self, ct: Common::CONDITION_TYPE, poasubs: Option<&super::super::UI::Shell::Common::IObjectArray>, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateCompoundFromArray(&self, ct: Common::CONDITION_TYPE, ppcondsubs: *const Option<ICondition>, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateStringLeaf(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: &windows_core::PCWSTR, pszlocalename: &windows_core::PCWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateIntegerLeaf(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateBooleanLeaf(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateLeaf(&self, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const windows_core::PROPVARIANT, pszsemantictype: &windows_core::PCWSTR, pszlocalename: &windows_core::PCWSTR, ppropertynameterm: Option<&IRichChunk>, poperationterm: Option<&IRichChunk>, pvalueterm: Option<&IRichChunk>, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ResolveCondition(&self, pc: Option<&ICondition>, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IConditionFactory2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IConditionFactory2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConditionFactory2_Vtbl
    where
        Identity: IConditionFactory2_Impl,
    {
        unsafe extern "system" fn CreateTrueFalse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fval: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionFactory2_Impl::CreateTrueFalse(this, core::mem::transmute_copy(&fval), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateNegation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcsub: *mut core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionFactory2_Impl::CreateNegation(this, windows_core::from_raw_borrowed(&pcsub), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateCompoundFromObjectArray<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: Common::CONDITION_TYPE, poasubs: *mut core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionFactory2_Impl::CreateCompoundFromObjectArray(this, core::mem::transmute_copy(&ct), windows_core::from_raw_borrowed(&poasubs), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateCompoundFromArray<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: Common::CONDITION_TYPE, ppcondsubs: *const *mut core::ffi::c_void, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionFactory2_Impl::CreateCompoundFromArray(this, core::mem::transmute_copy(&ct), core::mem::transmute_copy(&ppcondsubs), core::mem::transmute_copy(&csubs), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateStringLeaf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: windows_core::PCWSTR, pszlocalename: windows_core::PCWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionFactory2_Impl::CreateStringLeaf(this, core::mem::transmute_copy(&propkey), core::mem::transmute_copy(&cop), core::mem::transmute(&pszvalue), core::mem::transmute(&pszlocalename), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateIntegerLeaf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionFactory2_Impl::CreateIntegerLeaf(this, core::mem::transmute_copy(&propkey), core::mem::transmute_copy(&cop), core::mem::transmute_copy(&lvalue), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateBooleanLeaf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionFactory2_Impl::CreateBooleanLeaf(this, core::mem::transmute_copy(&propkey), core::mem::transmute_copy(&cop), core::mem::transmute_copy(&fvalue), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateLeaf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, pszsemantictype: windows_core::PCWSTR, pszlocalename: windows_core::PCWSTR, ppropertynameterm: *mut core::ffi::c_void, poperationterm: *mut core::ffi::c_void, pvalueterm: *mut core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionFactory2_Impl::CreateLeaf(this, core::mem::transmute_copy(&propkey), core::mem::transmute_copy(&cop), core::mem::transmute_copy(&propvar), core::mem::transmute(&pszsemantictype), core::mem::transmute(&pszlocalename), windows_core::from_raw_borrowed(&ppropertynameterm), windows_core::from_raw_borrowed(&poperationterm), windows_core::from_raw_borrowed(&pvalueterm), core::mem::transmute_copy(&cco), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ResolveCondition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pc: *mut core::ffi::c_void, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionFactory2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionFactory2_Impl::ResolveCondition(this, windows_core::from_raw_borrowed(&pc), core::mem::transmute_copy(&sqro), core::mem::transmute_copy(&pstreferencetime), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: IConditionFactory_Vtbl::new::<Identity, OFFSET>(),
            CreateTrueFalse: CreateTrueFalse::<Identity, OFFSET>,
            CreateNegation: CreateNegation::<Identity, OFFSET>,
            CreateCompoundFromObjectArray: CreateCompoundFromObjectArray::<Identity, OFFSET>,
            CreateCompoundFromArray: CreateCompoundFromArray::<Identity, OFFSET>,
            CreateStringLeaf: CreateStringLeaf::<Identity, OFFSET>,
            CreateIntegerLeaf: CreateIntegerLeaf::<Identity, OFFSET>,
            CreateBooleanLeaf: CreateBooleanLeaf::<Identity, OFFSET>,
            CreateLeaf: CreateLeaf::<Identity, OFFSET>,
            ResolveCondition: ResolveCondition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConditionFactory2 as windows_core::Interface>::IID || iid == &<IConditionFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
pub trait IConditionGenerator_Impl: Sized {
    fn Initialize(&self, pschemaprovider: Option<&ISchemaProvider>) -> windows_core::Result<()>;
    fn RecognizeNamedEntities(&self, pszinputstring: &windows_core::PCWSTR, lciduserlocale: u32, ptokencollection: Option<&ITokenCollection>, pnamedentities: Option<&INamedEntityCollector>) -> windows_core::Result<()>;
    fn GenerateForLeaf(&self, pconditionfactory: Option<&IConditionFactory>, pszpropertyname: &windows_core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: &windows_core::PCWSTR, pszvalue: &windows_core::PCWSTR, pszvalue2: &windows_core::PCWSTR, ppropertynameterm: Option<&IRichChunk>, poperationterm: Option<&IRichChunk>, pvalueterm: Option<&IRichChunk>, automaticwildcard: super::super::Foundation::BOOL, pnostringquery: *mut super::super::Foundation::BOOL) -> windows_core::Result<ICondition>;
    fn DefaultPhrase(&self, pszvaluetype: &windows_core::PCWSTR, ppropvar: *const windows_core::PROPVARIANT, fuseenglish: super::super::Foundation::BOOL, ppszphrase: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
impl windows_core::RuntimeName for IConditionGenerator {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
impl IConditionGenerator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConditionGenerator_Vtbl
    where
        Identity: IConditionGenerator_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pschemaprovider: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionGenerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionGenerator_Impl::Initialize(this, windows_core::from_raw_borrowed(&pschemaprovider)).into()
        }
        unsafe extern "system" fn RecognizeNamedEntities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinputstring: windows_core::PCWSTR, lciduserlocale: u32, ptokencollection: *mut core::ffi::c_void, pnamedentities: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionGenerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionGenerator_Impl::RecognizeNamedEntities(this, core::mem::transmute(&pszinputstring), core::mem::transmute_copy(&lciduserlocale), windows_core::from_raw_borrowed(&ptokencollection), windows_core::from_raw_borrowed(&pnamedentities)).into()
        }
        unsafe extern "system" fn GenerateForLeaf<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconditionfactory: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: windows_core::PCWSTR, pszvalue: windows_core::PCWSTR, pszvalue2: windows_core::PCWSTR, ppropertynameterm: *mut core::ffi::c_void, poperationterm: *mut core::ffi::c_void, pvalueterm: *mut core::ffi::c_void, automaticwildcard: super::super::Foundation::BOOL, pnostringquery: *mut super::super::Foundation::BOOL, ppqueryexpression: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IConditionGenerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IConditionGenerator_Impl::GenerateForLeaf(this, windows_core::from_raw_borrowed(&pconditionfactory), core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&cop), core::mem::transmute(&pszvaluetype), core::mem::transmute(&pszvalue), core::mem::transmute(&pszvalue2), windows_core::from_raw_borrowed(&ppropertynameterm), windows_core::from_raw_borrowed(&poperationterm), windows_core::from_raw_borrowed(&pvalueterm), core::mem::transmute_copy(&automaticwildcard), core::mem::transmute_copy(&pnostringquery)) {
                Ok(ok__) => {
                    ppqueryexpression.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvaluetype: windows_core::PCWSTR, ppropvar: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, fuseenglish: super::super::Foundation::BOOL, ppszphrase: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IConditionGenerator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConditionGenerator_Impl::DefaultPhrase(this, core::mem::transmute(&pszvaluetype), core::mem::transmute_copy(&ppropvar), core::mem::transmute_copy(&fuseenglish), core::mem::transmute_copy(&ppszphrase)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            RecognizeNamedEntities: RecognizeNamedEntities::<Identity, OFFSET>,
            GenerateForLeaf: GenerateForLeaf::<Identity, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConditionGenerator as windows_core::Interface>::IID
    }
}
pub trait IConvertType_Impl: Sized {
    fn CanConvert(&self, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IConvertType {}
impl IConvertType_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IConvertType_Vtbl
    where
        Identity: IConvertType_Impl,
    {
        unsafe extern "system" fn CanConvert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> windows_core::HRESULT
        where
            Identity: IConvertType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IConvertType_Impl::CanConvert(this, core::mem::transmute_copy(&wfromtype), core::mem::transmute_copy(&wtotype), core::mem::transmute_copy(&dwconvertflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CanConvert: CanConvert::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConvertType as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICreateRow_Impl: Sized {
    fn CreateRow(&self, punkouter: Option<&windows_core::IUnknown>, pwszurl: &windows_core::PCWSTR, dwbindurlflags: u32, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, pauthenticate: Option<&super::Com::IAuthenticate>, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppwsznewurl: *mut windows_core::PWSTR, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ICreateRow {}
#[cfg(feature = "Win32_System_Com")]
impl ICreateRow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ICreateRow_Vtbl
    where
        Identity: ICreateRow_Impl,
    {
        unsafe extern "system" fn CreateRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwbindurlflags: u32, rguid: *const windows_core::GUID, riid: *const windows_core::GUID, pauthenticate: *mut core::ffi::c_void, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppwsznewurl: *mut windows_core::PWSTR, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ICreateRow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICreateRow_Impl::CreateRow(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwbindurlflags), core::mem::transmute_copy(&rguid), core::mem::transmute_copy(&riid), windows_core::from_raw_borrowed(&pauthenticate), core::mem::transmute_copy(&pimplsession), core::mem::transmute_copy(&pdwbindstatus), core::mem::transmute_copy(&ppwsznewurl), core::mem::transmute_copy(&ppunk)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateRow: CreateRow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICreateRow as windows_core::Interface>::IID
    }
}
pub trait IDBAsynchNotify_Impl: Sized {
    fn OnLowResource(&self, dwreserved: usize) -> windows_core::Result<()>;
    fn OnProgress(&self, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnStop(&self, hchapter: usize, eoperation: u32, hrstatus: windows_core::HRESULT, pwszstatustext: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDBAsynchNotify {}
impl IDBAsynchNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBAsynchNotify_Vtbl
    where
        Identity: IDBAsynchNotify_Impl,
    {
        unsafe extern "system" fn OnLowResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: usize) -> windows_core::HRESULT
        where
            Identity: IDBAsynchNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBAsynchNotify_Impl::OnLowResource(this, core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IDBAsynchNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBAsynchNotify_Impl::OnProgress(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&eoperation), core::mem::transmute_copy(&ulprogress), core::mem::transmute_copy(&ulprogressmax), core::mem::transmute_copy(&easynchphase), core::mem::transmute(&pwszstatustext)).into()
        }
        unsafe extern "system" fn OnStop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, eoperation: u32, hrstatus: windows_core::HRESULT, pwszstatustext: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IDBAsynchNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBAsynchNotify_Impl::OnStop(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&eoperation), core::mem::transmute_copy(&hrstatus), core::mem::transmute(&pwszstatustext)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLowResource: OnLowResource::<Identity, OFFSET>,
            OnProgress: OnProgress::<Identity, OFFSET>,
            OnStop: OnStop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBAsynchNotify as windows_core::Interface>::IID
    }
}
pub trait IDBAsynchStatus_Impl: Sized {
    fn Abort(&self, hchapter: usize, eoperation: u32) -> windows_core::Result<()>;
    fn GetStatus(&self, hchapter: usize, eoperation: u32, pulprogress: *mut usize, pulprogressmax: *mut usize, peasynchphase: *mut u32, ppwszstatustext: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDBAsynchStatus {}
impl IDBAsynchStatus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBAsynchStatus_Vtbl
    where
        Identity: IDBAsynchStatus_Impl,
    {
        unsafe extern "system" fn Abort<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, eoperation: u32) -> windows_core::HRESULT
        where
            Identity: IDBAsynchStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBAsynchStatus_Impl::Abort(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&eoperation)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, eoperation: u32, pulprogress: *mut usize, pulprogressmax: *mut usize, peasynchphase: *mut u32, ppwszstatustext: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IDBAsynchStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBAsynchStatus_Impl::GetStatus(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&eoperation), core::mem::transmute_copy(&pulprogress), core::mem::transmute_copy(&pulprogressmax), core::mem::transmute_copy(&peasynchphase), core::mem::transmute_copy(&ppwszstatustext)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Abort: Abort::<Identity, OFFSET>, GetStatus: GetStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBAsynchStatus as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Variant"))]
pub trait IDBBinderProperties_Impl: Sized + IDBProperties_Impl {
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDBBinderProperties {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Variant"))]
impl IDBBinderProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBBinderProperties_Vtbl
    where
        Identity: IDBBinderProperties_Impl,
    {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDBBinderProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBBinderProperties_Impl::Reset(this).into()
        }
        Self { base__: IDBProperties_Vtbl::new::<Identity, OFFSET>(), Reset: Reset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBBinderProperties as windows_core::Interface>::IID || iid == &<IDBProperties as windows_core::Interface>::IID
    }
}
pub trait IDBCreateCommand_Impl: Sized {
    fn CreateCommand(&self, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IDBCreateCommand {}
impl IDBCreateCommand_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBCreateCommand_Vtbl
    where
        Identity: IDBCreateCommand_Impl,
    {
        unsafe extern "system" fn CreateCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppcommand: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDBCreateCommand_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDBCreateCommand_Impl::CreateCommand(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppcommand.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateCommand: CreateCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBCreateCommand as windows_core::Interface>::IID
    }
}
pub trait IDBCreateSession_Impl: Sized {
    fn CreateSession(&self, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IDBCreateSession {}
impl IDBCreateSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBCreateSession_Vtbl
    where
        Identity: IDBCreateSession_Impl,
    {
        unsafe extern "system" fn CreateSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdbsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDBCreateSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDBCreateSession_Impl::CreateSession(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppdbsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateSession: CreateSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBCreateSession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Variant"))]
pub trait IDBDataSourceAdmin_Impl: Sized {
    fn CreateDataSource(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, ppdbsession: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DestroyDataSource(&self) -> windows_core::Result<()>;
    fn GetCreationProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> windows_core::Result<()>;
    fn ModifyDataSource(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDBDataSourceAdmin {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Variant"))]
impl IDBDataSourceAdmin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBDataSourceAdmin_Vtbl
    where
        Identity: IDBDataSourceAdmin_Impl,
    {
        unsafe extern "system" fn CreateDataSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdbsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDBDataSourceAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBDataSourceAdmin_Impl::CreateDataSource(this, core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdbsession)).into()
        }
        unsafe extern "system" fn DestroyDataSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDBDataSourceAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBDataSourceAdmin_Impl::DestroyDataSource(this).into()
        }
        unsafe extern "system" fn GetCreationProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IDBDataSourceAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBDataSourceAdmin_Impl::GetCreationProperties(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertyinfosets), core::mem::transmute_copy(&prgpropertyinfosets), core::mem::transmute_copy(&ppdescbuffer)).into()
        }
        unsafe extern "system" fn ModifyDataSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: IDBDataSourceAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBDataSourceAdmin_Impl::ModifyDataSource(this, core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDataSource: CreateDataSource::<Identity, OFFSET>,
            DestroyDataSource: DestroyDataSource::<Identity, OFFSET>,
            GetCreationProperties: GetCreationProperties::<Identity, OFFSET>,
            ModifyDataSource: ModifyDataSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBDataSourceAdmin as windows_core::Interface>::IID
    }
}
pub trait IDBInfo_Impl: Sized {
    fn GetKeywords(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetLiteralInfo(&self, cliterals: u32, rgliterals: *const u32, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDBInfo {}
impl IDBInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBInfo_Vtbl
    where
        Identity: IDBInfo_Impl,
    {
        unsafe extern "system" fn GetKeywords<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszkeywords: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IDBInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDBInfo_Impl::GetKeywords(this) {
                Ok(ok__) => {
                    ppwszkeywords.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiteralInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cliterals: u32, rgliterals: *const u32, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IDBInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBInfo_Impl::GetLiteralInfo(this, core::mem::transmute_copy(&cliterals), core::mem::transmute_copy(&rgliterals), core::mem::transmute_copy(&pcliteralinfo), core::mem::transmute_copy(&prgliteralinfo), core::mem::transmute_copy(&ppcharbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetKeywords: GetKeywords::<Identity, OFFSET>,
            GetLiteralInfo: GetLiteralInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBInfo as windows_core::Interface>::IID
    }
}
pub trait IDBInitialize_Impl: Sized {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDBInitialize {}
impl IDBInitialize_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBInitialize_Vtbl
    where
        Identity: IDBInitialize_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDBInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBInitialize_Impl::Initialize(this).into()
        }
        unsafe extern "system" fn Uninitialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDBInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBInitialize_Impl::Uninitialize(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBInitialize as windows_core::Interface>::IID
    }
}
pub trait IDBPromptInitialize_Impl: Sized {
    fn PromptDataSource(&self, punkouter: Option<&windows_core::IUnknown>, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, csourcetypefilter: u32, rgsourcetypefilter: *const u32, pwszszzproviderfilter: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppdatasource: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn PromptFileName(&self, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, pwszinitialdirectory: &windows_core::PCWSTR, pwszinitialfile: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IDBPromptInitialize {}
impl IDBPromptInitialize_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBPromptInitialize_Vtbl
    where
        Identity: IDBPromptInitialize_Impl,
    {
        unsafe extern "system" fn PromptDataSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, csourcetypefilter: u32, rgsourcetypefilter: *const u32, pwszszzproviderfilter: windows_core::PCWSTR, riid: *const windows_core::GUID, ppdatasource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDBPromptInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBPromptInitialize_Impl::PromptDataSource(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwpromptoptions), core::mem::transmute_copy(&csourcetypefilter), core::mem::transmute_copy(&rgsourcetypefilter), core::mem::transmute(&pwszszzproviderfilter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdatasource)).into()
        }
        unsafe extern "system" fn PromptFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, pwszinitialdirectory: windows_core::PCWSTR, pwszinitialfile: windows_core::PCWSTR, ppwszselectedfile: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IDBPromptInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDBPromptInitialize_Impl::PromptFileName(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&dwpromptoptions), core::mem::transmute(&pwszinitialdirectory), core::mem::transmute(&pwszinitialfile)) {
                Ok(ok__) => {
                    ppwszselectedfile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PromptDataSource: PromptDataSource::<Identity, OFFSET>,
            PromptFileName: PromptFileName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBPromptInitialize as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Variant"))]
pub trait IDBProperties_Impl: Sized {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
    fn GetPropertyInfo(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> windows_core::Result<()>;
    fn SetProperties(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDBProperties {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Variant"))]
impl IDBProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBProperties_Vtbl
    where
        Identity: IDBProperties_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: IDBProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBProperties_Impl::GetProperties(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IDBProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBProperties_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertyinfosets), core::mem::transmute_copy(&prgpropertyinfosets), core::mem::transmute_copy(&ppdescbuffer)).into()
        }
        unsafe extern "system" fn SetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: IDBProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBProperties_Impl::SetProperties(this, core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            SetProperties: SetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBProperties as windows_core::Interface>::IID
    }
}
pub trait IDBSchemaCommand_Impl: Sized {
    fn GetCommand(&self, punkouter: Option<&windows_core::IUnknown>, rguidschema: *const windows_core::GUID) -> windows_core::Result<ICommand>;
    fn GetSchemas(&self, pcschemas: *mut u32, prgschemas: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDBSchemaCommand {}
impl IDBSchemaCommand_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBSchemaCommand_Vtbl
    where
        Identity: IDBSchemaCommand_Impl,
    {
        unsafe extern "system" fn GetCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, rguidschema: *const windows_core::GUID, ppcommand: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDBSchemaCommand_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDBSchemaCommand_Impl::GetCommand(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&rguidschema)) {
                Ok(ok__) => {
                    ppcommand.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemas<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDBSchemaCommand_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBSchemaCommand_Impl::GetSchemas(this, core::mem::transmute_copy(&pcschemas), core::mem::transmute_copy(&prgschemas)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCommand: GetCommand::<Identity, OFFSET>,
            GetSchemas: GetSchemas::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBSchemaCommand as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IDBSchemaRowset_Impl: Sized {
    fn GetRowset(&self, punkouter: Option<&windows_core::IUnknown>, rguidschema: *const windows_core::GUID, crestrictions: u32, rgrestrictions: *const windows_core::VARIANT, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetSchemas(&self, pcschemas: *mut u32, prgschemas: *mut *mut windows_core::GUID, prgrestrictionsupport: *mut *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IDBSchemaRowset {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IDBSchemaRowset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDBSchemaRowset_Vtbl
    where
        Identity: IDBSchemaRowset_Impl,
    {
        unsafe extern "system" fn GetRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, rguidschema: *const windows_core::GUID, crestrictions: u32, rgrestrictions: *const core::mem::MaybeUninit<windows_core::VARIANT>, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDBSchemaRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBSchemaRowset_Impl::GetRowset(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&rguidschema), core::mem::transmute_copy(&crestrictions), core::mem::transmute_copy(&rgrestrictions), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn GetSchemas<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut windows_core::GUID, prgrestrictionsupport: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: IDBSchemaRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDBSchemaRowset_Impl::GetSchemas(this, core::mem::transmute_copy(&pcschemas), core::mem::transmute_copy(&prgschemas), core::mem::transmute_copy(&prgrestrictionsupport)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRowset: GetRowset::<Identity, OFFSET>,
            GetSchemas: GetSchemas::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDBSchemaRowset as windows_core::Interface>::IID
    }
}
pub trait IDCInfo_Impl: Sized {
    fn GetInfo(&self, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> windows_core::Result<()>;
    fn SetInfo(&self, cinfo: u32, rginfo: *const DCINFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDCInfo {}
impl IDCInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDCInfo_Vtbl
    where
        Identity: IDCInfo_Impl,
    {
        unsafe extern "system" fn GetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> windows_core::HRESULT
        where
            Identity: IDCInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCInfo_Impl::GetInfo(this, core::mem::transmute_copy(&cinfo), core::mem::transmute_copy(&rgeinfotype), core::mem::transmute_copy(&prginfo)).into()
        }
        unsafe extern "system" fn SetInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinfo: u32, rginfo: *const DCINFO) -> windows_core::HRESULT
        where
            Identity: IDCInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDCInfo_Impl::SetInfo(this, core::mem::transmute_copy(&cinfo), core::mem::transmute_copy(&rginfo)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetInfo: GetInfo::<Identity, OFFSET>, SetInfo: SetInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDCInfo as windows_core::Interface>::IID
    }
}
pub trait IDataConvert_Impl: Sized {
    fn DataConvert(&self, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: *mut usize, psrc: *const core::ffi::c_void, pdst: *mut core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: *mut u32, bprecision: u8, bscale: u8, dwflags: u32) -> windows_core::Result<()>;
    fn CanConvert(&self, wsrctype: u16, wdsttype: u16) -> windows_core::Result<()>;
    fn GetConversionSize(&self, wsrctype: u16, wdsttype: u16, pcbsrclength: *const usize, pcbdstlength: *mut usize, psrc: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDataConvert {}
impl IDataConvert_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataConvert_Vtbl
    where
        Identity: IDataConvert_Impl,
    {
        unsafe extern "system" fn DataConvert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: *mut usize, psrc: *const core::ffi::c_void, pdst: *mut core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: *mut u32, bprecision: u8, bscale: u8, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IDataConvert_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataConvert_Impl::DataConvert(this, core::mem::transmute_copy(&wsrctype), core::mem::transmute_copy(&wdsttype), core::mem::transmute_copy(&cbsrclength), core::mem::transmute_copy(&pcbdstlength), core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&cbdstmaxlength), core::mem::transmute_copy(&dbssrcstatus), core::mem::transmute_copy(&pdbsstatus), core::mem::transmute_copy(&bprecision), core::mem::transmute_copy(&bscale), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CanConvert<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsrctype: u16, wdsttype: u16) -> windows_core::HRESULT
        where
            Identity: IDataConvert_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataConvert_Impl::CanConvert(this, core::mem::transmute_copy(&wsrctype), core::mem::transmute_copy(&wdsttype)).into()
        }
        unsafe extern "system" fn GetConversionSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsrctype: u16, wdsttype: u16, pcbsrclength: *const usize, pcbdstlength: *mut usize, psrc: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataConvert_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataConvert_Impl::GetConversionSize(this, core::mem::transmute_copy(&wsrctype), core::mem::transmute_copy(&wdsttype), core::mem::transmute_copy(&pcbsrclength), core::mem::transmute_copy(&pcbdstlength), core::mem::transmute_copy(&psrc)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DataConvert: DataConvert::<Identity, OFFSET>,
            CanConvert: CanConvert::<Identity, OFFSET>,
            GetConversionSize: GetConversionSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataConvert as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataInitialize_Impl: Sized {
    fn GetDataSource(&self, punkouter: Option<&windows_core::IUnknown>, dwclsctx: u32, pwszinitializationstring: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppdatasource: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetInitializationString(&self, pdatasource: Option<&windows_core::IUnknown>, fincludepassword: u8) -> windows_core::Result<windows_core::PWSTR>;
    fn CreateDBInstance(&self, clsidprovider: *const windows_core::GUID, punkouter: Option<&windows_core::IUnknown>, dwclsctx: u32, pwszreserved: &windows_core::PCWSTR, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn CreateDBInstanceEx(&self, clsidprovider: *const windows_core::GUID, punkouter: Option<&windows_core::IUnknown>, dwclsctx: u32, pwszreserved: &windows_core::PCWSTR, pserverinfo: *const super::Com::COSERVERINFO, cmq: u32, rgmqresults: *mut super::Com::MULTI_QI) -> windows_core::Result<()>;
    fn LoadStringFromStorage(&self, pwszfilename: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn WriteStringToStorage(&self, pwszfilename: &windows_core::PCWSTR, pwszinitializationstring: &windows_core::PCWSTR, dwcreationdisposition: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDataInitialize {}
#[cfg(feature = "Win32_System_Com")]
impl IDataInitialize_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataInitialize_Vtbl
    where
        Identity: IDataInitialize_Impl,
    {
        unsafe extern "system" fn GetDataSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, dwclsctx: u32, pwszinitializationstring: windows_core::PCWSTR, riid: *const windows_core::GUID, ppdatasource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataInitialize_Impl::GetDataSource(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&dwclsctx), core::mem::transmute(&pwszinitializationstring), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdatasource)).into()
        }
        unsafe extern "system" fn GetInitializationString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatasource: *mut core::ffi::c_void, fincludepassword: u8, ppwszinitstring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IDataInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataInitialize_Impl::GetInitializationString(this, windows_core::from_raw_borrowed(&pdatasource), core::mem::transmute_copy(&fincludepassword)) {
                Ok(ok__) => {
                    ppwszinitstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDBInstance<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidprovider: *const windows_core::GUID, punkouter: *mut core::ffi::c_void, dwclsctx: u32, pwszreserved: windows_core::PCWSTR, riid: *const windows_core::GUID, ppdatasource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataInitialize_Impl::CreateDBInstance(this, core::mem::transmute_copy(&clsidprovider), windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&dwclsctx), core::mem::transmute(&pwszreserved), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppdatasource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDBInstanceEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidprovider: *const windows_core::GUID, punkouter: *mut core::ffi::c_void, dwclsctx: u32, pwszreserved: windows_core::PCWSTR, pserverinfo: *const super::Com::COSERVERINFO, cmq: u32, rgmqresults: *mut super::Com::MULTI_QI) -> windows_core::HRESULT
        where
            Identity: IDataInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataInitialize_Impl::CreateDBInstanceEx(this, core::mem::transmute_copy(&clsidprovider), windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&dwclsctx), core::mem::transmute(&pwszreserved), core::mem::transmute_copy(&pserverinfo), core::mem::transmute_copy(&cmq), core::mem::transmute_copy(&rgmqresults)).into()
        }
        unsafe extern "system" fn LoadStringFromStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR, ppwszinitializationstring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IDataInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataInitialize_Impl::LoadStringFromStorage(this, core::mem::transmute(&pwszfilename)) {
                Ok(ok__) => {
                    ppwszinitializationstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteStringToStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR, pwszinitializationstring: windows_core::PCWSTR, dwcreationdisposition: u32) -> windows_core::HRESULT
        where
            Identity: IDataInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataInitialize_Impl::WriteStringToStorage(this, core::mem::transmute(&pwszfilename), core::mem::transmute(&pwszinitializationstring), core::mem::transmute_copy(&dwcreationdisposition)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDataSource: GetDataSource::<Identity, OFFSET>,
            GetInitializationString: GetInitializationString::<Identity, OFFSET>,
            CreateDBInstance: CreateDBInstance::<Identity, OFFSET>,
            CreateDBInstanceEx: CreateDBInstanceEx::<Identity, OFFSET>,
            LoadStringFromStorage: LoadStringFromStorage::<Identity, OFFSET>,
            WriteStringToStorage: WriteStringToStorage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataInitialize as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataSourceLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn hWnd(&self) -> windows_core::Result<super::super::Foundation::HWND>;
    fn SethWnd(&self, hwndparent: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn PromptNew(&self) -> windows_core::Result<super::Com::IDispatch>;
    fn PromptEdit(&self, ppadoconnection: *mut Option<super::Com::IDispatch>, pbsuccess: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDataSourceLocator {}
#[cfg(feature = "Win32_System_Com")]
impl IDataSourceLocator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataSourceLocator_Vtbl
    where
        Identity: IDataSourceLocator_Impl,
    {
        unsafe extern "system" fn hWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwndparent: *mut super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IDataSourceLocator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataSourceLocator_Impl::hWnd(this) {
                Ok(ok__) => {
                    phwndparent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SethWnd<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IDataSourceLocator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataSourceLocator_Impl::SethWnd(this, core::mem::transmute_copy(&hwndparent)).into()
        }
        unsafe extern "system" fn PromptNew<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppadoconnection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataSourceLocator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataSourceLocator_Impl::PromptNew(this) {
                Ok(ok__) => {
                    ppadoconnection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptEdit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppadoconnection: *mut *mut core::ffi::c_void, pbsuccess: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT
        where
            Identity: IDataSourceLocator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataSourceLocator_Impl::PromptEdit(this, core::mem::transmute_copy(&ppadoconnection), core::mem::transmute_copy(&pbsuccess)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            hWnd: hWnd::<Identity, OFFSET>,
            SethWnd: SethWnd::<Identity, OFFSET>,
            PromptNew: PromptNew::<Identity, OFFSET>,
            PromptEdit: PromptEdit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataSourceLocator as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IEntity_Impl: Sized {
    fn Name(&self, ppszname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn Base(&self) -> windows_core::Result<IEntity>;
    fn Relationships(&self, riid: *const windows_core::GUID, prelationships: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetRelationship(&self, pszrelationname: &windows_core::PCWSTR) -> windows_core::Result<IRelationship>;
    fn MetaData(&self, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn NamedEntities(&self, riid: *const windows_core::GUID, pnamedentities: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetNamedEntity(&self, pszvalue: &windows_core::PCWSTR) -> windows_core::Result<INamedEntity>;
    fn DefaultPhrase(&self, ppszphrase: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEntity {}
impl IEntity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEntity_Vtbl
    where
        Identity: IEntity_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IEntity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEntity_Impl::Name(this, core::mem::transmute_copy(&ppszname)).into()
        }
        unsafe extern "system" fn Base<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbaseentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEntity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEntity_Impl::Base(this) {
                Ok(ok__) => {
                    pbaseentity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Relationships<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, prelationships: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEntity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEntity_Impl::Relationships(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&prelationships)).into()
        }
        unsafe extern "system" fn GetRelationship<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrelationname: windows_core::PCWSTR, prelationship: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEntity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEntity_Impl::GetRelationship(this, core::mem::transmute(&pszrelationname)) {
                Ok(ok__) => {
                    prelationship.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEntity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEntity_Impl::MetaData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pmetadata)).into()
        }
        unsafe extern "system" fn NamedEntities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pnamedentities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEntity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEntity_Impl::NamedEntities(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pnamedentities)).into()
        }
        unsafe extern "system" fn GetNamedEntity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvalue: windows_core::PCWSTR, ppnamedentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEntity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEntity_Impl::GetNamedEntity(this, core::mem::transmute(&pszvalue)) {
                Ok(ok__) => {
                    ppnamedentity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszphrase: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IEntity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEntity_Impl::DefaultPhrase(this, core::mem::transmute_copy(&ppszphrase)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Base: Base::<Identity, OFFSET>,
            Relationships: Relationships::<Identity, OFFSET>,
            GetRelationship: GetRelationship::<Identity, OFFSET>,
            MetaData: MetaData::<Identity, OFFSET>,
            NamedEntities: NamedEntities::<Identity, OFFSET>,
            GetNamedEntity: GetNamedEntity::<Identity, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEntity as windows_core::Interface>::IID
    }
}
pub trait IEnumItemProperties_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ITEMPROP, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumItemProperties>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumItemProperties {}
impl IEnumItemProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumItemProperties_Vtbl
    where
        Identity: IEnumItemProperties_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut ITEMPROP, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumItemProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumItemProperties_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumItemProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumItemProperties_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumItemProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumItemProperties_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumItemProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumItemProperties_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumItemProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumItemProperties_Impl::GetCount(this) {
                Ok(ok__) => {
                    pncount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumItemProperties as windows_core::Interface>::IID
    }
}
pub trait IEnumSearchRoots_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut Option<ISearchRoot>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSearchRoots>;
}
impl windows_core::RuntimeName for IEnumSearchRoots {}
impl IEnumSearchRoots_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSearchRoots_Vtbl
    where
        Identity: IEnumSearchRoots_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSearchRoots_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSearchRoots_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSearchRoots_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSearchRoots_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSearchRoots_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSearchRoots_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSearchRoots_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSearchRoots_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSearchRoots as windows_core::Interface>::IID
    }
}
pub trait IEnumSearchScopeRules_Impl: Sized {
    fn Next(&self, celt: u32, pprgelt: *mut Option<ISearchScopeRule>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSearchScopeRules>;
}
impl windows_core::RuntimeName for IEnumSearchScopeRules {}
impl IEnumSearchScopeRules_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSearchScopeRules_Vtbl
    where
        Identity: IEnumSearchScopeRules_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pprgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSearchScopeRules_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSearchScopeRules_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pprgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSearchScopeRules_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSearchScopeRules_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSearchScopeRules_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSearchScopeRules_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSearchScopeRules_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSearchScopeRules_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSearchScopeRules as windows_core::Interface>::IID
    }
}
pub trait IEnumSubscription_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSubscription>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IEnumSubscription {}
impl IEnumSubscription_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IEnumSubscription_Vtbl
    where
        Identity: IEnumSubscription_Impl,
    {
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut windows_core::GUID, pceltfetched: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSubscription_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT
        where
            Identity: IEnumSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSubscription_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSubscription_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IEnumSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSubscription_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pncount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IEnumSubscription_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSubscription_Impl::GetCount(this) {
                Ok(ok__) => {
                    pncount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSubscription as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IErrorLookup_Impl: Sized {
    fn GetErrorDescription(&self, hrerror: windows_core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: *mut windows_core::BSTR, pbstrdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetHelpInfo(&self, hrerror: windows_core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut windows_core::BSTR, pdwhelpcontext: *mut u32) -> windows_core::Result<()>;
    fn ReleaseErrors(&self, dwdynamicerrorid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IErrorLookup {}
#[cfg(feature = "Win32_System_Com")]
impl IErrorLookup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IErrorLookup_Vtbl
    where
        Identity: IErrorLookup_Impl,
    {
        unsafe extern "system" fn GetErrorDescription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: IErrorLookup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IErrorLookup_Impl::GetErrorDescription(this, core::mem::transmute_copy(&hrerror), core::mem::transmute_copy(&dwlookupid), core::mem::transmute_copy(&pdispparams), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pbstrsource), core::mem::transmute_copy(&pbstrdescription)).into()
        }
        unsafe extern "system" fn GetHelpInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrerror: windows_core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut core::mem::MaybeUninit<windows_core::BSTR>, pdwhelpcontext: *mut u32) -> windows_core::HRESULT
        where
            Identity: IErrorLookup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IErrorLookup_Impl::GetHelpInfo(this, core::mem::transmute_copy(&hrerror), core::mem::transmute_copy(&dwlookupid), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&pbstrhelpfile), core::mem::transmute_copy(&pdwhelpcontext)).into()
        }
        unsafe extern "system" fn ReleaseErrors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdynamicerrorid: u32) -> windows_core::HRESULT
        where
            Identity: IErrorLookup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IErrorLookup_Impl::ReleaseErrors(this, core::mem::transmute_copy(&dwdynamicerrorid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorDescription: GetErrorDescription::<Identity, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, OFFSET>,
            ReleaseErrors: ReleaseErrors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorLookup as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IErrorRecords_Impl: Sized {
    fn AddErrorRecord(&self, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, punkcustomerror: Option<&windows_core::IUnknown>, dwdynamicerrorid: u32) -> windows_core::Result<()>;
    fn GetBasicErrorInfo(&self, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> windows_core::Result<()>;
    fn GetCustomErrorObject(&self, ulrecordnum: u32, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetErrorInfo(&self, ulrecordnum: u32, lcid: u32) -> windows_core::Result<super::Com::IErrorInfo>;
    fn GetErrorParameters(&self, ulrecordnum: u32) -> windows_core::Result<super::Com::DISPPARAMS>;
    fn GetRecordCount(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IErrorRecords {}
#[cfg(feature = "Win32_System_Com")]
impl IErrorRecords_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IErrorRecords_Vtbl
    where
        Identity: IErrorRecords_Impl,
    {
        unsafe extern "system" fn AddErrorRecord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, punkcustomerror: *mut core::ffi::c_void, dwdynamicerrorid: u32) -> windows_core::HRESULT
        where
            Identity: IErrorRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IErrorRecords_Impl::AddErrorRecord(this, core::mem::transmute_copy(&perrorinfo), core::mem::transmute_copy(&dwlookupid), core::mem::transmute_copy(&pdispparams), windows_core::from_raw_borrowed(&punkcustomerror), core::mem::transmute_copy(&dwdynamicerrorid)).into()
        }
        unsafe extern "system" fn GetBasicErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> windows_core::HRESULT
        where
            Identity: IErrorRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IErrorRecords_Impl::GetBasicErrorInfo(this, core::mem::transmute_copy(&ulrecordnum), core::mem::transmute_copy(&perrorinfo)).into()
        }
        unsafe extern "system" fn GetCustomErrorObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrecordnum: u32, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IErrorRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IErrorRecords_Impl::GetCustomErrorObject(this, core::mem::transmute_copy(&ulrecordnum), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrecordnum: u32, lcid: u32, pperrorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IErrorRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IErrorRecords_Impl::GetErrorInfo(this, core::mem::transmute_copy(&ulrecordnum), core::mem::transmute_copy(&lcid)) {
                Ok(ok__) => {
                    pperrorinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulrecordnum: u32, pdispparams: *mut super::Com::DISPPARAMS) -> windows_core::HRESULT
        where
            Identity: IErrorRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IErrorRecords_Impl::GetErrorParameters(this, core::mem::transmute_copy(&ulrecordnum)) {
                Ok(ok__) => {
                    pdispparams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecordCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcrecords: *mut u32) -> windows_core::HRESULT
        where
            Identity: IErrorRecords_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IErrorRecords_Impl::GetRecordCount(this) {
                Ok(ok__) => {
                    pcrecords.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddErrorRecord: AddErrorRecord::<Identity, OFFSET>,
            GetBasicErrorInfo: GetBasicErrorInfo::<Identity, OFFSET>,
            GetCustomErrorObject: GetCustomErrorObject::<Identity, OFFSET>,
            GetErrorInfo: GetErrorInfo::<Identity, OFFSET>,
            GetErrorParameters: GetErrorParameters::<Identity, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IErrorRecords as windows_core::Interface>::IID
    }
}
pub trait IGetDataSource_Impl: Sized {
    fn GetDataSource(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IGetDataSource {}
impl IGetDataSource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetDataSource_Vtbl
    where
        Identity: IGetDataSource_Impl,
    {
        unsafe extern "system" fn GetDataSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdatasource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGetDataSource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetDataSource_Impl::GetDataSource(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppdatasource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDataSource: GetDataSource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetDataSource as windows_core::Interface>::IID
    }
}
pub trait IGetRow_Impl: Sized {
    fn GetRowFromHROW(&self, punkouter: Option<&windows_core::IUnknown>, hrow: usize, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetURLFromHROW(&self, hrow: usize) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IGetRow {}
impl IGetRow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetRow_Vtbl
    where
        Identity: IGetRow_Impl,
    {
        unsafe extern "system" fn GetRowFromHROW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, hrow: usize, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGetRow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetRow_Impl::GetRowFromHROW(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURLFromHROW<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: usize, ppwszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IGetRow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetRow_Impl::GetURLFromHROW(this, core::mem::transmute_copy(&hrow)) {
                Ok(ok__) => {
                    ppwszurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRowFromHROW: GetRowFromHROW::<Identity, OFFSET>,
            GetURLFromHROW: GetURLFromHROW::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetRow as windows_core::Interface>::IID
    }
}
pub trait IGetSession_Impl: Sized {
    fn GetSession(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IGetSession {}
impl IGetSession_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetSession_Vtbl
    where
        Identity: IGetSession_Impl,
    {
        unsafe extern "system" fn GetSession<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGetSession_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetSession_Impl::GetSession(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppsession.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSession: GetSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetSession as windows_core::Interface>::IID
    }
}
pub trait IGetSourceRow_Impl: Sized {
    fn GetSourceRow(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IGetSourceRow {}
impl IGetSourceRow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetSourceRow_Vtbl
    where
        Identity: IGetSourceRow_Impl,
    {
        unsafe extern "system" fn GetSourceRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pprow: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IGetSourceRow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetSourceRow_Impl::GetSourceRow(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    pprow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSourceRow: GetSourceRow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetSourceRow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IIndexDefinition_Impl: Sized {
    fn CreateIndex(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, cindexcolumndescs: usize, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
    fn DropIndex(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IIndexDefinition {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IIndexDefinition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IIndexDefinition_Vtbl
    where
        Identity: IIndexDefinition_Impl,
    {
        unsafe extern "system" fn CreateIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, cindexcolumndescs: usize, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: IIndexDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIndexDefinition_Impl::CreateIndex(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pindexid), core::mem::transmute_copy(&cindexcolumndescs), core::mem::transmute_copy(&rgindexcolumndescs), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&ppindexid)).into()
        }
        unsafe extern "system" fn DropIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: IIndexDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIndexDefinition_Impl::DropIndex(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pindexid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateIndex: CreateIndex::<Identity, OFFSET>,
            DropIndex: DropIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIndexDefinition as windows_core::Interface>::IID
    }
}
pub trait IInterval_Impl: Sized {
    fn GetLimits(&self, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut windows_core::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IInterval {}
impl IInterval_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInterval_Vtbl
    where
        Identity: IInterval_Impl,
    {
        unsafe extern "system" fn GetLimits<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IInterval_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IInterval_Impl::GetLimits(this, core::mem::transmute_copy(&pilklower), core::mem::transmute_copy(&ppropvarlower), core::mem::transmute_copy(&pilkupper), core::mem::transmute_copy(&ppropvarupper)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetLimits: GetLimits::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInterval as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ILoadFilter_Impl: Sized {
    fn LoadIFilter(&self, pwcspath: &windows_core::PCWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: Option<&windows_core::IUnknown>, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut Option<super::super::Storage::IndexServer::IFilter>) -> windows_core::Result<()>;
    fn LoadIFilterFromStorage(&self, pstg: Option<&super::Com::StructuredStorage::IStorage>, punkouter: Option<&windows_core::IUnknown>, pwcsoverride: &windows_core::PCWSTR, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut Option<super::super::Storage::IndexServer::IFilter>) -> windows_core::Result<()>;
    fn LoadIFilterFromStream(&self, pstm: Option<&super::Com::IStream>, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: Option<&windows_core::IUnknown>, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut Option<super::super::Storage::IndexServer::IFilter>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for ILoadFilter {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ILoadFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILoadFilter_Vtbl
    where
        Identity: ILoadFilter_Impl,
    {
        unsafe extern "system" fn LoadIFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcspath: windows_core::PCWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoadFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoadFilter_Impl::LoadIFilter(this, core::mem::transmute(&pwcspath), core::mem::transmute_copy(&pfilteredsources), windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&fusedefault), core::mem::transmute_copy(&pfilterclsid), core::mem::transmute_copy(&searchdecsize), core::mem::transmute_copy(&pwcssearchdesc), core::mem::transmute_copy(&ppifilt)).into()
        }
        unsafe extern "system" fn LoadIFilterFromStorage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, pwcsoverride: windows_core::PCWSTR, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoadFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoadFilter_Impl::LoadIFilterFromStorage(this, windows_core::from_raw_borrowed(&pstg), windows_core::from_raw_borrowed(&punkouter), core::mem::transmute(&pwcsoverride), core::mem::transmute_copy(&fusedefault), core::mem::transmute_copy(&pfilterclsid), core::mem::transmute_copy(&searchdecsize), core::mem::transmute_copy(&pwcssearchdesc), core::mem::transmute_copy(&ppifilt)).into()
        }
        unsafe extern "system" fn LoadIFilterFromStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoadFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoadFilter_Impl::LoadIFilterFromStream(this, windows_core::from_raw_borrowed(&pstm), core::mem::transmute_copy(&pfilteredsources), windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&fusedefault), core::mem::transmute_copy(&pfilterclsid), core::mem::transmute_copy(&searchdecsize), core::mem::transmute_copy(&pwcssearchdesc), core::mem::transmute_copy(&ppifilt)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadIFilter: LoadIFilter::<Identity, OFFSET>,
            LoadIFilterFromStorage: LoadIFilterFromStorage::<Identity, OFFSET>,
            LoadIFilterFromStream: LoadIFilterFromStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILoadFilter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ILoadFilterWithPrivateComActivation_Impl: Sized + ILoadFilter_Impl {
    fn LoadIFilterWithPrivateComActivation(&self, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: super::super::Foundation::BOOL, filterclsid: *mut windows_core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut Option<super::super::Storage::IndexServer::IFilter>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for ILoadFilterWithPrivateComActivation {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ILoadFilterWithPrivateComActivation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ILoadFilterWithPrivateComActivation_Vtbl
    where
        Identity: ILoadFilterWithPrivateComActivation_Impl,
    {
        unsafe extern "system" fn LoadIFilterWithPrivateComActivation<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: super::super::Foundation::BOOL, filterclsid: *mut windows_core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ILoadFilterWithPrivateComActivation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ILoadFilterWithPrivateComActivation_Impl::LoadIFilterWithPrivateComActivation(this, core::mem::transmute_copy(&filteredsources), core::mem::transmute_copy(&usedefault), core::mem::transmute_copy(&filterclsid), core::mem::transmute_copy(&isfilterprivatecomactivated), core::mem::transmute_copy(&filterobj)).into()
        }
        Self {
            base__: ILoadFilter_Vtbl::new::<Identity, OFFSET>(),
            LoadIFilterWithPrivateComActivation: LoadIFilterWithPrivateComActivation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILoadFilterWithPrivateComActivation as windows_core::Interface>::IID || iid == &<ILoadFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IMDDataset_Impl: Sized {
    fn FreeAxisInfo(&self, caxes: usize, rgaxisinfo: *const MDAXISINFO) -> windows_core::Result<()>;
    fn GetAxisInfo(&self, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> windows_core::Result<()>;
    fn GetAxisRowset(&self, punkouter: Option<&windows_core::IUnknown>, iaxis: usize, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetCellData(&self, haccessor: HACCESSOR, ulstartcell: usize, ulendcell: usize, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetSpecification(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IMDDataset {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IMDDataset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDDataset_Vtbl
    where
        Identity: IMDDataset_Impl,
    {
        unsafe extern "system" fn FreeAxisInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, caxes: usize, rgaxisinfo: *const MDAXISINFO) -> windows_core::HRESULT
        where
            Identity: IMDDataset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDDataset_Impl::FreeAxisInfo(this, core::mem::transmute_copy(&caxes), core::mem::transmute_copy(&rgaxisinfo)).into()
        }
        unsafe extern "system" fn GetAxisInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> windows_core::HRESULT
        where
            Identity: IMDDataset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDDataset_Impl::GetAxisInfo(this, core::mem::transmute_copy(&pcaxes), core::mem::transmute_copy(&prgaxisinfo)).into()
        }
        unsafe extern "system" fn GetAxisRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, iaxis: usize, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDDataset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDDataset_Impl::GetAxisRowset(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&iaxis), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn GetCellData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, ulstartcell: usize, ulendcell: usize, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDDataset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDDataset_Impl::GetCellData(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&ulstartcell), core::mem::transmute_copy(&ulendcell), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetSpecification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppspecification: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDDataset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDDataset_Impl::GetSpecification(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppspecification.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FreeAxisInfo: FreeAxisInfo::<Identity, OFFSET>,
            GetAxisInfo: GetAxisInfo::<Identity, OFFSET>,
            GetAxisRowset: GetAxisRowset::<Identity, OFFSET>,
            GetCellData: GetCellData::<Identity, OFFSET>,
            GetSpecification: GetSpecification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDDataset as windows_core::Interface>::IID
    }
}
pub trait IMDFind_Impl: Sized {
    fn FindCell(&self, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *const windows_core::PCWSTR) -> windows_core::Result<usize>;
    fn FindTuple(&self, ulaxisidentifier: u32, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *const windows_core::PCWSTR) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IMDFind {}
impl IMDFind_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDFind_Vtbl
    where
        Identity: IMDFind_Impl,
    {
        unsafe extern "system" fn FindCell<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *const windows_core::PCWSTR, pulcellordinal: *mut usize) -> windows_core::HRESULT
        where
            Identity: IMDFind_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDFind_Impl::FindCell(this, core::mem::transmute_copy(&ulstartingordinal), core::mem::transmute_copy(&cmembers), core::mem::transmute_copy(&rgpwszmember)) {
                Ok(ok__) => {
                    pulcellordinal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTuple<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulaxisidentifier: u32, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *const windows_core::PCWSTR, pultupleordinal: *mut u32) -> windows_core::HRESULT
        where
            Identity: IMDFind_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMDFind_Impl::FindTuple(this, core::mem::transmute_copy(&ulaxisidentifier), core::mem::transmute_copy(&ulstartingordinal), core::mem::transmute_copy(&cmembers), core::mem::transmute_copy(&rgpwszmember)) {
                Ok(ok__) => {
                    pultupleordinal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindCell: FindCell::<Identity, OFFSET>, FindTuple: FindTuple::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDFind as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IMDRangeRowset_Impl: Sized {
    fn GetRangeRowset(&self, punkouter: Option<&windows_core::IUnknown>, ulstartcell: usize, ulendcell: usize, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IMDRangeRowset {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IMDRangeRowset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMDRangeRowset_Vtbl
    where
        Identity: IMDRangeRowset_Impl,
    {
        unsafe extern "system" fn GetRangeRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, ulstartcell: usize, ulendcell: usize, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMDRangeRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMDRangeRowset_Impl::GetRangeRowset(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&ulstartcell), core::mem::transmute_copy(&ulendcell), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pprowset)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRangeRowset: GetRangeRowset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMDRangeRowset as windows_core::Interface>::IID
    }
}
pub trait IMetaData_Impl: Sized {
    fn GetData(&self, ppszkey: *mut windows_core::PWSTR, ppszvalue: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMetaData {}
impl IMetaData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMetaData_Vtbl
    where
        Identity: IMetaData_Impl,
    {
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszkey: *mut windows_core::PWSTR, ppszvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IMetaData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMetaData_Impl::GetData(this, core::mem::transmute_copy(&ppszkey), core::mem::transmute_copy(&ppszvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetData: GetData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMetaData as windows_core::Interface>::IID
    }
}
pub trait IMultipleResults_Impl: Sized {
    fn GetResult(&self, punkouter: Option<&windows_core::IUnknown>, lresultflag: isize, riid: *const windows_core::GUID, pcrowsaffected: *mut isize, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMultipleResults {}
impl IMultipleResults_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IMultipleResults_Vtbl
    where
        Identity: IMultipleResults_Impl,
    {
        unsafe extern "system" fn GetResult<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, lresultflag: isize, riid: *const windows_core::GUID, pcrowsaffected: *mut isize, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IMultipleResults_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultipleResults_Impl::GetResult(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&lresultflag), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pcrowsaffected), core::mem::transmute_copy(&pprowset)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetResult: GetResult::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultipleResults as windows_core::Interface>::IID
    }
}
pub trait INamedEntity_Impl: Sized {
    fn GetValue(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn DefaultPhrase(&self, ppszphrase: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INamedEntity {}
impl INamedEntity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INamedEntity_Vtbl
    where
        Identity: INamedEntity_Impl,
    {
        unsafe extern "system" fn GetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INamedEntity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INamedEntity_Impl::GetValue(this) {
                Ok(ok__) => {
                    ppszvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultPhrase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszphrase: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: INamedEntity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INamedEntity_Impl::DefaultPhrase(this, core::mem::transmute_copy(&ppszphrase)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetValue: GetValue::<Identity, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INamedEntity as windows_core::Interface>::IID
    }
}
pub trait INamedEntityCollector_Impl: Sized {
    fn Add(&self, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: Option<&IEntity>, pszvalue: &windows_core::PCWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INamedEntityCollector {}
impl INamedEntityCollector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> INamedEntityCollector_Vtbl
    where
        Identity: INamedEntityCollector_Impl,
    {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: *mut core::ffi::c_void, pszvalue: windows_core::PCWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> windows_core::HRESULT
        where
            Identity: INamedEntityCollector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INamedEntityCollector_Impl::Add(this, core::mem::transmute_copy(&beginspan), core::mem::transmute_copy(&endspan), core::mem::transmute_copy(&beginactual), core::mem::transmute_copy(&endactual), windows_core::from_raw_borrowed(&ptype), core::mem::transmute(&pszvalue), core::mem::transmute_copy(&certainty)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INamedEntityCollector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
pub trait IObjectAccessControl_Impl: Sized {
    fn GetObjectAccessRights(&self, pobject: *const SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> windows_core::Result<()>;
    fn GetObjectOwner(&self, pobject: *const SEC_OBJECT) -> windows_core::Result<*mut super::super::Security::Authorization::TRUSTEE_W>;
    fn IsObjectAccessAllowed(&self, pobject: *const SEC_OBJECT, paccessentry: *const super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetObjectAccessRights(&self, pobject: *const SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> windows_core::Result<()>;
    fn SetObjectOwner(&self, pobject: *const SEC_OBJECT, powner: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
impl windows_core::RuntimeName for IObjectAccessControl {}
#[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
impl IObjectAccessControl_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IObjectAccessControl_Vtbl
    where
        Identity: IObjectAccessControl_Impl,
    {
        unsafe extern "system" fn GetObjectAccessRights<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> windows_core::HRESULT
        where
            Identity: IObjectAccessControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectAccessControl_Impl::GetObjectAccessRights(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&pcaccessentries), core::mem::transmute_copy(&prgaccessentries)).into()
        }
        unsafe extern "system" fn GetObjectOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const SEC_OBJECT, ppowner: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT
        where
            Identity: IObjectAccessControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectAccessControl_Impl::GetObjectOwner(this, core::mem::transmute_copy(&pobject)) {
                Ok(ok__) => {
                    ppowner.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsObjectAccessAllowed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const SEC_OBJECT, paccessentry: *const super::super::Security::Authorization::EXPLICIT_ACCESS_W, pfresult: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IObjectAccessControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectAccessControl_Impl::IsObjectAccessAllowed(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&paccessentry)) {
                Ok(ok__) => {
                    pfresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectAccessRights<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> windows_core::HRESULT
        where
            Identity: IObjectAccessControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectAccessControl_Impl::SetObjectAccessRights(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&caccessentries), core::mem::transmute_copy(&prgaccessentries)).into()
        }
        unsafe extern "system" fn SetObjectOwner<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *const SEC_OBJECT, powner: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT
        where
            Identity: IObjectAccessControl_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectAccessControl_Impl::SetObjectOwner(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&powner)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectAccessRights: GetObjectAccessRights::<Identity, OFFSET>,
            GetObjectOwner: GetObjectOwner::<Identity, OFFSET>,
            IsObjectAccessAllowed: IsObjectAccessAllowed::<Identity, OFFSET>,
            SetObjectAccessRights: SetObjectAccessRights::<Identity, OFFSET>,
            SetObjectOwner: SetObjectOwner::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectAccessControl as windows_core::Interface>::IID
    }
}
pub trait IOpLockStatus_Impl: Sized {
    fn IsOplockValid(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsOplockBroken(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetOplockEventHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
impl windows_core::RuntimeName for IOpLockStatus {}
impl IOpLockStatus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOpLockStatus_Vtbl
    where
        Identity: IOpLockStatus_Impl,
    {
        unsafe extern "system" fn IsOplockValid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisoplockvalid: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpLockStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpLockStatus_Impl::IsOplockValid(this) {
                Ok(ok__) => {
                    pfisoplockvalid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOplockBroken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisoplockbroken: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IOpLockStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpLockStatus_Impl::IsOplockBroken(this) {
                Ok(ok__) => {
                    pfisoplockbroken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOplockEventHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phoplockev: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: IOpLockStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOpLockStatus_Impl::GetOplockEventHandle(this) {
                Ok(ok__) => {
                    phoplockev.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsOplockValid: IsOplockValid::<Identity, OFFSET>,
            IsOplockBroken: IsOplockBroken::<Identity, OFFSET>,
            GetOplockEventHandle: GetOplockEventHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpLockStatus as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IOpenRowset_Impl: Sized {
    fn OpenRowset(&self, punkouter: Option<&windows_core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IOpenRowset {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IOpenRowset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOpenRowset_Vtbl
    where
        Identity: IOpenRowset_Impl,
    {
        unsafe extern "system" fn OpenRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOpenRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOpenRowset_Impl::OpenRowset(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pindexid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pprowset)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OpenRowset: OpenRowset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOpenRowset as windows_core::Interface>::IID
    }
}
pub trait IParentRowset_Impl: Sized {
    fn GetChildRowset(&self, punkouter: Option<&windows_core::IUnknown>, iordinal: usize, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IParentRowset {}
impl IParentRowset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IParentRowset_Vtbl
    where
        Identity: IParentRowset_Impl,
    {
        unsafe extern "system" fn GetChildRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, iordinal: usize, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IParentRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IParentRowset_Impl::GetChildRowset(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&iordinal), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    pprowset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetChildRowset: GetChildRowset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IParentRowset as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IProtocolHandlerSite_Impl: Sized {
    fn GetFilter(&self, pclsidobj: *const windows_core::GUID, pcwszcontenttype: &windows_core::PCWSTR, pcwszextension: &windows_core::PCWSTR) -> windows_core::Result<super::super::Storage::IndexServer::IFilter>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IProtocolHandlerSite {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IProtocolHandlerSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProtocolHandlerSite_Vtbl
    where
        Identity: IProtocolHandlerSite_Impl,
    {
        unsafe extern "system" fn GetFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsidobj: *const windows_core::GUID, pcwszcontenttype: windows_core::PCWSTR, pcwszextension: windows_core::PCWSTR, ppfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProtocolHandlerSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProtocolHandlerSite_Impl::GetFilter(this, core::mem::transmute_copy(&pclsidobj), core::mem::transmute(&pcwszcontenttype), core::mem::transmute(&pcwszextension)) {
                Ok(ok__) => {
                    ppfilter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetFilter: GetFilter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProtocolHandlerSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideMoniker_Impl: Sized {
    fn GetMoniker(&self) -> windows_core::Result<super::Com::IMoniker>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IProvideMoniker {}
#[cfg(feature = "Win32_System_Com")]
impl IProvideMoniker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IProvideMoniker_Vtbl
    where
        Identity: IProvideMoniker_Impl,
    {
        unsafe extern "system" fn GetMoniker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimoniker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IProvideMoniker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProvideMoniker_Impl::GetMoniker(this) {
                Ok(ok__) => {
                    ppimoniker.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMoniker: GetMoniker::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideMoniker as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IQueryParser_Impl: Sized {
    fn Parse(&self, pszinputstring: &windows_core::PCWSTR, pcustomproperties: Option<&super::Com::IEnumUnknown>) -> windows_core::Result<IQuerySolution>;
    fn SetOption(&self, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetOption(&self, option: STRUCTURED_QUERY_SINGLE_OPTION) -> windows_core::Result<windows_core::PROPVARIANT>;
    fn SetMultiOption(&self, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: &windows_core::PCWSTR, poptionvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetSchemaProvider(&self) -> windows_core::Result<ISchemaProvider>;
    fn RestateToString(&self, pcondition: Option<&ICondition>, fuseenglish: super::super::Foundation::BOOL) -> windows_core::Result<windows_core::PWSTR>;
    fn ParsePropertyValue(&self, pszpropertyname: &windows_core::PCWSTR, pszinputstring: &windows_core::PCWSTR) -> windows_core::Result<IQuerySolution>;
    fn RestatePropertyValueToString(&self, pcondition: Option<&ICondition>, fuseenglish: super::super::Foundation::BOOL, ppszpropertyname: *mut windows_core::PWSTR, ppszquerystring: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IQueryParser {}
#[cfg(feature = "Win32_System_Com")]
impl IQueryParser_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IQueryParser_Vtbl
    where
        Identity: IQueryParser_Impl,
    {
        unsafe extern "system" fn Parse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszinputstring: windows_core::PCWSTR, pcustomproperties: *mut core::ffi::c_void, ppsolution: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IQueryParser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IQueryParser_Impl::Parse(this, core::mem::transmute(&pszinputstring), windows_core::from_raw_borrowed(&pcustomproperties)) {
                Ok(ok__) => {
                    ppsolution.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IQueryParser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQueryParser_Impl::SetOption(this, core::mem::transmute_copy(&option), core::mem::transmute_copy(&poptionvalue)).into()
        }
        unsafe extern "system" fn GetOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IQueryParser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IQueryParser_Impl::GetOption(this, core::mem::transmute_copy(&option)) {
                Ok(ok__) => {
                    poptionvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: windows_core::PCWSTR, poptionvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IQueryParser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQueryParser_Impl::SetMultiOption(this, core::mem::transmute_copy(&option), core::mem::transmute(&pszoptionkey), core::mem::transmute_copy(&poptionvalue)).into()
        }
        unsafe extern "system" fn GetSchemaProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppschemaprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IQueryParser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IQueryParser_Impl::GetSchemaProvider(this) {
                Ok(ok__) => {
                    ppschemaprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestateToString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcondition: *mut core::ffi::c_void, fuseenglish: super::super::Foundation::BOOL, ppszquerystring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IQueryParser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IQueryParser_Impl::RestateToString(this, windows_core::from_raw_borrowed(&pcondition), core::mem::transmute_copy(&fuseenglish)) {
                Ok(ok__) => {
                    ppszquerystring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParsePropertyValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, pszinputstring: windows_core::PCWSTR, ppsolution: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IQueryParser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IQueryParser_Impl::ParsePropertyValue(this, core::mem::transmute(&pszpropertyname), core::mem::transmute(&pszinputstring)) {
                Ok(ok__) => {
                    ppsolution.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestatePropertyValueToString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcondition: *mut core::ffi::c_void, fuseenglish: super::super::Foundation::BOOL, ppszpropertyname: *mut windows_core::PWSTR, ppszquerystring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IQueryParser_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQueryParser_Impl::RestatePropertyValueToString(this, windows_core::from_raw_borrowed(&pcondition), core::mem::transmute_copy(&fuseenglish), core::mem::transmute_copy(&ppszpropertyname), core::mem::transmute_copy(&ppszquerystring)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Parse: Parse::<Identity, OFFSET>,
            SetOption: SetOption::<Identity, OFFSET>,
            GetOption: GetOption::<Identity, OFFSET>,
            SetMultiOption: SetMultiOption::<Identity, OFFSET>,
            GetSchemaProvider: GetSchemaProvider::<Identity, OFFSET>,
            RestateToString: RestateToString::<Identity, OFFSET>,
            ParsePropertyValue: ParsePropertyValue::<Identity, OFFSET>,
            RestatePropertyValueToString: RestatePropertyValueToString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryParser as windows_core::Interface>::IID
    }
}
pub trait IQueryParserManager_Impl: Sized {
    fn CreateLoadedParser(&self, pszcatalog: &windows_core::PCWSTR, langidforkeywords: u16, riid: *const windows_core::GUID, ppqueryparser: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn InitializeOptions(&self, funderstandnqs: super::super::Foundation::BOOL, fautowildcard: super::super::Foundation::BOOL, pqueryparser: Option<&IQueryParser>) -> windows_core::Result<()>;
    fn SetOption(&self, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IQueryParserManager {}
impl IQueryParserManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IQueryParserManager_Vtbl
    where
        Identity: IQueryParserManager_Impl,
    {
        unsafe extern "system" fn CreateLoadedParser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcatalog: windows_core::PCWSTR, langidforkeywords: u16, riid: *const windows_core::GUID, ppqueryparser: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IQueryParserManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQueryParserManager_Impl::CreateLoadedParser(this, core::mem::transmute(&pszcatalog), core::mem::transmute_copy(&langidforkeywords), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppqueryparser)).into()
        }
        unsafe extern "system" fn InitializeOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, funderstandnqs: super::super::Foundation::BOOL, fautowildcard: super::super::Foundation::BOOL, pqueryparser: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IQueryParserManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQueryParserManager_Impl::InitializeOptions(this, core::mem::transmute_copy(&funderstandnqs), core::mem::transmute_copy(&fautowildcard), windows_core::from_raw_borrowed(&pqueryparser)).into()
        }
        unsafe extern "system" fn SetOption<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IQueryParserManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQueryParserManager_Impl::SetOption(this, core::mem::transmute_copy(&option), core::mem::transmute_copy(&poptionvalue)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateLoadedParser: CreateLoadedParser::<Identity, OFFSET>,
            InitializeOptions: InitializeOptions::<Identity, OFFSET>,
            SetOption: SetOption::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryParserManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
pub trait IQuerySolution_Impl: Sized + IConditionFactory_Impl {
    fn GetQuery(&self, ppquerynode: *mut Option<ICondition>, ppmaintype: *mut Option<IEntity>) -> windows_core::Result<()>;
    fn GetErrors(&self, riid: *const windows_core::GUID, ppparseerrors: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetLexicalData(&self, ppszinputstring: *mut windows_core::PWSTR, pptokens: *mut Option<ITokenCollection>, plcid: *mut u32, ppwordbreaker: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
impl windows_core::RuntimeName for IQuerySolution {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
impl IQuerySolution_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IQuerySolution_Vtbl
    where
        Identity: IQuerySolution_Impl,
    {
        unsafe extern "system" fn GetQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppquerynode: *mut *mut core::ffi::c_void, ppmaintype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IQuerySolution_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQuerySolution_Impl::GetQuery(this, core::mem::transmute_copy(&ppquerynode), core::mem::transmute_copy(&ppmaintype)).into()
        }
        unsafe extern "system" fn GetErrors<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparseerrors: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IQuerySolution_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQuerySolution_Impl::GetErrors(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparseerrors)).into()
        }
        unsafe extern "system" fn GetLexicalData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszinputstring: *mut windows_core::PWSTR, pptokens: *mut *mut core::ffi::c_void, plcid: *mut u32, ppwordbreaker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IQuerySolution_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IQuerySolution_Impl::GetLexicalData(this, core::mem::transmute_copy(&ppszinputstring), core::mem::transmute_copy(&pptokens), core::mem::transmute_copy(&plcid), core::mem::transmute_copy(&ppwordbreaker)).into()
        }
        Self {
            base__: IConditionFactory_Vtbl::new::<Identity, OFFSET>(),
            GetQuery: GetQuery::<Identity, OFFSET>,
            GetErrors: GetErrors::<Identity, OFFSET>,
            GetLexicalData: GetLexicalData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQuerySolution as windows_core::Interface>::IID || iid == &<IConditionFactory as windows_core::Interface>::IID
    }
}
pub trait IReadData_Impl: Sized {
    fn ReadData(&self, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, haccessor: HACCESSOR, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> windows_core::Result<()>;
    fn ReleaseChapter(&self, hchapter: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IReadData {}
impl IReadData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IReadData_Vtbl
    where
        Identity: IReadData_Impl,
    {
        unsafe extern "system" fn ReadData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, haccessor: HACCESSOR, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> windows_core::HRESULT
        where
            Identity: IReadData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReadData_Impl::ReadData(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark), core::mem::transmute_copy(&lrowsoffset), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&pcrowsobtained), core::mem::transmute_copy(&ppfixeddata), core::mem::transmute_copy(&pcbvariabletotal), core::mem::transmute_copy(&ppvariabledata)).into()
        }
        unsafe extern "system" fn ReleaseChapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize) -> windows_core::HRESULT
        where
            Identity: IReadData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IReadData_Impl::ReleaseChapter(this, core::mem::transmute_copy(&hchapter)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadData: ReadData::<Identity, OFFSET>,
            ReleaseChapter: ReleaseChapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReadData as windows_core::Interface>::IID
    }
}
pub trait IRegisterProvider_Impl: Sized {
    fn GetURLMapping(&self, pwszurl: &windows_core::PCWSTR, dwreserved: usize) -> windows_core::Result<windows_core::GUID>;
    fn SetURLMapping(&self, pwszurl: &windows_core::PCWSTR, dwreserved: usize, rclsidprovider: *const windows_core::GUID) -> windows_core::Result<()>;
    fn UnregisterProvider(&self, pwszurl: &windows_core::PCWSTR, dwreserved: usize, rclsidprovider: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRegisterProvider {}
impl IRegisterProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRegisterProvider_Vtbl
    where
        Identity: IRegisterProvider_Impl,
    {
        unsafe extern "system" fn GetURLMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwreserved: usize, pclsidprovider: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRegisterProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRegisterProvider_Impl::GetURLMapping(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwreserved)) {
                Ok(ok__) => {
                    pclsidprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURLMapping<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwreserved: usize, rclsidprovider: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRegisterProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegisterProvider_Impl::SetURLMapping(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&rclsidprovider)).into()
        }
        unsafe extern "system" fn UnregisterProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwreserved: usize, rclsidprovider: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IRegisterProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRegisterProvider_Impl::UnregisterProvider(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&rclsidprovider)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetURLMapping: GetURLMapping::<Identity, OFFSET>,
            SetURLMapping: SetURLMapping::<Identity, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRegisterProvider as windows_core::Interface>::IID
    }
}
pub trait IRelationship_Impl: Sized {
    fn Name(&self, ppszname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn IsReal(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn Destination(&self) -> windows_core::Result<IEntity>;
    fn MetaData(&self, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn DefaultPhrase(&self, ppszphrase: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRelationship {}
impl IRelationship_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRelationship_Vtbl
    where
        Identity: IRelationship_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IRelationship_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRelationship_Impl::Name(this, core::mem::transmute_copy(&ppszname)).into()
        }
        unsafe extern "system" fn IsReal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisreal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRelationship_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRelationship_Impl::IsReal(this) {
                Ok(ok__) => {
                    pisreal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destination<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdestinationentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRelationship_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRelationship_Impl::Destination(this) {
                Ok(ok__) => {
                    pdestinationentity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRelationship_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRelationship_Impl::MetaData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pmetadata)).into()
        }
        unsafe extern "system" fn DefaultPhrase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszphrase: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IRelationship_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRelationship_Impl::DefaultPhrase(this, core::mem::transmute_copy(&ppszphrase)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            IsReal: IsReal::<Identity, OFFSET>,
            Destination: Destination::<Identity, OFFSET>,
            MetaData: MetaData::<Identity, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRelationship as windows_core::Interface>::IID
    }
}
pub trait IRichChunk_Impl: Sized {
    fn GetData(&self, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut windows_core::PWSTR, pvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRichChunk {}
impl IRichChunk_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRichChunk_Vtbl
    where
        Identity: IRichChunk_Impl,
    {
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut windows_core::PWSTR, pvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IRichChunk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRichChunk_Impl::GetData(this, core::mem::transmute_copy(&pfirstpos), core::mem::transmute_copy(&plength), core::mem::transmute_copy(&ppsz), core::mem::transmute_copy(&pvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetData: GetData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRichChunk as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IRow_Impl: Sized {
    fn GetColumns(&self, ccolumns: usize, rgcolumns: *mut DBCOLUMNACCESS) -> windows_core::Result<()>;
    fn GetSourceRowset(&self, riid: *const windows_core::GUID, pprowset: *mut Option<windows_core::IUnknown>, phrow: *mut usize) -> windows_core::Result<()>;
    fn Open(&self, punkouter: Option<&windows_core::IUnknown>, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const windows_core::GUID, dwbindflags: u32, riid: *const windows_core::GUID, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IRow {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IRow_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRow_Vtbl
    where
        Identity: IRow_Impl,
    {
        unsafe extern "system" fn GetColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumns: usize, rgcolumns: *mut DBCOLUMNACCESS) -> windows_core::HRESULT
        where
            Identity: IRow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRow_Impl::GetColumns(this, core::mem::transmute_copy(&ccolumns), core::mem::transmute_copy(&rgcolumns)).into()
        }
        unsafe extern "system" fn GetSourceRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void, phrow: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRow_Impl::GetSourceRowset(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pprowset), core::mem::transmute_copy(&phrow)).into()
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const windows_core::GUID, dwbindflags: u32, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRow_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRow_Impl::Open(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&pcolumnid), core::mem::transmute_copy(&rguidcolumntype), core::mem::transmute_copy(&dwbindflags), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColumns: GetColumns::<Identity, OFFSET>,
            GetSourceRowset: GetSourceRowset::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRow as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IRowChange_Impl: Sized {
    fn SetColumns(&self, ccolumns: usize, rgcolumns: *const DBCOLUMNACCESS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IRowChange {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IRowChange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowChange_Vtbl
    where
        Identity: IRowChange_Impl,
    {
        unsafe extern "system" fn SetColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumns: usize, rgcolumns: *const DBCOLUMNACCESS) -> windows_core::HRESULT
        where
            Identity: IRowChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowChange_Impl::SetColumns(this, core::mem::transmute_copy(&ccolumns), core::mem::transmute_copy(&rgcolumns)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetColumns: SetColumns::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowChange as windows_core::Interface>::IID
    }
}
pub trait IRowPosition_Impl: Sized {
    fn ClearRowPosition(&self) -> windows_core::Result<()>;
    fn GetRowPosition(&self, phchapter: *mut usize, phrow: *mut usize, pdwpositionflags: *mut u32) -> windows_core::Result<()>;
    fn GetRowset(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn Initialize(&self, prowset: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetRowPosition(&self, hchapter: usize, hrow: usize, dwpositionflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowPosition {}
impl IRowPosition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowPosition_Vtbl
    where
        Identity: IRowPosition_Impl,
    {
        unsafe extern "system" fn ClearRowPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowPosition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowPosition_Impl::ClearRowPosition(this).into()
        }
        unsafe extern "system" fn GetRowPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phchapter: *mut usize, phrow: *mut usize, pdwpositionflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowPosition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowPosition_Impl::GetRowPosition(this, core::mem::transmute_copy(&phchapter), core::mem::transmute_copy(&phrow), core::mem::transmute_copy(&pdwpositionflags)).into()
        }
        unsafe extern "system" fn GetRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowPosition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowPosition_Impl::GetRowset(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    pprowset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prowset: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowPosition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowPosition_Impl::Initialize(this, windows_core::from_raw_borrowed(&prowset)).into()
        }
        unsafe extern "system" fn SetRowPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, hrow: usize, dwpositionflags: u32) -> windows_core::HRESULT
        where
            Identity: IRowPosition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowPosition_Impl::SetRowPosition(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&dwpositionflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ClearRowPosition: ClearRowPosition::<Identity, OFFSET>,
            GetRowPosition: GetRowPosition::<Identity, OFFSET>,
            GetRowset: GetRowset::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            SetRowPosition: SetRowPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowPosition as windows_core::Interface>::IID
    }
}
pub trait IRowPositionChange_Impl: Sized {
    fn OnRowPositionChange(&self, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowPositionChange {}
impl IRowPositionChange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowPositionChange_Vtbl
    where
        Identity: IRowPositionChange_Impl,
    {
        unsafe extern "system" fn OnRowPositionChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRowPositionChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowPositionChange_Impl::OnRowPositionChange(this, core::mem::transmute_copy(&ereason), core::mem::transmute_copy(&ephase), core::mem::transmute_copy(&fcantdeny)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnRowPositionChange: OnRowPositionChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowPositionChange as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IRowSchemaChange_Impl: Sized + IRowChange_Impl {
    fn DeleteColumns(&self, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> windows_core::Result<()>;
    fn AddColumns(&self, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IRowSchemaChange {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IRowSchemaChange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowSchemaChange_Vtbl
    where
        Identity: IRowSchemaChange_Impl,
    {
        unsafe extern "system" fn DeleteColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowSchemaChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowSchemaChange_Impl::DeleteColumns(this, core::mem::transmute_copy(&ccolumns), core::mem::transmute_copy(&rgcolumnids), core::mem::transmute_copy(&rgdwstatus)).into()
        }
        unsafe extern "system" fn AddColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> windows_core::HRESULT
        where
            Identity: IRowSchemaChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowSchemaChange_Impl::AddColumns(this, core::mem::transmute_copy(&ccolumns), core::mem::transmute_copy(&rgnewcolumninfo), core::mem::transmute_copy(&rgcolumns)).into()
        }
        Self { base__: IRowChange_Vtbl::new::<Identity, OFFSET>(), DeleteColumns: DeleteColumns::<Identity, OFFSET>, AddColumns: AddColumns::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowSchemaChange as windows_core::Interface>::IID || iid == &<IRowChange as windows_core::Interface>::IID
    }
}
pub trait IRowset_Impl: Sized {
    fn AddRefRows(&self, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> windows_core::Result<()>;
    fn GetData(&self, hrow: usize, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetNextRows(&self, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> windows_core::Result<()>;
    fn ReleaseRows(&self, crows: usize, rghrows: *const usize, rgrowoptions: *const u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> windows_core::Result<()>;
    fn RestartPosition(&self, hreserved: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowset {}
impl IRowset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowset_Vtbl
    where
        Identity: IRowset_Impl,
    {
        unsafe extern "system" fn AddRefRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowset_Impl::AddRefRows(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&rgrefcounts), core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn GetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowset_Impl::GetData(this, core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetNextRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowset_Impl::GetNextRows(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&lrowsoffset), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&pcrowsobtained), core::mem::transmute_copy(&prghrows)).into()
        }
        unsafe extern "system" fn ReleaseRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: usize, rghrows: *const usize, rgrowoptions: *const u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowset_Impl::ReleaseRows(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&rgrowoptions), core::mem::transmute_copy(&rgrefcounts), core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn RestartPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize) -> windows_core::HRESULT
        where
            Identity: IRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowset_Impl::RestartPosition(this, core::mem::transmute_copy(&hreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefRows: AddRefRows::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            GetNextRows: GetNextRows::<Identity, OFFSET>,
            ReleaseRows: ReleaseRows::<Identity, OFFSET>,
            RestartPosition: RestartPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowset as windows_core::Interface>::IID
    }
}
pub trait IRowsetAsynch_Impl: Sized {
    fn RatioFinished(&self, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetAsynch {}
impl IRowsetAsynch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetAsynch_Vtbl
    where
        Identity: IRowsetAsynch_Impl,
    {
        unsafe extern "system" fn RatioFinished<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRowsetAsynch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetAsynch_Impl::RatioFinished(this, core::mem::transmute_copy(&puldenominator), core::mem::transmute_copy(&pulnumerator), core::mem::transmute_copy(&pcrows), core::mem::transmute_copy(&pfnewrows)).into()
        }
        unsafe extern "system" fn Stop<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetAsynch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetAsynch_Impl::Stop(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RatioFinished: RatioFinished::<Identity, OFFSET>, Stop: Stop::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetAsynch as windows_core::Interface>::IID
    }
}
pub trait IRowsetBookmark_Impl: Sized {
    fn PositionOnBookmark(&self, hchapter: usize, cbbookmark: usize, pbookmark: *const u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetBookmark {}
impl IRowsetBookmark_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetBookmark_Vtbl
    where
        Identity: IRowsetBookmark_Impl,
    {
        unsafe extern "system" fn PositionOnBookmark<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8) -> windows_core::HRESULT
        where
            Identity: IRowsetBookmark_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetBookmark_Impl::PositionOnBookmark(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PositionOnBookmark: PositionOnBookmark::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetBookmark as windows_core::Interface>::IID
    }
}
pub trait IRowsetChange_Impl: Sized {
    fn DeleteRows(&self, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> windows_core::Result<()>;
    fn SetData(&self, hrow: usize, haccessor: HACCESSOR, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn InsertRow(&self, hreserved: usize, haccessor: HACCESSOR, pdata: *const core::ffi::c_void) -> windows_core::Result<usize>;
}
impl windows_core::RuntimeName for IRowsetChange {}
impl IRowsetChange_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetChange_Vtbl
    where
        Identity: IRowsetChange_Impl,
    {
        unsafe extern "system" fn DeleteRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetChange_Impl::DeleteRows(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetChange_Impl::SetData(this, core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn InsertRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, haccessor: HACCESSOR, pdata: *const core::ffi::c_void, phrow: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetChange_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetChange_Impl::InsertRow(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)) {
                Ok(ok__) => {
                    phrow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeleteRows: DeleteRows::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            InsertRow: InsertRow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetChange as windows_core::Interface>::IID
    }
}
pub trait IRowsetChangeExtInfo_Impl: Sized {
    fn GetOriginalRow(&self, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> windows_core::Result<()>;
    fn GetPendingColumns(&self, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetChangeExtInfo {}
impl IRowsetChangeExtInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetChangeExtInfo_Vtbl
    where
        Identity: IRowsetChangeExtInfo_Impl,
    {
        unsafe extern "system" fn GetOriginalRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetChangeExtInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetChangeExtInfo_Impl::GetOriginalRow(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&phroworiginal)).into()
        }
        unsafe extern "system" fn GetPendingColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetChangeExtInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetChangeExtInfo_Impl::GetPendingColumns(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&ccolumnordinals), core::mem::transmute_copy(&rgiordinals), core::mem::transmute_copy(&rgcolumnstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOriginalRow: GetOriginalRow::<Identity, OFFSET>,
            GetPendingColumns: GetPendingColumns::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetChangeExtInfo as windows_core::Interface>::IID
    }
}
pub trait IRowsetChapterMember_Impl: Sized {
    fn IsRowInChapter(&self, hchapter: usize, hrow: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetChapterMember {}
impl IRowsetChapterMember_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetChapterMember_Vtbl
    where
        Identity: IRowsetChapterMember_Impl,
    {
        unsafe extern "system" fn IsRowInChapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, hrow: usize) -> windows_core::HRESULT
        where
            Identity: IRowsetChapterMember_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetChapterMember_Impl::IsRowInChapter(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&hrow)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsRowInChapter: IsRowInChapter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetChapterMember as windows_core::Interface>::IID
    }
}
pub trait IRowsetCopyRows_Impl: Sized {
    fn CloseSource(&self, hsourceid: u16) -> windows_core::Result<()>;
    fn CopyByHROWS(&self, hsourceid: u16, hreserved: usize, crows: isize, rghrows: *const usize, bflags: u32) -> windows_core::Result<()>;
    fn CopyRows(&self, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32) -> windows_core::Result<usize>;
    fn DefineSource(&self, prowsetsource: Option<&IRowset>, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize) -> windows_core::Result<u16>;
}
impl windows_core::RuntimeName for IRowsetCopyRows {}
impl IRowsetCopyRows_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetCopyRows_Vtbl
    where
        Identity: IRowsetCopyRows_Impl,
    {
        unsafe extern "system" fn CloseSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsourceid: u16) -> windows_core::HRESULT
        where
            Identity: IRowsetCopyRows_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetCopyRows_Impl::CloseSource(this, core::mem::transmute_copy(&hsourceid)).into()
        }
        unsafe extern "system" fn CopyByHROWS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsourceid: u16, hreserved: usize, crows: isize, rghrows: *const usize, bflags: u32) -> windows_core::HRESULT
        where
            Identity: IRowsetCopyRows_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetCopyRows_Impl::CopyByHROWS(this, core::mem::transmute_copy(&hsourceid), core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&bflags)).into()
        }
        unsafe extern "system" fn CopyRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32, pcrowscopied: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetCopyRows_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetCopyRows_Impl::CopyRows(this, core::mem::transmute_copy(&hsourceid), core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&bflags)) {
                Ok(ok__) => {
                    pcrowscopied.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefineSource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prowsetsource: *mut core::ffi::c_void, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize, phsourceid: *mut u16) -> windows_core::HRESULT
        where
            Identity: IRowsetCopyRows_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetCopyRows_Impl::DefineSource(this, windows_core::from_raw_borrowed(&prowsetsource), core::mem::transmute_copy(&ccolids), core::mem::transmute_copy(&rgsourcecolumns), core::mem::transmute_copy(&rgtargetcolumns)) {
                Ok(ok__) => {
                    phsourceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CloseSource: CloseSource::<Identity, OFFSET>,
            CopyByHROWS: CopyByHROWS::<Identity, OFFSET>,
            CopyRows: CopyRows::<Identity, OFFSET>,
            DefineSource: DefineSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetCopyRows as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IRowsetCurrentIndex_Impl: Sized + IRowsetIndex_Impl {
    fn GetIndex(&self) -> windows_core::Result<*mut super::super::Storage::IndexServer::DBID>;
    fn SetIndex(&self, pindexid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IRowsetCurrentIndex {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IRowsetCurrentIndex_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetCurrentIndex_Vtbl
    where
        Identity: IRowsetCurrentIndex_Impl,
    {
        unsafe extern "system" fn GetIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: IRowsetCurrentIndex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetCurrentIndex_Impl::GetIndex(this) {
                Ok(ok__) => {
                    ppindexid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexid: *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: IRowsetCurrentIndex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetCurrentIndex_Impl::SetIndex(this, core::mem::transmute_copy(&pindexid)).into()
        }
        Self { base__: IRowsetIndex_Vtbl::new::<Identity, OFFSET>(), GetIndex: GetIndex::<Identity, OFFSET>, SetIndex: SetIndex::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetCurrentIndex as windows_core::Interface>::IID || iid == &<IRowsetIndex as windows_core::Interface>::IID
    }
}
pub trait IRowsetEvents_Impl: Sized {
    fn OnNewItem(&self, itemid: *const windows_core::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::Result<()>;
    fn OnChangedItem(&self, itemid: *const windows_core::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::Result<()>;
    fn OnDeletedItem(&self, itemid: *const windows_core::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::Result<()>;
    fn OnRowsetEvent(&self, eventtype: ROWSETEVENT_TYPE, eventdata: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetEvents {}
impl IRowsetEvents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetEvents_Vtbl
    where
        Identity: IRowsetEvents_Impl,
    {
        unsafe extern "system" fn OnNewItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, newitemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT
        where
            Identity: IRowsetEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetEvents_Impl::OnNewItem(this, core::mem::transmute_copy(&itemid), core::mem::transmute_copy(&newitemstate)).into()
        }
        unsafe extern "system" fn OnChangedItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT
        where
            Identity: IRowsetEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetEvents_Impl::OnChangedItem(this, core::mem::transmute_copy(&itemid), core::mem::transmute_copy(&rowsetitemstate), core::mem::transmute_copy(&changeditemstate)).into()
        }
        unsafe extern "system" fn OnDeletedItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> windows_core::HRESULT
        where
            Identity: IRowsetEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetEvents_Impl::OnDeletedItem(this, core::mem::transmute_copy(&itemid), core::mem::transmute_copy(&deleteditemstate)).into()
        }
        unsafe extern "system" fn OnRowsetEvent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventtype: ROWSETEVENT_TYPE, eventdata: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IRowsetEvents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetEvents_Impl::OnRowsetEvent(this, core::mem::transmute_copy(&eventtype), core::mem::transmute_copy(&eventdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnNewItem: OnNewItem::<Identity, OFFSET>,
            OnChangedItem: OnChangedItem::<Identity, OFFSET>,
            OnDeletedItem: OnDeletedItem::<Identity, OFFSET>,
            OnRowsetEvent: OnRowsetEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetEvents as windows_core::Interface>::IID
    }
}
pub trait IRowsetExactScroll_Impl: Sized + IRowsetScroll_Impl {
    fn GetExactPosition(&self, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetExactScroll {}
impl IRowsetExactScroll_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetExactScroll_Vtbl
    where
        Identity: IRowsetExactScroll_Impl,
    {
        unsafe extern "system" fn GetExactPosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetExactScroll_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetExactScroll_Impl::GetExactPosition(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark), core::mem::transmute_copy(&pulposition), core::mem::transmute_copy(&pcrows)).into()
        }
        Self { base__: IRowsetScroll_Vtbl::new::<Identity, OFFSET>(), GetExactPosition: GetExactPosition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetExactScroll as windows_core::Interface>::IID || iid == &<IRowset as windows_core::Interface>::IID || iid == &<IRowsetLocate as windows_core::Interface>::IID || iid == &<IRowsetScroll as windows_core::Interface>::IID
    }
}
pub trait IRowsetFastLoad_Impl: Sized {
    fn InsertRow(&self, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Commit(&self, fdone: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetFastLoad {}
impl IRowsetFastLoad_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetFastLoad_Vtbl
    where
        Identity: IRowsetFastLoad_Impl,
    {
        unsafe extern "system" fn InsertRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetFastLoad_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetFastLoad_Impl::InsertRow(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn Commit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdone: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRowsetFastLoad_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetFastLoad_Impl::Commit(this, core::mem::transmute_copy(&fdone)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InsertRow: InsertRow::<Identity, OFFSET>, Commit: Commit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetFastLoad as windows_core::Interface>::IID
    }
}
pub trait IRowsetFind_Impl: Sized {
    fn FindNextRow(&self, hchapter: usize, haccessor: HACCESSOR, pfindvalue: *const core::ffi::c_void, compareop: u32, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetFind {}
impl IRowsetFind_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetFind_Vtbl
    where
        Identity: IRowsetFind_Impl,
    {
        unsafe extern "system" fn FindNextRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, haccessor: HACCESSOR, pfindvalue: *const core::ffi::c_void, compareop: u32, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetFind_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetFind_Impl::FindNextRow(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pfindvalue), core::mem::transmute_copy(&compareop), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark), core::mem::transmute_copy(&lrowsoffset), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&pcrowsobtained), core::mem::transmute_copy(&prghrows)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindNextRow: FindNextRow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetFind as windows_core::Interface>::IID
    }
}
pub trait IRowsetIdentity_Impl: Sized {
    fn IsSameRow(&self, hthisrow: usize, hthatrow: usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetIdentity {}
impl IRowsetIdentity_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetIdentity_Vtbl
    where
        Identity: IRowsetIdentity_Impl,
    {
        unsafe extern "system" fn IsSameRow<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hthisrow: usize, hthatrow: usize) -> windows_core::HRESULT
        where
            Identity: IRowsetIdentity_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetIdentity_Impl::IsSameRow(this, core::mem::transmute_copy(&hthisrow), core::mem::transmute_copy(&hthatrow)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsSameRow: IsSameRow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetIdentity as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IRowsetIndex_Impl: Sized {
    fn GetIndexInfo(&self, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
    fn Seek(&self, haccessor: HACCESSOR, ckeyvalues: usize, pdata: *const core::ffi::c_void, dwseekoptions: u32) -> windows_core::Result<()>;
    fn SetRange(&self, haccessor: HACCESSOR, cstartkeycolumns: usize, pstartdata: *const core::ffi::c_void, cendkeycolumns: usize, penddata: *const core::ffi::c_void, dwrangeoptions: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IRowsetIndex {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IRowsetIndex_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetIndex_Vtbl
    where
        Identity: IRowsetIndex_Impl,
    {
        unsafe extern "system" fn GetIndexInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: IRowsetIndex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetIndex_Impl::GetIndexInfo(this, core::mem::transmute_copy(&pckeycolumns), core::mem::transmute_copy(&prgindexcolumndesc), core::mem::transmute_copy(&pcindexpropertysets), core::mem::transmute_copy(&prgindexpropertysets)).into()
        }
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, ckeyvalues: usize, pdata: *const core::ffi::c_void, dwseekoptions: u32) -> windows_core::HRESULT
        where
            Identity: IRowsetIndex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetIndex_Impl::Seek(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&ckeyvalues), core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&dwseekoptions)).into()
        }
        unsafe extern "system" fn SetRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, cstartkeycolumns: usize, pstartdata: *const core::ffi::c_void, cendkeycolumns: usize, penddata: *const core::ffi::c_void, dwrangeoptions: u32) -> windows_core::HRESULT
        where
            Identity: IRowsetIndex_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetIndex_Impl::SetRange(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&cstartkeycolumns), core::mem::transmute_copy(&pstartdata), core::mem::transmute_copy(&cendkeycolumns), core::mem::transmute_copy(&penddata), core::mem::transmute_copy(&dwrangeoptions)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIndexInfo: GetIndexInfo::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            SetRange: SetRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetIndex as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IRowsetInfo_Impl: Sized {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
    fn GetReferencedRowset(&self, iordinal: usize, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetSpecification(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IRowsetInfo {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IRowsetInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetInfo_Vtbl
    where
        Identity: IRowsetInfo_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: IRowsetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetInfo_Impl::GetProperties(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn GetReferencedRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iordinal: usize, riid: *const windows_core::GUID, ppreferencedrowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetInfo_Impl::GetReferencedRowset(this, core::mem::transmute_copy(&iordinal), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppreferencedrowset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpecification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppspecification: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetInfo_Impl::GetSpecification(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppspecification.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetReferencedRowset: GetReferencedRowset::<Identity, OFFSET>,
            GetSpecification: GetSpecification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetInfo as windows_core::Interface>::IID
    }
}
pub trait IRowsetKeys_Impl: Sized {
    fn ListKeys(&self, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetKeys {}
impl IRowsetKeys_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetKeys_Vtbl
    where
        Identity: IRowsetKeys_Impl,
    {
        unsafe extern "system" fn ListKeys<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetKeys_Impl::ListKeys(this, core::mem::transmute_copy(&pccolumns), core::mem::transmute_copy(&prgcolumns)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ListKeys: ListKeys::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetKeys as windows_core::Interface>::IID
    }
}
pub trait IRowsetLocate_Impl: Sized + IRowset_Impl {
    fn Compare(&self, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8) -> windows_core::Result<u32>;
    fn GetRowsAt(&self, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> windows_core::Result<()>;
    fn GetRowsByBookmark(&self, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> windows_core::Result<()>;
    fn Hash(&self, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetLocate {}
impl IRowsetLocate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetLocate_Vtbl
    where
        Identity: IRowsetLocate_Impl,
    {
        unsafe extern "system" fn Compare<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8, pcomparison: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetLocate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetLocate_Impl::Compare(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&cbbookmark1), core::mem::transmute_copy(&pbookmark1), core::mem::transmute_copy(&cbbookmark2), core::mem::transmute_copy(&pbookmark2)) {
                Ok(ok__) => {
                    pcomparison.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRowsAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetLocate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetLocate_Impl::GetRowsAt(this, core::mem::transmute_copy(&hreserved1), core::mem::transmute_copy(&hreserved2), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark), core::mem::transmute_copy(&lrowsoffset), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&pcrowsobtained), core::mem::transmute_copy(&prghrows)).into()
        }
        unsafe extern "system" fn GetRowsByBookmark<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetLocate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetLocate_Impl::GetRowsByBookmark(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rgcbbookmarks), core::mem::transmute_copy(&rgpbookmarks), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&rgrowstatus)).into()
        }
        unsafe extern "system" fn Hash<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetLocate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetLocate_Impl::Hash(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&cbookmarks), core::mem::transmute_copy(&rgcbbookmarks), core::mem::transmute_copy(&rgpbookmarks), core::mem::transmute_copy(&rghashedvalues), core::mem::transmute_copy(&rgbookmarkstatus)).into()
        }
        Self {
            base__: IRowset_Vtbl::new::<Identity, OFFSET>(),
            Compare: Compare::<Identity, OFFSET>,
            GetRowsAt: GetRowsAt::<Identity, OFFSET>,
            GetRowsByBookmark: GetRowsByBookmark::<Identity, OFFSET>,
            Hash: Hash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetLocate as windows_core::Interface>::IID || iid == &<IRowset as windows_core::Interface>::IID
    }
}
pub trait IRowsetNewRowAfter_Impl: Sized {
    fn SetNewDataAfter(&self, hchapter: usize, cbbmprevious: u32, pbmprevious: *const u8, haccessor: HACCESSOR, pdata: *const u8) -> windows_core::Result<usize>;
}
impl windows_core::RuntimeName for IRowsetNewRowAfter {}
impl IRowsetNewRowAfter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetNewRowAfter_Vtbl
    where
        Identity: IRowsetNewRowAfter_Impl,
    {
        unsafe extern "system" fn SetNewDataAfter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, cbbmprevious: u32, pbmprevious: *const u8, haccessor: HACCESSOR, pdata: *const u8, phrow: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetNewRowAfter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetNewRowAfter_Impl::SetNewDataAfter(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&cbbmprevious), core::mem::transmute_copy(&pbmprevious), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)) {
                Ok(ok__) => {
                    phrow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetNewDataAfter: SetNewDataAfter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetNewRowAfter as windows_core::Interface>::IID
    }
}
pub trait IRowsetNextRowset_Impl: Sized {
    fn GetNextRowset(&self, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IRowsetNextRowset {}
impl IRowsetNextRowset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetNextRowset_Vtbl
    where
        Identity: IRowsetNextRowset_Impl,
    {
        unsafe extern "system" fn GetNextRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppnextrowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetNextRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetNextRowset_Impl::GetNextRowset(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppnextrowset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNextRowset: GetNextRowset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetNextRowset as windows_core::Interface>::IID
    }
}
pub trait IRowsetNotify_Impl: Sized {
    fn OnFieldChange(&self, prowset: Option<&IRowset>, hrow: usize, ccolumns: usize, rgcolumns: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnRowChange(&self, prowset: Option<&IRowset>, crows: usize, rghrows: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OnRowsetChange(&self, prowset: Option<&IRowset>, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetNotify {}
impl IRowsetNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetNotify_Vtbl
    where
        Identity: IRowsetNotify_Impl,
    {
        unsafe extern "system" fn OnFieldChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prowset: *mut core::ffi::c_void, hrow: usize, ccolumns: usize, rgcolumns: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRowsetNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetNotify_Impl::OnFieldChange(this, windows_core::from_raw_borrowed(&prowset), core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&ccolumns), core::mem::transmute_copy(&rgcolumns), core::mem::transmute_copy(&ereason), core::mem::transmute_copy(&ephase), core::mem::transmute_copy(&fcantdeny)).into()
        }
        unsafe extern "system" fn OnRowChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prowset: *mut core::ffi::c_void, crows: usize, rghrows: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRowsetNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetNotify_Impl::OnRowChange(this, windows_core::from_raw_borrowed(&prowset), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&ereason), core::mem::transmute_copy(&ephase), core::mem::transmute_copy(&fcantdeny)).into()
        }
        unsafe extern "system" fn OnRowsetChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prowset: *mut core::ffi::c_void, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IRowsetNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetNotify_Impl::OnRowsetChange(this, windows_core::from_raw_borrowed(&prowset), core::mem::transmute_copy(&ereason), core::mem::transmute_copy(&ephase), core::mem::transmute_copy(&fcantdeny)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnFieldChange: OnFieldChange::<Identity, OFFSET>,
            OnRowChange: OnRowChange::<Identity, OFFSET>,
            OnRowsetChange: OnRowsetChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetNotify as windows_core::Interface>::IID
    }
}
pub trait IRowsetPrioritization_Impl: Sized {
    fn SetScopePriority(&self, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> windows_core::Result<()>;
    fn GetScopePriority(&self, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> windows_core::Result<()>;
    fn GetScopeStatistics(&self, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetPrioritization {}
impl IRowsetPrioritization_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetPrioritization_Vtbl
    where
        Identity: IRowsetPrioritization_Impl,
    {
        unsafe extern "system" fn SetScopePriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> windows_core::HRESULT
        where
            Identity: IRowsetPrioritization_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetPrioritization_Impl::SetScopePriority(this, core::mem::transmute_copy(&priority), core::mem::transmute_copy(&scopestatisticseventfrequency)).into()
        }
        unsafe extern "system" fn GetScopePriority<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetPrioritization_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetPrioritization_Impl::GetScopePriority(this, core::mem::transmute_copy(&priority), core::mem::transmute_copy(&scopestatisticseventfrequency)).into()
        }
        unsafe extern "system" fn GetScopeStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetPrioritization_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetPrioritization_Impl::GetScopeStatistics(this, core::mem::transmute_copy(&indexeddocumentcount), core::mem::transmute_copy(&oustandingaddcount), core::mem::transmute_copy(&oustandingmodifycount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetScopePriority: SetScopePriority::<Identity, OFFSET>,
            GetScopePriority: GetScopePriority::<Identity, OFFSET>,
            GetScopeStatistics: GetScopeStatistics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetPrioritization as windows_core::Interface>::IID
    }
}
pub trait IRowsetQueryStatus_Impl: Sized {
    fn GetStatus(&self, pdwstatus: *mut u32) -> windows_core::Result<()>;
    fn GetStatusEx(&self, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetQueryStatus {}
impl IRowsetQueryStatus_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetQueryStatus_Vtbl
    where
        Identity: IRowsetQueryStatus_Impl,
    {
        unsafe extern "system" fn GetStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetQueryStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetQueryStatus_Impl::GetStatus(this, core::mem::transmute_copy(&pdwstatus)).into()
        }
        unsafe extern "system" fn GetStatusEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetQueryStatus_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetQueryStatus_Impl::GetStatusEx(this, core::mem::transmute_copy(&pdwstatus), core::mem::transmute_copy(&pcfiltereddocuments), core::mem::transmute_copy(&pcdocumentstofilter), core::mem::transmute_copy(&pdwratiofinisheddenominator), core::mem::transmute_copy(&pdwratiofinishednumerator), core::mem::transmute_copy(&cbbmk), core::mem::transmute_copy(&pbmk), core::mem::transmute_copy(&pirowbmk), core::mem::transmute_copy(&pcrowstotal)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetStatusEx: GetStatusEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetQueryStatus as windows_core::Interface>::IID
    }
}
pub trait IRowsetRefresh_Impl: Sized {
    fn RefreshVisibleData(&self, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: super::super::Foundation::BOOL, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::Result<()>;
    fn GetLastVisibleData(&self, hrow: usize, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetRefresh {}
impl IRowsetRefresh_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetRefresh_Vtbl
    where
        Identity: IRowsetRefresh_Impl,
    {
        unsafe extern "system" fn RefreshVisibleData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: super::super::Foundation::BOOL, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetRefresh_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetRefresh_Impl::RefreshVisibleData(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&foverwrite), core::mem::transmute_copy(&pcrowsrefreshed), core::mem::transmute_copy(&prghrowsrefreshed), core::mem::transmute_copy(&prgrowstatus)).into()
        }
        unsafe extern "system" fn GetLastVisibleData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetRefresh_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetRefresh_Impl::GetLastVisibleData(this, core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RefreshVisibleData: RefreshVisibleData::<Identity, OFFSET>,
            GetLastVisibleData: GetLastVisibleData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetRefresh as windows_core::Interface>::IID
    }
}
pub trait IRowsetResynch_Impl: Sized {
    fn GetVisibleData(&self, hrow: usize, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ResynchRows(&self, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetResynch {}
impl IRowsetResynch_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetResynch_Vtbl
    where
        Identity: IRowsetResynch_Impl,
    {
        unsafe extern "system" fn GetVisibleData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetResynch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetResynch_Impl::GetVisibleData(this, core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn ResynchRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetResynch_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetResynch_Impl::ResynchRows(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&pcrowsresynched), core::mem::transmute_copy(&prghrowsresynched), core::mem::transmute_copy(&prgrowstatus)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVisibleData: GetVisibleData::<Identity, OFFSET>,
            ResynchRows: ResynchRows::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetResynch as windows_core::Interface>::IID
    }
}
pub trait IRowsetScroll_Impl: Sized + IRowsetLocate_Impl {
    fn GetApproximatePosition(&self, hreserved: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> windows_core::Result<()>;
    fn GetRowsAtRatio(&self, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetScroll {}
impl IRowsetScroll_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetScroll_Vtbl
    where
        Identity: IRowsetScroll_Impl,
    {
        unsafe extern "system" fn GetApproximatePosition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetScroll_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetScroll_Impl::GetApproximatePosition(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark), core::mem::transmute_copy(&pulposition), core::mem::transmute_copy(&pcrows)).into()
        }
        unsafe extern "system" fn GetRowsAtRatio<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetScroll_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetScroll_Impl::GetRowsAtRatio(this, core::mem::transmute_copy(&hreserved1), core::mem::transmute_copy(&hreserved2), core::mem::transmute_copy(&ulnumerator), core::mem::transmute_copy(&uldenominator), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&pcrowsobtained), core::mem::transmute_copy(&prghrows)).into()
        }
        Self {
            base__: IRowsetLocate_Vtbl::new::<Identity, OFFSET>(),
            GetApproximatePosition: GetApproximatePosition::<Identity, OFFSET>,
            GetRowsAtRatio: GetRowsAtRatio::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetScroll as windows_core::Interface>::IID || iid == &<IRowset as windows_core::Interface>::IID || iid == &<IRowsetLocate as windows_core::Interface>::IID
    }
}
pub trait IRowsetUpdate_Impl: Sized + IRowsetChange_Impl {
    fn GetOriginalData(&self, hrow: usize, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPendingRows(&self, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> windows_core::Result<()>;
    fn GetRowStatus(&self, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> windows_core::Result<()>;
    fn Undo(&self, hreserved: usize, crows: usize, rghrows: *const usize, pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::Result<()>;
    fn Update(&self, hreserved: usize, crows: usize, rghrows: *const usize, pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetUpdate {}
impl IRowsetUpdate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetUpdate_Vtbl
    where
        Identity: IRowsetUpdate_Impl,
    {
        unsafe extern "system" fn GetOriginalData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetUpdate_Impl::GetOriginalData(this, core::mem::transmute_copy(&hrow), core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetPendingRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetUpdate_Impl::GetPendingRows(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&dwrowstatus), core::mem::transmute_copy(&pcpendingrows), core::mem::transmute_copy(&prgpendingrows), core::mem::transmute_copy(&prgpendingstatus)).into()
        }
        unsafe extern "system" fn GetRowStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetUpdate_Impl::GetRowStatus(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&rgpendingstatus)).into()
        }
        unsafe extern "system" fn Undo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetUpdate_Impl::Undo(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&pcrowsundone), core::mem::transmute_copy(&prgrowsundone), core::mem::transmute_copy(&prgrowstatus)).into()
        }
        unsafe extern "system" fn Update<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: IRowsetUpdate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetUpdate_Impl::Update(this, core::mem::transmute_copy(&hreserved), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rghrows), core::mem::transmute_copy(&pcrows), core::mem::transmute_copy(&prgrows), core::mem::transmute_copy(&prgrowstatus)).into()
        }
        Self {
            base__: IRowsetChange_Vtbl::new::<Identity, OFFSET>(),
            GetOriginalData: GetOriginalData::<Identity, OFFSET>,
            GetPendingRows: GetPendingRows::<Identity, OFFSET>,
            GetRowStatus: GetRowStatus::<Identity, OFFSET>,
            Undo: Undo::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetUpdate as windows_core::Interface>::IID || iid == &<IRowsetChange as windows_core::Interface>::IID
    }
}
pub trait IRowsetView_Impl: Sized {
    fn CreateView(&self, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetView(&self, hchapter: usize, riid: *const windows_core::GUID, phchaptersource: *mut usize, ppview: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetView {}
impl IRowsetView_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetView_Vtbl
    where
        Identity: IRowsetView_Impl,
    {
        unsafe extern "system" fn CreateView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetView_Impl::CreateView(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppview.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetView<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hchapter: usize, riid: *const windows_core::GUID, phchaptersource: *mut usize, ppview: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetView_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetView_Impl::GetView(this, core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&phchaptersource), core::mem::transmute_copy(&ppview)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateView: CreateView::<Identity, OFFSET>, GetView: GetView::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetView as windows_core::Interface>::IID
    }
}
pub trait IRowsetWatchAll_Impl: Sized {
    fn Acknowledge(&self) -> windows_core::Result<()>;
    fn Start(&self) -> windows_core::Result<()>;
    fn StopWatching(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetWatchAll {}
impl IRowsetWatchAll_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetWatchAll_Vtbl
    where
        Identity: IRowsetWatchAll_Impl,
    {
        unsafe extern "system" fn Acknowledge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetWatchAll_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWatchAll_Impl::Acknowledge(this).into()
        }
        unsafe extern "system" fn Start<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetWatchAll_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWatchAll_Impl::Start(this).into()
        }
        unsafe extern "system" fn StopWatching<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRowsetWatchAll_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWatchAll_Impl::StopWatching(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Acknowledge: Acknowledge::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            StopWatching: StopWatching::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetWatchAll as windows_core::Interface>::IID
    }
}
pub trait IRowsetWatchNotify_Impl: Sized {
    fn OnChange(&self, prowset: Option<&IRowset>, echangereason: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetWatchNotify {}
impl IRowsetWatchNotify_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetWatchNotify_Vtbl
    where
        Identity: IRowsetWatchNotify_Impl,
    {
        unsafe extern "system" fn OnChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prowset: *mut core::ffi::c_void, echangereason: u32) -> windows_core::HRESULT
        where
            Identity: IRowsetWatchNotify_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWatchNotify_Impl::OnChange(this, windows_core::from_raw_borrowed(&prowset), core::mem::transmute_copy(&echangereason)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetWatchNotify as windows_core::Interface>::IID
    }
}
pub trait IRowsetWatchRegion_Impl: Sized + IRowsetWatchAll_Impl {
    fn CreateWatchRegion(&self, dwwatchmode: u32) -> windows_core::Result<usize>;
    fn ChangeWatchMode(&self, hregion: usize, dwwatchmode: u32) -> windows_core::Result<()>;
    fn DeleteWatchRegion(&self, hregion: usize) -> windows_core::Result<()>;
    fn GetWatchRegionInfo(&self, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> windows_core::Result<()>;
    fn Refresh(&self, pcchangesobtained: *mut usize, prgchanges: *mut *mut DBROWWATCHCHANGE) -> windows_core::Result<()>;
    fn ShrinkWatchRegion(&self, hregion: usize, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, crows: isize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRowsetWatchRegion {}
impl IRowsetWatchRegion_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetWatchRegion_Vtbl
    where
        Identity: IRowsetWatchRegion_Impl,
    {
        unsafe extern "system" fn CreateWatchRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwatchmode: u32, phregion: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetWatchRegion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRowsetWatchRegion_Impl::CreateWatchRegion(this, core::mem::transmute_copy(&dwwatchmode)) {
                Ok(ok__) => {
                    phregion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeWatchMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hregion: usize, dwwatchmode: u32) -> windows_core::HRESULT
        where
            Identity: IRowsetWatchRegion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWatchRegion_Impl::ChangeWatchMode(this, core::mem::transmute_copy(&hregion), core::mem::transmute_copy(&dwwatchmode)).into()
        }
        unsafe extern "system" fn DeleteWatchRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hregion: usize) -> windows_core::HRESULT
        where
            Identity: IRowsetWatchRegion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWatchRegion_Impl::DeleteWatchRegion(this, core::mem::transmute_copy(&hregion)).into()
        }
        unsafe extern "system" fn GetWatchRegionInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> windows_core::HRESULT
        where
            Identity: IRowsetWatchRegion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWatchRegion_Impl::GetWatchRegionInfo(this, core::mem::transmute_copy(&hregion), core::mem::transmute_copy(&pdwwatchmode), core::mem::transmute_copy(&phchapter), core::mem::transmute_copy(&pcbbookmark), core::mem::transmute_copy(&ppbookmark), core::mem::transmute_copy(&pcrows)).into()
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchangesobtained: *mut usize, prgchanges: *mut *mut DBROWWATCHCHANGE) -> windows_core::HRESULT
        where
            Identity: IRowsetWatchRegion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWatchRegion_Impl::Refresh(this, core::mem::transmute_copy(&pcchangesobtained), core::mem::transmute_copy(&prgchanges)).into()
        }
        unsafe extern "system" fn ShrinkWatchRegion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hregion: usize, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, crows: isize) -> windows_core::HRESULT
        where
            Identity: IRowsetWatchRegion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWatchRegion_Impl::ShrinkWatchRegion(this, core::mem::transmute_copy(&hregion), core::mem::transmute_copy(&hchapter), core::mem::transmute_copy(&cbbookmark), core::mem::transmute_copy(&pbookmark), core::mem::transmute_copy(&crows)).into()
        }
        Self {
            base__: IRowsetWatchAll_Vtbl::new::<Identity, OFFSET>(),
            CreateWatchRegion: CreateWatchRegion::<Identity, OFFSET>,
            ChangeWatchMode: ChangeWatchMode::<Identity, OFFSET>,
            DeleteWatchRegion: DeleteWatchRegion::<Identity, OFFSET>,
            GetWatchRegionInfo: GetWatchRegionInfo::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            ShrinkWatchRegion: ShrinkWatchRegion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetWatchRegion as windows_core::Interface>::IID || iid == &<IRowsetWatchAll as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IRowsetWithParameters_Impl: Sized {
    fn GetParameterInfo(&self, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> windows_core::Result<()>;
    fn Requery(&self, pparams: *const DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IRowsetWithParameters {}
#[cfg(feature = "Win32_System_Com")]
impl IRowsetWithParameters_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRowsetWithParameters_Vtbl
    where
        Identity: IRowsetWithParameters_Impl,
    {
        unsafe extern "system" fn GetParameterInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IRowsetWithParameters_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWithParameters_Impl::GetParameterInfo(this, core::mem::transmute_copy(&pcparams), core::mem::transmute_copy(&prgparaminfo), core::mem::transmute_copy(&ppnamesbuffer)).into()
        }
        unsafe extern "system" fn Requery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *const DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> windows_core::HRESULT
        where
            Identity: IRowsetWithParameters_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRowsetWithParameters_Impl::Requery(this, core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&pulerrorparam), core::mem::transmute_copy(&phreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameterInfo: GetParameterInfo::<Identity, OFFSET>,
            Requery: Requery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRowsetWithParameters as windows_core::Interface>::IID
    }
}
pub trait ISQLErrorInfo_Impl: Sized {
    fn GetSQLInfo(&self, pbstrsqlstate: *mut windows_core::BSTR, plnativeerror: *mut i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISQLErrorInfo {}
impl ISQLErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISQLErrorInfo_Vtbl
    where
        Identity: ISQLErrorInfo_Impl,
    {
        unsafe extern "system" fn GetSQLInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsqlstate: *mut core::mem::MaybeUninit<windows_core::BSTR>, plnativeerror: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISQLErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISQLErrorInfo_Impl::GetSQLInfo(this, core::mem::transmute_copy(&pbstrsqlstate), core::mem::transmute_copy(&plnativeerror)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSQLInfo: GetSQLInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISQLErrorInfo as windows_core::Interface>::IID
    }
}
pub trait ISQLGetDiagField_Impl: Sized {
    fn GetDiagField(&self, pdiaginfo: *mut KAGGETDIAG) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISQLGetDiagField {}
impl ISQLGetDiagField_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISQLGetDiagField_Vtbl
    where
        Identity: ISQLGetDiagField_Impl,
    {
        unsafe extern "system" fn GetDiagField<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdiaginfo: *mut KAGGETDIAG) -> windows_core::HRESULT
        where
            Identity: ISQLGetDiagField_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISQLGetDiagField_Impl::GetDiagField(this, core::mem::transmute_copy(&pdiaginfo)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDiagField: GetDiagField::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISQLGetDiagField as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Variant")]
pub trait ISQLRequestDiagFields_Impl: Sized {
    fn RequestDiagFields(&self, cdiagfields: u32, rgdiagfields: *const KAGREQDIAG) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Variant")]
impl windows_core::RuntimeName for ISQLRequestDiagFields {}
#[cfg(feature = "Win32_System_Variant")]
impl ISQLRequestDiagFields_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISQLRequestDiagFields_Vtbl
    where
        Identity: ISQLRequestDiagFields_Impl,
    {
        unsafe extern "system" fn RequestDiagFields<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdiagfields: u32, rgdiagfields: *const KAGREQDIAG) -> windows_core::HRESULT
        where
            Identity: ISQLRequestDiagFields_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISQLRequestDiagFields_Impl::RequestDiagFields(this, core::mem::transmute_copy(&cdiagfields), core::mem::transmute_copy(&rgdiagfields)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RequestDiagFields: RequestDiagFields::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISQLRequestDiagFields as windows_core::Interface>::IID
    }
}
pub trait ISQLServerErrorInfo_Impl: Sized {
    fn GetErrorInfo(&self, pperrorinfo: *mut *mut SSERRORINFO, ppstringsbuffer: *mut *mut u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISQLServerErrorInfo {}
impl ISQLServerErrorInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISQLServerErrorInfo_Vtbl
    where
        Identity: ISQLServerErrorInfo_Impl,
    {
        unsafe extern "system" fn GetErrorInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperrorinfo: *mut *mut SSERRORINFO, ppstringsbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: ISQLServerErrorInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISQLServerErrorInfo_Impl::GetErrorInfo(this, core::mem::transmute_copy(&pperrorinfo), core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetErrorInfo: GetErrorInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISQLServerErrorInfo as windows_core::Interface>::IID
    }
}
pub trait ISchemaLocalizerSupport_Impl: Sized {
    fn Localize(&self, pszglobalstring: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for ISchemaLocalizerSupport {}
impl ISchemaLocalizerSupport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISchemaLocalizerSupport_Vtbl
    where
        Identity: ISchemaLocalizerSupport_Impl,
    {
        unsafe extern "system" fn Localize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszglobalstring: windows_core::PCWSTR, ppszlocalstring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISchemaLocalizerSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISchemaLocalizerSupport_Impl::Localize(this, core::mem::transmute(&pszglobalstring)) {
                Ok(ok__) => {
                    ppszlocalstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Localize: Localize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaLocalizerSupport as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait ISchemaLock_Impl: Sized {
    fn GetSchemaLock(&self, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> windows_core::Result<()>;
    fn ReleaseSchemaLock(&self, hlockhandle: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for ISchemaLock {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ISchemaLock_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISchemaLock_Vtbl
    where
        Identity: ISchemaLock_Impl,
    {
        unsafe extern "system" fn GetSchemaLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> windows_core::HRESULT
        where
            Identity: ISchemaLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchemaLock_Impl::GetSchemaLock(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&lmmode), core::mem::transmute_copy(&phlockhandle), core::mem::transmute_copy(&ptableversion)).into()
        }
        unsafe extern "system" fn ReleaseSchemaLock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hlockhandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: ISchemaLock_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchemaLock_Impl::ReleaseSchemaLock(this, core::mem::transmute_copy(&hlockhandle)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSchemaLock: GetSchemaLock::<Identity, OFFSET>,
            ReleaseSchemaLock: ReleaseSchemaLock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaLock as windows_core::Interface>::IID
    }
}
pub trait ISchemaProvider_Impl: Sized {
    fn Entities(&self, riid: *const windows_core::GUID, pentities: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RootEntity(&self) -> windows_core::Result<IEntity>;
    fn GetEntity(&self, pszentityname: &windows_core::PCWSTR) -> windows_core::Result<IEntity>;
    fn MetaData(&self, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Localize(&self, lcid: u32, pschemalocalizersupport: Option<&ISchemaLocalizerSupport>) -> windows_core::Result<()>;
    fn SaveBinary(&self, pszschemabinarypath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LookupAuthoredNamedEntity(&self, pentity: Option<&IEntity>, pszinputstring: &windows_core::PCWSTR, ptokencollection: Option<&ITokenCollection>, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISchemaProvider {}
impl ISchemaProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISchemaProvider_Vtbl
    where
        Identity: ISchemaProvider_Impl,
    {
        unsafe extern "system" fn Entities<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pentities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISchemaProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchemaProvider_Impl::Entities(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pentities)).into()
        }
        unsafe extern "system" fn RootEntity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prootentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISchemaProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISchemaProvider_Impl::RootEntity(this) {
                Ok(ok__) => {
                    prootentity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEntity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszentityname: windows_core::PCWSTR, pentity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISchemaProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISchemaProvider_Impl::GetEntity(this, core::mem::transmute(&pszentityname)) {
                Ok(ok__) => {
                    pentity.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetaData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISchemaProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchemaProvider_Impl::MetaData(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pmetadata)).into()
        }
        unsafe extern "system" fn Localize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32, pschemalocalizersupport: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISchemaProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchemaProvider_Impl::Localize(this, core::mem::transmute_copy(&lcid), windows_core::from_raw_borrowed(&pschemalocalizersupport)).into()
        }
        unsafe extern "system" fn SaveBinary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszschemabinarypath: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISchemaProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchemaProvider_Impl::SaveBinary(this, core::mem::transmute(&pszschemabinarypath)).into()
        }
        unsafe extern "system" fn LookupAuthoredNamedEntity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pentity: *mut core::ffi::c_void, pszinputstring: windows_core::PCWSTR, ptokencollection: *mut core::ffi::c_void, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISchemaProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISchemaProvider_Impl::LookupAuthoredNamedEntity(this, windows_core::from_raw_borrowed(&pentity), core::mem::transmute(&pszinputstring), windows_core::from_raw_borrowed(&ptokencollection), core::mem::transmute_copy(&ctokensbegin), core::mem::transmute_copy(&pctokenslength), core::mem::transmute_copy(&ppszvalue)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Entities: Entities::<Identity, OFFSET>,
            RootEntity: RootEntity::<Identity, OFFSET>,
            GetEntity: GetEntity::<Identity, OFFSET>,
            MetaData: MetaData::<Identity, OFFSET>,
            Localize: Localize::<Identity, OFFSET>,
            SaveBinary: SaveBinary::<Identity, OFFSET>,
            LookupAuthoredNamedEntity: LookupAuthoredNamedEntity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISchemaProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IScopedOperations_Impl: Sized + IBindResource_Impl {
    fn Copy(&self, crows: usize, rgpwszsourceurls: *const windows_core::PCWSTR, rgpwszdesturls: *const windows_core::PCWSTR, dwcopyflags: u32, pauthenticate: Option<&super::Com::IAuthenticate>, rgdwstatus: *mut u32, rgpwsznewurls: *mut windows_core::PWSTR, ppstringsbuffer: *mut *mut u16) -> windows_core::Result<()>;
    fn Move(&self, crows: usize, rgpwszsourceurls: *const windows_core::PCWSTR, rgpwszdesturls: *const windows_core::PCWSTR, dwmoveflags: u32, pauthenticate: Option<&super::Com::IAuthenticate>, rgdwstatus: *mut u32, rgpwsznewurls: *mut windows_core::PWSTR, ppstringsbuffer: *mut *mut u16) -> windows_core::Result<()>;
    fn Delete(&self, crows: usize, rgpwszurls: *const windows_core::PCWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> windows_core::Result<()>;
    fn OpenRowset(&self, punkouter: Option<&windows_core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for IScopedOperations {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl IScopedOperations_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IScopedOperations_Vtbl
    where
        Identity: IScopedOperations_Impl,
    {
        unsafe extern "system" fn Copy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: usize, rgpwszsourceurls: *const windows_core::PCWSTR, rgpwszdesturls: *const windows_core::PCWSTR, dwcopyflags: u32, pauthenticate: *mut core::ffi::c_void, rgdwstatus: *mut u32, rgpwsznewurls: *mut windows_core::PWSTR, ppstringsbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IScopedOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScopedOperations_Impl::Copy(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rgpwszsourceurls), core::mem::transmute_copy(&rgpwszdesturls), core::mem::transmute_copy(&dwcopyflags), windows_core::from_raw_borrowed(&pauthenticate), core::mem::transmute_copy(&rgdwstatus), core::mem::transmute_copy(&rgpwsznewurls), core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: usize, rgpwszsourceurls: *const windows_core::PCWSTR, rgpwszdesturls: *const windows_core::PCWSTR, dwmoveflags: u32, pauthenticate: *mut core::ffi::c_void, rgdwstatus: *mut u32, rgpwsznewurls: *mut windows_core::PWSTR, ppstringsbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: IScopedOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScopedOperations_Impl::Move(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rgpwszsourceurls), core::mem::transmute_copy(&rgpwszdesturls), core::mem::transmute_copy(&dwmoveflags), windows_core::from_raw_borrowed(&pauthenticate), core::mem::transmute_copy(&rgdwstatus), core::mem::transmute_copy(&rgpwsznewurls), core::mem::transmute_copy(&ppstringsbuffer)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, crows: usize, rgpwszurls: *const windows_core::PCWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> windows_core::HRESULT
        where
            Identity: IScopedOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScopedOperations_Impl::Delete(this, core::mem::transmute_copy(&crows), core::mem::transmute_copy(&rgpwszurls), core::mem::transmute_copy(&dwdeleteflags), core::mem::transmute_copy(&rgdwstatus)).into()
        }
        unsafe extern "system" fn OpenRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IScopedOperations_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IScopedOperations_Impl::OpenRowset(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pindexid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pprowset)).into()
        }
        Self {
            base__: IBindResource_Vtbl::new::<Identity, OFFSET>(),
            Copy: Copy::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            OpenRowset: OpenRowset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScopedOperations as windows_core::Interface>::IID || iid == &<IBindResource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchCatalogManager_Impl: Sized {
    fn Name(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetParameter(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<*mut windows_core::PROPVARIANT>;
    fn SetParameter(&self, pszname: &windows_core::PCWSTR, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetCatalogStatus(&self, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Reindex(&self) -> windows_core::Result<()>;
    fn ReindexMatchingURLs(&self, pszpattern: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ReindexSearchRoot(&self, pszrooturl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetConnectTimeout(&self, dwconnecttimeout: u32) -> windows_core::Result<()>;
    fn ConnectTimeout(&self) -> windows_core::Result<u32>;
    fn SetDataTimeout(&self, dwdatatimeout: u32) -> windows_core::Result<()>;
    fn DataTimeout(&self) -> windows_core::Result<u32>;
    fn NumberOfItems(&self) -> windows_core::Result<i32>;
    fn NumberOfItemsToIndex(&self, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> windows_core::Result<()>;
    fn URLBeingIndexed(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetURLIndexingState(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn GetPersistentItemsChangedSink(&self) -> windows_core::Result<ISearchPersistentItemsChangedSink>;
    fn RegisterViewForNotification(&self, pszview: &windows_core::PCWSTR, pviewchangedsink: Option<&ISearchViewChangedSink>) -> windows_core::Result<u32>;
    fn GetItemsChangedSink(&self, pisearchnotifyinlinesite: Option<&ISearchNotifyInlineSite>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void, pguidcatalogresetsignature: *mut windows_core::GUID, pguidcheckpointsignature: *mut windows_core::GUID, pdwlastcheckpointnumber: *mut u32) -> windows_core::Result<()>;
    fn UnregisterViewForNotification(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn SetExtensionClusion(&self, pszextension: &windows_core::PCWSTR, fexclude: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnumerateExcludedExtensions(&self) -> windows_core::Result<super::Com::IEnumString>;
    fn GetQueryHelper(&self) -> windows_core::Result<ISearchQueryHelper>;
    fn SetDiacriticSensitivity(&self, fdiacriticsensitive: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn DiacriticSensitivity(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetCrawlScopeManager(&self) -> windows_core::Result<ISearchCrawlScopeManager>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISearchCatalogManager {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchCatalogManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchCatalogManager_Vtbl
    where
        Identity: ISearchCatalogManager_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::Name(this) {
                Ok(ok__) => {
                    pszname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ppvalue: *mut *mut windows_core::PROPVARIANT) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::GetParameter(this, core::mem::transmute(&pszname)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::SetParameter(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetCatalogStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::GetCatalogStatus(this, core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&ppausedreason)).into()
        }
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Reindex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::Reindex(this).into()
        }
        unsafe extern "system" fn ReindexMatchingURLs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpattern: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::ReindexMatchingURLs(this, core::mem::transmute(&pszpattern)).into()
        }
        unsafe extern "system" fn ReindexSearchRoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrooturl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::ReindexSearchRoot(this, core::mem::transmute(&pszrooturl)).into()
        }
        unsafe extern "system" fn SetConnectTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnecttimeout: u32) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::SetConnectTimeout(this, core::mem::transmute_copy(&dwconnecttimeout)).into()
        }
        unsafe extern "system" fn ConnectTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconnecttimeout: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::ConnectTimeout(this) {
                Ok(ok__) => {
                    pdwconnecttimeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdatatimeout: u32) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::SetDataTimeout(this, core::mem::transmute_copy(&dwdatatimeout)).into()
        }
        unsafe extern "system" fn DataTimeout<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdatatimeout: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::DataTimeout(this) {
                Ok(ok__) => {
                    pdwdatatimeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::NumberOfItems(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfItemsToIndex<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::NumberOfItemsToIndex(this, core::mem::transmute_copy(&plincrementalcount), core::mem::transmute_copy(&plnotificationqueue), core::mem::transmute_copy(&plhighpriorityqueue)).into()
        }
        unsafe extern "system" fn URLBeingIndexed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::URLBeingIndexed(this) {
                Ok(ok__) => {
                    pszurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetURLIndexingState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pdwstate: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::GetURLIndexingState(this, core::mem::transmute(&pszurl)) {
                Ok(ok__) => {
                    pdwstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPersistentItemsChangedSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppisearchpersistentitemschangedsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::GetPersistentItemsChangedSink(this) {
                Ok(ok__) => {
                    ppisearchpersistentitemschangedsink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterViewForNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszview: windows_core::PCWSTR, pviewchangedsink: *mut core::ffi::c_void, pdwcookie: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::RegisterViewForNotification(this, core::mem::transmute(&pszview), windows_core::from_raw_borrowed(&pviewchangedsink)) {
                Ok(ok__) => {
                    pdwcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemsChangedSink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisearchnotifyinlinesite: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void, pguidcatalogresetsignature: *mut windows_core::GUID, pguidcheckpointsignature: *mut windows_core::GUID, pdwlastcheckpointnumber: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::GetItemsChangedSink(this, windows_core::from_raw_borrowed(&pisearchnotifyinlinesite), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv), core::mem::transmute_copy(&pguidcatalogresetsignature), core::mem::transmute_copy(&pguidcheckpointsignature), core::mem::transmute_copy(&pdwlastcheckpointnumber)).into()
        }
        unsafe extern "system" fn UnregisterViewForNotification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::UnregisterViewForNotification(this, core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn SetExtensionClusion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszextension: windows_core::PCWSTR, fexclude: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::SetExtensionClusion(this, core::mem::transmute(&pszextension), core::mem::transmute_copy(&fexclude)).into()
        }
        unsafe extern "system" fn EnumerateExcludedExtensions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppextensions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::EnumerateExcludedExtensions(this) {
                Ok(ok__) => {
                    ppextensions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueryHelper<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsearchqueryhelper: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::GetQueryHelper(this) {
                Ok(ok__) => {
                    ppsearchqueryhelper.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiacriticSensitivity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdiacriticsensitive: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager_Impl::SetDiacriticSensitivity(this, core::mem::transmute_copy(&fdiacriticsensitive)).into()
        }
        unsafe extern "system" fn DiacriticSensitivity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::DiacriticSensitivity(this) {
                Ok(ok__) => {
                    pfdiacriticsensitive.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCrawlScopeManager<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcrawlscopemanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCatalogManager_Impl::GetCrawlScopeManager(this) {
                Ok(ok__) => {
                    ppcrawlscopemanager.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            GetParameter: GetParameter::<Identity, OFFSET>,
            SetParameter: SetParameter::<Identity, OFFSET>,
            GetCatalogStatus: GetCatalogStatus::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Reindex: Reindex::<Identity, OFFSET>,
            ReindexMatchingURLs: ReindexMatchingURLs::<Identity, OFFSET>,
            ReindexSearchRoot: ReindexSearchRoot::<Identity, OFFSET>,
            SetConnectTimeout: SetConnectTimeout::<Identity, OFFSET>,
            ConnectTimeout: ConnectTimeout::<Identity, OFFSET>,
            SetDataTimeout: SetDataTimeout::<Identity, OFFSET>,
            DataTimeout: DataTimeout::<Identity, OFFSET>,
            NumberOfItems: NumberOfItems::<Identity, OFFSET>,
            NumberOfItemsToIndex: NumberOfItemsToIndex::<Identity, OFFSET>,
            URLBeingIndexed: URLBeingIndexed::<Identity, OFFSET>,
            GetURLIndexingState: GetURLIndexingState::<Identity, OFFSET>,
            GetPersistentItemsChangedSink: GetPersistentItemsChangedSink::<Identity, OFFSET>,
            RegisterViewForNotification: RegisterViewForNotification::<Identity, OFFSET>,
            GetItemsChangedSink: GetItemsChangedSink::<Identity, OFFSET>,
            UnregisterViewForNotification: UnregisterViewForNotification::<Identity, OFFSET>,
            SetExtensionClusion: SetExtensionClusion::<Identity, OFFSET>,
            EnumerateExcludedExtensions: EnumerateExcludedExtensions::<Identity, OFFSET>,
            GetQueryHelper: GetQueryHelper::<Identity, OFFSET>,
            SetDiacriticSensitivity: SetDiacriticSensitivity::<Identity, OFFSET>,
            DiacriticSensitivity: DiacriticSensitivity::<Identity, OFFSET>,
            GetCrawlScopeManager: GetCrawlScopeManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCatalogManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchCatalogManager2_Impl: Sized + ISearchCatalogManager_Impl {
    fn PrioritizeMatchingURLs(&self, pszpattern: &windows_core::PCWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISearchCatalogManager2 {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchCatalogManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchCatalogManager2_Vtbl
    where
        Identity: ISearchCatalogManager2_Impl,
    {
        unsafe extern "system" fn PrioritizeMatchingURLs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpattern: windows_core::PCWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> windows_core::HRESULT
        where
            Identity: ISearchCatalogManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCatalogManager2_Impl::PrioritizeMatchingURLs(this, core::mem::transmute(&pszpattern), core::mem::transmute_copy(&dwprioritizeflags)).into()
        }
        Self { base__: ISearchCatalogManager_Vtbl::new::<Identity, OFFSET>(), PrioritizeMatchingURLs: PrioritizeMatchingURLs::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCatalogManager2 as windows_core::Interface>::IID || iid == &<ISearchCatalogManager as windows_core::Interface>::IID
    }
}
pub trait ISearchCrawlScopeManager_Impl: Sized {
    fn AddDefaultScopeRule(&self, pszurl: &windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, ffollowflags: u32) -> windows_core::Result<()>;
    fn AddRoot(&self, psearchroot: Option<&ISearchRoot>) -> windows_core::Result<()>;
    fn RemoveRoot(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumerateRoots(&self) -> windows_core::Result<IEnumSearchRoots>;
    fn AddHierarchicalScope(&self, pszurl: &windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn AddUserScopeRule(&self, pszurl: &windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL, ffollowflags: u32) -> windows_core::Result<()>;
    fn RemoveScopeRule(&self, pszrule: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumerateScopeRules(&self) -> windows_core::Result<IEnumSearchScopeRules>;
    fn HasParentScopeRule(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn HasChildScopeRule(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IncludedInCrawlScope(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IncludedInCrawlScopeEx(&self, pszurl: &windows_core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> windows_core::Result<()>;
    fn RevertToDefaultScopes(&self) -> windows_core::Result<()>;
    fn SaveAll(&self) -> windows_core::Result<()>;
    fn GetParentScopeVersionId(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<i32>;
    fn RemoveDefaultScopeRule(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISearchCrawlScopeManager {}
impl ISearchCrawlScopeManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchCrawlScopeManager_Vtbl
    where
        Identity: ISearchCrawlScopeManager_Impl,
    {
        unsafe extern "system" fn AddDefaultScopeRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, ffollowflags: u32) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager_Impl::AddDefaultScopeRule(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&finclude), core::mem::transmute_copy(&ffollowflags)).into()
        }
        unsafe extern "system" fn AddRoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psearchroot: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager_Impl::AddRoot(this, windows_core::from_raw_borrowed(&psearchroot)).into()
        }
        unsafe extern "system" fn RemoveRoot<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager_Impl::RemoveRoot(this, core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn EnumerateRoots<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsearchroots: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCrawlScopeManager_Impl::EnumerateRoots(this) {
                Ok(ok__) => {
                    ppsearchroots.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddHierarchicalScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager_Impl::AddHierarchicalScope(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&finclude), core::mem::transmute_copy(&fdefault), core::mem::transmute_copy(&foverridechildren)).into()
        }
        unsafe extern "system" fn AddUserScopeRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL, ffollowflags: u32) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager_Impl::AddUserScopeRule(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&finclude), core::mem::transmute_copy(&foverridechildren), core::mem::transmute_copy(&ffollowflags)).into()
        }
        unsafe extern "system" fn RemoveScopeRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrule: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager_Impl::RemoveScopeRule(this, core::mem::transmute(&pszrule)).into()
        }
        unsafe extern "system" fn EnumerateScopeRules<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsearchscoperules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCrawlScopeManager_Impl::EnumerateScopeRules(this) {
                Ok(ok__) => {
                    ppsearchscoperules.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasParentScopeRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pfhasparentrule: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCrawlScopeManager_Impl::HasParentScopeRule(this, core::mem::transmute(&pszurl)) {
                Ok(ok__) => {
                    pfhasparentrule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildScopeRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pfhaschildrule: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCrawlScopeManager_Impl::HasChildScopeRule(this, core::mem::transmute(&pszurl)) {
                Ok(ok__) => {
                    pfhaschildrule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludedInCrawlScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCrawlScopeManager_Impl::IncludedInCrawlScope(this, core::mem::transmute(&pszurl)) {
                Ok(ok__) => {
                    pfisincluded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncludedInCrawlScopeEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager_Impl::IncludedInCrawlScopeEx(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&pfisincluded), core::mem::transmute_copy(&preason)).into()
        }
        unsafe extern "system" fn RevertToDefaultScopes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager_Impl::RevertToDefaultScopes(this).into()
        }
        unsafe extern "system" fn SaveAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager_Impl::SaveAll(this).into()
        }
        unsafe extern "system" fn GetParentScopeVersionId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, plscopeid: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchCrawlScopeManager_Impl::GetParentScopeVersionId(this, core::mem::transmute(&pszurl)) {
                Ok(ok__) => {
                    plscopeid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDefaultScopeRule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager_Impl::RemoveDefaultScopeRule(this, core::mem::transmute(&pszurl)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddDefaultScopeRule: AddDefaultScopeRule::<Identity, OFFSET>,
            AddRoot: AddRoot::<Identity, OFFSET>,
            RemoveRoot: RemoveRoot::<Identity, OFFSET>,
            EnumerateRoots: EnumerateRoots::<Identity, OFFSET>,
            AddHierarchicalScope: AddHierarchicalScope::<Identity, OFFSET>,
            AddUserScopeRule: AddUserScopeRule::<Identity, OFFSET>,
            RemoveScopeRule: RemoveScopeRule::<Identity, OFFSET>,
            EnumerateScopeRules: EnumerateScopeRules::<Identity, OFFSET>,
            HasParentScopeRule: HasParentScopeRule::<Identity, OFFSET>,
            HasChildScopeRule: HasChildScopeRule::<Identity, OFFSET>,
            IncludedInCrawlScope: IncludedInCrawlScope::<Identity, OFFSET>,
            IncludedInCrawlScopeEx: IncludedInCrawlScopeEx::<Identity, OFFSET>,
            RevertToDefaultScopes: RevertToDefaultScopes::<Identity, OFFSET>,
            SaveAll: SaveAll::<Identity, OFFSET>,
            GetParentScopeVersionId: GetParentScopeVersionId::<Identity, OFFSET>,
            RemoveDefaultScopeRule: RemoveDefaultScopeRule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCrawlScopeManager as windows_core::Interface>::IID
    }
}
pub trait ISearchCrawlScopeManager2_Impl: Sized + ISearchCrawlScopeManager_Impl {
    fn GetVersion(&self, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISearchCrawlScopeManager2 {}
impl ISearchCrawlScopeManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchCrawlScopeManager2_Vtbl
    where
        Identity: ISearchCrawlScopeManager2_Impl,
    {
        unsafe extern "system" fn GetVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT
        where
            Identity: ISearchCrawlScopeManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchCrawlScopeManager2_Impl::GetVersion(this, core::mem::transmute_copy(&plversion), core::mem::transmute_copy(&phfilemapping)).into()
        }
        Self { base__: ISearchCrawlScopeManager_Vtbl::new::<Identity, OFFSET>(), GetVersion: GetVersion::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchCrawlScopeManager2 as windows_core::Interface>::IID || iid == &<ISearchCrawlScopeManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchItemsChangedSink_Impl: Sized {
    fn StartedMonitoringScope(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn StoppedMonitoringScope(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnItemsChanged(&self, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISearchItemsChangedSink {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchItemsChangedSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchItemsChangedSink_Vtbl
    where
        Identity: ISearchItemsChangedSink_Impl,
    {
        unsafe extern "system" fn StartedMonitoringScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchItemsChangedSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchItemsChangedSink_Impl::StartedMonitoringScope(this, core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn StoppedMonitoringScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchItemsChangedSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchItemsChangedSink_Impl::StoppedMonitoringScope(this, core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn OnItemsChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ISearchItemsChangedSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchItemsChangedSink_Impl::OnItemsChanged(this, core::mem::transmute_copy(&dwnumberofchanges), core::mem::transmute_copy(&rgdatachangeentries), core::mem::transmute_copy(&rgdwdocids), core::mem::transmute_copy(&rghrcompletioncodes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartedMonitoringScope: StartedMonitoringScope::<Identity, OFFSET>,
            StoppedMonitoringScope: StoppedMonitoringScope::<Identity, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchItemsChangedSink as windows_core::Interface>::IID
    }
}
pub trait ISearchLanguageSupport_Impl: Sized {
    fn SetDiacriticSensitivity(&self, fdiacriticsensitive: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetDiacriticSensitivity(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn LoadWordBreaker(&self, lcid: u32, riid: *const windows_core::GUID, ppwordbreaker: *mut *mut core::ffi::c_void, plcidused: *mut u32) -> windows_core::Result<()>;
    fn LoadStemmer(&self, lcid: u32, riid: *const windows_core::GUID, ppstemmer: *mut *mut core::ffi::c_void, plcidused: *mut u32) -> windows_core::Result<()>;
    fn IsPrefixNormalized(&self, pwcsquerytoken: &windows_core::PCWSTR, cwcquerytoken: u32, pwcsdocumenttoken: &windows_core::PCWSTR, cwcdocumenttoken: u32) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ISearchLanguageSupport {}
impl ISearchLanguageSupport_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchLanguageSupport_Vtbl
    where
        Identity: ISearchLanguageSupport_Impl,
    {
        unsafe extern "system" fn SetDiacriticSensitivity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdiacriticsensitive: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchLanguageSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchLanguageSupport_Impl::SetDiacriticSensitivity(this, core::mem::transmute_copy(&fdiacriticsensitive)).into()
        }
        unsafe extern "system" fn GetDiacriticSensitivity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchLanguageSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchLanguageSupport_Impl::GetDiacriticSensitivity(this) {
                Ok(ok__) => {
                    pfdiacriticsensitive.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadWordBreaker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32, riid: *const windows_core::GUID, ppwordbreaker: *mut *mut core::ffi::c_void, plcidused: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchLanguageSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchLanguageSupport_Impl::LoadWordBreaker(this, core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppwordbreaker), core::mem::transmute_copy(&plcidused)).into()
        }
        unsafe extern "system" fn LoadStemmer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32, riid: *const windows_core::GUID, ppstemmer: *mut *mut core::ffi::c_void, plcidused: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchLanguageSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchLanguageSupport_Impl::LoadStemmer(this, core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppstemmer), core::mem::transmute_copy(&plcidused)).into()
        }
        unsafe extern "system" fn IsPrefixNormalized<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsquerytoken: windows_core::PCWSTR, cwcquerytoken: u32, pwcsdocumenttoken: windows_core::PCWSTR, cwcdocumenttoken: u32, pulprefixlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchLanguageSupport_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchLanguageSupport_Impl::IsPrefixNormalized(this, core::mem::transmute(&pwcsquerytoken), core::mem::transmute_copy(&cwcquerytoken), core::mem::transmute(&pwcsdocumenttoken), core::mem::transmute_copy(&cwcdocumenttoken)) {
                Ok(ok__) => {
                    pulprefixlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDiacriticSensitivity: SetDiacriticSensitivity::<Identity, OFFSET>,
            GetDiacriticSensitivity: GetDiacriticSensitivity::<Identity, OFFSET>,
            LoadWordBreaker: LoadWordBreaker::<Identity, OFFSET>,
            LoadStemmer: LoadStemmer::<Identity, OFFSET>,
            IsPrefixNormalized: IsPrefixNormalized::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchLanguageSupport as windows_core::Interface>::IID
    }
}
pub trait ISearchManager_Impl: Sized {
    fn GetIndexerVersionStr(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetIndexerVersion(&self, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::Result<()>;
    fn GetParameter(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<*mut windows_core::PROPVARIANT>;
    fn SetParameter(&self, pszname: &windows_core::PCWSTR, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn ProxyName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn BypassList(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetProxy(&self, suseproxy: PROXY_ACCESS, flocalbypassproxy: super::super::Foundation::BOOL, dwportnumber: u32, pszproxyname: &windows_core::PCWSTR, pszbypasslist: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCatalog(&self, pszcatalog: &windows_core::PCWSTR) -> windows_core::Result<ISearchCatalogManager>;
    fn UserAgent(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetUserAgent(&self, pszuseragent: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UseProxy(&self) -> windows_core::Result<PROXY_ACCESS>;
    fn LocalBypass(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn PortNumber(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ISearchManager {}
impl ISearchManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchManager_Vtbl
    where
        Identity: ISearchManager_Impl,
    {
        unsafe extern "system" fn GetIndexerVersionStr<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszversionstring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchManager_Impl::GetIndexerVersionStr(this) {
                Ok(ok__) => {
                    ppszversionstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndexerVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmajor: *mut u32, pdwminor: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchManager_Impl::GetIndexerVersion(this, core::mem::transmute_copy(&pdwmajor), core::mem::transmute_copy(&pdwminor)).into()
        }
        unsafe extern "system" fn GetParameter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, ppvalue: *mut *mut windows_core::PROPVARIANT) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchManager_Impl::GetParameter(this, core::mem::transmute(&pszname)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, pvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchManager_Impl::SetParameter(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ProxyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszproxyname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchManager_Impl::ProxyName(this) {
                Ok(ok__) => {
                    ppszproxyname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BypassList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszbypasslist: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchManager_Impl::BypassList(this) {
                Ok(ok__) => {
                    ppszbypasslist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, suseproxy: PROXY_ACCESS, flocalbypassproxy: super::super::Foundation::BOOL, dwportnumber: u32, pszproxyname: windows_core::PCWSTR, pszbypasslist: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchManager_Impl::SetProxy(this, core::mem::transmute_copy(&suseproxy), core::mem::transmute_copy(&flocalbypassproxy), core::mem::transmute_copy(&dwportnumber), core::mem::transmute(&pszproxyname), core::mem::transmute(&pszbypasslist)).into()
        }
        unsafe extern "system" fn GetCatalog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcatalog: windows_core::PCWSTR, ppcatalogmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchManager_Impl::GetCatalog(this, core::mem::transmute(&pszcatalog)) {
                Ok(ok__) => {
                    ppcatalogmanager.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserAgent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszuseragent: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchManager_Impl::UserAgent(this) {
                Ok(ok__) => {
                    ppszuseragent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserAgent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuseragent: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchManager_Impl::SetUserAgent(this, core::mem::transmute(&pszuseragent)).into()
        }
        unsafe extern "system" fn UseProxy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, puseproxy: *mut PROXY_ACCESS) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchManager_Impl::UseProxy(this) {
                Ok(ok__) => {
                    puseproxy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalBypass<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflocalbypass: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchManager_Impl::LocalBypass(this) {
                Ok(ok__) => {
                    pflocalbypass.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PortNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwportnumber: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchManager_Impl::PortNumber(this) {
                Ok(ok__) => {
                    pdwportnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIndexerVersionStr: GetIndexerVersionStr::<Identity, OFFSET>,
            GetIndexerVersion: GetIndexerVersion::<Identity, OFFSET>,
            GetParameter: GetParameter::<Identity, OFFSET>,
            SetParameter: SetParameter::<Identity, OFFSET>,
            ProxyName: ProxyName::<Identity, OFFSET>,
            BypassList: BypassList::<Identity, OFFSET>,
            SetProxy: SetProxy::<Identity, OFFSET>,
            GetCatalog: GetCatalog::<Identity, OFFSET>,
            UserAgent: UserAgent::<Identity, OFFSET>,
            SetUserAgent: SetUserAgent::<Identity, OFFSET>,
            UseProxy: UseProxy::<Identity, OFFSET>,
            LocalBypass: LocalBypass::<Identity, OFFSET>,
            PortNumber: PortNumber::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchManager as windows_core::Interface>::IID
    }
}
pub trait ISearchManager2_Impl: Sized + ISearchManager_Impl {
    fn CreateCatalog(&self, pszcatalog: &windows_core::PCWSTR) -> windows_core::Result<ISearchCatalogManager>;
    fn DeleteCatalog(&self, pszcatalog: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISearchManager2 {}
impl ISearchManager2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchManager2_Vtbl
    where
        Identity: ISearchManager2_Impl,
    {
        unsafe extern "system" fn CreateCatalog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcatalog: windows_core::PCWSTR, ppcatalogmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchManager2_Impl::CreateCatalog(this, core::mem::transmute(&pszcatalog)) {
                Ok(ok__) => {
                    ppcatalogmanager.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCatalog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcatalog: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchManager2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchManager2_Impl::DeleteCatalog(this, core::mem::transmute(&pszcatalog)).into()
        }
        Self {
            base__: ISearchManager_Vtbl::new::<Identity, OFFSET>(),
            CreateCatalog: CreateCatalog::<Identity, OFFSET>,
            DeleteCatalog: DeleteCatalog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchManager2 as windows_core::Interface>::IID || iid == &<ISearchManager as windows_core::Interface>::IID
    }
}
pub trait ISearchNotifyInlineSite_Impl: Sized {
    fn OnItemIndexedStatusChange(&self, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> windows_core::Result<()>;
    fn OnCatalogStatusChange(&self, guidcatalogresetsignature: *const windows_core::GUID, guidcheckpointsignature: *const windows_core::GUID, dwlastcheckpointnumber: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISearchNotifyInlineSite {}
impl ISearchNotifyInlineSite_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchNotifyInlineSite_Vtbl
    where
        Identity: ISearchNotifyInlineSite_Impl,
    {
        unsafe extern "system" fn OnItemIndexedStatusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> windows_core::HRESULT
        where
            Identity: ISearchNotifyInlineSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchNotifyInlineSite_Impl::OnItemIndexedStatusChange(this, core::mem::transmute_copy(&sipstatus), core::mem::transmute_copy(&dwnumentries), core::mem::transmute_copy(&rgitemstatusentries)).into()
        }
        unsafe extern "system" fn OnCatalogStatusChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidcatalogresetsignature: *const windows_core::GUID, guidcheckpointsignature: *const windows_core::GUID, dwlastcheckpointnumber: u32) -> windows_core::HRESULT
        where
            Identity: ISearchNotifyInlineSite_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchNotifyInlineSite_Impl::OnCatalogStatusChange(this, core::mem::transmute_copy(&guidcatalogresetsignature), core::mem::transmute_copy(&guidcheckpointsignature), core::mem::transmute_copy(&dwlastcheckpointnumber)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnItemIndexedStatusChange: OnItemIndexedStatusChange::<Identity, OFFSET>,
            OnCatalogStatusChange: OnCatalogStatusChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchNotifyInlineSite as windows_core::Interface>::IID
    }
}
pub trait ISearchPersistentItemsChangedSink_Impl: Sized {
    fn StartedMonitoringScope(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn StoppedMonitoringScope(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnItemsChanged(&self, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISearchPersistentItemsChangedSink {}
impl ISearchPersistentItemsChangedSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchPersistentItemsChangedSink_Vtbl
    where
        Identity: ISearchPersistentItemsChangedSink_Impl,
    {
        unsafe extern "system" fn StartedMonitoringScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchPersistentItemsChangedSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchPersistentItemsChangedSink_Impl::StartedMonitoringScope(this, core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn StoppedMonitoringScope<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchPersistentItemsChangedSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchPersistentItemsChangedSink_Impl::StoppedMonitoringScope(this, core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn OnItemsChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut windows_core::HRESULT) -> windows_core::HRESULT
        where
            Identity: ISearchPersistentItemsChangedSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchPersistentItemsChangedSink_Impl::OnItemsChanged(this, core::mem::transmute_copy(&dwnumberofchanges), core::mem::transmute_copy(&datachangeentries), core::mem::transmute_copy(&hrcompletioncodes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartedMonitoringScope: StartedMonitoringScope::<Identity, OFFSET>,
            StoppedMonitoringScope: StoppedMonitoringScope::<Identity, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchPersistentItemsChangedSink as windows_core::Interface>::IID
    }
}
pub trait ISearchProtocol_Impl: Sized {
    fn Init(&self, ptimeoutinfo: *const TIMEOUT_INFO, pprotocolhandlersite: Option<&IProtocolHandlerSite>, pproxyinfo: *const PROXY_INFO) -> windows_core::Result<()>;
    fn CreateAccessor(&self, pcwszurl: &windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO) -> windows_core::Result<IUrlAccessor>;
    fn CloseAccessor(&self, paccessor: Option<&IUrlAccessor>) -> windows_core::Result<()>;
    fn ShutDown(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISearchProtocol {}
impl ISearchProtocol_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchProtocol_Vtbl
    where
        Identity: ISearchProtocol_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimeoutinfo: *const TIMEOUT_INFO, pprotocolhandlersite: *mut core::ffi::c_void, pproxyinfo: *const PROXY_INFO) -> windows_core::HRESULT
        where
            Identity: ISearchProtocol_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchProtocol_Impl::Init(this, core::mem::transmute_copy(&ptimeoutinfo), windows_core::from_raw_borrowed(&pprotocolhandlersite), core::mem::transmute_copy(&pproxyinfo)).into()
        }
        unsafe extern "system" fn CreateAccessor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwszurl: windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, ppaccessor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchProtocol_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchProtocol_Impl::CreateAccessor(this, core::mem::transmute(&pcwszurl), core::mem::transmute_copy(&pauthenticationinfo), core::mem::transmute_copy(&pincrementalaccessinfo), core::mem::transmute_copy(&piteminfo)) {
                Ok(ok__) => {
                    ppaccessor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseAccessor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paccessor: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchProtocol_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchProtocol_Impl::CloseAccessor(this, windows_core::from_raw_borrowed(&paccessor)).into()
        }
        unsafe extern "system" fn ShutDown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchProtocol_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchProtocol_Impl::ShutDown(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            CreateAccessor: CreateAccessor::<Identity, OFFSET>,
            CloseAccessor: CloseAccessor::<Identity, OFFSET>,
            ShutDown: ShutDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchProtocol as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchProtocol2_Impl: Sized + ISearchProtocol_Impl {
    fn CreateAccessorEx(&self, pcwszurl: &windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, puserdata: *const super::Com::BLOB) -> windows_core::Result<IUrlAccessor>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISearchProtocol2 {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchProtocol2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchProtocol2_Vtbl
    where
        Identity: ISearchProtocol2_Impl,
    {
        unsafe extern "system" fn CreateAccessorEx<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwszurl: windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, puserdata: *const super::Com::BLOB, ppaccessor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchProtocol2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchProtocol2_Impl::CreateAccessorEx(this, core::mem::transmute(&pcwszurl), core::mem::transmute_copy(&pauthenticationinfo), core::mem::transmute_copy(&pincrementalaccessinfo), core::mem::transmute_copy(&piteminfo), core::mem::transmute_copy(&puserdata)) {
                Ok(ok__) => {
                    ppaccessor.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ISearchProtocol_Vtbl::new::<Identity, OFFSET>(), CreateAccessorEx: CreateAccessorEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchProtocol2 as windows_core::Interface>::IID || iid == &<ISearchProtocol as windows_core::Interface>::IID
    }
}
pub trait ISearchProtocolThreadContext_Impl: Sized {
    fn ThreadInit(&self) -> windows_core::Result<()>;
    fn ThreadShutdown(&self) -> windows_core::Result<()>;
    fn ThreadIdle(&self, dwtimeelaspedsincelastcallinms: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISearchProtocolThreadContext {}
impl ISearchProtocolThreadContext_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchProtocolThreadContext_Vtbl
    where
        Identity: ISearchProtocolThreadContext_Impl,
    {
        unsafe extern "system" fn ThreadInit<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchProtocolThreadContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchProtocolThreadContext_Impl::ThreadInit(this).into()
        }
        unsafe extern "system" fn ThreadShutdown<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISearchProtocolThreadContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchProtocolThreadContext_Impl::ThreadShutdown(this).into()
        }
        unsafe extern "system" fn ThreadIdle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeelaspedsincelastcallinms: u32) -> windows_core::HRESULT
        where
            Identity: ISearchProtocolThreadContext_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchProtocolThreadContext_Impl::ThreadIdle(this, core::mem::transmute_copy(&dwtimeelaspedsincelastcallinms)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ThreadInit: ThreadInit::<Identity, OFFSET>,
            ThreadShutdown: ThreadShutdown::<Identity, OFFSET>,
            ThreadIdle: ThreadIdle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchProtocolThreadContext as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISearchQueryHelper_Impl: Sized {
    fn ConnectionString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetQueryContentLocale(&self, lcid: u32) -> windows_core::Result<()>;
    fn QueryContentLocale(&self) -> windows_core::Result<u32>;
    fn SetQueryKeywordLocale(&self, lcid: u32) -> windows_core::Result<()>;
    fn QueryKeywordLocale(&self) -> windows_core::Result<u32>;
    fn SetQueryTermExpansion(&self, expandterms: SEARCH_TERM_EXPANSION) -> windows_core::Result<()>;
    fn QueryTermExpansion(&self) -> windows_core::Result<SEARCH_TERM_EXPANSION>;
    fn SetQuerySyntax(&self, querysyntax: SEARCH_QUERY_SYNTAX) -> windows_core::Result<()>;
    fn QuerySyntax(&self) -> windows_core::Result<SEARCH_QUERY_SYNTAX>;
    fn SetQueryContentProperties(&self, pszcontentproperties: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn QueryContentProperties(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetQuerySelectColumns(&self, pszselectcolumns: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn QuerySelectColumns(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetQueryWhereRestrictions(&self, pszrestrictions: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn QueryWhereRestrictions(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetQuerySorting(&self, pszsorting: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn QuerySorting(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GenerateSQLFromUserQuery(&self, pszquery: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn WriteProperties(&self, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SetQueryMaxResults(&self, cmaxresults: i32) -> windows_core::Result<()>;
    fn QueryMaxResults(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISearchQueryHelper {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISearchQueryHelper_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchQueryHelper_Vtbl
    where
        Identity: ISearchQueryHelper_Impl,
    {
        unsafe extern "system" fn ConnectionString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszconnectionstring: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::ConnectionString(this) {
                Ok(ok__) => {
                    pszconnectionstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryContentLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHelper_Impl::SetQueryContentLocale(this, core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn QueryContentLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::QueryContentLocale(this) {
                Ok(ok__) => {
                    plcid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryKeywordLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcid: u32) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHelper_Impl::SetQueryKeywordLocale(this, core::mem::transmute_copy(&lcid)).into()
        }
        unsafe extern "system" fn QueryKeywordLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcid: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::QueryKeywordLocale(this) {
                Ok(ok__) => {
                    plcid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryTermExpansion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, expandterms: SEARCH_TERM_EXPANSION) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHelper_Impl::SetQueryTermExpansion(this, core::mem::transmute_copy(&expandterms)).into()
        }
        unsafe extern "system" fn QueryTermExpansion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexpandterms: *mut SEARCH_TERM_EXPANSION) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::QueryTermExpansion(this) {
                Ok(ok__) => {
                    pexpandterms.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySyntax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, querysyntax: SEARCH_QUERY_SYNTAX) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHelper_Impl::SetQuerySyntax(this, core::mem::transmute_copy(&querysyntax)).into()
        }
        unsafe extern "system" fn QuerySyntax<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pquerysyntax: *mut SEARCH_QUERY_SYNTAX) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::QuerySyntax(this) {
                Ok(ok__) => {
                    pquerysyntax.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryContentProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcontentproperties: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHelper_Impl::SetQueryContentProperties(this, core::mem::transmute(&pszcontentproperties)).into()
        }
        unsafe extern "system" fn QueryContentProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcontentproperties: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::QueryContentProperties(this) {
                Ok(ok__) => {
                    ppszcontentproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySelectColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszselectcolumns: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHelper_Impl::SetQuerySelectColumns(this, core::mem::transmute(&pszselectcolumns)).into()
        }
        unsafe extern "system" fn QuerySelectColumns<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszselectcolumns: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::QuerySelectColumns(this) {
                Ok(ok__) => {
                    ppszselectcolumns.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueryWhereRestrictions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrestrictions: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHelper_Impl::SetQueryWhereRestrictions(this, core::mem::transmute(&pszrestrictions)).into()
        }
        unsafe extern "system" fn QueryWhereRestrictions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszrestrictions: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::QueryWhereRestrictions(this) {
                Ok(ok__) => {
                    ppszrestrictions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuerySorting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsorting: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHelper_Impl::SetQuerySorting(this, core::mem::transmute(&pszsorting)).into()
        }
        unsafe extern "system" fn QuerySorting<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszsorting: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::QuerySorting(this) {
                Ok(ok__) => {
                    ppszsorting.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateSQLFromUserQuery<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszquery: windows_core::PCWSTR, ppszsql: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::GenerateSQLFromUserQuery(this, core::mem::transmute(&pszquery)) {
                Ok(ok__) => {
                    ppszsql.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHelper_Impl::WriteProperties(this, core::mem::transmute_copy(&itemid), core::mem::transmute_copy(&dwnumberofcolumns), core::mem::transmute_copy(&pcolumns), core::mem::transmute_copy(&pvalues), core::mem::transmute_copy(&pftgathermodifiedtime)).into()
        }
        unsafe extern "system" fn SetQueryMaxResults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cmaxresults: i32) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHelper_Impl::SetQueryMaxResults(this, core::mem::transmute_copy(&cmaxresults)).into()
        }
        unsafe extern "system" fn QueryMaxResults<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmaxresults: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISearchQueryHelper_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchQueryHelper_Impl::QueryMaxResults(this) {
                Ok(ok__) => {
                    pcmaxresults.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectionString: ConnectionString::<Identity, OFFSET>,
            SetQueryContentLocale: SetQueryContentLocale::<Identity, OFFSET>,
            QueryContentLocale: QueryContentLocale::<Identity, OFFSET>,
            SetQueryKeywordLocale: SetQueryKeywordLocale::<Identity, OFFSET>,
            QueryKeywordLocale: QueryKeywordLocale::<Identity, OFFSET>,
            SetQueryTermExpansion: SetQueryTermExpansion::<Identity, OFFSET>,
            QueryTermExpansion: QueryTermExpansion::<Identity, OFFSET>,
            SetQuerySyntax: SetQuerySyntax::<Identity, OFFSET>,
            QuerySyntax: QuerySyntax::<Identity, OFFSET>,
            SetQueryContentProperties: SetQueryContentProperties::<Identity, OFFSET>,
            QueryContentProperties: QueryContentProperties::<Identity, OFFSET>,
            SetQuerySelectColumns: SetQuerySelectColumns::<Identity, OFFSET>,
            QuerySelectColumns: QuerySelectColumns::<Identity, OFFSET>,
            SetQueryWhereRestrictions: SetQueryWhereRestrictions::<Identity, OFFSET>,
            QueryWhereRestrictions: QueryWhereRestrictions::<Identity, OFFSET>,
            SetQuerySorting: SetQuerySorting::<Identity, OFFSET>,
            QuerySorting: QuerySorting::<Identity, OFFSET>,
            GenerateSQLFromUserQuery: GenerateSQLFromUserQuery::<Identity, OFFSET>,
            WriteProperties: WriteProperties::<Identity, OFFSET>,
            SetQueryMaxResults: SetQueryMaxResults::<Identity, OFFSET>,
            QueryMaxResults: QueryMaxResults::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchQueryHelper as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait ISearchQueryHits_Impl: Sized {
    fn Init(&self, pflt: Option<&super::super::Storage::IndexServer::IFilter>, ulflags: u32) -> i32;
    fn NextHitMoniker(&self, pcmnk: *mut u32, papmnk: *mut *mut Option<super::Com::IMoniker>) -> i32;
    fn NextHitOffset(&self, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ISearchQueryHits {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ISearchQueryHits_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchQueryHits_Vtbl
    where
        Identity: ISearchQueryHits_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflt: *mut core::ffi::c_void, ulflags: u32) -> i32
        where
            Identity: ISearchQueryHits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHits_Impl::Init(this, windows_core::from_raw_borrowed(&pflt), core::mem::transmute_copy(&ulflags))
        }
        unsafe extern "system" fn NextHitMoniker<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmnk: *mut u32, papmnk: *mut *mut Option<super::Com::IMoniker>) -> i32
        where
            Identity: ISearchQueryHits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHits_Impl::NextHitMoniker(this, core::mem::transmute_copy(&pcmnk), core::mem::transmute_copy(&papmnk))
        }
        unsafe extern "system" fn NextHitOffset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32
        where
            Identity: ISearchQueryHits_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchQueryHits_Impl::NextHitOffset(this, core::mem::transmute_copy(&pcregion), core::mem::transmute_copy(&paregion))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            NextHitMoniker: NextHitMoniker::<Identity, OFFSET>,
            NextHitOffset: NextHitOffset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchQueryHits as windows_core::Interface>::IID
    }
}
pub trait ISearchRoot_Impl: Sized {
    fn SetSchedule(&self, psztaskarg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Schedule(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetRootURL(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RootURL(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetIsHierarchical(&self, fishierarchical: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IsHierarchical(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetProvidesNotifications(&self, fprovidesnotifications: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ProvidesNotifications(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetUseNotificationsOnly(&self, fusenotificationsonly: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn UseNotificationsOnly(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnumerationDepth(&self, dwdepth: u32) -> windows_core::Result<()>;
    fn EnumerationDepth(&self) -> windows_core::Result<u32>;
    fn SetHostDepth(&self, dwdepth: u32) -> windows_core::Result<()>;
    fn HostDepth(&self) -> windows_core::Result<u32>;
    fn SetFollowDirectories(&self, ffollowdirectories: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn FollowDirectories(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAuthenticationType(&self, authtype: AUTH_TYPE) -> windows_core::Result<()>;
    fn AuthenticationType(&self) -> windows_core::Result<AUTH_TYPE>;
    fn SetUser(&self, pszuser: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn User(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetPassword(&self, pszpassword: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Password(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for ISearchRoot {}
impl ISearchRoot_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchRoot_Vtbl
    where
        Identity: ISearchRoot_Impl,
    {
        unsafe extern "system" fn SetSchedule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztaskarg: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetSchedule(this, core::mem::transmute(&psztaskarg)).into()
        }
        unsafe extern "system" fn Schedule<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsztaskarg: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::Schedule(this) {
                Ok(ok__) => {
                    ppsztaskarg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetRootURL(this, core::mem::transmute(&pszurl)).into()
        }
        unsafe extern "system" fn RootURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::RootURL(this) {
                Ok(ok__) => {
                    ppszurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHierarchical<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fishierarchical: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetIsHierarchical(this, core::mem::transmute_copy(&fishierarchical)).into()
        }
        unsafe extern "system" fn IsHierarchical<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfishierarchical: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::IsHierarchical(this) {
                Ok(ok__) => {
                    pfishierarchical.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProvidesNotifications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fprovidesnotifications: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetProvidesNotifications(this, core::mem::transmute_copy(&fprovidesnotifications)).into()
        }
        unsafe extern "system" fn ProvidesNotifications<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprovidesnotifications: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::ProvidesNotifications(this) {
                Ok(ok__) => {
                    pfprovidesnotifications.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseNotificationsOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fusenotificationsonly: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetUseNotificationsOnly(this, core::mem::transmute_copy(&fusenotificationsonly)).into()
        }
        unsafe extern "system" fn UseNotificationsOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfusenotificationsonly: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::UseNotificationsOnly(this) {
                Ok(ok__) => {
                    pfusenotificationsonly.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnumerationDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdepth: u32) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetEnumerationDepth(this, core::mem::transmute_copy(&dwdepth)).into()
        }
        unsafe extern "system" fn EnumerationDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdepth: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::EnumerationDepth(this) {
                Ok(ok__) => {
                    pdwdepth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdepth: u32) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetHostDepth(this, core::mem::transmute_copy(&dwdepth)).into()
        }
        unsafe extern "system" fn HostDepth<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdepth: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::HostDepth(this) {
                Ok(ok__) => {
                    pdwdepth.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFollowDirectories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffollowdirectories: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetFollowDirectories(this, core::mem::transmute_copy(&ffollowdirectories)).into()
        }
        unsafe extern "system" fn FollowDirectories<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffollowdirectories: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::FollowDirectories(this) {
                Ok(ok__) => {
                    pffollowdirectories.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, authtype: AUTH_TYPE) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetAuthenticationType(this, core::mem::transmute_copy(&authtype)).into()
        }
        unsafe extern "system" fn AuthenticationType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthtype: *mut AUTH_TYPE) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::AuthenticationType(this) {
                Ok(ok__) => {
                    pauthtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUser<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuser: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetUser(this, core::mem::transmute(&pszuser)).into()
        }
        unsafe extern "system" fn User<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszuser: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::User(this) {
                Ok(ok__) => {
                    ppszuser.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpassword: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchRoot_Impl::SetPassword(this, core::mem::transmute(&pszpassword)).into()
        }
        unsafe extern "system" fn Password<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpassword: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchRoot_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchRoot_Impl::Password(this) {
                Ok(ok__) => {
                    ppszpassword.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSchedule: SetSchedule::<Identity, OFFSET>,
            Schedule: Schedule::<Identity, OFFSET>,
            SetRootURL: SetRootURL::<Identity, OFFSET>,
            RootURL: RootURL::<Identity, OFFSET>,
            SetIsHierarchical: SetIsHierarchical::<Identity, OFFSET>,
            IsHierarchical: IsHierarchical::<Identity, OFFSET>,
            SetProvidesNotifications: SetProvidesNotifications::<Identity, OFFSET>,
            ProvidesNotifications: ProvidesNotifications::<Identity, OFFSET>,
            SetUseNotificationsOnly: SetUseNotificationsOnly::<Identity, OFFSET>,
            UseNotificationsOnly: UseNotificationsOnly::<Identity, OFFSET>,
            SetEnumerationDepth: SetEnumerationDepth::<Identity, OFFSET>,
            EnumerationDepth: EnumerationDepth::<Identity, OFFSET>,
            SetHostDepth: SetHostDepth::<Identity, OFFSET>,
            HostDepth: HostDepth::<Identity, OFFSET>,
            SetFollowDirectories: SetFollowDirectories::<Identity, OFFSET>,
            FollowDirectories: FollowDirectories::<Identity, OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Identity, OFFSET>,
            AuthenticationType: AuthenticationType::<Identity, OFFSET>,
            SetUser: SetUser::<Identity, OFFSET>,
            User: User::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            Password: Password::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchRoot as windows_core::Interface>::IID
    }
}
pub trait ISearchScopeRule_Impl: Sized {
    fn PatternOrURL(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsIncluded(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsDefault(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn FollowFlags(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ISearchScopeRule {}
impl ISearchScopeRule_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchScopeRule_Vtbl
    where
        Identity: ISearchScopeRule_Impl,
    {
        unsafe extern "system" fn PatternOrURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpatternorurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ISearchScopeRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchScopeRule_Impl::PatternOrURL(this) {
                Ok(ok__) => {
                    ppszpatternorurl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIncluded<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisincluded: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchScopeRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchScopeRule_Impl::IsIncluded(this) {
                Ok(ok__) => {
                    pfisincluded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisdefault: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchScopeRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchScopeRule_Impl::IsDefault(this) {
                Ok(ok__) => {
                    pfisdefault.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FollowFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfollowflags: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISearchScopeRule_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISearchScopeRule_Impl::FollowFlags(this) {
                Ok(ok__) => {
                    pfollowflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PatternOrURL: PatternOrURL::<Identity, OFFSET>,
            IsIncluded: IsIncluded::<Identity, OFFSET>,
            IsDefault: IsDefault::<Identity, OFFSET>,
            FollowFlags: FollowFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchScopeRule as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchViewChangedSink_Impl: Sized {
    fn OnChange(&self, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISearchViewChangedSink {}
#[cfg(feature = "Win32_System_Com")]
impl ISearchViewChangedSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISearchViewChangedSink_Vtbl
    where
        Identity: ISearchViewChangedSink_Impl,
    {
        unsafe extern "system" fn OnChange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISearchViewChangedSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISearchViewChangedSink_Impl::OnChange(this, core::mem::transmute_copy(&pdwdocid), core::mem::transmute_copy(&pchange), core::mem::transmute_copy(&pfinview)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISearchViewChangedSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security_Authorization")]
pub trait ISecurityInfo_Impl: Sized {
    fn GetCurrentTrustee(&self) -> windows_core::Result<*mut super::super::Security::Authorization::TRUSTEE_W>;
    fn GetObjectTypes(&self, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetPermissions(&self, objecttype: &windows_core::GUID) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Security_Authorization")]
impl windows_core::RuntimeName for ISecurityInfo {}
#[cfg(feature = "Win32_Security_Authorization")]
impl ISecurityInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISecurityInfo_Vtbl
    where
        Identity: ISecurityInfo_Impl,
    {
        unsafe extern "system" fn GetCurrentTrustee<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptrustee: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT
        where
            Identity: ISecurityInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityInfo_Impl::GetCurrentTrustee(this) {
                Ok(ok__) => {
                    pptrustee.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISecurityInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISecurityInfo_Impl::GetObjectTypes(this, core::mem::transmute_copy(&cobjecttypes), core::mem::transmute_copy(&rgobjecttypes)).into()
        }
        unsafe extern "system" fn GetPermissions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, objecttype: windows_core::GUID, ppermissions: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISecurityInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISecurityInfo_Impl::GetPermissions(this, core::mem::transmute(&objecttype)) {
                Ok(ok__) => {
                    ppermissions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentTrustee: GetCurrentTrustee::<Identity, OFFSET>,
            GetObjectTypes: GetObjectTypes::<Identity, OFFSET>,
            GetPermissions: GetPermissions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISecurityInfo as windows_core::Interface>::IID
    }
}
pub trait IService_Impl: Sized {
    fn InvokeService(&self, punkinner: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IService {}
impl IService_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IService_Vtbl
    where
        Identity: IService_Impl,
    {
        unsafe extern "system" fn InvokeService<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkinner: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IService_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IService_Impl::InvokeService(this, windows_core::from_raw_borrowed(&punkinner)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InvokeService: InvokeService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait ISessionProperties_Impl: Sized {
    fn GetProperties(&self, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
    fn SetProperties(&self, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for ISessionProperties {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ISessionProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISessionProperties_Vtbl
    where
        Identity: ISessionProperties_Impl,
    {
        unsafe extern "system" fn GetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: ISessionProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISessionProperties_Impl::GetProperties(this, core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets)).into()
        }
        unsafe extern "system" fn SetProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: ISessionProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISessionProperties_Impl::SetProperties(this, core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperties: GetProperties::<Identity, OFFSET>,
            SetProperties: SetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISessionProperties as windows_core::Interface>::IID
    }
}
pub trait ISimpleCommandCreator_Impl: Sized {
    fn CreateICommand(&self, ppiunknown: *mut Option<windows_core::IUnknown>, pouterunk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn VerifyCatalog(&self, pwszmachine: &windows_core::PCWSTR, pwszcatalogname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDefaultCatalog(&self, pwszcatalogname: &windows_core::PCWSTR, cwcin: u32, pcwcout: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISimpleCommandCreator {}
impl ISimpleCommandCreator_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISimpleCommandCreator_Vtbl
    where
        Identity: ISimpleCommandCreator_Impl,
    {
        unsafe extern "system" fn CreateICommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiunknown: *mut *mut core::ffi::c_void, pouterunk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISimpleCommandCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISimpleCommandCreator_Impl::CreateICommand(this, core::mem::transmute_copy(&ppiunknown), windows_core::from_raw_borrowed(&pouterunk)).into()
        }
        unsafe extern "system" fn VerifyCatalog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszmachine: windows_core::PCWSTR, pwszcatalogname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISimpleCommandCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISimpleCommandCreator_Impl::VerifyCatalog(this, core::mem::transmute(&pwszmachine), core::mem::transmute(&pwszcatalogname)).into()
        }
        unsafe extern "system" fn GetDefaultCatalog<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszcatalogname: windows_core::PCWSTR, cwcin: u32, pcwcout: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISimpleCommandCreator_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISimpleCommandCreator_Impl::GetDefaultCatalog(this, core::mem::transmute(&pwszcatalogname), core::mem::transmute_copy(&cwcin), core::mem::transmute_copy(&pcwcout)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateICommand: CreateICommand::<Identity, OFFSET>,
            VerifyCatalog: VerifyCatalog::<Identity, OFFSET>,
            GetDefaultCatalog: GetDefaultCatalog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimpleCommandCreator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait ISourcesRowset_Impl: Sized {
    fn GetSourcesRowset(&self, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for ISourcesRowset {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ISourcesRowset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISourcesRowset_Vtbl
    where
        Identity: ISourcesRowset_Impl,
    {
        unsafe extern "system" fn GetSourcesRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISourcesRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISourcesRowset_Impl::GetSourcesRowset(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgproperties), core::mem::transmute_copy(&ppsourcesrowset)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetSourcesRowset: GetSourcesRowset::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISourcesRowset as windows_core::Interface>::IID
    }
}
pub trait IStemmer_Impl: Sized {
    fn Init(&self, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GenerateWordForms(&self, pwcinbuf: &windows_core::PCWSTR, cwc: u32, pstemsink: Option<&IWordFormSink>) -> windows_core::Result<()>;
    fn GetLicenseToUse(&self, ppwcslicense: *const *const u16) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IStemmer {}
impl IStemmer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IStemmer_Vtbl
    where
        Identity: IStemmer_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IStemmer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStemmer_Impl::Init(this, core::mem::transmute_copy(&ulmaxtokensize), core::mem::transmute_copy(&pflicense)).into()
        }
        unsafe extern "system" fn GenerateWordForms<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcinbuf: windows_core::PCWSTR, cwc: u32, pstemsink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IStemmer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStemmer_Impl::GenerateWordForms(this, core::mem::transmute(&pwcinbuf), core::mem::transmute_copy(&cwc), windows_core::from_raw_borrowed(&pstemsink)).into()
        }
        unsafe extern "system" fn GetLicenseToUse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwcslicense: *const *const u16) -> windows_core::HRESULT
        where
            Identity: IStemmer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStemmer_Impl::GetLicenseToUse(this, core::mem::transmute_copy(&ppwcslicense)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GenerateWordForms: GenerateWordForms::<Identity, OFFSET>,
            GetLicenseToUse: GetLicenseToUse::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStemmer as windows_core::Interface>::IID
    }
}
pub trait ISubscriptionItem_Impl: Sized {
    fn GetCookie(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetSubscriptionItemInfo(&self, psubscriptioniteminfo: *mut SUBSCRIPTIONITEMINFO) -> windows_core::Result<()>;
    fn SetSubscriptionItemInfo(&self, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> windows_core::Result<()>;
    fn ReadProperties(&self, ncount: u32, rgwszname: *const windows_core::PCWSTR, rgvalue: *mut windows_core::VARIANT) -> windows_core::Result<()>;
    fn WriteProperties(&self, ncount: u32, rgwszname: *const windows_core::PCWSTR, rgvalue: *const windows_core::VARIANT) -> windows_core::Result<()>;
    fn EnumProperties(&self) -> windows_core::Result<IEnumItemProperties>;
    fn NotifyChanged(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISubscriptionItem {}
impl ISubscriptionItem_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISubscriptionItem_Vtbl
    where
        Identity: ISubscriptionItem_Impl,
    {
        unsafe extern "system" fn GetCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcookie: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISubscriptionItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISubscriptionItem_Impl::GetCookie(this) {
                Ok(ok__) => {
                    pcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubscriptioniteminfo: *mut SUBSCRIPTIONITEMINFO) -> windows_core::HRESULT
        where
            Identity: ISubscriptionItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionItem_Impl::GetSubscriptionItemInfo(this, core::mem::transmute_copy(&psubscriptioniteminfo)).into()
        }
        unsafe extern "system" fn SetSubscriptionItemInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> windows_core::HRESULT
        where
            Identity: ISubscriptionItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionItem_Impl::SetSubscriptionItemInfo(this, core::mem::transmute_copy(&psubscriptioniteminfo)).into()
        }
        unsafe extern "system" fn ReadProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncount: u32, rgwszname: *const windows_core::PCWSTR, rgvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISubscriptionItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionItem_Impl::ReadProperties(this, core::mem::transmute_copy(&ncount), core::mem::transmute_copy(&rgwszname), core::mem::transmute_copy(&rgvalue)).into()
        }
        unsafe extern "system" fn WriteProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncount: u32, rgwszname: *const windows_core::PCWSTR, rgvalue: *const core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISubscriptionItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionItem_Impl::WriteProperties(this, core::mem::transmute_copy(&ncount), core::mem::transmute_copy(&rgwszname), core::mem::transmute_copy(&rgvalue)).into()
        }
        unsafe extern "system" fn EnumProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumitemproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISubscriptionItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISubscriptionItem_Impl::EnumProperties(this) {
                Ok(ok__) => {
                    ppenumitemproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISubscriptionItem_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionItem_Impl::NotifyChanged(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCookie: GetCookie::<Identity, OFFSET>,
            GetSubscriptionItemInfo: GetSubscriptionItemInfo::<Identity, OFFSET>,
            SetSubscriptionItemInfo: SetSubscriptionItemInfo::<Identity, OFFSET>,
            ReadProperties: ReadProperties::<Identity, OFFSET>,
            WriteProperties: WriteProperties::<Identity, OFFSET>,
            EnumProperties: EnumProperties::<Identity, OFFSET>,
            NotifyChanged: NotifyChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISubscriptionItem as windows_core::Interface>::IID
    }
}
pub trait ISubscriptionMgr_Impl: Sized {
    fn DeleteSubscription(&self, pwszurl: &windows_core::PCWSTR, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn UpdateSubscription(&self, pwszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UpdateAll(&self) -> windows_core::Result<()>;
    fn IsSubscribed(&self, pwszurl: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetSubscriptionInfo(&self, pwszurl: &windows_core::PCWSTR, pinfo: *mut SUBSCRIPTIONINFO) -> windows_core::Result<()>;
    fn GetDefaultInfo(&self, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> windows_core::Result<()>;
    fn ShowSubscriptionProperties(&self, pwszurl: &windows_core::PCWSTR, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn CreateSubscription(&self, hwnd: super::super::Foundation::HWND, pwszurl: &windows_core::PCWSTR, pwszfriendlyname: &windows_core::PCWSTR, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISubscriptionMgr {}
impl ISubscriptionMgr_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISubscriptionMgr_Vtbl
    where
        Identity: ISubscriptionMgr_Impl,
    {
        unsafe extern "system" fn DeleteSubscription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr_Impl::DeleteSubscription(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn UpdateSubscription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr_Impl::UpdateSubscription(this, core::mem::transmute(&pwszurl)).into()
        }
        unsafe extern "system" fn UpdateAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr_Impl::UpdateAll(this).into()
        }
        unsafe extern "system" fn IsSubscribed<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pfsubscribed: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISubscriptionMgr_Impl::IsSubscribed(this, core::mem::transmute(&pwszurl)) {
                Ok(ok__) => {
                    pfsubscribed.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, pinfo: *mut SUBSCRIPTIONINFO) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr_Impl::GetSubscriptionInfo(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn GetDefaultInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr_Impl::GetDefaultInfo(this, core::mem::transmute_copy(&subtype), core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn ShowSubscriptionProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr_Impl::ShowSubscriptionProperties(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn CreateSubscription<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, pwszurl: windows_core::PCWSTR, pwszfriendlyname: windows_core::PCWSTR, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr_Impl::CreateSubscription(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&pwszurl), core::mem::transmute(&pwszfriendlyname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&substype), core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeleteSubscription: DeleteSubscription::<Identity, OFFSET>,
            UpdateSubscription: UpdateSubscription::<Identity, OFFSET>,
            UpdateAll: UpdateAll::<Identity, OFFSET>,
            IsSubscribed: IsSubscribed::<Identity, OFFSET>,
            GetSubscriptionInfo: GetSubscriptionInfo::<Identity, OFFSET>,
            GetDefaultInfo: GetDefaultInfo::<Identity, OFFSET>,
            ShowSubscriptionProperties: ShowSubscriptionProperties::<Identity, OFFSET>,
            CreateSubscription: CreateSubscription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISubscriptionMgr as windows_core::Interface>::IID
    }
}
pub trait ISubscriptionMgr2_Impl: Sized + ISubscriptionMgr_Impl {
    fn GetItemFromURL(&self, pwszurl: &windows_core::PCWSTR) -> windows_core::Result<ISubscriptionItem>;
    fn GetItemFromCookie(&self, psubscriptioncookie: *const windows_core::GUID) -> windows_core::Result<ISubscriptionItem>;
    fn GetSubscriptionRunState(&self, dwnumcookies: u32, pcookies: *const windows_core::GUID, pdwrunstate: *mut u32) -> windows_core::Result<()>;
    fn EnumSubscriptions(&self, dwflags: u32) -> windows_core::Result<IEnumSubscription>;
    fn UpdateItems(&self, dwflags: u32, dwnumcookies: u32, pcookies: *const windows_core::GUID) -> windows_core::Result<()>;
    fn AbortItems(&self, dwnumcookies: u32, pcookies: *const windows_core::GUID) -> windows_core::Result<()>;
    fn AbortAll(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISubscriptionMgr2 {}
impl ISubscriptionMgr2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISubscriptionMgr2_Vtbl
    where
        Identity: ISubscriptionMgr2_Impl,
    {
        unsafe extern "system" fn GetItemFromURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, ppsubscriptionitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISubscriptionMgr2_Impl::GetItemFromURL(this, core::mem::transmute(&pwszurl)) {
                Ok(ok__) => {
                    ppsubscriptionitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemFromCookie<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubscriptioncookie: *const windows_core::GUID, ppsubscriptionitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISubscriptionMgr2_Impl::GetItemFromCookie(this, core::mem::transmute_copy(&psubscriptioncookie)) {
                Ok(ok__) => {
                    ppsubscriptionitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriptionRunState<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnumcookies: u32, pcookies: *const windows_core::GUID, pdwrunstate: *mut u32) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr2_Impl::GetSubscriptionRunState(this, core::mem::transmute_copy(&dwnumcookies), core::mem::transmute_copy(&pcookies), core::mem::transmute_copy(&pdwrunstate)).into()
        }
        unsafe extern "system" fn EnumSubscriptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppenumsubscriptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISubscriptionMgr2_Impl::EnumSubscriptions(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppenumsubscriptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, dwnumcookies: u32, pcookies: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr2_Impl::UpdateItems(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwnumcookies), core::mem::transmute_copy(&pcookies)).into()
        }
        unsafe extern "system" fn AbortItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnumcookies: u32, pcookies: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr2_Impl::AbortItems(this, core::mem::transmute_copy(&dwnumcookies), core::mem::transmute_copy(&pcookies)).into()
        }
        unsafe extern "system" fn AbortAll<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISubscriptionMgr2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISubscriptionMgr2_Impl::AbortAll(this).into()
        }
        Self {
            base__: ISubscriptionMgr_Vtbl::new::<Identity, OFFSET>(),
            GetItemFromURL: GetItemFromURL::<Identity, OFFSET>,
            GetItemFromCookie: GetItemFromCookie::<Identity, OFFSET>,
            GetSubscriptionRunState: GetSubscriptionRunState::<Identity, OFFSET>,
            EnumSubscriptions: EnumSubscriptions::<Identity, OFFSET>,
            UpdateItems: UpdateItems::<Identity, OFFSET>,
            AbortItems: AbortItems::<Identity, OFFSET>,
            AbortAll: AbortAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISubscriptionMgr2 as windows_core::Interface>::IID || iid == &<ISubscriptionMgr as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait ITableCreation_Impl: Sized + ITableDefinition_Impl {
    fn GetTableDefinition(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: *mut usize, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut u16) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ITableCreation {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ITableCreation_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITableCreation_Vtbl
    where
        Identity: ITableCreation_Impl,
    {
        unsafe extern "system" fn GetTableDefinition<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: *mut usize, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut u16) -> windows_core::HRESULT
        where
            Identity: ITableCreation_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableCreation_Impl::GetTableDefinition(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pccolumndescs), core::mem::transmute_copy(&prgcolumndescs), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets), core::mem::transmute_copy(&pcconstraintdescs), core::mem::transmute_copy(&prgconstraintdescs), core::mem::transmute_copy(&ppwszstringbuffer)).into()
        }
        Self { base__: ITableDefinition_Vtbl::new::<Identity, OFFSET>(), GetTableDefinition: GetTableDefinition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableCreation as windows_core::Interface>::IID || iid == &<ITableDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait ITableDefinition_Impl: Sized {
    fn CreateTable(&self, punkouter: Option<&windows_core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *const DBCOLUMNDESC, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DropTable(&self, ptableid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
    fn AddColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
    fn DropColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ITableDefinition {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ITableDefinition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITableDefinition_Vtbl
    where
        Identity: ITableDefinition_Impl,
    {
        unsafe extern "system" fn CreateTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *const DBCOLUMNDESC, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITableDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableDefinition_Impl::CreateTable(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&ccolumndescs), core::mem::transmute_copy(&rgcolumndescs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pptableid), core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn DropTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: ITableDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableDefinition_Impl::DropTable(this, core::mem::transmute_copy(&ptableid)).into()
        }
        unsafe extern "system" fn AddColumn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: *mut *mut super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: ITableDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableDefinition_Impl::AddColumn(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pcolumndesc), core::mem::transmute_copy(&ppcolumnid)).into()
        }
        unsafe extern "system" fn DropColumn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: ITableDefinition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableDefinition_Impl::DropColumn(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pcolumnid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateTable: CreateTable::<Identity, OFFSET>,
            DropTable: DropTable::<Identity, OFFSET>,
            AddColumn: AddColumn::<Identity, OFFSET>,
            DropColumn: DropColumn::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait ITableDefinitionWithConstraints_Impl: Sized + ITableCreation_Impl {
    fn AddConstraint(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pconstraintdesc: *const DBCONSTRAINTDESC) -> windows_core::Result<()>;
    fn CreateTableWithConstraints(&self, punkouter: Option<&windows_core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *const DBCONSTRAINTDESC, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut Option<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn DropConstraint(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pconstraintid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ITableDefinitionWithConstraints {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ITableDefinitionWithConstraints_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITableDefinitionWithConstraints_Vtbl
    where
        Identity: ITableDefinitionWithConstraints_Impl,
    {
        unsafe extern "system" fn AddConstraint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pconstraintdesc: *const DBCONSTRAINTDESC) -> windows_core::HRESULT
        where
            Identity: ITableDefinitionWithConstraints_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableDefinitionWithConstraints_Impl::AddConstraint(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pconstraintdesc)).into()
        }
        unsafe extern "system" fn CreateTableWithConstraints<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *const DBCONSTRAINTDESC, riid: *const windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITableDefinitionWithConstraints_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableDefinitionWithConstraints_Impl::CreateTableWithConstraints(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&ccolumndescs), core::mem::transmute_copy(&rgcolumndescs), core::mem::transmute_copy(&cconstraintdescs), core::mem::transmute_copy(&rgconstraintdescs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets), core::mem::transmute_copy(&pptableid), core::mem::transmute_copy(&pprowset)).into()
        }
        unsafe extern "system" fn DropConstraint<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pconstraintid: *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: ITableDefinitionWithConstraints_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableDefinitionWithConstraints_Impl::DropConstraint(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&pconstraintid)).into()
        }
        Self {
            base__: ITableCreation_Vtbl::new::<Identity, OFFSET>(),
            AddConstraint: AddConstraint::<Identity, OFFSET>,
            CreateTableWithConstraints: CreateTableWithConstraints::<Identity, OFFSET>,
            DropConstraint: DropConstraint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableDefinitionWithConstraints as windows_core::Interface>::IID || iid == &<ITableDefinition as windows_core::Interface>::IID || iid == &<ITableCreation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait ITableRename_Impl: Sized {
    fn RenameColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, poldcolumnid: *const super::super::Storage::IndexServer::DBID, pnewcolumnid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
    fn RenameTable(&self, poldtableid: *const super::super::Storage::IndexServer::DBID, poldindexid: *const super::super::Storage::IndexServer::DBID, pnewtableid: *const super::super::Storage::IndexServer::DBID, pnewindexid: *const super::super::Storage::IndexServer::DBID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for ITableRename {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ITableRename_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITableRename_Vtbl
    where
        Identity: ITableRename_Impl,
    {
        unsafe extern "system" fn RenameColumn<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, poldcolumnid: *const super::super::Storage::IndexServer::DBID, pnewcolumnid: *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: ITableRename_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableRename_Impl::RenameColumn(this, core::mem::transmute_copy(&ptableid), core::mem::transmute_copy(&poldcolumnid), core::mem::transmute_copy(&pnewcolumnid)).into()
        }
        unsafe extern "system" fn RenameTable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, poldtableid: *const super::super::Storage::IndexServer::DBID, poldindexid: *const super::super::Storage::IndexServer::DBID, pnewtableid: *const super::super::Storage::IndexServer::DBID, pnewindexid: *const super::super::Storage::IndexServer::DBID) -> windows_core::HRESULT
        where
            Identity: ITableRename_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITableRename_Impl::RenameTable(this, core::mem::transmute_copy(&poldtableid), core::mem::transmute_copy(&poldindexid), core::mem::transmute_copy(&pnewtableid), core::mem::transmute_copy(&pnewindexid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RenameColumn: RenameColumn::<Identity, OFFSET>,
            RenameTable: RenameTable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableRename as windows_core::Interface>::IID
    }
}
pub trait ITokenCollection_Impl: Sized {
    fn NumberOfTokens(&self, pcount: *const u32) -> windows_core::Result<()>;
    fn GetToken(&self, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITokenCollection {}
impl ITokenCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITokenCollection_Vtbl
    where
        Identity: ITokenCollection_Impl,
    {
        unsafe extern "system" fn NumberOfTokens<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *const u32) -> windows_core::HRESULT
        where
            Identity: ITokenCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITokenCollection_Impl::NumberOfTokens(this, core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn GetToken<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: ITokenCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITokenCollection_Impl::GetToken(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&pbegin), core::mem::transmute_copy(&plength), core::mem::transmute_copy(&ppsz)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NumberOfTokens: NumberOfTokens::<Identity, OFFSET>,
            GetToken: GetToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITokenCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionJoin_Impl: Sized {
    fn GetOptionsObject(&self) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransactionOptions>;
    fn JoinTransaction(&self, punktransactioncoord: Option<&windows_core::IUnknown>, isolevel: i32, isoflags: u32, potheroptions: Option<&super::DistributedTransactionCoordinator::ITransactionOptions>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for ITransactionJoin {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionJoin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionJoin_Vtbl
    where
        Identity: ITransactionJoin_Impl,
    {
        unsafe extern "system" fn GetOptionsObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionJoin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionJoin_Impl::GetOptionsObject(this) {
                Ok(ok__) => {
                    ppoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punktransactioncoord: *mut core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionJoin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionJoin_Impl::JoinTransaction(this, windows_core::from_raw_borrowed(&punktransactioncoord), core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), windows_core::from_raw_borrowed(&potheroptions)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, OFFSET>,
            JoinTransaction: JoinTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionJoin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionLocal_Impl: Sized + super::DistributedTransactionCoordinator::ITransaction_Impl {
    fn GetOptionsObject(&self) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransactionOptions>;
    fn StartTransaction(&self, isolevel: i32, isoflags: u32, potheroptions: Option<&super::DistributedTransactionCoordinator::ITransactionOptions>, pultransactionlevel: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for ITransactionLocal {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionLocal_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionLocal_Vtbl
    where
        Identity: ITransactionLocal_Impl,
    {
        unsafe extern "system" fn GetOptionsObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionLocal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionLocal_Impl::GetOptionsObject(this) {
                Ok(ok__) => {
                    ppoptions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTransaction<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: *mut core::ffi::c_void, pultransactionlevel: *mut u32) -> windows_core::HRESULT
        where
            Identity: ITransactionLocal_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITransactionLocal_Impl::StartTransaction(this, core::mem::transmute_copy(&isolevel), core::mem::transmute_copy(&isoflags), windows_core::from_raw_borrowed(&potheroptions), core::mem::transmute_copy(&pultransactionlevel)).into()
        }
        Self {
            base__: super::DistributedTransactionCoordinator::ITransaction_Vtbl::new::<Identity, OFFSET>(),
            GetOptionsObject: GetOptionsObject::<Identity, OFFSET>,
            StartTransaction: StartTransaction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionLocal as windows_core::Interface>::IID || iid == &<super::DistributedTransactionCoordinator::ITransaction as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionObject_Impl: Sized {
    fn GetTransactionObject(&self, ultransactionlevel: u32) -> windows_core::Result<super::DistributedTransactionCoordinator::ITransaction>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl windows_core::RuntimeName for ITransactionObject {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITransactionObject_Vtbl
    where
        Identity: ITransactionObject_Impl,
    {
        unsafe extern "system" fn GetTransactionObject<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultransactionlevel: u32, pptransactionobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ITransactionObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITransactionObject_Impl::GetTransactionObject(this, core::mem::transmute_copy(&ultransactionlevel)) {
                Ok(ok__) => {
                    pptransactionobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTransactionObject: GetTransactionObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransactionObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
pub trait ITrusteeAdmin_Impl: Sized {
    fn CompareTrustees(&self, ptrustee1: *const super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()>;
    fn CreateTrustee(&self, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
    fn DeleteTrustee(&self, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()>;
    fn SetTrusteeProperties(&self, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::Result<()>;
    fn GetTrusteeProperties(&self, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
impl windows_core::RuntimeName for ITrusteeAdmin {}
#[cfg(all(feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
impl ITrusteeAdmin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITrusteeAdmin_Vtbl
    where
        Identity: ITrusteeAdmin_Impl,
    {
        unsafe extern "system" fn CompareTrustees<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee1: *const super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT
        where
            Identity: ITrusteeAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrusteeAdmin_Impl::CompareTrustees(this, core::mem::transmute_copy(&ptrustee1), core::mem::transmute_copy(&ptrustee2)).into()
        }
        unsafe extern "system" fn CreateTrustee<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: ITrusteeAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrusteeAdmin_Impl::CreateTrustee(this, core::mem::transmute_copy(&ptrustee), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
        }
        unsafe extern "system" fn DeleteTrustee<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT
        where
            Identity: ITrusteeAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrusteeAdmin_Impl::DeleteTrustee(this, core::mem::transmute_copy(&ptrustee)).into()
        }
        unsafe extern "system" fn SetTrusteeProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: ITrusteeAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrusteeAdmin_Impl::SetTrusteeProperties(this, core::mem::transmute_copy(&ptrustee), core::mem::transmute_copy(&cpropertysets), core::mem::transmute_copy(&rgpropertysets)).into()
        }
        unsafe extern "system" fn GetTrusteeProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> windows_core::HRESULT
        where
            Identity: ITrusteeAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrusteeAdmin_Impl::GetTrusteeProperties(this, core::mem::transmute_copy(&ptrustee), core::mem::transmute_copy(&cpropertyidsets), core::mem::transmute_copy(&rgpropertyidsets), core::mem::transmute_copy(&pcpropertysets), core::mem::transmute_copy(&prgpropertysets)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompareTrustees: CompareTrustees::<Identity, OFFSET>,
            CreateTrustee: CreateTrustee::<Identity, OFFSET>,
            DeleteTrustee: DeleteTrustee::<Identity, OFFSET>,
            SetTrusteeProperties: SetTrusteeProperties::<Identity, OFFSET>,
            GetTrusteeProperties: GetTrusteeProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrusteeAdmin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security_Authorization")]
pub trait ITrusteeGroupAdmin_Impl: Sized {
    fn AddMember(&self, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()>;
    fn DeleteMember(&self, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()>;
    fn IsMember(&self, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetMembers(&self, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()>;
    fn GetMemberships(&self, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Security_Authorization")]
impl windows_core::RuntimeName for ITrusteeGroupAdmin {}
#[cfg(feature = "Win32_Security_Authorization")]
impl ITrusteeGroupAdmin_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ITrusteeGroupAdmin_Vtbl
    where
        Identity: ITrusteeGroupAdmin_Impl,
    {
        unsafe extern "system" fn AddMember<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT
        where
            Identity: ITrusteeGroupAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrusteeGroupAdmin_Impl::AddMember(this, core::mem::transmute_copy(&pmembershiptrustee), core::mem::transmute_copy(&pmembertrustee)).into()
        }
        unsafe extern "system" fn DeleteMember<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT
        where
            Identity: ITrusteeGroupAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrusteeGroupAdmin_Impl::DeleteMember(this, core::mem::transmute_copy(&pmembershiptrustee), core::mem::transmute_copy(&pmembertrustee)).into()
        }
        unsafe extern "system" fn IsMember<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W, pfstatus: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ITrusteeGroupAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITrusteeGroupAdmin_Impl::IsMember(this, core::mem::transmute_copy(&pmembershiptrustee), core::mem::transmute_copy(&pmembertrustee)) {
                Ok(ok__) => {
                    pfstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMembers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT
        where
            Identity: ITrusteeGroupAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrusteeGroupAdmin_Impl::GetMembers(this, core::mem::transmute_copy(&pmembershiptrustee), core::mem::transmute_copy(&pcmembers), core::mem::transmute_copy(&prgmembers)).into()
        }
        unsafe extern "system" fn GetMemberships<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> windows_core::HRESULT
        where
            Identity: ITrusteeGroupAdmin_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITrusteeGroupAdmin_Impl::GetMemberships(this, core::mem::transmute_copy(&ptrustee), core::mem::transmute_copy(&pcmemberships), core::mem::transmute_copy(&prgmemberships)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddMember: AddMember::<Identity, OFFSET>,
            DeleteMember: DeleteMember::<Identity, OFFSET>,
            IsMember: IsMember::<Identity, OFFSET>,
            GetMembers: GetMembers::<Identity, OFFSET>,
            GetMemberships: GetMemberships::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITrusteeGroupAdmin as windows_core::Interface>::IID
    }
}
pub trait IUMS_Impl: Sized {
    fn SqlUmsSuspend(&self, ticks: u32);
    fn SqlUmsYield(&self, ticks: u32);
    fn SqlUmsSwitchPremptive(&self);
    fn SqlUmsSwitchNonPremptive(&self);
    fn SqlUmsFIsPremptive(&self) -> super::super::Foundation::BOOL;
}
impl IUMS_Vtbl {
    pub const fn new<Impl: IUMS_Impl>() -> IUMS_Vtbl {
        unsafe extern "system" fn SqlUmsSuspend<Impl: IUMS_Impl>(this: *mut core::ffi::c_void, ticks: u32) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IUMS_Impl::SqlUmsSuspend(this, core::mem::transmute_copy(&ticks))
        }
        unsafe extern "system" fn SqlUmsYield<Impl: IUMS_Impl>(this: *mut core::ffi::c_void, ticks: u32) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IUMS_Impl::SqlUmsYield(this, core::mem::transmute_copy(&ticks))
        }
        unsafe extern "system" fn SqlUmsSwitchPremptive<Impl: IUMS_Impl>(this: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IUMS_Impl::SqlUmsSwitchPremptive(this)
        }
        unsafe extern "system" fn SqlUmsSwitchNonPremptive<Impl: IUMS_Impl>(this: *mut core::ffi::c_void) {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IUMS_Impl::SqlUmsSwitchNonPremptive(this)
        }
        unsafe extern "system" fn SqlUmsFIsPremptive<Impl: IUMS_Impl>(this: *mut core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            IUMS_Impl::SqlUmsFIsPremptive(this)
        }
        Self {
            SqlUmsSuspend: SqlUmsSuspend::<Impl>,
            SqlUmsYield: SqlUmsYield::<Impl>,
            SqlUmsSwitchPremptive: SqlUmsSwitchPremptive::<Impl>,
            SqlUmsSwitchNonPremptive: SqlUmsSwitchNonPremptive::<Impl>,
            SqlUmsFIsPremptive: SqlUmsFIsPremptive::<Impl>,
        }
    }
}
#[doc(hidden)]
struct IUMS_ImplVtbl<T: IUMS_Impl>(std::marker::PhantomData<T>);
impl<T: IUMS_Impl> IUMS_ImplVtbl<T> {
    const VTABLE: IUMS_Vtbl = IUMS_Vtbl::new::<T>();
}
impl IUMS {
    pub fn new<'a, T: IUMS_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IUMS_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait IUMSInitialize_Impl: Sized {
    fn Initialize(&self, pums: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IUMSInitialize {}
impl IUMSInitialize_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUMSInitialize_Vtbl
    where
        Identity: IUMSInitialize_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pums: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUMSInitialize_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUMSInitialize_Impl::Initialize(this, core::mem::transmute_copy(&pums)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUMSInitialize as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IUrlAccessor_Impl: Sized {
    fn AddRequestParameter(&self, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetDocFormat(&self, wszdocformat: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn GetCLSID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetHost(&self, wszhost: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn IsDirectory(&self) -> windows_core::Result<()>;
    fn GetSize(&self) -> windows_core::Result<u64>;
    fn GetLastModified(&self) -> windows_core::Result<super::super::Foundation::FILETIME>;
    fn GetFileName(&self, wszfilename: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn GetSecurityDescriptor(&self, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn GetRedirectedURL(&self, wszredirectedurl: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn GetSecurityProvider(&self) -> windows_core::Result<windows_core::GUID>;
    fn BindToStream(&self) -> windows_core::Result<super::Com::IStream>;
    fn BindToFilter(&self) -> windows_core::Result<super::super::Storage::IndexServer::IFilter>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IUrlAccessor {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl IUrlAccessor_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUrlAccessor_Vtbl
    where
        Identity: IUrlAccessor_Impl,
    {
        unsafe extern "system" fn AddRequestParameter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor_Impl::AddRequestParameter(this, core::mem::transmute_copy(&pspec), core::mem::transmute_copy(&pvar)).into()
        }
        unsafe extern "system" fn GetDocFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdocformat: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor_Impl::GetDocFormat(this, core::mem::transmute_copy(&wszdocformat), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetCLSID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUrlAccessor_Impl::GetCLSID(this) {
                Ok(ok__) => {
                    pclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHost<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszhost: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor_Impl::GetHost(this, core::mem::transmute_copy(&wszhost), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn IsDirectory<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor_Impl::IsDirectory(this).into()
        }
        unsafe extern "system" fn GetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pllsize: *mut u64) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUrlAccessor_Impl::GetSize(this) {
                Ok(ok__) => {
                    pllsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastModified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftlastmodified: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUrlAccessor_Impl::GetLastModified(this) {
                Ok(ok__) => {
                    pftlastmodified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfilename: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor_Impl::GetFileName(this, core::mem::transmute_copy(&wszfilename), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor_Impl::GetSecurityDescriptor(this, core::mem::transmute_copy(&psd), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetRedirectedURL<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszredirectedurl: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor_Impl::GetRedirectedURL(this, core::mem::transmute_copy(&wszredirectedurl), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSecurityProvider<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspclsid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUrlAccessor_Impl::GetSecurityProvider(this) {
                Ok(ok__) => {
                    pspclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUrlAccessor_Impl::BindToStream(this) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfilter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUrlAccessor_Impl::BindToFilter(this) {
                Ok(ok__) => {
                    ppfilter.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRequestParameter: AddRequestParameter::<Identity, OFFSET>,
            GetDocFormat: GetDocFormat::<Identity, OFFSET>,
            GetCLSID: GetCLSID::<Identity, OFFSET>,
            GetHost: GetHost::<Identity, OFFSET>,
            IsDirectory: IsDirectory::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetLastModified: GetLastModified::<Identity, OFFSET>,
            GetFileName: GetFileName::<Identity, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, OFFSET>,
            GetRedirectedURL: GetRedirectedURL::<Identity, OFFSET>,
            GetSecurityProvider: GetSecurityProvider::<Identity, OFFSET>,
            BindToStream: BindToStream::<Identity, OFFSET>,
            BindToFilter: BindToFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlAccessor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IUrlAccessor2_Impl: Sized + IUrlAccessor_Impl {
    fn GetDisplayUrl(&self, wszdocurl: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
    fn IsDocument(&self) -> windows_core::Result<()>;
    fn GetCodePage(&self, wszcodepage: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IUrlAccessor2 {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl IUrlAccessor2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUrlAccessor2_Vtbl
    where
        Identity: IUrlAccessor2_Impl,
    {
        unsafe extern "system" fn GetDisplayUrl<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdocurl: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor2_Impl::GetDisplayUrl(this, core::mem::transmute_copy(&wszdocurl), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn IsDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor2_Impl::IsDocument(this).into()
        }
        unsafe extern "system" fn GetCodePage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszcodepage: windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor2_Impl::GetCodePage(this, core::mem::transmute_copy(&wszcodepage), core::mem::transmute_copy(&dwsize), core::mem::transmute_copy(&pdwlength)).into()
        }
        Self {
            base__: IUrlAccessor_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayUrl: GetDisplayUrl::<Identity, OFFSET>,
            IsDocument: IsDocument::<Identity, OFFSET>,
            GetCodePage: GetCodePage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlAccessor2 as windows_core::Interface>::IID || iid == &<IUrlAccessor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IUrlAccessor3_Impl: Sized + IUrlAccessor2_Impl {
    fn GetImpersonationSidBlobs(&self, pcwszurl: &windows_core::PCWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::RuntimeName for IUrlAccessor3 {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl IUrlAccessor3_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUrlAccessor3_Vtbl
    where
        Identity: IUrlAccessor3_Impl,
    {
        unsafe extern "system" fn GetImpersonationSidBlobs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwszurl: windows_core::PCWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor3_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IUrlAccessor3_Impl::GetImpersonationSidBlobs(this, core::mem::transmute(&pcwszurl), core::mem::transmute_copy(&pcsidcount), core::mem::transmute_copy(&ppsidblobs)).into()
        }
        Self { base__: IUrlAccessor2_Vtbl::new::<Identity, OFFSET>(), GetImpersonationSidBlobs: GetImpersonationSidBlobs::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlAccessor3 as windows_core::Interface>::IID || iid == &<IUrlAccessor as windows_core::Interface>::IID || iid == &<IUrlAccessor2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUrlAccessor4_Impl: Sized + IUrlAccessor3_Impl {
    fn ShouldIndexItemContent(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn ShouldIndexProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl windows_core::RuntimeName for IUrlAccessor4 {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IUrlAccessor4_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IUrlAccessor4_Vtbl
    where
        Identity: IUrlAccessor4_Impl,
    {
        unsafe extern "system" fn ShouldIndexItemContent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfindexcontent: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUrlAccessor4_Impl::ShouldIndexItemContent(this) {
                Ok(ok__) => {
                    pfindexcontent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldIndexProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfindexproperty: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IUrlAccessor4_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IUrlAccessor4_Impl::ShouldIndexProperty(this, core::mem::transmute_copy(&key)) {
                Ok(ok__) => {
                    pfindexproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IUrlAccessor3_Vtbl::new::<Identity, OFFSET>(),
            ShouldIndexItemContent: ShouldIndexItemContent::<Identity, OFFSET>,
            ShouldIndexProperty: ShouldIndexProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlAccessor4 as windows_core::Interface>::IID || iid == &<IUrlAccessor as windows_core::Interface>::IID || iid == &<IUrlAccessor2 as windows_core::Interface>::IID || iid == &<IUrlAccessor3 as windows_core::Interface>::IID
    }
}
pub trait IViewChapter_Impl: Sized {
    fn GetSpecification(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn OpenViewChapter(&self, hsource: usize, phviewchapter: *mut usize) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IViewChapter {}
impl IViewChapter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewChapter_Vtbl
    where
        Identity: IViewChapter_Impl,
    {
        unsafe extern "system" fn GetSpecification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewChapter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewChapter_Impl::GetSpecification(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    pprowset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenViewChapter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsource: usize, phviewchapter: *mut usize) -> windows_core::HRESULT
        where
            Identity: IViewChapter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewChapter_Impl::OpenViewChapter(this, core::mem::transmute_copy(&hsource), core::mem::transmute_copy(&phviewchapter)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSpecification: GetSpecification::<Identity, OFFSET>,
            OpenViewChapter: OpenViewChapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewChapter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IViewFilter_Impl: Sized {
    fn GetFilter(&self, haccessor: HACCESSOR, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetFilterBindings(&self, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> windows_core::Result<()>;
    fn SetFilter(&self, haccessor: HACCESSOR, crows: usize, compareops: *const u32, pcriteriadata: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IViewFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IViewFilter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewFilter_Vtbl
    where
        Identity: IViewFilter_Impl,
    {
        unsafe extern "system" fn GetFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewFilter_Impl::GetFilter(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&pcrows), core::mem::transmute_copy(&pcompareops), core::mem::transmute_copy(&pcriteriadata)).into()
        }
        unsafe extern "system" fn GetFilterBindings<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> windows_core::HRESULT
        where
            Identity: IViewFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewFilter_Impl::GetFilterBindings(this, core::mem::transmute_copy(&pcbindings), core::mem::transmute_copy(&prgbindings)).into()
        }
        unsafe extern "system" fn SetFilter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, haccessor: HACCESSOR, crows: usize, compareops: *const u32, pcriteriadata: *const core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewFilter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewFilter_Impl::SetFilter(this, core::mem::transmute_copy(&haccessor), core::mem::transmute_copy(&crows), core::mem::transmute_copy(&compareops), core::mem::transmute_copy(&pcriteriadata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFilter: GetFilter::<Identity, OFFSET>,
            GetFilterBindings: GetFilterBindings::<Identity, OFFSET>,
            SetFilter: SetFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewFilter as windows_core::Interface>::IID
    }
}
pub trait IViewRowset_Impl: Sized {
    fn GetSpecification(&self, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
    fn OpenViewRowset(&self, punkouter: Option<&windows_core::IUnknown>, riid: *const windows_core::GUID) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IViewRowset {}
impl IViewRowset_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewRowset_Vtbl
    where
        Identity: IViewRowset_Impl,
    {
        unsafe extern "system" fn GetSpecification<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewRowset_Impl::GetSpecification(this, core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    ppobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenViewRowset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, riid: *const windows_core::GUID, pprowset: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IViewRowset_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IViewRowset_Impl::OpenViewRowset(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&riid)) {
                Ok(ok__) => {
                    pprowset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSpecification: GetSpecification::<Identity, OFFSET>,
            OpenViewRowset: OpenViewRowset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewRowset as windows_core::Interface>::IID
    }
}
pub trait IViewSort_Impl: Sized {
    fn GetSortOrder(&self, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> windows_core::Result<()>;
    fn SetSortOrder(&self, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IViewSort {}
impl IViewSort_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IViewSort_Vtbl
    where
        Identity: IViewSort_Impl,
    {
        unsafe extern "system" fn GetSortOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> windows_core::HRESULT
        where
            Identity: IViewSort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewSort_Impl::GetSortOrder(this, core::mem::transmute_copy(&pcvalues), core::mem::transmute_copy(&prgcolumns), core::mem::transmute_copy(&prgorders)).into()
        }
        unsafe extern "system" fn SetSortOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> windows_core::HRESULT
        where
            Identity: IViewSort_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IViewSort_Impl::SetSortOrder(this, core::mem::transmute_copy(&cvalues), core::mem::transmute_copy(&rgcolumns), core::mem::transmute_copy(&rgorders)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSortOrder: GetSortOrder::<Identity, OFFSET>,
            SetSortOrder: SetSortOrder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IViewSort as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IWordBreaker_Impl: Sized {
    fn Init(&self, fquery: super::super::Foundation::BOOL, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn BreakText(&self, ptextsource: *mut TEXT_SOURCE, pwordsink: Option<&IWordSink>, pphrasesink: Option<&super::super::Storage::IndexServer::IPhraseSink>) -> windows_core::Result<()>;
    fn ComposePhrase(&self, pwcnoun: &windows_core::PCWSTR, cwcnoun: u32, pwcmodifier: &windows_core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: &windows_core::PCWSTR, pcwcphrase: *mut u32) -> windows_core::Result<()>;
    fn GetLicenseToUse(&self, ppwcslicense: *const *const u16) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IWordBreaker {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IWordBreaker_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWordBreaker_Vtbl
    where
        Identity: IWordBreaker_Impl,
    {
        unsafe extern "system" fn Init<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fquery: super::super::Foundation::BOOL, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWordBreaker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreaker_Impl::Init(this, core::mem::transmute_copy(&fquery), core::mem::transmute_copy(&ulmaxtokensize), core::mem::transmute_copy(&pflicense)).into()
        }
        unsafe extern "system" fn BreakText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptextsource: *mut TEXT_SOURCE, pwordsink: *mut core::ffi::c_void, pphrasesink: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWordBreaker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreaker_Impl::BreakText(this, core::mem::transmute_copy(&ptextsource), windows_core::from_raw_borrowed(&pwordsink), windows_core::from_raw_borrowed(&pphrasesink)).into()
        }
        unsafe extern "system" fn ComposePhrase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcnoun: windows_core::PCWSTR, cwcnoun: u32, pwcmodifier: windows_core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: windows_core::PCWSTR, pcwcphrase: *mut u32) -> windows_core::HRESULT
        where
            Identity: IWordBreaker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreaker_Impl::ComposePhrase(this, core::mem::transmute(&pwcnoun), core::mem::transmute_copy(&cwcnoun), core::mem::transmute(&pwcmodifier), core::mem::transmute_copy(&cwcmodifier), core::mem::transmute_copy(&ulattachmenttype), core::mem::transmute(&pwcphrase), core::mem::transmute_copy(&pcwcphrase)).into()
        }
        unsafe extern "system" fn GetLicenseToUse<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwcslicense: *const *const u16) -> windows_core::HRESULT
        where
            Identity: IWordBreaker_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordBreaker_Impl::GetLicenseToUse(this, core::mem::transmute_copy(&ppwcslicense)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            BreakText: BreakText::<Identity, OFFSET>,
            ComposePhrase: ComposePhrase::<Identity, OFFSET>,
            GetLicenseToUse: GetLicenseToUse::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWordBreaker as windows_core::Interface>::IID
    }
}
pub trait IWordFormSink_Impl: Sized {
    fn PutAltWord(&self, pwcinbuf: &windows_core::PCWSTR, cwc: u32) -> windows_core::Result<()>;
    fn PutWord(&self, pwcinbuf: &windows_core::PCWSTR, cwc: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWordFormSink {}
impl IWordFormSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWordFormSink_Vtbl
    where
        Identity: IWordFormSink_Impl,
    {
        unsafe extern "system" fn PutAltWord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcinbuf: windows_core::PCWSTR, cwc: u32) -> windows_core::HRESULT
        where
            Identity: IWordFormSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordFormSink_Impl::PutAltWord(this, core::mem::transmute(&pwcinbuf), core::mem::transmute_copy(&cwc)).into()
        }
        unsafe extern "system" fn PutWord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcinbuf: windows_core::PCWSTR, cwc: u32) -> windows_core::HRESULT
        where
            Identity: IWordFormSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordFormSink_Impl::PutWord(this, core::mem::transmute(&pwcinbuf), core::mem::transmute_copy(&cwc)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PutAltWord: PutAltWord::<Identity, OFFSET>, PutWord: PutWord::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWordFormSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IWordSink_Impl: Sized {
    fn PutWord(&self, cwc: u32, pwcinbuf: &windows_core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> windows_core::Result<()>;
    fn PutAltWord(&self, cwc: u32, pwcinbuf: &windows_core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> windows_core::Result<()>;
    fn StartAltPhrase(&self) -> windows_core::Result<()>;
    fn EndAltPhrase(&self) -> windows_core::Result<()>;
    fn PutBreak(&self, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl windows_core::RuntimeName for IWordSink {}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl IWordSink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWordSink_Vtbl
    where
        Identity: IWordSink_Impl,
    {
        unsafe extern "system" fn PutWord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cwc: u32, pwcinbuf: windows_core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> windows_core::HRESULT
        where
            Identity: IWordSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordSink_Impl::PutWord(this, core::mem::transmute_copy(&cwc), core::mem::transmute(&pwcinbuf), core::mem::transmute_copy(&cwcsrclen), core::mem::transmute_copy(&cwcsrcpos)).into()
        }
        unsafe extern "system" fn PutAltWord<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cwc: u32, pwcinbuf: windows_core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> windows_core::HRESULT
        where
            Identity: IWordSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordSink_Impl::PutAltWord(this, core::mem::transmute_copy(&cwc), core::mem::transmute(&pwcinbuf), core::mem::transmute_copy(&cwcsrclen), core::mem::transmute_copy(&cwcsrcpos)).into()
        }
        unsafe extern "system" fn StartAltPhrase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWordSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordSink_Impl::StartAltPhrase(this).into()
        }
        unsafe extern "system" fn EndAltPhrase<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWordSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordSink_Impl::EndAltPhrase(this).into()
        }
        unsafe extern "system" fn PutBreak<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> windows_core::HRESULT
        where
            Identity: IWordSink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWordSink_Impl::PutBreak(this, core::mem::transmute_copy(&breaktype)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutWord: PutWord::<Identity, OFFSET>,
            PutAltWord: PutAltWord::<Identity, OFFSET>,
            StartAltPhrase: StartAltPhrase::<Identity, OFFSET>,
            EndAltPhrase: EndAltPhrase::<Identity, OFFSET>,
            PutBreak: PutBreak::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWordSink as windows_core::Interface>::IID
    }
}
pub trait OLEDBSimpleProvider_Impl: Sized {
    fn getRowCount(&self) -> windows_core::Result<isize>;
    fn getColumnCount(&self) -> windows_core::Result<isize>;
    fn getRWStatus(&self, irow: isize, icolumn: isize) -> windows_core::Result<OSPRW>;
    fn getVariant(&self, irow: isize, icolumn: isize, format: OSPFORMAT) -> windows_core::Result<windows_core::VARIANT>;
    fn setVariant(&self, irow: isize, icolumn: isize, format: OSPFORMAT, var: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn getLocale(&self) -> windows_core::Result<windows_core::BSTR>;
    fn deleteRows(&self, irow: isize, crows: isize) -> windows_core::Result<isize>;
    fn insertRows(&self, irow: isize, crows: isize) -> windows_core::Result<isize>;
    fn find(&self, irowstart: isize, icolumn: isize, val: &windows_core::VARIANT, findflags: OSPFIND, comptype: OSPCOMP) -> windows_core::Result<isize>;
    fn addOLEDBSimpleProviderListener(&self, pospilistener: Option<&OLEDBSimpleProviderListener>) -> windows_core::Result<()>;
    fn removeOLEDBSimpleProviderListener(&self, pospilistener: Option<&OLEDBSimpleProviderListener>) -> windows_core::Result<()>;
    fn isAsync(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn getEstimatedRows(&self) -> windows_core::Result<isize>;
    fn stopTransfer(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for OLEDBSimpleProvider {}
impl OLEDBSimpleProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> OLEDBSimpleProvider_Vtbl
    where
        Identity: OLEDBSimpleProvider_Impl,
    {
        unsafe extern "system" fn getRowCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcrows: *mut isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match OLEDBSimpleProvider_Impl::getRowCount(this) {
                Ok(ok__) => {
                    pcrows.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getColumnCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccolumns: *mut isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match OLEDBSimpleProvider_Impl::getColumnCount(this) {
                Ok(ok__) => {
                    pccolumns.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getRWStatus<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, icolumn: isize, prwstatus: *mut OSPRW) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match OLEDBSimpleProvider_Impl::getRWStatus(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&icolumn)) {
                Ok(ok__) => {
                    prwstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getVariant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, icolumn: isize, format: OSPFORMAT, pvar: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match OLEDBSimpleProvider_Impl::getVariant(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&icolumn), core::mem::transmute_copy(&format)) {
                Ok(ok__) => {
                    pvar.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setVariant<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, icolumn: isize, format: OSPFORMAT, var: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProvider_Impl::setVariant(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&icolumn), core::mem::transmute_copy(&format), core::mem::transmute(&var)).into()
        }
        unsafe extern "system" fn getLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlocale: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match OLEDBSimpleProvider_Impl::getLocale(this) {
                Ok(ok__) => {
                    pbstrlocale.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn deleteRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, crows: isize, pcrowsdeleted: *mut isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match OLEDBSimpleProvider_Impl::deleteRows(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&crows)) {
                Ok(ok__) => {
                    pcrowsdeleted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, crows: isize, pcrowsinserted: *mut isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match OLEDBSimpleProvider_Impl::insertRows(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&crows)) {
                Ok(ok__) => {
                    pcrowsinserted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn find<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irowstart: isize, icolumn: isize, val: core::mem::MaybeUninit<windows_core::VARIANT>, findflags: OSPFIND, comptype: OSPCOMP, pirowfound: *mut isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match OLEDBSimpleProvider_Impl::find(this, core::mem::transmute_copy(&irowstart), core::mem::transmute_copy(&icolumn), core::mem::transmute(&val), core::mem::transmute_copy(&findflags), core::mem::transmute_copy(&comptype)) {
                Ok(ok__) => {
                    pirowfound.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addOLEDBSimpleProviderListener<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pospilistener: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProvider_Impl::addOLEDBSimpleProviderListener(this, windows_core::from_raw_borrowed(&pospilistener)).into()
        }
        unsafe extern "system" fn removeOLEDBSimpleProviderListener<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pospilistener: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProvider_Impl::removeOLEDBSimpleProviderListener(this, windows_core::from_raw_borrowed(&pospilistener)).into()
        }
        unsafe extern "system" fn isAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbasynch: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match OLEDBSimpleProvider_Impl::isAsync(this) {
                Ok(ok__) => {
                    pbasynch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getEstimatedRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pirows: *mut isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match OLEDBSimpleProvider_Impl::getEstimatedRows(this) {
                Ok(ok__) => {
                    pirows.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stopTransfer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProvider_Impl::stopTransfer(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            getRowCount: getRowCount::<Identity, OFFSET>,
            getColumnCount: getColumnCount::<Identity, OFFSET>,
            getRWStatus: getRWStatus::<Identity, OFFSET>,
            getVariant: getVariant::<Identity, OFFSET>,
            setVariant: setVariant::<Identity, OFFSET>,
            getLocale: getLocale::<Identity, OFFSET>,
            deleteRows: deleteRows::<Identity, OFFSET>,
            insertRows: insertRows::<Identity, OFFSET>,
            find: find::<Identity, OFFSET>,
            addOLEDBSimpleProviderListener: addOLEDBSimpleProviderListener::<Identity, OFFSET>,
            removeOLEDBSimpleProviderListener: removeOLEDBSimpleProviderListener::<Identity, OFFSET>,
            isAsync: isAsync::<Identity, OFFSET>,
            getEstimatedRows: getEstimatedRows::<Identity, OFFSET>,
            stopTransfer: stopTransfer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<OLEDBSimpleProvider as windows_core::Interface>::IID
    }
}
pub trait OLEDBSimpleProviderListener_Impl: Sized {
    fn aboutToChangeCell(&self, irow: isize, icolumn: isize) -> windows_core::Result<()>;
    fn cellChanged(&self, irow: isize, icolumn: isize) -> windows_core::Result<()>;
    fn aboutToDeleteRows(&self, irow: isize, crows: isize) -> windows_core::Result<()>;
    fn deletedRows(&self, irow: isize, crows: isize) -> windows_core::Result<()>;
    fn aboutToInsertRows(&self, irow: isize, crows: isize) -> windows_core::Result<()>;
    fn insertedRows(&self, irow: isize, crows: isize) -> windows_core::Result<()>;
    fn rowsAvailable(&self, irow: isize, crows: isize) -> windows_core::Result<()>;
    fn transferComplete(&self, xfer: OSPXFER) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for OLEDBSimpleProviderListener {}
impl OLEDBSimpleProviderListener_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> OLEDBSimpleProviderListener_Vtbl
    where
        Identity: OLEDBSimpleProviderListener_Impl,
    {
        unsafe extern "system" fn aboutToChangeCell<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, icolumn: isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProviderListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProviderListener_Impl::aboutToChangeCell(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&icolumn)).into()
        }
        unsafe extern "system" fn cellChanged<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, icolumn: isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProviderListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProviderListener_Impl::cellChanged(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&icolumn)).into()
        }
        unsafe extern "system" fn aboutToDeleteRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, crows: isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProviderListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProviderListener_Impl::aboutToDeleteRows(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn deletedRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, crows: isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProviderListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProviderListener_Impl::deletedRows(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn aboutToInsertRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, crows: isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProviderListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProviderListener_Impl::aboutToInsertRows(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn insertedRows<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, crows: isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProviderListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProviderListener_Impl::insertedRows(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn rowsAvailable<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, irow: isize, crows: isize) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProviderListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProviderListener_Impl::rowsAvailable(this, core::mem::transmute_copy(&irow), core::mem::transmute_copy(&crows)).into()
        }
        unsafe extern "system" fn transferComplete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xfer: OSPXFER) -> windows_core::HRESULT
        where
            Identity: OLEDBSimpleProviderListener_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            OLEDBSimpleProviderListener_Impl::transferComplete(this, core::mem::transmute_copy(&xfer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            aboutToChangeCell: aboutToChangeCell::<Identity, OFFSET>,
            cellChanged: cellChanged::<Identity, OFFSET>,
            aboutToDeleteRows: aboutToDeleteRows::<Identity, OFFSET>,
            deletedRows: deletedRows::<Identity, OFFSET>,
            aboutToInsertRows: aboutToInsertRows::<Identity, OFFSET>,
            insertedRows: insertedRows::<Identity, OFFSET>,
            rowsAvailable: rowsAvailable::<Identity, OFFSET>,
            transferComplete: transferComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<OLEDBSimpleProviderListener as windows_core::Interface>::IID
    }
}
