pub trait IContact_Impl: Sized {
    fn GetContactID(&self, pszcontactid: &windows_core::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> windows_core::Result<()>;
    fn GetPath(&self, pszpath: &windows_core::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> windows_core::Result<()>;
    fn CommitChanges(&self, dwcommitflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContact {}
impl IContact_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContact_Vtbl
    where
        Identity: IContact_Impl,
    {
        unsafe extern "system" fn GetContactID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcontactid: windows_core::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContact_Impl::GetContactID(this, core::mem::transmute(&pszcontactid), core::mem::transmute_copy(&cchcontactid), core::mem::transmute_copy(&pdwcchcontactidrequired)).into()
        }
        unsafe extern "system" fn GetPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContact_Impl::GetPath(this, core::mem::transmute(&pszpath), core::mem::transmute_copy(&cchpath), core::mem::transmute_copy(&pdwcchpathrequired)).into()
        }
        unsafe extern "system" fn CommitChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcommitflags: u32) -> windows_core::HRESULT
        where
            Identity: IContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContact_Impl::CommitChanges(this, core::mem::transmute_copy(&dwcommitflags)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContactID: GetContactID::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            CommitChanges: CommitChanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContact as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationAggregate_Impl: Sized {
    fn Save(&self) -> windows_core::Result<()>;
    fn GetComponentItems(&self) -> windows_core::Result<IContactAggregationContactCollection>;
    fn Link(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationGroupCollection>;
    fn AntiLink(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAntiLink(&self, pantilink: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FavoriteOrder(&self) -> windows_core::Result<u32>;
    fn SetFavoriteOrder(&self, favoriteorder: u32) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for IContactAggregationAggregate {}
impl IContactAggregationAggregate_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationAggregate_Vtbl
    where
        Identity: IContactAggregationAggregate_Impl,
    {
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationAggregate_Impl::Save(this).into()
        }
        unsafe extern "system" fn GetComponentItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomponentitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationAggregate_Impl::GetComponentItems(this) {
                Ok(ok__) => {
                    pcomponentitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationAggregate_Impl::Link(this, core::mem::transmute(&paggregateid)).into()
        }
        unsafe extern "system" fn get_Groups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationAggregate_Impl::get_Groups(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    ppgroups.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntiLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppantilink: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationAggregate_Impl::AntiLink(this) {
                Ok(ok__) => {
                    ppantilink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pantilink: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationAggregate_Impl::SetAntiLink(this, core::mem::transmute(&pantilink)).into()
        }
        unsafe extern "system" fn FavoriteOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfavoriteorder: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationAggregate_Impl::FavoriteOrder(this) {
                Ok(ok__) => {
                    pfavoriteorder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFavoriteOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, favoriteorder: u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationAggregate_Impl::SetFavoriteOrder(this, core::mem::transmute_copy(&favoriteorder)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitemid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregate_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationAggregate_Impl::Id(this) {
                Ok(ok__) => {
                    ppitemid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Save: Save::<Identity, OFFSET>,
            GetComponentItems: GetComponentItems::<Identity, OFFSET>,
            Link: Link::<Identity, OFFSET>,
            get_Groups: get_Groups::<Identity, OFFSET>,
            AntiLink: AntiLink::<Identity, OFFSET>,
            SetAntiLink: SetAntiLink::<Identity, OFFSET>,
            FavoriteOrder: FavoriteOrder::<Identity, OFFSET>,
            SetFavoriteOrder: SetFavoriteOrder::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationAggregate as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationAggregateCollection_Impl: Sized {
    fn FindFirst(&self) -> windows_core::Result<IContactAggregationAggregate>;
    fn FindFirstByAntiLinkId(&self, pantilinkid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationAggregate>;
    fn FindNext(&self) -> windows_core::Result<IContactAggregationAggregate>;
    fn Count(&self) -> windows_core::Result<i32>;
}
impl windows_core::RuntimeName for IContactAggregationAggregateCollection {}
impl IContactAggregationAggregateCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationAggregateCollection_Vtbl
    where
        Identity: IContactAggregationAggregateCollection_Impl,
    {
        unsafe extern "system" fn FindFirst<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaggregate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationAggregateCollection_Impl::FindFirst(this) {
                Ok(ok__) => {
                    ppaggregate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByAntiLinkId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pantilinkid: windows_core::PCWSTR, ppaggregate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationAggregateCollection_Impl::FindFirstByAntiLinkId(this, core::mem::transmute(&pantilinkid)) {
                Ok(ok__) => {
                    ppaggregate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaggregate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationAggregateCollection_Impl::FindNext(this) {
                Ok(ok__) => {
                    ppaggregate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationAggregateCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationAggregateCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindFirstByAntiLinkId: FindFirstByAntiLinkId::<Identity, OFFSET>,
            FindNext: FindNext::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationAggregateCollection as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationContact_Impl: Sized {
    fn Delete(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn MoveToAggregate(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Unlink(&self) -> windows_core::Result<()>;
    fn AccountId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccountId(&self, paccountid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AggregateId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsMe(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsExternal(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn NetworkSourceId(&self) -> windows_core::Result<u32>;
    fn SetNetworkSourceId(&self, networksourceid: u32) -> windows_core::Result<()>;
    fn NetworkSourceIdString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetNetworkSourceIdString(&self, pnetworksourceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoteObjectId(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
    fn SyncIdentityHash(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContactAggregationContact {}
impl IContactAggregationContact_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationContact_Vtbl
    where
        Identity: IContactAggregationContact_Impl,
    {
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationContact_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationContact_Impl::Save(this).into()
        }
        unsafe extern "system" fn MoveToAggregate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationContact_Impl::MoveToAggregate(this, core::mem::transmute(&paggregateid)).into()
        }
        unsafe extern "system" fn Unlink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationContact_Impl::Unlink(this).into()
        }
        unsafe extern "system" fn AccountId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaccountid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContact_Impl::AccountId(this) {
                Ok(ok__) => {
                    ppaccountid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paccountid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationContact_Impl::SetAccountId(this, core::mem::transmute(&paccountid)).into()
        }
        unsafe extern "system" fn AggregateId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaggregateid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContact_Impl::AggregateId(this) {
                Ok(ok__) => {
                    ppaggregateid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitemid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContact_Impl::Id(this) {
                Ok(ok__) => {
                    ppitemid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMe<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisme: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContact_Impl::IsMe(this) {
                Ok(ok__) => {
                    pisme.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsExternal<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisexternal: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContact_Impl::IsExternal(this) {
                Ok(ok__) => {
                    pisexternal.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkSourceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworksourceid: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContact_Impl::NetworkSourceId(this) {
                Ok(ok__) => {
                    pnetworksourceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSourceId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, networksourceid: u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationContact_Impl::SetNetworkSourceId(this, core::mem::transmute_copy(&networksourceid)).into()
        }
        unsafe extern "system" fn NetworkSourceIdString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetworksourceid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContact_Impl::NetworkSourceIdString(this) {
                Ok(ok__) => {
                    ppnetworksourceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworksourceid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationContact_Impl::SetNetworkSourceIdString(this, core::mem::transmute(&pnetworksourceid)).into()
        }
        unsafe extern "system" fn RemoteObjectId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContact_Impl::RemoteObjectId(this) {
                Ok(ok__) => {
                    ppremoteobjectid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteObjectId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationContact_Impl::SetRemoteObjectId(this, core::mem::transmute_copy(&premoteobjectid)).into()
        }
        unsafe extern "system" fn SyncIdentityHash<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContact_Impl::SyncIdentityHash(this) {
                Ok(ok__) => {
                    ppsyncidentityhash.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncIdentityHash<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContact_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationContact_Impl::SetSyncIdentityHash(this, core::mem::transmute_copy(&psyncidentityhash)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            MoveToAggregate: MoveToAggregate::<Identity, OFFSET>,
            Unlink: Unlink::<Identity, OFFSET>,
            AccountId: AccountId::<Identity, OFFSET>,
            SetAccountId: SetAccountId::<Identity, OFFSET>,
            AggregateId: AggregateId::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            IsMe: IsMe::<Identity, OFFSET>,
            IsExternal: IsExternal::<Identity, OFFSET>,
            NetworkSourceId: NetworkSourceId::<Identity, OFFSET>,
            SetNetworkSourceId: SetNetworkSourceId::<Identity, OFFSET>,
            NetworkSourceIdString: NetworkSourceIdString::<Identity, OFFSET>,
            SetNetworkSourceIdString: SetNetworkSourceIdString::<Identity, OFFSET>,
            RemoteObjectId: RemoteObjectId::<Identity, OFFSET>,
            SetRemoteObjectId: SetRemoteObjectId::<Identity, OFFSET>,
            SyncIdentityHash: SyncIdentityHash::<Identity, OFFSET>,
            SetSyncIdentityHash: SetSyncIdentityHash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationContact as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationContactCollection_Impl: Sized {
    fn FindFirst(&self) -> windows_core::Result<IContactAggregationContact>;
    fn FindNext(&self) -> windows_core::Result<IContactAggregationContact>;
    fn FindFirstByIdentityHash(&self, psourcetype: &windows_core::PCWSTR, paccountid: &windows_core::PCWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<IContactAggregationContact>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn FindFirstByRemoteId(&self, psourcetype: &windows_core::PCWSTR, paccountid: &windows_core::PCWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<IContactAggregationContact>;
}
impl windows_core::RuntimeName for IContactAggregationContactCollection {}
impl IContactAggregationContactCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationContactCollection_Vtbl
    where
        Identity: IContactAggregationContactCollection_Impl,
    {
        unsafe extern "system" fn FindFirst<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContactCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContactCollection_Impl::FindFirst(this) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContactCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContactCollection_Impl::FindNext(this) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByIdentityHash<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetype: windows_core::PCWSTR, paccountid: windows_core::PCWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContactCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContactCollection_Impl::FindFirstByIdentityHash(this, core::mem::transmute(&psourcetype), core::mem::transmute(&paccountid), core::mem::transmute_copy(&pidentityhash)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContactCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContactCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByRemoteId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetype: windows_core::PCWSTR, paccountid: windows_core::PCWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationContactCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationContactCollection_Impl::FindFirstByRemoteId(this, core::mem::transmute(&psourcetype), core::mem::transmute(&paccountid), core::mem::transmute_copy(&premoteobjectid)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindNext: FindNext::<Identity, OFFSET>,
            FindFirstByIdentityHash: FindFirstByIdentityHash::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            FindFirstByRemoteId: FindFirstByRemoteId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationContactCollection as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationGroup_Impl: Sized {
    fn Delete(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn Add(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Remove(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Members(&self) -> windows_core::Result<IContactAggregationAggregateCollection>;
    fn GlobalObjectId(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGlobalObjectId(&self, pglobalobjectid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Name(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetName(&self, pname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContactAggregationGroup {}
impl IContactAggregationGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationGroup_Vtbl
    where
        Identity: IContactAggregationGroup_Impl,
    {
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationGroup_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationGroup_Impl::Save(this).into()
        }
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationGroup_Impl::Add(this, core::mem::transmute(&paggregateid)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationGroup_Impl::Remove(this, core::mem::transmute(&paggregateid)).into()
        }
        unsafe extern "system" fn Members<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaggregatecontactcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationGroup_Impl::Members(this) {
                Ok(ok__) => {
                    ppaggregatecontactcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalObjectId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pglobalobjectid: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationGroup_Impl::GlobalObjectId(this) {
                Ok(ok__) => {
                    pglobalobjectid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGlobalObjectId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pglobalobjectid: *const windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationGroup_Impl::SetGlobalObjectId(this, core::mem::transmute_copy(&pglobalobjectid)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitemid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationGroup_Impl::Id(this) {
                Ok(ok__) => {
                    ppitemid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationGroup_Impl::Name(this) {
                Ok(ok__) => {
                    ppname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationGroup_Impl::SetName(this, core::mem::transmute(&pname)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Members: Members::<Identity, OFFSET>,
            GlobalObjectId: GlobalObjectId::<Identity, OFFSET>,
            SetGlobalObjectId: SetGlobalObjectId::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationGroup as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationGroupCollection_Impl: Sized {
    fn FindFirst(&self) -> windows_core::Result<IContactAggregationGroup>;
    fn FindFirstByGlobalObjectId(&self, pglobalobjectid: *const windows_core::GUID) -> windows_core::Result<IContactAggregationGroup>;
    fn FindNext(&self) -> windows_core::Result<IContactAggregationGroup>;
    fn Count(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IContactAggregationGroupCollection {}
impl IContactAggregationGroupCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationGroupCollection_Vtbl
    where
        Identity: IContactAggregationGroupCollection_Impl,
    {
        unsafe extern "system" fn FindFirst<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroupCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationGroupCollection_Impl::FindFirst(this) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByGlobalObjectId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pglobalobjectid: *const windows_core::GUID, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroupCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationGroupCollection_Impl::FindFirstByGlobalObjectId(this, core::mem::transmute_copy(&pglobalobjectid)) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroupCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationGroupCollection_Impl::FindNext(this) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationGroupCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationGroupCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindFirstByGlobalObjectId: FindFirstByGlobalObjectId::<Identity, OFFSET>,
            FindNext: FindNext::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationGroupCollection as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationLink_Impl: Sized {
    fn Delete(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn AccountId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccountId(&self, paccountid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsLinkResolved(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsLinkResolved(&self, islinkresolved: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn NetworkSourceIdString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetNetworkSourceIdString(&self, pnetworksourceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoteObjectId(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
    fn ServerPerson(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetServerPerson(&self, pserverpersonid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ServerPersonBaseline(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetServerPersonBaseline(&self, pserverpersonid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SyncIdentityHash(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContactAggregationLink {}
impl IContactAggregationLink_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationLink_Vtbl
    where
        Identity: IContactAggregationLink_Impl,
    {
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationLink_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationLink_Impl::Save(this).into()
        }
        unsafe extern "system" fn AccountId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaccountid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLink_Impl::AccountId(this) {
                Ok(ok__) => {
                    ppaccountid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paccountid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationLink_Impl::SetAccountId(this, core::mem::transmute(&paccountid)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitemid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLink_Impl::Id(this) {
                Ok(ok__) => {
                    ppitemid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLinkResolved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislinkresolved: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLink_Impl::IsLinkResolved(this) {
                Ok(ok__) => {
                    pislinkresolved.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLinkResolved<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, islinkresolved: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationLink_Impl::SetIsLinkResolved(this, core::mem::transmute_copy(&islinkresolved)).into()
        }
        unsafe extern "system" fn NetworkSourceIdString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetworksourceid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLink_Impl::NetworkSourceIdString(this) {
                Ok(ok__) => {
                    ppnetworksourceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworksourceid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationLink_Impl::SetNetworkSourceIdString(this, core::mem::transmute(&pnetworksourceid)).into()
        }
        unsafe extern "system" fn RemoteObjectId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLink_Impl::RemoteObjectId(this) {
                Ok(ok__) => {
                    ppremoteobjectid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteObjectId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationLink_Impl::SetRemoteObjectId(this, core::mem::transmute_copy(&premoteobjectid)).into()
        }
        unsafe extern "system" fn ServerPerson<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverpersonid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLink_Impl::ServerPerson(this) {
                Ok(ok__) => {
                    ppserverpersonid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerPerson<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserverpersonid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationLink_Impl::SetServerPerson(this, core::mem::transmute(&pserverpersonid)).into()
        }
        unsafe extern "system" fn ServerPersonBaseline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverpersonid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLink_Impl::ServerPersonBaseline(this) {
                Ok(ok__) => {
                    ppserverpersonid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerPersonBaseline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserverpersonid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationLink_Impl::SetServerPersonBaseline(this, core::mem::transmute(&pserverpersonid)).into()
        }
        unsafe extern "system" fn SyncIdentityHash<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLink_Impl::SyncIdentityHash(this) {
                Ok(ok__) => {
                    ppsyncidentityhash.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncIdentityHash<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLink_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationLink_Impl::SetSyncIdentityHash(this, core::mem::transmute_copy(&psyncidentityhash)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            AccountId: AccountId::<Identity, OFFSET>,
            SetAccountId: SetAccountId::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            IsLinkResolved: IsLinkResolved::<Identity, OFFSET>,
            SetIsLinkResolved: SetIsLinkResolved::<Identity, OFFSET>,
            NetworkSourceIdString: NetworkSourceIdString::<Identity, OFFSET>,
            SetNetworkSourceIdString: SetNetworkSourceIdString::<Identity, OFFSET>,
            RemoteObjectId: RemoteObjectId::<Identity, OFFSET>,
            SetRemoteObjectId: SetRemoteObjectId::<Identity, OFFSET>,
            ServerPerson: ServerPerson::<Identity, OFFSET>,
            SetServerPerson: SetServerPerson::<Identity, OFFSET>,
            ServerPersonBaseline: ServerPersonBaseline::<Identity, OFFSET>,
            SetServerPersonBaseline: SetServerPersonBaseline::<Identity, OFFSET>,
            SyncIdentityHash: SyncIdentityHash::<Identity, OFFSET>,
            SetSyncIdentityHash: SetSyncIdentityHash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationLink as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationLinkCollection_Impl: Sized {
    fn FindFirst(&self) -> windows_core::Result<IContactAggregationLink>;
    fn FindFirstByRemoteId(&self, psourcetype: &windows_core::PCWSTR, paccountid: &windows_core::PCWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<IContactAggregationLink>;
    fn FindNext(&self) -> windows_core::Result<IContactAggregationLink>;
    fn Count(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IContactAggregationLinkCollection {}
impl IContactAggregationLinkCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationLinkCollection_Vtbl
    where
        Identity: IContactAggregationLinkCollection_Impl,
    {
        unsafe extern "system" fn FindFirst<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservercontactlink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLinkCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLinkCollection_Impl::FindFirst(this) {
                Ok(ok__) => {
                    ppservercontactlink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByRemoteId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetype: windows_core::PCWSTR, paccountid: windows_core::PCWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB, ppservercontactlink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLinkCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLinkCollection_Impl::FindFirstByRemoteId(this, core::mem::transmute(&psourcetype), core::mem::transmute(&paccountid), core::mem::transmute_copy(&premoteid)) {
                Ok(ok__) => {
                    ppservercontactlink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservercontactlink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLinkCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLinkCollection_Impl::FindNext(this) {
                Ok(ok__) => {
                    ppservercontactlink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationLinkCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationLinkCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindFirstByRemoteId: FindFirstByRemoteId::<Identity, OFFSET>,
            FindNext: FindNext::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationLinkCollection as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationManager_Impl: Sized {
    fn GetVersionInfo(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> windows_core::Result<()>;
    fn CreateOrOpenGroup(&self, pgroupname: &windows_core::PCWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL) -> windows_core::Result<IContactAggregationGroup>;
    fn CreateExternalContact(&self) -> windows_core::Result<IContactAggregationContact>;
    fn CreateServerPerson(&self) -> windows_core::Result<IContactAggregationServerPerson>;
    fn CreateServerContactLink(&self) -> windows_core::Result<IContactAggregationLink>;
    fn Flush(&self) -> windows_core::Result<()>;
    fn OpenAggregateContact(&self, pitemid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationAggregate>;
    fn OpenContact(&self, pitemid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationContact>;
    fn OpenServerContactLink(&self, pitemid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationLink>;
    fn OpenServerPerson(&self, pitemid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationServerPerson>;
    fn get_Contacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationContactCollection>;
    fn get_AggregateContacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationAggregateCollection>;
    fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationGroupCollection>;
    fn ServerPersons(&self) -> windows_core::Result<IContactAggregationServerPersonCollection>;
    fn get_ServerContactLinks(&self, ppersonitemid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationLinkCollection>;
}
impl windows_core::RuntimeName for IContactAggregationManager {}
impl IContactAggregationManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationManager_Vtbl
    where
        Identity: IContactAggregationManager_Impl,
    {
        unsafe extern "system" fn GetVersionInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationManager_Impl::GetVersionInfo(this, core::mem::transmute_copy(&plmajorversion), core::mem::transmute_copy(&plminorversion)).into()
        }
        unsafe extern "system" fn CreateOrOpenGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroupname: windows_core::PCWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::CreateOrOpenGroup(this, core::mem::transmute(&pgroupname), core::mem::transmute_copy(&options), core::mem::transmute_copy(&pcreatedgroup)) {
                Ok(ok__) => {
                    ppgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateExternalContact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::CreateExternalContact(this) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServerPerson<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::CreateServerPerson(this) {
                Ok(ok__) => {
                    ppserverperson.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServerContactLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservercontactlink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::CreateServerContactLink(this) {
                Ok(ok__) => {
                    ppservercontactlink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationManager_Impl::Flush(this).into()
        }
        unsafe extern "system" fn OpenAggregateContact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: windows_core::PCWSTR, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::OpenAggregateContact(this, core::mem::transmute(&pitemid)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenContact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: windows_core::PCWSTR, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::OpenContact(this, core::mem::transmute(&pitemid)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenServerContactLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: windows_core::PCWSTR, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::OpenServerContactLink(this, core::mem::transmute(&pitemid)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenServerPerson<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: windows_core::PCWSTR, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::OpenServerPerson(this, core::mem::transmute(&pitemid)) {
                Ok(ok__) => {
                    ppitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Contacts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::get_Contacts(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    ppitems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_AggregateContacts<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppaggregates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::get_AggregateContacts(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    ppaggregates.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Groups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::get_Groups(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    ppgroups.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerPersons<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverpersoncollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::ServerPersons(this) {
                Ok(ok__) => {
                    ppserverpersoncollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ServerContactLinks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppersonitemid: windows_core::PCWSTR, ppservercontactlinkcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationManager_Impl::get_ServerContactLinks(this, core::mem::transmute(&ppersonitemid)) {
                Ok(ok__) => {
                    ppservercontactlinkcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersionInfo: GetVersionInfo::<Identity, OFFSET>,
            CreateOrOpenGroup: CreateOrOpenGroup::<Identity, OFFSET>,
            CreateExternalContact: CreateExternalContact::<Identity, OFFSET>,
            CreateServerPerson: CreateServerPerson::<Identity, OFFSET>,
            CreateServerContactLink: CreateServerContactLink::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            OpenAggregateContact: OpenAggregateContact::<Identity, OFFSET>,
            OpenContact: OpenContact::<Identity, OFFSET>,
            OpenServerContactLink: OpenServerContactLink::<Identity, OFFSET>,
            OpenServerPerson: OpenServerPerson::<Identity, OFFSET>,
            get_Contacts: get_Contacts::<Identity, OFFSET>,
            get_AggregateContacts: get_AggregateContacts::<Identity, OFFSET>,
            get_Groups: get_Groups::<Identity, OFFSET>,
            ServerPersons: ServerPersons::<Identity, OFFSET>,
            get_ServerContactLinks: get_ServerContactLinks::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationManager as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationServerPerson_Impl: Sized {
    fn Delete(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn AggregateId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAggregateId(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AntiLink(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAntiLink(&self, pantilink: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AntiLinkBaseline(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAntiLinkBaseline(&self, pantilink: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FavoriteOrder(&self) -> windows_core::Result<u32>;
    fn SetFavoriteOrder(&self, favoriteorder: u32) -> windows_core::Result<()>;
    fn FavoriteOrderBaseline(&self) -> windows_core::Result<u32>;
    fn SetFavoriteOrderBaseline(&self, favoriteorder: u32) -> windows_core::Result<()>;
    fn Groups(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetGroups(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
    fn GroupsBaseline(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetGroupsBaseline(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsTombstone(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsTombstone(&self, istombstone: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn LinkedAggregateId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLinkedAggregateId(&self, plinkedaggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ObjectId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetObjectId(&self, pobjectid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContactAggregationServerPerson {}
impl IContactAggregationServerPerson_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationServerPerson_Vtbl
    where
        Identity: IContactAggregationServerPerson_Impl,
    {
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Save<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::Save(this).into()
        }
        unsafe extern "system" fn AggregateId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaggregateid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::AggregateId(this) {
                Ok(ok__) => {
                    ppaggregateid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAggregateId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::SetAggregateId(this, core::mem::transmute(&paggregateid)).into()
        }
        unsafe extern "system" fn AntiLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppantilink: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::AntiLink(this) {
                Ok(ok__) => {
                    ppantilink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiLink<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pantilink: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::SetAntiLink(this, core::mem::transmute(&pantilink)).into()
        }
        unsafe extern "system" fn AntiLinkBaseline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppantilink: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::AntiLinkBaseline(this) {
                Ok(ok__) => {
                    ppantilink.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAntiLinkBaseline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pantilink: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::SetAntiLinkBaseline(this, core::mem::transmute(&pantilink)).into()
        }
        unsafe extern "system" fn FavoriteOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfavoriteorder: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::FavoriteOrder(this) {
                Ok(ok__) => {
                    pfavoriteorder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFavoriteOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, favoriteorder: u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::SetFavoriteOrder(this, core::mem::transmute_copy(&favoriteorder)).into()
        }
        unsafe extern "system" fn FavoriteOrderBaseline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfavoriteorder: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::FavoriteOrderBaseline(this) {
                Ok(ok__) => {
                    pfavoriteorder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFavoriteOrderBaseline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, favoriteorder: u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::SetFavoriteOrderBaseline(this, core::mem::transmute_copy(&favoriteorder)).into()
        }
        unsafe extern "system" fn Groups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::Groups(this) {
                Ok(ok__) => {
                    pgroups.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::SetGroups(this, core::mem::transmute_copy(&pgroups)).into()
        }
        unsafe extern "system" fn GroupsBaseline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::GroupsBaseline(this) {
                Ok(ok__) => {
                    ppgroups.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupsBaseline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::SetGroupsBaseline(this, core::mem::transmute_copy(&pgroups)).into()
        }
        unsafe extern "system" fn Id<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::Id(this) {
                Ok(ok__) => {
                    ppid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTombstone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistombstone: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::IsTombstone(this) {
                Ok(ok__) => {
                    pistombstone.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTombstone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, istombstone: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::SetIsTombstone(this, core::mem::transmute_copy(&istombstone)).into()
        }
        unsafe extern "system" fn LinkedAggregateId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplinkedaggregateid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::LinkedAggregateId(this) {
                Ok(ok__) => {
                    pplinkedaggregateid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLinkedAggregateId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plinkedaggregateid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::SetLinkedAggregateId(this, core::mem::transmute(&plinkedaggregateid)).into()
        }
        unsafe extern "system" fn ObjectId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPerson_Impl::ObjectId(this) {
                Ok(ok__) => {
                    ppobjectid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPerson_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactAggregationServerPerson_Impl::SetObjectId(this, core::mem::transmute(&pobjectid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            AggregateId: AggregateId::<Identity, OFFSET>,
            SetAggregateId: SetAggregateId::<Identity, OFFSET>,
            AntiLink: AntiLink::<Identity, OFFSET>,
            SetAntiLink: SetAntiLink::<Identity, OFFSET>,
            AntiLinkBaseline: AntiLinkBaseline::<Identity, OFFSET>,
            SetAntiLinkBaseline: SetAntiLinkBaseline::<Identity, OFFSET>,
            FavoriteOrder: FavoriteOrder::<Identity, OFFSET>,
            SetFavoriteOrder: SetFavoriteOrder::<Identity, OFFSET>,
            FavoriteOrderBaseline: FavoriteOrderBaseline::<Identity, OFFSET>,
            SetFavoriteOrderBaseline: SetFavoriteOrderBaseline::<Identity, OFFSET>,
            Groups: Groups::<Identity, OFFSET>,
            SetGroups: SetGroups::<Identity, OFFSET>,
            GroupsBaseline: GroupsBaseline::<Identity, OFFSET>,
            SetGroupsBaseline: SetGroupsBaseline::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            IsTombstone: IsTombstone::<Identity, OFFSET>,
            SetIsTombstone: SetIsTombstone::<Identity, OFFSET>,
            LinkedAggregateId: LinkedAggregateId::<Identity, OFFSET>,
            SetLinkedAggregateId: SetLinkedAggregateId::<Identity, OFFSET>,
            ObjectId: ObjectId::<Identity, OFFSET>,
            SetObjectId: SetObjectId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationServerPerson as windows_core::Interface>::IID
    }
}
pub trait IContactAggregationServerPersonCollection_Impl: Sized {
    fn FindFirst(&self) -> windows_core::Result<IContactAggregationServerPerson>;
    fn FindFirstByServerId(&self, pserverid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationServerPerson>;
    fn FindFirstByAggregateId(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationServerPerson>;
    fn FindFirstByLinkedAggregateId(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationServerPerson>;
    fn FindNext(&self) -> windows_core::Result<IContactAggregationServerPerson>;
    fn Count(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IContactAggregationServerPersonCollection {}
impl IContactAggregationServerPersonCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactAggregationServerPersonCollection_Vtbl
    where
        Identity: IContactAggregationServerPersonCollection_Impl,
    {
        unsafe extern "system" fn FindFirst<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPersonCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPersonCollection_Impl::FindFirst(this) {
                Ok(ok__) => {
                    ppserverperson.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByServerId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserverid: windows_core::PCWSTR, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPersonCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPersonCollection_Impl::FindFirstByServerId(this, core::mem::transmute(&pserverid)) {
                Ok(ok__) => {
                    ppserverperson.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByAggregateId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPersonCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPersonCollection_Impl::FindFirstByAggregateId(this, core::mem::transmute(&paggregateid)) {
                Ok(ok__) => {
                    ppserverperson.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstByLinkedAggregateId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPersonCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPersonCollection_Impl::FindFirstByLinkedAggregateId(this, core::mem::transmute(&paggregateid)) {
                Ok(ok__) => {
                    ppserverperson.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPersonCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPersonCollection_Impl::FindNext(this) {
                Ok(ok__) => {
                    ppserverperson.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactAggregationServerPersonCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactAggregationServerPersonCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindFirstByServerId: FindFirstByServerId::<Identity, OFFSET>,
            FindFirstByAggregateId: FindFirstByAggregateId::<Identity, OFFSET>,
            FindFirstByLinkedAggregateId: FindFirstByLinkedAggregateId::<Identity, OFFSET>,
            FindNext: FindNext::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationServerPersonCollection as windows_core::Interface>::IID
    }
}
pub trait IContactCollection_Impl: Sized {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<()>;
    fn GetCurrent(&self) -> windows_core::Result<IContact>;
}
impl windows_core::RuntimeName for IContactCollection {}
impl IContactCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactCollection_Vtbl
    where
        Identity: IContactCollection_Impl,
    {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactCollection_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactCollection_Impl::Next(this).into()
        }
        unsafe extern "system" fn GetCurrent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontact: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactCollection_Impl::GetCurrent(this) {
                Ok(ok__) => {
                    ppcontact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactCollection as windows_core::Interface>::IID
    }
}
pub trait IContactManager_Impl: Sized {
    fn Initialize(&self, pszappname: &windows_core::PCWSTR, pszappversion: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Load(&self, pszcontactid: &windows_core::PCWSTR) -> windows_core::Result<IContact>;
    fn MergeContactIDs(&self, psznewcontactid: &windows_core::PCWSTR, pszoldcontactid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetMeContact(&self) -> windows_core::Result<IContact>;
    fn SetMeContact(&self, pmecontact: Option<&IContact>) -> windows_core::Result<()>;
    fn GetContactCollection(&self) -> windows_core::Result<IContactCollection>;
}
impl windows_core::RuntimeName for IContactManager {}
impl IContactManager_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactManager_Vtbl
    where
        Identity: IContactManager_Impl,
    {
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszappname: windows_core::PCWSTR, pszappversion: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactManager_Impl::Initialize(this, core::mem::transmute(&pszappname), core::mem::transmute(&pszappversion)).into()
        }
        unsafe extern "system" fn Load<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcontactid: windows_core::PCWSTR, ppcontact: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactManager_Impl::Load(this, core::mem::transmute(&pszcontactid)) {
                Ok(ok__) => {
                    ppcontact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MergeContactIDs<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, psznewcontactid: windows_core::PCWSTR, pszoldcontactid: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactManager_Impl::MergeContactIDs(this, core::mem::transmute(&psznewcontactid), core::mem::transmute(&pszoldcontactid)).into()
        }
        unsafe extern "system" fn GetMeContact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmecontact: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactManager_Impl::GetMeContact(this) {
                Ok(ok__) => {
                    ppmecontact.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeContact<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmecontact: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactManager_Impl::SetMeContact(this, windows_core::from_raw_borrowed(&pmecontact)).into()
        }
        unsafe extern "system" fn GetContactCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontactcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactManager_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContactManager_Impl::GetContactCollection(this) {
                Ok(ok__) => {
                    ppcontactcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            MergeContactIDs: MergeContactIDs::<Identity, OFFSET>,
            GetMeContact: GetMeContact::<Identity, OFFSET>,
            SetMeContact: SetMeContact::<Identity, OFFSET>,
            GetContactCollection: GetContactCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IContactProperties_Impl: Sized {
    fn GetString(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, pszvalue: &windows_core::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> windows_core::Result<()>;
    fn GetDate(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetBinary(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, pszcontenttype: &windows_core::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut Option<super::Com::IStream>) -> windows_core::Result<()>;
    fn GetLabels(&self, pszarrayelementname: &windows_core::PCWSTR, dwflags: u32, pszlabels: &windows_core::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> windows_core::Result<()>;
    fn SetString(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, pszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetDate(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, ftdatetime: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SetBinary(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, pszcontenttype: &windows_core::PCWSTR, pstream: Option<&super::Com::IStream>) -> windows_core::Result<()>;
    fn SetLabels(&self, pszarrayelementname: &windows_core::PCWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CreateArrayNode(&self, pszarrayname: &windows_core::PCWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: &windows_core::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> windows_core::Result<()>;
    fn DeleteProperty(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn DeleteArrayNode(&self, pszarrayelementname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn DeleteLabels(&self, pszarrayelementname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetPropertyCollection(&self, pppropertycollection: *mut Option<IContactPropertyCollection>, dwflags: u32, pszmultivaluename: &windows_core::PCWSTR, dwlabelcount: u32, ppszlabels: *const windows_core::PCWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IContactProperties {}
#[cfg(feature = "Win32_System_Com")]
impl IContactProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactProperties_Vtbl
    where
        Identity: IContactProperties_Impl,
    {
        unsafe extern "system" fn GetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, pszvalue: windows_core::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::GetString(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszvalue), core::mem::transmute_copy(&cchvalue), core::mem::transmute_copy(&pdwcchpropertyvaluerequired)).into()
        }
        unsafe extern "system" fn GetDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::GetDate(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pftdatetime)).into()
        }
        unsafe extern "system" fn GetBinary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, pszcontenttype: windows_core::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::GetBinary(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszcontenttype), core::mem::transmute_copy(&cchcontenttype), core::mem::transmute_copy(&pdwcchcontenttyperequired), core::mem::transmute_copy(&ppstream)).into()
        }
        unsafe extern "system" fn GetLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayelementname: windows_core::PCWSTR, dwflags: u32, pszlabels: windows_core::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::GetLabels(this, core::mem::transmute(&pszarrayelementname), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszlabels), core::mem::transmute_copy(&cchlabels), core::mem::transmute_copy(&pdwcchlabelsrequired)).into()
        }
        unsafe extern "system" fn SetString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, pszvalue: windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::SetString(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszvalue)).into()
        }
        unsafe extern "system" fn SetDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::SetDate(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute(&ftdatetime)).into()
        }
        unsafe extern "system" fn SetBinary<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, pszcontenttype: windows_core::PCWSTR, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::SetBinary(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszcontenttype), windows_core::from_raw_borrowed(&pstream)).into()
        }
        unsafe extern "system" fn SetLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayelementname: windows_core::PCWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const windows_core::PCWSTR) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::SetLabels(this, core::mem::transmute(&pszarrayelementname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwlabelcount), core::mem::transmute_copy(&ppszlabels)).into()
        }
        unsafe extern "system" fn CreateArrayNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayname: windows_core::PCWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: windows_core::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::CreateArrayNode(this, core::mem::transmute(&pszarrayname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&fappend), core::mem::transmute(&psznewarrayelementname), core::mem::transmute_copy(&cchnewarrayelementname), core::mem::transmute_copy(&pdwcchnewarrayelementnamerequired)).into()
        }
        unsafe extern "system" fn DeleteProperty<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::DeleteProperty(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteArrayNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayelementname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::DeleteArrayNode(this, core::mem::transmute(&pszarrayelementname), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteLabels<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayelementname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::DeleteLabels(this, core::mem::transmute(&pszarrayelementname), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetPropertyCollection<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertycollection: *mut *mut core::ffi::c_void, dwflags: u32, pszmultivaluename: windows_core::PCWSTR, dwlabelcount: u32, ppszlabels: *const windows_core::PCWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IContactProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactProperties_Impl::GetPropertyCollection(this, core::mem::transmute_copy(&pppropertycollection), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszmultivaluename), core::mem::transmute_copy(&dwlabelcount), core::mem::transmute_copy(&ppszlabels), core::mem::transmute_copy(&fanylabelmatches)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetString: GetString::<Identity, OFFSET>,
            GetDate: GetDate::<Identity, OFFSET>,
            GetBinary: GetBinary::<Identity, OFFSET>,
            GetLabels: GetLabels::<Identity, OFFSET>,
            SetString: SetString::<Identity, OFFSET>,
            SetDate: SetDate::<Identity, OFFSET>,
            SetBinary: SetBinary::<Identity, OFFSET>,
            SetLabels: SetLabels::<Identity, OFFSET>,
            CreateArrayNode: CreateArrayNode::<Identity, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, OFFSET>,
            DeleteArrayNode: DeleteArrayNode::<Identity, OFFSET>,
            DeleteLabels: DeleteLabels::<Identity, OFFSET>,
            GetPropertyCollection: GetPropertyCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactProperties as windows_core::Interface>::IID
    }
}
pub trait IContactPropertyCollection_Impl: Sized {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<()>;
    fn GetPropertyName(&self, pszpropertyname: &windows_core::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> windows_core::Result<()>;
    fn GetPropertyType(&self, pdwtype: *mut u32) -> windows_core::Result<()>;
    fn GetPropertyVersion(&self, pdwversion: *mut u32) -> windows_core::Result<()>;
    fn GetPropertyModificationDate(&self, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetPropertyArrayElementID(&self, pszarrayelementid: &windows_core::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IContactPropertyCollection {}
impl IContactPropertyCollection_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContactPropertyCollection_Vtbl
    where
        Identity: IContactPropertyCollection_Impl,
    {
        unsafe extern "system" fn Reset<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactPropertyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactPropertyCollection_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Next<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IContactPropertyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactPropertyCollection_Impl::Next(this).into()
        }
        unsafe extern "system" fn GetPropertyName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactPropertyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactPropertyCollection_Impl::GetPropertyName(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&cchpropertyname), core::mem::transmute_copy(&pdwcchpropertynamerequired)).into()
        }
        unsafe extern "system" fn GetPropertyType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtype: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactPropertyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactPropertyCollection_Impl::GetPropertyType(this, core::mem::transmute_copy(&pdwtype)).into()
        }
        unsafe extern "system" fn GetPropertyVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactPropertyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactPropertyCollection_Impl::GetPropertyVersion(this, core::mem::transmute_copy(&pdwversion)).into()
        }
        unsafe extern "system" fn GetPropertyModificationDate<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT
        where
            Identity: IContactPropertyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactPropertyCollection_Impl::GetPropertyModificationDate(this, core::mem::transmute_copy(&pftmodificationdate)).into()
        }
        unsafe extern "system" fn GetPropertyArrayElementID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayelementid: windows_core::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> windows_core::HRESULT
        where
            Identity: IContactPropertyCollection_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContactPropertyCollection_Impl::GetPropertyArrayElementID(this, core::mem::transmute(&pszarrayelementid), core::mem::transmute_copy(&ccharrayelementid), core::mem::transmute_copy(&pdwccharrayelementidrequired)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            GetPropertyName: GetPropertyName::<Identity, OFFSET>,
            GetPropertyType: GetPropertyType::<Identity, OFFSET>,
            GetPropertyVersion: GetPropertyVersion::<Identity, OFFSET>,
            GetPropertyModificationDate: GetPropertyModificationDate::<Identity, OFFSET>,
            GetPropertyArrayElementID: GetPropertyArrayElementID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactPropertyCollection as windows_core::Interface>::IID
    }
}
