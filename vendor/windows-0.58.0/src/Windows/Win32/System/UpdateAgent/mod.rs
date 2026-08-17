#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAutomaticUpdates, IAutomaticUpdates_Vtbl, 0x673425bf_c082_4c7c_bdfd_569464b8e0ce);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAutomaticUpdates {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdates, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdates {
    pub unsafe fn DetectNow(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).DetectNow)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Resume)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ShowSettingsDialog(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShowSettingsDialog)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Settings(&self) -> windows_core::Result<IAutomaticUpdatesSettings> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Settings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ServiceEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EnableService(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).EnableService)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdates_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DetectNow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Resume: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowSettingsDialog: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Settings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Settings: usize,
    pub ServiceEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EnableService: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAutomaticUpdates2, IAutomaticUpdates2_Vtbl, 0x4a2f5c31_cfd9_410e_b7fb_29a653973a0f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAutomaticUpdates2 {
    type Target = IAutomaticUpdates;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdates2, windows_core::IUnknown, super::Com::IDispatch, IAutomaticUpdates);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdates2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Results(&self) -> windows_core::Result<IAutomaticUpdatesResults> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Results)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdates2_Vtbl {
    pub base__: IAutomaticUpdates_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Results: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Results: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAutomaticUpdatesResults, IAutomaticUpdatesResults_Vtbl, 0xe7a4d634_7942_4dd9_a111_82228ba33901);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAutomaticUpdatesResults {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdatesResults, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesResults {
    pub unsafe fn LastSearchSuccessDate(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastSearchSuccessDate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastInstallationSuccessDate(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastInstallationSuccessDate)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdatesResults_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub LastSearchSuccessDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub LastInstallationSuccessDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAutomaticUpdatesSettings, IAutomaticUpdatesSettings_Vtbl, 0x2ee48f22_af3c_405f_8970_f71be12ee9a2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAutomaticUpdatesSettings {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdatesSettings, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings {
    pub unsafe fn NotificationLevel(&self) -> windows_core::Result<AutomaticUpdatesNotificationLevel> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NotificationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNotificationLevel(&self, value: AutomaticUpdatesNotificationLevel) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetNotificationLevel)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Required(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Required)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ScheduledInstallationDay(&self) -> windows_core::Result<AutomaticUpdatesScheduledInstallationDay> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScheduledInstallationDay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScheduledInstallationDay(&self, value: AutomaticUpdatesScheduledInstallationDay) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScheduledInstallationDay)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn ScheduledInstallationTime(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ScheduledInstallationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetScheduledInstallationTime(&self, value: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetScheduledInstallationTime)(windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn Refresh(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Refresh)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdatesSettings_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub NotificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT,
    pub SetNotificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, AutomaticUpdatesNotificationLevel) -> windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Required: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ScheduledInstallationDay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT,
    pub SetScheduledInstallationDay: unsafe extern "system" fn(*mut core::ffi::c_void, AutomaticUpdatesScheduledInstallationDay) -> windows_core::HRESULT,
    pub ScheduledInstallationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetScheduledInstallationTime: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Refresh: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAutomaticUpdatesSettings2, IAutomaticUpdatesSettings2_Vtbl, 0x6abc136a_c3ca_4384_8171_cb2b1e59b8dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAutomaticUpdatesSettings2 {
    type Target = IAutomaticUpdatesSettings;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdatesSettings2, windows_core::IUnknown, super::Com::IDispatch, IAutomaticUpdatesSettings);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings2 {
    pub unsafe fn IncludeRecommendedUpdates(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IncludeRecommendedUpdates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIncludeRecommendedUpdates<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIncludeRecommendedUpdates)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn CheckPermission(&self, usertype: AutomaticUpdatesUserType, permissiontype: AutomaticUpdatesPermissionType) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckPermission)(windows_core::Interface::as_raw(self), usertype, permissiontype, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdatesSettings2_Vtbl {
    pub base__: IAutomaticUpdatesSettings_Vtbl,
    pub IncludeRecommendedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIncludeRecommendedUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CheckPermission: unsafe extern "system" fn(*mut core::ffi::c_void, AutomaticUpdatesUserType, AutomaticUpdatesPermissionType, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IAutomaticUpdatesSettings3, IAutomaticUpdatesSettings3_Vtbl, 0xb587f5c3_f57e_485f_bbf5_0d181c5cd0dc);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IAutomaticUpdatesSettings3 {
    type Target = IAutomaticUpdatesSettings2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IAutomaticUpdatesSettings3, windows_core::IUnknown, super::Com::IDispatch, IAutomaticUpdatesSettings, IAutomaticUpdatesSettings2);
#[cfg(feature = "Win32_System_Com")]
impl IAutomaticUpdatesSettings3 {
    pub unsafe fn NonAdministratorsElevated(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NonAdministratorsElevated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetNonAdministratorsElevated<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetNonAdministratorsElevated)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn FeaturedUpdatesEnabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FeaturedUpdatesEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFeaturedUpdatesEnabled<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFeaturedUpdatesEnabled)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IAutomaticUpdatesSettings3_Vtbl {
    pub base__: IAutomaticUpdatesSettings2_Vtbl,
    pub NonAdministratorsElevated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetNonAdministratorsElevated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub FeaturedUpdatesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetFeaturedUpdatesEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICategory, ICategory_Vtbl, 0x81ddc1b8_9d35_47a6_b471_5b80f519223b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICategory {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICategory, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICategory {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CategoryID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CategoryID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Children(&self) -> windows_core::Result<ICategoryCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Children)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> windows_core::Result<IImageInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Image)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Order(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Order)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> windows_core::Result<ICategory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Type(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICategory_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CategoryID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Image: usize,
    pub Order: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ICategoryCollection, ICategoryCollection_Vtbl, 0x3a56bfb8_576c_43f7_9335_fe4838fd7e37);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ICategoryCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ICategoryCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ICategoryCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<ICategory> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ICategoryCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDownloadCompletedCallback, IDownloadCompletedCallback_Vtbl, 0x77254866_9f5b_4c8e_b9e2_c77a8530d64b);
impl core::ops::Deref for IDownloadCompletedCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDownloadCompletedCallback, windows_core::IUnknown);
impl IDownloadCompletedCallback {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invoke<P0, P1>(&self, downloadjob: P0, callbackargs: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDownloadJob>,
        P1: windows_core::Param<IDownloadCompletedCallbackArgs>,
    {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), downloadjob.param().abi(), callbackargs.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDownloadCompletedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDownloadCompletedCallbackArgs, IDownloadCompletedCallbackArgs_Vtbl, 0xfa565b23_498c_47a0_979d_e7d5b1813360);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDownloadCompletedCallbackArgs {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDownloadCompletedCallbackArgs, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadCompletedCallbackArgs_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDownloadJob, IDownloadJob_Vtbl, 0xc574de85_7358_43f6_aae8_8697e62d8ba7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDownloadJob {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDownloadJob, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadJob {
    pub unsafe fn AsyncState(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AsyncState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsCompleted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCompleted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CleanUp(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CleanUp)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProgress(&self) -> windows_core::Result<IDownloadProgress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestAbort)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadJob_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    pub CleanUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProgress: usize,
    pub RequestAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDownloadProgress, IDownloadProgress_Vtbl, 0xd31a5bac_f719_4178_9dbb_5e2cb47fd18a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDownloadProgress {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDownloadProgress, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgress {
    pub unsafe fn CurrentUpdateBytesDownloaded(&self) -> windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentUpdateBytesDownloaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentUpdateBytesToDownload(&self) -> windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentUpdateBytesToDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentUpdateIndex(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentUpdateIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PercentComplete(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalBytesDownloaded(&self) -> windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalBytesDownloaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn TotalBytesToDownload(&self) -> windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TotalBytesToDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateDownloadResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CurrentUpdateDownloadPhase(&self) -> windows_core::Result<DownloadPhase> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentUpdateDownloadPhase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentUpdatePercentComplete(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentUpdatePercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadProgress_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CurrentUpdateBytesDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT,
    pub CurrentUpdateBytesToDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT,
    pub CurrentUpdateIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalBytesDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT,
    pub TotalBytesToDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
    pub CurrentUpdateDownloadPhase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DownloadPhase) -> windows_core::HRESULT,
    pub CurrentUpdatePercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDownloadProgressChangedCallback, IDownloadProgressChangedCallback_Vtbl, 0x8c3f1cdd_6173_4591_aebd_a56a53ca77c1);
impl core::ops::Deref for IDownloadProgressChangedCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDownloadProgressChangedCallback, windows_core::IUnknown);
impl IDownloadProgressChangedCallback {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invoke<P0, P1>(&self, downloadjob: P0, callbackargs: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDownloadJob>,
        P1: windows_core::Param<IDownloadProgressChangedCallbackArgs>,
    {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), downloadjob.param().abi(), callbackargs.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IDownloadProgressChangedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDownloadProgressChangedCallbackArgs, IDownloadProgressChangedCallbackArgs_Vtbl, 0x324ff2c6_4981_4b04_9412_57481745ab24);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDownloadProgressChangedCallbackArgs {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDownloadProgressChangedCallbackArgs, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadProgressChangedCallbackArgs {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Progress(&self) -> windows_core::Result<IDownloadProgress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Progress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadProgressChangedCallbackArgs_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Progress: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IDownloadResult, IDownloadResult_Vtbl, 0xdaa4fdd0_4727_4dbe_a1e7_745dca317144);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IDownloadResult {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IDownloadResult, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IDownloadResult {
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateDownloadResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IDownloadResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IImageInformation, IImageInformation_Vtbl, 0x7c907864_346c_4aeb_8f3f_57da289f969f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IImageInformation {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IImageInformation, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IImageInformation {
    pub unsafe fn AltText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AltText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Height(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Source(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Source)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Width(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IImageInformation_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AltText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IInstallationAgent, IInstallationAgent_Vtbl, 0x925cbc18_a2ea_4648_bf1c_ec8badcfe20a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IInstallationAgent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IInstallationAgent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationAgent {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecordInstallationResult<P0, P1>(&self, installationresultcookie: P0, hresult: i32, extendedreportingdata: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IStringCollection>,
    {
        (windows_core::Interface::vtable(self).RecordInstallationResult)(windows_core::Interface::as_raw(self), installationresultcookie.param().abi(), hresult, extendedreportingdata.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationAgent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RecordInstallationResult: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecordInstallationResult: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IInstallationBehavior, IInstallationBehavior_Vtbl, 0xd9a59339_e245_4dbd_9686_4d5763e39624);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IInstallationBehavior {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IInstallationBehavior, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationBehavior {
    pub unsafe fn CanRequestUserInput(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanRequestUserInput)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Impact(&self) -> windows_core::Result<InstallationImpact> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Impact)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RebootBehavior(&self) -> windows_core::Result<InstallationRebootBehavior> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RebootBehavior)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RequiresNetworkConnectivity(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RequiresNetworkConnectivity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationBehavior_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CanRequestUserInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Impact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InstallationImpact) -> windows_core::HRESULT,
    pub RebootBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InstallationRebootBehavior) -> windows_core::HRESULT,
    pub RequiresNetworkConnectivity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInstallationCompletedCallback, IInstallationCompletedCallback_Vtbl, 0x45f4f6f3_d602_4f98_9a8a_3efa152ad2d3);
impl core::ops::Deref for IInstallationCompletedCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInstallationCompletedCallback, windows_core::IUnknown);
impl IInstallationCompletedCallback {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invoke<P0, P1>(&self, installationjob: P0, callbackargs: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IInstallationJob>,
        P1: windows_core::Param<IInstallationCompletedCallbackArgs>,
    {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), installationjob.param().abi(), callbackargs.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IInstallationCompletedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IInstallationCompletedCallbackArgs, IInstallationCompletedCallbackArgs_Vtbl, 0x250e2106_8efb_4705_9653_ef13c581b6a1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IInstallationCompletedCallbackArgs {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IInstallationCompletedCallbackArgs, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationCompletedCallbackArgs_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IInstallationJob, IInstallationJob_Vtbl, 0x5c209f0b_bad5_432a_9556_4699bed2638a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IInstallationJob {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IInstallationJob, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationJob {
    pub unsafe fn AsyncState(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AsyncState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsCompleted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCompleted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CleanUp(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CleanUp)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProgress(&self) -> windows_core::Result<IInstallationProgress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RequestAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestAbort)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationJob_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    pub CleanUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProgress: usize,
    pub RequestAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IInstallationProgress, IInstallationProgress_Vtbl, 0x345c8244_43a3_4e32_a368_65f073b76f36);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IInstallationProgress {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IInstallationProgress, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgress {
    pub unsafe fn CurrentUpdateIndex(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentUpdateIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CurrentUpdatePercentComplete(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CurrentUpdatePercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn PercentComplete(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PercentComplete)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateInstallationResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationProgress_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CurrentUpdateIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentUpdatePercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
}
windows_core::imp::define_interface!(IInstallationProgressChangedCallback, IInstallationProgressChangedCallback_Vtbl, 0xe01402d5_f8da_43ba_a012_38894bd048f1);
impl core::ops::Deref for IInstallationProgressChangedCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IInstallationProgressChangedCallback, windows_core::IUnknown);
impl IInstallationProgressChangedCallback {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invoke<P0, P1>(&self, installationjob: P0, callbackargs: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IInstallationJob>,
        P1: windows_core::Param<IInstallationProgressChangedCallbackArgs>,
    {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), installationjob.param().abi(), callbackargs.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IInstallationProgressChangedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IInstallationProgressChangedCallbackArgs, IInstallationProgressChangedCallbackArgs_Vtbl, 0xe4f14e1e_689d_4218_a0b9_bc189c484a01);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IInstallationProgressChangedCallbackArgs {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IInstallationProgressChangedCallbackArgs, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationProgressChangedCallbackArgs {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Progress(&self) -> windows_core::Result<IInstallationProgress> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Progress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationProgressChangedCallbackArgs_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Progress: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IInstallationResult, IInstallationResult_Vtbl, 0xa43c56d6_7451_48d4_af96_b6cd2d0d9b7a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IInstallationResult {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IInstallationResult, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IInstallationResult {
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RebootRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetUpdateResult(&self, updateindex: i32) -> windows_core::Result<IUpdateInstallationResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetUpdateResult)(windows_core::Interface::as_raw(self), updateindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInstallationResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetUpdateResult: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetUpdateResult: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IInvalidProductLicenseException, IInvalidProductLicenseException_Vtbl, 0xa37d00f5_7bb0_4953_b414_f9e98326f2e8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IInvalidProductLicenseException {
    type Target = IUpdateException;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IInvalidProductLicenseException, windows_core::IUnknown, super::Com::IDispatch, IUpdateException);
#[cfg(feature = "Win32_System_Com")]
impl IInvalidProductLicenseException {
    pub unsafe fn Product(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Product)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IInvalidProductLicenseException_Vtbl {
    pub base__: IUpdateException_Vtbl,
    pub Product: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISearchCompletedCallback, ISearchCompletedCallback_Vtbl, 0x88aee058_d4b0_4725_a2f1_814a67ae964c);
impl core::ops::Deref for ISearchCompletedCallback {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISearchCompletedCallback, windows_core::IUnknown);
impl ISearchCompletedCallback {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Invoke<P0, P1>(&self, searchjob: P0, callbackargs: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ISearchJob>,
        P1: windows_core::Param<ISearchCompletedCallbackArgs>,
    {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), searchjob.param().abi(), callbackargs.param().abi()).ok()
    }
}
#[repr(C)]
pub struct ISearchCompletedCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Invoke: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISearchCompletedCallbackArgs, ISearchCompletedCallbackArgs_Vtbl, 0xa700a634_2850_4c47_938a_9e4b6e5af9a6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISearchCompletedCallbackArgs {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISearchCompletedCallbackArgs, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISearchCompletedCallbackArgs {}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISearchCompletedCallbackArgs_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISearchJob, ISearchJob_Vtbl, 0x7366ea16_7a1a_4ea2_b042_973d3e9cd99b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISearchJob {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISearchJob, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISearchJob {
    pub unsafe fn AsyncState(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AsyncState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsCompleted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsCompleted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CleanUp(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CleanUp)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn RequestAbort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RequestAbort)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISearchJob_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub AsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub IsCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CleanUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISearchResult, ISearchResult_Vtbl, 0xd40cff62_e08c_4498_941a_01e25f0fd33c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISearchResult {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISearchResult, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISearchResult {
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootCategories(&self) -> windows_core::Result<ICategoryCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RootCategories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Warnings(&self) -> windows_core::Result<IUpdateExceptionCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Warnings)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISearchResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootCategories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootCategories: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Warnings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Warnings: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IStringCollection, IStringCollection_Vtbl, 0xeff90582_2ddc_480f_a06d_60f3fbc362c3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IStringCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IStringCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IStringCollection {
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn put_Item<P0>(&self, index: i32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).put_Item)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok()
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Add<P0>(&self, value: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Copy(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Insert<P0>(&self, index: i32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IStringCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub put_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Copy: usize,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(ISystemInformation, ISystemInformation_Vtbl, 0xade87bf7_7b56_4275_8fab_b9b0e591844b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for ISystemInformation {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(ISystemInformation, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ISystemInformation {
    pub unsafe fn OemHardwareSupportLink(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OemHardwareSupportLink)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RebootRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct ISystemInformation_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub OemHardwareSupportLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdate, IUpdate_Vtbl, 0x6a92b07a_d821_4682_b423_5c805022cc4d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdate {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdate, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdate {
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AutoSelectOnWebSites(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoSelectOnWebSites)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BundledUpdates(&self) -> windows_core::Result<IUpdateCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BundledUpdates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn CanRequireSource(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanRequireSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> windows_core::Result<ICategoryCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Categories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Deadline(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Deadline)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DeltaCompressedContentAvailable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeltaCompressedContentAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DeltaCompressedContentPreferred(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeltaCompressedContentPreferred)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EulaAccepted(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EulaAccepted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn EulaText(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EulaText)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HandlerID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HandlerID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Identity(&self) -> windows_core::Result<IUpdateIdentity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Identity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Image(&self) -> windows_core::Result<IImageInformation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Image)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InstallationBehavior)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsBeta(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsBeta)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsDownloaded(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDownloaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsHidden(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsHidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsHidden<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsHidden)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn IsInstalled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsInstalled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsMandatory(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsMandatory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsUninstallable(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsUninstallable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Languages(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Languages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn LastDeploymentChangeTime(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).LastDeploymentChangeTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MaxDownloadSize(&self) -> windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MaxDownloadSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn MinDownloadSize(&self) -> windows_core::Result<super::super::Foundation::DECIMAL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MinDownloadSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn MoreInfoUrls(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MoreInfoUrls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn MsrcSeverity(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MsrcSeverity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RecommendedCpuSpeed(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RecommendedCpuSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RecommendedHardDiskSpace(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RecommendedHardDiskSpace)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RecommendedMemory(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RecommendedMemory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ReleaseNotes(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReleaseNotes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SecurityBulletinIDs(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SecurityBulletinIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupersededUpdateIDs(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupersededUpdateIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SupportUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Type(&self) -> windows_core::Result<UpdateType> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UninstallationNotes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationBehavior(&self) -> windows_core::Result<IInstallationBehavior> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UninstallationBehavior)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UninstallationSteps)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn KBArticleIDs(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).KBArticleIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AcceptEula(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).AcceptEula)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DeploymentAction(&self) -> windows_core::Result<DeploymentAction> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeploymentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CopyFromCache<P0, P1>(&self, path: P0, toextractcabfiles: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).CopyFromCache)(windows_core::Interface::as_raw(self), path.param().abi(), toextractcabfiles.param().abi()).ok()
    }
    pub unsafe fn DownloadPriority(&self) -> windows_core::Result<DownloadPriority> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DownloadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DownloadContents(&self) -> windows_core::Result<IUpdateDownloadContentCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DownloadContents)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdate_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AutoSelectOnWebSites: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BundledUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BundledUpdates: usize,
    pub CanRequireSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Categories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Categories: usize,
    pub Deadline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub DeltaCompressedContentAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DeltaCompressedContentPreferred: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EulaAccepted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub EulaText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HandlerID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Identity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Identity: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Image: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Image: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InstallationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InstallationBehavior: usize,
    pub IsBeta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsDownloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsInstalled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsMandatory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsUninstallable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Languages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Languages: usize,
    pub LastDeploymentChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub MaxDownloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT,
    pub MinDownloadSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DECIMAL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub MoreInfoUrls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    MoreInfoUrls: usize,
    pub MsrcSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RecommendedCpuSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RecommendedHardDiskSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RecommendedMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReleaseNotes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SecurityBulletinIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SecurityBulletinIDs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SupersededUpdateIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SupersededUpdateIDs: usize,
    pub SupportUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateType) -> windows_core::HRESULT,
    pub UninstallationNotes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UninstallationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UninstallationBehavior: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UninstallationSteps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UninstallationSteps: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub KBArticleIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    KBArticleIDs: usize,
    pub AcceptEula: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeploymentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DeploymentAction) -> windows_core::HRESULT,
    pub CopyFromCache: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub DownloadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DownloadPriority) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DownloadContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DownloadContents: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdate2, IUpdate2_Vtbl, 0x144fe9b0_d23d_4a8b_8634_fb4457533b7a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdate2 {
    type Target = IUpdate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdate2, windows_core::IUnknown, super::Com::IDispatch, IUpdate);
#[cfg(feature = "Win32_System_Com")]
impl IUpdate2 {
    pub unsafe fn RebootRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsPresent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPresent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CveIDs(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CveIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyToCache<P0>(&self, pfiles: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStringCollection>,
    {
        (windows_core::Interface::vtable(self).CopyToCache)(windows_core::Interface::as_raw(self), pfiles.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdate2_Vtbl {
    pub base__: IUpdate_Vtbl,
    pub RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CveIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CveIDs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyToCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyToCache: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdate3, IUpdate3_Vtbl, 0x112eda6b_95b3_476f_9d90_aee82c6b8181);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdate3 {
    type Target = IUpdate2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdate3, windows_core::IUnknown, super::Com::IDispatch, IUpdate, IUpdate2);
#[cfg(feature = "Win32_System_Com")]
impl IUpdate3 {
    pub unsafe fn BrowseOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BrowseOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdate3_Vtbl {
    pub base__: IUpdate2_Vtbl,
    pub BrowseOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdate4, IUpdate4_Vtbl, 0x27e94b0d_5139_49a2_9a61_93522dc54652);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdate4 {
    type Target = IUpdate3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdate4, windows_core::IUnknown, super::Com::IDispatch, IUpdate, IUpdate2, IUpdate3);
#[cfg(feature = "Win32_System_Com")]
impl IUpdate4 {
    pub unsafe fn PerUser(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PerUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdate4_Vtbl {
    pub base__: IUpdate3_Vtbl,
    pub PerUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdate5, IUpdate5_Vtbl, 0xc1c2f21a_d2f4_4902_b5c6_8a081c19a890);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdate5 {
    type Target = IUpdate4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdate5, windows_core::IUnknown, super::Com::IDispatch, IUpdate, IUpdate2, IUpdate3, IUpdate4);
#[cfg(feature = "Win32_System_Com")]
impl IUpdate5 {
    pub unsafe fn AutoSelection(&self) -> windows_core::Result<AutoSelectionMode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AutoDownload(&self) -> windows_core::Result<AutoDownloadMode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdate5_Vtbl {
    pub base__: IUpdate4_Vtbl,
    pub AutoSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoSelectionMode) -> windows_core::HRESULT,
    pub AutoDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoDownloadMode) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateCollection, IUpdateCollection_Vtbl, 0x07f7438c_7709_4ca5_b518_91279288134e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IUpdate> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn put_Item<P0>(&self, index: i32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUpdate>,
    {
        (windows_core::Interface::vtable(self).put_Item)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok()
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, value: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IUpdate>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn Clear(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Copy(&self) -> windows_core::Result<IUpdateCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Copy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Insert<P0>(&self, index: i32, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUpdate>,
    {
        (windows_core::Interface::vtable(self).Insert)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub put_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    put_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Copy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Insert: usize,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateDownloadContent, IUpdateDownloadContent_Vtbl, 0x54a2cb2d_9a0c_48b6_8a50_9abb69ee2d02);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateDownloadContent {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateDownloadContent, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContent {
    pub unsafe fn DownloadUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DownloadUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateDownloadContent_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DownloadUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateDownloadContent2, IUpdateDownloadContent2_Vtbl, 0xc97ad11b_f257_420b_9d9f_377f733f6f68);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateDownloadContent2 {
    type Target = IUpdateDownloadContent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateDownloadContent2, windows_core::IUnknown, super::Com::IDispatch, IUpdateDownloadContent);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContent2 {
    pub unsafe fn IsDeltaCompressedContent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDeltaCompressedContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateDownloadContent2_Vtbl {
    pub base__: IUpdateDownloadContent_Vtbl,
    pub IsDeltaCompressedContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateDownloadContentCollection, IUpdateDownloadContentCollection_Vtbl, 0xbc5513c8_b3b8_4bf7_a4d4_361c0d8c88ba);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateDownloadContentCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateDownloadContentCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadContentCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IUpdateDownloadContent> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateDownloadContentCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateDownloadResult, IUpdateDownloadResult_Vtbl, 0xbf99af76_b575_42ad_8aa4_33cbb5477af1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateDownloadResult {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateDownloadResult, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloadResult {
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateDownloadResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateDownloader, IUpdateDownloader_Vtbl, 0x68f1c6f9_7ecc_4666_a464_247fe12496c3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateDownloader {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateDownloader, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateDownloader {
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetClientApplicationID<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientApplicationID)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn IsForced(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsForced)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsForced<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsForced)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn Priority(&self) -> windows_core::Result<DownloadPriority> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Priority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPriority(&self, value: DownloadPriority) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetPriority)(windows_core::Interface::as_raw(self), value).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetUpdates<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUpdateCollection>,
    {
        (windows_core::Interface::vtable(self).SetUpdates)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginDownload<P0, P1, P2>(&self, onprogresschanged: P0, oncompleted: P1, state: P2) -> windows_core::Result<IDownloadJob>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginDownload)(windows_core::Interface::as_raw(self), onprogresschanged.param().abi(), oncompleted.param().abi(), state.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Download(&self) -> windows_core::Result<IDownloadResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndDownload<P0>(&self, value: P0) -> windows_core::Result<IDownloadResult>
    where
        P0: windows_core::Param<IDownloadJob>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndDownload)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateDownloader_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsForced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsForced: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DownloadPriority) -> windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(*mut core::ffi::c_void, DownloadPriority) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetUpdates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginDownload: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Download: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndDownload: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateException, IUpdateException_Vtbl, 0xa376dd5e_09d4_427f_af7c_fed5b6e1c1d6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateException {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateException, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateException {
    pub unsafe fn Message(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Context(&self) -> windows_core::Result<UpdateExceptionContext> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Context)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateException_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Context: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateExceptionContext) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateExceptionCollection, IUpdateExceptionCollection_Vtbl, 0x503626a3_8e14_4729_9355_0fe664bd2321);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateExceptionCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateExceptionCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateExceptionCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IUpdateException> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateExceptionCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateHistoryEntry, IUpdateHistoryEntry_Vtbl, 0xbe56a644_af0e_4e0e_a311_c1d8e695cbff);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateHistoryEntry {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateHistoryEntry, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntry {
    pub unsafe fn Operation(&self) -> windows_core::Result<UpdateOperation> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Operation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Date(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Date)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateIdentity(&self) -> windows_core::Result<IUpdateIdentity> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UpdateIdentity)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UnmappedResultCode(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UnmappedResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ServerSelection(&self) -> windows_core::Result<ServerSelection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServerSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UninstallationSteps(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UninstallationSteps)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn UninstallationNotes(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UninstallationNotes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SupportUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SupportUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateHistoryEntry_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Operation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateOperation) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Date: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateIdentity: usize,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UnmappedResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ServerSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ServerSelection) -> windows_core::HRESULT,
    pub ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub UninstallationSteps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UninstallationSteps: usize,
    pub UninstallationNotes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SupportUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateHistoryEntry2, IUpdateHistoryEntry2_Vtbl, 0xc2bfb780_4539_4132_ab8c_0a8772013ab6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateHistoryEntry2 {
    type Target = IUpdateHistoryEntry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateHistoryEntry2, windows_core::IUnknown, super::Com::IDispatch, IUpdateHistoryEntry);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntry2 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Categories(&self) -> windows_core::Result<ICategoryCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Categories)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateHistoryEntry2_Vtbl {
    pub base__: IUpdateHistoryEntry_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Categories: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Categories: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateHistoryEntryCollection, IUpdateHistoryEntryCollection_Vtbl, 0xa7f04f3c_a290_435b_aadf_a116c3357a5c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateHistoryEntryCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateHistoryEntryCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateHistoryEntryCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IUpdateHistoryEntry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateHistoryEntryCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateIdentity, IUpdateIdentity_Vtbl, 0x46297823_9940_4c09_aed9_cd3ea6d05968);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateIdentity {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateIdentity, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateIdentity {
    pub unsafe fn RevisionNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RevisionNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn UpdateID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UpdateID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateIdentity_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub RevisionNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UpdateID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateInstallationResult, IUpdateInstallationResult_Vtbl, 0xd940f0f8_3cbb_4fd0_993f_471e7f2328ad);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateInstallationResult {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateInstallationResult, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstallationResult {
    pub unsafe fn HResult(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).HResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn RebootRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ResultCode(&self) -> windows_core::Result<OperationResultCode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ResultCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateInstallationResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub HResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ResultCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut OperationResultCode) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateInstaller, IUpdateInstaller_Vtbl, 0x7b929c68_ccdc_4226_96b1_8724600b54c2);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateInstaller {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateInstaller, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller {
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetClientApplicationID<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientApplicationID)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn IsForced(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsForced)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIsForced<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIsForced)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ParentHwnd(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParentHwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetParentHwnd<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
    {
        (windows_core::Interface::vtable(self).SetParentHwnd)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn SetParentWindow<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetParentWindow)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ParentWindow)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Updates(&self) -> windows_core::Result<IUpdateCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Updates)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetUpdates<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IUpdateCollection>,
    {
        (windows_core::Interface::vtable(self).SetUpdates)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginInstall<P0, P1, P2>(&self, onprogresschanged: P0, oncompleted: P1, state: P2) -> windows_core::Result<IInstallationJob>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginInstall)(windows_core::Interface::as_raw(self), onprogresschanged.param().abi(), oncompleted.param().abi(), state.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginUninstall<P0, P1, P2>(&self, onprogresschanged: P0, oncompleted: P1, state: P2) -> windows_core::Result<IInstallationJob>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginUninstall)(windows_core::Interface::as_raw(self), onprogresschanged.param().abi(), oncompleted.param().abi(), state.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndInstall<P0>(&self, value: P0) -> windows_core::Result<IInstallationResult>
    where
        P0: windows_core::Param<IInstallationJob>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndInstall)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndUninstall<P0>(&self, value: P0) -> windows_core::Result<IInstallationResult>
    where
        P0: windows_core::Param<IInstallationJob>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndUninstall)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Install(&self) -> windows_core::Result<IInstallationResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Install)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RunWizard<P0>(&self, dialogtitle: P0) -> windows_core::Result<IInstallationResult>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RunWizard)(windows_core::Interface::as_raw(self), dialogtitle.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsBusy(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsBusy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Uninstall(&self) -> windows_core::Result<IInstallationResult> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Uninstall)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn AllowSourcePrompts(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowSourcePrompts)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowSourcePrompts<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowSourcePrompts)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn RebootRequiredBeforeInstallation(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RebootRequiredBeforeInstallation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateInstaller_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsForced: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIsForced: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ParentHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub SetParentHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub SetParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParentWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Updates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Updates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetUpdates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginInstall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginUninstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginUninstall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndInstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndInstall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndUninstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndUninstall: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Install: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Install: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RunWizard: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RunWizard: usize,
    pub IsBusy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Uninstall: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Uninstall: usize,
    pub AllowSourcePrompts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAllowSourcePrompts: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub RebootRequiredBeforeInstallation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateInstaller2, IUpdateInstaller2_Vtbl, 0x3442d4fe_224d_4cee_98cf_30e0c4d229e6);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateInstaller2 {
    type Target = IUpdateInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateInstaller2, windows_core::IUnknown, super::Com::IDispatch, IUpdateInstaller);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller2 {
    pub unsafe fn ForceQuiet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ForceQuiet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetForceQuiet<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetForceQuiet)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateInstaller2_Vtbl {
    pub base__: IUpdateInstaller_Vtbl,
    pub ForceQuiet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetForceQuiet: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateInstaller3, IUpdateInstaller3_Vtbl, 0x16d11c35_099a_48d0_8338_5fae64047f8e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateInstaller3 {
    type Target = IUpdateInstaller2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateInstaller3, windows_core::IUnknown, super::Com::IDispatch, IUpdateInstaller, IUpdateInstaller2);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller3 {
    pub unsafe fn AttemptCloseAppsIfNecessary(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AttemptCloseAppsIfNecessary)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAttemptCloseAppsIfNecessary<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAttemptCloseAppsIfNecessary)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateInstaller3_Vtbl {
    pub base__: IUpdateInstaller2_Vtbl,
    pub AttemptCloseAppsIfNecessary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAttemptCloseAppsIfNecessary: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateInstaller4, IUpdateInstaller4_Vtbl, 0xef8208ea_2304_492d_9109_23813b0958e1);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateInstaller4 {
    type Target = IUpdateInstaller3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateInstaller4, windows_core::IUnknown, super::Com::IDispatch, IUpdateInstaller, IUpdateInstaller2, IUpdateInstaller3);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateInstaller4 {
    pub unsafe fn Commit(&self, dwflags: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), dwflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateInstaller4_Vtbl {
    pub base__: IUpdateInstaller3_Vtbl,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateLockdown, IUpdateLockdown_Vtbl, 0xa976c28d_75a1_42aa_94ae_8af8b872089a);
impl core::ops::Deref for IUpdateLockdown {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUpdateLockdown, windows_core::IUnknown);
impl IUpdateLockdown {
    pub unsafe fn LockDown(&self, flags: i32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockDown)(windows_core::Interface::as_raw(self), flags).ok()
    }
}
#[repr(C)]
pub struct IUpdateLockdown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LockDown: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateSearcher, IUpdateSearcher_Vtbl, 0x8f45abf1_f9ae_4b95_a933_f0f66e5056ea);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateSearcher {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateSearcher, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher {
    pub unsafe fn CanAutomaticallyUpgradeService(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanAutomaticallyUpgradeService)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetCanAutomaticallyUpgradeService<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetCanAutomaticallyUpgradeService)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetClientApplicationID<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientApplicationID)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn IncludePotentiallySupersededUpdates(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IncludePotentiallySupersededUpdates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIncludePotentiallySupersededUpdates<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIncludePotentiallySupersededUpdates)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ServerSelection(&self) -> windows_core::Result<ServerSelection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServerSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetServerSelection(&self, value: ServerSelection) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetServerSelection)(windows_core::Interface::as_raw(self), value).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BeginSearch<P0, P1, P2>(&self, criteria: P0, oncompleted: P1, state: P2) -> windows_core::Result<ISearchJob>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BeginSearch)(windows_core::Interface::as_raw(self), criteria.param().abi(), oncompleted.param().abi(), state.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EndSearch<P0>(&self, searchjob: P0) -> windows_core::Result<ISearchResult>
    where
        P0: windows_core::Param<ISearchJob>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EndSearch)(windows_core::Interface::as_raw(self), searchjob.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn EscapeString<P0>(&self, unescaped: P0) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EscapeString)(windows_core::Interface::as_raw(self), unescaped.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryHistory(&self, startindex: i32, count: i32) -> windows_core::Result<IUpdateHistoryEntryCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryHistory)(windows_core::Interface::as_raw(self), startindex, count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Search<P0>(&self, criteria: P0) -> windows_core::Result<ISearchResult>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Search)(windows_core::Interface::as_raw(self), criteria.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Online(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Online)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetOnline<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetOnline)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn GetTotalHistoryCount(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetTotalHistoryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetServiceID<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetServiceID)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSearcher_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub CanAutomaticallyUpgradeService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetCanAutomaticallyUpgradeService: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IncludePotentiallySupersededUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIncludePotentiallySupersededUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ServerSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ServerSelection) -> windows_core::HRESULT,
    pub SetServerSelection: unsafe extern "system" fn(*mut core::ffi::c_void, ServerSelection) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BeginSearch: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BeginSearch: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EndSearch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EndSearch: usize,
    pub EscapeString: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryHistory: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryHistory: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Search: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Search: usize,
    pub Online: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetOnline: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub GetTotalHistoryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateSearcher2, IUpdateSearcher2_Vtbl, 0x4cbdcb2d_1589_4beb_bd1c_3e582ff0add0);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateSearcher2 {
    type Target = IUpdateSearcher;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateSearcher2, windows_core::IUnknown, super::Com::IDispatch, IUpdateSearcher);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher2 {
    pub unsafe fn IgnoreDownloadPriority(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IgnoreDownloadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetIgnoreDownloadPriority<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetIgnoreDownloadPriority)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSearcher2_Vtbl {
    pub base__: IUpdateSearcher_Vtbl,
    pub IgnoreDownloadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetIgnoreDownloadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateSearcher3, IUpdateSearcher3_Vtbl, 0x04c6895d_eaf2_4034_97f3_311de9be413a);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateSearcher3 {
    type Target = IUpdateSearcher2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateSearcher3, windows_core::IUnknown, super::Com::IDispatch, IUpdateSearcher, IUpdateSearcher2);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSearcher3 {
    pub unsafe fn SearchScope(&self) -> windows_core::Result<SearchScope> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SearchScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetSearchScope(&self, value: SearchScope) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetSearchScope)(windows_core::Interface::as_raw(self), value).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSearcher3_Vtbl {
    pub base__: IUpdateSearcher2_Vtbl,
    pub SearchScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SearchScope) -> windows_core::HRESULT,
    pub SetSearchScope: unsafe extern "system" fn(*mut core::ffi::c_void, SearchScope) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateService, IUpdateService_Vtbl, 0x76b3b17e_aed6_4da5_85f0_83587f81abe3);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateService {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateService, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateService {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ContentValidationCert(&self) -> windows_core::Result<windows_core::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ContentValidationCert)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ExpirationDate(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ExpirationDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsManaged(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsManaged)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsRegisteredWithAU(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRegisteredWithAU)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IssueDate(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IssueDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OffersWindowsUpdates(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OffersWindowsUpdates)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RedirectUrls(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RedirectUrls)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsScanPackageService(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsScanPackageService)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn CanRegisterWithAU(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CanRegisterWithAU)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ServiceUrl(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceUrl)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetupPrefix(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetupPrefix)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateService_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ContentValidationCert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsManaged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsRegisteredWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IssueDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub OffersWindowsUpdates: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RedirectUrls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RedirectUrls: usize,
    pub ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsScanPackageService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub CanRegisterWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ServiceUrl: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetupPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateService2, IUpdateService2_Vtbl, 0x1518b460_6518_4172_940f_c75883b24ceb);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateService2 {
    type Target = IUpdateService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateService2, windows_core::IUnknown, super::Com::IDispatch, IUpdateService);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateService2 {
    pub unsafe fn IsDefaultAUService(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsDefaultAUService)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateService2_Vtbl {
    pub base__: IUpdateService_Vtbl,
    pub IsDefaultAUService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateServiceCollection, IUpdateServiceCollection_Vtbl, 0x9b0353aa_0e52_44ff_b8b0_1f7fa0437f88);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateServiceCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateServiceCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IUpdateService> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateServiceCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateServiceManager, IUpdateServiceManager_Vtbl, 0x23857e3c_02ba_44a3_9423_b1c900805f37);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateServiceManager {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateServiceManager, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Services(&self) -> windows_core::Result<IUpdateServiceCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Services)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddService<P0, P1>(&self, serviceid: P0, authorizationcabpath: P1) -> windows_core::Result<IUpdateService>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddService)(windows_core::Interface::as_raw(self), serviceid.param().abi(), authorizationcabpath.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RegisterServiceWithAU<P0>(&self, serviceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RegisterServiceWithAU)(windows_core::Interface::as_raw(self), serviceid.param().abi()).ok()
    }
    pub unsafe fn RemoveService<P0>(&self, serviceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveService)(windows_core::Interface::as_raw(self), serviceid.param().abi()).ok()
    }
    pub unsafe fn UnregisterServiceWithAU<P0>(&self, serviceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).UnregisterServiceWithAU)(windows_core::Interface::as_raw(self), serviceid.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddScanPackageService<P0, P1>(&self, servicename: P0, scanfilelocation: P1, flags: i32) -> windows_core::Result<IUpdateService>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddScanPackageService)(windows_core::Interface::as_raw(self), servicename.param().abi(), scanfilelocation.param().abi(), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOption<P0, P1>(&self, optionname: P0, optionvalue: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::VARIANT>,
    {
        (windows_core::Interface::vtable(self).SetOption)(windows_core::Interface::as_raw(self), optionname.param().abi(), optionvalue.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateServiceManager_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Services: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Services: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddService: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddService: usize,
    pub RegisterServiceWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub RemoveService: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub UnregisterServiceWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddScanPackageService: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddScanPackageService: usize,
    pub SetOption: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateServiceManager2, IUpdateServiceManager2_Vtbl, 0x0bb8531d_7e8d_424f_986c_a0b8f60a3e7b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateServiceManager2 {
    type Target = IUpdateServiceManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateServiceManager2, windows_core::IUnknown, super::Com::IDispatch, IUpdateServiceManager);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceManager2 {
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetClientApplicationID<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientApplicationID)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryServiceRegistration<P0>(&self, serviceid: P0) -> windows_core::Result<IUpdateServiceRegistration>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryServiceRegistration)(windows_core::Interface::as_raw(self), serviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddService2<P0, P1>(&self, serviceid: P0, flags: i32, authorizationcabpath: P1) -> windows_core::Result<IUpdateServiceRegistration>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AddService2)(windows_core::Interface::as_raw(self), serviceid.param().abi(), flags, authorizationcabpath.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateServiceManager2_Vtbl {
    pub base__: IUpdateServiceManager_Vtbl,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryServiceRegistration: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryServiceRegistration: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddService2: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddService2: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateServiceRegistration, IUpdateServiceRegistration_Vtbl, 0xdde02280_12b3_4e0b_937b_6747f6acb286);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateServiceRegistration {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateServiceRegistration, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateServiceRegistration {
    pub unsafe fn RegistrationState(&self) -> windows_core::Result<UpdateServiceRegistrationState> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RegistrationState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn ServiceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ServiceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn IsPendingRegistrationWithAU(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPendingRegistrationWithAU)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Service(&self) -> windows_core::Result<IUpdateService2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Service)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateServiceRegistration_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub RegistrationState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UpdateServiceRegistrationState) -> windows_core::HRESULT,
    pub ServiceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub IsPendingRegistrationWithAU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Service: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Service: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateSession, IUpdateSession_Vtbl, 0x816858a4_260d_4260_933a_2585f1abc76b);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateSession {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateSession, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession {
    pub unsafe fn ClientApplicationID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ClientApplicationID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetClientApplicationID<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetClientApplicationID)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WebProxy(&self) -> windows_core::Result<IWebProxy> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WebProxy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWebProxy<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IWebProxy>,
    {
        (windows_core::Interface::vtable(self).SetWebProxy)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateSearcher(&self) -> windows_core::Result<IUpdateSearcher> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateUpdateSearcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateDownloader(&self) -> windows_core::Result<IUpdateDownloader> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateUpdateDownloader)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateInstaller(&self) -> windows_core::Result<IUpdateInstaller> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateUpdateInstaller)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSession_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetClientApplicationID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub WebProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WebProxy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWebProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWebProxy: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateSearcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateSearcher: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateDownloader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateDownloader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateInstaller: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateInstaller: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateSession2, IUpdateSession2_Vtbl, 0x91caf7b0_eb23_49ed_9937_c52d817f46f7);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateSession2 {
    type Target = IUpdateSession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateSession2, windows_core::IUnknown, super::Com::IDispatch, IUpdateSession);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession2 {
    pub unsafe fn UserLocale(&self) -> windows_core::Result<u32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserLocale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetUserLocale(&self, lcid: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetUserLocale)(windows_core::Interface::as_raw(self), lcid).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSession2_Vtbl {
    pub base__: IUpdateSession_Vtbl,
    pub UserLocale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetUserLocale: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IUpdateSession3, IUpdateSession3_Vtbl, 0x918efd1e_b5d8_4c90_8540_aeb9bdc56f9d);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IUpdateSession3 {
    type Target = IUpdateSession2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IUpdateSession3, windows_core::IUnknown, super::Com::IDispatch, IUpdateSession, IUpdateSession2);
#[cfg(feature = "Win32_System_Com")]
impl IUpdateSession3 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateUpdateServiceManager(&self) -> windows_core::Result<IUpdateServiceManager2> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CreateUpdateServiceManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryHistory<P0>(&self, criteria: P0, startindex: i32, count: i32) -> windows_core::Result<IUpdateHistoryEntryCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryHistory)(windows_core::Interface::as_raw(self), criteria.param().abi(), startindex, count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IUpdateSession3_Vtbl {
    pub base__: IUpdateSession2_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateUpdateServiceManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateUpdateServiceManager: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryHistory: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryHistory: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWebProxy, IWebProxy_Vtbl, 0x174c81fe_aecd_4dae_b8a0_2c6318dd86a8);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWebProxy {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWebProxy, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWebProxy {
    pub unsafe fn Address(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Address)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetAddress<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetAddress)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BypassList(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BypassList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBypassList<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStringCollection>,
    {
        (windows_core::Interface::vtable(self).SetBypassList)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn BypassProxyOnLocal(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BypassProxyOnLocal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetBypassProxyOnLocal<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetBypassProxyOnLocal)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn ReadOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).ReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn UserName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetUserName<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetUserName)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn SetPassword<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
    pub unsafe fn PromptForCredentials<P0, P1>(&self, parentwindow: P0, title: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PromptForCredentials)(windows_core::Interface::as_raw(self), parentwindow.param().abi(), title.param().abi()).ok()
    }
    pub unsafe fn PromptForCredentialsFromHwnd<P0, P1>(&self, parentwindow: P0, title: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::HWND>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PromptForCredentialsFromHwnd)(windows_core::Interface::as_raw(self), parentwindow.param().abi(), title.param().abi()).ok()
    }
    pub unsafe fn AutoDetect(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoDetect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAutoDetect<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::VARIANT_BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAutoDetect)(windows_core::Interface::as_raw(self), value.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWebProxy_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Address: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub BypassList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BypassList: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBypassList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBypassList: usize,
    pub BypassProxyOnLocal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetBypassProxyOnLocal: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub ReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PromptForCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PromptForCredentialsFromHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AutoDetect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub SetAutoDetect: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsDriverUpdate, IWindowsDriverUpdate_Vtbl, 0xb383cd1a_5ce9_4504_9f63_764b1236f191);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsDriverUpdate {
    type Target = IUpdate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdate, windows_core::IUnknown, super::Com::IDispatch, IUpdate);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate {
    pub unsafe fn DriverClass(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverClass)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverHardwareID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverManufacturer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DriverModel(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverModel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DriverProvider(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DriverVerDate(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverVerDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DeviceProblemNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceProblemNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DeviceStatus(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdate_Vtbl {
    pub base__: IUpdate_Vtbl,
    pub DriverClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DriverHardwareID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DriverManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DriverModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DriverProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DriverVerDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub DeviceProblemNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsDriverUpdate2, IWindowsDriverUpdate2_Vtbl, 0x615c4269_7a48_43bd_96b7_bf6ca27d6c3e);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsDriverUpdate2 {
    type Target = IWindowsDriverUpdate;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdate2, windows_core::IUnknown, super::Com::IDispatch, IUpdate, IWindowsDriverUpdate);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate2 {
    pub unsafe fn RebootRequired(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).RebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn IsPresent(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsPresent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CveIDs(&self) -> windows_core::Result<IStringCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CveIDs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyToCache<P0>(&self, pfiles: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IStringCollection>,
    {
        (windows_core::Interface::vtable(self).CopyToCache)(windows_core::Interface::as_raw(self), pfiles.param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdate2_Vtbl {
    pub base__: IWindowsDriverUpdate_Vtbl,
    pub RebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    pub IsPresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CveIDs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CveIDs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CopyToCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CopyToCache: usize,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsDriverUpdate3, IWindowsDriverUpdate3_Vtbl, 0x49ebd502_4a96_41bd_9e3e_4c5057f4250c);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsDriverUpdate3 {
    type Target = IWindowsDriverUpdate2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdate3, windows_core::IUnknown, super::Com::IDispatch, IUpdate, IWindowsDriverUpdate, IWindowsDriverUpdate2);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate3 {
    pub unsafe fn BrowseOnly(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).BrowseOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdate3_Vtbl {
    pub base__: IWindowsDriverUpdate2_Vtbl,
    pub BrowseOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsDriverUpdate4, IWindowsDriverUpdate4_Vtbl, 0x004c6a2b_0c19_4c69_9f5c_a269b2560db9);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsDriverUpdate4 {
    type Target = IWindowsDriverUpdate3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdate4, windows_core::IUnknown, super::Com::IDispatch, IUpdate, IWindowsDriverUpdate, IWindowsDriverUpdate2, IWindowsDriverUpdate3);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate4 {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn WindowsDriverUpdateEntries(&self) -> windows_core::Result<IWindowsDriverUpdateEntryCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).WindowsDriverUpdateEntries)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn PerUser(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PerUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdate4_Vtbl {
    pub base__: IWindowsDriverUpdate3_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub WindowsDriverUpdateEntries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    WindowsDriverUpdateEntries: usize,
    pub PerUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsDriverUpdate5, IWindowsDriverUpdate5_Vtbl, 0x70cf5c82_8642_42bb_9dbc_0cfd263c6c4f);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsDriverUpdate5 {
    type Target = IWindowsDriverUpdate4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdate5, windows_core::IUnknown, super::Com::IDispatch, IUpdate, IWindowsDriverUpdate, IWindowsDriverUpdate2, IWindowsDriverUpdate3, IWindowsDriverUpdate4);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdate5 {
    pub unsafe fn AutoSelection(&self) -> windows_core::Result<AutoSelectionMode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AutoDownload(&self) -> windows_core::Result<AutoDownloadMode> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AutoDownload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdate5_Vtbl {
    pub base__: IWindowsDriverUpdate4_Vtbl,
    pub AutoSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoSelectionMode) -> windows_core::HRESULT,
    pub AutoDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutoDownloadMode) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsDriverUpdateEntry, IWindowsDriverUpdateEntry_Vtbl, 0xed8bfe40_a60b_42ea_9652_817dfcfa23ec);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsDriverUpdateEntry {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdateEntry, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdateEntry {
    pub unsafe fn DriverClass(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverClass)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DriverHardwareID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverHardwareID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DriverManufacturer(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverManufacturer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DriverModel(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverModel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DriverProvider(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn DriverVerDate(&self) -> windows_core::Result<f64> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DriverVerDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DeviceProblemNumber(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceProblemNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn DeviceStatus(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).DeviceStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdateEntry_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DriverClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DriverHardwareID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DriverManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DriverModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DriverProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub DriverVerDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub DeviceProblemNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsDriverUpdateEntryCollection, IWindowsDriverUpdateEntryCollection_Vtbl, 0x0d521700_a372_4bef_828b_3d00c10adebd);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsDriverUpdateEntryCollection {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsDriverUpdateEntryCollection, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsDriverUpdateEntryCollection {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IWindowsDriverUpdateEntry> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsDriverUpdateEntryCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::define_interface!(IWindowsUpdateAgentInfo, IWindowsUpdateAgentInfo_Vtbl, 0x85713fa1_7796_4fa2_be3b_e2d6124dd373);
#[cfg(feature = "Win32_System_Com")]
impl core::ops::Deref for IWindowsUpdateAgentInfo {
    type Target = super::Com::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
windows_core::imp::interface_hierarchy!(IWindowsUpdateAgentInfo, windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl IWindowsUpdateAgentInfo {
    pub unsafe fn GetInfo<P0>(&self, varinfoidentifier: P0) -> windows_core::Result<windows_core::VARIANT>
    where
        P0: windows_core::Param<windows_core::VARIANT>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), varinfoidentifier.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct IWindowsUpdateAgentInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::VARIANT>, *mut core::mem::MaybeUninit<windows_core::VARIANT>) -> windows_core::HRESULT,
}
pub const LIBID_WUApiLib: windows_core::GUID = windows_core::GUID::from_u128(0xb596cc9f_56e5_419e_a622_e01bb457431e);
pub const UPDATE_LOCKDOWN_WEBSITE_ACCESS: u32 = 1u32;
pub const WU_E_ALL_UPDATES_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80240022_u32 as _);
pub const WU_E_AUCLIENT_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80243FFF_u32 as _);
pub const WU_E_AU_CALL_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80240055_u32 as _);
pub const WU_E_AU_DETECT_SVCID_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8024A006_u32 as _);
pub const WU_E_AU_LEGACYCLIENTDISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8024A003_u32 as _);
pub const WU_E_AU_NONLEGACYSERVER: windows_core::HRESULT = windows_core::HRESULT(0x8024A002_u32 as _);
pub const WU_E_AU_NOSERVICE: windows_core::HRESULT = windows_core::HRESULT(0x8024A000_u32 as _);
pub const WU_E_AU_NO_REGISTERED_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x8024A005_u32 as _);
pub const WU_E_AU_OOBE_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8024A008_u32 as _);
pub const WU_E_AU_PAUSED: windows_core::HRESULT = windows_core::HRESULT(0x8024A004_u32 as _);
pub const WU_E_AU_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8024AFFF_u32 as _);
pub const WU_E_BAD_FILE_URL: windows_core::HRESULT = windows_core::HRESULT(0x80240046_u32 as _);
pub const WU_E_BAD_XML_HARDWARECAPABILITY: windows_core::HRESULT = windows_core::HRESULT(0x8024B102_u32 as _);
pub const WU_E_BIN_SOURCE_ABSENT: windows_core::HRESULT = windows_core::HRESULT(0x8024002C_u32 as _);
pub const WU_E_CALLBACK_COOKIE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x8024F005_u32 as _);
pub const WU_E_CALL_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x8024000B_u32 as _);
pub const WU_E_CALL_CANCELLED_BY_HIDE: windows_core::HRESULT = windows_core::HRESULT(0x8024005A_u32 as _);
pub const WU_E_CALL_CANCELLED_BY_INTERACTIVE_SEARCH: windows_core::HRESULT = windows_core::HRESULT(0x80240063_u32 as _);
pub const WU_E_CALL_CANCELLED_BY_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x8024005B_u32 as _);
pub const WU_E_CALL_CANCELLED_BY_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x8024002F_u32 as _);
pub const WU_E_COULDNOTCANCEL: windows_core::HRESULT = windows_core::HRESULT(0x8024000A_u32 as _);
pub const WU_E_CYCLE_DETECTED: windows_core::HRESULT = windows_core::HRESULT(0x8024000F_u32 as _);
pub const WU_E_DM_BG_ERROR_TOKEN_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8024600F_u32 as _);
pub const WU_E_DM_BITSTRANSFERERROR: windows_core::HRESULT = windows_core::HRESULT(0x80246009_u32 as _);
pub const WU_E_DM_CONTENTCHANGED: windows_core::HRESULT = windows_core::HRESULT(0x8024600B_u32 as _);
pub const WU_E_DM_DOSVC_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8024601E_u32 as _);
pub const WU_E_DM_DOWNLOADFILEMISSING: windows_core::HRESULT = windows_core::HRESULT(0x80246012_u32 as _);
pub const WU_E_DM_DOWNLOADFILEPATHUNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x80246011_u32 as _);
pub const WU_E_DM_DOWNLOADLIMITEDBYUPDATESIZE: windows_core::HRESULT = windows_core::HRESULT(0x8024600C_u32 as _);
pub const WU_E_DM_DOWNLOADLOCATIONCHANGED: windows_core::HRESULT = windows_core::HRESULT(0x8024600A_u32 as _);
pub const WU_E_DM_DOWNLOADSANDBOXNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80246010_u32 as _);
pub const WU_E_DM_DOWNLOAD_VOLUME_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x8024601B_u32 as _);
pub const WU_E_DM_FAILTOCONNECTTOBITS: windows_core::HRESULT = windows_core::HRESULT(0x80246008_u32 as _);
pub const WU_E_DM_FALLINGBACKTOBITS: windows_core::HRESULT = windows_core::HRESULT(0x8024601A_u32 as _);
pub const WU_E_DM_HARDRESERVEID_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x8024601D_u32 as _);
pub const WU_E_DM_INCORRECTFILEHASH: windows_core::HRESULT = windows_core::HRESULT(0x80246002_u32 as _);
pub const WU_E_DM_NEEDDOWNLOADREQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80246004_u32 as _);
pub const WU_E_DM_NONETWORK: windows_core::HRESULT = windows_core::HRESULT(0x80246005_u32 as _);
pub const WU_E_DM_NOTDOWNLOADED: windows_core::HRESULT = windows_core::HRESULT(0x80246007_u32 as _);
pub const WU_E_DM_READRANGEFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80246014_u32 as _);
pub const WU_E_DM_SANDBOX_HASH_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8024601C_u32 as _);
pub const WU_E_DM_UNAUTHORIZED: windows_core::HRESULT = windows_core::HRESULT(0x8024600E_u32 as _);
pub const WU_E_DM_UNAUTHORIZED_DOMAIN_USER: windows_core::HRESULT = windows_core::HRESULT(0x80246018_u32 as _);
pub const WU_E_DM_UNAUTHORIZED_LOCAL_USER: windows_core::HRESULT = windows_core::HRESULT(0x80246017_u32 as _);
pub const WU_E_DM_UNAUTHORIZED_MSA_USER: windows_core::HRESULT = windows_core::HRESULT(0x80246019_u32 as _);
pub const WU_E_DM_UNAUTHORIZED_NO_USER: windows_core::HRESULT = windows_core::HRESULT(0x80246016_u32 as _);
pub const WU_E_DM_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80246FFF_u32 as _);
pub const WU_E_DM_UNKNOWNALGORITHM: windows_core::HRESULT = windows_core::HRESULT(0x80246003_u32 as _);
pub const WU_E_DM_UPDATEREMOVED: windows_core::HRESULT = windows_core::HRESULT(0x80246013_u32 as _);
pub const WU_E_DM_URLNOTAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80246001_u32 as _);
pub const WU_E_DM_WRONGBITSVERSION: windows_core::HRESULT = windows_core::HRESULT(0x80246006_u32 as _);
pub const WU_E_DOWNLOAD_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80240034_u32 as _);
pub const WU_E_DRV_DEVICE_PROBLEM: windows_core::HRESULT = windows_core::HRESULT(0x8024C008_u32 as _);
pub const WU_E_DRV_MISSING_ATTRIBUTE: windows_core::HRESULT = windows_core::HRESULT(0x8024C005_u32 as _);
pub const WU_E_DRV_NOPROP_OR_LEGACY: windows_core::HRESULT = windows_core::HRESULT(0x8024C002_u32 as _);
pub const WU_E_DRV_NO_METADATA: windows_core::HRESULT = windows_core::HRESULT(0x8024C004_u32 as _);
pub const WU_E_DRV_NO_PRINTER_CONTENT: windows_core::HRESULT = windows_core::HRESULT(0x8024C007_u32 as _);
pub const WU_E_DRV_PRUNED: windows_core::HRESULT = windows_core::HRESULT(0x8024C001_u32 as _);
pub const WU_E_DRV_REG_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8024C003_u32 as _);
pub const WU_E_DRV_SYNC_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8024C006_u32 as _);
pub const WU_E_DRV_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8024CFFF_u32 as _);
pub const WU_E_DS_BADVERSION: windows_core::HRESULT = windows_core::HRESULT(0x80248006_u32 as _);
pub const WU_E_DS_CANNOTREGISTER: windows_core::HRESULT = windows_core::HRESULT(0x80248010_u32 as _);
pub const WU_E_DS_CANTDELETE: windows_core::HRESULT = windows_core::HRESULT(0x8024800B_u32 as _);
pub const WU_E_DS_DATANOTAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x8024801E_u32 as _);
pub const WU_E_DS_DATANOTLOADED: windows_core::HRESULT = windows_core::HRESULT(0x8024801F_u32 as _);
pub const WU_E_DS_DECLINENOTALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80248016_u32 as _);
pub const WU_E_DS_DUPLICATEUPDATEID: windows_core::HRESULT = windows_core::HRESULT(0x80248013_u32 as _);
pub const WU_E_DS_IMPERSONATED: windows_core::HRESULT = windows_core::HRESULT(0x8024801D_u32 as _);
pub const WU_E_DS_INUSE: windows_core::HRESULT = windows_core::HRESULT(0x80248001_u32 as _);
pub const WU_E_DS_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80248002_u32 as _);
pub const WU_E_DS_INVALIDOPERATION: windows_core::HRESULT = windows_core::HRESULT(0x8024801A_u32 as _);
pub const WU_E_DS_INVALIDTABLENAME: windows_core::HRESULT = windows_core::HRESULT(0x80248005_u32 as _);
pub const WU_E_DS_LOCKTIMEOUTEXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x8024800C_u32 as _);
pub const WU_E_DS_MISSINGDATA: windows_core::HRESULT = windows_core::HRESULT(0x80248008_u32 as _);
pub const WU_E_DS_MISSINGREF: windows_core::HRESULT = windows_core::HRESULT(0x80248009_u32 as _);
pub const WU_E_DS_NEEDWINDOWSSERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80248019_u32 as _);
pub const WU_E_DS_NOCATEGORIES: windows_core::HRESULT = windows_core::HRESULT(0x8024800D_u32 as _);
pub const WU_E_DS_NODATA: windows_core::HRESULT = windows_core::HRESULT(0x80248007_u32 as _);
pub const WU_E_DS_NODATA_CCR: windows_core::HRESULT = windows_core::HRESULT(0x80248026_u32 as _);
pub const WU_E_DS_NODATA_COOKIE: windows_core::HRESULT = windows_core::HRESULT(0x80248024_u32 as _);
pub const WU_E_DS_NODATA_DOWNLOADJOB: windows_core::HRESULT = windows_core::HRESULT(0x80248028_u32 as _);
pub const WU_E_DS_NODATA_EULA: windows_core::HRESULT = windows_core::HRESULT(0x80248022_u32 as _);
pub const WU_E_DS_NODATA_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80248027_u32 as _);
pub const WU_E_DS_NODATA_NOSUCHREVISION: windows_core::HRESULT = windows_core::HRESULT(0x80248020_u32 as _);
pub const WU_E_DS_NODATA_NOSUCHUPDATE: windows_core::HRESULT = windows_core::HRESULT(0x80248021_u32 as _);
pub const WU_E_DS_NODATA_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80248023_u32 as _);
pub const WU_E_DS_NODATA_TIMER: windows_core::HRESULT = windows_core::HRESULT(0x80248025_u32 as _);
pub const WU_E_DS_NODATA_TMI: windows_core::HRESULT = windows_core::HRESULT(0x80248029_u32 as _);
pub const WU_E_DS_RESETREQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8024801C_u32 as _);
pub const WU_E_DS_ROWEXISTS: windows_core::HRESULT = windows_core::HRESULT(0x8024800E_u32 as _);
pub const WU_E_DS_SCHEMAMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8024801B_u32 as _);
pub const WU_E_DS_SERVICEEXPIRED: windows_core::HRESULT = windows_core::HRESULT(0x80248015_u32 as _);
pub const WU_E_DS_SESSIONLOCKMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80248018_u32 as _);
pub const WU_E_DS_SHUTDOWN: windows_core::HRESULT = windows_core::HRESULT(0x80248000_u32 as _);
pub const WU_E_DS_STOREFILELOCKED: windows_core::HRESULT = windows_core::HRESULT(0x8024800F_u32 as _);
pub const WU_E_DS_TABLEINCORRECT: windows_core::HRESULT = windows_core::HRESULT(0x80248004_u32 as _);
pub const WU_E_DS_TABLEMISSING: windows_core::HRESULT = windows_core::HRESULT(0x80248003_u32 as _);
pub const WU_E_DS_TABLESESSIONMISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x80248017_u32 as _);
pub const WU_E_DS_UNABLETOSTART: windows_core::HRESULT = windows_core::HRESULT(0x80248011_u32 as _);
pub const WU_E_DS_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80248FFF_u32 as _);
pub const WU_E_DS_UNKNOWNHANDLER: windows_core::HRESULT = windows_core::HRESULT(0x8024800A_u32 as _);
pub const WU_E_DS_UNKNOWNSERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80248014_u32 as _);
pub const WU_E_DUPLICATE_ITEM: windows_core::HRESULT = windows_core::HRESULT(0x80240013_u32 as _);
pub const WU_E_EE_CLUSTER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8024E007_u32 as _);
pub const WU_E_EE_INVALID_ATTRIBUTEDATA: windows_core::HRESULT = windows_core::HRESULT(0x8024E006_u32 as _);
pub const WU_E_EE_INVALID_EXPRESSION: windows_core::HRESULT = windows_core::HRESULT(0x8024E002_u32 as _);
pub const WU_E_EE_INVALID_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8024E004_u32 as _);
pub const WU_E_EE_MISSING_METADATA: windows_core::HRESULT = windows_core::HRESULT(0x8024E003_u32 as _);
pub const WU_E_EE_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8024E005_u32 as _);
pub const WU_E_EE_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8024EFFF_u32 as _);
pub const WU_E_EE_UNKNOWN_EXPRESSION: windows_core::HRESULT = windows_core::HRESULT(0x8024E001_u32 as _);
pub const WU_E_EULAS_DECLINED: windows_core::HRESULT = windows_core::HRESULT(0x80240023_u32 as _);
pub const WU_E_EULA_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80240033_u32 as _);
pub const WU_E_EXCLUSIVE_INSTALL_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x80240019_u32 as _);
pub const WU_E_EXTENDEDERROR_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8024005F_u32 as _);
pub const WU_E_EXTENDEDERROR_NOTSET: windows_core::HRESULT = windows_core::HRESULT(0x8024005E_u32 as _);
pub const WU_E_FILETRUST_DUALSIGNATURE_ECC: windows_core::HRESULT = windows_core::HRESULT(0x8024B302_u32 as _);
pub const WU_E_FILETRUST_DUALSIGNATURE_RSA: windows_core::HRESULT = windows_core::HRESULT(0x8024B301_u32 as _);
pub const WU_E_FILETRUST_SHA2SIGNATURE_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80240061_u32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_DISCOVERY: windows_core::HRESULT = windows_core::HRESULT(0x8024004F_u32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_DOWNLOAD: windows_core::HRESULT = windows_core::HRESULT(0x80240051_u32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_INSTALL: windows_core::HRESULT = windows_core::HRESULT(0x80240052_u32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_OTHER: windows_core::HRESULT = windows_core::HRESULT(0x80240053_u32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_SEARCH: windows_core::HRESULT = windows_core::HRESULT(0x80240050_u32 as _);
pub const WU_E_IDLESHUTDOWN_OPCOUNT_SERVICEREGISTRATION: windows_core::HRESULT = windows_core::HRESULT(0x80240060_u32 as _);
pub const WU_E_INFRASTRUCTUREFILE_INVALID_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x8024004D_u32 as _);
pub const WU_E_INFRASTRUCTUREFILE_REQUIRES_SSL: windows_core::HRESULT = windows_core::HRESULT(0x8024004E_u32 as _);
pub const WU_E_INSTALLATION_RESULTS_INVALID_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80243002_u32 as _);
pub const WU_E_INSTALLATION_RESULTS_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80243003_u32 as _);
pub const WU_E_INSTALLATION_RESULTS_UNKNOWN_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80243001_u32 as _);
pub const WU_E_INSTALL_JOB_NOT_SUSPENDED: windows_core::HRESULT = windows_core::HRESULT(0x80240065_u32 as _);
pub const WU_E_INSTALL_JOB_RESUME_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80240064_u32 as _);
pub const WU_E_INSTALL_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80240016_u32 as _);
pub const WU_E_INSTALL_USERCONTEXT_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80240066_u32 as _);
pub const WU_E_INTERACTIVE_CALL_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80240054_u32 as _);
pub const WU_E_INVALIDINDEX: windows_core::HRESULT = windows_core::HRESULT(0x80240007_u32 as _);
pub const WU_E_INVALID_CRITERIA: windows_core::HRESULT = windows_core::HRESULT(0x80240032_u32 as _);
pub const WU_E_INVALID_EVENT: windows_core::HRESULT = windows_core::HRESULT(0x8024F003_u32 as _);
pub const WU_E_INVALID_EVENT_PAYLOAD: windows_core::HRESULT = windows_core::HRESULT(0x80247003_u32 as _);
pub const WU_E_INVALID_EVENT_PAYLOADSIZE: windows_core::HRESULT = windows_core::HRESULT(0x80247004_u32 as _);
pub const WU_E_INVALID_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80240031_u32 as _);
pub const WU_E_INVALID_INSTALL_REQUESTED: windows_core::HRESULT = windows_core::HRESULT(0x80240014_u32 as _);
pub const WU_E_INVALID_NOTIFICATION_INFO: windows_core::HRESULT = windows_core::HRESULT(0x80240048_u32 as _);
pub const WU_E_INVALID_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x80240036_u32 as _);
pub const WU_E_INVALID_PRODUCT_LICENSE: windows_core::HRESULT = windows_core::HRESULT(0x80240029_u32 as _);
pub const WU_E_INVALID_PROXY_SERVER: windows_core::HRESULT = windows_core::HRESULT(0x80240030_u32 as _);
pub const WU_E_INVALID_RELATIONSHIP: windows_core::HRESULT = windows_core::HRESULT(0x80240011_u32 as _);
pub const WU_E_INVALID_SERIALIZATION_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80240058_u32 as _);
pub const WU_E_INVALID_UPDATE: windows_core::HRESULT = windows_core::HRESULT(0x8024001D_u32 as _);
pub const WU_E_INVALID_UPDATE_TYPE: windows_core::HRESULT = windows_core::HRESULT(0x80240026_u32 as _);
pub const WU_E_INVALID_VOLUMEID: windows_core::HRESULT = windows_core::HRESULT(0x8024005C_u32 as _);
pub const WU_E_INVENTORY_GET_INVENTORY_TYPE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80249002_u32 as _);
pub const WU_E_INVENTORY_PARSEFAILED: windows_core::HRESULT = windows_core::HRESULT(0x80249001_u32 as _);
pub const WU_E_INVENTORY_RESULT_UPLOAD_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80249003_u32 as _);
pub const WU_E_INVENTORY_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80249004_u32 as _);
pub const WU_E_INVENTORY_WMI_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80249005_u32 as _);
pub const WU_E_ITEMNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80240008_u32 as _);
pub const WU_E_LEGACYSERVER: windows_core::HRESULT = windows_core::HRESULT(0x8024002B_u32 as _);
pub const WU_E_LOW_BATTERY: windows_core::HRESULT = windows_core::HRESULT(0x8024004C_u32 as _);
pub const WU_E_MAX_CAPACITY_REACHED: windows_core::HRESULT = windows_core::HRESULT(0x80240002_u32 as _);
pub const WU_E_METADATATRUST_CERTIFICATECHAIN_VERIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x80247150_u32 as _);
pub const WU_E_METADATATRUST_UNTRUSTED_CERTIFICATECHAIN: windows_core::HRESULT = windows_core::HRESULT(0x80247151_u32 as _);
pub const WU_E_METADATA_BAD_FRAGMENTSIGNING_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80247107_u32 as _);
pub const WU_E_METADATA_BAD_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x80247140_u32 as _);
pub const WU_E_METADATA_CERT_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80247180_u32 as _);
pub const WU_E_METADATA_CERT_UNTRUSTED: windows_core::HRESULT = windows_core::HRESULT(0x80247183_u32 as _);
pub const WU_E_METADATA_CONFIG_INVALID_BINARY_ENCODING: windows_core::HRESULT = windows_core::HRESULT(0x80247101_u32 as _);
pub const WU_E_METADATA_FAILURE_PROCESSING_FRAGMENTSIGNING_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80247108_u32 as _);
pub const WU_E_METADATA_FETCH_CONFIG: windows_core::HRESULT = windows_core::HRESULT(0x80247102_u32 as _);
pub const WU_E_METADATA_INTCERT_BAD_TRANSPORT_ENCODING: windows_core::HRESULT = windows_core::HRESULT(0x80247182_u32 as _);
pub const WU_E_METADATA_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0x80247104_u32 as _);
pub const WU_E_METADATA_LEAFCERT_BAD_TRANSPORT_ENCODING: windows_core::HRESULT = windows_core::HRESULT(0x80247181_u32 as _);
pub const WU_E_METADATA_NOOP: windows_core::HRESULT = windows_core::HRESULT(0x80247100_u32 as _);
pub const WU_E_METADATA_NO_VERIFICATION_DATA: windows_core::HRESULT = windows_core::HRESULT(0x80247106_u32 as _);
pub const WU_E_METADATA_SIGNATURE_VERIFY_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80247142_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_ALL_BAD: windows_core::HRESULT = windows_core::HRESULT(0x80247167_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_CACHELOOKUP: windows_core::HRESULT = windows_core::HRESULT(0x80247169_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_CERTCHAIN: windows_core::HRESULT = windows_core::HRESULT(0x80247165_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80247160_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_NODATA: windows_core::HRESULT = windows_core::HRESULT(0x80247168_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_REFRESHONLINE: windows_core::HRESULT = windows_core::HRESULT(0x80247166_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x80247164_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8024717F_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_UNTRUSTED: windows_core::HRESULT = windows_core::HRESULT(0x80247162_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VALIDITYWINDOW_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8024717E_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VALIDITY_WINDOW: windows_core::HRESULT = windows_core::HRESULT(0x80247163_u32 as _);
pub const WU_E_METADATA_TIMESTAMP_TOKEN_VERIFICATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80247161_u32 as _);
pub const WU_E_METADATA_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80247105_u32 as _);
pub const WU_E_METADATA_UNSUPPORTED_HASH_ALG: windows_core::HRESULT = windows_core::HRESULT(0x80247141_u32 as _);
pub const WU_E_METADATA_XML_BASE64CERDATA_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80247128_u32 as _);
pub const WU_E_METADATA_XML_FRAGMENTSIGNING_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80247121_u32 as _);
pub const WU_E_METADATA_XML_INTERMEDIATECERT_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80247126_u32 as _);
pub const WU_E_METADATA_XML_LEAFCERT_ID_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80247127_u32 as _);
pub const WU_E_METADATA_XML_LEAFCERT_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80247125_u32 as _);
pub const WU_E_METADATA_XML_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80247120_u32 as _);
pub const WU_E_METADATA_XML_MODE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80247123_u32 as _);
pub const WU_E_METADATA_XML_MODE_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x80247122_u32 as _);
pub const WU_E_METADATA_XML_VALIDITY_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80247124_u32 as _);
pub const WU_E_MISSING_HANDLER: windows_core::HRESULT = windows_core::HRESULT(0x8024002A_u32 as _);
pub const WU_E_MSI_NOT_CONFIGURED: windows_core::HRESULT = windows_core::HRESULT(0x80241002_u32 as _);
pub const WU_E_MSI_NOT_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x80241005_u32 as _);
pub const WU_E_MSI_WRONG_APP_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x80241004_u32 as _);
pub const WU_E_MSI_WRONG_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80241001_u32 as _);
pub const WU_E_MSP_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80241003_u32 as _);
pub const WU_E_MSP_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80241FFF_u32 as _);
pub const WU_E_NETWORK_COST_EXCEEDS_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x80240059_u32 as _);
pub const WU_E_NON_UI_MODE: windows_core::HRESULT = windows_core::HRESULT(0x80243FFD_u32 as _);
pub const WU_E_NOOP: windows_core::HRESULT = windows_core::HRESULT(0x8024000C_u32 as _);
pub const WU_E_NOT_APPLICABLE: windows_core::HRESULT = windows_core::HRESULT(0x80240017_u32 as _);
pub const WU_E_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80240004_u32 as _);
pub const WU_E_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80240037_u32 as _);
pub const WU_E_NO_CONNECTION: windows_core::HRESULT = windows_core::HRESULT(0x8024001F_u32 as _);
pub const WU_E_NO_INTERACTIVE_USER: windows_core::HRESULT = windows_core::HRESULT(0x80240020_u32 as _);
pub const WU_E_NO_SERVER_CORE_SUPPORT: windows_core::HRESULT = windows_core::HRESULT(0x80240040_u32 as _);
pub const WU_E_NO_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80240001_u32 as _);
pub const WU_E_NO_SUCH_HANDLER_PLUGIN: windows_core::HRESULT = windows_core::HRESULT(0x80240057_u32 as _);
pub const WU_E_NO_UI_SUPPORT: windows_core::HRESULT = windows_core::HRESULT(0x80240043_u32 as _);
pub const WU_E_NO_UPDATE: windows_core::HRESULT = windows_core::HRESULT(0x80240024_u32 as _);
pub const WU_E_NO_USERTOKEN: windows_core::HRESULT = windows_core::HRESULT(0x80240018_u32 as _);
pub const WU_E_OL_INVALID_SCANFILE: windows_core::HRESULT = windows_core::HRESULT(0x80247001_u32 as _);
pub const WU_E_OL_NEWCLIENT_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80247002_u32 as _);
pub const WU_E_OL_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80247FFF_u32 as _);
pub const WU_E_OPERATIONINPROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80240009_u32 as _);
pub const WU_E_ORPHANED_DOWNLOAD_JOB: windows_core::HRESULT = windows_core::HRESULT(0x8024004B_u32 as _);
pub const WU_E_OUTOFRANGE: windows_core::HRESULT = windows_core::HRESULT(0x80240049_u32 as _);
pub const WU_E_PER_MACHINE_UPDATE_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x80240044_u32 as _);
pub const WU_E_POLICY_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x8024001A_u32 as _);
pub const WU_E_PT_ADDRESS_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0x80240448_u32 as _);
pub const WU_E_PT_ADDRESS_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80240449_u32 as _);
pub const WU_E_PT_CATALOG_SYNC_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80240436_u32 as _);
pub const WU_E_PT_CONFIG_PROP_MISSING: windows_core::HRESULT = windows_core::HRESULT(0x8024402A_u32 as _);
pub const WU_E_PT_DATA_BOUNDARY_RESTRICTED: windows_core::HRESULT = windows_core::HRESULT(0x80244100_u32 as _);
pub const WU_E_PT_DOUBLE_INITIALIZATION: windows_core::HRESULT = windows_core::HRESULT(0x80244012_u32 as _);
pub const WU_E_PT_ECP_FAILURE_TO_DECOMPRESS_CAB_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80244034_u32 as _);
pub const WU_E_PT_ECP_FAILURE_TO_EXTRACT_DIGEST: windows_core::HRESULT = windows_core::HRESULT(0x80244033_u32 as _);
pub const WU_E_PT_ECP_FILE_LOCATION_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80244035_u32 as _);
pub const WU_E_PT_ECP_INIT_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80244030_u32 as _);
pub const WU_E_PT_ECP_INVALID_FILE_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80244031_u32 as _);
pub const WU_E_PT_ECP_INVALID_METADATA: windows_core::HRESULT = windows_core::HRESULT(0x80244032_u32 as _);
pub const WU_E_PT_ECP_SUCCEEDED_WITH_ERRORS: windows_core::HRESULT = windows_core::HRESULT(0x8024402F_u32 as _);
pub const WU_E_PT_ENDPOINTURL_NOTAVAIL: windows_core::HRESULT = windows_core::HRESULT(0x8024043F_u32 as _);
pub const WU_E_PT_ENDPOINT_DISCONNECTED: windows_core::HRESULT = windows_core::HRESULT(0x80240440_u32 as _);
pub const WU_E_PT_ENDPOINT_REFRESH_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8024043E_u32 as _);
pub const WU_E_PT_ENDPOINT_UNREACHABLE: windows_core::HRESULT = windows_core::HRESULT(0x80240438_u32 as _);
pub const WU_E_PT_EXCEEDED_MAX_SERVER_TRIPS: windows_core::HRESULT = windows_core::HRESULT(0x80244010_u32 as _);
pub const WU_E_PT_FILE_LOCATIONS_CHANGED: windows_core::HRESULT = windows_core::HRESULT(0x80244025_u32 as _);
pub const WU_E_PT_GENERAL_AAD_CLIENT_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x80244101_u32 as _);
pub const WU_E_PT_HTTP_STATUS_BAD_GATEWAY: windows_core::HRESULT = windows_core::HRESULT(0x80244021_u32 as _);
pub const WU_E_PT_HTTP_STATUS_BAD_METHOD: windows_core::HRESULT = windows_core::HRESULT(0x8024401A_u32 as _);
pub const WU_E_PT_HTTP_STATUS_BAD_REQUEST: windows_core::HRESULT = windows_core::HRESULT(0x80244016_u32 as _);
pub const WU_E_PT_HTTP_STATUS_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0x8024401D_u32 as _);
pub const WU_E_PT_HTTP_STATUS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0x80244017_u32 as _);
pub const WU_E_PT_HTTP_STATUS_FORBIDDEN: windows_core::HRESULT = windows_core::HRESULT(0x80244018_u32 as _);
pub const WU_E_PT_HTTP_STATUS_GATEWAY_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x80244023_u32 as _);
pub const WU_E_PT_HTTP_STATUS_GONE: windows_core::HRESULT = windows_core::HRESULT(0x8024401E_u32 as _);
pub const WU_E_PT_HTTP_STATUS_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80244019_u32 as _);
pub const WU_E_PT_HTTP_STATUS_NOT_MAPPED: windows_core::HRESULT = windows_core::HRESULT(0x8024402B_u32 as _);
pub const WU_E_PT_HTTP_STATUS_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80244020_u32 as _);
pub const WU_E_PT_HTTP_STATUS_PROXY_AUTH_REQ: windows_core::HRESULT = windows_core::HRESULT(0x8024401B_u32 as _);
pub const WU_E_PT_HTTP_STATUS_REQUEST_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0x8024401C_u32 as _);
pub const WU_E_PT_HTTP_STATUS_SERVER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8024401F_u32 as _);
pub const WU_E_PT_HTTP_STATUS_SERVICE_UNAVAIL: windows_core::HRESULT = windows_core::HRESULT(0x80244022_u32 as _);
pub const WU_E_PT_HTTP_STATUS_VERSION_NOT_SUP: windows_core::HRESULT = windows_core::HRESULT(0x80244024_u32 as _);
pub const WU_E_PT_INVALID_COMPUTER_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80244013_u32 as _);
pub const WU_E_PT_INVALID_CONFIG_PROP: windows_core::HRESULT = windows_core::HRESULT(0x80244029_u32 as _);
pub const WU_E_PT_INVALID_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0x80240439_u32 as _);
pub const WU_E_PT_INVALID_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0x80240441_u32 as _);
pub const WU_E_PT_INVALID_URL: windows_core::HRESULT = windows_core::HRESULT(0x8024043A_u32 as _);
pub const WU_E_PT_LOAD_SHEDDING: windows_core::HRESULT = windows_core::HRESULT(0x8024402D_u32 as _);
pub const WU_E_PT_NO_AUTH_COOKIES_CREATED: windows_core::HRESULT = windows_core::HRESULT(0x80244028_u32 as _);
pub const WU_E_PT_NO_AUTH_PLUGINS_REQUESTED: windows_core::HRESULT = windows_core::HRESULT(0x80244027_u32 as _);
pub const WU_E_PT_NO_MANAGED_RECOVER: windows_core::HRESULT = windows_core::HRESULT(0x8024502E_u32 as _);
pub const WU_E_PT_NO_TRANSLATION_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80240447_u32 as _);
pub const WU_E_PT_NUMERIC_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0x80240443_u32 as _);
pub const WU_E_PT_NWS_NOT_LOADED: windows_core::HRESULT = windows_core::HRESULT(0x8024043B_u32 as _);
pub const WU_E_PT_OBJECT_FAULTED: windows_core::HRESULT = windows_core::HRESULT(0x80240442_u32 as _);
pub const WU_E_PT_OPERATION_ABANDONED: windows_core::HRESULT = windows_core::HRESULT(0x80240445_u32 as _);
pub const WU_E_PT_OPERATION_ABORTED: windows_core::HRESULT = windows_core::HRESULT(0x80240444_u32 as _);
pub const WU_E_PT_OTHER: windows_core::HRESULT = windows_core::HRESULT(0x8024044A_u32 as _);
pub const WU_E_PT_PROXY_AUTH_SCHEME_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8024043C_u32 as _);
pub const WU_E_PT_QUOTA_EXCEEDED: windows_core::HRESULT = windows_core::HRESULT(0x80240446_u32 as _);
pub const WU_E_PT_REFRESH_CACHE_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80244015_u32 as _);
pub const WU_E_PT_REGISTRATION_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80244026_u32 as _);
pub const WU_E_PT_SAME_REDIR_ID: windows_core::HRESULT = windows_core::HRESULT(0x8024502D_u32 as _);
pub const WU_E_PT_SECURITY_SYSTEM_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8024044B_u32 as _);
pub const WU_E_PT_SECURITY_VERIFICATION_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80240437_u32 as _);
pub const WU_E_PT_SOAPCLIENT_BASE: windows_core::HRESULT = windows_core::HRESULT(0x80244000_u32 as _);
pub const WU_E_PT_SOAPCLIENT_CONNECT: windows_core::HRESULT = windows_core::HRESULT(0x80244004_u32 as _);
pub const WU_E_PT_SOAPCLIENT_GENERATE: windows_core::HRESULT = windows_core::HRESULT(0x80244003_u32 as _);
pub const WU_E_PT_SOAPCLIENT_INITIALIZE: windows_core::HRESULT = windows_core::HRESULT(0x80244001_u32 as _);
pub const WU_E_PT_SOAPCLIENT_OUTOFMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x80244002_u32 as _);
pub const WU_E_PT_SOAPCLIENT_PARSE: windows_core::HRESULT = windows_core::HRESULT(0x8024400A_u32 as _);
pub const WU_E_PT_SOAPCLIENT_PARSEFAULT: windows_core::HRESULT = windows_core::HRESULT(0x80244008_u32 as _);
pub const WU_E_PT_SOAPCLIENT_READ: windows_core::HRESULT = windows_core::HRESULT(0x80244009_u32 as _);
pub const WU_E_PT_SOAPCLIENT_SEND: windows_core::HRESULT = windows_core::HRESULT(0x80244005_u32 as _);
pub const WU_E_PT_SOAPCLIENT_SERVER: windows_core::HRESULT = windows_core::HRESULT(0x80244006_u32 as _);
pub const WU_E_PT_SOAPCLIENT_SOAPFAULT: windows_core::HRESULT = windows_core::HRESULT(0x80244007_u32 as _);
pub const WU_E_PT_SOAP_CLIENT: windows_core::HRESULT = windows_core::HRESULT(0x8024400D_u32 as _);
pub const WU_E_PT_SOAP_MUST_UNDERSTAND: windows_core::HRESULT = windows_core::HRESULT(0x8024400C_u32 as _);
pub const WU_E_PT_SOAP_SERVER: windows_core::HRESULT = windows_core::HRESULT(0x8024400E_u32 as _);
pub const WU_E_PT_SOAP_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8024400B_u32 as _);
pub const WU_E_PT_SUS_SERVER_NOT_SET: windows_core::HRESULT = windows_core::HRESULT(0x80244011_u32 as _);
pub const WU_E_PT_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80244FFF_u32 as _);
pub const WU_E_PT_WINHTTP_NAME_NOT_RESOLVED: windows_core::HRESULT = windows_core::HRESULT(0x8024402C_u32 as _);
pub const WU_E_PT_WMI_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8024400F_u32 as _);
pub const WU_E_RANGEOVERLAP: windows_core::HRESULT = windows_core::HRESULT(0x80240005_u32 as _);
pub const WU_E_REBOOT_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8024A007_u32 as _);
pub const WU_E_REDIRECTOR_ATTRPROVIDER_EXCEEDED_MAX_NAMEVALUE: windows_core::HRESULT = windows_core::HRESULT(0x80245008_u32 as _);
pub const WU_E_REDIRECTOR_ATTRPROVIDER_INVALID_NAME: windows_core::HRESULT = windows_core::HRESULT(0x80245009_u32 as _);
pub const WU_E_REDIRECTOR_ATTRPROVIDER_INVALID_VALUE: windows_core::HRESULT = windows_core::HRESULT(0x8024500A_u32 as _);
pub const WU_E_REDIRECTOR_CONNECT_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x8024500C_u32 as _);
pub const WU_E_REDIRECTOR_ID_SMALLER: windows_core::HRESULT = windows_core::HRESULT(0x80245003_u32 as _);
pub const WU_E_REDIRECTOR_INVALID_RESPONSE: windows_core::HRESULT = windows_core::HRESULT(0x80245006_u32 as _);
pub const WU_E_REDIRECTOR_LOAD_XML: windows_core::HRESULT = windows_core::HRESULT(0x80245001_u32 as _);
pub const WU_E_REDIRECTOR_ONLINE_DISALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8024500D_u32 as _);
pub const WU_E_REDIRECTOR_SLS_GENERIC_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x8024500B_u32 as _);
pub const WU_E_REDIRECTOR_S_FALSE: windows_core::HRESULT = windows_core::HRESULT(0x80245002_u32 as _);
pub const WU_E_REDIRECTOR_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x802450FF_u32 as _);
pub const WU_E_REDIRECTOR_UNKNOWN_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80245004_u32 as _);
pub const WU_E_REDIRECTOR_UNSUPPORTED_CONTENTTYPE: windows_core::HRESULT = windows_core::HRESULT(0x80245005_u32 as _);
pub const WU_E_REG_VALUE_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x80240012_u32 as _);
pub const WU_E_REPORTER_EVENTCACHECORRUPT: windows_core::HRESULT = windows_core::HRESULT(0x8024F001_u32 as _);
pub const WU_E_REPORTER_EVENTNAMESPACEPARSEFAILED: windows_core::HRESULT = windows_core::HRESULT(0x8024F002_u32 as _);
pub const WU_E_REPORTER_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8024FFFF_u32 as _);
pub const WU_E_REVERT_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80240047_u32 as _);
pub const WU_E_SELFUPDATE_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8024001B_u32 as _);
pub const WU_E_SELFUPDATE_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8024D011_u32 as _);
pub const WU_E_SELFUPDATE_REQUIRED_ADMIN: windows_core::HRESULT = windows_core::HRESULT(0x8024D012_u32 as _);
pub const WU_E_SELFUPDATE_SKIP_ON_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8024D008_u32 as _);
pub const WU_E_SERVER_BUSY: windows_core::HRESULT = windows_core::HRESULT(0x8024F004_u32 as _);
pub const WU_E_SERVICEPROP_NOTAVAIL: windows_core::HRESULT = windows_core::HRESULT(0x8024043D_u32 as _);
pub const WU_E_SERVICE_NOT_REGISTERED: windows_core::HRESULT = windows_core::HRESULT(0x80247005_u32 as _);
pub const WU_E_SERVICE_STOP: windows_core::HRESULT = windows_core::HRESULT(0x8024001E_u32 as _);
pub const WU_E_SETUP_ALREADYRUNNING: windows_core::HRESULT = windows_core::HRESULT(0x8024D00D_u32 as _);
pub const WU_E_SETUP_ALREADY_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8024D003_u32 as _);
pub const WU_E_SETUP_BLOCKED_CONFIGURATION: windows_core::HRESULT = windows_core::HRESULT(0x8024D00B_u32 as _);
pub const WU_E_SETUP_DEFERRABLE_REBOOT_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x8024D014_u32 as _);
pub const WU_E_SETUP_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x8024D016_u32 as _);
pub const WU_E_SETUP_HANDLER_EXEC_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8024D00F_u32 as _);
pub const WU_E_SETUP_INVALID_IDENTDATA: windows_core::HRESULT = windows_core::HRESULT(0x8024D002_u32 as _);
pub const WU_E_SETUP_INVALID_INFDATA: windows_core::HRESULT = windows_core::HRESULT(0x8024D001_u32 as _);
pub const WU_E_SETUP_INVALID_REGISTRY_DATA: windows_core::HRESULT = windows_core::HRESULT(0x8024D010_u32 as _);
pub const WU_E_SETUP_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8024004A_u32 as _);
pub const WU_E_SETUP_NON_DEFERRABLE_REBOOT_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x8024D015_u32 as _);
pub const WU_E_SETUP_NOT_INITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x8024D004_u32 as _);
pub const WU_E_SETUP_REBOOTREQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8024D00E_u32 as _);
pub const WU_E_SETUP_REBOOT_TO_FIX: windows_core::HRESULT = windows_core::HRESULT(0x8024D00C_u32 as _);
pub const WU_E_SETUP_REGISTRATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x8024D007_u32 as _);
pub const WU_E_SETUP_SKIP_UPDATE: windows_core::HRESULT = windows_core::HRESULT(0x8024D009_u32 as _);
pub const WU_E_SETUP_SOURCE_VERSION_MISMATCH: windows_core::HRESULT = windows_core::HRESULT(0x8024D005_u32 as _);
pub const WU_E_SETUP_TARGET_VERSION_GREATER: windows_core::HRESULT = windows_core::HRESULT(0x8024D006_u32 as _);
pub const WU_E_SETUP_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8024DFFF_u32 as _);
pub const WU_E_SETUP_UNSUPPORTED_CONFIGURATION: windows_core::HRESULT = windows_core::HRESULT(0x8024D00A_u32 as _);
pub const WU_E_SETUP_WRONG_SERVER_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x8024D013_u32 as _);
pub const WU_E_SIH_ACTION_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80245105_u32 as _);
pub const WU_E_SIH_ANOTHER_INSTANCE_RUNNING: windows_core::HRESULT = windows_core::HRESULT(0x80245113_u32 as _);
pub const WU_E_SIH_BLOCKED_FOR_PLATFORM: windows_core::HRESULT = windows_core::HRESULT(0x80245112_u32 as _);
pub const WU_E_SIH_DNSRESILIENCY_OFF: windows_core::HRESULT = windows_core::HRESULT(0x80245114_u32 as _);
pub const WU_E_SIH_ENGINE_EXCEPTION: windows_core::HRESULT = windows_core::HRESULT(0x80245111_u32 as _);
pub const WU_E_SIH_INVALIDHASH: windows_core::HRESULT = windows_core::HRESULT(0x80245107_u32 as _);
pub const WU_E_SIH_NONSTDEXCEPTION: windows_core::HRESULT = windows_core::HRESULT(0x80245110_u32 as _);
pub const WU_E_SIH_NO_ENGINE: windows_core::HRESULT = windows_core::HRESULT(0x80245108_u32 as _);
pub const WU_E_SIH_PARSE: windows_core::HRESULT = windows_core::HRESULT(0x8024510B_u32 as _);
pub const WU_E_SIH_POLICY: windows_core::HRESULT = windows_core::HRESULT(0x8024510E_u32 as _);
pub const WU_E_SIH_POST_REBOOT_INSTALL_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x80245109_u32 as _);
pub const WU_E_SIH_POST_REBOOT_NO_CACHED_SLS_RESPONSE: windows_core::HRESULT = windows_core::HRESULT(0x8024510A_u32 as _);
pub const WU_E_SIH_PPL: windows_core::HRESULT = windows_core::HRESULT(0x8024510D_u32 as _);
pub const WU_E_SIH_SECURITY: windows_core::HRESULT = windows_core::HRESULT(0x8024510C_u32 as _);
pub const WU_E_SIH_SLS_PARSE: windows_core::HRESULT = windows_core::HRESULT(0x80245106_u32 as _);
pub const WU_E_SIH_STDEXCEPTION: windows_core::HRESULT = windows_core::HRESULT(0x8024510F_u32 as _);
pub const WU_E_SIH_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x802451FF_u32 as _);
pub const WU_E_SIH_VERIFY_DOWNLOAD_ENGINE: windows_core::HRESULT = windows_core::HRESULT(0x80245101_u32 as _);
pub const WU_E_SIH_VERIFY_DOWNLOAD_PAYLOAD: windows_core::HRESULT = windows_core::HRESULT(0x80245102_u32 as _);
pub const WU_E_SIH_VERIFY_STAGE_ENGINE: windows_core::HRESULT = windows_core::HRESULT(0x80245103_u32 as _);
pub const WU_E_SIH_VERIFY_STAGE_PAYLOAD: windows_core::HRESULT = windows_core::HRESULT(0x80245104_u32 as _);
pub const WU_E_SKIPPED_UPDATE_INSTALLATION: windows_core::HRESULT = windows_core::HRESULT(0x8024B105_u32 as _);
pub const WU_E_SLS_INVALID_REVISION: windows_core::HRESULT = windows_core::HRESULT(0x8024B201_u32 as _);
pub const WU_E_SOURCE_ABSENT: windows_core::HRESULT = windows_core::HRESULT(0x8024002D_u32 as _);
pub const WU_E_SYSPREP_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x80240041_u32 as _);
pub const WU_E_SYSTEM_UNSUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x80240056_u32 as _);
pub const WU_E_TIME_OUT: windows_core::HRESULT = windows_core::HRESULT(0x80240021_u32 as _);
pub const WU_E_TOOMANYRANGES: windows_core::HRESULT = windows_core::HRESULT(0x80240006_u32 as _);
pub const WU_E_TOO_DEEP_RELATION: windows_core::HRESULT = windows_core::HRESULT(0x80240010_u32 as _);
pub const WU_E_TOO_MANY_RESYNC: windows_core::HRESULT = windows_core::HRESULT(0x80240039_u32 as _);
pub const WU_E_TRAYICON_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80243004_u32 as _);
pub const WU_E_TRUST_PROVIDER_UNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x8024B304_u32 as _);
pub const WU_E_TRUST_SUBJECT_NOT_TRUSTED: windows_core::HRESULT = windows_core::HRESULT(0x8024B303_u32 as _);
pub const WU_E_UH_APPX_DEFAULT_PACKAGE_VOLUME_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80242021_u32 as _);
pub const WU_E_UH_APPX_INSTALLED_PACKAGE_VOLUME_UNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80242022_u32 as _);
pub const WU_E_UH_APPX_INVALID_PACKAGE_VOLUME: windows_core::HRESULT = windows_core::HRESULT(0x80242020_u32 as _);
pub const WU_E_UH_APPX_NOT_PRESENT: windows_core::HRESULT = windows_core::HRESULT(0x8024201E_u32 as _);
pub const WU_E_UH_APPX_PACKAGE_FAMILY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80242023_u32 as _);
pub const WU_E_UH_APPX_SYSTEM_VOLUME_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0x80242024_u32 as _);
pub const WU_E_UH_BADCBSPACKAGEID: windows_core::HRESULT = windows_core::HRESULT(0x80242013_u32 as _);
pub const WU_E_UH_BADHANDLERXML: windows_core::HRESULT = windows_core::HRESULT(0x80242009_u32 as _);
pub const WU_E_UH_CALLED_BACK_FAILURE: windows_core::HRESULT = windows_core::HRESULT(0x80242018_u32 as _);
pub const WU_E_UH_CANREQUIREINPUT: windows_core::HRESULT = windows_core::HRESULT(0x8024200A_u32 as _);
pub const WU_E_UH_CUSTOMINSTALLER_INVALID_SIGNATURE: windows_core::HRESULT = windows_core::HRESULT(0x80242019_u32 as _);
pub const WU_E_UH_DECRYPTFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8024201C_u32 as _);
pub const WU_E_UH_DOESNOTSUPPORTACTION: windows_core::HRESULT = windows_core::HRESULT(0x80242004_u32 as _);
pub const WU_E_UH_FALLBACKERROR: windows_core::HRESULT = windows_core::HRESULT(0x80242010_u32 as _);
pub const WU_E_UH_FALLBACKTOSELFCONTAINED: windows_core::HRESULT = windows_core::HRESULT(0x8024200C_u32 as _);
pub const WU_E_UH_HANDLER_DISABLEDUNTILREBOOT: windows_core::HRESULT = windows_core::HRESULT(0x8024201D_u32 as _);
pub const WU_E_UH_INCONSISTENT_FILE_NAMES: windows_core::HRESULT = windows_core::HRESULT(0x8024200F_u32 as _);
pub const WU_E_UH_INSTALLERFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8024200B_u32 as _);
pub const WU_E_UH_INSTALLERHUNG: windows_core::HRESULT = windows_core::HRESULT(0x80242007_u32 as _);
pub const WU_E_UH_INVALIDMETADATA: windows_core::HRESULT = windows_core::HRESULT(0x80242006_u32 as _);
pub const WU_E_UH_INVALID_TARGETSESSION: windows_core::HRESULT = windows_core::HRESULT(0x8024201B_u32 as _);
pub const WU_E_UH_LOCALONLY: windows_core::HRESULT = windows_core::HRESULT(0x80242001_u32 as _);
pub const WU_E_UH_NEEDANOTHERDOWNLOAD: windows_core::HRESULT = windows_core::HRESULT(0x8024200D_u32 as _);
pub const WU_E_UH_NEW_SERVICING_STACK_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x80242017_u32 as _);
pub const WU_E_UH_NOTIFYFAILURE: windows_core::HRESULT = windows_core::HRESULT(0x8024200E_u32 as _);
pub const WU_E_UH_NOTREADYTOCOMMIT: windows_core::HRESULT = windows_core::HRESULT(0x8024201F_u32 as _);
pub const WU_E_UH_OPERATIONCANCELLED: windows_core::HRESULT = windows_core::HRESULT(0x80242008_u32 as _);
pub const WU_E_UH_POSTREBOOTRESULTUNKNOWN: windows_core::HRESULT = windows_core::HRESULT(0x80242015_u32 as _);
pub const WU_E_UH_POSTREBOOTSTILLPENDING: windows_core::HRESULT = windows_core::HRESULT(0x80242014_u32 as _);
pub const WU_E_UH_POSTREBOOTUNEXPECTEDSTATE: windows_core::HRESULT = windows_core::HRESULT(0x80242016_u32 as _);
pub const WU_E_UH_REMOTEALREADYACTIVE: windows_core::HRESULT = windows_core::HRESULT(0x80242003_u32 as _);
pub const WU_E_UH_REMOTEUNAVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0x80242000_u32 as _);
pub const WU_E_UH_TOOMANYDOWNLOADREQUESTS: windows_core::HRESULT = windows_core::HRESULT(0x80242011_u32 as _);
pub const WU_E_UH_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80242FFF_u32 as _);
pub const WU_E_UH_UNEXPECTEDCBSRESPONSE: windows_core::HRESULT = windows_core::HRESULT(0x80242012_u32 as _);
pub const WU_E_UH_UNKNOWNHANDLER: windows_core::HRESULT = windows_core::HRESULT(0x80242002_u32 as _);
pub const WU_E_UH_UNSUPPORTED_INSTALLCONTEXT: windows_core::HRESULT = windows_core::HRESULT(0x8024201A_u32 as _);
pub const WU_E_UH_WRONGHANDLER: windows_core::HRESULT = windows_core::HRESULT(0x80242005_u32 as _);
pub const WU_E_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x80240FFF_u32 as _);
pub const WU_E_UNINSTALL_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x80240028_u32 as _);
pub const WU_E_UNKNOWN_HARDWARECAPABILITY: windows_core::HRESULT = windows_core::HRESULT(0x8024B101_u32 as _);
pub const WU_E_UNKNOWN_ID: windows_core::HRESULT = windows_core::HRESULT(0x80240003_u32 as _);
pub const WU_E_UNKNOWN_SERVICE: windows_core::HRESULT = windows_core::HRESULT(0x80240042_u32 as _);
pub const WU_E_UNRECOGNIZED_VOLUMEID: windows_core::HRESULT = windows_core::HRESULT(0x8024005D_u32 as _);
pub const WU_E_UNSUPPORTED_SEARCHSCOPE: windows_core::HRESULT = windows_core::HRESULT(0x80240045_u32 as _);
pub const WU_E_UPDATE_MERGE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8024B104_u32 as _);
pub const WU_E_UPDATE_NOT_APPROVED: windows_core::HRESULT = windows_core::HRESULT(0x80240062_u32 as _);
pub const WU_E_UPDATE_NOT_PROCESSED: windows_core::HRESULT = windows_core::HRESULT(0x80240035_u32 as _);
pub const WU_E_URL_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0x80240027_u32 as _);
pub const WU_E_USER_ACCESS_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x80240025_u32 as _);
pub const WU_E_WINHTTP_INVALID_FILE: windows_core::HRESULT = windows_core::HRESULT(0x80240038_u32 as _);
pub const WU_E_WMI_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x8024B103_u32 as _);
pub const WU_E_WUCLTUI_UNSUPPORTED_VERSION: windows_core::HRESULT = windows_core::HRESULT(0x80243FFE_u32 as _);
pub const WU_E_WUTASK_CANCELINSTALL_DISALLOWED: windows_core::HRESULT = windows_core::HRESULT(0x8024B005_u32 as _);
pub const WU_E_WUTASK_INPROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x8024B001_u32 as _);
pub const WU_E_WUTASK_NOT_STARTED: windows_core::HRESULT = windows_core::HRESULT(0x8024B003_u32 as _);
pub const WU_E_WUTASK_RETRY: windows_core::HRESULT = windows_core::HRESULT(0x8024B004_u32 as _);
pub const WU_E_WUTASK_STATUS_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8024B002_u32 as _);
pub const WU_E_WU_DISABLED: windows_core::HRESULT = windows_core::HRESULT(0x8024002E_u32 as _);
pub const WU_E_XML_INVALID: windows_core::HRESULT = windows_core::HRESULT(0x8024000E_u32 as _);
pub const WU_E_XML_MISSINGDATA: windows_core::HRESULT = windows_core::HRESULT(0x8024000D_u32 as _);
pub const WU_S_AAD_DEVICE_TICKET_NOT_NEEDED: windows_core::HRESULT = windows_core::HRESULT(0x248002_u32 as _);
pub const WU_S_ALREADY_DOWNLOADED: windows_core::HRESULT = windows_core::HRESULT(0x240008_u32 as _);
pub const WU_S_ALREADY_INSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x240006_u32 as _);
pub const WU_S_ALREADY_REVERTED: windows_core::HRESULT = windows_core::HRESULT(0x24000A_u32 as _);
pub const WU_S_ALREADY_UNINSTALLED: windows_core::HRESULT = windows_core::HRESULT(0x240007_u32 as _);
pub const WU_S_DM_ALREADYDOWNLOADING: windows_core::HRESULT = windows_core::HRESULT(0x246001_u32 as _);
pub const WU_S_MARKED_FOR_DISCONNECT: windows_core::HRESULT = windows_core::HRESULT(0x240004_u32 as _);
pub const WU_S_METADATA_IGNORED_SIGNATURE_VERIFICATION: windows_core::HRESULT = windows_core::HRESULT(0x247102_u32 as _);
pub const WU_S_METADATA_SKIPPED_BY_ENFORCEMENTMODE: windows_core::HRESULT = windows_core::HRESULT(0x247101_u32 as _);
pub const WU_S_REBOOT_REQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x240005_u32 as _);
pub const WU_S_SEARCH_CRITERIA_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0x240010_u32 as _);
pub const WU_S_SEARCH_LOAD_SHEDDING: windows_core::HRESULT = windows_core::HRESULT(0x248001_u32 as _);
pub const WU_S_SELFUPDATE: windows_core::HRESULT = windows_core::HRESULT(0x240002_u32 as _);
pub const WU_S_SERVICE_STOP: windows_core::HRESULT = windows_core::HRESULT(0x240001_u32 as _);
pub const WU_S_SIH_NOOP: windows_core::HRESULT = windows_core::HRESULT(0x245001_u32 as _);
pub const WU_S_SOME_UPDATES_SKIPPED_ON_BATTERY: windows_core::HRESULT = windows_core::HRESULT(0x240009_u32 as _);
pub const WU_S_UH_DOWNLOAD_SIZE_CALCULATED: windows_core::HRESULT = windows_core::HRESULT(0x242016_u32 as _);
pub const WU_S_UH_INSTALLSTILLPENDING: windows_core::HRESULT = windows_core::HRESULT(0x242015_u32 as _);
pub const WU_S_UPDATE_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x240003_u32 as _);
pub const adAlwaysAutoDownload: AutoDownloadMode = AutoDownloadMode(2i32);
pub const adLetWindowsUpdateDecide: AutoDownloadMode = AutoDownloadMode(0i32);
pub const adNeverAutoDownload: AutoDownloadMode = AutoDownloadMode(1i32);
pub const asAlwaysAutoSelect: AutoSelectionMode = AutoSelectionMode(3i32);
pub const asAutoSelectIfDownloaded: AutoSelectionMode = AutoSelectionMode(1i32);
pub const asLetWindowsUpdateDecide: AutoSelectionMode = AutoSelectionMode(0i32);
pub const asNeverAutoSelect: AutoSelectionMode = AutoSelectionMode(2i32);
pub const asfAllowOnlineRegistration: AddServiceFlag = AddServiceFlag(2i32);
pub const asfAllowPendingRegistration: AddServiceFlag = AddServiceFlag(1i32);
pub const asfRegisterServiceWithAU: AddServiceFlag = AddServiceFlag(4i32);
pub const aunlDisabled: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(1i32);
pub const aunlNotConfigured: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(0i32);
pub const aunlNotifyBeforeDownload: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(2i32);
pub const aunlNotifyBeforeInstallation: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(3i32);
pub const aunlScheduledInstallation: AutomaticUpdatesNotificationLevel = AutomaticUpdatesNotificationLevel(4i32);
pub const auptDisableAutomaticUpdates: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(2i32);
pub const auptSetFeaturedUpdatesEnabled: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(4i32);
pub const auptSetIncludeRecommendedUpdates: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(3i32);
pub const auptSetNonAdministratorsElevated: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(5i32);
pub const auptSetNotificationLevel: AutomaticUpdatesPermissionType = AutomaticUpdatesPermissionType(1i32);
pub const ausidEveryDay: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(0i32);
pub const ausidEveryFriday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(6i32);
pub const ausidEveryMonday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(2i32);
pub const ausidEverySaturday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(7i32);
pub const ausidEverySunday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(1i32);
pub const ausidEveryThursday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(5i32);
pub const ausidEveryTuesday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(3i32);
pub const ausidEveryWednesday: AutomaticUpdatesScheduledInstallationDay = AutomaticUpdatesScheduledInstallationDay(4i32);
pub const auutCurrentUser: AutomaticUpdatesUserType = AutomaticUpdatesUserType(1i32);
pub const auutLocalAdministrator: AutomaticUpdatesUserType = AutomaticUpdatesUserType(2i32);
pub const daDetection: DeploymentAction = DeploymentAction(3i32);
pub const daInstallation: DeploymentAction = DeploymentAction(1i32);
pub const daNone: DeploymentAction = DeploymentAction(0i32);
pub const daOptionalInstallation: DeploymentAction = DeploymentAction(4i32);
pub const daUninstallation: DeploymentAction = DeploymentAction(2i32);
pub const dpExtraHigh: DownloadPriority = DownloadPriority(4i32);
pub const dpHigh: DownloadPriority = DownloadPriority(3i32);
pub const dpLow: DownloadPriority = DownloadPriority(1i32);
pub const dpNormal: DownloadPriority = DownloadPriority(2i32);
pub const dphDownloading: DownloadPhase = DownloadPhase(2i32);
pub const dphInitializing: DownloadPhase = DownloadPhase(1i32);
pub const dphVerifying: DownloadPhase = DownloadPhase(3i32);
pub const iiMinor: InstallationImpact = InstallationImpact(1i32);
pub const iiNormal: InstallationImpact = InstallationImpact(0i32);
pub const iiRequiresExclusiveHandling: InstallationImpact = InstallationImpact(2i32);
pub const irbAlwaysRequiresReboot: InstallationRebootBehavior = InstallationRebootBehavior(1i32);
pub const irbCanRequestReboot: InstallationRebootBehavior = InstallationRebootBehavior(2i32);
pub const irbNeverReboots: InstallationRebootBehavior = InstallationRebootBehavior(0i32);
pub const orcAborted: OperationResultCode = OperationResultCode(5i32);
pub const orcFailed: OperationResultCode = OperationResultCode(4i32);
pub const orcInProgress: OperationResultCode = OperationResultCode(1i32);
pub const orcNotStarted: OperationResultCode = OperationResultCode(0i32);
pub const orcSucceeded: OperationResultCode = OperationResultCode(2i32);
pub const orcSucceededWithErrors: OperationResultCode = OperationResultCode(3i32);
pub const searchScopeAllUsers: SearchScope = SearchScope(5i32);
pub const searchScopeCurrentUserOnly: SearchScope = SearchScope(2i32);
pub const searchScopeDefault: SearchScope = SearchScope(0i32);
pub const searchScopeMachineAndAllUsers: SearchScope = SearchScope(4i32);
pub const searchScopeMachineAndCurrentUser: SearchScope = SearchScope(3i32);
pub const searchScopeMachineOnly: SearchScope = SearchScope(1i32);
pub const ssDefault: ServerSelection = ServerSelection(0i32);
pub const ssManagedServer: ServerSelection = ServerSelection(1i32);
pub const ssOthers: ServerSelection = ServerSelection(3i32);
pub const ssWindowsUpdate: ServerSelection = ServerSelection(2i32);
pub const uecGeneral: UpdateExceptionContext = UpdateExceptionContext(1i32);
pub const uecSearchIncomplete: UpdateExceptionContext = UpdateExceptionContext(4i32);
pub const uecWindowsDriver: UpdateExceptionContext = UpdateExceptionContext(2i32);
pub const uecWindowsInstaller: UpdateExceptionContext = UpdateExceptionContext(3i32);
pub const uloForWebsiteAccess: UpdateLockdownOption = UpdateLockdownOption(1i32);
pub const uoInstallation: UpdateOperation = UpdateOperation(1i32);
pub const uoUninstallation: UpdateOperation = UpdateOperation(2i32);
pub const usoNonVolatileService: UpdateServiceOption = UpdateServiceOption(1i32);
pub const usrsNotRegistered: UpdateServiceRegistrationState = UpdateServiceRegistrationState(1i32);
pub const usrsRegistered: UpdateServiceRegistrationState = UpdateServiceRegistrationState(3i32);
pub const usrsRegistrationPending: UpdateServiceRegistrationState = UpdateServiceRegistrationState(2i32);
pub const utDriver: UpdateType = UpdateType(2i32);
pub const utSoftware: UpdateType = UpdateType(1i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AddServiceFlag(pub i32);
impl windows_core::TypeKind for AddServiceFlag {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AddServiceFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AddServiceFlag").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutoDownloadMode(pub i32);
impl windows_core::TypeKind for AutoDownloadMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutoDownloadMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutoDownloadMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutoSelectionMode(pub i32);
impl windows_core::TypeKind for AutoSelectionMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutoSelectionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutoSelectionMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutomaticUpdatesNotificationLevel(pub i32);
impl windows_core::TypeKind for AutomaticUpdatesNotificationLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutomaticUpdatesNotificationLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutomaticUpdatesNotificationLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutomaticUpdatesPermissionType(pub i32);
impl windows_core::TypeKind for AutomaticUpdatesPermissionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutomaticUpdatesPermissionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutomaticUpdatesPermissionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutomaticUpdatesScheduledInstallationDay(pub i32);
impl windows_core::TypeKind for AutomaticUpdatesScheduledInstallationDay {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutomaticUpdatesScheduledInstallationDay {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutomaticUpdatesScheduledInstallationDay").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AutomaticUpdatesUserType(pub i32);
impl windows_core::TypeKind for AutomaticUpdatesUserType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AutomaticUpdatesUserType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AutomaticUpdatesUserType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeploymentAction(pub i32);
impl windows_core::TypeKind for DeploymentAction {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeploymentAction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeploymentAction").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DownloadPhase(pub i32);
impl windows_core::TypeKind for DownloadPhase {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DownloadPhase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DownloadPhase").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DownloadPriority(pub i32);
impl windows_core::TypeKind for DownloadPriority {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DownloadPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DownloadPriority").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InstallationImpact(pub i32);
impl windows_core::TypeKind for InstallationImpact {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InstallationImpact {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InstallationImpact").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InstallationRebootBehavior(pub i32);
impl windows_core::TypeKind for InstallationRebootBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InstallationRebootBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InstallationRebootBehavior").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct OperationResultCode(pub i32);
impl windows_core::TypeKind for OperationResultCode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for OperationResultCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OperationResultCode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SearchScope(pub i32);
impl windows_core::TypeKind for SearchScope {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SearchScope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SearchScope").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ServerSelection(pub i32);
impl windows_core::TypeKind for ServerSelection {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ServerSelection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ServerSelection").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UpdateExceptionContext(pub i32);
impl windows_core::TypeKind for UpdateExceptionContext {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UpdateExceptionContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UpdateExceptionContext").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UpdateLockdownOption(pub i32);
impl windows_core::TypeKind for UpdateLockdownOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UpdateLockdownOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UpdateLockdownOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UpdateOperation(pub i32);
impl windows_core::TypeKind for UpdateOperation {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UpdateOperation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UpdateOperation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UpdateServiceOption(pub i32);
impl windows_core::TypeKind for UpdateServiceOption {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UpdateServiceOption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UpdateServiceOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UpdateServiceRegistrationState(pub i32);
impl windows_core::TypeKind for UpdateServiceRegistrationState {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UpdateServiceRegistrationState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UpdateServiceRegistrationState").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct UpdateType(pub i32);
impl windows_core::TypeKind for UpdateType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for UpdateType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("UpdateType").field(&self.0).finish()
    }
}
pub const AutomaticUpdates: windows_core::GUID = windows_core::GUID::from_u128(0xbfe18e9c_6d87_4450_b37c_e02f0b373803);
pub const InstallationAgent: windows_core::GUID = windows_core::GUID::from_u128(0x317e92fc_1679_46fd_a0b5_f08914dd8623);
pub const StringCollection: windows_core::GUID = windows_core::GUID::from_u128(0x72c97d74_7c3b_40ae_b77d_abdb22eba6fb);
pub const SystemInformation: windows_core::GUID = windows_core::GUID::from_u128(0xc01b9ba0_bea7_41ba_b604_d0a36f469133);
pub const UpdateCollection: windows_core::GUID = windows_core::GUID::from_u128(0x13639463_00db_4646_803d_528026140d88);
pub const UpdateDownloader: windows_core::GUID = windows_core::GUID::from_u128(0x5baf654a_5a07_4264_a255_9ff54c7151e7);
pub const UpdateInstaller: windows_core::GUID = windows_core::GUID::from_u128(0xd2e0fe7f_d23e_48e1_93c0_6fa8cc346474);
pub const UpdateSearcher: windows_core::GUID = windows_core::GUID::from_u128(0xb699e5e8_67ff_4177_88b0_3684a3388bfb);
pub const UpdateServiceManager: windows_core::GUID = windows_core::GUID::from_u128(0xf8d253d9_89a4_4daa_87b6_1168369f0b21);
pub const UpdateSession: windows_core::GUID = windows_core::GUID::from_u128(0x4cb43d7f_7eee_4906_8698_60da1c38f2fe);
pub const WebProxy: windows_core::GUID = windows_core::GUID::from_u128(0x650503cf_9108_4ddc_a2ce_6c2341e1c582);
pub const WindowsUpdateAgentInfo: windows_core::GUID = windows_core::GUID::from_u128(0xc2e88c2f_6f5b_4aaa_894b_55c847ad3a2d);
#[cfg(feature = "implement")]
core::include!("impl.rs");
