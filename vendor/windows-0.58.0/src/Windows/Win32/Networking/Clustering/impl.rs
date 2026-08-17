pub trait IGetClusterDataInfo_Impl: Sized {
    fn GetClusterName(&self, lpszname: windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()>;
    fn GetClusterHandle(&self) -> HCLUSTER;
    fn GetObjectCount(&self) -> i32;
}
impl windows_core::RuntimeName for IGetClusterDataInfo {}
impl IGetClusterDataInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetClusterDataInfo_Vtbl
    where
        Identity: IGetClusterDataInfo_Impl,
    {
        unsafe extern "system" fn GetClusterName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszname: core::mem::MaybeUninit<windows_core::BSTR>, pcchname: *mut i32) -> windows_core::HRESULT
        where
            Identity: IGetClusterDataInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterDataInfo_Impl::GetClusterName(this, core::mem::transmute_copy(&lpszname), core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetClusterHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> HCLUSTER
        where
            Identity: IGetClusterDataInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterDataInfo_Impl::GetClusterHandle(this)
        }
        unsafe extern "system" fn GetObjectCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32
        where
            Identity: IGetClusterDataInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterDataInfo_Impl::GetObjectCount(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClusterName: GetClusterName::<Identity, OFFSET>,
            GetClusterHandle: GetClusterHandle::<Identity, OFFSET>,
            GetObjectCount: GetObjectCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterDataInfo as windows_core::Interface>::IID
    }
}
pub trait IGetClusterGroupInfo_Impl: Sized {
    fn GetGroupHandle(&self, lobjindex: i32) -> HGROUP;
}
impl windows_core::RuntimeName for IGetClusterGroupInfo {}
impl IGetClusterGroupInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetClusterGroupInfo_Vtbl
    where
        Identity: IGetClusterGroupInfo_Impl,
    {
        unsafe extern "system" fn GetGroupHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> HGROUP
        where
            Identity: IGetClusterGroupInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterGroupInfo_Impl::GetGroupHandle(this, core::mem::transmute_copy(&lobjindex))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetGroupHandle: GetGroupHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterGroupInfo as windows_core::Interface>::IID
    }
}
pub trait IGetClusterNetInterfaceInfo_Impl: Sized {
    fn GetNetInterfaceHandle(&self, lobjindex: i32) -> HNETINTERFACE;
}
impl windows_core::RuntimeName for IGetClusterNetInterfaceInfo {}
impl IGetClusterNetInterfaceInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetClusterNetInterfaceInfo_Vtbl
    where
        Identity: IGetClusterNetInterfaceInfo_Impl,
    {
        unsafe extern "system" fn GetNetInterfaceHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> HNETINTERFACE
        where
            Identity: IGetClusterNetInterfaceInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterNetInterfaceInfo_Impl::GetNetInterfaceHandle(this, core::mem::transmute_copy(&lobjindex))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNetInterfaceHandle: GetNetInterfaceHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterNetInterfaceInfo as windows_core::Interface>::IID
    }
}
pub trait IGetClusterNetworkInfo_Impl: Sized {
    fn GetNetworkHandle(&self, lobjindex: i32) -> HNETWORK;
}
impl windows_core::RuntimeName for IGetClusterNetworkInfo {}
impl IGetClusterNetworkInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetClusterNetworkInfo_Vtbl
    where
        Identity: IGetClusterNetworkInfo_Impl,
    {
        unsafe extern "system" fn GetNetworkHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> HNETWORK
        where
            Identity: IGetClusterNetworkInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterNetworkInfo_Impl::GetNetworkHandle(this, core::mem::transmute_copy(&lobjindex))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNetworkHandle: GetNetworkHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterNetworkInfo as windows_core::Interface>::IID
    }
}
pub trait IGetClusterNodeInfo_Impl: Sized {
    fn GetNodeHandle(&self, lobjindex: i32) -> HNODE;
}
impl windows_core::RuntimeName for IGetClusterNodeInfo {}
impl IGetClusterNodeInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetClusterNodeInfo_Vtbl
    where
        Identity: IGetClusterNodeInfo_Impl,
    {
        unsafe extern "system" fn GetNodeHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> HNODE
        where
            Identity: IGetClusterNodeInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterNodeInfo_Impl::GetNodeHandle(this, core::mem::transmute_copy(&lobjindex))
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNodeHandle: GetNodeHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterNodeInfo as windows_core::Interface>::IID
    }
}
pub trait IGetClusterObjectInfo_Impl: Sized {
    fn GetObjectName(&self, lobjindex: i32, lpszname: windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()>;
    fn GetObjectType(&self, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE;
}
impl windows_core::RuntimeName for IGetClusterObjectInfo {}
impl IGetClusterObjectInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetClusterObjectInfo_Vtbl
    where
        Identity: IGetClusterObjectInfo_Impl,
    {
        unsafe extern "system" fn GetObjectName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32, lpszname: core::mem::MaybeUninit<windows_core::BSTR>, pcchname: *mut i32) -> windows_core::HRESULT
        where
            Identity: IGetClusterObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterObjectInfo_Impl::GetObjectName(this, core::mem::transmute_copy(&lobjindex), core::mem::transmute_copy(&lpszname), core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetObjectType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> CLUADMEX_OBJECT_TYPE
        where
            Identity: IGetClusterObjectInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterObjectInfo_Impl::GetObjectType(this, core::mem::transmute_copy(&lobjindex))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectName: GetObjectName::<Identity, OFFSET>,
            GetObjectType: GetObjectType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterObjectInfo as windows_core::Interface>::IID
    }
}
pub trait IGetClusterResourceInfo_Impl: Sized {
    fn GetResourceHandle(&self, lobjindex: i32) -> HRESOURCE;
    fn GetResourceTypeName(&self, lobjindex: i32, lpszrestypename: windows_core::BSTR, pcchrestypename: *mut i32) -> windows_core::Result<()>;
    fn GetResourceNetworkName(&self, lobjindex: i32, lpsznetname: windows_core::BSTR, pcchnetname: *mut u32) -> super::super::Foundation::BOOL;
}
impl windows_core::RuntimeName for IGetClusterResourceInfo {}
impl IGetClusterResourceInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetClusterResourceInfo_Vtbl
    where
        Identity: IGetClusterResourceInfo_Impl,
    {
        unsafe extern "system" fn GetResourceHandle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32) -> HRESOURCE
        where
            Identity: IGetClusterResourceInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterResourceInfo_Impl::GetResourceHandle(this, core::mem::transmute_copy(&lobjindex))
        }
        unsafe extern "system" fn GetResourceTypeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32, lpszrestypename: core::mem::MaybeUninit<windows_core::BSTR>, pcchrestypename: *mut i32) -> windows_core::HRESULT
        where
            Identity: IGetClusterResourceInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterResourceInfo_Impl::GetResourceTypeName(this, core::mem::transmute_copy(&lobjindex), core::mem::transmute_copy(&lpszrestypename), core::mem::transmute_copy(&pcchrestypename)).into()
        }
        unsafe extern "system" fn GetResourceNetworkName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lobjindex: i32, lpsznetname: core::mem::MaybeUninit<windows_core::BSTR>, pcchnetname: *mut u32) -> super::super::Foundation::BOOL
        where
            Identity: IGetClusterResourceInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterResourceInfo_Impl::GetResourceNetworkName(this, core::mem::transmute_copy(&lobjindex), core::mem::transmute_copy(&lpsznetname), core::mem::transmute_copy(&pcchnetname))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetResourceHandle: GetResourceHandle::<Identity, OFFSET>,
            GetResourceTypeName: GetResourceTypeName::<Identity, OFFSET>,
            GetResourceNetworkName: GetResourceNetworkName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterResourceInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IGetClusterUIInfo_Impl: Sized {
    fn GetClusterName(&self, lpszname: windows_core::BSTR, pcchname: *mut i32) -> windows_core::Result<()>;
    fn GetLocale(&self) -> u32;
    fn GetFont(&self) -> super::super::Graphics::Gdi::HFONT;
    fn GetIcon(&self) -> super::super::UI::WindowsAndMessaging::HICON;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IGetClusterUIInfo {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IGetClusterUIInfo_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IGetClusterUIInfo_Vtbl
    where
        Identity: IGetClusterUIInfo_Impl,
    {
        unsafe extern "system" fn GetClusterName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszname: core::mem::MaybeUninit<windows_core::BSTR>, pcchname: *mut i32) -> windows_core::HRESULT
        where
            Identity: IGetClusterUIInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterUIInfo_Impl::GetClusterName(this, core::mem::transmute_copy(&lpszname), core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetLocale<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32
        where
            Identity: IGetClusterUIInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterUIInfo_Impl::GetLocale(this)
        }
        unsafe extern "system" fn GetFont<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Graphics::Gdi::HFONT
        where
            Identity: IGetClusterUIInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterUIInfo_Impl::GetFont(this)
        }
        unsafe extern "system" fn GetIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::UI::WindowsAndMessaging::HICON
        where
            Identity: IGetClusterUIInfo_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IGetClusterUIInfo_Impl::GetIcon(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClusterName: GetClusterName::<Identity, OFFSET>,
            GetLocale: GetLocale::<Identity, OFFSET>,
            GetFont: GetFont::<Identity, OFFSET>,
            GetIcon: GetIcon::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetClusterUIInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DomainNames(&self) -> windows_core::Result<ISDomainNames>;
    fn get_ClusterNames(&self, bstrdomainname: &windows_core::BSTR) -> windows_core::Result<ISClusterNames>;
    fn OpenCluster(&self, bstrclustername: &windows_core::BSTR) -> windows_core::Result<ISCluster>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusApplication {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusApplication_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusApplication_Vtbl
    where
        Identity: ISClusApplication_Impl,
    {
        unsafe extern "system" fn DomainNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdomains: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusApplication_Impl::DomainNames(this) {
                Ok(ok__) => {
                    ppdomains.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ClusterNames<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdomainname: core::mem::MaybeUninit<windows_core::BSTR>, ppclusters: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusApplication_Impl::get_ClusterNames(this, core::mem::transmute(&bstrdomainname)) {
                Ok(ok__) => {
                    ppclusters.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCluster<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclustername: core::mem::MaybeUninit<windows_core::BSTR>, pcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusApplication_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusApplication_Impl::OpenCluster(this, core::mem::transmute(&bstrclustername)) {
                Ok(ok__) => {
                    pcluster.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DomainNames: DomainNames::<Identity, OFFSET>,
            get_ClusterNames: get_ClusterNames::<Identity, OFFSET>,
            OpenCluster: OpenCluster::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusApplication as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusCryptoKeys_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn AddItem(&self, bstrcryptokey: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusCryptoKeys {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusCryptoKeys_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusCryptoKeys_Vtbl
    where
        Identity: ISClusCryptoKeys_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusCryptoKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusCryptoKeys_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusCryptoKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusCryptoKeys_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusCryptoKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusCryptoKeys_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, pbstrcyrptokey: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusCryptoKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusCryptoKeys_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    pbstrcyrptokey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcryptokey: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusCryptoKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusCryptoKeys_Impl::AddItem(this, core::mem::transmute(&bstrcryptokey)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusCryptoKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusCryptoKeys_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusCryptoKeys as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusDisk_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Signature(&self) -> windows_core::Result<i32>;
    fn ScsiAddress(&self) -> windows_core::Result<ISClusScsiAddress>;
    fn DiskNumber(&self) -> windows_core::Result<i32>;
    fn Partitions(&self) -> windows_core::Result<ISClusPartitions>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusDisk {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusDisk_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusDisk_Vtbl
    where
        Identity: ISClusDisk_Impl,
    {
        unsafe extern "system" fn Signature<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsignature: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusDisk_Impl::Signature(this) {
                Ok(ok__) => {
                    plsignature.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScsiAddress<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppscsiaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusDisk_Impl::ScsiAddress(this) {
                Ok(ok__) => {
                    ppscsiaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiskNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldisknumber: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusDisk_Impl::DiskNumber(this) {
                Ok(ok__) => {
                    pldisknumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Partitions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppartitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusDisk_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusDisk_Impl::Partitions(this) {
                Ok(ok__) => {
                    pppartitions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Signature: Signature::<Identity, OFFSET>,
            ScsiAddress: ScsiAddress::<Identity, OFFSET>,
            DiskNumber: DiskNumber::<Identity, OFFSET>,
            Partitions: Partitions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusDisk as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusDisks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusDisk>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusDisks {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusDisks_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusDisks_Vtbl
    where
        Identity: ISClusDisks_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusDisks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusDisks_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusDisks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusDisks_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppdisk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusDisks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusDisks_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppdisk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusDisks as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetInterface_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn State(&self) -> windows_core::Result<CLUSTER_NETINTERFACE_STATE>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusNetInterface {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetInterface_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusNetInterface_Vtbl
    where
        Identity: ISClusNetInterface_Impl,
    {
        unsafe extern "system" fn CommonProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterface_Impl::CommonProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterface_Impl::PrivateProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterface_Impl::CommonROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterface_Impl::PrivateROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterface_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterface_Impl::Handle(this) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: *mut CLUSTER_NETINTERFACE_STATE) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterface_Impl::State(this) {
                Ok(ok__) => {
                    dwstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterface_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterface_Impl::Cluster(this) {
                Ok(ok__) => {
                    ppcluster.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNetInterface as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusNetInterface>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusNetInterfaces {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetInterfaces_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusNetInterfaces_Vtbl
    where
        Identity: ISClusNetInterfaces_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterfaces_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterfaces_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusNetInterfaces_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusnetinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetInterfaces_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusnetinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNetInterfaces as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetwork_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrnetworkname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NetworkID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<CLUSTER_NETWORK_STATE>;
    fn NetInterfaces(&self) -> windows_core::Result<ISClusNetworkNetInterfaces>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusNetwork {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetwork_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusNetwork_Vtbl
    where
        Identity: ISClusNetwork_Impl,
    {
        unsafe extern "system" fn CommonProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetwork_Impl::CommonProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetwork_Impl::PrivateProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetwork_Impl::CommonROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetwork_Impl::PrivateROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetwork_Impl::Handle(this) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetwork_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrnetworkname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusNetwork_Impl::SetName(this, core::mem::transmute(&bstrnetworkname)).into()
        }
        unsafe extern "system" fn NetworkID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnetworkid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetwork_Impl::NetworkID(this) {
                Ok(ok__) => {
                    pbstrnetworkid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: *mut CLUSTER_NETWORK_STATE) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetwork_Impl::State(this) {
                Ok(ok__) => {
                    dwstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusnetinterfaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetwork_Impl::NetInterfaces(this) {
                Ok(ok__) => {
                    ppclusnetinterfaces.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetwork_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetwork_Impl::Cluster(this) {
                Ok(ok__) => {
                    ppcluster.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            NetworkID: NetworkID::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNetwork as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetworkNetInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusNetInterface>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusNetworkNetInterfaces {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworkNetInterfaces_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusNetworkNetInterfaces_Vtbl
    where
        Identity: ISClusNetworkNetInterfaces_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusNetworkNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetworkNetInterfaces_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetworkNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetworkNetInterfaces_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetworkNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusNetworkNetInterfaces_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusnetinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetworkNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetworkNetInterfaces_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusnetinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNetworkNetInterfaces as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNetworks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusNetwork>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusNetworks {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNetworks_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusNetworks_Vtbl
    where
        Identity: ISClusNetworks_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusNetworks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetworks_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetworks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetworks_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetworks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusNetworks_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusnetwork: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNetworks_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNetworks_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusnetwork.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNetworks as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNode_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn NodeID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn State(&self) -> windows_core::Result<CLUSTER_NODE_STATE>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Evict(&self) -> windows_core::Result<()>;
    fn ResourceGroups(&self) -> windows_core::Result<ISClusResGroups>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
    fn NetInterfaces(&self) -> windows_core::Result<ISClusNodeNetInterfaces>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusNode {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusNode_Vtbl
    where
        Identity: ISClusNode_Impl,
    {
        unsafe extern "system" fn CommonProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::CommonProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::PrivateProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::CommonROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::PrivateROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::Handle(this) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NodeID<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrnodeid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::NodeID(this) {
                Ok(ok__) => {
                    pbstrnodeid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: *mut CLUSTER_NODE_STATE) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::State(this) {
                Ok(ok__) => {
                    dwstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusNode_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusNode_Impl::Resume(this).into()
        }
        unsafe extern "system" fn Evict<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusNode_Impl::Evict(this).into()
        }
        unsafe extern "system" fn ResourceGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresourcegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::ResourceGroups(this) {
                Ok(ok__) => {
                    ppresourcegroups.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::Cluster(this) {
                Ok(ok__) => {
                    ppcluster.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusnetinterfaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNode_Impl::NetInterfaces(this) {
                Ok(ok__) => {
                    ppclusnetinterfaces.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            NodeID: NodeID::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Evict: Evict::<Identity, OFFSET>,
            ResourceGroups: ResourceGroups::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNode as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNodeNetInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusNetInterface>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusNodeNetInterfaces {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodeNetInterfaces_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusNodeNetInterfaces_Vtbl
    where
        Identity: ISClusNodeNetInterfaces_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusNodeNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNodeNetInterfaces_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNodeNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNodeNetInterfaces_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNodeNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusNodeNetInterfaces_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusnetinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNodeNetInterfaces_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNodeNetInterfaces_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusnetinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNodeNetInterfaces as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusNode>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusNodes {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusNodes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusNodes_Vtbl
    where
        Identity: ISClusNodes_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNodes_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNodes_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusNodes_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusNodes_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppnode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusNodes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPartition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Flags(&self) -> windows_core::Result<i32>;
    fn DeviceName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeLabel(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SerialNumber(&self) -> windows_core::Result<i32>;
    fn MaximumComponentLength(&self) -> windows_core::Result<i32>;
    fn FileSystemFlags(&self) -> windows_core::Result<i32>;
    fn FileSystem(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusPartition {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartition_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusPartition_Vtbl
    where
        Identity: ISClusPartition_Impl,
    {
        unsafe extern "system" fn Flags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plflags: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPartition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartition_Impl::Flags(this) {
                Ok(ok__) => {
                    plflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdevicename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusPartition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartition_Impl::DeviceName(this) {
                Ok(ok__) => {
                    pbstrdevicename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeLabel<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrvolumelabel: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusPartition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartition_Impl::VolumeLabel(this) {
                Ok(ok__) => {
                    pbstrvolumelabel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plserialnumber: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPartition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartition_Impl::SerialNumber(this) {
                Ok(ok__) => {
                    plserialnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumComponentLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmaximumcomponentlength: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPartition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartition_Impl::MaximumComponentLength(this) {
                Ok(ok__) => {
                    plmaximumcomponentlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plfilesystemflags: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPartition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartition_Impl::FileSystemFlags(this) {
                Ok(ok__) => {
                    plfilesystemflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilesystem: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusPartition_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartition_Impl::FileSystem(this) {
                Ok(ok__) => {
                    pbstrfilesystem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Flags: Flags::<Identity, OFFSET>,
            DeviceName: DeviceName::<Identity, OFFSET>,
            VolumeLabel: VolumeLabel::<Identity, OFFSET>,
            SerialNumber: SerialNumber::<Identity, OFFSET>,
            MaximumComponentLength: MaximumComponentLength::<Identity, OFFSET>,
            FileSystemFlags: FileSystemFlags::<Identity, OFFSET>,
            FileSystem: FileSystem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPartition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPartitionEx_Impl: Sized + ISClusPartition_Impl {
    fn TotalSize(&self) -> windows_core::Result<i32>;
    fn FreeSpace(&self) -> windows_core::Result<i32>;
    fn DeviceNumber(&self) -> windows_core::Result<i32>;
    fn PartitionNumber(&self) -> windows_core::Result<i32>;
    fn VolumeGuid(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusPartitionEx {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitionEx_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusPartitionEx_Vtbl
    where
        Identity: ISClusPartitionEx_Impl,
    {
        unsafe extern "system" fn TotalSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pltotalsize: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPartitionEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartitionEx_Impl::TotalSize(this) {
                Ok(ok__) => {
                    pltotalsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSpace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plfreespace: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPartitionEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartitionEx_Impl::FreeSpace(this) {
                Ok(ok__) => {
                    plfreespace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pldevicenumber: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPartitionEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartitionEx_Impl::DeviceNumber(this) {
                Ok(ok__) => {
                    pldevicenumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartitionNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plpartitionnumber: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPartitionEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartitionEx_Impl::PartitionNumber(this) {
                Ok(ok__) => {
                    plpartitionnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrvolumeguid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusPartitionEx_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartitionEx_Impl::VolumeGuid(this) {
                Ok(ok__) => {
                    pbstrvolumeguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISClusPartition_Vtbl::new::<Identity, OFFSET>(),
            TotalSize: TotalSize::<Identity, OFFSET>,
            FreeSpace: FreeSpace::<Identity, OFFSET>,
            DeviceNumber: DeviceNumber::<Identity, OFFSET>,
            PartitionNumber: PartitionNumber::<Identity, OFFSET>,
            VolumeGuid: VolumeGuid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPartitionEx as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISClusPartition as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPartitions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusPartition>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusPartitions {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPartitions_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusPartitions_Vtbl
    where
        Identity: ISClusPartitions_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPartitions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartitions_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusPartitions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartitions_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, pppartition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusPartitions_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPartitions_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    pppartition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPartitions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusProperty>;
    fn CreateItem(&self, bstrname: &windows_core::BSTR, varvalue: &windows_core::VARIANT) -> windows_core::Result<ISClusProperty>;
    fn UseDefaultValue(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn SaveChanges(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn ReadOnly(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Private(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Common(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Modified(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusProperties {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusProperties_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusProperties_Vtbl
    where
        Identity: ISClusProperties_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperties_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperties_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusProperties_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperties_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, varvalue: core::mem::MaybeUninit<windows_core::VARIANT>, pproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperties_Impl::CreateItem(this, core::mem::transmute(&bstrname), core::mem::transmute(&varvalue)) {
                Ok(ok__) => {
                    pproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDefaultValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusProperties_Impl::UseDefaultValue(this, core::mem::transmute(&varindex)).into()
        }
        unsafe extern "system" fn SaveChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarstatuscode: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperties_Impl::SaveChanges(this) {
                Ok(ok__) => {
                    pvarstatuscode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarreadonly: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperties_Impl::ReadOnly(this) {
                Ok(ok__) => {
                    pvarreadonly.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Private<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprivate: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperties_Impl::Private(this) {
                Ok(ok__) => {
                    pvarprivate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Common<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcommon: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperties_Impl::Common(this) {
                Ok(ok__) => {
                    pvarcommon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodified: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperties_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperties_Impl::Modified(this) {
                Ok(ok__) => {
                    pvarmodified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            UseDefaultValue: UseDefaultValue::<Identity, OFFSET>,
            SaveChanges: SaveChanges::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            Private: Private::<Identity, OFFSET>,
            Common: Common::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusProperties as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn ValueCount(&self) -> windows_core::Result<i32>;
    fn Values(&self) -> windows_core::Result<ISClusPropertyValues>;
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, varvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<CLUSTER_PROPERTY_TYPE>;
    fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::Result<()>;
    fn Format(&self) -> windows_core::Result<CLUSTER_PROPERTY_FORMAT>;
    fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::Result<()>;
    fn ReadOnly(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Private(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Common(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Modified(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn UseDefaultValue(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusProperty_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusProperty_Vtbl
    where
        Identity: ISClusProperty_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plength: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::Length(this) {
                Ok(ok__) => {
                    plength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::ValueCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Values<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusterpropertyvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::Values(this) {
                Ok(ok__) => {
                    ppclusterpropertyvalues.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::Value(this) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusProperty_Impl::SetValue(this, core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::Type(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusProperty_Impl::SetType(this, core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Format<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::Format(this) {
                Ok(ok__) => {
                    pformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusProperty_Impl::SetFormat(this, core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn ReadOnly<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarreadonly: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::ReadOnly(this) {
                Ok(ok__) => {
                    pvarreadonly.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Private<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarprivate: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::Private(this) {
                Ok(ok__) => {
                    pvarprivate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Common<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarcommon: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::Common(this) {
                Ok(ok__) => {
                    pvarcommon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodified: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusProperty_Impl::Modified(this) {
                Ok(ok__) => {
                    pvarmodified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseDefaultValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusProperty_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusProperty_Impl::UseDefaultValue(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            ValueCount: ValueCount::<Identity, OFFSET>,
            Values: Values::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            Format: Format::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            ReadOnly: ReadOnly::<Identity, OFFSET>,
            Private: Private::<Identity, OFFSET>,
            Common: Common::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
            UseDefaultValue: UseDefaultValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusProperty as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyValue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Value(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SetValue(&self, varvalue: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<CLUSTER_PROPERTY_TYPE>;
    fn SetType(&self, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::Result<()>;
    fn Format(&self) -> windows_core::Result<CLUSTER_PROPERTY_FORMAT>;
    fn SetFormat(&self, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::Result<()>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn DataCount(&self) -> windows_core::Result<i32>;
    fn Data(&self) -> windows_core::Result<ISClusPropertyValueData>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusPropertyValue {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValue_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusPropertyValue_Vtbl
    where
        Identity: ISClusPropertyValue_Impl,
    {
        unsafe extern "system" fn Value<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValue_Impl::Value(this) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusPropertyValue_Impl::SetValue(this, core::mem::transmute(&varvalue)).into()
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValue_Impl::Type(this) {
                Ok(ok__) => {
                    ptype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: CLUSTER_PROPERTY_TYPE) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusPropertyValue_Impl::SetType(this, core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn Format<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *mut CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValue_Impl::Format(this) {
                Ok(ok__) => {
                    pformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: CLUSTER_PROPERTY_FORMAT) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusPropertyValue_Impl::SetFormat(this, core::mem::transmute_copy(&format)).into()
        }
        unsafe extern "system" fn Length<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plength: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValue_Impl::Length(this) {
                Ok(ok__) => {
                    plength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValue_Impl::DataCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusterpropertyvaluedata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValue_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValue_Impl::Data(this) {
                Ok(ok__) => {
                    ppclusterpropertyvaluedata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            Format: Format::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            DataCount: DataCount::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPropertyValue as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyValueData_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn CreateItem(&self, varvalue: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn RemoveItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusPropertyValueData {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValueData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusPropertyValueData_Vtbl
    where
        Identity: ISClusPropertyValueData_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValueData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValueData_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValueData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValueData_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, pvarvalue: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValueData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValueData_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varvalue: core::mem::MaybeUninit<windows_core::VARIANT>, pvardata: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValueData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValueData_Impl::CreateItem(this, core::mem::transmute(&varvalue)) {
                Ok(ok__) => {
                    pvardata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValueData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusPropertyValueData_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPropertyValueData as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusPropertyValues_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusPropertyValue>;
    fn CreateItem(&self, bstrname: &windows_core::BSTR, varvalue: &windows_core::VARIANT) -> windows_core::Result<ISClusPropertyValue>;
    fn RemoveItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusPropertyValues {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusPropertyValues_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusPropertyValues_Vtbl
    where
        Identity: ISClusPropertyValues_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValues_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValues_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, pppropertyvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValues_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    pppropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, varvalue: core::mem::MaybeUninit<windows_core::VARIANT>, pppropertyvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusPropertyValues_Impl::CreateItem(this, core::mem::transmute(&bstrname), core::mem::transmute(&varvalue)) {
                Ok(ok__) => {
                    pppropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusPropertyValues_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusPropertyValues_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusPropertyValues as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusRefObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Handle(&self) -> windows_core::Result<usize>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusRefObject {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusRefObject_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusRefObject_Vtbl
    where
        Identity: ISClusRefObject_Impl,
    {
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT
        where
            Identity: ISClusRefObject_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusRefObject_Impl::Handle(this) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Handle: Handle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusRefObject as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusRegistryKeys_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn AddItem(&self, bstrregistrykey: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusRegistryKeys {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusRegistryKeys_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusRegistryKeys_Vtbl
    where
        Identity: ISClusRegistryKeys_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusRegistryKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusRegistryKeys_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusRegistryKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusRegistryKeys_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusRegistryKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusRegistryKeys_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, pbstrregistrykey: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusRegistryKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusRegistryKeys_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    pbstrregistrykey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrregistrykey: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusRegistryKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusRegistryKeys_Impl::AddItem(this, core::mem::transmute(&bstrregistrykey)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusRegistryKeys_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusRegistryKeys_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusRegistryKeys as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResDependencies_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AddItem(&self, presource: Option<&ISClusResource>) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResDependencies {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResDependencies_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResDependencies_Vtbl
    where
        Identity: ISClusResDependencies_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusResDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResDependencies_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResDependencies_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResDependencies_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResDependencies_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: core::mem::MaybeUninit<windows_core::BSTR>, bstrresourcetype: core::mem::MaybeUninit<windows_core::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResDependencies_Impl::CreateItem(this, core::mem::transmute(&bstrresourcename), core::mem::transmute(&bstrresourcetype), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppclusterresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResDependencies_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
        }
        unsafe extern "system" fn AddItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResDependencies_Impl::AddItem(this, windows_core::from_raw_borrowed(&presource)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResDependencies_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResDependencies_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResDependencies as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResDependents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn AddItem(&self, presource: Option<&ISClusResource>) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResDependents {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResDependents_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResDependents_Vtbl
    where
        Identity: ISClusResDependents_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusResDependents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResDependents_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResDependents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResDependents_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResDependents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResDependents_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResDependents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResDependents_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: core::mem::MaybeUninit<windows_core::BSTR>, bstrresourcetype: core::mem::MaybeUninit<windows_core::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResDependents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResDependents_Impl::CreateItem(this, core::mem::transmute(&bstrresourcename), core::mem::transmute(&bstrresourcetype), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppclusterresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResDependents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResDependents_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
        }
        unsafe extern "system" fn AddItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResDependents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResDependents_Impl::AddItem(this, windows_core::from_raw_borrowed(&presource)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResDependents_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResDependents_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResDependents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrgroupname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<CLUSTER_GROUP_STATE>;
    fn OwnerNode(&self) -> windows_core::Result<ISClusNode>;
    fn Resources(&self) -> windows_core::Result<ISClusResGroupResources>;
    fn PreferredOwnerNodes(&self) -> windows_core::Result<ISClusResGroupPreferredOwnerNodes>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Online(&self, vartimeout: &windows_core::VARIANT, varnode: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn Move(&self, vartimeout: &windows_core::VARIANT, varnode: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn Offline(&self, vartimeout: &windows_core::VARIANT) -> windows_core::Result<windows_core::VARIANT>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroup_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResGroup_Vtbl
    where
        Identity: ISClusResGroup_Impl,
    {
        unsafe extern "system" fn CommonProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::CommonProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::PrivateProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::CommonROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::PrivateROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::Handle(this) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroup_Impl::SetName(this, core::mem::transmute(&bstrgroupname)).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: *mut CLUSTER_GROUP_STATE) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::State(this) {
                Ok(ok__) => {
                    dwstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppownernode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::OwnerNode(this) {
                Ok(ok__) => {
                    ppownernode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclustergroupresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::Resources(this) {
                Ok(ok__) => {
                    ppclustergroupresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredOwnerNodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppownernodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::PreferredOwnerNodes(this) {
                Ok(ok__) => {
                    ppownernodes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroup_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Online<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vartimeout: core::mem::MaybeUninit<windows_core::VARIANT>, varnode: core::mem::MaybeUninit<windows_core::VARIANT>, pvarpending: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::Online(this, core::mem::transmute(&vartimeout), core::mem::transmute(&varnode)) {
                Ok(ok__) => {
                    pvarpending.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vartimeout: core::mem::MaybeUninit<windows_core::VARIANT>, varnode: core::mem::MaybeUninit<windows_core::VARIANT>, pvarpending: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::Move(this, core::mem::transmute(&vartimeout), core::mem::transmute(&varnode)) {
                Ok(ok__) => {
                    pvarpending.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Offline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, vartimeout: core::mem::MaybeUninit<windows_core::VARIANT>, pvarpending: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::Offline(this, core::mem::transmute(&vartimeout)) {
                Ok(ok__) => {
                    pvarpending.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroup_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroup_Impl::Cluster(this) {
                Ok(ok__) => {
                    ppcluster.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            OwnerNode: OwnerNode::<Identity, OFFSET>,
            Resources: Resources::<Identity, OFFSET>,
            PreferredOwnerNodes: PreferredOwnerNodes::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Online: Online::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            Offline: Offline::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResGroup as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroupPreferredOwnerNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusNode>;
    fn InsertItem(&self, pnode: Option<&ISClusNode>, nposition: i32) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Modified(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn SaveChanges(&self) -> windows_core::Result<()>;
    fn AddItem(&self, pnode: Option<&ISClusNode>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResGroupPreferredOwnerNodes {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupPreferredOwnerNodes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResGroupPreferredOwnerNodes_Vtbl
    where
        Identity: ISClusResGroupPreferredOwnerNodes_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupPreferredOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroupPreferredOwnerNodes_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupPreferredOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroupPreferredOwnerNodes_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupPreferredOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroupPreferredOwnerNodes_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupPreferredOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroupPreferredOwnerNodes_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppnode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, nposition: i32) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupPreferredOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroupPreferredOwnerNodes_Impl::InsertItem(this, windows_core::from_raw_borrowed(&pnode), core::mem::transmute_copy(&nposition)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupPreferredOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroupPreferredOwnerNodes_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
        }
        unsafe extern "system" fn Modified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodified: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupPreferredOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroupPreferredOwnerNodes_Impl::Modified(this) {
                Ok(ok__) => {
                    pvarmodified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveChanges<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupPreferredOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroupPreferredOwnerNodes_Impl::SaveChanges(this).into()
        }
        unsafe extern "system" fn AddItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupPreferredOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroupPreferredOwnerNodes_Impl::AddItem(this, windows_core::from_raw_borrowed(&pnode)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            InsertItem: InsertItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
            SaveChanges: SaveChanges::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResGroupPreferredOwnerNodes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroupResources_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResGroupResources {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroupResources_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResGroupResources_Vtbl
    where
        Identity: ISClusResGroupResources_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroupResources_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroupResources_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroupResources_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroupResources_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: core::mem::MaybeUninit<windows_core::BSTR>, bstrresourcetype: core::mem::MaybeUninit<windows_core::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroupResources_Impl::CreateItem(this, core::mem::transmute(&bstrresourcename), core::mem::transmute(&bstrresourcetype), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppclusterresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResGroupResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroupResources_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResGroupResources as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResGroups_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusResGroup>;
    fn CreateItem(&self, bstrresourcegroupname: &windows_core::BSTR) -> windows_core::Result<ISClusResGroup>;
    fn DeleteItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResGroups {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResGroups_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResGroups_Vtbl
    where
        Identity: ISClusResGroups_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusResGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroups_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroups_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroups_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusresgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroups_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusresgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcegroupname: core::mem::MaybeUninit<windows_core::BSTR>, ppresourcegroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResGroups_Impl::CreateItem(this, core::mem::transmute(&bstrresourcegroupname)) {
                Ok(ok__) => {
                    ppresourcegroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResGroups_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResGroups_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResGroups as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResPossibleOwnerNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusNode>;
    fn AddItem(&self, pnode: Option<&ISClusNode>) -> windows_core::Result<()>;
    fn RemoveItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
    fn Modified(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResPossibleOwnerNodes {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResPossibleOwnerNodes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResPossibleOwnerNodes_Vtbl
    where
        Identity: ISClusResPossibleOwnerNodes_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusResPossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResPossibleOwnerNodes_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResPossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResPossibleOwnerNodes_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResPossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResPossibleOwnerNodes_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResPossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResPossibleOwnerNodes_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppnode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResPossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResPossibleOwnerNodes_Impl::AddItem(this, windows_core::from_raw_borrowed(&pnode)).into()
        }
        unsafe extern "system" fn RemoveItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResPossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResPossibleOwnerNodes_Impl::RemoveItem(this, core::mem::transmute(&varindex)).into()
        }
        unsafe extern "system" fn Modified<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmodified: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResPossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResPossibleOwnerNodes_Impl::Modified(this) {
                Ok(ok__) => {
                    pvarmodified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            AddItem: AddItem::<Identity, OFFSET>,
            RemoveItem: RemoveItem::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResPossibleOwnerNodes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResType_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
    fn Resources(&self) -> windows_core::Result<ISClusResTypeResources>;
    fn PossibleOwnerNodes(&self) -> windows_core::Result<ISClusResTypePossibleOwnerNodes>;
    fn AvailableDisks(&self) -> windows_core::Result<ISClusDisks>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResType {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResType_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResType_Vtbl
    where
        Identity: ISClusResType_Impl,
    {
        unsafe extern "system" fn CommonProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResType_Impl::CommonProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResType_Impl::PrivateProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResType_Impl::CommonROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResType_Impl::PrivateROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusResType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResType_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResType_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Cluster<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResType_Impl::Cluster(this) {
                Ok(ok__) => {
                    ppcluster.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusterrestyperesources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResType_Impl::Resources(this) {
                Ok(ok__) => {
                    ppclusterrestyperesources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PossibleOwnerNodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppownernodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResType_Impl::PossibleOwnerNodes(this) {
                Ok(ok__) => {
                    ppownernodes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableDisks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppavailabledisks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResType_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResType_Impl::AvailableDisks(this) {
                Ok(ok__) => {
                    ppavailabledisks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
            Resources: Resources::<Identity, OFFSET>,
            PossibleOwnerNodes: PossibleOwnerNodes::<Identity, OFFSET>,
            AvailableDisks: AvailableDisks::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResType as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypePossibleOwnerNodes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusNode>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResTypePossibleOwnerNodes {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypePossibleOwnerNodes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResTypePossibleOwnerNodes_Vtbl
    where
        Identity: ISClusResTypePossibleOwnerNodes_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusResTypePossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypePossibleOwnerNodes_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypePossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypePossibleOwnerNodes_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypePossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResTypePossibleOwnerNodes_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypePossibleOwnerNodes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypePossibleOwnerNodes_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppnode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResTypePossibleOwnerNodes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypeResources_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrgroupname: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResTypeResources {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypeResources_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResTypeResources_Vtbl
    where
        Identity: ISClusResTypeResources_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusResTypeResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypeResources_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypeResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypeResources_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypeResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResTypeResources_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypeResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypeResources_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: core::mem::MaybeUninit<windows_core::BSTR>, bstrgroupname: core::mem::MaybeUninit<windows_core::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypeResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypeResources_Impl::CreateItem(this, core::mem::transmute(&bstrresourcename), core::mem::transmute(&bstrgroupname), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppclusterresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResTypeResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResTypeResources_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResTypeResources as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResTypes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusResType>;
    fn CreateItem(&self, bstrresourcetypename: &windows_core::BSTR, bstrdisplayname: &windows_core::BSTR, bstrresourcetypedll: &windows_core::BSTR, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32) -> windows_core::Result<ISClusResType>;
    fn DeleteItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResTypes {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResTypes_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResTypes_Vtbl
    where
        Identity: ISClusResTypes_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusResTypes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypes_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypes_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResTypes_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusrestype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypes_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusrestype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcetypename: core::mem::MaybeUninit<windows_core::BSTR>, bstrdisplayname: core::mem::MaybeUninit<windows_core::BSTR>, bstrresourcetypedll: core::mem::MaybeUninit<windows_core::BSTR>, dwlooksalivepollinterval: i32, dwisalivepollinterval: i32, ppresourcetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResTypes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResTypes_Impl::CreateItem(this, core::mem::transmute(&bstrresourcetypename), core::mem::transmute(&bstrdisplayname), core::mem::transmute(&bstrresourcetypedll), core::mem::transmute_copy(&dwlooksalivepollinterval), core::mem::transmute_copy(&dwisalivepollinterval)) {
                Ok(ok__) => {
                    ppresourcetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResTypes_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResTypes_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResTypes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResource_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrresourcename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<CLUSTER_RESOURCE_STATE>;
    fn CoreFlag(&self) -> windows_core::Result<CLUS_FLAGS>;
    fn BecomeQuorumResource(&self, bstrdevicepath: &windows_core::BSTR, lmaxlogsize: i32) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Fail(&self) -> windows_core::Result<()>;
    fn Online(&self, ntimeout: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn Offline(&self, ntimeout: i32) -> windows_core::Result<windows_core::VARIANT>;
    fn ChangeResourceGroup(&self, presourcegroup: Option<&ISClusResGroup>) -> windows_core::Result<()>;
    fn AddResourceNode(&self, pnode: Option<&ISClusNode>) -> windows_core::Result<()>;
    fn RemoveResourceNode(&self, pnode: Option<&ISClusNode>) -> windows_core::Result<()>;
    fn CanResourceBeDependent(&self, presource: Option<&ISClusResource>) -> windows_core::Result<windows_core::VARIANT>;
    fn PossibleOwnerNodes(&self) -> windows_core::Result<ISClusResPossibleOwnerNodes>;
    fn Dependencies(&self) -> windows_core::Result<ISClusResDependencies>;
    fn Dependents(&self) -> windows_core::Result<ISClusResDependents>;
    fn Group(&self) -> windows_core::Result<ISClusResGroup>;
    fn OwnerNode(&self) -> windows_core::Result<ISClusNode>;
    fn Cluster(&self) -> windows_core::Result<ISCluster>;
    fn ClassInfo(&self) -> windows_core::Result<CLUSTER_RESOURCE_CLASS>;
    fn Disk(&self) -> windows_core::Result<ISClusDisk>;
    fn RegistryKeys(&self) -> windows_core::Result<ISClusRegistryKeys>;
    fn CryptoKeys(&self) -> windows_core::Result<ISClusCryptoKeys>;
    fn TypeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<ISClusResType>;
    fn MaintenanceMode(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetMaintenanceMode(&self, bmaintenancemode: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResource {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResource_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResource_Vtbl
    where
        Identity: ISClusResource_Impl,
    {
        unsafe extern "system" fn CommonProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::CommonProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::PrivateProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::CommonROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::PrivateROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::Handle(this) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResource_Impl::SetName(this, core::mem::transmute(&bstrresourcename)).into()
        }
        unsafe extern "system" fn State<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstate: *mut CLUSTER_RESOURCE_STATE) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::State(this) {
                Ok(ok__) => {
                    dwstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoreFlag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcoreflag: *mut CLUS_FLAGS) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::CoreFlag(this) {
                Ok(ok__) => {
                    dwcoreflag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BecomeQuorumResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdevicepath: core::mem::MaybeUninit<windows_core::BSTR>, lmaxlogsize: i32) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResource_Impl::BecomeQuorumResource(this, core::mem::transmute(&bstrdevicepath), core::mem::transmute_copy(&lmaxlogsize)).into()
        }
        unsafe extern "system" fn Delete<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResource_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Fail<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResource_Impl::Fail(this).into()
        }
        unsafe extern "system" fn Online<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntimeout: i32, pvarpending: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::Online(this, core::mem::transmute_copy(&ntimeout)) {
                Ok(ok__) => {
                    pvarpending.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Offline<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntimeout: i32, pvarpending: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::Offline(this, core::mem::transmute_copy(&ntimeout)) {
                Ok(ok__) => {
                    pvarpending.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeResourceGroup<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presourcegroup: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResource_Impl::ChangeResourceGroup(this, windows_core::from_raw_borrowed(&presourcegroup)).into()
        }
        unsafe extern "system" fn AddResourceNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResource_Impl::AddResourceNode(this, windows_core::from_raw_borrowed(&pnode)).into()
        }
        unsafe extern "system" fn RemoveResourceNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResource_Impl::RemoveResourceNode(this, windows_core::from_raw_borrowed(&pnode)).into()
        }
        unsafe extern "system" fn CanResourceBeDependent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, presource: *mut core::ffi::c_void, pvardependent: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::CanResourceBeDependent(this, windows_core::from_raw_borrowed(&presource)) {
                Ok(ok__) => {
                    pvardependent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PossibleOwnerNodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppownernodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::PossibleOwnerNodes(this) {
                Ok(ok__) => {
                    ppownernodes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependencies<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresdependencies: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::Dependencies(this) {
                Ok(ok__) => {
                    ppresdependencies.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependents<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresdependents: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::Dependents(this) {
                Ok(ok__) => {
                    ppresdependents.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::Group(this) {
                Ok(ok__) => {
                    ppresgroup.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppownernode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::OwnerNode(this) {
                Ok(ok__) => {
                    ppownernode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cluster<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcluster: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::Cluster(this) {
                Ok(ok__) => {
                    ppcluster.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, prcclassinfo: *mut CLUSTER_RESOURCE_CLASS) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::ClassInfo(this) {
                Ok(ok__) => {
                    prcclassinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disk<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::Disk(this) {
                Ok(ok__) => {
                    ppdisk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegistryKeys<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppregistrykeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::RegistryKeys(this) {
                Ok(ok__) => {
                    ppregistrykeys.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CryptoKeys<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcryptokeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::CryptoKeys(this) {
                Ok(ok__) => {
                    ppcryptokeys.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtypename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::TypeName(this) {
                Ok(ok__) => {
                    pbstrtypename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresourcetype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::Type(this) {
                Ok(ok__) => {
                    ppresourcetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaintenanceMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmaintenancemode: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResource_Impl::MaintenanceMode(this) {
                Ok(ok__) => {
                    pbmaintenancemode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintenanceMode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmaintenancemode: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: ISClusResource_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResource_Impl::SetMaintenanceMode(this, core::mem::transmute_copy(&bmaintenancemode)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            CoreFlag: CoreFlag::<Identity, OFFSET>,
            BecomeQuorumResource: BecomeQuorumResource::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Fail: Fail::<Identity, OFFSET>,
            Online: Online::<Identity, OFFSET>,
            Offline: Offline::<Identity, OFFSET>,
            ChangeResourceGroup: ChangeResourceGroup::<Identity, OFFSET>,
            AddResourceNode: AddResourceNode::<Identity, OFFSET>,
            RemoveResourceNode: RemoveResourceNode::<Identity, OFFSET>,
            CanResourceBeDependent: CanResourceBeDependent::<Identity, OFFSET>,
            PossibleOwnerNodes: PossibleOwnerNodes::<Identity, OFFSET>,
            Dependencies: Dependencies::<Identity, OFFSET>,
            Dependents: Dependents::<Identity, OFFSET>,
            Group: Group::<Identity, OFFSET>,
            OwnerNode: OwnerNode::<Identity, OFFSET>,
            Cluster: Cluster::<Identity, OFFSET>,
            ClassInfo: ClassInfo::<Identity, OFFSET>,
            Disk: Disk::<Identity, OFFSET>,
            RegistryKeys: RegistryKeys::<Identity, OFFSET>,
            CryptoKeys: CryptoKeys::<Identity, OFFSET>,
            TypeName: TypeName::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            MaintenanceMode: MaintenanceMode::<Identity, OFFSET>,
            SetMaintenanceMode: SetMaintenanceMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResource as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusResources_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<ISClusResource>;
    fn CreateItem(&self, bstrresourcename: &windows_core::BSTR, bstrresourcetype: &windows_core::BSTR, bstrgroupname: &windows_core::BSTR, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS) -> windows_core::Result<ISClusResource>;
    fn DeleteItem(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusResources {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusResources_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusResources_Vtbl
    where
        Identity: ISClusResources_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResources_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResources_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResources_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, ppclusresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResources_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    ppclusresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourcename: core::mem::MaybeUninit<windows_core::BSTR>, bstrresourcetype: core::mem::MaybeUninit<windows_core::BSTR>, bstrgroupname: core::mem::MaybeUninit<windows_core::BSTR>, dwflags: CLUSTER_RESOURCE_CREATE_FLAGS, ppclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusResources_Impl::CreateItem(this, core::mem::transmute(&bstrresourcename), core::mem::transmute(&bstrresourcetype), core::mem::transmute(&bstrgroupname), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    ppclusterresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusResources_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusResources_Impl::DeleteItem(this, core::mem::transmute(&varindex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            CreateItem: CreateItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusResources as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusScsiAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PortNumber(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn PathId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn TargetId(&self) -> windows_core::Result<windows_core::VARIANT>;
    fn Lun(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusScsiAddress {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusScsiAddress_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusScsiAddress_Vtbl
    where
        Identity: ISClusScsiAddress_Impl,
    {
        unsafe extern "system" fn PortNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarportnumber: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusScsiAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusScsiAddress_Impl::PortNumber(this) {
                Ok(ok__) => {
                    pvarportnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarpathid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusScsiAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusScsiAddress_Impl::PathId(this) {
                Ok(ok__) => {
                    pvarpathid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvartargetid: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusScsiAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusScsiAddress_Impl::TargetId(this) {
                Ok(ok__) => {
                    pvartargetid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lun<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarlun: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusScsiAddress_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusScsiAddress_Impl::Lun(this) {
                Ok(ok__) => {
                    pvarlun.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PortNumber: PortNumber::<Identity, OFFSET>,
            PathId: PathId::<Identity, OFFSET>,
            TargetId: TargetId::<Identity, OFFSET>,
            Lun: Lun::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusScsiAddress as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusVersion_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
    fn BuildNumber(&self) -> windows_core::Result<i16>;
    fn VendorId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CSDVersion(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClusterHighestVersion(&self) -> windows_core::Result<i32>;
    fn ClusterLowestVersion(&self) -> windows_core::Result<i32>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn MixedVersion(&self) -> windows_core::Result<windows_core::VARIANT>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusVersion {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusVersion_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusVersion_Vtbl
    where
        Identity: ISClusVersion_Impl,
    {
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrclustername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusVersion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusVersion_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrclustername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnmajorversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusVersion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusVersion_Impl::MajorVersion(this) {
                Ok(ok__) => {
                    pnmajorversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnminorversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusVersion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusVersion_Impl::MinorVersion(this) {
                Ok(ok__) => {
                    pnminorversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildNumber<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnbuildnumber: *mut i16) -> windows_core::HRESULT
        where
            Identity: ISClusVersion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusVersion_Impl::BuildNumber(this) {
                Ok(ok__) => {
                    pnbuildnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorId<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrvendorid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusVersion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusVersion_Impl::VendorId(this) {
                Ok(ok__) => {
                    pbstrvendorid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSDVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcsdversion: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusVersion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusVersion_Impl::CSDVersion(this) {
                Ok(ok__) => {
                    pbstrcsdversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClusterHighestVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnclusterhighestversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusVersion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusVersion_Impl::ClusterHighestVersion(this) {
                Ok(ok__) => {
                    pnclusterhighestversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClusterLowestVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnclusterlowestversion: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusVersion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusVersion_Impl::ClusterLowestVersion(this) {
                Ok(ok__) => {
                    pnclusterlowestversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnflags: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusVersion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusVersion_Impl::Flags(this) {
                Ok(ok__) => {
                    pnflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MixedVersion<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarmixedversion: *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT
        where
            Identity: ISClusVersion_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusVersion_Impl::MixedVersion(this) {
                Ok(ok__) => {
                    pvarmixedversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
            BuildNumber: BuildNumber::<Identity, OFFSET>,
            VendorId: VendorId::<Identity, OFFSET>,
            CSDVersion: CSDVersion::<Identity, OFFSET>,
            ClusterHighestVersion: ClusterHighestVersion::<Identity, OFFSET>,
            ClusterLowestVersion: ClusterLowestVersion::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            MixedVersion: MixedVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusVersion as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISCluster_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CommonProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn CommonROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn PrivateROProperties(&self) -> windows_core::Result<ISClusProperties>;
    fn Handle(&self) -> windows_core::Result<usize>;
    fn Open(&self, bstrclustername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bstrclustername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self) -> windows_core::Result<ISClusVersion>;
    fn SetQuorumResource(&self, pclusterresource: Option<&ISClusResource>) -> windows_core::Result<()>;
    fn QuorumResource(&self) -> windows_core::Result<ISClusResource>;
    fn QuorumLogSize(&self) -> windows_core::Result<i32>;
    fn SetQuorumLogSize(&self, nlogsize: i32) -> windows_core::Result<()>;
    fn QuorumPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetQuorumPath(&self, ppath: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Nodes(&self) -> windows_core::Result<ISClusNodes>;
    fn ResourceGroups(&self) -> windows_core::Result<ISClusResGroups>;
    fn Resources(&self) -> windows_core::Result<ISClusResources>;
    fn ResourceTypes(&self) -> windows_core::Result<ISClusResTypes>;
    fn Networks(&self) -> windows_core::Result<ISClusNetworks>;
    fn NetInterfaces(&self) -> windows_core::Result<ISClusNetInterfaces>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISCluster {}
#[cfg(feature = "Win32_System_Com")]
impl ISCluster_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISCluster_Vtbl
    where
        Identity: ISCluster_Impl,
    {
        unsafe extern "system" fn CommonProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::CommonProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::PrivateProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::CommonROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateROProperties<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::PrivateROProperties(this) {
                Ok(ok__) => {
                    ppproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut usize) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::Handle(this) {
                Ok(ok__) => {
                    phandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclustername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCluster_Impl::Open(this, core::mem::transmute(&bstrclustername)).into()
        }
        unsafe extern "system" fn Name<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::Name(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclustername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCluster_Impl::SetName(this, core::mem::transmute(&bstrclustername)).into()
        }
        unsafe extern "system" fn Version<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusversion: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::Version(this) {
                Ok(ok__) => {
                    ppclusversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclusterresource: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCluster_Impl::SetQuorumResource(this, windows_core::from_raw_borrowed(&pclusterresource)).into()
        }
        unsafe extern "system" fn QuorumResource<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclusterresource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::QuorumResource(this) {
                Ok(ok__) => {
                    pclusterresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuorumLogSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnlogsize: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::QuorumLogSize(this) {
                Ok(ok__) => {
                    pnlogsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumLogSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, nlogsize: i32) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCluster_Impl::SetQuorumLogSize(this, core::mem::transmute_copy(&nlogsize)).into()
        }
        unsafe extern "system" fn QuorumPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::QuorumPath(this) {
                Ok(ok__) => {
                    pppath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuorumPath<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISCluster_Impl::SetQuorumPath(this, core::mem::transmute(&ppath)).into()
        }
        unsafe extern "system" fn Nodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnodes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::Nodes(this) {
                Ok(ok__) => {
                    ppnodes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceGroups<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusterresourcegroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::ResourceGroups(this) {
                Ok(ok__) => {
                    ppclusterresourcegroups.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclusterresources: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::Resources(this) {
                Ok(ok__) => {
                    ppclusterresources.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceTypes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresourcetypes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::ResourceTypes(this) {
                Ok(ok__) => {
                    ppresourcetypes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Networks<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetworks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::Networks(this) {
                Ok(ok__) => {
                    ppnetworks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetInterfaces<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetinterfaces: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISCluster_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISCluster_Impl::NetInterfaces(this) {
                Ok(ok__) => {
                    ppnetinterfaces.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CommonProperties: CommonProperties::<Identity, OFFSET>,
            PrivateProperties: PrivateProperties::<Identity, OFFSET>,
            CommonROProperties: CommonROProperties::<Identity, OFFSET>,
            PrivateROProperties: PrivateROProperties::<Identity, OFFSET>,
            Handle: Handle::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            SetQuorumResource: SetQuorumResource::<Identity, OFFSET>,
            QuorumResource: QuorumResource::<Identity, OFFSET>,
            QuorumLogSize: QuorumLogSize::<Identity, OFFSET>,
            SetQuorumLogSize: SetQuorumLogSize::<Identity, OFFSET>,
            QuorumPath: QuorumPath::<Identity, OFFSET>,
            SetQuorumPath: SetQuorumPath::<Identity, OFFSET>,
            Nodes: Nodes::<Identity, OFFSET>,
            ResourceGroups: ResourceGroups::<Identity, OFFSET>,
            Resources: Resources::<Identity, OFFSET>,
            ResourceTypes: ResourceTypes::<Identity, OFFSET>,
            Networks: Networks::<Identity, OFFSET>,
            NetInterfaces: NetInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISCluster as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISClusterNames_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn DomainName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISClusterNames {}
#[cfg(feature = "Win32_System_Com")]
impl ISClusterNames_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISClusterNames_Vtbl
    where
        Identity: ISClusterNames_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISClusterNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusterNames_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusterNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusterNames_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISClusterNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISClusterNames_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, pbstrclustername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusterNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusterNames_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    pbstrclustername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdomainname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISClusterNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISClusterNames_Impl::DomainName(this) {
                Ok(ok__) => {
                    pbstrdomainname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            DomainName: DomainName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISClusterNames as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISDomainNames_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Refresh(&self) -> windows_core::Result<()>;
    fn get_Item(&self, varindex: &windows_core::VARIANT) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISDomainNames {}
#[cfg(feature = "Win32_System_Com")]
impl ISDomainNames_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> ISDomainNames_Vtbl
    where
        Identity: ISDomainNames_Impl,
    {
        unsafe extern "system" fn Count<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT
        where
            Identity: ISDomainNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISDomainNames_Impl::Count(this) {
                Ok(ok__) => {
                    plcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISDomainNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISDomainNames_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    retval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: ISDomainNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISDomainNames_Impl::Refresh(this).into()
        }
        unsafe extern "system" fn get_Item<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, varindex: core::mem::MaybeUninit<windows_core::VARIANT>, pbstrdomainname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT
        where
            Identity: ISDomainNames_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISDomainNames_Impl::get_Item(this, core::mem::transmute(&varindex)) {
                Ok(ok__) => {
                    pbstrdomainname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Refresh: Refresh::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISDomainNames as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IWCContextMenuCallback_Impl: Sized {
    fn AddExtensionMenuItem(&self, lpszname: &windows_core::BSTR, lpszstatusbartext: &windows_core::BSTR, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWCContextMenuCallback {}
impl IWCContextMenuCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWCContextMenuCallback_Vtbl
    where
        Identity: IWCContextMenuCallback_Impl,
    {
        unsafe extern "system" fn AddExtensionMenuItem<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpszname: core::mem::MaybeUninit<windows_core::BSTR>, lpszstatusbartext: core::mem::MaybeUninit<windows_core::BSTR>, ncommandid: u32, nsubmenucommandid: u32, uflags: u32) -> windows_core::HRESULT
        where
            Identity: IWCContextMenuCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWCContextMenuCallback_Impl::AddExtensionMenuItem(this, core::mem::transmute(&lpszname), core::mem::transmute(&lpszstatusbartext), core::mem::transmute_copy(&ncommandid), core::mem::transmute_copy(&nsubmenucommandid), core::mem::transmute_copy(&uflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddExtensionMenuItem: AddExtensionMenuItem::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWCContextMenuCallback as windows_core::Interface>::IID
    }
}
pub trait IWCPropertySheetCallback_Impl: Sized {
    fn AddPropertySheetPage(&self, hpage: *const i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWCPropertySheetCallback {}
impl IWCPropertySheetCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWCPropertySheetCallback_Vtbl
    where
        Identity: IWCPropertySheetCallback_Impl,
    {
        unsafe extern "system" fn AddPropertySheetPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: *const i32) -> windows_core::HRESULT
        where
            Identity: IWCPropertySheetCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWCPropertySheetCallback_Impl::AddPropertySheetPage(this, core::mem::transmute_copy(&hpage)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddPropertySheetPage: AddPropertySheetPage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWCPropertySheetCallback as windows_core::Interface>::IID
    }
}
pub trait IWCWizard97Callback_Impl: Sized {
    fn AddWizard97Page(&self, hpage: *const i32) -> windows_core::Result<()>;
    fn EnableNext(&self, hpage: *const i32, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWCWizard97Callback {}
impl IWCWizard97Callback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWCWizard97Callback_Vtbl
    where
        Identity: IWCWizard97Callback_Impl,
    {
        unsafe extern "system" fn AddWizard97Page<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: *const i32) -> windows_core::HRESULT
        where
            Identity: IWCWizard97Callback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWCWizard97Callback_Impl::AddWizard97Page(this, core::mem::transmute_copy(&hpage)).into()
        }
        unsafe extern "system" fn EnableNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWCWizard97Callback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWCWizard97Callback_Impl::EnableNext(this, core::mem::transmute_copy(&hpage), core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddWizard97Page: AddWizard97Page::<Identity, OFFSET>,
            EnableNext: EnableNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWCWizard97Callback as windows_core::Interface>::IID
    }
}
pub trait IWCWizardCallback_Impl: Sized {
    fn AddWizardPage(&self, hpage: *const i32) -> windows_core::Result<()>;
    fn EnableNext(&self, hpage: *const i32, benable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWCWizardCallback {}
impl IWCWizardCallback_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWCWizardCallback_Vtbl
    where
        Identity: IWCWizardCallback_Impl,
    {
        unsafe extern "system" fn AddWizardPage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: *const i32) -> windows_core::HRESULT
        where
            Identity: IWCWizardCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWCWizardCallback_Impl::AddWizardPage(this, core::mem::transmute_copy(&hpage)).into()
        }
        unsafe extern "system" fn EnableNext<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hpage: *const i32, benable: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IWCWizardCallback_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWCWizardCallback_Impl::EnableNext(this, core::mem::transmute_copy(&hpage), core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddWizardPage: AddWizardPage::<Identity, OFFSET>,
            EnableNext: EnableNext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWCWizardCallback as windows_core::Interface>::IID
    }
}
pub trait IWEExtendContextMenu_Impl: Sized {
    fn AddContextMenuItems(&self, pidata: Option<&windows_core::IUnknown>, picallback: Option<&IWCContextMenuCallback>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWEExtendContextMenu {}
impl IWEExtendContextMenu_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWEExtendContextMenu_Vtbl
    where
        Identity: IWEExtendContextMenu_Impl,
    {
        unsafe extern "system" fn AddContextMenuItems<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidata: *mut core::ffi::c_void, picallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWEExtendContextMenu_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWEExtendContextMenu_Impl::AddContextMenuItems(this, windows_core::from_raw_borrowed(&pidata), windows_core::from_raw_borrowed(&picallback)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddContextMenuItems: AddContextMenuItems::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWEExtendContextMenu as windows_core::Interface>::IID
    }
}
pub trait IWEExtendPropertySheet_Impl: Sized {
    fn CreatePropertySheetPages(&self, pidata: Option<&windows_core::IUnknown>, picallback: Option<&IWCPropertySheetCallback>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWEExtendPropertySheet {}
impl IWEExtendPropertySheet_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWEExtendPropertySheet_Vtbl
    where
        Identity: IWEExtendPropertySheet_Impl,
    {
        unsafe extern "system" fn CreatePropertySheetPages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidata: *mut core::ffi::c_void, picallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWEExtendPropertySheet_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWEExtendPropertySheet_Impl::CreatePropertySheetPages(this, windows_core::from_raw_borrowed(&pidata), windows_core::from_raw_borrowed(&picallback)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreatePropertySheetPages: CreatePropertySheetPages::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWEExtendPropertySheet as windows_core::Interface>::IID
    }
}
pub trait IWEExtendWizard_Impl: Sized {
    fn CreateWizardPages(&self, pidata: Option<&windows_core::IUnknown>, picallback: Option<&IWCWizardCallback>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWEExtendWizard {}
impl IWEExtendWizard_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWEExtendWizard_Vtbl
    where
        Identity: IWEExtendWizard_Impl,
    {
        unsafe extern "system" fn CreateWizardPages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidata: *mut core::ffi::c_void, picallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWEExtendWizard_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWEExtendWizard_Impl::CreateWizardPages(this, windows_core::from_raw_borrowed(&pidata), windows_core::from_raw_borrowed(&picallback)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateWizardPages: CreateWizardPages::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWEExtendWizard as windows_core::Interface>::IID
    }
}
pub trait IWEExtendWizard97_Impl: Sized {
    fn CreateWizard97Pages(&self, pidata: Option<&windows_core::IUnknown>, picallback: Option<&IWCWizard97Callback>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWEExtendWizard97 {}
impl IWEExtendWizard97_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWEExtendWizard97_Vtbl
    where
        Identity: IWEExtendWizard97_Impl,
    {
        unsafe extern "system" fn CreateWizard97Pages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidata: *mut core::ffi::c_void, picallback: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWEExtendWizard97_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWEExtendWizard97_Impl::CreateWizard97Pages(this, windows_core::from_raw_borrowed(&pidata), windows_core::from_raw_borrowed(&picallback)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateWizard97Pages: CreateWizard97Pages::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWEExtendWizard97 as windows_core::Interface>::IID
    }
}
pub trait IWEInvokeCommand_Impl: Sized {
    fn InvokeCommand(&self, ncommandid: u32, pidata: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWEInvokeCommand {}
impl IWEInvokeCommand_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IWEInvokeCommand_Vtbl
    where
        Identity: IWEInvokeCommand_Impl,
    {
        unsafe extern "system" fn InvokeCommand<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncommandid: u32, pidata: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IWEInvokeCommand_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWEInvokeCommand_Impl::InvokeCommand(this, core::mem::transmute_copy(&ncommandid), windows_core::from_raw_borrowed(&pidata)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InvokeCommand: InvokeCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWEInvokeCommand as windows_core::Interface>::IID
    }
}
