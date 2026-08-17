windows_core::imp::define_interface!(IImageFeatureDescriptor, IImageFeatureDescriptor_Vtbl, 0x365585a5_171a_4a2a_985f_265159d3895a);
impl windows_core::RuntimeType for IImageFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageFeatureDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub BitmapPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    BitmapPixelFormat: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub BitmapAlphaMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    BitmapAlphaMode: usize,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageFeatureDescriptor2, IImageFeatureDescriptor2_Vtbl, 0x2b27cca7_d533_5862_bb98_1611b155b0e1);
impl windows_core::RuntimeType for IImageFeatureDescriptor2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageFeatureDescriptor2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PixelRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LearningModelPixelRange) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageFeatureValue, IImageFeatureValue_Vtbl, 0xf0414fd9_c9aa_4405_b7fb_94f87c8a3037);
impl windows_core::RuntimeType for IImageFeatureValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageFeatureValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media")]
    pub VideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media"))]
    VideoFrame: usize,
}
windows_core::imp::define_interface!(IImageFeatureValueStatics, IImageFeatureValueStatics_Vtbl, 0x1bc317fd_23cb_4610_b085_c8e1c87ebaa0);
impl windows_core::RuntimeType for IImageFeatureValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IImageFeatureValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media")]
    pub CreateFromVideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media"))]
    CreateFromVideoFrame: usize,
}
windows_core::imp::define_interface!(ILearningModel, ILearningModel_Vtbl, 0x5b8e4920_489f_4e86_9128_265a327b78fa);
impl windows_core::RuntimeType for ILearningModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InputFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InputFeatures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OutputFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OutputFeatures: usize,
}
windows_core::imp::define_interface!(ILearningModelBinding, ILearningModelBinding_Vtbl, 0xea312f20_168f_4f8c_94fe_2e7ac31b4aa8);
impl windows_core::RuntimeType for ILearningModelBinding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelBinding_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BindWithProperties: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BindWithProperties: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelBindingFactory, ILearningModelBindingFactory_Vtbl, 0xc95f7a7a_e788_475e_8917_23aa381faf0b);
impl windows_core::RuntimeType for ILearningModelBindingFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelBindingFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelDevice, ILearningModelDevice_Vtbl, 0xf5c2c8fe_3f56_4a8c_ac5f_fdb92d8b8252);
impl windows_core::RuntimeType for ILearningModelDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics")]
    pub AdapterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Graphics::DisplayAdapterId) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics"))]
    AdapterId: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
windows_core::imp::define_interface!(ILearningModelDeviceFactory, ILearningModelDeviceFactory_Vtbl, 0x9cffd74d_b1e5_4f20_80ad_0a56690db06b);
impl windows_core::RuntimeType for ILearningModelDeviceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelDeviceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, LearningModelDeviceKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelDeviceStatics, ILearningModelDeviceStatics_Vtbl, 0x49f32107_a8bf_42bb_92c7_10b12dc5d21f);
impl windows_core::RuntimeType for ILearningModelDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Device: usize,
}
windows_core::imp::define_interface!(ILearningModelEvaluationResult, ILearningModelEvaluationResult_Vtbl, 0xb2f9bfcd_960e_49c0_8593_eb190ae3eee2);
impl windows_core::RuntimeType for ILearningModelEvaluationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelEvaluationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ErrorStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Outputs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Outputs: usize,
}
windows_core::imp::define_interface!(ILearningModelFeatureDescriptor, ILearningModelFeatureDescriptor_Vtbl, 0xbc08cf7c_6ed0_4004_97ba_b9a2eecd2b4f);
impl core::ops::Deref for ILearningModelFeatureDescriptor {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILearningModelFeatureDescriptor, windows_core::IUnknown, windows_core::IInspectable);
impl ILearningModelFeatureDescriptor {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ILearningModelFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelFeatureDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LearningModelFeatureKind) -> windows_core::HRESULT,
    pub IsRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelFeatureValue, ILearningModelFeatureValue_Vtbl, 0xf51005db_4085_4dfe_9fed_95eb0c0cf75c);
impl core::ops::Deref for ILearningModelFeatureValue {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILearningModelFeatureValue, windows_core::IUnknown, windows_core::IInspectable);
impl ILearningModelFeatureValue {
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ILearningModelFeatureValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelFeatureValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LearningModelFeatureKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelOperatorProvider, ILearningModelOperatorProvider_Vtbl, 0x2a222e5d_afb1_47ed_bfad_b5b3a459ec04);
impl core::ops::Deref for ILearningModelOperatorProvider {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILearningModelOperatorProvider, windows_core::IUnknown, windows_core::IInspectable);
impl ILearningModelOperatorProvider {}
impl windows_core::RuntimeType for ILearningModelOperatorProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelOperatorProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ILearningModelSession, ILearningModelSession_Vtbl, 0x8e58f8f6_b787_4c11_90f0_7129aeca74a9);
impl windows_core::RuntimeType for ILearningModelSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelSession_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Model: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Device: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluationProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluationProperties: usize,
    pub EvaluateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluateFeaturesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluateFeaturesAsync: usize,
    pub Evaluate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub EvaluateFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    EvaluateFeatures: usize,
}
windows_core::imp::define_interface!(ILearningModelSessionFactory, ILearningModelSessionFactory_Vtbl, 0x0f6b881d_1c9b_47b6_bfe0_f1cf62a67579);
impl windows_core::RuntimeType for ILearningModelSessionFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelSessionFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromModelOnDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelSessionFactory2, ILearningModelSessionFactory2_Vtbl, 0x4e5c88bf_0a1f_5fec_ade0_2fd91e4ef29b);
impl windows_core::RuntimeType for ILearningModelSessionFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelSessionFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromModelOnDeviceWithSessionOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelSessionOptions, ILearningModelSessionOptions_Vtbl, 0xb8f63fa1_134d_5133_8cff_3a5c3c263beb);
impl windows_core::RuntimeType for ILearningModelSessionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelSessionOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BatchSizeOverride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetBatchSizeOverride: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelSessionOptions2, ILearningModelSessionOptions2_Vtbl, 0x6fcd1dc4_175f_5bd2_8de5_2f2006a25adf);
impl windows_core::RuntimeType for ILearningModelSessionOptions2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelSessionOptions2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CloseModelOnSessionCreation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCloseModelOnSessionCreation: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelSessionOptions3, ILearningModelSessionOptions3_Vtbl, 0x58e15cee_d8c2_56fc_92e8_76d751081086);
impl windows_core::RuntimeType for ILearningModelSessionOptions3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelSessionOptions3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OverrideNamedDimension: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILearningModelStatics, ILearningModelStatics_Vtbl, 0xe3b977e8_6952_4e47_8ef4_1f7f07897c6d);
impl windows_core::RuntimeType for ILearningModelStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ILearningModelStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub LoadFromStorageFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    LoadFromStorageFileAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamAsync: usize,
    pub LoadFromFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStream: usize,
    #[cfg(feature = "Storage")]
    pub LoadFromStorageFileWithOperatorProviderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    LoadFromStorageFileWithOperatorProviderAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamWithOperatorProviderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamWithOperatorProviderAsync: usize,
    pub LoadFromFilePathWithOperatorProvider: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub LoadFromStreamWithOperatorProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadFromStreamWithOperatorProvider: usize,
}
windows_core::imp::define_interface!(IMapFeatureDescriptor, IMapFeatureDescriptor_Vtbl, 0x530424bd_a257_436d_9e60_c2981f7cc5c4);
impl windows_core::RuntimeType for IMapFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMapFeatureDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub KeyKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TensorKind) -> windows_core::HRESULT,
    pub ValueDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISequenceFeatureDescriptor, ISequenceFeatureDescriptor_Vtbl, 0x84f6945a_562b_4d62_a851_739aced96668);
impl windows_core::RuntimeType for ISequenceFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ISequenceFeatureDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ElementDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensor, ITensor_Vtbl, 0x05489593_a305_4a25_ad09_440119b4b7f6);
impl core::ops::Deref for ITensor {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITensor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ITensor, ILearningModelFeatureValue);
impl ITensor {
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ITensor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TensorKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TensorKind) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Shape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shape: usize,
}
windows_core::imp::define_interface!(ITensorBoolean, ITensorBoolean_Vtbl, 0x50f311ed_29e9_4a5c_a44d_8fc512584eed);
impl windows_core::RuntimeType for ITensorBoolean {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorBoolean_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorBooleanStatics, ITensorBooleanStatics_Vtbl, 0x2796862c_2357_49a7_b476_d0aa3dfe6866);
impl windows_core::RuntimeType for ITensorBooleanStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorBooleanStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorBooleanStatics2, ITensorBooleanStatics2_Vtbl, 0xa3a4a501_6a2d_52d7_b04b_c435baee0115);
impl windows_core::RuntimeType for ITensorBooleanStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorBooleanStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorDouble, ITensorDouble_Vtbl, 0x91e41252_7a8f_4f0e_a28f_9637ffc8a3d0);
impl windows_core::RuntimeType for ITensorDouble {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorDouble_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorDoubleStatics, ITensorDoubleStatics_Vtbl, 0xa86693c5_9538_44e7_a3ca_5df374a5a70c);
impl windows_core::RuntimeType for ITensorDoubleStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorDoubleStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorDoubleStatics2, ITensorDoubleStatics2_Vtbl, 0x93a570de_5e9a_5094_85c8_592c655e68ac);
impl windows_core::RuntimeType for ITensorDoubleStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorDoubleStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorFeatureDescriptor, ITensorFeatureDescriptor_Vtbl, 0x74455c80_946a_4310_a19c_ee0af028fce4);
impl windows_core::RuntimeType for ITensorFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorFeatureDescriptor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TensorKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TensorKind) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Shape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Shape: usize,
}
windows_core::imp::define_interface!(ITensorFloat, ITensorFloat_Vtbl, 0xf2282d82_aa02_42c8_a0c8_df1efc9676e1);
impl windows_core::RuntimeType for ITensorFloat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorFloat_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorFloat16Bit, ITensorFloat16Bit_Vtbl, 0x0ab994fc_5b89_4c3c_b5e4_5282a5316c0a);
impl windows_core::RuntimeType for ITensorFloat16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorFloat16Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorFloat16BitStatics, ITensorFloat16BitStatics_Vtbl, 0xa52db6f5_318a_44d4_820b_0cdc7054a84a);
impl windows_core::RuntimeType for ITensorFloat16BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorFloat16BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorFloat16BitStatics2, ITensorFloat16BitStatics2_Vtbl, 0x68545726_2dc7_51bf_b470_0b344cc2a1bc);
impl windows_core::RuntimeType for ITensorFloat16BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorFloat16BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorFloatStatics, ITensorFloatStatics_Vtbl, 0xdbcd395b_3ba3_452f_b10d_3c135e573fa9);
impl windows_core::RuntimeType for ITensorFloatStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorFloatStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorFloatStatics2, ITensorFloatStatics2_Vtbl, 0x24610bc1_5e44_5713_b281_8f4ad4d555e8);
impl windows_core::RuntimeType for ITensorFloatStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorFloatStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorInt16Bit, ITensorInt16Bit_Vtbl, 0x98a32d39_e6d6_44af_8afa_baebc44dc020);
impl windows_core::RuntimeType for ITensorInt16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt16Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorInt16BitStatics, ITensorInt16BitStatics_Vtbl, 0x98646293_266e_4b1a_821f_e60d70898b91);
impl windows_core::RuntimeType for ITensorInt16BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt16BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorInt16BitStatics2, ITensorInt16BitStatics2_Vtbl, 0x0cd70cf4_696c_5e5f_95d8_5ebf9670148b);
impl windows_core::RuntimeType for ITensorInt16BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt16BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const i16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorInt32Bit, ITensorInt32Bit_Vtbl, 0x2c0c28d3_207c_4486_a7d2_884522c5e589);
impl windows_core::RuntimeType for ITensorInt32Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt32Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorInt32BitStatics, ITensorInt32BitStatics_Vtbl, 0x6539864b_52fa_4e35_907c_834cac417b50);
impl windows_core::RuntimeType for ITensorInt32BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt32BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorInt32BitStatics2, ITensorInt32BitStatics2_Vtbl, 0x7c4b079a_e956_5ce0_a3bd_157d9d79b5ec);
impl windows_core::RuntimeType for ITensorInt32BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt32BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorInt64Bit, ITensorInt64Bit_Vtbl, 0x499665ba_1fa2_45ad_af25_a0bd9bda4c87);
impl windows_core::RuntimeType for ITensorInt64Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt64Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorInt64BitStatics, ITensorInt64BitStatics_Vtbl, 0x9648ad9d_1198_4d74_9517_783ab62b9cc2);
impl windows_core::RuntimeType for ITensorInt64BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt64BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorInt64BitStatics2, ITensorInt64BitStatics2_Vtbl, 0x6d3d9dcb_ff40_5ec2_89fe_084e2b6bc6db);
impl windows_core::RuntimeType for ITensorInt64BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt64BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorInt8Bit, ITensorInt8Bit_Vtbl, 0xcddd97c5_ffd8_4fef_aefb_30e1a485b2ee);
impl windows_core::RuntimeType for ITensorInt8Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt8Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorInt8BitStatics, ITensorInt8BitStatics_Vtbl, 0xb1a12284_095c_4c76_a661_ac4cee1f3e8b);
impl windows_core::RuntimeType for ITensorInt8BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt8BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorInt8BitStatics2, ITensorInt8BitStatics2_Vtbl, 0xc0d59637_c468_56fb_9535_c052bdb93dc0);
impl windows_core::RuntimeType for ITensorInt8BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorInt8BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorString, ITensorString_Vtbl, 0x582335c8_bdb1_4610_bc75_35e9cbf009b7);
impl windows_core::RuntimeType for ITensorString {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorString_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorStringStatics, ITensorStringStatics_Vtbl, 0x83623324_cf26_4f17_a2d4_20ef8d097d53);
impl windows_core::RuntimeType for ITensorStringStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorStringStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorStringStatics2, ITensorStringStatics2_Vtbl, 0x9e355ed0_c8e2_5254_9137_0193a3668fd8);
impl windows_core::RuntimeType for ITensorStringStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorStringStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITensorUInt16Bit, ITensorUInt16Bit_Vtbl, 0x68140f4b_23c0_42f3_81f6_a891c011bc3f);
impl windows_core::RuntimeType for ITensorUInt16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt16Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorUInt16BitStatics, ITensorUInt16BitStatics_Vtbl, 0x5df745dd_028a_481a_a27c_c7e6435e52dd);
impl windows_core::RuntimeType for ITensorUInt16BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt16BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorUInt16BitStatics2, ITensorUInt16BitStatics2_Vtbl, 0x8af40c64_d69f_5315_9348_490877bbd642);
impl windows_core::RuntimeType for ITensorUInt16BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt16BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorUInt32Bit, ITensorUInt32Bit_Vtbl, 0xd8c9c2ff_7511_45a3_bfac_c38f370d2237);
impl windows_core::RuntimeType for ITensorUInt32Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt32Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorUInt32BitStatics, ITensorUInt32BitStatics_Vtbl, 0x417c3837_e773_4378_8e7f_0cc33dbea697);
impl windows_core::RuntimeType for ITensorUInt32BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt32BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorUInt32BitStatics2, ITensorUInt32BitStatics2_Vtbl, 0xef1a1f1c_314e_569d_b496_5c8447d20cd2);
impl windows_core::RuntimeType for ITensorUInt32BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt32BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorUInt64Bit, ITensorUInt64Bit_Vtbl, 0x2e70ffad_04bf_4825_839a_82baef8c7886);
impl windows_core::RuntimeType for ITensorUInt64Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt64Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorUInt64BitStatics, ITensorUInt64BitStatics_Vtbl, 0x7a7e20eb_242f_47cb_a9c6_f602ecfbfee4);
impl windows_core::RuntimeType for ITensorUInt64BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt64BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorUInt64BitStatics2, ITensorUInt64BitStatics2_Vtbl, 0x085a687d_67e1_5b1e_b232_4fabe9ca20b3);
impl windows_core::RuntimeType for ITensorUInt64BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt64BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
windows_core::imp::define_interface!(ITensorUInt8Bit, ITensorUInt8Bit_Vtbl, 0x58e1ae27_622b_48e3_be22_d867aed1daac);
impl windows_core::RuntimeType for ITensorUInt8Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt8Bit_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsVectorView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsVectorView: usize,
}
windows_core::imp::define_interface!(ITensorUInt8BitStatics, ITensorUInt8BitStatics_Vtbl, 0x05f67583_bc24_4220_8a41_2dcd8c5ed33c);
impl windows_core::RuntimeType for ITensorUInt8BitStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt8BitStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Create2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Create2: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromArray: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateFromIterable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateFromIterable: usize,
}
windows_core::imp::define_interface!(ITensorUInt8BitStatics2, ITensorUInt8BitStatics2_Vtbl, 0x2ba042d6_373e_5a3a_a2fc_a6c41bd52789);
impl windows_core::RuntimeType for ITensorUInt8BitStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct ITensorUInt8BitStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromShapeArrayAndDataArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromBuffer: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ImageFeatureDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageFeatureDescriptor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ImageFeatureDescriptor, ILearningModelFeatureDescriptor);
impl ImageFeatureDescriptor {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn BitmapPixelFormat(&self) -> windows_core::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapPixelFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn BitmapAlphaMode(&self) -> windows_core::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BitmapAlphaMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Height(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelRange(&self) -> windows_core::Result<LearningModelPixelRange> {
        let this = &windows_core::Interface::cast::<IImageFeatureDescriptor2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelRange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ImageFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageFeatureDescriptor>();
}
unsafe impl windows_core::Interface for ImageFeatureDescriptor {
    type Vtable = IImageFeatureDescriptor_Vtbl;
    const IID: windows_core::GUID = <IImageFeatureDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureDescriptor";
}
unsafe impl Send for ImageFeatureDescriptor {}
unsafe impl Sync for ImageFeatureDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct ImageFeatureValue(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageFeatureValue, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ImageFeatureValue, ILearningModelFeatureValue);
impl ImageFeatureValue {
    #[cfg(feature = "Media")]
    pub fn VideoFrame(&self) -> windows_core::Result<super::super::Media::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VideoFrame)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Media")]
    pub fn CreateFromVideoFrame<P0>(image: P0) -> windows_core::Result<ImageFeatureValue>
    where
        P0: windows_core::Param<super::super::Media::VideoFrame>,
    {
        Self::IImageFeatureValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromVideoFrame)(windows_core::Interface::as_raw(this), image.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[doc(hidden)]
    pub fn IImageFeatureValueStatics<R, F: FnOnce(&IImageFeatureValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ImageFeatureValue, IImageFeatureValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ImageFeatureValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageFeatureValue>();
}
unsafe impl windows_core::Interface for ImageFeatureValue {
    type Vtable = IImageFeatureValue_Vtbl;
    const IID: windows_core::GUID = <IImageFeatureValue as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageFeatureValue {
    const NAME: &'static str = "Windows.AI.MachineLearning.ImageFeatureValue";
}
unsafe impl Send for ImageFeatureValue {}
unsafe impl Sync for ImageFeatureValue {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LearningModel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModel, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LearningModel, super::super::Foundation::IClosable);
impl LearningModel {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Author(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Author)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Domain(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Domain)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Version(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metadata(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Metadata)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InputFeatures(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputFeatures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn OutputFeatures(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<ILearningModelFeatureDescriptor>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OutputFeatures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage")]
    pub fn LoadFromStorageFileAsync<P0>(modelfile: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStorageFileAsync)(windows_core::Interface::as_raw(this), modelfile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamAsync<P0>(modelstream: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStreamAsync)(windows_core::Interface::as_raw(this), modelstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LoadFromFilePath(filepath: &windows_core::HSTRING) -> windows_core::Result<LearningModel> {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromFilePath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<P0>(modelstream: P0) -> windows_core::Result<LearningModel>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStream)(windows_core::Interface::as_raw(this), modelstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage")]
    pub fn LoadFromStorageFileWithOperatorProviderAsync<P0, P1>(modelfile: P0, operatorprovider: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: windows_core::Param<super::super::Storage::IStorageFile>,
        P1: windows_core::Param<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStorageFileWithOperatorProviderAsync)(windows_core::Interface::as_raw(this), modelfile.param().abi(), operatorprovider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamWithOperatorProviderAsync<P0, P1>(modelstream: P0, operatorprovider: P1) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModel>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
        P1: windows_core::Param<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStreamWithOperatorProviderAsync)(windows_core::Interface::as_raw(this), modelstream.param().abi(), operatorprovider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LoadFromFilePathWithOperatorProvider<P0>(filepath: &windows_core::HSTRING, operatorprovider: P0) -> windows_core::Result<LearningModel>
    where
        P0: windows_core::Param<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromFilePathWithOperatorProvider)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), operatorprovider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStreamWithOperatorProvider<P0, P1>(modelstream: P0, operatorprovider: P1) -> windows_core::Result<LearningModel>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
        P1: windows_core::Param<ILearningModelOperatorProvider>,
    {
        Self::ILearningModelStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromStreamWithOperatorProvider)(windows_core::Interface::as_raw(this), modelstream.param().abi(), operatorprovider.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelStatics<R, F: FnOnce(&ILearningModelStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModel, ILearningModelStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LearningModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModel>();
}
unsafe impl windows_core::Interface for LearningModel {
    type Vtable = ILearningModel_Vtbl;
    const IID: windows_core::GUID = <ILearningModel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModel {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModel";
}
unsafe impl Send for LearningModel {}
unsafe impl Sync for LearningModel {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LearningModelBinding(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModelBinding, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(LearningModelBinding, super::super::Foundation::Collections::IIterable::<super::super::Foundation::Collections::IKeyValuePair::<windows_core::HSTRING, windows_core::IInspectable>>, super::super::Foundation::Collections::IMapView::<windows_core::HSTRING, windows_core::IInspectable>);
impl LearningModelBinding {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Bind<P0>(&self, name: &windows_core::HSTRING, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Bind)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BindWithProperties<P0, P1>(&self, name: &windows_core::HSTRING, value: P0, props: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).BindWithProperties)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), value.param().abi(), props.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateFromSession<P0>(session: P0) -> windows_core::Result<LearningModelBinding>
    where
        P0: windows_core::Param<LearningModelSession>,
    {
        Self::ILearningModelBindingFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromSession)(windows_core::Interface::as_raw(this), session.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(key), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut Option<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>, second: &mut Option<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Split)(windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc(hidden)]
    pub fn ILearningModelBindingFactory<R, F: FnOnce(&ILearningModelBindingFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelBinding, ILearningModelBindingFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LearningModelBinding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModelBinding>();
}
unsafe impl windows_core::Interface for LearningModelBinding {
    type Vtable = ILearningModelBinding_Vtbl;
    const IID: windows_core::GUID = <ILearningModelBinding as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModelBinding {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelBinding";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for LearningModelBinding {
    type Item = super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &LearningModelBinding {
    type Item = super::super::Foundation::Collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
unsafe impl Send for LearningModelBinding {}
unsafe impl Sync for LearningModelBinding {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LearningModelDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModelDevice, windows_core::IUnknown, windows_core::IInspectable);
impl LearningModelDevice {
    #[cfg(feature = "Graphics")]
    pub fn AdapterId(&self) -> windows_core::Result<super::super::Graphics::DisplayAdapterId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdapterId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> windows_core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direct3D11Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(devicekind: LearningModelDeviceKind) -> windows_core::Result<LearningModelDevice> {
        Self::ILearningModelDeviceFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), devicekind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11Device<P0>(device: P0) -> windows_core::Result<LearningModelDevice>
    where
        P0: windows_core::Param<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>,
    {
        Self::ILearningModelDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromDirect3D11Device)(windows_core::Interface::as_raw(this), device.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelDeviceFactory<R, F: FnOnce(&ILearningModelDeviceFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelDevice, ILearningModelDeviceFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILearningModelDeviceStatics<R, F: FnOnce(&ILearningModelDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelDevice, ILearningModelDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LearningModelDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModelDevice>();
}
unsafe impl windows_core::Interface for LearningModelDevice {
    type Vtable = ILearningModelDevice_Vtbl;
    const IID: windows_core::GUID = <ILearningModelDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModelDevice {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelDevice";
}
unsafe impl Send for LearningModelDevice {}
unsafe impl Sync for LearningModelDevice {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LearningModelEvaluationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModelEvaluationResult, windows_core::IUnknown, windows_core::IInspectable);
impl LearningModelEvaluationResult {
    pub fn CorrelationId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CorrelationId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ErrorStatus(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Outputs(&self) -> windows_core::Result<super::super::Foundation::Collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Outputs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LearningModelEvaluationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModelEvaluationResult>();
}
unsafe impl windows_core::Interface for LearningModelEvaluationResult {
    type Vtable = ILearningModelEvaluationResult_Vtbl;
    const IID: windows_core::GUID = <ILearningModelEvaluationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModelEvaluationResult {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelEvaluationResult";
}
unsafe impl Send for LearningModelEvaluationResult {}
unsafe impl Sync for LearningModelEvaluationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LearningModelSession(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModelSession, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LearningModelSession, super::super::Foundation::IClosable);
impl LearningModelSession {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Model(&self) -> windows_core::Result<LearningModel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Model)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Device(&self) -> windows_core::Result<LearningModelDevice> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Device)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluationProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EvaluationProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EvaluateAsync<P0>(&self, bindings: P0, correlationid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>
    where
        P0: windows_core::Param<LearningModelBinding>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EvaluateAsync)(windows_core::Interface::as_raw(this), bindings.param().abi(), core::mem::transmute_copy(correlationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluateFeaturesAsync<P0>(&self, features: P0, correlationid: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::IAsyncOperation<LearningModelEvaluationResult>>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EvaluateFeaturesAsync)(windows_core::Interface::as_raw(this), features.param().abi(), core::mem::transmute_copy(correlationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Evaluate<P0>(&self, bindings: P0, correlationid: &windows_core::HSTRING) -> windows_core::Result<LearningModelEvaluationResult>
    where
        P0: windows_core::Param<LearningModelBinding>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Evaluate)(windows_core::Interface::as_raw(this), bindings.param().abi(), core::mem::transmute_copy(correlationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn EvaluateFeatures<P0>(&self, features: P0, correlationid: &windows_core::HSTRING) -> windows_core::Result<LearningModelEvaluationResult>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EvaluateFeatures)(windows_core::Interface::as_raw(this), features.param().abi(), core::mem::transmute_copy(correlationid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromModel<P0>(model: P0) -> windows_core::Result<LearningModelSession>
    where
        P0: windows_core::Param<LearningModel>,
    {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromModel)(windows_core::Interface::as_raw(this), model.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromModelOnDevice<P0, P1>(model: P0, devicetorunon: P1) -> windows_core::Result<LearningModelSession>
    where
        P0: windows_core::Param<LearningModel>,
        P1: windows_core::Param<LearningModelDevice>,
    {
        Self::ILearningModelSessionFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromModelOnDevice)(windows_core::Interface::as_raw(this), model.param().abi(), devicetorunon.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromModelOnDeviceWithSessionOptions<P0, P1, P2>(model: P0, devicetorunon: P1, learningmodelsessionoptions: P2) -> windows_core::Result<LearningModelSession>
    where
        P0: windows_core::Param<LearningModel>,
        P1: windows_core::Param<LearningModelDevice>,
        P2: windows_core::Param<LearningModelSessionOptions>,
    {
        Self::ILearningModelSessionFactory2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromModelOnDeviceWithSessionOptions)(windows_core::Interface::as_raw(this), model.param().abi(), devicetorunon.param().abi(), learningmodelsessionoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ILearningModelSessionFactory<R, F: FnOnce(&ILearningModelSessionFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelSession, ILearningModelSessionFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ILearningModelSessionFactory2<R, F: FnOnce(&ILearningModelSessionFactory2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelSession, ILearningModelSessionFactory2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LearningModelSession {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModelSession>();
}
unsafe impl windows_core::Interface for LearningModelSession {
    type Vtable = ILearningModelSession_Vtbl;
    const IID: windows_core::GUID = <ILearningModelSession as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModelSession {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSession";
}
unsafe impl Send for LearningModelSession {}
unsafe impl Sync for LearningModelSession {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct LearningModelSessionOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LearningModelSessionOptions, windows_core::IUnknown, windows_core::IInspectable);
impl LearningModelSessionOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LearningModelSessionOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn BatchSizeOverride(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BatchSizeOverride)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBatchSizeOverride(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBatchSizeOverride)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CloseModelOnSessionCreation(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloseModelOnSessionCreation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCloseModelOnSessionCreation(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILearningModelSessionOptions2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCloseModelOnSessionCreation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OverrideNamedDimension(&self, name: &windows_core::HSTRING, dimension: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ILearningModelSessionOptions3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OverrideNamedDimension)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), dimension).ok() }
    }
}
impl windows_core::RuntimeType for LearningModelSessionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILearningModelSessionOptions>();
}
unsafe impl windows_core::Interface for LearningModelSessionOptions {
    type Vtable = ILearningModelSessionOptions_Vtbl;
    const IID: windows_core::GUID = <ILearningModelSessionOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LearningModelSessionOptions {
    const NAME: &'static str = "Windows.AI.MachineLearning.LearningModelSessionOptions";
}
unsafe impl Send for LearningModelSessionOptions {}
unsafe impl Sync for LearningModelSessionOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct MapFeatureDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MapFeatureDescriptor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MapFeatureDescriptor, ILearningModelFeatureDescriptor);
impl MapFeatureDescriptor {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn KeyKind(&self) -> windows_core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ValueDescriptor(&self) -> windows_core::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValueDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MapFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMapFeatureDescriptor>();
}
unsafe impl windows_core::Interface for MapFeatureDescriptor {
    type Vtable = IMapFeatureDescriptor_Vtbl;
    const IID: windows_core::GUID = <IMapFeatureDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MapFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.MapFeatureDescriptor";
}
unsafe impl Send for MapFeatureDescriptor {}
unsafe impl Sync for MapFeatureDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct SequenceFeatureDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SequenceFeatureDescriptor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SequenceFeatureDescriptor, ILearningModelFeatureDescriptor);
impl SequenceFeatureDescriptor {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ElementDescriptor(&self) -> windows_core::Result<ILearningModelFeatureDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementDescriptor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for SequenceFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISequenceFeatureDescriptor>();
}
unsafe impl windows_core::Interface for SequenceFeatureDescriptor {
    type Vtable = ISequenceFeatureDescriptor_Vtbl;
    const IID: windows_core::GUID = <ISequenceFeatureDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SequenceFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.SequenceFeatureDescriptor";
}
unsafe impl Send for SequenceFeatureDescriptor {}
unsafe impl Sync for SequenceFeatureDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorBoolean(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorBoolean, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorBoolean, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorBoolean {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorBoolean>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[bool]) -> windows_core::Result<TensorBoolean>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorBoolean>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<bool>>,
    {
        Self::ITensorBooleanStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[bool]) -> windows_core::Result<TensorBoolean> {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorBoolean>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorBooleanStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorBooleanStatics<R, F: FnOnce(&ITensorBooleanStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorBoolean, ITensorBooleanStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorBooleanStatics2<R, F: FnOnce(&ITensorBooleanStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorBoolean, ITensorBooleanStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorBoolean {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorBoolean>();
}
unsafe impl windows_core::Interface for TensorBoolean {
    type Vtable = ITensorBoolean_Vtbl;
    const IID: windows_core::GUID = <ITensorBoolean as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorBoolean {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorBoolean";
}
unsafe impl Send for TensorBoolean {}
unsafe impl Sync for TensorBoolean {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorDouble(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorDouble, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorDouble, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorDouble {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorDouble> {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorDouble>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[f64]) -> windows_core::Result<TensorDouble>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorDouble>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<f64>>,
    {
        Self::ITensorDoubleStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f64]) -> windows_core::Result<TensorDouble> {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorDouble>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorDoubleStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorDoubleStatics<R, F: FnOnce(&ITensorDoubleStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorDouble, ITensorDoubleStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorDoubleStatics2<R, F: FnOnce(&ITensorDoubleStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorDouble, ITensorDoubleStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorDouble {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorDouble>();
}
unsafe impl windows_core::Interface for TensorDouble {
    type Vtable = ITensorDouble_Vtbl;
    const IID: windows_core::GUID = <ITensorDouble as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorDouble {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorDouble";
}
unsafe impl Send for TensorDouble {}
unsafe impl Sync for TensorDouble {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorFeatureDescriptor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorFeatureDescriptor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorFeatureDescriptor, ILearningModelFeatureDescriptor);
impl TensorFeatureDescriptor {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsRequired(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureDescriptor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRequired)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TensorFeatureDescriptor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorFeatureDescriptor>();
}
unsafe impl windows_core::Interface for TensorFeatureDescriptor {
    type Vtable = ITensorFeatureDescriptor_Vtbl;
    const IID: windows_core::GUID = <ITensorFeatureDescriptor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorFeatureDescriptor {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFeatureDescriptor";
}
unsafe impl Send for TensorFeatureDescriptor {}
unsafe impl Sync for TensorFeatureDescriptor {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorFloat(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorFloat, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorFloat, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorFloat {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorFloat> {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorFloat>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[f32]) -> windows_core::Result<TensorFloat>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorFloat>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<f32>>,
    {
        Self::ITensorFloatStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f32]) -> windows_core::Result<TensorFloat> {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorFloat>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorFloatStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorFloatStatics<R, F: FnOnce(&ITensorFloatStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorFloat, ITensorFloatStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorFloatStatics2<R, F: FnOnce(&ITensorFloatStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorFloat, ITensorFloatStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorFloat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorFloat>();
}
unsafe impl windows_core::Interface for TensorFloat {
    type Vtable = ITensorFloat_Vtbl;
    const IID: windows_core::GUID = <ITensorFloat as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorFloat {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat";
}
unsafe impl Send for TensorFloat {}
unsafe impl Sync for TensorFloat {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorFloat16Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorFloat16Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorFloat16Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorFloat16Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<f32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorFloat16Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[f32]) -> windows_core::Result<TensorFloat16Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorFloat16Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<f32>>,
    {
        Self::ITensorFloat16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[f32]) -> windows_core::Result<TensorFloat16Bit> {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorFloat16Bit>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorFloat16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorFloat16BitStatics<R, F: FnOnce(&ITensorFloat16BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorFloat16BitStatics2<R, F: FnOnce(&ITensorFloat16BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorFloat16Bit, ITensorFloat16BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorFloat16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorFloat16Bit>();
}
unsafe impl windows_core::Interface for TensorFloat16Bit {
    type Vtable = ITensorFloat16Bit_Vtbl;
    const IID: windows_core::GUID = <ITensorFloat16Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorFloat16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorFloat16Bit";
}
unsafe impl Send for TensorFloat16Bit {}
unsafe impl Sync for TensorFloat16Bit {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorInt16Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorInt16Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorInt16Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorInt16Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i16>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorInt16Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[i16]) -> windows_core::Result<TensorInt16Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorInt16Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<i16>>,
    {
        Self::ITensorInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i16]) -> windows_core::Result<TensorInt16Bit> {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorInt16Bit>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt16BitStatics<R, F: FnOnce(&ITensorInt16BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt16BitStatics2<R, F: FnOnce(&ITensorInt16BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt16Bit, ITensorInt16BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorInt16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorInt16Bit>();
}
unsafe impl windows_core::Interface for TensorInt16Bit {
    type Vtable = ITensorInt16Bit_Vtbl;
    const IID: windows_core::GUID = <ITensorInt16Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt16Bit";
}
unsafe impl Send for TensorInt16Bit {}
unsafe impl Sync for TensorInt16Bit {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorInt32Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorInt32Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorInt32Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorInt32Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorInt32Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[i32]) -> windows_core::Result<TensorInt32Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorInt32Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<i32>>,
    {
        Self::ITensorInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i32]) -> windows_core::Result<TensorInt32Bit> {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorInt32Bit>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt32BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt32BitStatics<R, F: FnOnce(&ITensorInt32BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt32BitStatics2<R, F: FnOnce(&ITensorInt32BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt32Bit, ITensorInt32BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorInt32Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorInt32Bit>();
}
unsafe impl windows_core::Interface for TensorInt32Bit {
    type Vtable = ITensorInt32Bit_Vtbl;
    const IID: windows_core::GUID = <ITensorInt32Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt32Bit";
}
unsafe impl Send for TensorInt32Bit {}
unsafe impl Sync for TensorInt32Bit {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorInt64Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorInt64Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorInt64Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorInt64Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorInt64Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[i64]) -> windows_core::Result<TensorInt64Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorInt64Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[i64]) -> windows_core::Result<TensorInt64Bit> {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorInt64Bit>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt64BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt64BitStatics<R, F: FnOnce(&ITensorInt64BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt64BitStatics2<R, F: FnOnce(&ITensorInt64BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt64Bit, ITensorInt64BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorInt64Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorInt64Bit>();
}
unsafe impl windows_core::Interface for TensorInt64Bit {
    type Vtable = ITensorInt64Bit_Vtbl;
    const IID: windows_core::GUID = <ITensorInt64Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt64Bit";
}
unsafe impl Send for TensorInt64Bit {}
unsafe impl Sync for TensorInt64Bit {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorInt8Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorInt8Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorInt8Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorInt8Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorInt8Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u8]) -> windows_core::Result<TensorInt8Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorInt8Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<u8>>,
    {
        Self::ITensorInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u8]) -> windows_core::Result<TensorInt8Bit> {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorInt8Bit>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorInt8BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorInt8BitStatics<R, F: FnOnce(&ITensorInt8BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorInt8BitStatics2<R, F: FnOnce(&ITensorInt8BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorInt8Bit, ITensorInt8BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorInt8Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorInt8Bit>();
}
unsafe impl windows_core::Interface for TensorInt8Bit {
    type Vtable = ITensorInt8Bit_Vtbl;
    const IID: windows_core::GUID = <ITensorInt8Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorInt8Bit";
}
unsafe impl Send for TensorInt8Bit {}
unsafe impl Sync for TensorInt8Bit {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorString(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorString, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorString, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorString {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorString> {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorString>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[windows_core::HSTRING]) -> windows_core::Result<TensorString>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), core::mem::transmute(data.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorString>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<windows_core::HSTRING>>,
    {
        Self::ITensorStringStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[windows_core::HSTRING]) -> windows_core::Result<TensorString> {
        Self::ITensorStringStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), core::mem::transmute(data.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorStringStatics<R, F: FnOnce(&ITensorStringStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorString, ITensorStringStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorStringStatics2<R, F: FnOnce(&ITensorStringStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorString, ITensorStringStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorString {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorString>();
}
unsafe impl windows_core::Interface for TensorString {
    type Vtable = ITensorString_Vtbl;
    const IID: windows_core::GUID = <ITensorString as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorString {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorString";
}
unsafe impl Send for TensorString {}
unsafe impl Sync for TensorString {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorUInt16Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorUInt16Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorUInt16Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorUInt16Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u16>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorUInt16Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u16]) -> windows_core::Result<TensorUInt16Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorUInt16Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<u16>>,
    {
        Self::ITensorUInt16BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u16]) -> windows_core::Result<TensorUInt16Bit> {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorUInt16Bit>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt16BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt16BitStatics<R, F: FnOnce(&ITensorUInt16BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt16BitStatics2<R, F: FnOnce(&ITensorUInt16BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt16Bit, ITensorUInt16BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorUInt16Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorUInt16Bit>();
}
unsafe impl windows_core::Interface for TensorUInt16Bit {
    type Vtable = ITensorUInt16Bit_Vtbl;
    const IID: windows_core::GUID = <ITensorUInt16Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorUInt16Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt16Bit";
}
unsafe impl Send for TensorUInt16Bit {}
unsafe impl Sync for TensorUInt16Bit {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorUInt32Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorUInt32Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorUInt32Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorUInt32Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorUInt32Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u32]) -> windows_core::Result<TensorUInt32Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorUInt32Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<u32>>,
    {
        Self::ITensorUInt32BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u32]) -> windows_core::Result<TensorUInt32Bit> {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorUInt32Bit>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt32BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt32BitStatics<R, F: FnOnce(&ITensorUInt32BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt32BitStatics2<R, F: FnOnce(&ITensorUInt32BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt32Bit, ITensorUInt32BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorUInt32Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorUInt32Bit>();
}
unsafe impl windows_core::Interface for TensorUInt32Bit {
    type Vtable = ITensorUInt32Bit_Vtbl;
    const IID: windows_core::GUID = <ITensorUInt32Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorUInt32Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt32Bit";
}
unsafe impl Send for TensorUInt32Bit {}
unsafe impl Sync for TensorUInt32Bit {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorUInt64Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorUInt64Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorUInt64Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorUInt64Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorUInt64Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u64]) -> windows_core::Result<TensorUInt64Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorUInt64Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<u64>>,
    {
        Self::ITensorUInt64BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u64]) -> windows_core::Result<TensorUInt64Bit> {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorUInt64Bit>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt64BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt64BitStatics<R, F: FnOnce(&ITensorUInt64BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt64BitStatics2<R, F: FnOnce(&ITensorUInt64BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt64Bit, ITensorUInt64BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorUInt64Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorUInt64Bit>();
}
unsafe impl windows_core::Interface for TensorUInt64Bit {
    type Vtable = ITensorUInt64Bit_Vtbl;
    const IID: windows_core::GUID = <ITensorUInt64Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorUInt64Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt64Bit";
}
unsafe impl Send for TensorUInt64Bit {}
unsafe impl Sync for TensorUInt64Bit {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct TensorUInt8Bit(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TensorUInt8Bit, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TensorUInt8Bit, super::super::Foundation::IClosable, ILearningModelFeatureValue, super::super::Foundation::IMemoryBuffer, ITensor);
impl TensorUInt8Bit {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<LearningModelFeatureKind> {
        let this = &windows_core::Interface::cast::<ILearningModelFeatureValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CreateReference(&self) -> windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TensorKind(&self) -> windows_core::Result<TensorKind> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TensorKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Shape(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<i64>> {
        let this = &windows_core::Interface::cast::<ITensor>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Shape)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsVectorView(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAsVectorView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create() -> windows_core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create2<P0>(shape: P0) -> windows_core::Result<TensorUInt8Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create2)(windows_core::Interface::as_raw(this), shape.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromArray<P0>(shape: P0, data: &[u8]) -> windows_core::Result<TensorUInt8Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromArray)(windows_core::Interface::as_raw(this), shape.param().abi(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateFromIterable<P0, P1>(shape: P0, data: P1) -> windows_core::Result<TensorUInt8Bit>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IIterable<i64>>,
        P1: windows_core::Param<super::super::Foundation::Collections::IIterable<u8>>,
    {
        Self::ITensorUInt8BitStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromIterable)(windows_core::Interface::as_raw(this), shape.param().abi(), data.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromShapeArrayAndDataArray(shape: &[i64], data: &[u8]) -> windows_core::Result<TensorUInt8Bit> {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromShapeArrayAndDataArray)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), data.len().try_into().unwrap(), data.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromBuffer<P0>(shape: &[i64], buffer: P0) -> windows_core::Result<TensorUInt8Bit>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IBuffer>,
    {
        Self::ITensorUInt8BitStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromBuffer)(windows_core::Interface::as_raw(this), shape.len().try_into().unwrap(), shape.as_ptr(), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn ITensorUInt8BitStatics<R, F: FnOnce(&ITensorUInt8BitStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITensorUInt8BitStatics2<R, F: FnOnce(&ITensorUInt8BitStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TensorUInt8Bit, ITensorUInt8BitStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TensorUInt8Bit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITensorUInt8Bit>();
}
unsafe impl windows_core::Interface for TensorUInt8Bit {
    type Vtable = ITensorUInt8Bit_Vtbl;
    const IID: windows_core::GUID = <ITensorUInt8Bit as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TensorUInt8Bit {
    const NAME: &'static str = "Windows.AI.MachineLearning.TensorUInt8Bit";
}
unsafe impl Send for TensorUInt8Bit {}
unsafe impl Sync for TensorUInt8Bit {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LearningModelDeviceKind(pub i32);
impl LearningModelDeviceKind {
    pub const Default: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const DirectX: Self = Self(2i32);
    pub const DirectXHighPerformance: Self = Self(3i32);
    pub const DirectXMinPower: Self = Self(4i32);
}
impl windows_core::TypeKind for LearningModelDeviceKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LearningModelDeviceKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LearningModelDeviceKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LearningModelDeviceKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelDeviceKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LearningModelFeatureKind(pub i32);
impl LearningModelFeatureKind {
    pub const Tensor: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
    pub const Map: Self = Self(2i32);
    pub const Image: Self = Self(3i32);
}
impl windows_core::TypeKind for LearningModelFeatureKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LearningModelFeatureKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LearningModelFeatureKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LearningModelFeatureKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelFeatureKind;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LearningModelPixelRange(pub i32);
impl LearningModelPixelRange {
    pub const ZeroTo255: Self = Self(0i32);
    pub const ZeroToOne: Self = Self(1i32);
    pub const MinusOneToOne: Self = Self(2i32);
}
impl windows_core::TypeKind for LearningModelPixelRange {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LearningModelPixelRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LearningModelPixelRange").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for LearningModelPixelRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.LearningModelPixelRange;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TensorKind(pub i32);
impl TensorKind {
    pub const Undefined: Self = Self(0i32);
    pub const Float: Self = Self(1i32);
    pub const UInt8: Self = Self(2i32);
    pub const Int8: Self = Self(3i32);
    pub const UInt16: Self = Self(4i32);
    pub const Int16: Self = Self(5i32);
    pub const Int32: Self = Self(6i32);
    pub const Int64: Self = Self(7i32);
    pub const String: Self = Self(8i32);
    pub const Boolean: Self = Self(9i32);
    pub const Float16: Self = Self(10i32);
    pub const Double: Self = Self(11i32);
    pub const UInt32: Self = Self(12i32);
    pub const UInt64: Self = Self(13i32);
    pub const Complex64: Self = Self(14i32);
    pub const Complex128: Self = Self(15i32);
}
impl windows_core::TypeKind for TensorKind {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TensorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TensorKind").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for TensorKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.MachineLearning.TensorKind;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
