windows_core::imp::define_interface!(IPrint3DManager, IPrint3DManager_Vtbl, 0x4d2fcb0a_7366_4971_8bd5_17c4e3e8c6c0);
impl windows_core::RuntimeType for IPrint3DManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TaskRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveTaskRequested: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DManagerStatics, IPrint3DManagerStatics_Vtbl, 0x0ef1cafe_a9ad_4c08_a917_1d1f863eabcb);
impl windows_core::RuntimeType for IPrint3DManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowPrintUIAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DTask, IPrint3DTask_Vtbl, 0x8ce3d080_2118_4c28_80de_f426d70191ae);
impl windows_core::RuntimeType for IPrint3DTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DTask_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Submitting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSubmitting: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub SourceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
    pub RemoveSourceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::EventRegistrationToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DTaskCompletedEventArgs, IPrint3DTaskCompletedEventArgs_Vtbl, 0xcc1914af_2614_4f1d_accc_d6fc4fda5455);
impl windows_core::RuntimeType for IPrint3DTaskCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DTaskCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Completion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Print3DTaskCompletion) -> windows_core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Print3DTaskDetail) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DTaskRequest, IPrint3DTaskRequest_Vtbl, 0x2595c46f_2245_4c5a_8731_0d604dc6bc3c);
impl windows_core::RuntimeType for IPrint3DTaskRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DTaskRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateTask: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DTaskRequestedEventArgs, IPrint3DTaskRequestedEventArgs_Vtbl, 0x150cb77f_18c5_40d7_9f40_fab3096e05a9);
impl windows_core::RuntimeType for IPrint3DTaskRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DTaskRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DTaskSourceChangedEventArgs, IPrint3DTaskSourceChangedEventArgs_Vtbl, 0x5bcd34af_24e9_4c10_8d07_14c346ba3fcf);
impl windows_core::RuntimeType for IPrint3DTaskSourceChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DTaskSourceChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrint3DTaskSourceRequestedArgs, IPrint3DTaskSourceRequestedArgs_Vtbl, 0xc77c9aba_24af_424d_a3bf_92250c355602);
impl windows_core::RuntimeType for IPrint3DTaskSourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrint3DTaskSourceRequestedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3D3MFPackage, IPrinting3D3MFPackage_Vtbl, 0xf64dd5c8_2ab7_45a9_a1b7_267e948d5b18);
impl windows_core::RuntimeType for IPrinting3D3MFPackage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3D3MFPackage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SaveAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SaveAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PrintTicket: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPrintTicket: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPrintTicket: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ModelPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ModelPart: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelPart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelPart: usize,
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Textures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Textures: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadModelFromPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadModelFromPackageAsync: usize,
    pub SaveModelToPackageAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3D3MFPackage2, IPrinting3D3MFPackage2_Vtbl, 0x965c7ac4_93cb_4430_92b8_789cd454f883);
impl windows_core::RuntimeType for IPrinting3D3MFPackage2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3D3MFPackage2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Compression: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Printing3DPackageCompression) -> windows_core::HRESULT,
    pub SetCompression: unsafe extern "system" fn(*mut core::ffi::c_void, Printing3DPackageCompression) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3D3MFPackageStatics, IPrinting3D3MFPackageStatics_Vtbl, 0x7058d9af_7a9a_4787_b817_f6f459214823);
impl windows_core::RuntimeType for IPrinting3D3MFPackageStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3D3MFPackageStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub LoadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadAsync: usize,
}
windows_core::imp::define_interface!(IPrinting3DBaseMaterial, IPrinting3DBaseMaterial_Vtbl, 0xd0f0e743_c50c_4bcb_9d04_fc16adcea2c9);
impl windows_core::RuntimeType for IPrinting3DBaseMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DBaseMaterial_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Color: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DBaseMaterialGroup, IPrinting3DBaseMaterialGroup_Vtbl, 0x94f070b8_2515_4a8d_a1f0_d0fc13d06021);
impl windows_core::RuntimeType for IPrinting3DBaseMaterialGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DBaseMaterialGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Bases: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bases: usize,
    pub MaterialGroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DBaseMaterialGroupFactory, IPrinting3DBaseMaterialGroupFactory_Vtbl, 0x5c1546dc_8697_4193_976b_84bb4116e5bf);
impl windows_core::RuntimeType for IPrinting3DBaseMaterialGroupFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DBaseMaterialGroupFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DBaseMaterialStatics, IPrinting3DBaseMaterialStatics_Vtbl, 0x815a47bc_374a_476d_be92_3ecfd1cb9776);
impl windows_core::RuntimeType for IPrinting3DBaseMaterialStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DBaseMaterialStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Abs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Pla: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DColorMaterial, IPrinting3DColorMaterial_Vtbl, 0xe1899928_7ce7_4285_a35d_f145c9510c7b);
impl windows_core::RuntimeType for IPrinting3DColorMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DColorMaterial_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DColorMaterial2, IPrinting3DColorMaterial2_Vtbl, 0xfab0e852_0aef_44e9_9ddd_36eeea5acd44);
impl windows_core::RuntimeType for IPrinting3DColorMaterial2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DColorMaterial2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::UI::Color) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
}
windows_core::imp::define_interface!(IPrinting3DColorMaterialGroup, IPrinting3DColorMaterialGroup_Vtbl, 0x001a6bd0_aadf_4226_afe9_f369a0b45004);
impl windows_core::RuntimeType for IPrinting3DColorMaterialGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DColorMaterialGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Colors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Colors: usize,
    pub MaterialGroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DColorMaterialGroupFactory, IPrinting3DColorMaterialGroupFactory_Vtbl, 0x71d38d6d_b1ea_4a5b_bc54_19c65f3df044);
impl windows_core::RuntimeType for IPrinting3DColorMaterialGroupFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DColorMaterialGroupFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DComponent, IPrinting3DComponent_Vtbl, 0x7e287845_bf7f_4cdb_a27f_30a01437fede);
impl windows_core::RuntimeType for IPrinting3DComponent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DComponent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMesh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Components: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Components: usize,
    pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Printing3DObjectType) -> windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, Printing3DObjectType) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub PartNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetPartNumber: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DComponentWithMatrix, IPrinting3DComponentWithMatrix_Vtbl, 0x3279f335_0ef0_456b_9a21_49bebe8b51c2);
impl windows_core::RuntimeType for IPrinting3DComponentWithMatrix {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DComponentWithMatrix_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Component: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Matrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Matrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Numerics::Matrix4x4) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
}
windows_core::imp::define_interface!(IPrinting3DCompositeMaterial, IPrinting3DCompositeMaterial_Vtbl, 0x462238dd_562e_4f6c_882d_f4d841fd63c7);
impl windows_core::RuntimeType for IPrinting3DCompositeMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterial_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Values: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Values: usize,
}
windows_core::imp::define_interface!(IPrinting3DCompositeMaterialGroup, IPrinting3DCompositeMaterialGroup_Vtbl, 0x8d946a5b_40f1_496d_a5fb_340a5a678e30);
impl windows_core::RuntimeType for IPrinting3DCompositeMaterialGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterialGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Composites: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Composites: usize,
    pub MaterialGroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MaterialIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MaterialIndices: usize,
}
windows_core::imp::define_interface!(IPrinting3DCompositeMaterialGroup2, IPrinting3DCompositeMaterialGroup2_Vtbl, 0x06e86d62_7d3b_41e1_944c_bafde4555483);
impl windows_core::RuntimeType for IPrinting3DCompositeMaterialGroup2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterialGroup2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BaseMaterialGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBaseMaterialGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DCompositeMaterialGroupFactory, IPrinting3DCompositeMaterialGroupFactory_Vtbl, 0xd08ecd13_92ff_43aa_a627_8d43c22c817e);
impl windows_core::RuntimeType for IPrinting3DCompositeMaterialGroupFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DCompositeMaterialGroupFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DFaceReductionOptions, IPrinting3DFaceReductionOptions_Vtbl, 0xbbfed397_2d74_46f7_be85_99a67bbb6629);
impl windows_core::RuntimeType for IPrinting3DFaceReductionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DFaceReductionOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxReductionArea: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMaxReductionArea: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub TargetTriangleCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTargetTriangleCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub MaxEdgeLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMaxEdgeLength: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DMaterial, IPrinting3DMaterial_Vtbl, 0x378db256_ed62_4952_b85b_03567d7c465e);
impl windows_core::RuntimeType for IPrinting3DMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DMaterial_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub BaseGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BaseGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ColorGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ColorGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Texture2CoordGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Texture2CoordGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CompositeGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CompositeGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MultiplePropertyGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MultiplePropertyGroups: usize,
}
windows_core::imp::define_interface!(IPrinting3DMesh, IPrinting3DMesh_Vtbl, 0x192e90dc_0228_2e01_bc20_c5290cbf32c4);
impl windows_core::RuntimeType for IPrinting3DMesh {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DMesh_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VertexCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetVertexCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IndexCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetIndexCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub VertexPositionsDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Printing3DBufferDescription) -> windows_core::HRESULT,
    pub SetVertexPositionsDescription: unsafe extern "system" fn(*mut core::ffi::c_void, Printing3DBufferDescription) -> windows_core::HRESULT,
    pub VertexNormalsDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Printing3DBufferDescription) -> windows_core::HRESULT,
    pub SetVertexNormalsDescription: unsafe extern "system" fn(*mut core::ffi::c_void, Printing3DBufferDescription) -> windows_core::HRESULT,
    pub TriangleIndicesDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Printing3DBufferDescription) -> windows_core::HRESULT,
    pub SetTriangleIndicesDescription: unsafe extern "system" fn(*mut core::ffi::c_void, Printing3DBufferDescription) -> windows_core::HRESULT,
    pub TriangleMaterialIndicesDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Printing3DBufferDescription) -> windows_core::HRESULT,
    pub SetTriangleMaterialIndicesDescription: unsafe extern "system" fn(*mut core::ffi::c_void, Printing3DBufferDescription) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetVertexPositions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetVertexPositions: usize,
    pub CreateVertexPositions: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetVertexNormals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetVertexNormals: usize,
    pub CreateVertexNormals: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetTriangleIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetTriangleIndices: usize,
    pub CreateTriangleIndices: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetTriangleMaterialIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetTriangleMaterialIndices: usize,
    pub CreateTriangleMaterialIndices: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BufferDescriptionSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BufferDescriptionSet: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BufferSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BufferSet: usize,
    pub VerifyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, Printing3DMeshVerificationMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DMeshVerificationResult, IPrinting3DMeshVerificationResult_Vtbl, 0x195671ba_e93a_4e8a_a46f_dea8e852197e);
impl windows_core::RuntimeType for IPrinting3DMeshVerificationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DMeshVerificationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsValid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub NonmanifoldTriangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NonmanifoldTriangles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReversedNormalTriangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReversedNormalTriangles: usize,
}
windows_core::imp::define_interface!(IPrinting3DModel, IPrinting3DModel_Vtbl, 0x2d012ef0_52fb_919a_77b0_4b1a3b80324f);
impl windows_core::RuntimeType for IPrinting3DModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DModel_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Unit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Printing3DModelUnit) -> windows_core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(*mut core::ffi::c_void, Printing3DModelUnit) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Textures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Textures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Meshes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Meshes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Components: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Components: usize,
    pub Material: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMaterial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Build: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBuild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequiredExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequiredExtensions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    pub RepairAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DModel2, IPrinting3DModel2_Vtbl, 0xc92069c7_c841_47f3_a84e_a149fd08b657);
impl windows_core::RuntimeType for IPrinting3DModel2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DModel2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryPartialRepairAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryPartialRepairWithTimeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryReduceFacesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryReduceFacesWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryReduceFacesWithOptionsAndTimeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::Foundation::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RepairWithProgressAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DModelTexture, IPrinting3DModelTexture_Vtbl, 0x5dafcf01_b59d_483c_97bb_a4d546d1c75c);
impl windows_core::RuntimeType for IPrinting3DModelTexture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DModelTexture_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TextureResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTextureResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TileStyleU: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Printing3DTextureEdgeBehavior) -> windows_core::HRESULT,
    pub SetTileStyleU: unsafe extern "system" fn(*mut core::ffi::c_void, Printing3DTextureEdgeBehavior) -> windows_core::HRESULT,
    pub TileStyleV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Printing3DTextureEdgeBehavior) -> windows_core::HRESULT,
    pub SetTileStyleV: unsafe extern "system" fn(*mut core::ffi::c_void, Printing3DTextureEdgeBehavior) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DMultiplePropertyMaterial, IPrinting3DMultiplePropertyMaterial_Vtbl, 0x25a6254b_c6e9_484d_a214_a25e5776ba62);
impl windows_core::RuntimeType for IPrinting3DMultiplePropertyMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DMultiplePropertyMaterial_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub MaterialIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MaterialIndices: usize,
}
windows_core::imp::define_interface!(IPrinting3DMultiplePropertyMaterialGroup, IPrinting3DMultiplePropertyMaterialGroup_Vtbl, 0xf0950519_aeb9_4515_a39b_a088fbbb277c);
impl windows_core::RuntimeType for IPrinting3DMultiplePropertyMaterialGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DMultiplePropertyMaterialGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub MultipleProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MultipleProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MaterialGroupIndices: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MaterialGroupIndices: usize,
    pub MaterialGroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DMultiplePropertyMaterialGroupFactory, IPrinting3DMultiplePropertyMaterialGroupFactory_Vtbl, 0x323e196e_d4c6_451e_a814_4d78a210fe53);
impl windows_core::RuntimeType for IPrinting3DMultiplePropertyMaterialGroupFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DMultiplePropertyMaterialGroupFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DTexture2CoordMaterial, IPrinting3DTexture2CoordMaterial_Vtbl, 0x8d844bfb_07e9_4986_9833_8dd3d48c6859);
impl windows_core::RuntimeType for IPrinting3DTexture2CoordMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DTexture2CoordMaterial_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Texture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub U: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetU: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub V: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetV: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DTexture2CoordMaterialGroup, IPrinting3DTexture2CoordMaterialGroup_Vtbl, 0x627d7ca7_6d90_4fb9_9fc4_9feff3dfa892);
impl windows_core::RuntimeType for IPrinting3DTexture2CoordMaterialGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DTexture2CoordMaterialGroup_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Texture2Coords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Texture2Coords: usize,
    pub MaterialGroupId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DTexture2CoordMaterialGroup2, IPrinting3DTexture2CoordMaterialGroup2_Vtbl, 0x69fbdbba_b12e_429b_8386_df5284f6e80f);
impl windows_core::RuntimeType for IPrinting3DTexture2CoordMaterialGroup2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DTexture2CoordMaterialGroup2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Texture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DTexture2CoordMaterialGroupFactory, IPrinting3DTexture2CoordMaterialGroupFactory_Vtbl, 0xcbb049b0_468a_4c6f_b2a2_8eb8ba8dea48);
impl windows_core::RuntimeType for IPrinting3DTexture2CoordMaterialGroupFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DTexture2CoordMaterialGroupFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPrinting3DTextureResource, IPrinting3DTextureResource_Vtbl, 0xa70df32d_6ab1_44ae_bc45_a27382c0d38c);
impl windows_core::RuntimeType for IPrinting3DTextureResource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPrinting3DTextureResource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub TextureData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TextureData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetTextureData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetTextureData: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Print3DManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DManager, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DManager {
    pub fn TaskRequested<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TaskRequested)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTaskRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTaskRequested)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<Print3DManager> {
        Self::IPrint3DManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ShowPrintUIAsync() -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPrint3DManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShowPrintUIAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrint3DManagerStatics<R, F: FnOnce(&IPrint3DManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Print3DManager, IPrint3DManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Print3DManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DManager>();
}
unsafe impl windows_core::Interface for Print3DManager {
    type Vtable = IPrint3DManager_Vtbl;
    const IID: windows_core::GUID = <IPrint3DManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DManager {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DManager";
}
unsafe impl Send for Print3DManager {}
unsafe impl Sync for Print3DManager {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Print3DTask(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DTask, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DTask {
    pub fn Source(&self) -> windows_core::Result<Printing3D3MFPackage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Submitting<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Print3DTask, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Submitting)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSubmitting(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSubmitting)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn Completed<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompleted(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveCompleted)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn SourceChanged<P0>(&self, eventhandler: P0) -> windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceChanged)(windows_core::Interface::as_raw(this), eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSourceChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveSourceChanged)(windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
}
impl windows_core::RuntimeType for Print3DTask {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DTask>();
}
unsafe impl windows_core::Interface for Print3DTask {
    type Vtable = IPrint3DTask_Vtbl;
    const IID: windows_core::GUID = <IPrint3DTask as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DTask {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTask";
}
unsafe impl Send for Print3DTask {}
unsafe impl Sync for Print3DTask {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Print3DTaskCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DTaskCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DTaskCompletedEventArgs {
    pub fn Completion(&self) -> windows_core::Result<Print3DTaskCompletion> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedStatus(&self) -> windows_core::Result<Print3DTaskDetail> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for Print3DTaskCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DTaskCompletedEventArgs>();
}
unsafe impl windows_core::Interface for Print3DTaskCompletedEventArgs {
    type Vtable = IPrint3DTaskCompletedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrint3DTaskCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs";
}
unsafe impl Send for Print3DTaskCompletedEventArgs {}
unsafe impl Sync for Print3DTaskCompletedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Print3DTaskRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DTaskRequest, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DTaskRequest {
    pub fn CreateTask<P0>(&self, title: &windows_core::HSTRING, printerid: &windows_core::HSTRING, handler: P0) -> windows_core::Result<Print3DTask>
    where
        P0: windows_core::Param<Print3DTaskSourceRequestedHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTask)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(title), core::mem::transmute_copy(printerid), handler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Print3DTaskRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DTaskRequest>();
}
unsafe impl windows_core::Interface for Print3DTaskRequest {
    type Vtable = IPrint3DTaskRequest_Vtbl;
    const IID: windows_core::GUID = <IPrint3DTaskRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskRequest";
}
unsafe impl Send for Print3DTaskRequest {}
unsafe impl Sync for Print3DTaskRequest {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Print3DTaskRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DTaskRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DTaskRequestedEventArgs {
    pub fn Request(&self) -> windows_core::Result<Print3DTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Request)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Print3DTaskRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DTaskRequestedEventArgs>();
}
unsafe impl windows_core::Interface for Print3DTaskRequestedEventArgs {
    type Vtable = IPrint3DTaskRequestedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrint3DTaskRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs";
}
unsafe impl Send for Print3DTaskRequestedEventArgs {}
unsafe impl Sync for Print3DTaskRequestedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Print3DTaskSourceChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DTaskSourceChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DTaskSourceChangedEventArgs {
    pub fn Source(&self) -> windows_core::Result<Printing3D3MFPackage> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Print3DTaskSourceChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DTaskSourceChangedEventArgs>();
}
unsafe impl windows_core::Interface for Print3DTaskSourceChangedEventArgs {
    type Vtable = IPrint3DTaskSourceChangedEventArgs_Vtbl;
    const IID: windows_core::GUID = <IPrint3DTaskSourceChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DTaskSourceChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs";
}
unsafe impl Send for Print3DTaskSourceChangedEventArgs {}
unsafe impl Sync for Print3DTaskSourceChangedEventArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Print3DTaskSourceRequestedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Print3DTaskSourceRequestedArgs, windows_core::IUnknown, windows_core::IInspectable);
impl Print3DTaskSourceRequestedArgs {
    pub fn SetSource<P0>(&self, source: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3D3MFPackage>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), source.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for Print3DTaskSourceRequestedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrint3DTaskSourceRequestedArgs>();
}
unsafe impl windows_core::Interface for Print3DTaskSourceRequestedArgs {
    type Vtable = IPrint3DTaskSourceRequestedArgs_Vtbl;
    const IID: windows_core::GUID = <IPrint3DTaskSourceRequestedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Print3DTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs";
}
unsafe impl Send for Print3DTaskSourceRequestedArgs {}
unsafe impl Sync for Print3DTaskSourceRequestedArgs {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3D3MFPackage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3D3MFPackage, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3D3MFPackage {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3D3MFPackage, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PrintTicket(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrintTicket)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPrintTicket<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrintTicket)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ModelPart(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ModelPart)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelPart<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetModelPart)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Thumbnail(&self) -> windows_core::Result<Printing3DTextureResource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetThumbnail<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DTextureResource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetThumbnail)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Textures(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DTextureResource>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Textures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadModelFromPackageAsync<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Printing3DModel>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadModelFromPackageAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SaveModelToPackageAsync<P0>(&self, value: P0) -> windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<Printing3DModel>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveModelToPackageAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Compression(&self) -> windows_core::Result<Printing3DPackageCompression> {
        let this = &windows_core::Interface::cast::<IPrinting3D3MFPackage2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Compression)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCompression(&self, value: Printing3DPackageCompression) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrinting3D3MFPackage2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCompression)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadAsync<P0>(value: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Printing3D3MFPackage>>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IPrinting3D3MFPackageStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadAsync)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3D3MFPackageStatics<R, F: FnOnce(&IPrinting3D3MFPackageStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3D3MFPackage, IPrinting3D3MFPackageStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Printing3D3MFPackage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3D3MFPackage>();
}
unsafe impl windows_core::Interface for Printing3D3MFPackage {
    type Vtable = IPrinting3D3MFPackage_Vtbl;
    const IID: windows_core::GUID = <IPrinting3D3MFPackage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3D3MFPackage {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3D3MFPackage";
}
unsafe impl Send for Printing3D3MFPackage {}
unsafe impl Sync for Printing3D3MFPackage {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DBaseMaterial(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DBaseMaterial, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DBaseMaterial {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DBaseMaterial, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Color(&self) -> windows_core::Result<Printing3DColorMaterial> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Color)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetColor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DColorMaterial>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetColor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Abs() -> windows_core::Result<windows_core::HSTRING> {
        Self::IPrinting3DBaseMaterialStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Abs)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Pla() -> windows_core::Result<windows_core::HSTRING> {
        Self::IPrinting3DBaseMaterialStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Pla)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DBaseMaterialStatics<R, F: FnOnce(&IPrinting3DBaseMaterialStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DBaseMaterial, IPrinting3DBaseMaterialStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Printing3DBaseMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DBaseMaterial>();
}
unsafe impl windows_core::Interface for Printing3DBaseMaterial {
    type Vtable = IPrinting3DBaseMaterial_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DBaseMaterial as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DBaseMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DBaseMaterial";
}
unsafe impl Send for Printing3DBaseMaterial {}
unsafe impl Sync for Printing3DBaseMaterial {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DBaseMaterialGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DBaseMaterialGroup, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DBaseMaterialGroup {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bases(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterial>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bases)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaterialGroupId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialGroupId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(materialgroupid: u32) -> windows_core::Result<Printing3DBaseMaterialGroup> {
        Self::IPrinting3DBaseMaterialGroupFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), materialgroupid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DBaseMaterialGroupFactory<R, F: FnOnce(&IPrinting3DBaseMaterialGroupFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DBaseMaterialGroup, IPrinting3DBaseMaterialGroupFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Printing3DBaseMaterialGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DBaseMaterialGroup>();
}
unsafe impl windows_core::Interface for Printing3DBaseMaterialGroup {
    type Vtable = IPrinting3DBaseMaterialGroup_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DBaseMaterialGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DBaseMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup";
}
unsafe impl Send for Printing3DBaseMaterialGroup {}
unsafe impl Sync for Printing3DBaseMaterialGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DColorMaterial(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DColorMaterial, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DColorMaterial {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DColorMaterial, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Value(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValue(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> windows_core::Result<super::super::UI::Color> {
        let this = &windows_core::Interface::cast::<IPrinting3DColorMaterial2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Color)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::UI::Color) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPrinting3DColorMaterial2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetColor)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for Printing3DColorMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DColorMaterial>();
}
unsafe impl windows_core::Interface for Printing3DColorMaterial {
    type Vtable = IPrinting3DColorMaterial_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DColorMaterial as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DColorMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DColorMaterial";
}
unsafe impl Send for Printing3DColorMaterial {}
unsafe impl Sync for Printing3DColorMaterial {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DColorMaterialGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DColorMaterialGroup, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DColorMaterialGroup {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Colors(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterial>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Colors)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaterialGroupId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialGroupId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(materialgroupid: u32) -> windows_core::Result<Printing3DColorMaterialGroup> {
        Self::IPrinting3DColorMaterialGroupFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), materialgroupid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DColorMaterialGroupFactory<R, F: FnOnce(&IPrinting3DColorMaterialGroupFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DColorMaterialGroup, IPrinting3DColorMaterialGroupFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Printing3DColorMaterialGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DColorMaterialGroup>();
}
unsafe impl windows_core::Interface for Printing3DColorMaterialGroup {
    type Vtable = IPrinting3DColorMaterialGroup_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DColorMaterialGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DColorMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DColorMaterialGroup";
}
unsafe impl Send for Printing3DColorMaterialGroup {}
unsafe impl Sync for Printing3DColorMaterialGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DComponent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DComponent, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DComponent {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DComponent, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mesh(&self) -> windows_core::Result<Printing3DMesh> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Mesh)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMesh<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DMesh>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMesh)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Components(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DComponentWithMatrix>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Components)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Thumbnail(&self) -> windows_core::Result<Printing3DTextureResource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetThumbnail<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DTextureResource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetThumbnail)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Type(&self) -> windows_core::Result<Printing3DObjectType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetType(&self, value: Printing3DObjectType) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetType)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn PartNumber(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PartNumber)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetPartNumber(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPartNumber)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for Printing3DComponent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DComponent>();
}
unsafe impl windows_core::Interface for Printing3DComponent {
    type Vtable = IPrinting3DComponent_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DComponent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DComponent {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DComponent";
}
unsafe impl Send for Printing3DComponent {}
unsafe impl Sync for Printing3DComponent {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DComponentWithMatrix(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DComponentWithMatrix, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DComponentWithMatrix {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DComponentWithMatrix, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Component(&self) -> windows_core::Result<Printing3DComponent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Component)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetComponent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DComponent>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetComponent)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Matrix(&self) -> windows_core::Result<super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Matrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetMatrix(&self, value: super::super::Foundation::Numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for Printing3DComponentWithMatrix {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DComponentWithMatrix>();
}
unsafe impl windows_core::Interface for Printing3DComponentWithMatrix {
    type Vtable = IPrinting3DComponentWithMatrix_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DComponentWithMatrix as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DComponentWithMatrix {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DComponentWithMatrix";
}
unsafe impl Send for Printing3DComponentWithMatrix {}
unsafe impl Sync for Printing3DComponentWithMatrix {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DCompositeMaterial(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DCompositeMaterial, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DCompositeMaterial {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DCompositeMaterial, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Values(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<f64>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Values)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Printing3DCompositeMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DCompositeMaterial>();
}
unsafe impl windows_core::Interface for Printing3DCompositeMaterial {
    type Vtable = IPrinting3DCompositeMaterial_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DCompositeMaterial as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DCompositeMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DCompositeMaterial";
}
unsafe impl Send for Printing3DCompositeMaterial {}
unsafe impl Sync for Printing3DCompositeMaterial {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DCompositeMaterialGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DCompositeMaterialGroup, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DCompositeMaterialGroup {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Composites(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterial>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Composites)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaterialGroupId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialGroupId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MaterialIndices(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialIndices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn BaseMaterialGroup(&self) -> windows_core::Result<Printing3DBaseMaterialGroup> {
        let this = &windows_core::Interface::cast::<IPrinting3DCompositeMaterialGroup2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseMaterialGroup)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBaseMaterialGroup<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DBaseMaterialGroup>,
    {
        let this = &windows_core::Interface::cast::<IPrinting3DCompositeMaterialGroup2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBaseMaterialGroup)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create(materialgroupid: u32) -> windows_core::Result<Printing3DCompositeMaterialGroup> {
        Self::IPrinting3DCompositeMaterialGroupFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), materialgroupid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DCompositeMaterialGroupFactory<R, F: FnOnce(&IPrinting3DCompositeMaterialGroupFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DCompositeMaterialGroup, IPrinting3DCompositeMaterialGroupFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Printing3DCompositeMaterialGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DCompositeMaterialGroup>();
}
unsafe impl windows_core::Interface for Printing3DCompositeMaterialGroup {
    type Vtable = IPrinting3DCompositeMaterialGroup_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DCompositeMaterialGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DCompositeMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup";
}
unsafe impl Send for Printing3DCompositeMaterialGroup {}
unsafe impl Sync for Printing3DCompositeMaterialGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DFaceReductionOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DFaceReductionOptions, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DFaceReductionOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DFaceReductionOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaxReductionArea(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxReductionArea)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxReductionArea(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxReductionArea)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TargetTriangleCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TargetTriangleCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTargetTriangleCount(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTargetTriangleCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxEdgeLength(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxEdgeLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxEdgeLength(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxEdgeLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for Printing3DFaceReductionOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DFaceReductionOptions>();
}
unsafe impl windows_core::Interface for Printing3DFaceReductionOptions {
    type Vtable = IPrinting3DFaceReductionOptions_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DFaceReductionOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DFaceReductionOptions {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DFaceReductionOptions";
}
unsafe impl Send for Printing3DFaceReductionOptions {}
unsafe impl Sync for Printing3DFaceReductionOptions {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DMaterial(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DMaterial, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DMaterial {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DMaterial, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BaseGroups(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseGroups)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ColorGroups(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColorGroups)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Texture2CoordGroups(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Texture2CoordGroups)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CompositeGroups(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompositeGroups)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MultiplePropertyGroups(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MultiplePropertyGroups)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Printing3DMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DMaterial>();
}
unsafe impl windows_core::Interface for Printing3DMaterial {
    type Vtable = IPrinting3DMaterial_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DMaterial as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMaterial";
}
unsafe impl Send for Printing3DMaterial {}
unsafe impl Sync for Printing3DMaterial {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DMesh(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DMesh, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DMesh {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DMesh, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn VertexCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VertexCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVertexCount(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVertexCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IndexCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIndexCount(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetIndexCount)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VertexPositionsDescription(&self) -> windows_core::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VertexPositionsDescription)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVertexPositionsDescription(&self, value: Printing3DBufferDescription) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVertexPositionsDescription)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VertexNormalsDescription(&self) -> windows_core::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VertexNormalsDescription)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVertexNormalsDescription(&self, value: Printing3DBufferDescription) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVertexNormalsDescription)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriangleIndicesDescription(&self) -> windows_core::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriangleIndicesDescription)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTriangleIndicesDescription(&self, value: Printing3DBufferDescription) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTriangleIndicesDescription)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriangleMaterialIndicesDescription(&self) -> windows_core::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TriangleMaterialIndicesDescription)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTriangleMaterialIndicesDescription(&self, value: Printing3DBufferDescription) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTriangleMaterialIndicesDescription)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetVertexPositions(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVertexPositions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateVertexPositions(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CreateVertexPositions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetVertexNormals(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetVertexNormals)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateVertexNormals(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CreateVertexNormals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTriangleIndices(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTriangleIndices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateTriangleIndices(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CreateTriangleIndices)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTriangleMaterialIndices(&self) -> windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTriangleMaterialIndices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateTriangleMaterialIndices(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).CreateTriangleMaterialIndices)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BufferDescriptionSet(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferDescriptionSet)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn BufferSet(&self) -> windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BufferSet)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn VerifyAsync(&self, value: Printing3DMeshVerificationMode) -> windows_core::Result<super::super::Foundation::IAsyncOperation<Printing3DMeshVerificationResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerifyAsync)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Printing3DMesh {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DMesh>();
}
unsafe impl windows_core::Interface for Printing3DMesh {
    type Vtable = IPrinting3DMesh_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DMesh as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DMesh {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMesh";
}
unsafe impl Send for Printing3DMesh {}
unsafe impl Sync for Printing3DMesh {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DMeshVerificationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DMeshVerificationResult, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DMeshVerificationResult {
    pub fn IsValid(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsValid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn NonmanifoldTriangles(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NonmanifoldTriangles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReversedNormalTriangles(&self) -> windows_core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReversedNormalTriangles)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Printing3DMeshVerificationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DMeshVerificationResult>();
}
unsafe impl windows_core::Interface for Printing3DMeshVerificationResult {
    type Vtable = IPrinting3DMeshVerificationResult_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DMeshVerificationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DMeshVerificationResult {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMeshVerificationResult";
}
unsafe impl Send for Printing3DMeshVerificationResult {}
unsafe impl Sync for Printing3DMeshVerificationResult {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DModel(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DModel, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DModel {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DModel, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Unit(&self) -> windows_core::Result<Printing3DModelUnit> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Unit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUnit(&self, value: Printing3DModelUnit) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Textures(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DModelTexture>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Textures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Meshes(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DMesh>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Meshes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Components(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DComponent>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Components)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Material(&self) -> windows_core::Result<Printing3DMaterial> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Material)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetMaterial<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DMaterial>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaterial)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Build(&self) -> windows_core::Result<Printing3DComponent> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Build)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetBuild<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DComponent>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetBuild)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Version(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Version)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetVersion(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetVersion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequiredExtensions(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequiredExtensions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metadata(&self) -> windows_core::Result<super::super::Foundation::Collections::IMap<windows_core::HSTRING, windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Metadata)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RepairAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RepairAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Clone(&self) -> windows_core::Result<Printing3DModel> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clone)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryPartialRepairAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryPartialRepairAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryPartialRepairWithTimeAsync(&self, maxwaittime: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryPartialRepairWithTimeAsync)(windows_core::Interface::as_raw(this), maxwaittime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryReduceFacesAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>> {
        let this = &windows_core::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryReduceFacesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryReduceFacesWithOptionsAsync<P0>(&self, printing3dfacereductionoptions: P0) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>
    where
        P0: windows_core::Param<Printing3DFaceReductionOptions>,
    {
        let this = &windows_core::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryReduceFacesWithOptionsAsync)(windows_core::Interface::as_raw(this), printing3dfacereductionoptions.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryReduceFacesWithOptionsAndTimeAsync<P0>(&self, printing3dfacereductionoptions: P0, maxwait: super::super::Foundation::TimeSpan) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>
    where
        P0: windows_core::Param<Printing3DFaceReductionOptions>,
    {
        let this = &windows_core::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryReduceFacesWithOptionsAndTimeAsync)(windows_core::Interface::as_raw(this), printing3dfacereductionoptions.param().abi(), maxwait, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RepairWithProgressAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>> {
        let this = &windows_core::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RepairWithProgressAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Printing3DModel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DModel>();
}
unsafe impl windows_core::Interface for Printing3DModel {
    type Vtable = IPrinting3DModel_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DModel as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DModel {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DModel";
}
unsafe impl Send for Printing3DModel {}
unsafe impl Sync for Printing3DModel {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DModelTexture(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DModelTexture, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DModelTexture {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DModelTexture, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TextureResource(&self) -> windows_core::Result<Printing3DTextureResource> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextureResource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTextureResource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DTextureResource>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextureResource)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn TileStyleU(&self) -> windows_core::Result<Printing3DTextureEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileStyleU)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTileStyleU(&self, value: Printing3DTextureEdgeBehavior) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTileStyleU)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TileStyleV(&self) -> windows_core::Result<Printing3DTextureEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileStyleV)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTileStyleV(&self, value: Printing3DTextureEdgeBehavior) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTileStyleV)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for Printing3DModelTexture {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DModelTexture>();
}
unsafe impl windows_core::Interface for Printing3DModelTexture {
    type Vtable = IPrinting3DModelTexture_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DModelTexture as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DModelTexture {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DModelTexture";
}
unsafe impl Send for Printing3DModelTexture {}
unsafe impl Sync for Printing3DModelTexture {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DMultiplePropertyMaterial(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DMultiplePropertyMaterial, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DMultiplePropertyMaterial {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DMultiplePropertyMaterial, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MaterialIndices(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialIndices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Printing3DMultiplePropertyMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DMultiplePropertyMaterial>();
}
unsafe impl windows_core::Interface for Printing3DMultiplePropertyMaterial {
    type Vtable = IPrinting3DMultiplePropertyMaterial_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DMultiplePropertyMaterial as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DMultiplePropertyMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial";
}
unsafe impl Send for Printing3DMultiplePropertyMaterial {}
unsafe impl Sync for Printing3DMultiplePropertyMaterial {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DMultiplePropertyMaterialGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DMultiplePropertyMaterialGroup, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DMultiplePropertyMaterialGroup {
    #[cfg(feature = "Foundation_Collections")]
    pub fn MultipleProperties(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterial>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MultipleProperties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MaterialGroupIndices(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialGroupIndices)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaterialGroupId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialGroupId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Create(materialgroupid: u32) -> windows_core::Result<Printing3DMultiplePropertyMaterialGroup> {
        Self::IPrinting3DMultiplePropertyMaterialGroupFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), materialgroupid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DMultiplePropertyMaterialGroupFactory<R, F: FnOnce(&IPrinting3DMultiplePropertyMaterialGroupFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DMultiplePropertyMaterialGroup, IPrinting3DMultiplePropertyMaterialGroupFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Printing3DMultiplePropertyMaterialGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DMultiplePropertyMaterialGroup>();
}
unsafe impl windows_core::Interface for Printing3DMultiplePropertyMaterialGroup {
    type Vtable = IPrinting3DMultiplePropertyMaterialGroup_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DMultiplePropertyMaterialGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DMultiplePropertyMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup";
}
unsafe impl Send for Printing3DMultiplePropertyMaterialGroup {}
unsafe impl Sync for Printing3DMultiplePropertyMaterialGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DTexture2CoordMaterial(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DTexture2CoordMaterial, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DTexture2CoordMaterial {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DTexture2CoordMaterial, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Texture(&self) -> windows_core::Result<Printing3DModelTexture> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Texture)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTexture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DModelTexture>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTexture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn U(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).U)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetU(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetU)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn V(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).V)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetV(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetV)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for Printing3DTexture2CoordMaterial {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DTexture2CoordMaterial>();
}
unsafe impl windows_core::Interface for Printing3DTexture2CoordMaterial {
    type Vtable = IPrinting3DTexture2CoordMaterial_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DTexture2CoordMaterial as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DTexture2CoordMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial";
}
unsafe impl Send for Printing3DTexture2CoordMaterial {}
unsafe impl Sync for Printing3DTexture2CoordMaterial {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DTexture2CoordMaterialGroup(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DTexture2CoordMaterialGroup, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DTexture2CoordMaterialGroup {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Texture2Coords(&self) -> windows_core::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterial>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Texture2Coords)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn MaterialGroupId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaterialGroupId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Texture(&self) -> windows_core::Result<Printing3DModelTexture> {
        let this = &windows_core::Interface::cast::<IPrinting3DTexture2CoordMaterialGroup2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Texture)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTexture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Printing3DModelTexture>,
    {
        let this = &windows_core::Interface::cast::<IPrinting3DTexture2CoordMaterialGroup2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTexture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Create(materialgroupid: u32) -> windows_core::Result<Printing3DTexture2CoordMaterialGroup> {
        Self::IPrinting3DTexture2CoordMaterialGroupFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), materialgroupid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DTexture2CoordMaterialGroupFactory<R, F: FnOnce(&IPrinting3DTexture2CoordMaterialGroupFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DTexture2CoordMaterialGroup, IPrinting3DTexture2CoordMaterialGroupFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Printing3DTexture2CoordMaterialGroup {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DTexture2CoordMaterialGroup>();
}
unsafe impl windows_core::Interface for Printing3DTexture2CoordMaterialGroup {
    type Vtable = IPrinting3DTexture2CoordMaterialGroup_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DTexture2CoordMaterialGroup as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DTexture2CoordMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup";
}
unsafe impl Send for Printing3DTexture2CoordMaterialGroup {}
unsafe impl Sync for Printing3DTexture2CoordMaterialGroup {}
#[repr(transparent)]
#[derive(PartialEq, Eq, core::fmt::Debug, Clone)]
pub struct Printing3DTextureResource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Printing3DTextureResource, windows_core::IUnknown, windows_core::IInspectable);
impl Printing3DTextureResource {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Printing3DTextureResource, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn TextureData(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextureData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextureData<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamWithContentType>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextureData)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for Printing3DTextureResource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPrinting3DTextureResource>();
}
unsafe impl windows_core::Interface for Printing3DTextureResource {
    type Vtable = IPrinting3DTextureResource_Vtbl;
    const IID: windows_core::GUID = <IPrinting3DTextureResource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Printing3DTextureResource {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DTextureResource";
}
unsafe impl Send for Printing3DTextureResource {}
unsafe impl Sync for Printing3DTextureResource {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Print3DTaskCompletion(pub i32);
impl Print3DTaskCompletion {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Slicing: Self = Self(3i32);
    pub const Submitted: Self = Self(4i32);
}
impl windows_core::TypeKind for Print3DTaskCompletion {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Print3DTaskCompletion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Print3DTaskCompletion").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Print3DTaskCompletion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Print3DTaskCompletion;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Print3DTaskDetail(pub i32);
impl Print3DTaskDetail {
    pub const Unknown: Self = Self(0i32);
    pub const ModelExceedsPrintBed: Self = Self(1i32);
    pub const UploadFailed: Self = Self(2i32);
    pub const InvalidMaterialSelection: Self = Self(3i32);
    pub const InvalidModel: Self = Self(4i32);
    pub const ModelNotManifold: Self = Self(5i32);
    pub const InvalidPrintTicket: Self = Self(6i32);
}
impl windows_core::TypeKind for Print3DTaskDetail {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Print3DTaskDetail {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Print3DTaskDetail").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Print3DTaskDetail {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Print3DTaskDetail;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Printing3DBufferFormat(pub i32);
impl Printing3DBufferFormat {
    pub const Unknown: Self = Self(0i32);
    pub const R32G32B32A32Float: Self = Self(2i32);
    pub const R32G32B32A32UInt: Self = Self(3i32);
    pub const R32G32B32Float: Self = Self(6i32);
    pub const R32G32B32UInt: Self = Self(7i32);
    pub const Printing3DDouble: Self = Self(500i32);
    pub const Printing3DUInt: Self = Self(501i32);
}
impl windows_core::TypeKind for Printing3DBufferFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Printing3DBufferFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Printing3DBufferFormat").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Printing3DBufferFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DBufferFormat;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Printing3DMeshVerificationMode(pub i32);
impl Printing3DMeshVerificationMode {
    pub const FindFirstError: Self = Self(0i32);
    pub const FindAllErrors: Self = Self(1i32);
}
impl windows_core::TypeKind for Printing3DMeshVerificationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Printing3DMeshVerificationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Printing3DMeshVerificationMode").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Printing3DMeshVerificationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DMeshVerificationMode;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Printing3DModelUnit(pub i32);
impl Printing3DModelUnit {
    pub const Meter: Self = Self(0i32);
    pub const Micron: Self = Self(1i32);
    pub const Millimeter: Self = Self(2i32);
    pub const Centimeter: Self = Self(3i32);
    pub const Inch: Self = Self(4i32);
    pub const Foot: Self = Self(5i32);
}
impl windows_core::TypeKind for Printing3DModelUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Printing3DModelUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Printing3DModelUnit").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Printing3DModelUnit {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DModelUnit;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Printing3DObjectType(pub i32);
impl Printing3DObjectType {
    pub const Model: Self = Self(0i32);
    pub const Support: Self = Self(1i32);
    pub const Others: Self = Self(2i32);
}
impl windows_core::TypeKind for Printing3DObjectType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Printing3DObjectType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Printing3DObjectType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Printing3DObjectType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DObjectType;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Printing3DPackageCompression(pub i32);
impl Printing3DPackageCompression {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl windows_core::TypeKind for Printing3DPackageCompression {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Printing3DPackageCompression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Printing3DPackageCompression").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Printing3DPackageCompression {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DPackageCompression;i4)");
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Printing3DTextureEdgeBehavior(pub i32);
impl Printing3DTextureEdgeBehavior {
    pub const None: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
    pub const Mirror: Self = Self(2i32);
    pub const Clamp: Self = Self(3i32);
}
impl windows_core::TypeKind for Printing3DTextureEdgeBehavior {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Printing3DTextureEdgeBehavior {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Printing3DTextureEdgeBehavior").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for Printing3DTextureEdgeBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DTextureEdgeBehavior;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Printing3DBufferDescription {
    pub Format: Printing3DBufferFormat,
    pub Stride: u32,
}
impl windows_core::TypeKind for Printing3DBufferDescription {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Printing3DBufferDescription {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Printing3D.Printing3DBufferDescription;enum(Windows.Graphics.Printing3D.Printing3DBufferFormat;i4);u4)");
}
impl Default for Printing3DBufferDescription {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(Print3DTaskSourceRequestedHandler, Print3DTaskSourceRequestedHandler_Vtbl, 0xe9175e70_c917_46de_bb51_d9a94db3711f);
impl Print3DTaskSourceRequestedHandler {
    pub fn new<F: FnMut(Option<&Print3DTaskSourceRequestedArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = Print3DTaskSourceRequestedHandlerBox::<F> { vtable: &Print3DTaskSourceRequestedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Print3DTaskSourceRequestedArgs>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
}
#[repr(C)]
struct Print3DTaskSourceRequestedHandlerBox<F: FnMut(Option<&Print3DTaskSourceRequestedArgs>) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const Print3DTaskSourceRequestedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(Option<&Print3DTaskSourceRequestedArgs>) -> windows_core::Result<()> + Send + 'static> Print3DTaskSourceRequestedHandlerBox<F> {
    const VTABLE: Print3DTaskSourceRequestedHandler_Vtbl = Print3DTaskSourceRequestedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = this as *mut *mut core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <Print3DTaskSourceRequestedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
        (this.invoke)(windows_core::from_raw_borrowed(&args)).into()
    }
}
impl windows_core::RuntimeType for Print3DTaskSourceRequestedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct Print3DTaskSourceRequestedHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
