windows_core::imp::define_interface!(IOplockBreakingHandler, IOplockBreakingHandler_Vtbl, 0x826abe3d_3acd_47d3_84f2_88aaedcf6304);
impl core::ops::Deref for IOplockBreakingHandler {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IOplockBreakingHandler, windows_core::IUnknown);
impl IOplockBreakingHandler {
    pub unsafe fn OplockBreaking(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OplockBreaking)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IOplockBreakingHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OplockBreaking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRandomAccessStreamFileAccessMode, IRandomAccessStreamFileAccessMode_Vtbl, 0x332e5848_2e15_458e_85c4_c911c0c3d6f4);
impl core::ops::Deref for IRandomAccessStreamFileAccessMode {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRandomAccessStreamFileAccessMode, windows_core::IUnknown);
impl IRandomAccessStreamFileAccessMode {
    pub unsafe fn GetMode(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IRandomAccessStreamFileAccessMode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageFolderHandleAccess, IStorageFolderHandleAccess_Vtbl, 0xdf19938f_5462_48a0_be65_d2a3271a08d6);
impl core::ops::Deref for IStorageFolderHandleAccess {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageFolderHandleAccess, windows_core::IUnknown);
impl IStorageFolderHandleAccess {
    pub unsafe fn Create<P0, P1>(&self, filename: P0, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: P1) -> windows_core::Result<super::super::super::Foundation::HANDLE>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IOplockBreakingHandler>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), filename.param().abi(), creationoptions, accessoptions, sharingoptions, options, oplockbreakinghandler.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IStorageFolderHandleAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, HANDLE_CREATION_OPTIONS, HANDLE_ACCESS_OPTIONS, HANDLE_SHARING_OPTIONS, HANDLE_OPTIONS, *mut core::ffi::c_void, *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageItemHandleAccess, IStorageItemHandleAccess_Vtbl, 0x5ca296b2_2c25_4d22_b785_b885c8201e6a);
impl core::ops::Deref for IStorageItemHandleAccess {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStorageItemHandleAccess, windows_core::IUnknown);
impl IStorageItemHandleAccess {
    pub unsafe fn Create<P0>(&self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: P0) -> windows_core::Result<super::super::super::Foundation::HANDLE>
    where
        P0: windows_core::Param<IOplockBreakingHandler>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Create)(windows_core::Interface::as_raw(self), accessoptions, sharingoptions, options, oplockbreakinghandler.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IStorageItemHandleAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, HANDLE_ACCESS_OPTIONS, HANDLE_SHARING_OPTIONS, HANDLE_OPTIONS, *mut core::ffi::c_void, *mut super::super::super::Foundation::HANDLE) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUnbufferedFileHandleOplockCallback, IUnbufferedFileHandleOplockCallback_Vtbl, 0xd1019a0e_6243_4329_8497_2e75894d7710);
impl core::ops::Deref for IUnbufferedFileHandleOplockCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUnbufferedFileHandleOplockCallback, windows_core::IUnknown);
impl IUnbufferedFileHandleOplockCallback {
    pub unsafe fn OnBrokenCallback(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OnBrokenCallback)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUnbufferedFileHandleOplockCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnBrokenCallback: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUnbufferedFileHandleProvider, IUnbufferedFileHandleProvider_Vtbl, 0xa65c9109_42ab_4b94_a7b1_dd2e4e68515e);
impl core::ops::Deref for IUnbufferedFileHandleProvider {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUnbufferedFileHandleProvider, windows_core::IUnknown);
impl IUnbufferedFileHandleProvider {
    pub unsafe fn OpenUnbufferedFileHandle<P0>(&self, oplockbreakcallback: P0) -> windows_core::Result<usize>
    where
        P0: windows_core::Param<IUnbufferedFileHandleOplockCallback>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OpenUnbufferedFileHandle)(windows_core::Interface::as_raw(self), oplockbreakcallback.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn CloseUnbufferedFileHandle(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CloseUnbufferedFileHandle)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IUnbufferedFileHandleProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OpenUnbufferedFileHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub CloseUnbufferedFileHandle: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub const HAO_DELETE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(65536i32);
pub const HAO_NONE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(0i32);
pub const HAO_READ: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(1179785i32);
pub const HAO_READ_ATTRIBUTES: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(128i32);
pub const HAO_WRITE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(1179926i32);
pub const HCO_CREATE_ALWAYS: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(2i32);
pub const HCO_CREATE_NEW: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(1i32);
pub const HCO_OPEN_ALWAYS: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(4i32);
pub const HCO_OPEN_EXISTING: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(3i32);
pub const HCO_TRUNCATE_EXISTING: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(5i32);
pub const HO_DELETE_ON_CLOSE: HANDLE_OPTIONS = HANDLE_OPTIONS(67108864u32);
pub const HO_NONE: HANDLE_OPTIONS = HANDLE_OPTIONS(0u32);
pub const HO_NO_BUFFERING: HANDLE_OPTIONS = HANDLE_OPTIONS(536870912u32);
pub const HO_OPEN_REQUIRING_OPLOCK: HANDLE_OPTIONS = HANDLE_OPTIONS(262144u32);
pub const HO_OVERLAPPED: HANDLE_OPTIONS = HANDLE_OPTIONS(1073741824u32);
pub const HO_RANDOM_ACCESS: HANDLE_OPTIONS = HANDLE_OPTIONS(268435456u32);
pub const HO_SEQUENTIAL_SCAN: HANDLE_OPTIONS = HANDLE_OPTIONS(134217728u32);
pub const HO_WRITE_THROUGH: HANDLE_OPTIONS = HANDLE_OPTIONS(2147483648u32);
pub const HSO_SHARE_DELETE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(4i32);
pub const HSO_SHARE_NONE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(0i32);
pub const HSO_SHARE_READ: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(1i32);
pub const HSO_SHARE_WRITE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HANDLE_ACCESS_OPTIONS(pub i32);
impl windows_core::TypeKind for HANDLE_ACCESS_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HANDLE_ACCESS_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HANDLE_ACCESS_OPTIONS").field(&self.0).finish()
    }
}
impl HANDLE_ACCESS_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HANDLE_ACCESS_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HANDLE_ACCESS_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HANDLE_CREATION_OPTIONS(pub i32);
impl windows_core::TypeKind for HANDLE_CREATION_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HANDLE_CREATION_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HANDLE_CREATION_OPTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HANDLE_OPTIONS(pub u32);
impl windows_core::TypeKind for HANDLE_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HANDLE_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HANDLE_OPTIONS").field(&self.0).finish()
    }
}
impl HANDLE_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HANDLE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HANDLE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HANDLE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HANDLE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HANDLE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HANDLE_SHARING_OPTIONS(pub i32);
impl windows_core::TypeKind for HANDLE_SHARING_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HANDLE_SHARING_OPTIONS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HANDLE_SHARING_OPTIONS").field(&self.0).finish()
    }
}
impl HANDLE_SHARING_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HANDLE_SHARING_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HANDLE_SHARING_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
