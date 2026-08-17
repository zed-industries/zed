windows_core::imp::define_interface!(IApiInformationStatics, IApiInformationStatics_Vtbl, 0x997439fe_f681_4a11_b416_c13a47e8ba36);
impl windows_core::RuntimeType for IApiInformationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IApiInformationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsTypePresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsMethodPresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsMethodPresentWithArity: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, u32, *mut bool) -> windows_core::HRESULT,
    pub IsEventPresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsPropertyPresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsReadOnlyPropertyPresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsWriteablePropertyPresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsEnumNamedValuePresent: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut bool) -> windows_core::HRESULT,
    pub IsApiContractPresentByMajor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u16, *mut bool) -> windows_core::HRESULT,
    pub IsApiContractPresentByMajorAndMinor: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u16, u16, *mut bool) -> windows_core::HRESULT,
}
pub struct ApiInformation;
impl ApiInformation {
    pub fn IsTypePresent(typename: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTypePresent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(typename), &mut result__).map(|| result__)
        })
    }
    pub fn IsMethodPresent(typename: &windows_core::HSTRING, methodname: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMethodPresent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(typename), core::mem::transmute_copy(methodname), &mut result__).map(|| result__)
        })
    }
    pub fn IsMethodPresentWithArity(typename: &windows_core::HSTRING, methodname: &windows_core::HSTRING, inputparametercount: u32) -> windows_core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsMethodPresentWithArity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(typename), core::mem::transmute_copy(methodname), inputparametercount, &mut result__).map(|| result__)
        })
    }
    pub fn IsEventPresent(typename: &windows_core::HSTRING, eventname: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEventPresent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(typename), core::mem::transmute_copy(eventname), &mut result__).map(|| result__)
        })
    }
    pub fn IsPropertyPresent(typename: &windows_core::HSTRING, propertyname: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPropertyPresent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(typename), core::mem::transmute_copy(propertyname), &mut result__).map(|| result__)
        })
    }
    pub fn IsReadOnlyPropertyPresent(typename: &windows_core::HSTRING, propertyname: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReadOnlyPropertyPresent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(typename), core::mem::transmute_copy(propertyname), &mut result__).map(|| result__)
        })
    }
    pub fn IsWriteablePropertyPresent(typename: &windows_core::HSTRING, propertyname: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsWriteablePropertyPresent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(typename), core::mem::transmute_copy(propertyname), &mut result__).map(|| result__)
        })
    }
    pub fn IsEnumNamedValuePresent(enumtypename: &windows_core::HSTRING, valuename: &windows_core::HSTRING) -> windows_core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnumNamedValuePresent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(enumtypename), core::mem::transmute_copy(valuename), &mut result__).map(|| result__)
        })
    }
    pub fn IsApiContractPresentByMajor(contractname: &windows_core::HSTRING, majorversion: u16) -> windows_core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsApiContractPresentByMajor)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contractname), majorversion, &mut result__).map(|| result__)
        })
    }
    pub fn IsApiContractPresentByMajorAndMinor(contractname: &windows_core::HSTRING, majorversion: u16, minorversion: u16) -> windows_core::Result<bool> {
        Self::IApiInformationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsApiContractPresentByMajorAndMinor)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(contractname), majorversion, minorversion, &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IApiInformationStatics<R, F: FnOnce(&IApiInformationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ApiInformation, IApiInformationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for ApiInformation {
    const NAME: &'static str = "Windows.Foundation.Metadata.ApiInformation";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct AttributeTargets(pub u32);
impl AttributeTargets {
    pub const All: Self = Self(4294967295u32);
    pub const Delegate: Self = Self(1u32);
    pub const Enum: Self = Self(2u32);
    pub const Event: Self = Self(4u32);
    pub const Field: Self = Self(8u32);
    pub const Interface: Self = Self(16u32);
    pub const Method: Self = Self(64u32);
    pub const Parameter: Self = Self(128u32);
    pub const Property: Self = Self(256u32);
    pub const RuntimeClass: Self = Self(512u32);
    pub const Struct: Self = Self(1024u32);
    pub const InterfaceImpl: Self = Self(2048u32);
    pub const ApiContract: Self = Self(8192u32);
}
impl windows_core::TypeKind for AttributeTargets {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for AttributeTargets {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("AttributeTargets").field(&self.0).finish()
    }
}
impl AttributeTargets {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for AttributeTargets {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for AttributeTargets {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for AttributeTargets {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for AttributeTargets {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for AttributeTargets {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl windows_core::RuntimeType for AttributeTargets {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.AttributeTargets;u4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CompositionType(pub i32);
impl CompositionType {
    pub const Protected: Self = Self(1i32);
    pub const Public: Self = Self(2i32);
}
impl windows_core::TypeKind for CompositionType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CompositionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CompositionType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for CompositionType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.CompositionType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DeprecationType(pub i32);
impl DeprecationType {
    pub const Deprecate: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
}
impl windows_core::TypeKind for DeprecationType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DeprecationType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DeprecationType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for DeprecationType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.DeprecationType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FeatureStage(pub i32);
impl FeatureStage {
    pub const AlwaysDisabled: Self = Self(0i32);
    pub const DisabledByDefault: Self = Self(1i32);
    pub const EnabledByDefault: Self = Self(2i32);
    pub const AlwaysEnabled: Self = Self(3i32);
}
impl windows_core::TypeKind for FeatureStage {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FeatureStage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FeatureStage").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for FeatureStage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.FeatureStage;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GCPressureAmount(pub i32);
impl GCPressureAmount {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl windows_core::TypeKind for GCPressureAmount {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GCPressureAmount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GCPressureAmount").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for GCPressureAmount {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.GCPressureAmount;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MarshalingType(pub i32);
impl MarshalingType {
    pub const None: Self = Self(1i32);
    pub const Agile: Self = Self(2i32);
    pub const Standard: Self = Self(3i32);
    pub const InvalidMarshaling: Self = Self(0i32);
}
impl windows_core::TypeKind for MarshalingType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MarshalingType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MarshalingType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for MarshalingType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.MarshalingType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Platform(pub i32);
impl Platform {
    pub const Windows: Self = Self(0i32);
    pub const WindowsPhone: Self = Self(1i32);
}
impl windows_core::TypeKind for Platform {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Platform {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Platform").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Platform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.Platform;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ThreadingModel(pub i32);
impl ThreadingModel {
    pub const STA: Self = Self(1i32);
    pub const MTA: Self = Self(2i32);
    pub const Both: Self = Self(3i32);
    pub const InvalidThreading: Self = Self(0i32);
}
impl windows_core::TypeKind for ThreadingModel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ThreadingModel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ThreadingModel").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for ThreadingModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.Metadata.ThreadingModel;i4)");
}
