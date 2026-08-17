#[cfg(feature = "Graphics_Display_Core")]
pub mod Core;
windows_core::imp::define_interface!(IAdvancedColorInfo, IAdvancedColorInfo_Vtbl, 0x8797dcfb_b229_4081_ae9a_2cc85e34ad6a);
impl windows_core::RuntimeType for IAdvancedColorInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IAdvancedColorInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentAdvancedColorKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AdvancedColorKind) -> windows_core::HRESULT,
    pub RedPrimary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub GreenPrimary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub BluePrimary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub WhitePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub MaxLuminanceInNits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub MinLuminanceInNits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub MaxAverageFullFrameLuminanceInNits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SdrWhiteLevelInNits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub IsHdrMetadataFormatCurrentlySupported: unsafe extern "system" fn(*mut core::ffi::c_void, HdrMetadataFormat, *mut bool) -> windows_core::HRESULT,
    pub IsAdvancedColorKindAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, AdvancedColorKind, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBrightnessOverride, IBrightnessOverride_Vtbl, 0x96c9621a_c143_4392_bedd_4a7e9574c8fd);
impl windows_core::RuntimeType for IBrightnessOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBrightnessOverride_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsOverrideActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub BrightnessLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetBrightnessLevel: unsafe extern "system" fn(*mut core::ffi::c_void, f64, DisplayBrightnessOverrideOptions) -> windows_core::HRESULT,
    pub SetBrightnessScenario: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayBrightnessScenario, DisplayBrightnessOverrideOptions) -> windows_core::HRESULT,
    pub GetLevelForScenario: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayBrightnessScenario, *mut f64) -> windows_core::HRESULT,
    pub StartOverride: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopOverride: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSupportedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsSupportedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub IsOverrideActiveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsOverrideActiveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub BrightnessLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveBrightnessLevelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBrightnessOverrideSettings, IBrightnessOverrideSettings_Vtbl, 0xd112ab2a_7604_4dba_bcf8_4b6f49502cb0);
impl windows_core::RuntimeType for IBrightnessOverrideSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBrightnessOverrideSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub DesiredNits: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBrightnessOverrideSettingsStatics, IBrightnessOverrideSettingsStatics_Vtbl, 0xd487dc90_6f74_440b_b383_5fe96cf00b0f);
impl windows_core::RuntimeType for IBrightnessOverrideSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBrightnessOverrideSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromLevel: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromNits: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromDisplayBrightnessOverrideScenario: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayBrightnessOverrideScenario, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBrightnessOverrideStatics, IBrightnessOverrideStatics_Vtbl, 0x03a7b9ed_e1f1_4a68_a11f_946ad8ce5393);
impl windows_core::RuntimeType for IBrightnessOverrideStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IBrightnessOverrideStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefaultForSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveForSystemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IColorOverrideSettings, IColorOverrideSettings_Vtbl, 0xfbefa134_4a81_4c4d_a5b6_7d1b5c4bd00b);
impl windows_core::RuntimeType for IColorOverrideSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IColorOverrideSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredDisplayColorOverrideScenario: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayColorOverrideScenario) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IColorOverrideSettingsStatics, IColorOverrideSettingsStatics_Vtbl, 0xb068e05f_c41f_4ac9_afab_827ab6248f9a);
impl windows_core::RuntimeType for IColorOverrideSettingsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IColorOverrideSettingsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromDisplayColorOverrideScenario: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayColorOverrideScenario, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayEnhancementOverride, IDisplayEnhancementOverride_Vtbl, 0x429594cf_d97a_4b02_a428_5c4292f7f522);
impl windows_core::RuntimeType for IDisplayEnhancementOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayEnhancementOverride_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ColorOverrideSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColorOverrideSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BrightnessOverrideSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBrightnessOverrideSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanOverride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsOverrideActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetCurrentDisplayEnhancementOverrideCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestOverride: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopOverride: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanOverrideChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCanOverrideChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub IsOverrideActiveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveIsOverrideActiveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub DisplayEnhancementOverrideCapabilitiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDisplayEnhancementOverrideCapabilitiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayEnhancementOverrideCapabilities, IDisplayEnhancementOverrideCapabilities_Vtbl, 0x457060de_ee5a_47b7_9918_1e51e812ccc8);
impl windows_core::RuntimeType for IDisplayEnhancementOverrideCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayEnhancementOverrideCapabilities_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsBrightnessControlSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsBrightnessNitsControlSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedNitRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedNitRanges: usize,
}
windows_core::imp::define_interface!(IDisplayEnhancementOverrideCapabilitiesChangedEventArgs, IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl, 0xdb61e664_15fa_49da_8b77_07dbd2af585d);
impl windows_core::RuntimeType for IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayEnhancementOverrideStatics, IDisplayEnhancementOverrideStatics_Vtbl, 0xcf5b7ec1_9791_4453_b013_29b6f778e519);
impl windows_core::RuntimeType for IDisplayEnhancementOverrideStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayEnhancementOverrideStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayInformation, IDisplayInformation_Vtbl, 0xbed112ae_adc3_4dc9_ae65_851f4d7d4799);
impl windows_core::RuntimeType for IDisplayInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayInformation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayOrientations) -> windows_core::HRESULT,
    pub NativeOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayOrientations) -> windows_core::HRESULT,
    pub OrientationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveOrientationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub ResolutionScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ResolutionScale) -> windows_core::HRESULT,
    pub LogicalDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub RawDpiX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub RawDpiY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub DpiChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDpiChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub StereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub StereoEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveStereoEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetColorProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetColorProfileAsync: usize,
    pub ColorProfileChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveColorProfileChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayInformation2, IDisplayInformation2_Vtbl, 0x4dcd0021_fad1_4b8e_8edf_775887b8bf19);
impl windows_core::RuntimeType for IDisplayInformation2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayInformation2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RawPixelsPerViewPixel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayInformation3, IDisplayInformation3_Vtbl, 0xdb15011d_0f09_4466_8ff3_11de9a3c929a);
impl windows_core::RuntimeType for IDisplayInformation3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayInformation3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DiagonalSizeInInches: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayInformation4, IDisplayInformation4_Vtbl, 0xc972ce2f_1242_46be_b536_e1aafe9e7acf);
impl windows_core::RuntimeType for IDisplayInformation4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayInformation4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ScreenWidthInRawPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ScreenHeightInRawPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayInformation5, IDisplayInformation5_Vtbl, 0x3a5442dc_2cde_4a8d_80d1_21dc5adcc1aa);
impl windows_core::RuntimeType for IDisplayInformation5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayInformation5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAdvancedColorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AdvancedColorInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveAdvancedColorInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDisplayInformationStatics, IDisplayInformationStatics_Vtbl, 0xc6a02a6c_d452_44dc_ba07_96f3c6adf9d1);
impl windows_core::RuntimeType for IDisplayInformationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayInformationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AutoRotationPreferences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayOrientations) -> windows_core::HRESULT,
    pub SetAutoRotationPreferences: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayOrientations) -> windows_core::HRESULT,
    pub DisplayContentsInvalidated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveDisplayContentsInvalidated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(IDisplayPropertiesStatics, IDisplayPropertiesStatics_Vtbl, 0x6937ed8d_30ea_4ded_8271_4553ff02f68a);
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for IDisplayPropertiesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct IDisplayPropertiesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CurrentOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentOrientation: usize,
    #[cfg(feature = "deprecated")]
    pub NativeOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    NativeOrientation: usize,
    #[cfg(feature = "deprecated")]
    pub AutoRotationPreferences: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AutoRotationPreferences: usize,
    #[cfg(feature = "deprecated")]
    pub SetAutoRotationPreferences: unsafe extern "system" fn(*mut core::ffi::c_void, DisplayOrientations) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetAutoRotationPreferences: usize,
    #[cfg(feature = "deprecated")]
    pub OrientationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OrientationChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveOrientationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveOrientationChanged: usize,
    #[cfg(feature = "deprecated")]
    pub ResolutionScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ResolutionScale) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ResolutionScale: usize,
    #[cfg(feature = "deprecated")]
    pub LogicalDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LogicalDpi: usize,
    #[cfg(feature = "deprecated")]
    pub LogicalDpiChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    LogicalDpiChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveLogicalDpiChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveLogicalDpiChanged: usize,
    #[cfg(feature = "deprecated")]
    pub StereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StereoEnabled: usize,
    #[cfg(feature = "deprecated")]
    pub StereoEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    StereoEnabledChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveStereoEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveStereoEnabledChanged: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub GetColorProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    GetColorProfileAsync: usize,
    #[cfg(feature = "deprecated")]
    pub ColorProfileChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ColorProfileChanged: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveColorProfileChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveColorProfileChanged: usize,
    #[cfg(feature = "deprecated")]
    pub DisplayContentsInvalidated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    DisplayContentsInvalidated: usize,
    #[cfg(feature = "deprecated")]
    pub RemoveDisplayContentsInvalidated: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    RemoveDisplayContentsInvalidated: usize,
}
windows_core::imp::define_interface!(IDisplayServices, IDisplayServices_Vtbl, 0x1b54f32b_890d_5747_bd26_fdbdeb0c8a71);
impl windows_core::RuntimeType for IDisplayServices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayServices_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDisplayServicesStatics, IDisplayServicesStatics_Vtbl, 0xdc2096bf_730a_5560_b461_91c13d692e0c);
impl windows_core::RuntimeType for IDisplayServicesStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDisplayServicesStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FindAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut super::DisplayId) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AdvancedColorInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AdvancedColorInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AdvancedColorInfo {
    pub fn CurrentAdvancedColorKind(&self) -> windows_core::Result<AdvancedColorKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentAdvancedColorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RedPrimary(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RedPrimary)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GreenPrimary(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GreenPrimary)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BluePrimary(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BluePrimary)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn WhitePoint(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WhitePoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxLuminanceInNits(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxLuminanceInNits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MinLuminanceInNits(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinLuminanceInNits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn MaxAverageFullFrameLuminanceInNits(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxAverageFullFrameLuminanceInNits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SdrWhiteLevelInNits(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SdrWhiteLevelInNits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsHdrMetadataFormatCurrentlySupported(&self, format: HdrMetadataFormat) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHdrMetadataFormatCurrentlySupported)(windows_core::Interface::as_raw(this), format, &mut result__).map(|| result__)
        }
    }
    pub fn IsAdvancedColorKindAvailable(&self, kind: AdvancedColorKind) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAdvancedColorKindAvailable)(windows_core::Interface::as_raw(this), kind, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for AdvancedColorInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAdvancedColorInfo>();
}
unsafe impl windows_core::Interface for AdvancedColorInfo {
    type Vtable = IAdvancedColorInfo_Vtbl;
    const IID: windows_core::GUID = <IAdvancedColorInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AdvancedColorInfo {
    const NAME: &'static str = "Windows.Graphics.Display.AdvancedColorInfo";
}
unsafe impl Send for AdvancedColorInfo {}
unsafe impl Sync for AdvancedColorInfo {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BrightnessOverride(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BrightnessOverride, windows_core::IUnknown, windows_core::IInspectable);
impl BrightnessOverride {
    pub fn IsSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOverrideActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOverrideActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn BrightnessLevel(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BrightnessLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBrightnessLevel(&self, brightnesslevel: f64, options: DisplayBrightnessOverrideOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBrightnessLevel)(windows_core::Interface::as_raw(this), brightnesslevel, options).ok() }
    }
    pub fn SetBrightnessScenario(&self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBrightnessScenario)(windows_core::Interface::as_raw(this), scenario, options).ok() }
    }
    pub fn GetLevelForScenario(&self, scenario: DisplayBrightnessScenario) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLevelForScenario)(windows_core::Interface::as_raw(this), scenario, &mut result__).map(|| result__)
        }
    }
    pub fn StartOverride(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StartOverride)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StopOverride(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopOverride)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsSupportedChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BrightnessOverride, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupportedChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsSupportedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsSupportedChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsOverrideActiveChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BrightnessOverride, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOverrideActiveChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsOverrideActiveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsOverrideActiveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn BrightnessLevelChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<BrightnessOverride, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BrightnessLevelChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveBrightnessLevelChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveBrightnessLevelChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDefaultForSystem() -> windows_core::Result<BrightnessOverride> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultForSystem)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForCurrentView() -> windows_core::Result<BrightnessOverride> {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn SaveForSystemAsync<P0>(value: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: windows_core::Param<BrightnessOverride>,
    {
        Self::IBrightnessOverrideStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveForSystemAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBrightnessOverrideStatics<R, F: FnOnce(&IBrightnessOverrideStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BrightnessOverride, IBrightnessOverrideStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BrightnessOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBrightnessOverride>();
}
unsafe impl windows_core::Interface for BrightnessOverride {
    type Vtable = IBrightnessOverride_Vtbl;
    const IID: windows_core::GUID = <IBrightnessOverride as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BrightnessOverride {
    const NAME: &'static str = "Windows.Graphics.Display.BrightnessOverride";
}
unsafe impl Send for BrightnessOverride {}
unsafe impl Sync for BrightnessOverride {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct BrightnessOverrideSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BrightnessOverrideSettings, windows_core::IUnknown, windows_core::IInspectable);
impl BrightnessOverrideSettings {
    pub fn DesiredLevel(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredLevel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DesiredNits(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredNits)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateFromLevel(level: f64) -> windows_core::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromLevel)(windows_core::Interface::as_raw(this), level, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromNits(nits: f32) -> windows_core::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromNits)(windows_core::Interface::as_raw(this), nits, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromDisplayBrightnessOverrideScenario(overridescenario: DisplayBrightnessOverrideScenario) -> windows_core::Result<BrightnessOverrideSettings> {
        Self::IBrightnessOverrideSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromDisplayBrightnessOverrideScenario)(windows_core::Interface::as_raw(this), overridescenario, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IBrightnessOverrideSettingsStatics<R, F: FnOnce(&IBrightnessOverrideSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BrightnessOverrideSettings, IBrightnessOverrideSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BrightnessOverrideSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBrightnessOverrideSettings>();
}
unsafe impl windows_core::Interface for BrightnessOverrideSettings {
    type Vtable = IBrightnessOverrideSettings_Vtbl;
    const IID: windows_core::GUID = <IBrightnessOverrideSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BrightnessOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.BrightnessOverrideSettings";
}
unsafe impl Send for BrightnessOverrideSettings {}
unsafe impl Sync for BrightnessOverrideSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ColorOverrideSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ColorOverrideSettings, windows_core::IUnknown, windows_core::IInspectable);
impl ColorOverrideSettings {
    pub fn DesiredDisplayColorOverrideScenario(&self) -> windows_core::Result<DisplayColorOverrideScenario> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredDisplayColorOverrideScenario)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateFromDisplayColorOverrideScenario(overridescenario: DisplayColorOverrideScenario) -> windows_core::Result<ColorOverrideSettings> {
        Self::IColorOverrideSettingsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromDisplayColorOverrideScenario)(windows_core::Interface::as_raw(this), overridescenario, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IColorOverrideSettingsStatics<R, F: FnOnce(&IColorOverrideSettingsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ColorOverrideSettings, IColorOverrideSettingsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ColorOverrideSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IColorOverrideSettings>();
}
unsafe impl windows_core::Interface for ColorOverrideSettings {
    type Vtable = IColorOverrideSettings_Vtbl;
    const IID: windows_core::GUID = <IColorOverrideSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ColorOverrideSettings {
    const NAME: &'static str = "Windows.Graphics.Display.ColorOverrideSettings";
}
unsafe impl Send for ColorOverrideSettings {}
unsafe impl Sync for ColorOverrideSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DisplayEnhancementOverride(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayEnhancementOverride, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayEnhancementOverride {
    pub fn ColorOverrideSettings(&self) -> windows_core::Result<ColorOverrideSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorOverrideSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetColorOverrideSettings<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ColorOverrideSettings>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColorOverrideSettings)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BrightnessOverrideSettings(&self) -> windows_core::Result<BrightnessOverrideSettings> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BrightnessOverrideSettings)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBrightnessOverrideSettings<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BrightnessOverrideSettings>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBrightnessOverrideSettings)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CanOverride(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanOverride)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsOverrideActive(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOverrideActive)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetCurrentDisplayEnhancementOverrideCapabilities(&self) -> windows_core::Result<DisplayEnhancementOverrideCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentDisplayEnhancementOverrideCapabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestOverride(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RequestOverride)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StopOverride(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopOverride)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CanOverrideChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanOverrideChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCanOverrideChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCanOverrideChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsOverrideActiveChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsOverrideActiveChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveIsOverrideActiveChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveIsOverrideActiveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DisplayEnhancementOverrideCapabilitiesChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayEnhancementOverrideCapabilitiesChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDisplayEnhancementOverrideCapabilitiesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDisplayEnhancementOverrideCapabilitiesChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<DisplayEnhancementOverride> {
        Self::IDisplayEnhancementOverrideStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IDisplayEnhancementOverrideStatics<R, F: FnOnce(&IDisplayEnhancementOverrideStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayEnhancementOverride, IDisplayEnhancementOverrideStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DisplayEnhancementOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayEnhancementOverride>();
}
unsafe impl windows_core::Interface for DisplayEnhancementOverride {
    type Vtable = IDisplayEnhancementOverride_Vtbl;
    const IID: windows_core::GUID = <IDisplayEnhancementOverride as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayEnhancementOverride {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverride";
}
unsafe impl Send for DisplayEnhancementOverride {}
unsafe impl Sync for DisplayEnhancementOverride {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DisplayEnhancementOverrideCapabilities(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayEnhancementOverrideCapabilities, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayEnhancementOverrideCapabilities {
    pub fn IsBrightnessControlSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBrightnessControlSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsBrightnessNitsControlSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsBrightnessNitsControlSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedNitRanges(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<NitRange>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSupportedNitRanges)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayEnhancementOverrideCapabilities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayEnhancementOverrideCapabilities>();
}
unsafe impl windows_core::Interface for DisplayEnhancementOverrideCapabilities {
    type Vtable = IDisplayEnhancementOverrideCapabilities_Vtbl;
    const IID: windows_core::GUID = <IDisplayEnhancementOverrideCapabilities as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayEnhancementOverrideCapabilities {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverrideCapabilities";
}
unsafe impl Send for DisplayEnhancementOverrideCapabilities {}
unsafe impl Sync for DisplayEnhancementOverrideCapabilities {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DisplayEnhancementOverrideCapabilitiesChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayEnhancementOverrideCapabilitiesChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    pub fn Capabilities(&self) -> windows_core::Result<DisplayEnhancementOverrideCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capabilities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayEnhancementOverrideCapabilitiesChangedEventArgs>();
}
unsafe impl windows_core::Interface for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    type Vtable = IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IDisplayEnhancementOverrideCapabilitiesChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayEnhancementOverrideCapabilitiesChangedEventArgs";
}
unsafe impl Send for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
unsafe impl Sync for DisplayEnhancementOverrideCapabilitiesChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DisplayInformation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayInformation, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayInformation {
    pub fn CurrentOrientation(&self) -> windows_core::Result<DisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentOrientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NativeOrientation(&self) -> windows_core::Result<DisplayOrientations> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NativeOrientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OrientationChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DisplayInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientationChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOrientationChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOrientationChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn ResolutionScale(&self) -> windows_core::Result<ResolutionScale> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolutionScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LogicalDpi(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogicalDpi)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RawDpiX(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawDpiX)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn RawDpiY(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawDpiY)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DpiChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DisplayInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DpiChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDpiChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveDpiChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn StereoEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StereoEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StereoEnabledChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DisplayInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StereoEnabledChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveStereoEnabledChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveStereoEnabledChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetColorProfileAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetColorProfileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ColorProfileChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DisplayInformation, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorProfileChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveColorProfileChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveColorProfileChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RawPixelsPerViewPixel(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IDisplayInformation2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawPixelsPerViewPixel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DiagonalSizeInInches(&self) -> windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &windows_core::Interface::cast::<IDisplayInformation3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DiagonalSizeInInches)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ScreenWidthInRawPixels(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IDisplayInformation4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScreenWidthInRawPixels)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ScreenHeightInRawPixels(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IDisplayInformation4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ScreenHeightInRawPixels)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetAdvancedColorInfo(&self) -> windows_core::Result<AdvancedColorInfo> {
        let this = &windows_core::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAdvancedColorInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AdvancedColorInfoChanged<P0>(&self, handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DisplayInformation, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvancedColorInfoChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAdvancedColorInfoChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDisplayInformation5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAdvancedColorInfoChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<DisplayInformation> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AutoRotationPreferences() -> windows_core::Result<DisplayOrientations> {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoRotationPreferences)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetAutoRotationPreferences(value: DisplayOrientations) -> windows_core::Result<()> {
        Self::IDisplayInformationStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetAutoRotationPreferences)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn DisplayContentsInvalidated<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<DisplayInformation, windows_core::IInspectable>>,
    {
        Self::IDisplayInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayContentsInvalidated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RemoveDisplayContentsInvalidated(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IDisplayInformationStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveDisplayContentsInvalidated)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IDisplayInformationStatics<R, F: FnOnce(&IDisplayInformationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayInformation, IDisplayInformationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DisplayInformation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayInformation>();
}
unsafe impl windows_core::Interface for DisplayInformation {
    type Vtable = IDisplayInformation_Vtbl;
    const IID: windows_core::GUID = <IDisplayInformation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayInformation";
}
unsafe impl Send for DisplayInformation {}
unsafe impl Sync for DisplayInformation {}
#[cfg(feature = "deprecated")]
pub struct DisplayProperties;
#[cfg(feature = "deprecated")]
impl DisplayProperties {
    #[cfg(feature = "deprecated")]
    pub fn CurrentOrientation() -> windows_core::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentOrientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn NativeOrientation() -> windows_core::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NativeOrientation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn AutoRotationPreferences() -> windows_core::Result<DisplayOrientations> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoRotationPreferences)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn SetAutoRotationPreferences(value: DisplayOrientations) -> windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetAutoRotationPreferences)(windows_core::Interface::as_raw(this), value).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn OrientationChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<DisplayPropertiesEventHandler>,
    {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OrientationChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveOrientationChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveOrientationChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn ResolutionScale() -> windows_core::Result<ResolutionScale> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolutionScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn LogicalDpi() -> windows_core::Result<f32> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogicalDpi)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn LogicalDpiChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<DisplayPropertiesEventHandler>,
    {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LogicalDpiChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveLogicalDpiChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveLogicalDpiChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn StereoEnabled() -> windows_core::Result<bool> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StereoEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn StereoEnabledChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<DisplayPropertiesEventHandler>,
    {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StereoEnabledChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveStereoEnabledChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveStereoEnabledChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn GetColorProfileAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetColorProfileAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn ColorProfileChanged<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<DisplayPropertiesEventHandler>,
    {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorProfileChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveColorProfileChanged(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveColorProfileChanged)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[cfg(feature = "deprecated")]
    pub fn DisplayContentsInvalidated<P0>(handler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<DisplayPropertiesEventHandler>,
    {
        Self::IDisplayPropertiesStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayContentsInvalidated)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        })
    }
    #[cfg(feature = "deprecated")]
    pub fn RemoveDisplayContentsInvalidated(token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        Self::IDisplayPropertiesStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveDisplayContentsInvalidated)(windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IDisplayPropertiesStatics<R, F: FnOnce(&IDisplayPropertiesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayProperties, IDisplayPropertiesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeName for DisplayProperties {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayProperties";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DisplayServices(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DisplayServices, windows_core::IUnknown, windows_core::IInspectable);
impl DisplayServices {
    pub fn FindAll() -> windows_core::Result<windows_core::Array<super::DisplayId>> {
        Self::IDisplayServicesStatics(|this| unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).FindAll)(windows_core::Interface::as_raw(this), windows_core::Array::<super::DisplayId>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        })
    }
    #[doc(hidden)]
    pub fn IDisplayServicesStatics<R, F: FnOnce(&IDisplayServicesStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DisplayServices, IDisplayServicesStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DisplayServices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDisplayServices>();
}
unsafe impl windows_core::Interface for DisplayServices {
    type Vtable = IDisplayServices_Vtbl;
    const IID: windows_core::GUID = <IDisplayServices as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DisplayServices {
    const NAME: &'static str = "Windows.Graphics.Display.DisplayServices";
}
unsafe impl Send for DisplayServices {}
unsafe impl Sync for DisplayServices {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AdvancedColorKind(pub i32);
impl AdvancedColorKind {
    pub const StandardDynamicRange: Self = Self(0i32);
    pub const WideColorGamut: Self = Self(1i32);
    pub const HighDynamicRange: Self = Self(2i32);
}
impl windows_core::TypeKind for AdvancedColorKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AdvancedColorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AdvancedColorKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for AdvancedColorKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.AdvancedColorKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayBrightnessOverrideOptions(pub u32);
impl DisplayBrightnessOverrideOptions {
    pub const None: Self = Self(0u32);
    pub const UseDimmedPolicyWhenBatteryIsLow: Self = Self(1u32);
}
impl windows_core::TypeKind for DisplayBrightnessOverrideOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayBrightnessOverrideOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayBrightnessOverrideOptions").field(&self.0).finish()
    }
}
impl DisplayBrightnessOverrideOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DisplayBrightnessOverrideOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DisplayBrightnessOverrideOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DisplayBrightnessOverrideOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DisplayBrightnessOverrideOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessOverrideOptions;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayBrightnessOverrideScenario(pub i32);
impl DisplayBrightnessOverrideScenario {
    pub const IdleBrightness: Self = Self(0i32);
    pub const BarcodeReadingBrightness: Self = Self(1i32);
    pub const FullBrightness: Self = Self(2i32);
}
impl windows_core::TypeKind for DisplayBrightnessOverrideScenario {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayBrightnessOverrideScenario {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayBrightnessOverrideScenario").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayBrightnessOverrideScenario {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessOverrideScenario;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayBrightnessScenario(pub i32);
impl DisplayBrightnessScenario {
    pub const DefaultBrightness: Self = Self(0i32);
    pub const IdleBrightness: Self = Self(1i32);
    pub const BarcodeReadingBrightness: Self = Self(2i32);
    pub const FullBrightness: Self = Self(3i32);
}
impl windows_core::TypeKind for DisplayBrightnessScenario {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayBrightnessScenario {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayBrightnessScenario").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayBrightnessScenario {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayBrightnessScenario;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayColorOverrideScenario(pub i32);
impl DisplayColorOverrideScenario {
    pub const Accurate: Self = Self(0i32);
}
impl windows_core::TypeKind for DisplayColorOverrideScenario {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayColorOverrideScenario {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayColorOverrideScenario").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DisplayColorOverrideScenario {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayColorOverrideScenario;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DisplayOrientations(pub u32);
impl DisplayOrientations {
    pub const None: Self = Self(0u32);
    pub const Landscape: Self = Self(1u32);
    pub const Portrait: Self = Self(2u32);
    pub const LandscapeFlipped: Self = Self(4u32);
    pub const PortraitFlipped: Self = Self(8u32);
}
impl windows_core::TypeKind for DisplayOrientations {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DisplayOrientations {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DisplayOrientations").field(&self.0).finish()
    }
}
impl DisplayOrientations {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DisplayOrientations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DisplayOrientations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DisplayOrientations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for DisplayOrientations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for DisplayOrientations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for DisplayOrientations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.DisplayOrientations;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HdrMetadataFormat(pub i32);
impl HdrMetadataFormat {
    pub const Hdr10: Self = Self(0i32);
    pub const Hdr10Plus: Self = Self(1i32);
}
impl windows_core::TypeKind for HdrMetadataFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HdrMetadataFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HdrMetadataFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for HdrMetadataFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.HdrMetadataFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ResolutionScale(pub i32);
impl ResolutionScale {
    pub const Invalid: Self = Self(0i32);
    pub const Scale100Percent: Self = Self(100i32);
    pub const Scale120Percent: Self = Self(120i32);
    pub const Scale125Percent: Self = Self(125i32);
    pub const Scale140Percent: Self = Self(140i32);
    pub const Scale150Percent: Self = Self(150i32);
    pub const Scale160Percent: Self = Self(160i32);
    pub const Scale175Percent: Self = Self(175i32);
    pub const Scale180Percent: Self = Self(180i32);
    pub const Scale200Percent: Self = Self(200i32);
    pub const Scale225Percent: Self = Self(225i32);
    pub const Scale250Percent: Self = Self(250i32);
    pub const Scale300Percent: Self = Self(300i32);
    pub const Scale350Percent: Self = Self(350i32);
    pub const Scale400Percent: Self = Self(400i32);
    pub const Scale450Percent: Self = Self(450i32);
    pub const Scale500Percent: Self = Self(500i32);
}
impl windows_core::TypeKind for ResolutionScale {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ResolutionScale {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ResolutionScale").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ResolutionScale {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.ResolutionScale;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NitRange {
    pub MinNits: f32,
    pub MaxNits: f32,
    pub StepSizeNits: f32,
}
impl windows_core::TypeKind for NitRange {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for NitRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Display.NitRange;f4;f4;f4)");
}
impl Default for NitRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "deprecated")]
windows_core::imp::define_interface!(DisplayPropertiesEventHandler, DisplayPropertiesEventHandler_Vtbl, 0xdbdd8b01_f1a1_46d1_9ee3_543bcc995980);
#[cfg(feature = "deprecated")]
impl DisplayPropertiesEventHandler {
    pub fn new<F: FnMut(Option<&windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = DisplayPropertiesEventHandlerBox::<F> { vtable: &DisplayPropertiesEventHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    #[cfg(feature = "deprecated")]
    pub fn Invoke<P0>(&self, sender: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
#[repr(C)]
struct DisplayPropertiesEventHandlerBox<F: FnMut(Option<&windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const DisplayPropertiesEventHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
#[cfg(feature = "deprecated")]
impl<F: FnMut(Option<&windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static> DisplayPropertiesEventHandlerBox<F> {
    const VTABLE: DisplayPropertiesEventHandler_Vtbl = DisplayPropertiesEventHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <DisplayPropertiesEventHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
        if (*interface).is_null() {
            windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&sender)).into()
    }
}
#[cfg(feature = "deprecated")]
impl windows_core::RuntimeType for DisplayPropertiesEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "deprecated")]
#[repr(C)]
pub struct DisplayPropertiesEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Invoke: usize,
}
